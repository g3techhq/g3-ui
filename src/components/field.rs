//! Text inputs and the label/helper/error shell every field shares.
use super::overlay::js_string;
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use std::time::Duration;

/// Label, control, helper, and error, wired together by id.
#[component]
pub(crate) fn FieldShell(
    id: String,
    label: Option<String>,
    required: bool,
    helper: Option<String>,
    error: Option<String>,
    start: Option<Element>,
    end: Option<Element>,
    class: Option<String>,
    children: Element,
) -> Element {
    let helper = helper.filter(|text| !text.is_empty());
    let error = error.filter(|text| !text.is_empty());
    let invalid = error.is_some();
    rsx! {
        div {
            class: merge_classes("g3-field", class.as_deref()),
            "data-invalid": invalid.then_some("true"),
            if let Some(label) = label {
                label {
                    r#for: id.clone(),
                    id: format!("{id}-label"),
                    class: if required { "g3-field-label g3-field-required" } else { "g3-field-label" },
                    "{label}"
                }
            }
            div { class: "g3-field-control",
                if let Some(start) = start {
                    span { class: "g3-field-start", {start} }
                }
                {children}
                if let Some(end) = end {
                    span { class: "g3-field-end", {end} }
                }
            }
            if let Some(helper) = helper {
                p { id: format!("{id}-helper"), class: "g3-field-helper", "{helper}" }
            }
            if let Some(error) = error {
                p {
                    id: format!("{id}-error"),
                    class: "g3-field-error",
                    aria_live: "polite",
                    "{error}"
                }
            }
        }
    }
}

/// The `aria-describedby` value for a field's helper and error text.
pub(crate) fn described_by(
    id: &str,
    helper: &Option<String>,
    error: &Option<String>,
) -> Option<String> {
    let mut ids = Vec::new();
    if helper.as_ref().is_some_and(|text| !text.is_empty()) {
        ids.push(format!("{id}-helper"));
    }
    if error.as_ref().is_some_and(|text| !text.is_empty()) {
        ids.push(format!("{id}-error"));
    }
    (!ids.is_empty()).then(|| ids.join(" "))
}

pub(crate) fn is_invalid(error: &Option<String>) -> bool {
    error.as_ref().is_some_and(|text| !text.is_empty())
}

/// The kind of value an [`Input`] takes. Each maps to an HTML input `type`,
/// which picks the on-screen keyboard and any native picker.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum InputType {
    /// Free text.
    #[default]
    Text,
    /// An email address.
    Email,
    /// A password, masked.
    Password,
    /// A number. `min`, `max`, and `step` apply.
    Number,
    /// A phone number.
    Tel,
    /// A URL.
    Url,
    /// A search query.
    Search,
    /// A date (`YYYY-MM-DD`), with the platform date picker.
    Date,
    /// A time (`HH:MM`), with the platform time picker.
    Time,
    /// A local date and time (`YYYY-MM-DDTHH:MM`).
    DateTime,
    /// A month (`YYYY-MM`).
    Month,
}

impl InputType {
    fn as_str(self) -> &'static str {
        match self {
            InputType::Text => "text",
            InputType::Email => "email",
            InputType::Password => "password",
            InputType::Number => "number",
            InputType::Tel => "tel",
            InputType::Url => "url",
            InputType::Search => "search",
            InputType::Date => "date",
            InputType::Time => "time",
            InputType::DateTime => "datetime-local",
            InputType::Month => "month",
        }
    }
}

/// Whether a typed value may stand while the user is still typing. Only
/// limits that no longer-typed value could satisfy apply: exceeding `max` or
/// `maxlength`. A value below `min` or `minlength` may still grow into a valid
/// one, so those are left to validation.
pub(crate) fn within_typing_limits(
    value: &str,
    max: Option<f64>,
    maxlength: Option<usize>,
) -> bool {
    if let Some(maxlength) = maxlength
        && value.chars().count() > maxlength
    {
        return false;
    }
    if let Some(max) = max
        && let Ok(number) = value.parse::<f64>()
        && number > max
    {
        return false;
    }
    true
}

/// Put the element's displayed value back to `value`. Needed after refusing
/// a keystroke: the signal did not change, so no render would do it.
fn restore_value(id: &str, value: &str) {
    let _ = document::eval(&format!(
        "const el = document.getElementById({}); if (el) el.value = {};",
        js_string(id),
        js_string(value),
    ));
}

/// Shared typing behaviour of [`Input`] and [`TextArea`].
fn use_text_events(
    id: String,
    mut value: Signal<String>,
    max: Option<f64>,
    maxlength: Option<usize>,
    debounce_ms: Option<u64>,
    oninput: Option<EventHandler<String>>,
    onchange: Option<EventHandler<String>>,
) -> (Callback<FormEvent>, Callback<FormEvent>) {
    let mut generation = use_signal(|| 0_u64);
    let input_id = id.clone();
    let handle_input = use_callback(move |event: FormEvent| {
        let next = event.value();
        if !within_typing_limits(&next, max, maxlength) {
            restore_value(&input_id, &value.peek());
            return;
        }
        value.set(next.clone());
        if let Some(oninput) = oninput {
            oninput.call(next.clone());
        }
        if let Some(delay) = debounce_ms {
            let current = generation.with_mut(|g| {
                *g += 1;
                *g
            });
            spawn(async move {
                dioxus_sdk_time::sleep(Duration::from_millis(delay)).await;
                if *generation.peek() == current
                    && let Some(onchange) = onchange
                {
                    onchange.call(next);
                }
            });
        }
    });
    let handle_change = use_callback(move |event: FormEvent| {
        if debounce_ms.is_none()
            && let Some(onchange) = onchange
        {
            onchange.call(event.value());
        }
    });
    (handle_input, handle_change)
}

/// A single-line text field with a label. Like Ionic's `ion-input`.
///
/// Pass a `value` signal to own the text, or leave it out and read changes
/// from `onchange`. `oninput` fires on every keystroke; `onchange` fires when
/// the user commits (blur or Enter), or after `debounce_ms` of no typing when
/// that is set, which suits search-as-you-type.
///
/// ```rust,ignore
/// let email = use_signal(String::new);
/// rsx! {
///     Input {
///         label: "Email",
///         value: email,
///         input_type: InputType::Email,
///         autocomplete: "email",
///         required: true,
///         error: email_error(),
///     }
/// }
/// ```
#[component]
pub fn Input(
    /// Visible label, which also names the field.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// The text. Kept internally when not given.
    value: Option<Signal<String>>,
    /// The kind of value. Defaults to [`InputType::Text`].
    input_type: Option<InputType>,
    /// Hint shown while empty.
    placeholder: Option<String>,
    /// Help text below the field.
    helper: Option<String>,
    /// Error text below the field. Marks the field invalid when not empty.
    error: Option<String>,
    /// Mark the field required.
    required: Option<bool>,
    /// Disable the field.
    disabled: Option<bool>,
    /// Make the field read-only.
    readonly: Option<bool>,
    /// Smallest number, for numeric and date types.
    min: Option<f64>,
    /// Largest number. Typing past it is refused.
    max: Option<f64>,
    /// Number granularity.
    step: Option<f64>,
    /// Shortest valid text, in characters.
    minlength: Option<usize>,
    /// Longest text, in characters. Typing past it is refused.
    maxlength: Option<usize>,
    /// Browser autofill hint, such as `"email"` or `"current-password"`.
    autocomplete: Option<String>,
    /// Show a button that clears the text.
    clearable: Option<bool>,
    /// Content inside the field before the text, usually an icon.
    start: Option<Element>,
    /// Content inside the field after the text, such as a unit or a button.
    end: Option<Element>,
    /// Delay `onchange` until typing pauses for this many milliseconds.
    debounce_ms: Option<u64>,
    /// Called with the new text on every keystroke.
    oninput: Option<EventHandler<String>>,
    /// Called with the text when the user commits it, or after the debounce.
    onchange: Option<EventHandler<String>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
    /// Any other attribute for the `<input>`, such as `name` or `inputmode`.
    #[props(extends = input)]
    attributes: Vec<Attribute>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("input", id);
    let mut value = use_controlled(value, String::new);
    let (handle_input, handle_change) = use_text_events(
        id.clone(),
        value,
        max,
        maxlength,
        debounce_ms,
        oninput,
        onchange,
    );
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    let disabled = disabled.unwrap_or(false);
    let clear_button =
        (clearable.unwrap_or(false) && !disabled && !value.read().is_empty()).then(|| {
            rsx! {
                button {
                    r#type: "button",
                    class: "g3-field-clear",
                    aria_label: strings.clear.clone(),
                    onclick: move |_| {
                        value.set(String::new());
                        if let Some(oninput) = oninput {
                            oninput.call(String::new());
                        }
                        if let Some(onchange) = onchange {
                            onchange.call(String::new());
                        }
                    },
                    X { size: 16 }
                }
            }
        });
    let end = clear_button.or(end);
    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: required.unwrap_or(false),
            helper,
            error,
            start,
            end,
            class,
            input {
                id,
                class: classes(["g3-input", mode.pick("g3-input-ios", "g3-input-md")]),
                r#type: input_type.unwrap_or_default().as_str(),
                value: value(),
                placeholder,
                disabled,
                readonly,
                required,
                min,
                max,
                step,
                minlength,
                maxlength,
                autocomplete,
                aria_label,
                aria_describedby: describedby,
                aria_invalid: invalid.then_some("true"),
                oninput: handle_input,
                onchange: handle_change,
                ..attributes,
            }
        }
    }
}

/// A multi-line text field with a label.
///
/// Behaves like [`Input`]: the same value, label, helper, error, and event
/// props.
#[component]
pub fn TextArea(
    /// Visible label, which also names the field.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// The text. Kept internally when not given.
    value: Option<Signal<String>>,
    /// Hint shown while empty.
    placeholder: Option<String>,
    /// Help text below the field.
    helper: Option<String>,
    /// Error text below the field. Marks the field invalid when not empty.
    error: Option<String>,
    /// Visible rows. Defaults to 3.
    rows: Option<u32>,
    /// Mark the field required.
    required: Option<bool>,
    /// Disable the field.
    disabled: Option<bool>,
    /// Make the field read-only.
    readonly: Option<bool>,
    /// Shortest valid text, in characters.
    minlength: Option<usize>,
    /// Longest text, in characters. Typing past it is refused.
    maxlength: Option<usize>,
    /// Delay `onchange` until typing pauses for this many milliseconds.
    debounce_ms: Option<u64>,
    /// Called with the new text on every keystroke.
    oninput: Option<EventHandler<String>>,
    /// Called with the text when the user commits it, or after the debounce.
    onchange: Option<EventHandler<String>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
    /// Any other attribute for the `<textarea>`.
    #[props(extends = textarea)]
    attributes: Vec<Attribute>,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("textarea", id);
    let value = use_controlled(value, String::new);
    let (handle_input, handle_change) = use_text_events(
        id.clone(),
        value,
        None,
        maxlength,
        debounce_ms,
        oninput,
        onchange,
    );
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: required.unwrap_or(false),
            helper,
            error,
            class,
            textarea {
                id,
                class: classes(["g3-input", mode.pick("g3-input-ios", "g3-input-md")]),
                rows: rows.unwrap_or(3),
                value: value(),
                placeholder,
                disabled,
                readonly,
                required,
                minlength,
                maxlength,
                aria_label,
                aria_describedby: describedby,
                aria_invalid: invalid.then_some("true"),
                oninput: handle_input,
                onchange: handle_change,
                ..attributes,
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn InputPlaygroundDemo() -> Element {
    let value = use_signal(String::new);
    let label = use_signal(|| "Club".to_string());
    let disabled = use_signal(|| false);
    let invalid = use_signal(|| false);
    let clearable = use_signal(|| true);
    let input_type = use_signal(|| InputType::Text);
    let notes = use_signal(String::new);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                Input { label: "Label", value: label }
                crate::Select {
                    label: "Type",
                    value: input_type,
                    options: vec![
                        crate::SelectOption::new(InputType::Text, "Text"),
                        crate::SelectOption::new(InputType::Number, "Number"),
                        crate::SelectOption::new(InputType::Password, "Password"),
                        crate::SelectOption::new(InputType::Date, "Date"),
                        crate::SelectOption::new(InputType::Time, "Time"),
                    ],
                }
                crate::Checkbox { checked: disabled, label: "Disabled" }
                crate::Checkbox { checked: invalid, label: "Invalid" }
                crate::Checkbox { checked: clearable, label: "Clearable" }
            },
            div { class: "playground-stack",
                Input {
                    label: label(),
                    value,
                    input_type: input_type(),
                    placeholder: "Club name",
                    helper: "Shown on the scorecard.",
                    error: invalid().then(|| "Enter a club name.".to_string()),
                    disabled: disabled(),
                    clearable: clearable(),
                }
                TextArea { label: "Notes", value: notes, rows: 3 }
            }
        }
    }
}

crate::g3_playground! {
    name: "Input",
    description: "Text, number, date, and multi-line fields with helper and error text.",
    components: ["Input", "TextArea"],
    demo: InputPlaygroundDemo,
    source: "src/components/field.rs",
}
