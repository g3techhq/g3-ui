//! Centred dialogs.
use super::overlay::{use_lock_body_scroll, use_overlay_focus};
use crate::state::{use_element_id, use_synced_signal};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// The ARIA role of a [`Modal`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ModalRole {
    /// A general dialog.
    #[default]
    Dialog,
    /// A dialog that interrupts to ask for a response, such as a
    /// confirmation. Screen readers announce it immediately.
    AlertDialog,
}

/// Width of a [`Modal`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ModalSize {
    /// Alert width, for short messages and confirmations.
    #[default]
    Compact,
    /// Wider, for forms and longer content.
    Wide,
}

/// A centred dialog over a dimmed page.
///
/// While open, it holds keyboard focus, closes on Escape (unless
/// `dismissible` is false), and returns focus to where it was when it
/// closes. The title names the dialog and the content describes it.
///
/// ```rust,ignore
/// rsx! {
///     Modal { open, title: "Rename round",
///         actions: rsx! {
///             Button { fill: ButtonFill::Clear, onclick: move |_| open.set(false), "Cancel" }
///             Button { onclick: move |_| save(), "Save" }
///         },
///         Input { label: "Name", value: name }
///     }
/// }
/// ```
#[component]
pub fn Modal(
    /// Whether the dialog is open.
    open: Signal<bool>,
    /// Heading, which also names the dialog.
    title: Option<String>,
    /// Accessible name when there is no `title`.
    aria_label: Option<String>,
    /// Buttons along the bottom.
    actions: Option<Element>,
    /// ARIA role. Defaults to [`ModalRole::Dialog`].
    role: Option<ModalRole>,
    /// Width. Defaults to [`ModalSize::Compact`].
    size: Option<ModalSize>,
    /// Close on Escape and on a tap outside. Defaults to `true`.
    dismissible: Option<bool>,
    /// Called after the user dismisses the dialog.
    on_dismiss: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the dialog surface.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("modal", None);
    let title_id = format!("{id}-title");
    let body_id = format!("{id}-body");
    let dismissible = use_synced_signal(dismissible.unwrap_or(true));
    let dismiss = use_callback(move |()| {
        if !dismissible() {
            return;
        }
        let mut open = open;
        open.set(false);
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.call(());
        }
    });
    use_lock_body_scroll(open.into());
    use_overlay_focus(open.into(), id.clone(), true, dismiss);
    let mut ever_opened = use_signal(|| false);
    use_effect(move || {
        if open() {
            ever_opened.set(true);
        }
    });
    let is_open = open();
    if !is_open && !ever_opened() {
        return rsx! {};
    }
    let state = if is_open { "open" } else { "closed" };
    let cls = classes([
        "g3-modal",
        mode.pick("g3-modal-ios", "g3-modal-md"),
        match size.unwrap_or_default() {
            ModalSize::Compact => "",
            ModalSize::Wide => "g3-modal-wide",
        },
    ]);
    let name = if title.is_none() { aria_label } else { None };
    let labelledby = title.is_some().then(|| title_id.clone());
    let role = match role.unwrap_or_default() {
        ModalRole::Dialog => "dialog",
        ModalRole::AlertDialog => "alertdialog",
    };
    rsx! {
        div {
            class: "g3-modal-overlay",
            "data-state": state,
            aria_hidden: (!is_open).then_some("true"),
            inert: (!is_open).then_some(true),
            onclick: move |_| dismiss.call(()),
            div {
                id,
                class: merge_classes(cls, class.as_deref()),
                role,
                aria_modal: "true",
                aria_label: name,
                aria_labelledby: labelledby,
                aria_describedby: body_id.clone(),
                tabindex: "-1",
                onclick: |event| event.stop_propagation(),
                if let Some(title) = title {
                    h2 { id: title_id, class: "g3-modal-title", "{title}" }
                }
                div { id: body_id, class: "g3-modal-body", {children} }
                if let Some(actions) = actions {
                    div { class: "g3-modal-actions", {actions} }
                }
            }
        }
    }
}
