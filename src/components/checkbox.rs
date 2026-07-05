//! Checkbox component with Ionic-style label placement.

use super::checkbox_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlLabelPlacement {
    #[default]
    Start,
    End,
    Fixed,
    Stacked,
}

impl ControlLabelPlacement {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Start => s::PLACEMENT_START,
            Self::End => s::PLACEMENT_END,
            Self::Fixed => s::PLACEMENT_FIXED,
            Self::Stacked => s::PLACEMENT_STACKED,
        }
    }
}

#[component]
pub fn Checkbox(
    mut checked: Signal<bool>,
    label: String,
    indeterminate: Option<bool>,
    disabled: Option<bool>,
    error: Option<String>,
    hint: Option<String>,
    label_placement: Option<ControlLabelPlacement>,
    mode: Option<ComponentMode>,
    class: Option<String>,
    onchange: Option<Callback<bool>>,
) -> Element {
    let mode = use_component_mode(mode);
    let is_checked = checked();
    let is_indeterminate = indeterminate.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);
    let has_error = error.as_ref().is_some_and(|value| !value.is_empty());
    let placement = label_placement.unwrap_or_default();
    let mode_cls = match mode {
        ComponentMode::Ios => s::CHECKBOX_IOS,
        ComponentMode::Md => s::CHECKBOX_MD,
    };
    let checked_cls = if is_checked { "checked" } else { "" };
    let indeterminate_cls = if is_indeterminate {
        "indeterminate"
    } else {
        ""
    };
    let disabled_cls = if is_disabled { "disabled" } else { "" };
    let invalid_cls = if has_error { "invalid" } else { "" };
    let cls = merge_classes(
        format!(
            "{} {mode_cls} {} {checked_cls} {indeterminate_cls} {disabled_cls} {invalid_cls}",
            s::CHECKBOX,
            placement.class(),
        ),
        class.as_deref(),
    );
    let control_id = checkbox_id(&label);
    let hint_id = format!("{control_id}-hint");
    let error_id = format!("{control_id}-error");
    let aria_checked = if is_indeterminate {
        "mixed".to_string()
    } else {
        is_checked.to_string()
    };
    let aria_describedby = describedby(&hint, &error, &hint_id, &error_id);

    rsx! {
        button {
            class: cls,
            r#type: "button",
            role: "checkbox",
            aria_checked,
            aria_invalid: has_error.to_string(),
            aria_describedby,
            disabled: is_disabled,
            onclick: move |_| {
                if is_disabled {
                    return;
                }
                let next = !checked();
                checked.set(next);
                if let Some(ref onchange) = onchange {
                    onchange.call(next);
                }
            },
            span { class: s::CONTROL, aria_hidden: "true",
                span { class: s::MARK }
            }
            span { class: s::LABEL, "{label}" }
            if let Some(hint) = hint.filter(|value| !value.is_empty()) {
                span { id: hint_id, class: s::HINT, "{hint}" }
            }
            if let Some(error) = error.filter(|value| !value.is_empty()) {
                span { id: error_id, class: s::ERROR, "{error}" }
            }
        }
    }
}

fn checkbox_id(label: &str) -> String {
    let mut slug = String::with_capacity(label.len());
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "g3-checkbox".to_string()
    } else {
        format!("g3-checkbox-{slug}")
    }
}

fn describedby(
    hint: &Option<String>,
    error: &Option<String>,
    hint_id: &str,
    error_id: &str,
) -> Option<String> {
    let mut ids = Vec::new();
    if hint.as_ref().is_some_and(|value| !value.is_empty()) {
        ids.push(hint_id);
    }
    if error.as_ref().is_some_and(|value| !value.is_empty()) {
        ids.push(error_id);
    }
    (!ids.is_empty()).then(|| ids.join(" "))
}

#[cfg(feature = "playground")]
#[component]
pub fn CheckboxPlaygroundDemo() -> Element {
    let checked = use_signal(|| true);
    let mut disabled = use_signal(|| false);
    let mut indeterminate = use_signal(|| false);

    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: disabled(), onchange: move |_| disabled.toggle() } span { "Disabled" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: indeterminate(), onchange: move |_| indeterminate.toggle() } span { "Indeterminate" } }
            },
            div { class: "g3-checkbox-demo-stack",
                Checkbox {
                    checked,
                    label: "Push notifications",
                    hint: "Course updates and tee-time reminders",
                    indeterminate: indeterminate(),
                    disabled: disabled(),
                    onchange: move |_| indeterminate.set(false),
                }
                Checkbox {
                    checked: use_signal(|| false),
                    label: "Share scorecard",
                    label_placement: ControlLabelPlacement::End,
                    error: "Requires a signed-in player",
                }
                Checkbox {
                    checked: use_signal(|| true),
                    label: "Skins game",
                    label_placement: ControlLabelPlacement::Stacked,
                    hint: "Shown as a stacked mobile setting row",
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Checkbox",
    g3_name: "G3Checkbox",
    description: "Controlled checkbox with Ionic-style label placement.",
    demo: CheckboxPlaygroundDemo,
    source: "src/components/checkbox.rs",
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
    fn CheckboxSmokeApp() -> Element {
        let checked = use_signal(|| false);

        rsx! {
            G3ThemeProvider { mode: ComponentMode::Ios,
                Checkbox {
                    checked,
                    label: "Accept terms",
                    hint: "Required before play",
                    onchange: |_| {},
                }
            }
        }
    }

    #[test]
    fn checkbox_renders() {
        render(CheckboxSmokeApp);
    }
}
