//! Radio group and radio components.

use super::checkbox::ControlLabelPlacement;
use super::radio_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone)]
struct RadioGroupContext {
    value: Signal<String>,
    disabled: bool,
    allow_empty_selection: bool,
    on_change: Option<Callback<String>>,
}

#[component]
pub fn RadioGroup(
    value: Signal<String>,
    disabled: Option<bool>,
    allow_empty_selection: Option<bool>,
    class: Option<String>,
    on_change: Option<Callback<String>>,
    children: Element,
) -> Element {
    provide_context(RadioGroupContext {
        value,
        disabled: disabled.unwrap_or(false),
        allow_empty_selection: allow_empty_selection.unwrap_or(false),
        on_change,
    });

    rsx! {
        div {
            class: merge_classes(s::GROUP, class.as_deref()),
            role: "radiogroup",
            {children}
        }
    }
}

#[component]
pub fn Radio(
    value: String,
    label: Option<String>,
    disabled: Option<bool>,
    placement: Option<ControlLabelPlacement>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);
    let mut context = use_context::<RadioGroupContext>();
    let selected = (context.value)() == value;
    let is_disabled = disabled.unwrap_or(false) || context.disabled;
    let placement = placement.unwrap_or_default();
    let mode_cls = match mode {
        ComponentMode::Ios => s::RADIO_IOS,
        ComponentMode::Md => s::RADIO_MD,
    };
    let cls = merge_classes(
        format!("{} {mode_cls} {}", s::RADIO, placement.class()),
        class.as_deref(),
    );
    let aria_checked = selected.to_string();
    let tab_index = if is_disabled { "-1" } else { "0" };

    rsx! {
        button {
            class: cls,
            r#type: "button",
            role: "radio",
            aria_checked,
            disabled: is_disabled,
            tabindex: tab_index,
            onclick: move |_| {
                if is_disabled {
                    return;
                }

                let next = if *(context.value).peek() == value && context.allow_empty_selection {
                    String::new()
                } else {
                    value.clone()
                };

                if *(context.value).peek() == next {
                    return;
                }

                (context.value).set(next.clone());
                if let Some(ref on_change) = context.on_change {
                    on_change.call(next);
                }
            },
            span { class: s::CONTROL, aria_hidden: "true",
                span { class: s::MARK, aria_hidden: "true" }
            }
            if let Some(label) = label.filter(|value| !value.is_empty()) {
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn RadioPlaygroundDemo() -> Element {
    let selected = use_signal(|| "push".to_string());
    let mut disabled = use_signal(|| false);
    let mut allow_empty = use_signal(|| false);

    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: disabled(), onchange: move |_| disabled.toggle() } span { "Disabled" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: allow_empty(), onchange: move |_| allow_empty.toggle() } span { "Allow empty" } }
            },
            div { class: "g3-radio-demo-stack",
                RadioGroup {
                    value: selected,
                disabled: disabled(),
                allow_empty_selection: allow_empty(),
                Radio { value: "push", label: "Push notifications" }
                Radio { value: "email", label: "Email summaries" }
                    Radio {
                        value: "none",
                        label: "No reminders",
                        placement: ControlLabelPlacement::End,
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Radio",
    g3_name: "G3RadioGroup / G3Radio",
    description: "Single-select radio group.",
    demo: RadioPlaygroundDemo,
    source: "src/components/radio.rs",
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::G3ThemeProvider;

    fn render(app: fn() -> Element) {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
    }

    #[component]
    fn RadioSmokeApp() -> Element {
        let selected = use_signal(|| "walking".to_string());

        rsx! {
            G3ThemeProvider { mode: ComponentMode::Ios,
                RadioGroup { value: selected, on_change: |_| {},
                    Radio { value: "walking", label: "Walking" }
                    Radio {
                        value: "riding",
                        label: "Riding",
                        placement: ControlLabelPlacement::End,
                    }
                }
            }
        }
    }

    #[test]
    fn radio_group_renders() {
        render(RadioSmokeApp);
    }
}
