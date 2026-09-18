//! Action sheets: a list of choices rising from the bottom.
use super::{BottomSheet, Color};
use crate::theme::{ComponentMode, classes, merge_classes, use_strings};
use dioxus::prelude::*;

/// One choice in an [`ActionSheet`].
#[derive(Clone, Debug, PartialEq)]
pub struct ActionSheetButton {
    /// Button text.
    pub label: String,
    /// Colour. [`Color::Danger`] marks a destructive choice.
    pub color: Option<Color>,
    /// Whether it can be chosen.
    pub disabled: bool,
}

impl ActionSheetButton {
    /// An ordinary choice.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            color: None,
            disabled: false,
        }
    }

    /// A destructive choice.
    pub fn destructive(label: impl Into<String>) -> Self {
        Self {
            color: Some(Color::Danger),
            ..Self::new(label)
        }
    }

    /// Make the choice unavailable.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A sheet of choices with a separate cancel button. Like Ionic's
/// `ion-action-sheet`.
///
/// `on_select` receives the index of the chosen button, or `None` when the
/// user cancels. To open one from an event handler and await the choice, use
/// [`use_action_sheet`](crate::use_action_sheet).
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let open = use_signal(|| false);
/// # fn delete() {}
/// rsx! {
///     ActionSheet {
///         open,
///         title: "Round 12",
///         buttons: vec![ActionSheetButton::new("Share"), ActionSheetButton::destructive("Delete")],
///         on_select: move |choice| if choice == Some(1) { delete() },
///     }
/// }
/// # }
/// ```
#[component]
pub fn ActionSheet(
    /// Whether the sheet is showing.
    open: Signal<bool>,
    /// Heading.
    title: Option<String>,
    /// Supporting text under the heading.
    message: Option<String>,
    /// The choices.
    buttons: Vec<ActionSheetButton>,
    /// Cancel button text. Defaults to
    /// [`Strings::cancel`](crate::Strings::cancel).
    cancel_label: Option<String>,
    /// Called once with the chosen index, or `None` on cancel.
    on_select: Option<EventHandler<Option<usize>>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the sheet.
    class: Option<String>,
) -> Element {
    let cancel_label = cancel_label.unwrap_or_else(|| use_strings().cancel);
    let finish = move |choice: Option<usize>| {
        let mut open = open;
        open.set(false);
        if let Some(on_select) = on_select {
            on_select.call(choice);
        }
    };
    rsx! {
        BottomSheet {
            open,
            title,
            draggable: false,
            on_dismiss: move |_| {
                if let Some(on_select) = on_select {
                    on_select.call(None);
                }
            },
            mode,
            class: merge_classes("g3-action-sheet", class.as_deref()),
            if let Some(message) = message {
                p { class: "g3-action-sheet-message", "{message}" }
            }
            div { class: "g3-action-sheet-group", role: "group",
                for (index, button) in buttons.into_iter().enumerate() {
                    button {
                        key: "{index}",
                        r#type: "button",
                        class: classes([
                            "g3-action-sheet-button",
                            if button.color == Some(Color::Danger) { "g3-action-sheet-button-danger" } else { "" },
                        ]),
                        disabled: button.disabled,
                        onclick: move |_| finish(Some(index)),
                        "{button.label}"
                    }
                }
            }
            button {
                r#type: "button",
                class: "g3-action-sheet-button g3-action-sheet-cancel",
                onclick: move |_| finish(None),
                "{cancel_label}"
            }
        }
    }
}
