//! What bottom sheets, side sheets, and navigation drawers share: the
//! entrance, the backdrop, scroll locking, focus handling, and the app-wide
//! count of open sheets.
use super::overlay::{use_lock_body_scroll, use_overlay_focus};
use crate::state::{use_element_id, use_synced_signal};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Dismissible sheets open right now. Global on purpose: the question it
/// answers, "is a sheet up?", is app-wide.
static OPEN_SHEETS: GlobalSignal<usize> = Signal::global(|| 0);

/// How many dismissible sheets are open in the app.
///
/// Reactive: a component that reads it re-renders as sheets open and close.
/// Meant for app-level dismissal such as the Android Back button, which must
/// know a sheet is up even when the page that opened it keeps its `open`
/// signal private. Navigation drawers are not counted.
///
/// To close the topmost sheet, click its `[data-g3-sheet-dismiss]` control;
/// every open dismissible sheet renders one, whatever its backdrop. See the
/// crate docs for an example.
pub fn open_sheet_count() -> usize {
    OPEN_SHEETS()
}

/// What sits behind an open sheet.
///
/// Ionic's `showBackdrop` and `backdropDismiss` joined: the scrim is what a
/// tap outside the sheet lands on, so removing one removes the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SheetBackdrop {
    /// A dimming scrim. Tapping it closes the sheet, the page behind cannot
    /// scroll, and keyboard focus stays inside the sheet.
    #[default]
    Dismiss,
    /// No scrim, for a sheet over a page that stays in use, such as comments
    /// under a playing video. The page stays interactive and scrollable; the
    /// sheet closes from its handle, Escape, or its owner.
    None,
}

/// A side of the screen, in reading direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SheetEdge {
    /// The leading edge: left in left-to-right documents.
    #[default]
    Start,
    /// The trailing edge: right in left-to-right documents.
    End,
}

impl SheetEdge {
    fn as_str(self) -> &'static str {
        match self {
            SheetEdge::Start => "start",
            SheetEdge::End => "end",
        }
    }
}

/// How a [`SideSheet`](crate::SideSheet) moves the page beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SideSheetBehavior {
    /// The sheet slides over the page.
    #[default]
    Overlay,
    /// The sheet slides in and pushes the page aside by the same distance.
    Push,
    /// The sheet stays still and the page slides away to reveal it.
    Reveal,
}

#[derive(Clone, PartialEq)]
pub(crate) enum FrameKind {
    Bottom {
        draggable: bool,
        detents: Vec<f64>,
        detent: Signal<usize>,
        backdrop_detent: usize,
        max_height: Option<String>,
    },
    Side {
        edge: SheetEdge,
        behavior: SideSheetBehavior,
        width: Option<String>,
    },
    Drawer {
        edge: SheetEdge,
        width: Option<String>,
    },
}

const DISMISS_DISTANCE: f64 = 96.0;

/// Pointer and keyboard behaviour of a bottom sheet's handle. Messages back to
/// Rust: `-1` dismiss, `-2` cycle detents, `n >= 0` settle on detent `n`.
const DRAG_SCRIPT: &str = r#"
const dialog = document.getElementById(__DIALOG__);
const handle = document.getElementById(__HANDLE__);
if (dialog && handle && handle.dataset.g3Bound !== "true") {
    handle.dataset.g3Bound = "true";
    // Read on each gesture, so a sheet whose detents change keeps up.
    const readDetents = () => JSON.parse(dialog.dataset.detents || "[]");
    let detents = readDetents();
    const DISMISS = __DISMISS__;
    let dragging = false;
    let dragged = false;
    let startY = 0;
    let deltaY = 0;
    let startHeight = 0;
    let fraction = 1;

    handle.addEventListener("pointerdown", (event) => {
        dragging = true;
        dragged = false;
        detents = readDetents();
        startY = event.clientY;
        deltaY = 0;
        startHeight = dialog.getBoundingClientRect().height;
        fraction = parseFloat(dialog.style.getPropertyValue("--g3-sheet-detent")) || 1;
        dialog.dataset.dragging = "true";
        try { handle.setPointerCapture(event.pointerId); } catch (error) {}
        event.preventDefault();
    });

    handle.addEventListener("pointermove", (event) => {
        if (!dragging) return;
        deltaY = event.clientY - startY;
        if (Math.abs(deltaY) > 4) dragged = true;
        if (detents.length > 0) {
            dialog.style.height = Math.max(0, startHeight - deltaY) + "px";
        } else {
            dialog.style.setProperty("--g3-sheet-drag-y", Math.max(0, deltaY) + "px");
        }
        event.preventDefault();
    });

    const end = () => {
        if (!dragging) return;
        dragging = false;
        if (!dragged) {
            delete dialog.dataset.dragging;
            dialog.style.removeProperty("height");
            dialog.style.setProperty("--g3-sheet-drag-y", "0px");
            return;
        }
        if (detents.length === 0) {
            delete dialog.dataset.dragging;
            dialog.style.setProperty("--g3-sheet-drag-y", "0px");
            if (deltaY > DISMISS) dioxus.send(-1);
            return;
        }
        const ceiling = startHeight / fraction;
        const height = startHeight - deltaY;
        const lowest = detents[0] * ceiling;
        if (height < lowest - DISMISS) {
            delete dialog.dataset.dragging;
            dialog.style.removeProperty("height");
            dioxus.send(-1);
            return;
        }
        let nearest = 0;
        detents.forEach((detent, index) => {
            if (Math.abs(detent * ceiling - height) < Math.abs(detents[nearest] * ceiling - height)) {
                nearest = index;
            }
        });
        // Synchronize the CSS-sized surface before releasing the temporary
        // pixel height. Otherwise the Rust update can land a frame later,
        // leaving the visible sheet and its outside-click hit area briefly at
        // different detents.
        dialog.style.setProperty("--g3-sheet-detent", detents[nearest]);
        dialog.style.removeProperty("height");
        dialog.style.setProperty("--g3-sheet-drag-y", "0px");
        // Flush the new geometry before the backdrop can receive another
        // pointer. This keeps the sheet's visual edge and hit-test edge in
        // lockstep after moving from a larger detent to a smaller one.
        dialog.getBoundingClientRect();
        delete dialog.dataset.dragging;
        dioxus.send(nearest);
    };
    handle.addEventListener("pointerup", end);
    handle.addEventListener("pointercancel", end);

    // Keyboard and plain taps: cycle detents, or close a sheet without any.
    handle.addEventListener("click", (event) => {
        if (dragged) {
            dragged = false;
            event.preventDefault();
            return;
        }
        dioxus.send(readDetents().length > 0 ? -2 : -1);
    });
}
"#;

#[component]
pub(crate) fn SheetFrame(
    open: Signal<bool>,
    kind: FrameKind,
    backdrop: SheetBackdrop,
    title: Option<String>,
    aria_label: Option<String>,
    on_dismiss: Option<EventHandler<()>>,
    padding: Option<bool>,
    mode: Option<ComponentMode>,
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("sheet", None);
    let title_id = format!("{id}-title");
    let handle_id = format!("{id}-handle");
    let is_drawer = matches!(kind, FrameKind::Drawer { .. });
    let is_bottom = matches!(kind, FrameKind::Bottom { .. });
    let has_backdrop = !is_drawer && backdrop == SheetBackdrop::Dismiss;
    // A sheet resting below its backdrop detent leaves the page usable.
    let below_backdrop = match &kind {
        FrameKind::Bottom {
            detents,
            detent,
            backdrop_detent,
            ..
        } if !detents.is_empty() => detent() < *backdrop_detent,
        _ => false,
    };
    let modal = has_backdrop && !below_backdrop;
    let has_backdrop_signal = use_synced_signal(modal);
    let is_open = open();

    let dismiss = use_callback(move |()| {
        let mut open = open;
        open.set(false);
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.call(());
        }
    });

    // A surface that mounts open is part of the initial layout (a drawer,
    // usually) and paints open without an entrance. Opening it later is a
    // presentation and animates.
    let mut ever_opened = use_signal(|| is_open);
    let mut has_been_closed = use_signal(|| !is_open);
    // Cleared when the entrance finishes, so a detent change while open
    // animates its height again.
    let mut entered = use_signal(|| false);
    use_effect(move || {
        if open() {
            ever_opened.set(true);
        } else {
            has_been_closed.set(true);
            entered.set(false);
        }
    });

    let locked = use_memo(move || open() && has_backdrop_signal());
    use_lock_body_scroll(locked.into());

    if !is_drawer {
        use_overlay_focus(open.into(), id.clone(), has_backdrop, dismiss);
        use_open_count(open);
    }

    let (draggable, detent_signal, count) = match &kind {
        FrameKind::Bottom {
            draggable: true,
            detents,
            detent,
            ..
        } => (true, Some(*detent), detents.len()),
        _ => (false, None, 0),
    };
    let detent_count = use_synced_signal(count);
    {
        let id = id.clone();
        let handle_id = handle_id.clone();
        use_effect(move || {
            if !draggable || !open() {
                return;
            }
            let script = DRAG_SCRIPT
                .replace("__DIALOG__", &format!("{id:?}"))
                .replace("__HANDLE__", &format!("{handle_id:?}"))
                .replace("__DISMISS__", &DISMISS_DISTANCE.to_string());
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(message) = eval.recv::<i64>().await {
                    match (message, detent_signal) {
                        (-1, _) => dismiss.call(()),
                        (-2, Some(mut detent)) => {
                            let count = detent_count.peek().max(1);
                            let next = (*detent.peek() + 1) % count;
                            detent.set(next);
                        }
                        (index, Some(mut detent)) if index >= 0 => {
                            detent.set(index as usize);
                        }
                        _ => {}
                    }
                }
            });
        });
    }

    if !is_open && !ever_opened() {
        return rsx! {};
    }

    let state = if is_open { "open" } else { "closed" };
    let enter_cls = if is_open && has_been_closed() && !entered() {
        "g3-sheet-enter"
    } else {
        ""
    };
    let (kind_cls, style, detent_attr, handle) = match &kind {
        FrameKind::Bottom {
            draggable,
            detents,
            detent,
            max_height,
            ..
        } => {
            let mut style = String::new();
            if let Some(max_height) = max_height {
                style.push_str(&format!("--g3-sheet-max-height: {max_height};"));
            }
            let detent_attr = if detents.is_empty() {
                None
            } else {
                let index = detent().min(detents.len() - 1);
                style.push_str(&format!("--g3-sheet-detent: {};", detents[index]));
                Some(index.to_string())
            };
            (
                "g3-sheet g3-sheet-bottom".to_string(),
                style,
                detent_attr,
                *draggable,
            )
        }
        FrameKind::Side {
            edge,
            behavior,
            width,
        } => {
            let behavior_cls = behavior_class(*behavior);
            (
                format!(
                    "g3-sheet g3-sheet-side g3-sheet-{} {behavior_cls}",
                    edge.as_str()
                ),
                width_style(width),
                None,
                false,
            )
        }
        FrameKind::Drawer { edge, width } => (
            format!("g3-drawer g3-drawer-{}", edge.as_str()),
            width_style(width),
            None,
            false,
        ),
    };
    // The scrim only needs the behaviour: the side-sheet classes would give
    // it the sheet's slide instead of a fade.
    let behavior_cls = match &kind {
        FrameKind::Side { behavior, .. } => behavior_class(*behavior),
        _ => "",
    };
    let surface_cls = classes([
        kind_cls.as_str(),
        if is_drawer {
            ""
        } else {
            mode.pick("g3-sheet-ios", "g3-sheet-md")
        },
        enter_cls,
    ]);
    let label = aria_label.or_else(|| {
        title.is_none().then(|| {
            if is_drawer {
                strings.navigation_drawer.clone()
            } else {
                strings.sheet.clone()
            }
        })
    });
    let labelledby = title.as_ref().map(|_| title_id.clone());

    rsx! {
        if has_backdrop {
            button {
                r#type: "button",
                class: classes(["g3-sheet-backdrop", behavior_cls, enter_cls]),
                "data-state": if modal && is_open { "open" } else { "closed" },
                // The scrim is a pointer target only. Keyboard users close the
                // sheet with Escape, so it stays out of the tab order.
                tabindex: "-1",
                aria_hidden: "true",
                // Dismiss on click, not pointerdown: closing on the press
                // removes the scrim before the press ends, and the click lands
                // on whatever was underneath.
                onclick: move |_| dismiss.call(()),
            }
        }
        div {
            id: id.clone(),
            class: merge_classes(surface_cls, class.as_deref()),
            style,
            role: if is_drawer { "navigation" } else { "dialog" },
            aria_modal: (!is_drawer).then(|| modal.to_string()),
            aria_label: label,
            aria_labelledby: labelledby,
            aria_hidden: (!is_open).then_some("true"),
            inert: (!is_open).then_some(true),
            tabindex: (!is_drawer).then_some("-1"),
            "data-state": state,
            "data-detent": detent_attr,
            onanimationend: move |event| {
                // Animations inside the sheet bubble here too.
                if event.data().animation_name().starts_with("g3-sheet-enter") && !*entered.peek() {
                    entered.set(true);
                }
            },
            "data-detents": match &kind {
                FrameKind::Bottom { detents, .. } if !detents.is_empty() => Some(format!("{detents:?}")),
                _ => None,
            },
            if handle && is_bottom {
                button {
                    id: handle_id,
                    r#type: "button",
                    class: "g3-sheet-handle",
                    aria_label: strings.sheet_handle,
                    span { class: "g3-sheet-handle-bar" }
                }
            }
            // A close control for app-level dismissal such as Android Back.
            // Hidden, and never in the tab order.
            if is_open && !is_drawer {
                button {
                    r#type: "button",
                    class: "g3-sheet-dismiss",
                    tabindex: "-1",
                    aria_hidden: "true",
                    "data-g3-sheet-dismiss": "",
                    onclick: move |_| dismiss.call(()),
                    "{strings.close}"
                }
            }
            if let Some(title) = title {
                div { class: "g3-sheet-header",
                    h2 { id: title_id, class: "g3-sheet-title", "{title}" }
                }
            }
            div {
                class: "g3-sheet-content",
                "data-padding": padding.unwrap_or(true).to_string(),
                {children}
            }
        }
    }
}

fn behavior_class(behavior: SideSheetBehavior) -> &'static str {
    match behavior {
        SideSheetBehavior::Overlay => "",
        SideSheetBehavior::Push => "g3-sheet-push",
        SideSheetBehavior::Reveal => "g3-sheet-reveal",
    }
}

fn width_style(width: &Option<String>) -> String {
    width
        .as_ref()
        .map(|width| format!("--g3-side-sheet-width: {width};"))
        .unwrap_or_default()
}

/// Keep [`OPEN_SHEETS`] in step with this sheet, including when it unmounts
/// while open because its page navigated away.
fn use_open_count(open: Signal<bool>) {
    let counted = use_hook(|| Rc::new(Cell::new(false)));
    {
        let counted = counted.clone();
        use_effect(move || {
            let is_open = open();
            if counted.get() != is_open {
                counted.set(is_open);
                let mut count = OPEN_SHEETS.write();
                *count = if is_open {
                    *count + 1
                } else {
                    count.saturating_sub(1)
                };
            }
        });
    }
    use_drop(move || {
        if counted.get() {
            let mut count = OPEN_SHEETS.write();
            *count = count.saturating_sub(1);
        }
    });
}
