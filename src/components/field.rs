//! Field (text input) component with iOS/Android styling.

use super::field_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[component]
pub fn Field(
    #[props(extends=input)] attributes: Vec<Attribute>,
    label: String,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: EventHandler<Event<FormData>>,
    debounce: Option<u32>,
    value: ReadSignal<String>,
    disabled: Option<bool>,
    maxlength: Option<usize>,
    minlength: Option<usize>,
    min: Option<isize>,
    max: Option<isize>,
    r#type: Option<String>,
    placeholder: Option<String>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut modified = use_signal(|| false);
    let mut reapply_value = use_signal(|| false);
    let mut debounce_generation = use_signal(|| 0_u64);
    let mode = use_component_mode(mode);

    let mode_cls = match mode {
        ComponentMode::Ios => s::FIELD_IOS,
        ComponentMode::Md => s::FIELD_MD,
    };
    let field_cls = merge_classes(format!("{} {mode_cls}", s::FIELD), class.as_deref());

    let id = label.clone();
    let r#for = label.clone();

    let combo_value = use_memo(move || {
        if reapply_value() {
            format!("{}0", value())
        } else {
            format!("{}1", value())
        }
    });

    let mut check_validity = move |new_value: String| -> bool {
        if let Some(max) = max {
            if let Ok(v) = new_value.parse::<isize>() {
                if v > max {
                    reapply_value.toggle();
                    return false;
                }
            }
        }
        if let Some(min) = min {
            if let Ok(v) = new_value.parse::<isize>() {
                if v < min {
                    reapply_value.toggle();
                    return false;
                }
            }
        }
        if let Some(minlength) = minlength {
            if new_value.len() < minlength {
                reapply_value.toggle();
                return false;
            }
        }
        if let Some(maxlength) = maxlength {
            if new_value.len() > maxlength {
                reapply_value.toggle();
                return false;
            }
        }
        true
    };

    rsx! {
        div { class: "relative w-full",
            label { r#for,
                div { class: "text-sm font-medium mb-1", "{label}" }
            }
            input {
                id,
                class: field_cls,
                style: "width: stretch;",
                onfocus: move |_| { is_focused.set(true); },
                onblur: move |_| { is_focused.set(false); },
                oninput: move |event: Event<FormData>| {
                    if let Some(oninput) = oninput {
                        oninput.call(event.clone());
                    }
                    if let Some(d) = debounce {
                        let generation = debounce_generation.with_mut(|generation| {
                            *generation += 1;
                            *generation
                        });
                        if !check_validity(event.value()) { return; }
                        let ev = event;
                        spawn(async move {
                            dioxus_sdk_time::sleep(std::time::Duration::from_millis(d as u64)).await;
                            if debounce_generation() != generation { return; }
                            modified.set(true);
                            onchange.call(ev);
                        });
                    }
                },
                onchange: move |event| {
                    if debounce.is_some() { return; }
                    if !check_validity(event.value()) { return; }
                    modified.set(true);
                    onchange.call(event);
                },
                value: &combo_value()[..combo_value().len() - 1],
                min,
                max,
                minlength,
                maxlength,
                placeholder,
                disabled,
                r#type,
                ..attributes,
            }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn FieldPlaygroundDemo() -> Element {
    let value = use_signal(String::new);
    let mut label = use_signal(|| "Club".to_string());
    let mut placeholder = use_signal(|| "Club name".to_string());
    let mut disabled = use_signal(|| false);
    let mut numeric = use_signal(|| false);

    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-control", span { "Label" } input { value: "{label()}", oninput: move |event: Event<FormData>| label.set(event.value()) } }
                label { class: "g3-playground-control", span { "Placeholder" } input { value: "{placeholder()}", oninput: move |event: Event<FormData>| placeholder.set(event.value()) } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: disabled(), onchange: move |_| disabled.toggle() } span { "Disabled" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: numeric(), onchange: move |_| numeric.toggle() } span { "Number type" } }
            },
            Field {
                label: label(),
                value,
                placeholder: placeholder(),
                disabled: disabled(),
                r#type: if numeric() { "number" } else { "text" },
                onchange: |_| {},
            }
        }
    }
}
crate::g3_playground! {
    name: "Field",
    g3_name: "G3Field",
    description: "Text input with optional validation limits and debounce.",
    demo: FieldPlaygroundDemo,
}
