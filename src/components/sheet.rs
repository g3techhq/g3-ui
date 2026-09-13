//! Sheet component - bottom and side sheet with drag-to-dismiss and platform styling.
use super::overlay_scroll::use_lock_body_scroll;
use super::sheet_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
#[cfg(feature = "playground")]
use dioxus_icons::lucide::Menu;
use std::sync::atomic::{AtomicU64, Ordering};
const SHEET_DISMISS_DISTANCE: f64 = 96.0;
static SHEET_INSTANCE_ID: AtomicU64 = AtomicU64::new(0);
/// Dismissible sheets open right now, kept by every `Sheet` as it opens, closes
/// and unmounts. Global on purpose: the question it answers is app-wide.
static OPEN_SHEETS: GlobalSignal<usize> = Signal::global(|| 0);
/// How many dismissible sheets are open in this app.
///
/// Reactive, so a component that reads it re-renders as sheets open and close.
/// Meant for app-level dismissal such as Android Back, which has to know a
/// sheet is up even when the page that opened it holds its `is_open` signal
/// privately. A persistent `Menu` side sheet is not a dismissible layer and is
/// not counted. Close the topmost one by clicking its `[data-g3-sheet-dismiss]`
/// control, which every open dismissible sheet renders whatever its backdrop.
pub fn open_sheet_count() -> usize {
    OPEN_SHEETS()
}
const SHEET_DRAG_SCRIPT: &str = r#"
const dialog = document.getElementById("__DIALOG_ID__");
const handle = document.getElementById("__HANDLE_ID__");
if (dialog && handle && handle.dataset.g3SheetDragBound !== "true") {
    handle.dataset.g3SheetDragBound = "true";

    let dragging = false;
    let startY = 0;
    let deltaY = 0;
    const DISMISS_DISTANCE = __DISMISS_DISTANCE__;

    const onDown = (e) => {
        dragging = true;
        startY = e.clientY;
        deltaY = 0;
        dialog.style.setProperty("transition", "none");
        dialog.style.setProperty("touch-action", "none");
        try { handle.setPointerCapture(e.pointerId); } catch (err) {}
        e.preventDefault();
    };

    const onMove = (e) => {
        if (!dragging) return;
        deltaY = Math.max(0, e.clientY - startY);
        dialog.style.setProperty("--g3-sheet-drag-y", deltaY + "px");
        e.preventDefault();
    };

    const onEnd = () => {
        if (!dragging) return;
        dragging = false;
        dialog.style.removeProperty("transition");
        dialog.style.removeProperty("touch-action");
        dialog.style.setProperty("--g3-sheet-drag-y", "0px");
        if (deltaY > DISMISS_DISTANCE) {
            dioxus.send(true);
        }
    };

    handle.addEventListener("pointerdown", onDown);
    handle.addEventListener("pointermove", onMove);
    handle.addEventListener("pointerup", onEnd);
    handle.addEventListener("pointercancel", onEnd);
}
"#;
/// How a side sheet interacts with the app content beside it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SideSheetType {
    /// Slide above the app content without moving it.
    #[default]
    Overlay,
    /// Slide in while moving the app content by the same distance.
    Push,
    /// Stay beneath the app content while the content moves away to reveal it.
    Reveal,
    /// Reserve space beside the complete app page as persistent navigation.
    Menu,
}
/// Which edge a sheet uses and, for side sheets, how it affects app content.
///
/// `Push`, `Reveal`, and `Menu` side sheets should be direct children of
/// `AppWrapper`, beside one root content element, matching Ionic's
/// menu/content structure.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetPlacement {
    /// Rises from the bottom - the standard mobile action sheet.
    #[default]
    Bottom,
    /// Slides in from the leading edge, as a navigation drawer does.
    Left(SideSheetType),
    /// Slides in from the trailing edge, for inspectors and filters.
    Right(SideSheetType),
}
/// What sits behind an open sheet.
///
/// The two useful halves of Ionic's `showBackdrop` and `backdropDismiss`,
/// joined: the scrim is the dismiss target, so removing one removes the other.
/// A `Menu` side sheet never renders a backdrop, whatever this says.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SheetBackdrop {
    /// A dimming scrim over the page. Tapping it closes the sheet, and the page
    /// behind does not scroll while the sheet is open.
    #[default]
    Dismiss,
    /// No scrim, for a sheet that sits over a page the viewer is still using -
    /// comments below a playing video, say. The page stays visible, interactive
    /// and scrollable, and nothing outside the sheet closes it: a bottom sheet
    /// is dismissed by dragging its handle. With `draggable: false` as well, only
    /// the owner setting `is_open` to false closes it.
    None,
}
#[component]
pub fn Sheet(
    mut is_open: Signal<bool>,
    placement: Option<SheetPlacement>,
    draggable: Option<bool>,
    backdrop: Option<SheetBackdrop>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let placement = placement.unwrap_or_default();
    let is_draggable = draggable.unwrap_or(true);
    let instance_id = use_hook(|| SHEET_INSTANCE_ID.fetch_add(1, Ordering::Relaxed));
    let mode_cls = match mode {
        ComponentMode::Ios => s::SHEET_IOS,
        ComponentMode::Md => s::SHEET_MD,
    };
    let (placement_cls, side_type_cls) = match placement {
        SheetPlacement::Bottom => (s::SHEET_BOTTOM, ""),
        SheetPlacement::Left(side_type) => (s::SHEET_LEFT, side_sheet_type_class(side_type)),
        SheetPlacement::Right(side_type) => (s::SHEET_RIGHT, side_sheet_type_class(side_type)),
    };
    let side_cls = if side_type_cls.is_empty() {
        ""
    } else {
        s::SHEET_SIDE
    };
    let is_menu = matches!(
        placement,
        SheetPlacement::Left(SideSheetType::Menu) | SheetPlacement::Right(SideSheetType::Menu)
    );
    let sheet_cls = format!(
        "{} {mode_cls} {placement_cls} {side_cls} {side_type_cls}",
        s::SHEET,
    );
    let has_handle = matches!(placement, SheetPlacement::Bottom);
    let dialog_id = format!("g3-sheet-{instance_id}");
    let handle_id = format!("g3-sheet-handle-{instance_id}");
    let is_open_now = is_open();
    let mut ever_opened = use_signal(|| is_open_now);
    // A sheet that mounts open is part of the initial layout - a persistent
    // `Menu` rail is the usual case - so it is painted open with no entrance.
    // Once it has been closed, opening it again is a presentation and animates.
    let mut has_been_closed = use_signal(|| !is_open_now);
    use_effect(move || {
        if is_open() {
            ever_opened.set(true);
        } else {
            has_been_closed.set(true);
        }
    });
    {
        let dialog_id = dialog_id.clone();
        let handle_id = handle_id.clone();
        use_effect(move || {
            if !(is_open() && is_draggable && has_handle) {
                return;
            }
            let script = SHEET_DRAG_SCRIPT
                .replace("__DIALOG_ID__", &dialog_id)
                .replace("__HANDLE_ID__", &handle_id)
                .replace("__DISMISS_DISTANCE__", &SHEET_DISMISS_DISTANCE.to_string());
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(true) = eval.recv::<bool>().await {
                    is_open.set(false);
                }
            });
        });
    }
    let has_backdrop = backdrop.unwrap_or_default() == SheetBackdrop::Dismiss;
    // Without a backdrop the page behind is meant to stay usable, so it keeps
    // its scroll too. The hook takes a signal, and handing it `is_open` would
    // lock the page whatever the backdrop is.
    let mut scroll_locked = use_signal(|| is_open_now && has_backdrop);
    use_effect(move || {
        let locked = is_open() && has_backdrop;
        if *scroll_locked.peek() != locked {
            scroll_locked.set(locked);
        }
    });
    use_lock_body_scroll(scroll_locked);
    // Whether this sheet is in `OPEN_SHEETS`. A cell rather than a signal
    // because the drop below runs as the scope is torn down, and it must not
    // read state that is going away with it.
    let counted = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(false)));
    {
        let counted = counted.clone();
        use_effect(move || {
            let open = is_open() && !is_menu;
            if counted.get() != open {
                counted.set(open);
                let mut count = OPEN_SHEETS.write();
                *count = if open {
                    *count + 1
                } else {
                    count.saturating_sub(1)
                };
            }
        });
    }
    // A sheet unmounted while open - its page navigated away - must not leave
    // the count claiming a layer that no longer exists.
    use_drop(move || {
        if counted.get() {
            let mut count = OPEN_SHEETS.write();
            *count = count.saturating_sub(1);
        }
    });
    if !is_open_now && !ever_opened() {
        return rsx! {};
    }
    let visual_open = is_open_now;
    // The entrance is a keyframe animation rather than a transition between two
    // painted states. A transition needs its start value to have been painted,
    // which for a sheet inserted on open it never has - that is what the old
    // `requestAnimationFrame` round trip through `document::eval` was buying, at
    // the cost of a bridge hop before anything moved. Keyframes run on the frame
    // the class lands, so opening is immediate on web, desktop and mobile alike.
    // The exit stays a transition: the open state has been painted by then.
    let enter_cls = if is_open_now && has_been_closed() {
        s::ENTER
    } else {
        ""
    };
    let backdrop_cls = format!(
        "{} {} {enter_cls} {placement_cls} {side_cls} {side_type_cls}",
        s::BACKDROP,
        if visual_open {
            "g3-sheet-backdrop-open"
        } else {
            "g3-sheet-backdrop-closed"
        },
    );
    let state_cls = if visual_open {
        s::STATE_OPEN
    } else {
        s::STATE_CLOSED
    };
    let state_cls = format!("{state_cls} {enter_cls}");
    rsx! {
        if !is_menu && has_backdrop {
            button {
                r#type: "button",
                aria_label: "Close sheet",
                aria_hidden: (!is_open_now).to_string(),
                tabindex: if is_open_now { "0" } else { "-1" },
                class: backdrop_cls,
                // Dismiss on click, never on pointerdown. Closing on the press
                // removes this button before the press completes, so the click
                // is delivered to whatever the scrim was covering - tapping to
                // dismiss also activated the control underneath.
                //
                // Waiting for the click costs nothing: it fires for touch and
                // mouse alike, and a press that ends outside the scrim is a
                // cancelled gesture that should leave the sheet open.
                onclick: move |_| is_open.set(false),
            }
        }
        div {
            id: dialog_id,
            role: if is_menu { "navigation" } else { "dialog" },
            // Modal only while the page behind is shut off. A sheet with no
            // backdrop leaves that page in use, and announcing it as modal
            // would hide the page from assistive technology for no reason.
            aria_modal: (!is_menu && has_backdrop).to_string(),
            aria_label: if is_menu { "Menu" } else { "Sheet" },
            aria_hidden: (!
                    is_open_now).to_string(),
            inert: (!is_open_now).then(|| "".to_string()),
            class: merge_classes(format!("{sheet_cls} {state_cls}"), class.as_deref()),
            if is_draggable && has_handle {
                button {
                    id: handle_id,
                    r#type: "button",
                    class: s::HANDLE_WRAP_IOS,
                    aria_label: "Sheet handle",
                    div { class: s::HANDLE_IOS }
                }
            }
            // A close control that does not depend on the backdrop, for an app's
            // own dismissal paths - Android Back above all. A sheet with
            // `SheetBackdrop::None` has no scrim to click, so this is the one
            // hook every dismissible sheet shares. Hidden and out of the tab
            // order: nothing on screen reaches it.
            if is_open_now && !is_menu {
                button {
                    r#type: "button",
                    hidden: true,
                    tabindex: "-1",
                    aria_hidden: "true",
                    "data-g3-sheet-dismiss": "",
                    onclick: move |_| is_open.set(false),
                }
            }
            div { class: s::CONTENT, {children} }
        }
    }
}
fn side_sheet_type_class(side_type: SideSheetType) -> &'static str {
    match side_type {
        SideSheetType::Overlay => s::SHEET_OVERLAY,
        SideSheetType::Push => s::SHEET_PUSH,
        SideSheetType::Reveal => s::SHEET_REVEAL,
        SideSheetType::Menu => s::SHEET_MENU,
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn SheetPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let placement_index = use_signal(|| 0_usize);
    let side_type_index = use_signal(|| 0_usize);
    let backdrop_index = use_signal(|| 0_usize);
    // Offered for bottom sheets only, where the handle is left to dismiss.
    let backdrop = if placement_index() == 0 && backdrop_index() == 1 {
        SheetBackdrop::None
    } else {
        SheetBackdrop::Dismiss
    };
    let side_type = match side_type_index() {
        1 => SideSheetType::Push,
        2 => SideSheetType::Reveal,
        3 => SideSheetType::Menu,
        _ => SideSheetType::Overlay,
    };
    let placement = match placement_index() {
        1 => SheetPlacement::Left(side_type),
        2 => SheetPlacement::Right(side_type),
        _ => SheetPlacement::Bottom,
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                div {
                    span { "Placement" }
                    crate::SegmentGroup { active: placement_index,
                        crate::SegmentButton { index: 0, "Bottom" }
                        crate::SegmentButton { index: 1, "Left" }
                        crate::SegmentButton { index: 2, "Right" }
                    }
                }
                if placement_index() != 0 {
                    div {
                        span { "Side type" }
                        crate::SegmentGroup { active: side_type_index,
                            crate::SegmentButton { index: 0, "Overlay" }
                            crate::SegmentButton { index: 1, "Push" }
                            crate::SegmentButton { index: 2, "Reveal" }
                            crate::SegmentButton { index: 3, "Menu" }
                        }
                    }
                } else {
                    div {
                        span { "Backdrop" }
                        crate::SegmentGroup { active: backdrop_index,
                            crate::SegmentButton { index: 0, "Dismiss" }
                            crate::SegmentButton { index: 1, "None" }
                        }
                    }
                }
                crate::Checkbox { checked: open, label: "Open"
                            .to_string() }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                div { class: "g3-sheet-demo-content-root",
                    crate::Header {
                        title: "Side sheets",
                        start_button: rsx! {
                            crate::Button {
                                style: crate::ButtonStyle::Clear,
                                size: crate::ButtonSize::Sm,
                                aria_label: "Toggle menu".to_string(),
                                onclick: move |_| open.toggle(),
                                Menu { size: 22 }
                            }
                        },
                    }
                    crate::Body { has_footer_space: false,
                        crate::Card { title: "Round settings",
                            "Push moves this page. Menu preserves its complete layout beside a desktop rail."
                        }
                        crate::Button { onclick: move |_| open.set(true), "Open sheet" }
                    }
                }
                Sheet {
                    is_open: open,
                    placement,
                    backdrop,
                    class: "g3-sheet-demo-surface",
                    div { class: "g3-sheet-demo-menu",
                        div { class: "g3-sheet-demo-menu-header",
                            span { class: "g3-sheet-demo-menu-eyebrow", "Fairway" }
                            strong { "Round menu" }
                            span { "Choose a destination. Wide menus close from the header toggle." }
                        }
                        crate::List { lines: crate::ListLines::Full,
                            crate::Item { label: "Scorecard", metadata: "12 / 18" }
                            crate::Item { label: "Players", metadata: "4" }
                            crate::Item { label: "Round settings" }
                        }
                    }
                }
            }
        }
    }
}
crate::g3_playground! {
    name: "Sheet",
    description: "Bottom sheet plus overlay, push, reveal, and persistent menu side sheets.",
    demo: SheetPlaygroundDemo,
    source: "src/components/sheet.rs",
}
