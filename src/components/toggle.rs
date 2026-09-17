//! On/off switches.
use super::checkbox::{ControlLabelPlacement, ControlText, control_classes};
use super::field::described_by;
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Size of a [`Toggle`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ToggleSize {
    /// Compact, for dense rows.
    Sm,
    /// Standard platform size.
    #[default]
    Md,
}

/// An on/off switch row. Like Ionic's `ion-toggle`.
///
/// The label comes first by default, as in a settings list:
///
/// ```rust,ignore
/// rsx! {
///     Toggle { checked: notifications, label: "Notifications" }
/// }
/// ```
#[component]
pub fn Toggle(
    /// Whether it is on. Kept internally when not given.
    checked: Option<Signal<bool>>,
    /// Visible label, which also names the switch.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text under the label.
    helper: Option<String>,
    /// Disable the switch.
    disabled: Option<bool>,
    /// Size. Defaults to [`ToggleSize::Md`].
    size: Option<ToggleSize>,
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
    let id = use_element_id("toggle", id);
    let mut checked = use_controlled(checked, || false);
    let bare = label.is_none() && helper.is_none();
    let cls = control_classes("g3-toggle", mode, label_placement.unwrap_or_default(), bare);
    let switch_cls = classes([
        "g3-switch",
        mode.pick("g3-switch-ios", "g3-switch-md"),
        match size.unwrap_or_default() {
            ToggleSize::Sm => "g3-switch-sm",
            ToggleSize::Md => "",
        },
    ]);
    let describedby = described_by(&id, &helper, &None);
    rsx! {
        button {
            id: id.clone(),
            class: merge_classes(cls, class.as_deref()),
            r#type: "button",
            role: "switch",
            aria_checked: checked().to_string(),
            aria_label,
            aria_describedby: describedby,
            disabled,
            onclick: move |_| {
                let next = !checked();
                checked.set(next);
                if let Some(onchange) = onchange {
                    onchange.call(next);
                }
            },
            span { class: "g3-control-mark", aria_hidden: "true",
                span { class: switch_cls,
                    span { class: "g3-switch-thumb" }
                }
            }
            ControlText { id: id.clone(), label, helper }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TogglePlaygroundDemo() -> Element {
    let checked = use_signal(|| true);
    let small = use_signal(|| false);
    let disabled = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: small, label: "Small" }
                crate::Checkbox { checked: disabled, label: "Disabled" }
            },
            div { class: "playground-stack",
                Toggle {
                    checked,
                    label: "Notifications",
                    helper: "Round reminders and results.",
                    label_placement: ControlLabelPlacement::Start,
                    size: if small() { ToggleSize::Sm } else { ToggleSize::Md },
                    disabled: disabled(),
                }
                Toggle { checked, aria_label: "Notifications (compact)" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Toggle",
    description: "Platform on/off switch with an optional label.",
    demo: TogglePlaygroundDemo,
    source: "src/components/toggle.rs",
}
