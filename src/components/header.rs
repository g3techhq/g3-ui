//! Header component with start/end buttons and an optional toolbar.

use super::header_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[component]
pub fn Header(
    title: String,
    start_button: Option<Element>,
    end_button: Option<Element>,
    toolbar: Option<Element>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);

    let header_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::HEADER_BASE, s::HEADER_IOS),
        ComponentMode::Md => format!("{} {}", s::HEADER_BASE, s::HEADER_MD),
    };

    rsx! {
        header {
            class: merge_classes(format!("{header_cls} relative"), class.as_deref()),
            div { class: s::HEADER_ROW,
                div { class: s::HEADER_START_SLOT,
                    if let Some(start) = start_button {
                        {start}
                    }
                }
                h1 { class: s::HEADER_TITLE, "{title}" }
                div { class: s::HEADER_END_SLOT,
                    if let Some(end) = end_button {
                        {end}
                    }
                }
            }
            if let Some(t) = toolbar {
                div { class: s::TOOLBAR,
                    {t}
                }
            }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn HeaderPlaygroundDemo() -> Element {
    let active = use_signal(|| 0_usize);
    let mut title = use_signal(|| "Pending Game".to_string());
    let mut toolbar = use_signal(|| true);
    let mut start_button = use_signal(|| true);
    let mut end_button = use_signal(|| true);
    let mut start_text = use_signal(|| "Close".to_string());
    let mut end_text = use_signal(|| "Create".to_string());
    let playground_mode = crate::use_component_mode(None);
    let toolbar_slot = toolbar().then(|| {
        rsx! {
            crate::SegmentGroup { active, toolbar: true,
                crate::SegmentButton { index: 0, "Players" }
                crate::SegmentButton { index: 1, "Bet" }
            }
        }
    });

    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                label { class: "g3-playground-control", span { "Title" } input { value: "{title()}", oninput: move |event: Event<FormData>| title.set(event.value()) } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: start_button(), onchange: move |_| start_button.toggle() } span { "Start button" } }
                label { class: "g3-playground-control", span { "Start text" } input { value: "{start_text()}", oninput: move |event: Event<FormData>| start_text.set(event.value()) } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: end_button(), onchange: move |_| end_button.toggle() } span { "End button" } }
                label { class: "g3-playground-control", span { "End text" } input { value: "{end_text()}", oninput: move |event: Event<FormData>| end_text.set(event.value()) } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: toolbar(), onchange: move |_| toolbar.toggle() } span { "Toolbar" } }
            },
            crate::AppWrapper { mode: playground_mode, class: "g3-playground-device-app",
                Header {
                    title: title(),
                    start_button: start_button().then(|| rsx! { crate::Button { style: crate::ButtonStyle::Clear, onclick: |_| {}, "{start_text()}" } }),
                    end_button: end_button().then(|| rsx! { crate::Button { style: crate::ButtonStyle::Outline, onclick: |_| {}, "{end_text()}" } }),
                    toolbar: toolbar_slot,
                }
                crate::Body { has_footer_space: false,
                    crate::Card { title: "Preview", "Toolbar stays attached to the header." }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Header",
    g3_name: "G3Header",
    description: "App header with start, title, end, and toolbar slots.",
    demo: HeaderPlaygroundDemo,
}
