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
    /// The theme's body text color.
    #[default]
    Primary,
    /// De-emphasised.
    Secondary,
    /// Least emphasis.
    Tertiary,
}

/// Text in the theme's type scale and colors.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     Text { variant: TextVariant::Heading, "Leaderboard" }
///     Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Updated 2 min ago" }
///     Text { color: Color::Danger, "Payment failed" }
/// }
/// # }
/// ```
#[component]
pub fn Text(
    /// Role and element. Defaults to [`TextVariant::Body`].
    variant: Option<TextVariant>,
    /// Emphasis. Defaults to [`TextTone::Primary`]; ignored when `color` is
    /// set.
    tone: Option<TextTone>,
    /// A semantic color, such as [`Color::Danger`] for an error.
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

/// The level of a component's own heading, `h1` to `h6`. Pick the one that
/// keeps the page's outline in order, so headings never skip a level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum HeadingLevel {
    /// `h1`, the page title.
    H1,
    /// `h2`, a section. The default for component titles.
    #[default]
    H2,
    /// `h3`.
    H3,
    /// `h4`.
    H4,
    /// `h5`.
    H5,
    /// `h6`.
    H6,
}

/// A heading at a caller-chosen level. Components with a title use it so an
/// app can keep its heading outline in order.
#[component]
pub(crate) fn Heading(level: HeadingLevel, class: String, children: Element) -> Element {
    match level {
        HeadingLevel::H1 => rsx! {
            h1 { class, {children} }
        },
        HeadingLevel::H2 => rsx! {
            h2 { class, {children} }
        },
        HeadingLevel::H3 => rsx! {
            h3 { class, {children} }
        },
        HeadingLevel::H4 => rsx! {
            h4 { class, {children} }
        },
        HeadingLevel::H5 => rsx! {
            h5 { class, {children} }
        },
        HeadingLevel::H6 => rsx! {
            h6 { class, {children} }
        },
    }
}
