//! Status badges.
use super::Color;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// A small pill for a count or a status. Like Ionic's `ion-badge`.
///
/// ```rust,ignore
/// rsx! { Badge { color: Color::Success, "Open" } }
/// ```
#[component]
pub fn Badge(
    /// Colour. Defaults to [`Color::Neutral`].
    color: Option<Color>,
    /// What the badge means, when its text alone does not say, such as
    /// "3 unread" for a badge showing "3".
    aria_label: Option<String>,
    /// Extra classes for the badge.
    class: Option<String>,
    children: Element,
) -> Element {
    let color = color.unwrap_or(Color::Neutral);
    rsx! {
        span {
            class: merge_classes(format!("g3-badge g3-badge-{}", color.as_str()), class.as_deref()),
            if let Some(label) = aria_label {
                span { aria_hidden: "true", {children} }
                span { class: "g3-sr-only", "{label}" }
            } else {
                {children}
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn BadgePlaygroundDemo() -> Element {
    rsx! {
        crate::PlaygroundDemoFrame {
            div { class: "playground-row",
                Badge { "Draft" }
                Badge { color: Color::Accent, "Live" }
                Badge { color: Color::Success, "Won" }
                Badge { color: Color::Warning, "Rain delay" }
                Badge { color: Color::Danger, "3" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Badge",
    description: "Status and count pills in each colour.",
    demo: BadgePlaygroundDemo,
    source: "src/components/badge.rs",
}
