//! A panel for a screen with nothing to show: no results, nothing saved yet,
//! or a load that failed.
use super::color::Color;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// What fills a screen, or a part of one, when there is nothing to show:
/// an icon, a headline, a line of explanation, and usually a way forward.
///
/// Use it for an empty list ("No saved rounds"), a search with no matches,
/// or a load that failed. A failure is `color: Color::Danger`, which tints the
/// icon and announces the panel to screen readers as it appears.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # fn start_round() {}
/// rsx! {
///     EmptyState {
///         title: "No rounds yet",
///         action: rsx! { Button { onclick: move |_| start_round(), "Start a round" } },
///         "Rounds you play show up here."
///     }
/// }
/// # }
/// ```
///
/// While the data is still loading, show a [`Spinner`](crate::Spinner) or
/// [`Skeleton`](crate::Skeleton) instead; this is for once it has arrived.
#[component]
pub fn EmptyState(
    /// The headline, such as "No results".
    title: String,
    /// An icon or small illustration above the title.
    icon: Option<Element>,
    /// A way forward, usually a [`Button`](crate::Button).
    action: Option<Element>,
    /// Tone. Defaults to [`Color::Neutral`]; use [`Color::Danger`] for a
    /// failed load.
    color: Option<Color>,
    /// Heading level of the title, 1 to 6. Defaults to 2; set it so the page's
    /// headings do not skip a level.
    heading_level: Option<u8>,
    /// Extra classes for the panel.
    class: Option<String>,
    /// The explanation under the title.
    children: Element,
) -> Element {
    let color = color.unwrap_or(Color::Neutral);
    // A failure should be heard when it replaces the content; an empty list
    // is simply part of the page.
    let role = (color == Color::Danger).then_some("alert");
    rsx! {
        div {
            class: merge_classes("g3-empty-state", class.as_deref()),
            "data-color": color.as_str(),
            role,
            if let Some(icon) = icon {
                div { class: "g3-empty-state-icon", aria_hidden: "true", {icon} }
            }
            super::text::Heading {
                level: heading_level.unwrap_or(2),
                class: "g3-empty-state-title",
                "{title}"
            }
            div { class: "g3-empty-state-message", {children} }
            if let Some(action) = action {
                div { class: "g3-empty-state-action", {action} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn EmptyStatePlaygroundDemo() -> Element {
    use dioxus_icons::lucide::{CircleAlert, Flag, SearchX};
    let kind = use_signal(|| 0_usize);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::SegmentGroup { value: kind, aria_label: "State",
                    crate::SegmentButton { value: 0_usize, "Empty" }
                    crate::SegmentButton { value: 1_usize, "No results" }
                    crate::SegmentButton { value: 2_usize, "Error" }
                }
            },
            match kind() {
                1 => rsx! {
                    EmptyState {
                        title: "No courses match",
                        icon: rsx! { SearchX { size: 40 } },
                        "Try a shorter name, or search by town."
                    }
                },
                2 => rsx! {
                    EmptyState {
                        title: "Couldn't load your rounds",
                        color: Color::Danger,
                        icon: rsx! { CircleAlert { size: 40 } },
                        action: rsx! {
                            crate::Button { fill: crate::ButtonFill::Outline, "Try again" }
                        },
                        "Check your connection and try again."
                    }
                },
                _ => rsx! {
                    EmptyState {
                        title: "No rounds yet",
                        icon: rsx! { Flag { size: 40 } },
                        action: rsx! { crate::Button { "Start a round" } },
                        "Rounds you play show up here."
                    }
                },
            }
        }
    }
}

crate::g3_playground! {
    name: "EmptyState",
    description: "What a screen shows when there is nothing in it, or the load failed.",
    demo: EmptyStatePlaygroundDemo,
    source: "src/components/empty_state.rs",
}
