//! Content cards.
use super::pressable::Destination;
use crate::components::pressable::{Pressable, Target};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// How a [`Card`] separates itself from the page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum CardVariant {
    /// Lifted off the page with a shadow.
    #[default]
    Raised,
    /// Level with the page, outlined by a border instead of a shadow.
    Flat,
    /// Level with the page on a tinted surface, for cards grouped inside
    /// another surface.
    Filled,
}

/// A content container with an optional title, subtitle, trailing content,
/// and media. Like Ionic's `ion-card`.
///
/// Give it `onclick`, `to`, or `href` to make the whole card tappable. The
/// title then becomes the card's button or link, so the card has an accessible
/// name, and controls inside it (such as an [`InfoButton`](crate::InfoButton)
/// in `end`) keep working on their own.
///
/// ```rust,ignore
/// rsx! {
///     Card {
///         title: "Saturday four-ball",
///         subtitle: "Tee time 8:10",
///         end: rsx! { Badge { color: Color::Success, "Open" } },
///         to: Route::Game { id },
///         "Two spots left."
///     }
/// }
/// ```
#[component]
pub fn Card(
    /// Heading.
    title: Option<String>,
    /// Secondary line under the title.
    subtitle: Option<String>,
    /// Heading level of the title, 1 to 6. Defaults to 2; set it so the page's
    /// headings do not skip a level.
    heading_level: Option<u8>,
    /// Content at the trailing end of the header, such as a badge or value.
    end: Option<Element>,
    /// Content above everything else, usually an image, drawn edge to edge.
    media: Option<Element>,
    /// Surface treatment. Defaults to [`CardVariant::Raised`].
    variant: Option<CardVariant>,
    /// Highlight the card as chosen. Exposed as `aria-pressed` on a tappable
    /// card.
    selected: Option<bool>,
    /// Called when the card is pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Router destination for a tappable card.
    #[props(default, into)]
    to: Destination,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Disable a tappable card.
    disabled: Option<bool>,
    /// Accessible name of a tappable card with no title.
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the card.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let interactive = onclick.is_some() || to.is_some() || href.is_some();
    let target = Target::from_props(href, to, false);
    let selected = selected.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let title = title.filter(|title| !title.is_empty());
    let cls = classes([
        "g3-card",
        mode.pick("g3-card-ios", "g3-card-md"),
        match variant.unwrap_or_default() {
            CardVariant::Raised => "",
            CardVariant::Flat => "g3-card-flat",
            CardVariant::Filled => "g3-card-filled",
        },
        if selected { "g3-card-selected" } else { "" },
        if interactive {
            "g3-card-interactive"
        } else {
            ""
        },
    ]);
    let mut action_attributes = Vec::new();
    if !target.is_link() && interactive {
        action_attributes.push(Attribute::new(
            "aria-pressed",
            selected.to_string(),
            None,
            false,
        ));
    }
    if title.is_none()
        && let Some(label) = aria_label
    {
        action_attributes.push(Attribute::new("aria-label", label, None, false));
    }
    let has_header = title.is_some() || subtitle.is_some() || end.is_some();
    let title_node = title.map(|title| {
        if interactive {
            rsx! {
                Pressable {
                    class: "g3-card-action",
                    target: target.clone(),
                    disabled,
                    onclick,
                    attributes: action_attributes.clone(),
                    "{title}"
                }
            }
        } else {
            rsx! { "{title}" }
        }
    });
    let untitled_action = interactive && title_node.is_none();
    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            aria_disabled: (interactive && disabled).then_some("true"),
            if untitled_action {
                Pressable {
                    class: "g3-card-action",
                    target,
                    disabled,
                    onclick,
                    attributes: action_attributes,
                }
            }
            if let Some(media) = media {
                div { class: "g3-card-media", {media} }
            }
            if has_header {
                div { class: "g3-card-header",
                    div { class: "g3-card-heading",
                        if let Some(title) = title_node {
                            super::text::Heading { level: heading_level.unwrap_or(2), class: "g3-card-title", {title} }
                        }
                        if let Some(subtitle) = subtitle {
                            span { class: "g3-card-subtitle", "{subtitle}" }
                        }
                    }
                    if let Some(end) = end {
                        div { class: "g3-card-end", {end} }
                    }
                }
            }
            div { class: "g3-card-body", {children} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn CardPlaygroundDemo() -> Element {
    let title = use_signal(|| "Saturday four-ball".to_string());
    let variant = use_signal(|| CardVariant::Raised);
    let mut selected = use_signal(|| false);
    let interactive = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                crate::Input { label: "Title", value: title }
                crate::SegmentGroup { value: variant, aria_label: "Variant",
                    crate::SegmentButton { value: CardVariant::Raised, "Raised" }
                    crate::SegmentButton { value: CardVariant::Flat, "Flat" }
                    crate::SegmentButton { value: CardVariant::Filled, "Filled" }
                }
                crate::Checkbox { checked: interactive, label: "Selectable" }
            },
            Card {
                title: title(),
                subtitle: "Tee time 8:10",
                variant: variant(),
                selected: selected(),
                onclick: interactive().then(|| EventHandler::new(move |_| selected.toggle())),
                end: rsx! {
                    crate::InfoButton {
                        sheet_title: "Four-ball",
                        sheet: rsx! {
                            p { "Best ball of each pair counts." }
                        },
                    }
                },
                "Two spots left. Tap the card to select it."
            }
        }
    }
}

crate::g3_playground! {
    name: "Card",
    description: "Content card with title, subtitle, media, and an optional action.",
    demo: CardPlaygroundDemo,
    source: "src/components/card.rs",
}
