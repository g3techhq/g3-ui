//! Alerts: short dialogs that ask for a decision or a value.
use super::{Button, ButtonFill, Color, Input, InputType, Modal, ModalRole};
use crate::theme::ComponentMode;
use dioxus::prelude::*;

/// What an [`AlertButton`] does, which decides its look and position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AlertButtonRole {
    /// An ordinary choice.
    #[default]
    Default,
    /// Dismisses without acting. Also chosen by Escape.
    Cancel,
    /// A destructive choice, drawn in the danger color.
    Destructive,
}

/// One button of an [`Alert`].
#[derive(Clone, Debug, PartialEq)]
pub struct AlertButton {
    /// Button text.
    pub label: String,
    /// What it does.
    pub role: AlertButtonRole,
}

impl AlertButton {
    /// An ordinary button.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Default,
        }
    }

    /// A cancel button.
    pub fn cancel(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Cancel,
        }
    }

    /// A destructive button.
    pub fn destructive(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Destructive,
        }
    }
}

/// A text field inside an [`Alert`], for a prompt.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct AlertInput {
    /// Field label.
    pub label: String,
    /// Hint shown while empty.
    pub placeholder: Option<String>,
    /// Starting text.
    pub value: String,
    /// Kind of value.
    pub input_type: InputType,
}

/// What the user did with an [`Alert`].
#[derive(Clone, Debug, PartialEq)]
pub struct AlertResult {
    /// Index of the pressed button, or `None` when dismissed with Escape or a
    /// tap outside.
    pub button: Option<usize>,
    /// The text entered, for an alert with an input.
    pub value: Option<String>,
}

impl AlertResult {
    /// Whether the user pressed a button that is not a cancel button.
    pub fn confirmed(&self, buttons: &[AlertButton]) -> bool {
        self.button
            .and_then(|index| buttons.get(index))
            .is_some_and(|button| button.role != AlertButtonRole::Cancel)
    }
}

/// A short dialog with a title, message, buttons, and an optional text field.
/// Like Ionic's `ion-alert`.
///
/// Any button closes it and reports its index. To open alerts from event
/// handlers and await the answer, use [`use_alert`](crate::use_alert).
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let open = use_signal(|| false);
/// # fn leave() {}
/// rsx! {
///     Alert {
///         open,
///         title: "Leave round?",
///         message: "Your scores are saved.",
///         buttons: vec![AlertButton::cancel("Stay"), AlertButton::destructive("Leave")],
///         on_result: move |result: AlertResult| if result.button == Some(1) { leave() },
///     }
/// }
/// # }
/// ```
#[component]
pub fn Alert(
    /// Whether the alert is showing.
    open: Signal<bool>,
    /// The question or headline.
    title: String,
    /// Supporting text.
    message: Option<String>,
    /// The buttons, in order. Cancel buttons are placed first.
    buttons: Vec<AlertButton>,
    /// A text field, for prompts.
    input: Option<AlertInput>,
    /// Called once with what the user did.
    on_result: Option<EventHandler<AlertResult>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the dialog surface.
    class: Option<String>,
) -> Element {
    let has_input = input.is_some();
    let mut value = use_signal(String::new);
    let initial = input.as_ref().map(|input| input.value.clone());
    use_effect(move || {
        if open()
            && let Some(initial) = initial.clone()
        {
            value.set(initial);
        }
    });
    let finish = move |button: Option<usize>| {
        let mut open = open;
        open.set(false);
        if let Some(on_result) = on_result {
            on_result.call(AlertResult {
                button,
                value: has_input.then(|| value.cloned()),
            });
        }
    };
    let mut ordered: Vec<(usize, AlertButton)> = buttons.into_iter().enumerate().collect();
    ordered.sort_by_key(|(_, button)| button.role != AlertButtonRole::Cancel);
    rsx! {
        Modal {
            open,
            title,
            role: ModalRole::AlertDialog,
            on_dismiss: move |_| finish(None),
            mode,
            class,
            actions: rsx! {
                for (index, button) in ordered {
                    Button {
                        key: "{index}",
                        mode,
                        fill: match button.role {
                            AlertButtonRole::Cancel => ButtonFill::Outline,
                            _ => ButtonFill::Solid,
                        },
                        color: match button.role {
                            AlertButtonRole::Cancel => Color::Neutral,
                            AlertButtonRole::Destructive => Color::Danger,
                            AlertButtonRole::Default => Color::Accent,
                        },
                        onclick: move |_| finish(Some(index)),
                        "{button.label}"
                    }
                }
            },
            if let Some(message) = message {
                p { "{message}" }
            }
            if let Some(input) = input {
                Input {
                    label: input.label,
                    placeholder: input.placeholder,
                    input_type: input.input_type,
                    value,
                    autofocus: true,
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn AlertPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut modal_open = use_signal(|| false);
    let mut last = use_signal(|| "Nothing yet".to_string());
    let alerts = crate::use_alert();
    let sheets = crate::use_action_sheet();
    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack {
                crate::Button { onclick: move |_| open.set(true), "Declarative alert" }
                crate::Button {
                    fill: ButtonFill::Outline,
                    onclick: move |_| async move {
                        let name = alerts
                            .prompt(
                                "Rename round",
                                AlertInput {
                                    label: "Name".into(),
                                    value: "Saturday".into(),
                                    ..AlertInput::default()
                                },
                            )
                            .await;
                        last.set(format!("Prompt: {name:?}"));
                    },
                    "Prompt from code"
                }
                crate::Button {
                    fill: ButtonFill::Outline,
                    onclick: move |_| async move {
                        let choice = sheets
                            .show(crate::ActionSheetOptions {
                                title: Some("Round 12".into()),
                                message: None,
                                buttons: vec![
                                    crate::ActionSheetButton::new("Share"),
                                    crate::ActionSheetButton::destructive("Delete"),
                                ],
                            })
                            .await;
                        last.set(format!("Action sheet: {choice:?}"));
                    },
                    "Action sheet from code"
                }
                crate::Button {
                    fill: ButtonFill::Clear,
                    onclick: move |_| modal_open.set(true),
                    "Modal"
                }
                p { "Last result: {last}" }
            }
            Alert {
                open,
                title: "Leave round?",
                message: "Your scores are saved.",
                buttons: vec![AlertButton::cancel("Stay"), AlertButton::destructive("Leave")],
                on_result: move |result: AlertResult| last.set(format!("Alert: {:?}", result.button)),
            }
            Modal {
                open: modal_open,
                title: "Round details",
                size: crate::ModalSize::Wide,
                actions: rsx! {
                    Button { fill: ButtonFill::Clear, onclick: move |_| modal_open.set(false), "Close" }
                },
                p { "Started 42 minutes ago at Pebble Creek." }
            }
        }
    }
}

crate::g3_playground! {
    name: "Alert",
    description: "Alerts, prompts, action sheets, and modals, declared or opened from code.",
    components: ["use_alert", "use_action_sheet", "Alert", "ActionSheet"],
    demo: AlertPlaygroundDemo,
    source: "src/components/alert.rs",
}
