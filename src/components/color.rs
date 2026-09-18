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
    /// The class suffix the stylesheet uses for this color.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Color::Accent => "accent",
            Color::Neutral => "neutral",
            Color::Success => "success",
            Color::Warning => "warning",
            Color::Danger => "danger",
        }
    }
}
