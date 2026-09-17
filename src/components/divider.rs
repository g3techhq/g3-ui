//! Dividers.
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// Direction of a [`Divider`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum DividerOrientation {
    /// A horizontal rule between stacked content.
    #[default]
    Horizontal,
    /// A vertical rule between side-by-side content.
    Vertical,
}

/// A thin rule that separates content. Set `--g3-divider-color` to recolour
/// it.
#[component]
pub fn Divider(
    /// Direction. Defaults to [`DividerOrientation::Horizontal`].
    orientation: Option<DividerOrientation>,
    /// Add space on both sides of the rule.
    spaced: Option<bool>,
    /// Extra classes for the rule.
    class: Option<String>,
) -> Element {
    let orientation = orientation.unwrap_or_default();
    let (direction_cls, aria_orientation) = match orientation {
        DividerOrientation::Horizontal => ("g3-divider-h", "horizontal"),
        DividerOrientation::Vertical => ("g3-divider-v", "vertical"),
    };
    let cls = classes([
        "g3-divider",
        direction_cls,
        if spaced.unwrap_or(false) {
            "g3-divider-margins"
        } else {
            ""
        },
    ]);
    rsx! {
        hr {
            class: merge_classes(cls, class.as_deref()),
            aria_orientation: (orientation == DividerOrientation::Vertical).then_some(aria_orientation),
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn DividerPlaygroundDemo() -> Element {
    let vertical = use_signal(|| false);
    let spaced = use_signal(|| true);
    let orientation = if vertical() {
        DividerOrientation::Vertical
    } else {
        DividerOrientation::Horizontal
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: vertical, label: "Vertical" }
                crate::Checkbox { checked: spaced, label: "Spaced" }
            },
            div {
                class: if vertical() { "playground-divider-surface playground-divider-vertical" } else { "playground-divider-surface" },
                span { "Front nine" }
                Divider { orientation, spaced: spaced() }
                span { "Back nine" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Divider",
    description: "Horizontal or vertical separator.",
    demo: DividerPlaygroundDemo,
    source: "src/components/divider.rs",
}
