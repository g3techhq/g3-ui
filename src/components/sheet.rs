//! Sheet component - bottom and side sheet with drag-to-dismiss and platform styling.

use super::overlay_scroll::use_lock_body_scroll;
use super::sheet_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

const SHEET_DISMISS_DISTANCE: f64 = 96.0;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SheetPlacement {
    #[default]
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Sheet(
    mut is_open: Signal<bool>,
    placement: Option<SheetPlacement>,
    draggable: Option<bool>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    children: Element,
) -> Element {
    let mut drag_start_y = use_signal(|| 0.0);
    let mut current_y = use_signal(|| 0.0);
    let mut is_dragging = use_signal(|| false);
    let mode = use_component_mode(mode);
    let placement = placement.unwrap_or_default();
    let is_draggable = draggable.unwrap_or(true);

    let mode_cls = match mode {
        ComponentMode::Ios => s::SHEET_IOS,
        ComponentMode::Md => s::SHEET_MD,
    };
    let placement_cls = match placement {
        SheetPlacement::Bottom => s::SHEET_BOTTOM,
        SheetPlacement::Left => s::SHEET_LEFT,
        SheetPlacement::Right => s::SHEET_RIGHT,
    };

    let sheet_cls = format!("{} {mode_cls} {placement_cls}", s::SHEET);
    let has_handle = placement == SheetPlacement::Bottom;

    let handle_start = move |evt: MouseEvent| {
        evt.prevent_default();
        is_dragging.set(true);
        drag_start_y.set(evt.client_coordinates().y);
        current_y.set(0.0);
    };

    let handle_start_pointer = move |evt: PointerEvent| {
        evt.prevent_default();
        is_dragging.set(true);
        drag_start_y.set(evt.client_coordinates().y);
        current_y.set(0.0);
    };

    let handle_move = move |evt: MouseEvent| {
        if is_dragging() {
            evt.prevent_default();
            let delta_y = (evt.client_coordinates().y - drag_start_y()).max(0.0);
            current_y.set(delta_y);
        }
    };

    let handle_move_pointer = move |evt: PointerEvent| {
        if is_dragging() {
            evt.prevent_default();
            let delta_y = (evt.client_coordinates().y - drag_start_y()).max(0.0);
            current_y.set(delta_y);
        }
    };

    let handle_end = move |evt: MouseEvent| {
        if is_dragging() {
            evt.prevent_default();
            is_dragging.set(false);
            if current_y() > SHEET_DISMISS_DISTANCE {
                is_open.set(false);
            }
            current_y.set(0.0);
        }
    };

    let handle_end_pointer = move |evt: PointerEvent| {
        if is_dragging() {
            evt.prevent_default();
            is_dragging.set(false);
            if current_y() > SHEET_DISMISS_DISTANCE {
                is_open.set(false);
            }
            current_y.set(0.0);
        }
    };

    let backdrop_cls = format!(
        "{} {}",
        s::BACKDROP,
        if is_open() {
            "g3-sheet-backdrop-open"
        } else {
            "g3-sheet-backdrop-closed pointer-events-none"
        }
    );
    use_lock_body_scroll(is_open);
    let is_open_now = is_open();
    let state_cls = if is_open_now {
        s::STATE_OPEN
    } else {
        s::STATE_CLOSED
    };

    rsx! {
        button {
            r#type: "button",
            aria_label: "Sheet backdrop",
            class: format!("{backdrop_cls} appearance-none border-0 p-0"),
            onclick: move |_| is_open.set(false),
            onpointermove: handle_move_pointer,
            onpointerup: handle_end_pointer,
            onpointercancel: handle_end_pointer,
        }
        div {
            role: "dialog",
            aria_modal: "true",
            aria_label: "Sheet",
            aria_hidden: (!is_open_now).to_string(),
            inert: (!is_open_now).then(|| "".to_string()),
            class: merge_classes(format!("{sheet_cls} {state_cls}"), class.as_deref()),
            style: if is_dragging() && placement == SheetPlacement::Bottom { format!("--g3-sheet-drag-y: {}px; touch-action: none;", current_y()) } else { "".to_string() },
            onmousemove: handle_move,
            onpointermove: handle_move_pointer,
            onmouseup: handle_end,
            onpointerup: handle_end_pointer,
            onpointercancel: handle_end_pointer,
            if is_draggable && has_handle {
                button {
                    r#type: "button",
                    class: s::HANDLE_WRAP_IOS,
                    aria_label: "Sheet handle",
                    onmousedown: handle_start,
                    onmousemove: handle_move,
                    onmouseup: handle_end,
                            onpointerdown: handle_start_pointer,
                    onpointermove: handle_move_pointer,
                    onpointerup: handle_end_pointer,
                            onpointercancel: handle_end_pointer,
                    div { class: s::HANDLE_IOS }
                }
            }
            div { class: s::CONTENT, {children} }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn SheetPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let placement_index = use_signal(|| 0_usize);
    let placement = match placement_index() {
        1 => SheetPlacement::Left,
        2 => SheetPlacement::Right,
        _ => SheetPlacement::Bottom,
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                div {
                    span { "Placement" }
                    crate::SegmentGroup { active: placement_index,
                        crate::SegmentButton { index: 0, "Bottom" }
                        crate::SegmentButton { index: 1, "Left" }
                        crate::SegmentButton { index: 2, "Right" }
                    }
                }
                crate::Checkbox { checked: open, label: "Open".to_string() }
            },
            crate::Button { onclick: move |_| open.set(true), "Open sheet" }
            Sheet { is_open: open, placement,
                crate::List { inset: true, lines: crate::ListLines::None,
                    crate::Item { label: "Round settings", description: "Use the handle or backdrop to close." }
                    crate::Item { label: "Tee time", metadata: "9:40" }
                    crate::Item { label: "Players", metadata: "4" }
                }
            }
        }
    }
}
crate::g3_playground! {
    name: "Sheet",
    g3_name: "G3Sheet",
    description: "Bottom and side sheet with backdrop dismissal.",
    demo: SheetPlaygroundDemo,
    source: "src/components/sheet.rs",
}
