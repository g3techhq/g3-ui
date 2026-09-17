//! Typography.
use super::Color;
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// The typographic role of [`Text`], which also picks its HTML element.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TextVariant {
    /// A large page or section title (`h2`).
    Title,
    /// A section heading (`h3`).
    Heading,
    /// Body copy (`p`).
    #[default]
    Body,
    /// Small supporting copy (`p`).
    Caption,
    /// A short label (`span`).
    Label,
    /// Small uppercase text above a heading (`span`).
    Overline,
}

/// How strongly [`Text`] stands out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TextTone {
    /// The theme's body text colour.
    #[default]
    Primary,
    /// De-emphasised.
    Secondary,
    /// Least emphasis.
    Tertiary,
}

/// Text in the theme's type scale and colours.
///
/// ```rust,ignore
/// rsx! {
///     Text { variant: TextVariant::Heading, "Leaderboard" }
///     Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Updated 2 min ago" }
///     Text { color: Color::Danger, "Payment failed" }
/// }
/// ```
#[component]
pub fn Text(
    /// Role and element. Defaults to [`TextVariant::Body`].
    variant: Option<TextVariant>,
    /// Emphasis. Defaults to [`TextTone::Primary`]; ignored when `color` is
    /// set.
    tone: Option<TextTone>,
    /// A semantic colour, such as [`Color::Danger`] for an error.
    color: Option<Color>,
    /// Cut off with an ellipsis instead of wrapping.
    truncate: Option<bool>,
    /// Extra classes for the element.
    class: Option<String>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or_default();
    let variant_cls = match variant {
        TextVariant::Title => "g3-text-title",
        TextVariant::Heading => "g3-text-heading",
        TextVariant::Body => "g3-text-body",
        TextVariant::Caption => "g3-text-caption",
        TextVariant::Label => "g3-text-label",
        TextVariant::Overline => "g3-text-overline",
    };
    let color_cls = match (color, tone.unwrap_or_default()) {
        (Some(color), _) => format!("g3-text-{}", color.as_str()),
        (None, TextTone::Primary) => String::new(),
        (None, TextTone::Secondary) => "g3-text-secondary".to_string(),
        (None, TextTone::Tertiary) => "g3-text-tertiary".to_string(),
    };
    let cls = merge_classes(
        classes([
            "g3-text",
            variant_cls,
            &color_cls,
            if truncate.unwrap_or(false) {
                "g3-text-truncate"
            } else {
                ""
            },
        ]),
        class.as_deref(),
    );
    match variant {
        TextVariant::Title => rsx! {
            h2 { class: cls, {children} }
        },
        TextVariant::Heading => rsx! {
            h3 { class: cls, {children} }
        },
        TextVariant::Body | TextVariant::Caption => rsx! {
            p { class: cls, {children} }
        },
        TextVariant::Label | TextVariant::Overline => rsx! {
            span { class: cls, {children} }
        },
    }
}
