//! Button component with iOS/Android mode support.

use super::button_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Visual variant of the button.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonStyle {
    #[default]
    Solid,
    Outline,
    Clear,
    Neutral,
}

/// Button size.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Button(
    style: Option<ButtonStyle>,
    size: Option<ButtonSize>,
    disabled: Option<bool>,
    expand: Option<bool>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    start: Option<Element>,
    onclick: Callback<Event<MouseData>>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let mode_cls = match mode {
        ComponentMode::Ios => s::IOS,
        ComponentMode::Md => s::MD,
    };

    let style_cls = match style.unwrap_or_default() {
        ButtonStyle::Solid => s::SOLID,
        ButtonStyle::Outline => s::OUTLINE,
        ButtonStyle::Clear => s::CLEAR,
        ButtonStyle::Neutral => s::NEUTRAL,
    };

    let size_cls = match size.unwrap_or_default() {
        ButtonSize::Sm => s::SM,
        ButtonSize::Md => s::MD_SIZE,
        ButtonSize::Lg => s::LG,
    };

    let expand_cls = if expand.unwrap_or(false) {
        "w-full"
    } else {
        ""
    };
    let is_disabled = disabled.unwrap_or(false);

    let cls = merge_classes(
        format!("{} {mode_cls} {style_cls} {size_cls} {expand_cls}", s::BASE),
        class.as_deref(),
    );

    rsx! {
        button {
            class: cls,
            r#type: "button",
            disabled: is_disabled,
            onclick: move |event| {
                onclick.call(event);
            },
            if let Some(start) = start {
                div { class: "g3-btn-content g3-btn-content-start",
                    span { class: "g3-btn-start", {start} }
                    span { class: "g3-btn-label", {children} }
                    span { class: "g3-btn-end-spacer" }
                }
            } else {
                div { class: "g3-btn-content", {children} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn ButtonPlaygroundDemo() -> Element {
    let mut style = use_signal(ButtonStyle::default);
    let mut size = use_signal(ButtonSize::default);
    let mut disabled = use_signal(|| false);
    let mut expand = use_signal(|| false);
    let mut label = use_signal(|| "Create".to_string());

    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-control",
                    span { "Label" }
                    input { value: "{label()}", oninput: move |event: Event<FormData>| label.set(event.value()) }
                }
                div { class: "g3-playground-control",
                    span { "Style" }
                    div { class: "g3-playground-segments",
                        button { class: if style() == ButtonStyle::Solid { "selected" } else { "" }, r#type: "button", onclick: move |_| style.set(ButtonStyle::Solid), "Solid" }
                        button { class: if style() == ButtonStyle::Outline { "selected" } else { "" }, r#type: "button", onclick: move |_| style.set(ButtonStyle::Outline), "Outline" }
                        button { class: if style() == ButtonStyle::Clear { "selected" } else { "" }, r#type: "button", onclick: move |_| style.set(ButtonStyle::Clear), "Clear" }
                        button { class: if style() == ButtonStyle::Neutral { "selected" } else { "" }, r#type: "button", onclick: move |_| style.set(ButtonStyle::Neutral), "Neutral" }
                    }
                }
                div { class: "g3-playground-control",
                    span { "Size" }
                    div { class: "g3-playground-segments",
                        button { class: if size() == ButtonSize::Sm { "selected" } else { "" }, r#type: "button", onclick: move |_| size.set(ButtonSize::Sm), "Sm" }
                        button { class: if size() == ButtonSize::Md { "selected" } else { "" }, r#type: "button", onclick: move |_| size.set(ButtonSize::Md), "Md" }
                        button { class: if size() == ButtonSize::Lg { "selected" } else { "" }, r#type: "button", onclick: move |_| size.set(ButtonSize::Lg), "Lg" }
                    }
                }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: disabled(), onchange: move |_| disabled.toggle() } span { "Disabled" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: expand(), onchange: move |_| expand.toggle() } span { "Expand" } }
            },
            Button { style: style(), size: size(), disabled: disabled(), expand: expand(), onclick: |_| {}, "{label()}" }
        }
    }
}
crate::g3_playground! {
    name: "Button",
    g3_name: "G3Button",
    description: "Ionic-style action button with solid, outline, clear, and neutral variants.",
    demo: ButtonPlaygroundDemo,
}
