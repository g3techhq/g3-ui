//! A single-choice picker.
use super::field::{FieldShell, described_by, is_invalid};
use super::popover::{PopoverFrame, PopoverPlacement};
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};

/// How wide a [`Select`] is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SelectWidth {
    /// As wide as its container, like other form fields.
    #[default]
    Fill,
    /// As wide as the chosen option, for toolbars and inline filters.
    Fit,
    /// A fixed 8rem.
    Sm,
    /// A fixed 12rem.
    Md,
    /// A fixed 18rem.
    Lg,
}

/// One choice in a [`Select`].
///
/// ```
/// use g3_ui::SelectOption;
///
/// let option = SelectOption::new("gb", "United Kingdom").description("GBP");
/// # let _ = option;
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption<T> {
    /// The value chosen.
    pub value: T,
    /// Text shown for it.
    pub label: String,
    /// Secondary text under the label.
    pub description: Option<String>,
    /// Whether it can be chosen.
    pub disabled: bool,
}

impl<T> SelectOption<T> {
    /// An option with a value and its display text.
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
            description: None,
            disabled: false,
        }
    }

    /// Add secondary text.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Make the option unavailable.
    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

impl From<&str> for SelectOption<String> {
    fn from(value: &str) -> Self {
        Self::new(value.to_string(), value)
    }
}

impl From<String> for SelectOption<String> {
    fn from(value: String) -> Self {
        Self::new(value.clone(), value)
    }
}

impl<T, L: Into<String>> From<(T, L)> for SelectOption<T> {
    fn from((value, label): (T, L)) -> Self {
        Self::new(value, label)
    }
}

/// A field that picks one value from a list. Like Ionic's `ion-select`.
///
/// The list opens as a bottom sheet on compact shells and as a dropdown on
/// wide ones. The trigger shows the chosen option's label, or `placeholder`
/// when the value matches no option.
///
/// The value type is anything comparable, so enums work directly:
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Clone, Copy, PartialEq)] enum Tees { White, Yellow }
/// let tees = use_signal(|| Tees::White);
/// rsx! {
///     Select {
///         label: "Tees",
///         value: tees,
///         options: vec![
///             SelectOption::new(Tees::White, "White"),
///             SelectOption::new(Tees::Yellow, "Yellow"),
///         ],
///     }
/// }
/// # }
/// ```
#[component]
pub fn Select<T: Clone + PartialEq + 'static>(
    /// The chosen value. When not given the select keeps its own, starting at
    /// `default_value` or the first option.
    value: Option<Signal<T>>,
    /// Starting value when `value` is not given.
    default_value: Option<T>,
    /// The choices.
    options: Vec<SelectOption<T>>,
    /// Visible label, which also names the field and the list.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Trigger text when the value matches no option. Defaults to
    /// [`Strings::select_placeholder`](crate::Strings::select_placeholder).
    placeholder: Option<String>,
    /// Help text below the field.
    helper: Option<String>,
    /// Error text below the field.
    error: Option<String>,
    /// Mark the field required.
    required: Option<bool>,
    /// Disable the field.
    disabled: Option<bool>,
    /// Called with the value the user picks.
    onchange: Option<EventHandler<T>>,
    /// Element id of the trigger. Generated when not given.
    id: Option<String>,
    /// How wide the field is. Defaults to [`SelectWidth::Fill`].
    width: Option<SelectWidth>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let width_cls = match width.unwrap_or_default() {
        SelectWidth::Fill => "",
        SelectWidth::Fit => "g3-select-width-fit",
        SelectWidth::Sm => "g3-select-width-sm",
        SelectWidth::Md => "g3-select-width-md",
        SelectWidth::Lg => "g3-select-width-lg",
    };
    let class = Some(crate::theme::merge_classes(width_cls, class.as_deref()));
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("select", id);
    let list_id = format!("{id}-list");
    let initial = default_value.or_else(|| options.first().map(|option| option.value.clone()));
    let local = use_signal(|| initial);
    let mut open = use_signal(|| false);
    let current: Option<T> = match value {
        Some(value) => Some(value()),
        None => local(),
    };
    let chosen = options
        .iter()
        .find(|option| Some(&option.value) == current.as_ref());
    let placeholder = placeholder.unwrap_or(strings.select_placeholder);
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    let labelledby = label.as_ref().map(|_| format!("{id}-label {id}-value"));
    let list_label = label.clone().or(aria_label.clone());
    let trigger = rsx! {
        button {
            id: id.clone(),
            r#type: "button",
            class: classes(["g3-input", "g3-select", mode.pick("g3-input-ios", "g3-input-md")]),
            disabled,
            aria_haspopup: "listbox",
            aria_expanded: open().to_string(),
            aria_controls: list_id.clone(),
            aria_label,
            aria_labelledby: labelledby,
            aria_describedby: describedby,
            aria_invalid: invalid.then_some("true"),
            onclick: move |_| open.toggle(),
            // Arrow keys open the list, as a native select does.
            onkeydown: move |event| {
                if matches!(event.key(), Key::ArrowDown | Key::ArrowUp) && !open() {
                    event.prevent_default();
                    open.set(true);
                }
            },
            match chosen {
                Some(option) => rsx! {
                    span { id: format!("{id}-value"), class: "g3-select-value", "{option.label}" }
                },
                None => rsx! {
                    span { id: format!("{id}-value"), class: "g3-select-value g3-select-placeholder", "{placeholder}" }
                },
            }
            ChevronDown { class: "g3-select-icon", size: 16 }
        }
    };
    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: required.unwrap_or(false),
            helper,
            error,
            class,
            div { class: "g3-select-anchor",
                PopoverFrame {
                    open,
                    trigger,
                    placement: PopoverPlacement::BottomStart,
                    sheet_on_compact: true,
                    role: "listbox",
                    roving: Some("[role=option]"),
                    id: list_id,
                    aria_label: list_label,
                    mode,
                    class: "g3-select-list",
                    for option in options {
                        SelectOptionRow {
                            option: option.clone(),
                            selected: Some(&option.value) == current.as_ref(),
                            onpick: move |picked: T| {
                                match value {
                                    Some(mut value) => value.set(picked.clone()),
                                    None => {
                                        let mut local = local;
                                        local.set(Some(picked.clone()));
                                    }
                                }
                                open.set(false);
                                if let Some(onchange) = onchange {
                                    onchange.call(picked);
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SelectOptionRow<T: Clone + PartialEq + 'static>(
    option: SelectOption<T>,
    selected: bool,
    onpick: EventHandler<T>,
) -> Element {
    let SelectOption {
        value,
        label,
        description,
        disabled,
    } = option;
    rsx! {
        button {
            r#type: "button",
            class: "g3-select-option",
            role: "option",
            tabindex: "-1",
            aria_selected: selected.to_string(),
            disabled,
            onclick: move |_| onpick.call(value.clone()),
            span { "{label}" }
            if selected {
                span { class: "g3-select-check", aria_hidden: "true",
                    Check { size: 18 }
                }
            }
            if let Some(description) = description {
                span { class: "g3-select-option-description", "{description}" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SelectPlaygroundDemo() -> Element {
    let club = use_signal(|| "Driver".to_string());
    let disabled = use_signal(|| false);
    let width = use_signal(|| SelectWidth::Fill);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: disabled, label: "Disabled" }
                Select {
                    label: "Width",
                    value: width,
                    options: vec![
                        SelectOption::new(SelectWidth::Fill, "Fill container"),
                        SelectOption::new(SelectWidth::Fit, "Fit content"),
                        SelectOption::new(SelectWidth::Sm, "Small (8rem)"),
                        SelectOption::new(SelectWidth::Md, "Medium (12rem)"),
                        SelectOption::new(SelectWidth::Lg, "Large (18rem)"),
                    ],
                }
            },
            Select {
                label: "Club",
                value: club,
                width: width(),
                disabled: disabled(),
                helper: format!("Selected: {club}"),
                options: vec![
                    SelectOption::from("Driver"),
                    SelectOption::from("Iron").description("3 through 9"),
                    SelectOption::from("Wedge"),
                    SelectOption::from("Putter").disabled(),
                ],
            }
            Select::<u8> {
                label: "Tee time",
                width: width(),
                placeholder: "Choose a time",
                options: vec![SelectOption::new(8, "8:00"), SelectOption::new(9, "9:00")],
                default_value: 0,
            }
        }
    }
}

crate::g3_playground! {
    name: "Select",
    description: "Single-choice picker: a sheet on phones, a dropdown on wide shells.",
    demo: SelectPlaygroundDemo,
    source: "src/components/select.rs",
}
