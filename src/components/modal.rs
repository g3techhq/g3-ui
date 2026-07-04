//! Modal component - generic alert dialog surface with platform styling.

use super::modal_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[component]
pub fn Modal(
    open: Signal<bool>,
    title: String,
    description: Option<Element>,
    actions: Option<Element>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    children: Option<Element>,
) -> Element {
    let mode = use_component_mode(mode);
    let state = if open() { "open" } else { "closed" };

    let modal_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::MODAL, s::MODAL_IOS),
        ComponentMode::Md => format!("{} {}", s::MODAL, s::MODAL_MD),
    };

    rsx! {
        div {
            class: s::OVERLAY,
            "data-state": state,
            aria_hidden: (!open()).to_string(),
            onclick: move |_| open.set(false),
            div {
                class: merge_classes(
                    format!("{modal_cls} g3-modal-card"),
                    class.as_deref(),
                ),
                role: "alertdialog",
                aria_modal: "true",
                "data-state": state,
                onclick: move |event| event.stop_propagation(),
                h2 { class: s::TITLE, "{title}" }
                if let Some(description) = description {
                    div { class: "g3-modal-description", {description} }
                }
                if let Some(children) = children {
                    div { class: "g3-modal-body", {children} }
                }
                if let Some(actions) = actions {
                    div { class: s::ACTIONS, {actions} }
                }
            }
        }
    }
}
