//! Field (text input) component with iOS/Android styling.

use super::field_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[component]
pub fn Field(
    #[props(extends=input)] attributes: Vec<Attribute>,
    label: String,
    mut value: Signal<String>,
    oninput: Option<EventHandler<Event<FormData>>>,
    onchange: Option<EventHandler<Event<FormData>>>,
    debounce: Option<u32>,
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
                    let new_value = event.value();
                    if !check_validity(new_value.clone()) {
                        return;
                    }
                    value.set(new_value);
                    modified.set(true);
                    if let Some(oninput) = oninput {
                        oninput.call(event.clone());
                    }
                    if let Some(d) = debounce {
                        let generation = debounce_generation.with_mut(|generation| {
                            *generation += 1;
                            *generation
                        });
                        let ev = event;
                        spawn(async move {
                            dioxus_sdk_time::sleep(std::time::Duration::from_millis(d as u64)).await;
                            if debounce_generation() != generation { return; }
                            if let Some(onchange) = onchange {
                                onchange.call(ev);
                            }
                        });
                    }
                },
                onchange: move |event| {
                    if debounce.is_some() { return; }
                    if let Some(onchange) = onchange {
                        onchange.call(event);
                    }
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
    let label = use_signal(|| "Club".to_string());
    let placeholder = use_signal(|| "Club name".to_string());
    let disabled = use_signal(|| false);
    let numeric = use_signal(|| false);

    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Field { label: "Label".to_string(), value: label }
                crate::Field { label: "Placeholder".to_string(), value: placeholder }
                crate::Checkbox { checked: disabled, label: "Disabled".to_string() }
                crate::Checkbox { checked: numeric, label: "Number type".to_string() }
            },
            Field {
                label: label(),
                value,
                placeholder: placeholder(),
                disabled: disabled(),
                r#type: if numeric() { "number" } else { "text" },
            }
        }
    }
}
crate::g3_playground! {
    name: "Field",
    g3_name: "G3Field",
    description: "Text input with optional validation limits and debounce.",
    demo: FieldPlaygroundDemo,
    source: "src/components/field.rs",
}
