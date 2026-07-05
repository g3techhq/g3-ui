//! Toggle (switch) component with iOS/Android styling.

use super::toggle_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ToggleSize {
    #[default]
    Sm,
    Md,
}

#[component]
pub fn Toggle(
    mut checked: Signal<bool>,
    on_checked_change: Option<Callback<bool>>,
    size: Option<ToggleSize>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);

    let switch_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::SWITCH, s::SWITCH_IOS),
        ComponentMode::Md => format!("{} {}", s::SWITCH, s::SWITCH_MD),
    };
    let thumb_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::THUMB, s::THUMB_IOS),
        ComponentMode::Md => format!("{} {}", s::THUMB, s::THUMB_MD),
    };
    let size_cls = if size.unwrap_or_default() == ToggleSize::Sm {
        s::SM
    } else {
        ""
    };

    rsx! {
        button {
            class: merge_classes(
                format!("{switch_cls} {size_cls} {}", if checked() { "checked" } else { "" }),
                class.as_deref(),
            ),
            r#type: "button",
            role: "switch",
            aria_checked: checked().to_string(),
            onclick: move |_| {
                let new_checked = !checked();
                checked.set(new_checked);
                if let Some(ref on_checked_change) = on_checked_change {
                    on_checked_change.call(new_checked);
                }
            },
            span { class: thumb_cls, aria_hidden: "true" }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn TogglePlaygroundDemo() -> Element {
    let mut checked = use_signal(|| true);
    let mut large = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: checked(), onchange: move |_| checked.toggle() } span { "Checked" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: large(), onchange: move |_| large.toggle() } span { "Large" } }
            },
            Toggle { checked, size: if large() { ToggleSize::Md } else { ToggleSize::Sm } }
        }
    }
}

crate::g3_playground! {
    name: "Toggle",
    g3_name: "G3Toggle",
    description: "Platform-styled switch control.",
    demo: TogglePlaygroundDemo,
    source: "src/components/toggle.rs",
}
