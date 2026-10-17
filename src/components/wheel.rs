//! The spinning columns of an iOS-style picker.
use super::overlay::js_string;
use crate::state::{use_element_id, use_synced_signal};
use dioxus::prelude::*;

/// Keeps a column's scroll position and its value in step. Scrolling,
/// flicking, or dragging with a mouse reports the item that settles in the
/// middle; a new value from Rust scrolls to it.
const WHEEL_SCRIPT: &str = include_str!("wheel.js");

/// One column of a wheel picker, such as the hours of a time.
///
/// Screen readers and keyboards meet a spin button: arrow keys step, Home
/// and End jump to the ends, Page Up and Page Down move by five. The visible
/// list is for pointers and is hidden from assistive technology.
#[component]
pub(crate) fn WheelColumn(
    labels: Vec<String>,
    index: usize,
    onchange: EventHandler<usize>,
    aria_label: String,
    /// Unavailable options. Scrolling and keyboard movement skip them.
    #[props(default)]
    disabled_indices: Vec<usize>,
    /// Text alignment and share of the width: month names need more room.
    #[props(default)]
    wide: bool,
) -> Element {
    let id = use_element_id("wheel", None);
    let scroll_id = format!("{id}-scroll");
    let last = labels.len().saturating_sub(1);
    let index = index.min(last);
    let current = use_synced_signal(index);
    let mut first = use_signal(|| true);
    let report_disabled = disabled_indices.clone();
    let session = super::script::use_dom_script(
        Some(WHEEL_SCRIPT.replace("__ID__", &js_string(&scroll_id))),
        move |reported: usize| {
            if reported <= last
                && !report_disabled.contains(&reported)
                && reported != *current.peek()
            {
                onchange.call(reported);
            }
        },
    );
    // Reads `current`, so it reruns when the parent passes a new index.
    use_effect(move || {
        let index = current();
        if let Some(eval) = session() {
            let smooth = !*first.peek();
            first.set(false);
            let _ = eval.send((index, smooth));
        }
    });
    let unavailable = disabled_indices.contains(&index);
    let step_disabled = disabled_indices.clone();
    let step = move |to: usize| {
        let to = to.min(last);
        let allowed = |item: &usize| !step_disabled.contains(item);
        let next = if to >= index {
            (to..=last).find(allowed)
        } else {
            (0..=to).rev().find(allowed)
        };
        if let Some(next) = next
            && next != index
        {
            onchange.call(next);
        }
    };
    let value_text = labels.get(index).cloned().unwrap_or_default();
    let keyboard_step = step.clone();

    rsx! {
        div {
            class: if wide { "g3-wheel-column g3-wheel-column-wide" } else { "g3-wheel-column" },
            role: "spinbutton",
            tabindex: "0",
            aria_label,
            aria_valuemin: "0",
            aria_valuemax: last.to_string(),
            aria_valuenow: index.to_string(),
            aria_valuetext: value_text,
            aria_invalid: unavailable.then_some("true"),
            onkeydown: move |event| {
                let to = match event.key() {
                    Key::ArrowUp => index.saturating_sub(1),
                    Key::ArrowDown => index + 1,
                    Key::PageUp => index.saturating_sub(5),
                    Key::PageDown => index + 5,
                    Key::Home => 0,
                    Key::End => last,
                    _ => return,
                };
                event.prevent_default();
                keyboard_step(to);
            },
            div { id: scroll_id, class: "g3-wheel-scroll", aria_hidden: "true",
                div { class: "g3-wheel-spacer" }
                for (item, label) in labels.iter().enumerate() {
                    div {
                        key: "{item}",
                        class: "g3-wheel-item",
                        "data-selected": (item == index).then_some("true"),
                        "data-disabled": disabled_indices.contains(&item).then_some("true"),
                        onclick: {
                            let disabled = disabled_indices.contains(&item);
                            let step = step.clone();
                            move |_| { if !disabled { step(item); } }
                        },
                        "{label}"
                    }
                }
                div { class: "g3-wheel-spacer" }
            }
        }
    }
}
