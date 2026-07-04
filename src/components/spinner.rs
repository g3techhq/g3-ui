//! Spinner component - loading indicator with CSS animation.

use super::spinner_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[component]
pub fn Spinner(class: Option<String>, center: Option<bool>) -> Element {
    let wrapper_cls = if center.unwrap_or(false) {
        format!("{} {}", s::WRAPPER, s::CENTERED)
    } else {
        s::WRAPPER.to_string()
    };

    rsx! {
        div {
            class: merge_classes(wrapper_cls, class.as_deref()),
            role: "status",
            aria_live: "polite",
            aria_label: "Loading spinner",
            div { class: s::SPINNER, aria_hidden: "true" }
            span { class: "sr-only", "Loading" }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn SpinnerPlaygroundDemo() -> Element {
    let mut center = use_signal(|| true);
    let box_cls = if center() {
        "g3-playground-spinner-box g3-playground-spinner-box-centered"
    } else {
        "g3-playground-spinner-box"
    };

    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: center(), onchange: move |_| center.toggle() } span { "Center" } }
            },
            div { class: box_cls,
                Spinner { center: center() }
            }
        }
    }
}
crate::g3_playground! {
    name: "Spinner",
    g3_name: "G3Spinner",
    description: "Accessible loading indicator.",
    demo: SpinnerPlaygroundDemo,
}
