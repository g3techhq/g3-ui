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
///
/// A horizontal divider can carry a `label` in its middle, such as "or" between
/// two ways of signing in:
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     Button { "Continue with Google" }
///     Divider { label: "or continue with email", spaced: true }
///     Input { label: "Email" }
/// }
/// # }
/// ```
#[component]
pub fn Divider(
    /// Direction. Defaults to [`DividerOrientation::Horizontal`].
    orientation: Option<DividerOrientation>,
    /// Add space on both sides of the rule.
    spaced: Option<bool>,
    /// Text in the middle of a horizontal rule. It is read as ordinary text;
    /// the lines either side are decoration.
    label: Option<String>,
    /// Extra classes for the rule.
    class: Option<String>,
) -> Element {
    let orientation = orientation.unwrap_or_default();
    if let Some(label) = label.filter(|_| orientation == DividerOrientation::Horizontal) {
        // Not an <hr>: a separator's content is presentational, so its label
        // would go unread. The text is the part worth hearing.
        return rsx! {
            div {
                class: merge_classes(
                    classes([
                        "g3-divider-labelled",
                        if spaced.unwrap_or(false) { "g3-divider-margins" } else { "" },
                    ]),
                    class.as_deref(),
                ),
                span { class: "g3-divider g3-divider-h", aria_hidden: "true" }
                span { class: "g3-divider-label", "{label}" }
                span { class: "g3-divider g3-divider-h", aria_hidden: "true" }
            }
        };
    }
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
            crate::Card { variant: crate::CardVariant::Flat,
                crate::Stack { horizontal: vertical(), gap: crate::Space::Sm,
                    span { "Front nine" }
                    Divider { orientation, spaced: spaced() }
                    span { "Back nine" }
                }
            }
            crate::Card { variant: crate::CardVariant::Flat,
                Divider { label: "or continue with email", spaced: spaced() }
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
