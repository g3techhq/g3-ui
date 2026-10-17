//! Behaviour shared by overlays: page scroll locking and keyboard focus.
use crate::state::use_synced_signal;
use dioxus::prelude::*;
use std::cell::Cell;
use std::time::Duration;

/// `text` as a JavaScript string literal, safe to splice into an `eval`
/// script whatever it contains.
pub(crate) fn js_string(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 2);
    out.push('"');
    for ch in text.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '<' => out.push_str("\\u003c"),
            ch if (ch as u32) < 0x20 || ch == '\u{2028}' || ch == '\u{2029}' => {
                out.push_str(&format!("\\u{:04x}", ch as u32));
            }
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Put a rendered overlay wrapper in the browser's top layer while it is
/// open. Unlike `position: fixed`, a top-layer element cannot be clipped by a
/// scrolling table, card, route-transition pane, or transformed ancestor.
/// The wrapper remains open for `exit_delay` so its children can finish their
/// exit animations before the browser hides it.
pub(crate) fn use_top_layer(open: ReadSignal<bool>, id: String, exit_delay: Duration) {
    use_top_layer_with_priority(open, id, exit_delay, 0);
}

pub(crate) fn use_top_layer_with_priority(
    open: ReadSignal<bool>,
    id: String,
    exit_delay: Duration,
    priority: u8,
) {
    let config = use_synced_signal((open, id, exit_delay, priority));
    use_effect(move || {
        let (open, id, exit_delay, priority) = config();
        let is_open = open();
        let script = format!(
            "{}\nwindow[Symbol.for('g3-ui.layers')].present({id}, {is_open}, {delay}, {priority});",
            include_str!("layers.js"),
            id = js_string(&id),
            is_open = is_open,
            delay = exit_delay.as_millis(),
        );
        let _ = document::eval(&script);
    });
    use_drop(move || {
        let id = js_string(&config.peek().1);
        document::eval(&format!(
            "window[Symbol.for('g3-ui.layers')]?.remove({id});"
        ));
    });
}

#[derive(Clone, PartialEq)]
struct OverlayEntry {
    id: String,
    dismiss: Callback<()>,
}

/// The app's currently open sheets, dialogs and popovers, in presentation order.
/// Use [`use_overlay_controller`] to obtain it for Android Back handling.
#[derive(Clone, Copy)]
pub struct OverlayController {
    entries: Signal<Vec<OverlayEntry>>,
}

impl OverlayController {
    /// Number of open blocking or dismissible surfaces. Toasts and permanent
    /// navigation drawers are excluded. Reading this subscribes the caller.
    pub fn count(self) -> usize {
        self.entries.read().len()
    }

    /// Handle Back by dismissing the most recently opened surface. Returns
    /// whether an overlay consumed Back. A nondismissible modal consumes it
    /// without closing, so Back cannot accidentally close the surface below.
    pub fn dismiss_top(self) -> bool {
        let dismiss = self.entries.peek().last().map(|entry| entry.dismiss);
        if let Some(dismiss) = dismiss {
            dismiss.call(());
            true
        } else {
            false
        }
    }
}

/// Access the enclosing app's overlay stack. Call once in a component, then
/// use `dismiss_top()` from a Back handler before navigating the app.
pub fn use_overlay_controller() -> OverlayController {
    use_hook(|| {
        try_consume_context::<OverlayController>().unwrap_or_else(|| {
            dioxus::core::provide_root_context(OverlayController {
                entries: Signal::new_in_scope(Vec::new(), ScopeId::ROOT),
            })
        })
    })
}

pub(crate) fn use_overlay_registration(open: ReadSignal<bool>, id: String, dismiss: Callback<()>) {
    let mut controller = use_overlay_controller();
    let config = use_synced_signal((open, id));
    let held = use_hook(|| std::rc::Rc::new(std::cell::RefCell::new(None::<String>)));
    {
        let held = held.clone();
        use_effect(move || {
            let (open, id) = config();
            let want = open();
            if held
                .borrow()
                .as_ref()
                .is_some_and(|previous| !want || *previous != id)
            {
                let previous = held.borrow_mut().take();
                controller
                    .entries
                    .write()
                    .retain(|entry| Some(&entry.id) != previous.as_ref());
            }
            if want && held.borrow().is_none() {
                controller.entries.write().push(OverlayEntry {
                    id: id.clone(),
                    dismiss,
                });
                *held.borrow_mut() = Some(id);
            }
        });
    }
    use_drop(move || {
        if let Some(id) = held.borrow_mut().take() {
            controller.entries.write().retain(|entry| entry.id != id);
        }
    });
}

/// Playground previews render overlays inside a device frame, where they must
/// not lock the real page around the frame.
#[derive(Clone, Copy)]
struct DisableBodyScrollLock;

#[cfg(feature = "playground")]
pub(crate) fn disable_body_scroll_lock_for_subtree() {
    provide_context(DisableBodyScrollLock);
}

thread_local! {
    /// How many overlays currently hold the page scroll lock. The page is
    /// locked while this is above zero, so closing one of two stacked overlays
    /// leaves the page locked for the other.
    static LOCKS: Cell<usize> = const { Cell::new(0) };
}

/// Hold the page scroll lock while `locked` is true, and release it on unmount.
pub(crate) fn use_lock_body_scroll(locked: ReadSignal<bool>) {
    let locked = use_synced_signal(locked);
    let disabled = try_consume_context::<DisableBodyScrollLock>().is_some();
    let held = use_hook(|| std::rc::Rc::new(Cell::new(false)));
    {
        let held = held.clone();
        use_effect(move || {
            let want = locked()() && !disabled;
            if held.get() != want {
                held.set(want);
                adjust_lock(want);
            }
        });
    }
    use_drop(move || {
        if held.get() {
            adjust_lock(false);
        }
    });
}

fn adjust_lock(acquire: bool) {
    let count = LOCKS.with(|locks| {
        let next = if acquire {
            locks.get() + 1
        } else {
            locks.get().saturating_sub(1)
        };
        locks.set(next);
        next
    });
    // Only the transitions between locked and unlocked touch the page.
    if (acquire && count == 1) || (!acquire && count == 0) {
        set_page_locked(count > 0);
    }
}

#[cfg(target_arch = "wasm32")]
fn set_page_locked(locked: bool) {
    let body = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.body());
    if let Some(body) = body {
        let _ = body
            .class_list()
            .toggle_with_force("g3-overlay-scroll-locked", locked);
    }
}

/// Desktop and mobile render into a webview, reached through `eval`.
#[cfg(not(target_arch = "wasm32"))]
fn set_page_locked(locked: bool) {
    let _ = document::eval(&format!(
        "document.body && document.body.classList.toggle('g3-overlay-scroll-locked', {locked});"
    ));
}

const FOCUS_SCRIPT: &str = include_str!("focus.js");

/// Keyboard behaviour for an open dialog or sheet.
///
/// While `open` is true: focus moves into the element with id `id`, Escape
/// calls `on_escape`, and with `trap` Tab cycles inside the element. When
/// `open` turns false, focus returns to whatever had it before.
///
/// The element needs `tabindex: "-1"` so it can take focus when it has no
/// focusable children.
pub(crate) fn use_overlay_focus(
    open: ReadSignal<bool>,
    id: String,
    trap: bool,
    on_escape: Callback<()>,
) {
    use_overlay_focus_with(open, id, trap, None, on_escape);
}

/// [`use_overlay_focus`], plus arrow-key movement between the elements that
/// match `roving`, as a listbox or menu needs. Focus starts on the selected
/// one.
pub(crate) fn use_overlay_focus_with(
    open: ReadSignal<bool>,
    id: String,
    trap: bool,
    roving: Option<&'static str>,
    on_escape: Callback<()>,
) {
    let config = use_synced_signal((id.clone(), trap, open));
    use_effect(move || {
        let (id, trap, open) = config();
        let _ = open();
        document::eval(&format!(
            "const el = document.getElementById({}); if (el) el.dataset.g3TrapFocus = {};",
            js_string(&id),
            js_string(if trap { "true" } else { "false" })
        ));
    });
    let script = open().then(|| {
        FOCUS_SCRIPT.replace("__ID__", &js_string(&id)).replace(
            "__ROVING__",
            &roving.map_or_else(|| "null".to_string(), js_string),
        )
    });
    super::script::use_dom_script::<String>(script, move |message| {
        if message == "escape" {
            on_escape.call(());
        }
    });
}
