//! Checkboxes, and the label layout shared with toggles and radios.
use super::field::{described_by, is_invalid};
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Where a checkbox, toggle, or radio sits relative to its label.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ControlLabelPlacement {
    /// Label first, control at the trailing edge, as in iOS settings.
    #[default]
    Start,
    /// Control first, label after it.
    End,
    /// Label in a fixed-width leading column, so labels align across rows.
    Fixed,
    /// Label above the control.
    Stacked,
}

impl ControlLabelPlacement {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Start => "g3-control-label-start",
            Self::End => "",
            Self::Fixed => "g3-control-label-fixed",
            Self::Stacked => "g3-control-label-stacked",
        }
    }
}

/// Classes shared by every control row.
pub(crate) fn control_classes(
    kind: &'static str,
    mode: ComponentMode,
    placement: ControlLabelPlacement,
    bare: bool,
) -> String {
    classes([
        "g3-control",
        kind,
        mode.pick("g3-control-ios", "g3-control-md"),
        if bare {
            "g3-control-bare"
        } else {
            placement.class()
        },
    ])
}

/// A checkbox row: the box, a label, and optional helper and error text. The
/// whole row is the control. Like Ionic's `ion-checkbox`.
///
/// ```rust,ignore
/// let agreed = use_signal(|| false);
/// rsx! {
///     Checkbox { checked: agreed, label: "I agree to the terms",
///         error: (!agreed()).then(|| "Required".to_string()) }
/// }
/// ```
#[component]
pub fn Checkbox(
    /// Whether it is checked. Kept internally when not given.
    checked: Option<Signal<bool>>,
    /// Visible label, which also names the checkbox.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Show a dash for a partly selected group. Pressing still toggles
    /// `checked`.
    indeterminate: Option<bool>,
    /// Help text under the label.
    helper: Option<String>,
    /// Error text under the label. Marks the checkbox invalid when not empty.
    error: Option<String>,
    /// Disable the checkbox.
    disabled: Option<bool>,
    /// Label position. Defaults to [`ControlLabelPlacement::Start`].
    label_placement: Option<ControlLabelPlacement>,
    /// Called with the new state.
    onchange: Option<EventHandler<bool>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the row.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("checkbox", id);
    let mut checked = use_controlled(checked, || false);
    let aria_checked = if indeterminate.unwrap_or(false) {
        "mixed"
    } else if checked() {
        "true"
    } else {
        "false"
    };
    let bare = label.is_none() && helper.is_none() && error.is_none();
    let cls = control_classes(
        "g3-checkbox",
        mode,
        label_placement.unwrap_or_default(),
        bare,
    );
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    rsx! {
        button {
            id: id.clone(),
            class: merge_classes(cls, class.as_deref()),
            r#type: "button",
            role: "checkbox",
            aria_checked,
            aria_label,
            aria_describedby: describedby,
            aria_invalid: invalid.then_some("true"),
            disabled,
            onclick: move |_| {
                let next = !checked();
                checked.set(next);
                if let Some(onchange) = onchange {
                    onchange.call(next);
                }
            },
            span { class: "g3-control-mark", aria_hidden: "true",
                span { class: "g3-checkbox-box",
                    span { class: "g3-checkbox-check" }
                }
            }
            ControlText { id: id.clone(), label, helper, error }
        }
    }
}

/// Label, helper, and error text inside a control row.
#[component]
pub(crate) fn ControlText(
    id: String,
    label: Option<String>,
    helper: Option<String>,
    error: Option<String>,
) -> Element {
    rsx! {
        if let Some(label) = label {
            span { class: "g3-control-label", "{label}" }
        }
        if let Some(helper) = helper.filter(|text| !text.is_empty()) {
            span { id: format!("{id}-helper"), class: "g3-control-helper", "{helper}" }
        }
        if let Some(error) = error.filter(|text| !text.is_empty()) {
            span { id: format!("{id}-error"), class: "g3-control-error", "{error}" }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn CheckboxPlaygroundDemo() -> Element {
    let checked = use_signal(|| true);
    let disabled = use_signal(|| false);
    let indeterminate = use_signal(|| false);
    let invalid = use_signal(|| false);
    let placement = use_signal(|| ControlLabelPlacement::Start);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Select {
                    label: "Label placement",
                    value: placement,
                    options: vec![
                        crate::SelectOption::new(ControlLabelPlacement::Start, "Start"),
                        crate::SelectOption::new(ControlLabelPlacement::End, "End"),
                        crate::SelectOption::new(ControlLabelPlacement::Fixed, "Fixed"),
                        crate::SelectOption::new(ControlLabelPlacement::Stacked, "Stacked"),
                    ],
                }
                Checkbox { checked: indeterminate, label: "Indeterminate" }
                Checkbox { checked: disabled, label: "Disabled" }
                Checkbox { checked: invalid, label: "Invalid" }
            },
            div { class: "playground-stack",
                Checkbox {
                    checked,
                    label: "Use handicaps",
                    helper: "Adjusts each player's score.",
                    error: invalid().then(|| "Choose a scoring option.".to_string()),
                    indeterminate: indeterminate(),
                    disabled: disabled(),
                    label_placement: placement(),
                }
                Checkbox { aria_label: "Select row" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Checkbox",
    description: "Checkbox rows with helper, error, and label placement.",
    demo: CheckboxPlaygroundDemo,
    source: "src/components/checkbox.rs",
}
