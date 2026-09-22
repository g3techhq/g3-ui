//! A form field that picks a date.
use super::calendar::{Calendar, name_at, short_date};
use super::datetime::{CalendarDate, days_in_month};
use super::field::{FieldShell, described_by, is_invalid};
use super::popover::{PopoverFrame, PopoverPlacement};
use super::wheel::WheelColumn;
use super::{BottomSheet, Button, ButtonFill, Modal};
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Calendar as CalendarIcon, Clock};

/// How a [`DatePicker`] or [`TimePicker`](crate::TimePicker) opens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum PickerStyle {
    /// Wheels on iOS, a dialog on Material Design.
    #[default]
    Auto,
    /// Spinning columns in a bottom sheet with a Done button, as iOS's
    /// wheel picker.
    Wheels,
    /// A modal with a calendar or clock face and Cancel and OK buttons, as
    /// Material's date and time pickers.
    Dialog,
    /// A calendar (or wheels, for a time) anchored to the field, as iOS's
    /// compact picker. A bottom sheet on phones.
    Popover,
}

impl PickerStyle {
    pub(crate) fn resolve(self, mode: ComponentMode) -> Self {
        match self {
            PickerStyle::Auto => mode.pick(PickerStyle::Wheels, PickerStyle::Dialog),
            style => style,
        }
    }
}

/// The button that shows a picker's value and opens it, styled as a field.
#[allow(clippy::too_many_arguments)]
pub(crate) fn picker_trigger(
    id: &str,
    mut open: Signal<bool>,
    text: Option<String>,
    placeholder: &str,
    has_label: bool,
    aria_label: Option<String>,
    describedby: Option<String>,
    invalid: bool,
    disabled: bool,
    mode: ComponentMode,
    icon: Element,
) -> Element {
    let value_id = format!("{id}-value");
    let labelledby = has_label.then(|| format!("{id}-label {value_id}"));
    rsx! {
        button {
            id: id.to_string(),
            r#type: "button",
            class: classes(["g3-input", "g3-select", "g3-picker-trigger", mode.pick("g3-input-ios", "g3-input-md")]),
            disabled,
            aria_haspopup: "dialog",
            aria_expanded: open().to_string(),
            aria_label,
            aria_labelledby: labelledby,
            aria_describedby: describedby,
            aria_invalid: invalid.then_some("true"),
            onclick: move |_| open.toggle(),
            onkeydown: move |event| {
                if matches!(event.key(), Key::ArrowDown | Key::ArrowUp) && !open() {
                    event.prevent_default();
                    open.set(true);
                }
            },
            match text {
                Some(text) => rsx! {
                    span { id: value_id, class: "g3-select-value", "{text}" }
                },
                None => rsx! {
                    span { id: value_id, class: "g3-select-value g3-select-placeholder", "{placeholder}" }
                },
            }
            span { class: "g3-select-icon g3-picker-icon", aria_hidden: "true", {icon} }
        }
    }
}

pub(crate) fn calendar_icon() -> Element {
    rsx! { CalendarIcon { size: 18 } }
}

pub(crate) fn clock_icon() -> Element {
    rsx! { Clock { size: 18 } }
}

/// Month, day, and year columns for a wheel date picker.
#[component]
fn DateWheels(
    draft: Signal<CalendarDate>,
    min: Option<CalendarDate>,
    max: Option<CalendarDate>,
) -> Element {
    let strings = use_strings();
    let today = use_hook(CalendarDate::today);
    let date = draft();
    let first_year = min.map_or(today.year() - 100, CalendarDate::year);
    let last_year = max
        .map_or(today.year() + 50, CalendarDate::year)
        .max(first_year);
    let set = move |year: i32, month: u8, day: u8| {
        let mut draft = draft;
        let day = day.min(days_in_month(year, month));
        if let Some(next) = CalendarDate::new(year, month, day) {
            draft.set(next.clamp_to(min, max));
        }
    };
    let months = (0..12)
        .map(|index| name_at(&strings.months, index))
        .collect::<Vec<_>>();
    let days = (1..=days_in_month(date.year(), date.month()))
        .map(|day| day.to_string())
        .collect::<Vec<_>>();
    let years = (first_year..=last_year)
        .map(|year| year.to_string())
        .collect::<Vec<_>>();
    rsx! {
        div { class: "g3-wheel",
            WheelColumn {
                labels: months,
                index: usize::from(date.month() - 1),
                wide: true,
                aria_label: strings.month.clone(),
                onchange: move |index: usize| set(date.year(), index as u8 + 1, date.day()),
            }
            WheelColumn {
                labels: days,
                index: usize::from(date.day() - 1),
                aria_label: strings.day.clone(),
                onchange: move |index: usize| set(date.year(), date.month(), index as u8 + 1),
            }
            WheelColumn {
                labels: years,
                index: (date.year() - first_year).max(0) as usize,
                aria_label: strings.year.clone(),
                onchange: move |index: usize| set(first_year + index as i32, date.month(), date.day()),
            }
        }
    }
}

/// A form field that picks a date, with iOS wheels or a Material calendar
/// dialog. Like Ionic's `ion-datetime` in a modal.
///
/// The value is a [`CalendarDate`], so there is no string format to agree on.
/// The field shows it as "Sep 19, 2026"; pass `format` to show it
/// differently.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let tee_day = use_signal(|| None::<CalendarDate>);
/// rsx! {
///     DatePicker { label: "Tee day", value: tee_day, min: CalendarDate::today() }
/// }
/// # }
/// ```
#[component]
pub fn DatePicker(
    /// The chosen date. Kept internally when not given.
    value: Option<Signal<Option<CalendarDate>>>,
    /// Visible label, which also names the field.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Text while no date is chosen. Defaults to
    /// [`Strings::choose_date`](crate::Strings::choose_date).
    placeholder: Option<String>,
    /// Help text below the field.
    helper: Option<String>,
    /// Error text below the field.
    error: Option<String>,
    /// Mark the field required.
    required: Option<bool>,
    /// Disable the field.
    disabled: Option<bool>,
    /// The earliest date that can be picked.
    min: Option<CalendarDate>,
    /// The latest date that can be picked.
    max: Option<CalendarDate>,
    /// Returns `true` for dates that cannot be picked. Applies to the
    /// calendar styles.
    is_date_disabled: Option<Callback<CalendarDate, bool>>,
    /// How the picker opens. Defaults to [`PickerStyle::Auto`].
    style: Option<PickerStyle>,
    /// How the field shows the chosen date.
    format: Option<Callback<CalendarDate, String>>,
    /// Called with the date the user confirms.
    onchange: Option<EventHandler<CalendarDate>>,
    /// Element id of the field. Generated when not given.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("date", id);
    let mut value = use_controlled(value, || None);
    let mut open = use_signal(|| false);
    let today = use_hook(CalendarDate::today);
    // The date being chosen, committed on Done or OK.
    let mut draft = use_signal(|| today.clamp_to(min, max));
    let mut draft_choice = use_signal(|| None::<CalendarDate>);
    use_effect(move || {
        if open() {
            let start = value.peek().unwrap_or(today).clamp_to(min, max);
            draft.set(start);
            draft_choice.set(Some(start));
        }
    });
    let mut commit = move |date: CalendarDate| {
        value.set(Some(date));
        open.set(false);
        if let Some(onchange) = onchange {
            onchange.call(date);
        }
    };

    let style = style.unwrap_or_default().resolve(mode);
    let disabled = disabled.unwrap_or(false);
    let placeholder = placeholder.unwrap_or_else(|| strings.choose_date.clone());
    let text = value().map(|date| match format {
        Some(format) => format.call(date),
        None => short_date(&strings, date),
    });
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    let title = label.clone().unwrap_or_else(|| strings.choose_date.clone());
    let trigger = picker_trigger(
        &id,
        open,
        text,
        &placeholder,
        label.is_some(),
        aria_label.clone(),
        describedby,
        invalid,
        disabled,
        mode,
        calendar_icon(),
    );
    let headline = draft_choice().map(|date| short_date(&strings, date));

    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: required.unwrap_or(false),
            helper,
            error,
            class: crate::theme::merge_classes("g3-picker-field", class.as_deref()),
            match style {
                PickerStyle::Popover => rsx! {
                    div { class: "g3-select-anchor",
                        PopoverFrame {
                            open,
                            trigger,
                            placement: PopoverPlacement::BottomStart,
                            sheet_on_compact: true,
                            role: "dialog",
                            aria_label: title.clone(),
                            mode,
                            class: "g3-picker-popover",
                            Calendar {
                                value: draft_choice,
                                min,
                                max,
                                is_date_disabled,
                                mode,
                                onchange: commit,
                            }
                        }
                    }
                },
                PickerStyle::Dialog => rsx! {
                    {trigger}
                    Modal {
                        open,
                        title: strings.choose_date.clone(),
                        mode,
                        class: "g3-picker-dialog",
                        actions: rsx! {
                            Button { fill: ButtonFill::Clear, mode, onclick: move |_| open.set(false), "{strings.cancel}" }
                            Button {
                                fill: ButtonFill::Clear,
                                mode,
                                disabled: draft_choice().is_none(),
                                onclick: move |_| {
                                    if let Some(date) = draft_choice() {
                                        commit(date);
                                    }
                                },
                                "{strings.confirm}"
                            }
                        },
                        if let Some(headline) = headline {
                            p { class: "g3-picker-headline", aria_live: "polite", "{headline}" }
                        }
                        if open() {
                            Calendar { value: draft_choice, min, max, is_date_disabled, mode }
                        }
                    }
                },
                _ => rsx! {
                    {trigger}
                    BottomSheet {
                        open,
                        title,
                        mode,
                        class: "g3-picker-sheet",
                        div { class: "g3-picker-toolbar",
                            Button {
                                fill: ButtonFill::Clear,
                                mode,
                                onclick: move |_| commit(draft()),
                                "{strings.done}"
                            }
                        }
                        DateWheels { draft, min, max }
                    }
                },
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn DatePickerPlaygroundDemo() -> Element {
    let picked = use_signal(|| None::<CalendarDate>);
    let style = use_signal(|| PickerStyle::Auto);
    let limit = use_signal(|| false);
    let inline = use_signal(|| false);
    let today = use_hook(CalendarDate::today);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                crate::Select {
                    label: "Style",
                    value: style,
                    options: vec![
                        crate::SelectOption::new(PickerStyle::Auto, "Auto (wheels on iOS)"),
                        crate::SelectOption::new(PickerStyle::Wheels, "Wheels in a sheet"),
                        crate::SelectOption::new(PickerStyle::Dialog, "Calendar dialog"),
                        crate::SelectOption::new(PickerStyle::Popover, "Calendar popover"),
                    ],
                }
                crate::Checkbox { checked: limit, label: "From today, one year ahead" }
                crate::Checkbox { checked: inline, label: "Show the calendar on its own" }
            },
            crate::Stack {
                DatePicker {
                    label: "Tee day",
                    value: picked,
                    style: style(),
                    min: limit().then_some(today),
                    max: limit().then(|| today.add_months(12)),
                    helper: match picked() {
                        Some(date) => format!("Chosen: {date}"),
                        None => "Nothing chosen yet".to_string(),
                    },
                }
                if inline() {
                    crate::Card { variant: crate::CardVariant::Flat,
                        Calendar {
                            value: picked,
                            min: limit().then_some(today),
                            max: limit().then(|| today.add_months(12)),
                        }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "DatePicker",
    description: "Pick a date with iOS wheels or a Material calendar.",
    components: ["DatePicker", "Calendar"],
    demo: DatePickerPlaygroundDemo,
    source: "src/components/date_picker.rs",
}
