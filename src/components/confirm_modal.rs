//! A yes-or-no confirmation dialog.
use super::{Button, ButtonFill, Color, Modal, ModalRole};
use crate::theme::{ComponentMode, use_strings};
use dioxus::prelude::*;

/// A confirmation dialog with cancel and confirm buttons.
///
/// Both buttons close it. Set `destructive` for actions such as deleting,
/// which colours the confirm button as dangerous.
///
/// ```rust,ignore
/// rsx! {
///     ConfirmModal {
///         open,
///         title: "Delete round?",
///         message: "This cannot be undone.",
///         confirm_label: "Delete",
///         destructive: true,
///         on_confirm: move |_| delete_round(),
///     }
/// }
/// ```
#[component]
pub fn ConfirmModal(
    /// Whether the dialog is open.
    open: Signal<bool>,
    /// The question being asked.
    title: String,
    /// Supporting text.
    message: Option<String>,
    /// Confirm button text. Defaults to
    /// [`Strings::confirm`](crate::Strings::confirm).
    confirm_label: Option<String>,
    /// Cancel button text. Defaults to
    /// [`Strings::cancel`](crate::Strings::cancel).
    cancel_label: Option<String>,
    /// Style the confirm button as destructive.
    destructive: Option<bool>,
    /// Called when the user confirms.
    on_confirm: EventHandler<()>,
    /// Called when the user cancels, including with Escape or a tap outside.
    on_cancel: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the dialog surface.
    class: Option<String>,
    /// Extra content between the message and the buttons.
    children: Element,
) -> Element {
    let strings = use_strings();
    let confirm_label = confirm_label.unwrap_or(strings.confirm);
    let cancel_label = cancel_label.unwrap_or(strings.cancel);
    let cancel = move |_| {
        open.set(false);
        if let Some(on_cancel) = on_cancel {
            on_cancel.call(());
        }
    };
    let confirm_color = if destructive.unwrap_or(false) {
        Color::Danger
    } else {
        Color::Accent
    };
    rsx! {
        Modal {
            open,
            title,
            role: ModalRole::AlertDialog,
            on_dismiss: move |_| {
                if let Some(on_cancel) = on_cancel {
                    on_cancel.call(());
                }
            },
            mode,
            class,
            actions: rsx! {
                Button { fill: ButtonFill::Outline, color: Color::Neutral, mode, onclick: cancel,
                    "{cancel_label}"
                }
                Button {
                    color: confirm_color,
                    mode,
                    onclick: move |_| {
                        open.set(false);
                        on_confirm.call(());
                    },
                    "{confirm_label}"
                }
            },
            if let Some(message) = message {
                p { "{message}" }
            }
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ConfirmModalPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let title = use_signal(|| "Delete game?".to_string());
    let confirm_label = use_signal(|| "Delete".to_string());
    let destructive = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Input { label: "Title", value: title }
                crate::Input { label: "Confirm label", value: confirm_label }
                crate::Checkbox { checked: destructive, label: "Destructive" }
            },
            Button { onclick: move |_| open.set(true), "Open confirmation" }
            ConfirmModal {
                open,
                title: title(),
                message: "This cannot be undone.",
                confirm_label: confirm_label(),
                destructive: destructive(),
                on_confirm: |_| {},
            }
        }
    }
}

crate::g3_playground! {
    name: "ConfirmModal",
    description: "Confirmation dialog, with a destructive variant.",
    demo: ConfirmModalPlaygroundDemo,
    source: "src/components/confirm_modal.rs",
}
