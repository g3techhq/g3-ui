//! Card component - container with title, optional right slot, and body.

use super::card_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum RightSlot {
    Text(String),
    Element(Element),
}

#[component]
pub fn Card(
    title: Option<String>,
    right_slot: Option<RightSlot>,
    image: Option<Element>,
    selected: Option<bool>,
    inset: Option<bool>,
    top_margin: Option<bool>,
    onclick: Option<Callback<Event<MouseData>>>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let has_top_margin = top_margin.unwrap_or(false);

    let platform_cls = match mode {
        ComponentMode::Ios => s::CARD_IOS,
        ComponentMode::Md => s::CARD_MD,
    };
    let card_cls = if inset.unwrap_or(false) {
        format!("{} {platform_cls} {}", s::CARD, s::INSET)
    } else {
        format!("{} {platform_cls}", s::CARD)
    };

    let is_selected = selected.unwrap_or(false);
    let title = title.filter(|title| !title.is_empty());
    let root_cls = merge_classes(
        format!(
            "{card_cls} {} {} {} {}",
            if onclick.is_some() {
                s::INTERACTIVE
            } else {
                ""
            },
            if title.is_none() { s::CONTROL } else { "" },
            if is_selected { s::SELECTED } else { "" },
            if has_top_margin { "mt-3" } else { "" }
        ),
        class.as_deref(),
    );
    let role = if onclick.is_some() { "button" } else { "group" };
    let tabindex = if onclick.is_some() { "0" } else { "-1" };

    rsx! {
        div {
            class: root_cls,
            role,
            tabindex,
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            if let Some(image) = image {
                div { class: "mb-2 justify-items-center", {image} }
            }
            if let Some(title) = title.as_deref() {
                div { class: s::HEADER,
                    div { class: s::TITLE, "{title}" }
                    match right_slot {
                        Some(RightSlot::Text(text)) => rsx! {
                            div { class: s::RIGHT_TEXT, "{text}" }
                        },
                        Some(RightSlot::Element(element)) => rsx! { {element} },
                        None => rsx! {},
                    }
                }
            } else if let Some(right_slot) = right_slot {
                div { class: s::HEADER,
                    div {}
                    match right_slot {
                        RightSlot::Text(text) => rsx! {
                            div { class: s::RIGHT_TEXT, "{text}" }
                        },
                        RightSlot::Element(element) => rsx! { {element} },
                    }
                }
            }
            div { class: s::BODY, {children} }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn CardPlaygroundDemo() -> Element {
    let mut title = use_signal(|| "Game".to_string());
    let mut inset = use_signal(|| false);
    let mut selected = use_signal(|| false);
    let mut interactive = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                label { class: "g3-playground-control", span { "Title" } input { value: "{title()}", oninput: move |event: Event<FormData>| title.set(event.value()) } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: inset(), onchange: move |_| inset.toggle() } span { "Inset" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: selected(), onchange: move |_| selected.toggle() } span { "Selected" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: interactive(), onchange: move |_| interactive.toggle() } span { "Interactive" } }
            },
            Card {
                title: title(),
                inset: inset(),
                selected: selected(),
                onclick: if interactive() { Some(Callback::new(|_| {})) } else { None },
                right_slot: RightSlot::Text("Ready".to_string()),
                "Traditional match play"
            }
        }
    }
}
crate::g3_playground! {
    name: "Card",
    g3_name: "G3Card",
    description: "Ionic-style content card with title, body, and optional right slot.",
    demo: CardPlaygroundDemo,
    source: "src/components/card.rs",
}
