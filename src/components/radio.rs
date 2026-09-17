//! Radio groups.
use super::checkbox::{ControlLabelPlacement, ControlText, control_classes};
use super::field::described_by;
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

struct RadioContext<T: 'static> {
    value: Signal<Option<T>>,
    name: Signal<String>,
    disabled: Signal<bool>,
    allow_empty: Signal<bool>,
    placement: Signal<ControlLabelPlacement>,
    onchange: Option<EventHandler<Option<T>>>,
}

// Written out: a derive would demand `T: Copy`.
impl<T> Clone for RadioContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RadioContext<T> {}

/// A set of [`Radio`] choices where one may be selected. Like Ionic's
/// `ion-radio-group`.
///
/// Radios use native inputs, so arrow keys move between them. The value type
/// is anything comparable; `None` means nothing is selected.
///
/// ```rust,ignore
/// let format = use_signal(|| Some(Format::Stroke));
/// rsx! {
///     RadioGroup { value: format, label: "Format",
///         Radio { value: Format::Stroke, label: "Stroke play" }
///         Radio { value: Format::Match, label: "Match play" }
///     }
/// }
/// ```
#[component]
pub fn RadioGroup<T: Clone + PartialEq + 'static>(
    /// The selected value. Kept internally, starting empty, when not given.
    value: Option<Signal<Option<T>>>,
    /// Visible group label, which also names the group.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text under the group.
    helper: Option<String>,
    /// The inputs' shared `name`. Generated when not given.
    name: Option<String>,
    /// Disable every radio.
    disabled: Option<bool>,
    /// Let pressing the selected radio clear the selection.
    allow_empty: Option<bool>,
    /// Label position for every radio. Defaults to
    /// [`ControlLabelPlacement::Start`].
    label_placement: Option<ControlLabelPlacement>,
    /// Called with the new selection.
    onchange: Option<EventHandler<Option<T>>>,
    /// Extra classes for the group.
    class: Option<String>,
    children: Element,
) -> Element {
    let id = use_element_id("radio-group", None);
    let value = use_controlled(value, || None);
    let name = use_element_id("radio", name);
    let name = crate::state::use_synced_signal(name);
    let disabled = crate::state::use_synced_signal(disabled.unwrap_or(false));
    let allow_empty = crate::state::use_synced_signal(allow_empty.unwrap_or(false));
    let placement = crate::state::use_synced_signal(label_placement.unwrap_or_default());
    use_context_provider(|| RadioContext {
        value,
        name,
        disabled,
        allow_empty,
        placement,
        onchange,
    });
    let label_id = format!("{id}-label");
    let group_name = if label.is_none() { aria_label } else { None };
    let labelledby = label.is_some().then(|| label_id.clone());
    let group_describedby = described_by(&id, &helper, &None);
    rsx! {
        div {
            id: id.clone(),
            class: merge_classes("g3-radio-group", class.as_deref()),
            role: "radiogroup",
            aria_label: group_name,
            aria_labelledby: labelledby,
            aria_describedby: group_describedby,
            aria_disabled: disabled().then_some("true"),
            if let Some(label) = label {
                div { id: label_id, class: "g3-radio-group-label", "{label}" }
            }
            {children}
            if let Some(helper) = helper {
                p { id: format!("{id}-helper"), class: "g3-field-helper", "{helper}" }
            }
        }
    }
}

/// One choice in a [`RadioGroup`].
#[component]
pub fn Radio<T: Clone + PartialEq + 'static>(
    /// The value this radio selects.
    value: T,
    /// Visible label.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text under the label.
    helper: Option<String>,
    /// Disable this radio.
    disabled: Option<bool>,
    /// Label position. Defaults to the group's.
    label_placement: Option<ControlLabelPlacement>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the row.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("radio", None);
    let context = use_context::<RadioContext<T>>();
    let RadioContext {
        value: mut group_value,
        name,
        disabled: group_disabled,
        allow_empty,
        placement,
        onchange,
    } = context;
    let selected = group_value.read().as_ref() == Some(&value);
    let disabled = disabled.unwrap_or(false) || group_disabled();
    let bare = label.is_none() && helper.is_none();
    let cls = control_classes(
        "g3-radio",
        mode,
        label_placement.unwrap_or(placement()),
        bare,
    );
    let mut select = move |next: Option<T>| {
        group_value.set(next.clone());
        if let Some(onchange) = onchange {
            onchange.call(next);
        }
    };
    let pick = value.clone();
    let describedby = described_by(&id, &helper, &None);
    let clear_on_press = selected && allow_empty();
    rsx! {
        label {
            class: merge_classes(cls, class.as_deref()),
            // A native radio cannot be unchecked by clicking it, so clearing
            // an allow-empty selection happens on the press, before the click.
            onclick: move |event| {
                if clear_on_press && !disabled {
                    event.prevent_default();
                    select(None);
                }
            },
            input {
                id: id.clone(),
                class: "g3-radio-input",
                r#type: "radio",
                name: name(),
                checked: selected,
                disabled,
                aria_label,
                aria_describedby: describedby,
                onchange: move |_| {
                    if !disabled {
                        select(Some(pick.clone()));
                    }
                },
            }
            span { class: "g3-control-mark", aria_hidden: "true",
                span { class: "g3-radio-circle",
                    span { class: "g3-radio-dot" }
                }
            }
            ControlText { id: id.clone(), label, helper }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn RadioPlaygroundDemo() -> Element {
    let format = use_signal(|| Some("stroke"));
    let allow_empty = use_signal(|| false);
    let disabled = use_signal(|| false);
    let placement = use_signal(|| ControlLabelPlacement::Start);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::SegmentGroup { value: placement, aria_label: "Label placement",
                    crate::SegmentButton { value: ControlLabelPlacement::Start, "Start" }
                    crate::SegmentButton { value: ControlLabelPlacement::End, "End" }
                    crate::SegmentButton { value: ControlLabelPlacement::Stacked, "Stacked" }
                }
                crate::Checkbox { checked: allow_empty, label: "Allow empty" }
                crate::Checkbox { checked: disabled, label: "Disabled" }
            },
            RadioGroup {
                value: format,
                label: "Format",
                helper: format!("Selected: {}", format().unwrap_or("none")),
                allow_empty: allow_empty(),
                disabled: disabled(),
                label_placement: placement(),
                Radio { value: "stroke", label: "Stroke play" }
                Radio { value: "match", label: "Match play", helper: "Hole by hole." }
                Radio { value: "scramble", label: "Scramble" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Radio",
    description: "Radio groups with optional empty selection.",
    demo: RadioPlaygroundDemo,
    source: "src/components/radio.rs",
}
