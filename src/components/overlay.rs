//! Behaviour shared by overlays: page scroll locking and keyboard focus.
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
    use_effect(move || {
        let is_open = open();
        let script = format!(
            r#"
const layer = document.getElementById({id});
if (layer && typeof layer.showPopover === "function") {{
    clearTimeout(layer.g3HideTimer);
    if ({is_open}) {{
        if (!layer.matches(":popover-open")) layer.showPopover();
    }} else if (layer.matches(":popover-open")) {{
        layer.g3HideTimer = setTimeout(() => {{
            if (layer.isConnected && layer.matches(":popover-open")) layer.hidePopover();
        }}, {delay});
    }}
}} else if (layer) {{
    // Older embedded WebViews (notably the minimum supported iOS versions)
    // do not have the Popover API. Removing the attribute restores the
    // ordinary fixed-position fallback instead of leaving the wrapper hidden
    // by the user-agent `[popover]` rule.
    layer.removeAttribute("popover");
}}
"#,
            id = js_string(&id),
            is_open = is_open,
            delay = exit_delay.as_millis(),
        );
        let _ = document::eval(&script);
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
    let disabled = try_consume_context::<DisableBodyScrollLock>().is_some();
    let held = use_hook(|| std::rc::Rc::new(Cell::new(false)));
    {
        let held = held.clone();
        use_effect(move || {
            let want = locked() && !disabled;
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

const FOCUS_SCRIPT: &str = r#"
const el = document.getElementById(__ID__);
if (el) {
    const previous = document.activeElement;
    const selector = 'a[href],button:not([disabled]),input:not([disabled]),select:not([disabled]),textarea:not([disabled]),[tabindex]:not([tabindex="-1"])';
    const focusables = () => [...el.querySelectorAll(selector)]
        .filter((node) => !node.hidden && !node.closest('[inert]') && node.getClientRects().length > 0);
    const roving = __ROVING__;
    const onKey = (event) => {
        if (event.key === 'Escape') {
            event.stopPropagation();
            dioxus.send('escape');
            return;
        }
        // Typing a letter moves to the next item starting with it.
        if (roving && event.key.length === 1 && event.key.trim() !== "" && !event.ctrlKey && !event.metaKey && !event.altKey) {
            const items = [...el.querySelectorAll(roving)].filter((node) => !node.disabled && node.getAttribute('aria-disabled') !== 'true');
            const current = items.indexOf(document.activeElement);
            const key = event.key.toLowerCase();
            const ordered = items.slice(current + 1).concat(items.slice(0, current + 1));
            const match = ordered.find((node) => node.textContent.trim().toLowerCase().startsWith(key));
            if (match) {
                event.preventDefault();
                match.focus();
            }
            return;
        }
        if (roving && ['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
            const items = [...el.querySelectorAll(roving)].filter((node) => !node.disabled && node.getAttribute('aria-disabled') !== 'true');
            if (items.length === 0) return;
            event.preventDefault();
            const current = items.indexOf(document.activeElement);
            let next = 0;
            if (event.key === 'ArrowDown') next = current < 0 ? 0 : (current + 1) % items.length;
            if (event.key === 'ArrowUp') next = current <= 0 ? items.length - 1 : current - 1;
            if (event.key === 'End') next = items.length - 1;
            items[next].focus();
            return;
        }
        if (event.key !== 'Tab' || !__TRAP__) return;
        const items = focusables();
        if (items.length === 0) {
            event.preventDefault();
            el.focus();
            return;
        }
        const first = items[0];
        const last = items[items.length - 1];
        if (event.shiftKey && (document.activeElement === first || document.activeElement === el)) {
            event.preventDefault();
            last.focus();
        } else if (!event.shiftKey && document.activeElement === last) {
            event.preventDefault();
            first.focus();
        }
    };
    el.addEventListener('keydown', onKey);
    requestAnimationFrame(() => {
        if (el.contains(document.activeElement)) return;
        const items = focusables();
        const selected = roving && el.querySelector(roving + '[aria-selected="true"], ' + roving + '[aria-checked="true"]');
        const target = items.find((node) => node.hasAttribute('autofocus')) || selected || items[0] || el;
        target.focus({ preventScroll: true });
    });
    await dioxus.recv();
    el.removeEventListener('keydown', onKey);
    if (previous && previous.isConnected && typeof previous.focus === 'function' && el.contains(document.activeElement)) {
        previous.focus({ preventScroll: true });
    }
}
"#;

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
    let session = use_hook(|| std::rc::Rc::new(Cell::new(None::<document::Eval>)));
    {
        let session = session.clone();
        use_effect(move || {
            let is_open = open();
            match (is_open, session.get()) {
                (true, None) => {
                    let script = FOCUS_SCRIPT
                        .replace("__ID__", &js_string(&id))
                        .replace("__TRAP__", if trap { "true" } else { "false" })
                        .replace(
                            "__ROVING__",
                            &roving.map_or_else(|| "null".to_string(), js_string),
                        );
                    let eval = document::eval(&script);
                    session.set(Some(eval));
                    spawn(async move {
                        let mut reader = eval;
                        while let Ok(message) = reader.recv::<String>().await {
                            if message == "escape" {
                                on_escape.call(());
                            }
                        }
                    });
                }
                (false, Some(eval)) => {
                    session.set(None);
                    let _ = eval.send("close");
                }
                _ => {}
            }
        });
    }
    use_drop(move || {
        if let Some(eval) = session.take() {
            let _ = eval.send("close");
        }
    });
}
