//! Numeric controls: a slider and a stepper.
use super::field::{FieldShell, described_by};
use crate::state::{use_controlled, use_element_id};
use crate::theme::merge_classes;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Minus, Plus};

/// A slider for a number in a range. Like Ionic's `ion-range`.
///
/// ```rust,ignore
/// rsx! { Range { label: "Volume", value: volume, min: 0.0, max: 100.0, show_value: true } }
/// ```
#[component]
pub fn Range(
    /// The value. Kept internally, starting at `min`, when not given.
    value: Option<Signal<f64>>,
    /// Smallest value. Defaults to `0`.
    min: Option<f64>,
    /// Largest value. Defaults to `100`.
    max: Option<f64>,
    /// Granularity. Defaults to `1`.
    step: Option<f64>,
    /// Visible label, which also names the slider.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text below.
    helper: Option<String>,
    /// Show the current value beside the slider.
    show_value: Option<bool>,
    /// Content before the slider, such as a "less" icon.
    start: Option<Element>,
    /// Content after the slider, such as a "more" icon.
    end: Option<Element>,
    /// Disable the slider.
    disabled: Option<bool>,
    /// Called with every new value while dragging.
    oninput: Option<EventHandler<f64>>,
    /// Called with the value when the user lets go.
    onchange: Option<EventHandler<f64>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let min = min.unwrap_or(0.0);
    let max = max.unwrap_or(100.0).max(min);
    let id = use_element_id("range", id);
    let mut value = use_controlled(value, || min);
    let current = value().clamp(min, max);
    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: false,
            helper: helper.clone(),
            class,
            div { class: "g3-range",
                {start}
                input {
                    id: id.clone(),
                    class: "g3-range-input",
                    r#type: "range",
                    min,
                    max,
                    step: step.unwrap_or(1.0),
                    value: current,
                    disabled,
                    aria_label,
                    aria_describedby: described_by(&id, &helper, &None),
                    oninput: move |event| {
                        if let Ok(next) = event.value().parse::<f64>() {
                            value.set(next);
                            if let Some(oninput) = oninput {
                                oninput.call(next);
                            }
                        }
                    },
                    onchange: move |event| {
                        if let (Ok(next), Some(onchange)) = (event.value().parse::<f64>(), onchange) {
                            onchange.call(next);
                        }
                    },
                }
                {end}
                if show_value.unwrap_or(false) {
                    output { class: "g3-range-value", "{current}" }
                }
            }
        }
    }
}

/// A number with decrease and increase buttons, for small counts such as
/// players or holes.
///
/// ```rust,ignore
/// rsx! { Stepper { label: "Players", value: players, min: 1, max: 8 } }
/// ```
#[component]
pub fn Stepper(
    /// The value. Kept internally, starting at `min` or `0`, when not given.
    value: Option<Signal<i64>>,
    /// Smallest value.
    min: Option<i64>,
    /// Largest value.
    max: Option<i64>,
    /// Amount each press changes the value. Defaults to `1`.
    step: Option<i64>,
    /// Visible label, which also names the stepper.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text below.
    helper: Option<String>,
    /// Disable the stepper.
    disabled: Option<bool>,
    /// Called with the new value.
    onchange: Option<EventHandler<i64>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let id = use_element_id("stepper", id);
    let mut value = use_controlled(value, || min.unwrap_or(0));
    let step = step.unwrap_or(1).max(1);
    let disabled = disabled.unwrap_or(false);
    let current = value();
    let can_decrease = !disabled && min.is_none_or(|min| current > min);
    let can_increase = !disabled && max.is_none_or(|max| current < max);
    let mut change = move |delta: i64| {
        let mut next = value().saturating_add(delta);
        if let Some(min) = min {
            next = next.max(min);
        }
        if let Some(max) = max {
            next = next.min(max);
        }
        value.set(next);
        if let Some(onchange) = onchange {
            onchange.call(next);
        }
    };
    let name = label.clone().or(aria_label);
    rsx! {
        FieldShell {
            id: id.clone(),
            label: label.clone(),
            required: false,
            helper: helper.clone(),
            class: merge_classes("g3-stepper-field", class.as_deref()),
            div {
                id: id.clone(),
                class: "g3-stepper",
                role: "spinbutton",
                tabindex: if disabled { "-1" } else { "0" },
                aria_label: name.clone(),
                aria_valuenow: current.to_string(),
                aria_valuemin: min.map(|min| min.to_string()),
                aria_valuemax: max.map(|max| max.to_string()),
                aria_disabled: disabled.then_some("true"),
                aria_describedby: described_by(&id, &helper, &None),
                onkeydown: move |event| match event.key() {
                    Key::ArrowUp if can_increase => {
                        event.prevent_default();
                        change(step);
                    }
                    Key::ArrowDown if can_decrease => {
                        event.prevent_default();
                        change(-step);
                    }
                    _ => {}
                },
                button {
                    r#type: "button",
                    class: "g3-stepper-button",
                    tabindex: "-1",
                    aria_hidden: "true",
                    disabled: !can_decrease,
                    onclick: move |_| change(-step),
                    Minus { size: 18 }
                }
                span { class: "g3-stepper-value", "{current}" }
                button {
                    r#type: "button",
                    class: "g3-stepper-button",
                    tabindex: "-1",
                    aria_hidden: "true",
                    disabled: !can_increase,
                    onclick: move |_| change(step),
                    Plus { size: 18 }
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn RangePlaygroundDemo() -> Element {
    let volume = use_signal(|| 40.0);
    let players = use_signal(|| 4_i64);
    rsx! {
        crate::PlaygroundDemoFrame {
            div { class: "playground-stack",
                Range {
                    label: "Volume",
                    value: volume,
                    show_value: true,
                    step: 5.0,
                }
                Stepper {
                    label: "Players",
                    value: players,
                    min: 1,
                    max: 8,
                    helper: "Up to eight per group.",
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Range",
    description: "Slider and stepper for numeric values.",
    demo: RangePlaygroundDemo,
    source: "src/components/range.rs",
}
