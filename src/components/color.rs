//! The semantic color roles components accept.

/// A semantic color for buttons, badges, chips, toasts, and swipe actions.
///
/// Each role maps to a [`Theme`](crate::Theme) token, so it follows the app's
/// theme.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Color {
    /// The theme accent. The default for actions.
    #[default]
    Accent,
    /// A plain, unemphasised surface or text color.
    Neutral,
    /// A positive or completed state.
    Success,
    /// A state that needs attention.
    Warning,
    /// An error or a destructive action.
    Danger,
}

impl Color {
    /// The name the stylesheet knows this color by, such as `"success"`:
    /// the value of a `data-color` attribute, as on a [`Table`](crate::Table)
    /// cell.
    ///
    /// ```
    /// # use g3_ui::Color;
    /// assert_eq!(Color::Danger.as_str(), "danger");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self {
            Color::Accent => "accent",
            Color::Neutral => "neutral",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Danger => "danger",
        }
    }
}

/// A color picker for playground demos.
#[cfg(feature = "playground")]
mod playground {
    use super::Color;
    use dioxus::prelude::*;

    #[component]
    pub(crate) fn ColorSelect(value: Signal<Color>) -> Element {
        rsx! {
            crate::Select {
                label: "Color",
                value,
                options: vec![
                    crate::SelectOption::new(Color::Accent, "Accent"),
                    crate::SelectOption::new(Color::Neutral, "Neutral"),
                    crate::SelectOption::new(Color::Success, "Success"),
                    crate::SelectOption::new(Color::Warning, "Warning"),
                    crate::SelectOption::new(Color::Danger, "Danger"),
                ],
            }
        }
    }
}
#[cfg(feature = "playground")]
pub(crate) use playground::ColorSelect;
