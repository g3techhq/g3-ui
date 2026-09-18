//! Chips.
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// A compact pill for a filter, tag, or choice. Like Ionic's `ion-chip`.
///
/// With `onclick` it is a button; with `selected` as well it is a toggle
/// button (`aria-pressed`). Without `onclick` it is plain content.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let mut nearby = use_signal(|| false);
/// # #[component] fn MapPin() -> Element { rsx! {} }
/// rsx! {
///     Chip { selected: nearby(), onclick: move |_| nearby.toggle(), "Nearby" }
///     Chip { start: rsx! { MapPin {} }, "Pebble Creek" }
/// }
/// # }
/// ```
#[component]
pub fn Chip(
    /// Show the chip as on. Makes a pressable chip a toggle button.
    selected: Option<bool>,
    /// Disable a pressable chip.
    disabled: Option<bool>,
    /// Content before the label, usually an icon or avatar.
    start: Option<Element>,
    /// Content after the label.
    end: Option<Element>,
    /// Called when pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Extra classes for the chip.
    class: Option<String>,
    children: Element,
) -> Element {
    let body = rsx! {
        if let Some(start) = start {
            span { class: "g3-chip-start", {start} }
        }
        span { class: "g3-chip-label", {children} }
        if let Some(end) = end {
            span { class: "g3-chip-end", {end} }
        }
    };
    match onclick {
        Some(onclick) => rsx! {
            button {
                class: merge_classes("g3-chip", class.as_deref()),
                r#type: "button",
                disabled,
                aria_pressed: selected.map(|selected| selected.to_string()),
                onclick: move |event| onclick.call(event),
                {body}
            }
        },
        None => rsx! {
            span {
                class: merge_classes("g3-chip g3-chip-static", class.as_deref()),
                {body}
            }
        },
    }
}

#[cfg(feature = "playground")]
#[component]
fn ChipPlaygroundDemo() -> Element {
    let mut nearby = use_signal(|| true);
    let mut friends = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            div { class: "playground-row",
                Chip { selected: nearby(), onclick: move |_| nearby.toggle(), "Nearby" }
                Chip { selected: friends(), onclick: move |_| friends.toggle(), "Friends" }
                Chip { disabled: true, onclick: |_| {}, "Weekend" }
                Chip { "Static tag" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Chip",
    description: "Filter chips, toggle chips, and tags.",
    demo: ChipPlaygroundDemo,
    source: "src/components/chip.rs",
}
