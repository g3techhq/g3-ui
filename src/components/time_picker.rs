//! A form field that picks a time of day.
use super::clock::{ClockDial, DialMode};
use super::date_picker::{PickerStyle, clock_icon, picker_trigger};
use super::datetime::{HourCycle, TimeOfDay};
use super::field::{FieldShell, described_by, is_invalid};
use super::popover::{PopoverFrame, PopoverPlacement};
use super::wheel::WheelColumn;
use super::{BottomSheet, Button, ButtonFill, ButtonSize, Input, InputType, Modal};
use crate::state::{use_controlled, use_element_id, use_synced_signal};
use crate::theme::{ComponentMode, Strings, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::{Clock, Keyboard};

/// "2:05 PM" on a 12-hour clock, "14:05" on a 24-hour one.
pub(crate) fn format_time(strings: &Strings, time: TimeOfDay, cycle: HourCycle) -> String {
    match cycle {
        HourCycle::H24 => time.to_string(),
        HourCycle::H12 => {
            let (hour, pm) = time.hour12();
            let half = if pm { &strings.pm } else { &strings.am };
            format!("{hour}:{:02} {half}", time.minute())
        }
    }
}

/// Hour, minute, and (on a 12-hour clock) AM/PM columns.
#[component]
fn TimeWheels(draft: Signal<TimeOfDay>, cycle: HourCycle, step: u8) -> Element {
    let strings = use_strings();
    let time = draft();
    let (hour12, pm) = time.hour12();
    let step = step.clamp(1, 30);
    let minutes = (0..60).step_by(usize::from(step)).collect::<Vec<u8>>();
    let minute_index = minutes
        .iter()
        .position(|minute| *minute >= time.minute())
        .unwrap_or(0);
    let hours: Vec<String> = match cycle {
        HourCycle::H12 => (1..=12).map(|hour| hour.to_string()).collect(),
        HourCycle::H24 => (0..24).map(|hour| format!("{hour:02}")).collect(),
    };
    let hour_index = match cycle {
        HourCycle::H12 => usize::from(hour12 - 1),
        HourCycle::H24 => usize::from(time.hour()),
    };
    let set = move |hour: u8, minute: u8| {
        let mut draft = draft;
        if let Some(next) = TimeOfDay::new(hour, minute) {
            draft.set(next);
        }
    };
    rsx! {
        div { class: "g3-wheel",
            WheelColumn {
                labels: hours,
                index: hour_index,
                aria_label: strings.hour.clone(),
                onchange: move |index: usize| {
                    let hour = match cycle {
                        HourCycle::H12 => {
                            TimeOfDay::from_hour12(index as u8 + 1, time.minute(), pm)
                                .map_or(time.hour(), TimeOfDay::hour)
                        }
                        HourCycle::H24 => index as u8,
                    };
                    set(hour, time.minute());
                },
            }
            WheelColumn {
                labels: minutes.iter().map(|minute| format!("{minute:02}")).collect(),
                index: minute_index,
                aria_label: strings.minute.clone(),
                onchange: move |index: usize| set(time.hour(), index as u8 * step),
            }
            if cycle == HourCycle::H12 {
                WheelColumn {
                    labels: vec![strings.am.clone(), strings.pm.clone()],
                    index: usize::from(pm),
                    aria_label: strings.day_period.clone(),
                    onchange: move |index: usize| {
                        let (hour12, _) = time.hour12();
                        if let Some(next) = TimeOfDay::from_hour12(hour12, time.minute(), index == 1) {
                            let mut draft = draft;
                            draft.set(next);
                        }
                    },
                }
            }
        }
    }
}

/// Which part of the time the dial is editing.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    Hour,
    Minute,
}

/// The Material time picker body: a readout above a clock face, or two
/// fields when the user would rather type.
#[component]
fn TimeDialog(draft: Signal<TimeOfDay>, cycle: HourCycle, typing: Signal<bool>) -> Element {
    let strings = use_strings();
    let mut part = use_signal(|| Part::Hour);
    let time = draft();
    let (hour12, pm) = time.hour12();
    let hour_text = match cycle {
        HourCycle::H12 => hour12.to_string(),
        HourCycle::H24 => format!("{:02}", time.hour()),
    };
    let minute_text = format!("{:02}", time.minute());
    // The typed fields follow the draft, and the draft follows what is typed.
    let hour_field = use_synced_signal(hour_text.clone());
    let minute_field = use_synced_signal(minute_text.clone());
    let set_hour = move |hour: u8| {
        let mut draft = draft;
        let minute = draft.peek().minute();
        if let Some(next) = TimeOfDay::new(hour.min(23), minute) {
            draft.set(next);
        }
    };
    let set_hour12 = move |hour: u8| {
        let mut draft = draft;
        let current = *draft.peek();
        if let Some(next) = TimeOfDay::from_hour12(hour, current.minute(), current.hour() >= 12) {
            draft.set(next);
        }
    };
    let set_minute = move |minute: u8| {
        let mut draft = draft;
        let hour = draft.peek().hour();
        if let Some(next) = TimeOfDay::new(hour, minute.min(59)) {
            draft.set(next);
        }
    };
    let set_half = move |afternoon: bool| {
        let mut draft = draft;
        let current = *draft.peek();
        let (hour12, _) = current.hour12();
        if let Some(next) = TimeOfDay::from_hour12(hour12, current.minute(), afternoon) {
            draft.set(next);
        }
    };

    rsx! {
        div { class: "g3-time-picker",
            div { class: "g3-time-readout",
                button {
                    r#type: "button",
                    class: "g3-time-part",
                    aria_label: format!("{}, {hour_text}", strings.hour),
                    aria_pressed: (part() == Part::Hour && !typing()).to_string(),
                    onclick: move |_| part.set(Part::Hour),
                    "{hour_text}"
                }
                span { class: "g3-time-separator", aria_hidden: "true", ":" }
                button {
                    r#type: "button",
                    class: "g3-time-part",
                    aria_label: format!("{}, {minute_text}", strings.minute),
                    aria_pressed: (part() == Part::Minute && !typing()).to_string(),
                    onclick: move |_| part.set(Part::Minute),
                    "{minute_text}"
                }
                if cycle == HourCycle::H12 {
                    div { class: "g3-time-half", role: "radiogroup", aria_label: strings.day_period.clone(),
                        button {
                            r#type: "button",
                            class: "g3-time-half-option",
                            role: "radio",
                            aria_checked: (!pm).to_string(),
                            onclick: move |_| set_half(false),
                            "{strings.am}"
                        }
                        button {
                            r#type: "button",
                            class: "g3-time-half-option",
                            role: "radio",
                            aria_checked: pm.to_string(),
                            onclick: move |_| set_half(true),
                            "{strings.pm}"
                        }
                    }
                }
            }
            if typing() {
                div { class: "g3-time-inputs",
                    Input {
                        label: strings.hour.clone(),
                        input_type: InputType::Number,
                        value: hour_field,
                        min: match cycle { HourCycle::H12 => 1.0, HourCycle::H24 => 0.0 },
                        max: match cycle { HourCycle::H12 => 12.0, HourCycle::H24 => 23.0 },
                        maxlength: 2,
                        oninput: move |text: String| {
                            if let Ok(hour) = text.trim().parse::<u8>() {
                                match cycle {
                                    HourCycle::H12 if (1..=12).contains(&hour) => set_hour12(hour),
                                    HourCycle::H24 if hour <= 23 => set_hour(hour),
                                    _ => {}
                                }
                            }
                        },
                    }
                    Input {
                        label: strings.minute.clone(),
                        input_type: InputType::Number,
                        value: minute_field,
                        min: 0.0,
                        max: 59.0,
                        maxlength: 2,
                        oninput: move |text: String| {
                            if let Ok(minute) = text.trim().parse::<u8>()
                                && minute <= 59
                            {
                                set_minute(minute);
                            }
                        },
                    }
                }
            } else {
                match part() {
                    Part::Hour => rsx! {
                        ClockDial {
                            mode: match cycle { HourCycle::H12 => DialMode::Hours12, HourCycle::H24 => DialMode::Hours24 },
                            value: match cycle { HourCycle::H12 => hour12, HourCycle::H24 => time.hour() },
                            aria_label: strings.hour.clone(),
                            mark_label: Callback::new(move |hour: u8| format!("{hour} o'clock")),
                            onchange: move |hour: u8| match cycle {
                                HourCycle::H12 => set_hour12(hour),
                                HourCycle::H24 => set_hour(hour),
                            },
                            on_release: move |()| part.set(Part::Minute),
                        }
                    },
                    Part::Minute => rsx! {
                        ClockDial {
                            mode: DialMode::Minutes,
                            value: time.minute(),
                            aria_label: strings.minute.clone(),
                            mark_label: Callback::new(move |minute: u8| format!("{minute} minutes")),
                            onchange: move |minute: u8| set_minute(minute),
                            on_release: move |()| {},
                        }
                    },
                }
            }
        }
    }
}

/// A form field that picks a time, with iOS wheels or a Material clock
/// dialog. Like Ionic's `ion-datetime` with `presentation="time"`.
///
/// The Material dialog also has a typing mode, for entering a time straight
/// from the keyboard.
///
/// ```rust,ignore
/// let tee_time = use_signal(|| None::<TimeOfDay>);
/// rsx! {
///     TimePicker { label: "Tee time", value: tee_time, minute_step: 10 }
/// }
/// ```
#[component]
pub fn TimePicker(
    /// The chosen time. Kept internally when not given.
    value: Option<Signal<Option<TimeOfDay>>>,
    /// Visible label, which also names the field.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Text while no time is chosen. Defaults to
    /// [`Strings::choose_time`](crate::Strings::choose_time).
    placeholder: Option<String>,
    /// Help text below the field.
    helper: Option<String>,
    /// Error text below the field.
    error: Option<String>,
    /// Mark the field required.
    required: Option<bool>,
    /// Disable the field.
    disabled: Option<bool>,
    /// 12-hour or 24-hour clock. Defaults to [`HourCycle::H12`].
    hour_cycle: Option<HourCycle>,
    /// Minutes between the values a wheel offers. Defaults to 1.
    minute_step: Option<u8>,
    /// How the picker opens. Defaults to [`PickerStyle::Auto`].
    style: Option<PickerStyle>,
    /// How the field shows the chosen time.
    format: Option<Callback<TimeOfDay, String>>,
    /// Called with the time the user confirms.
    onchange: Option<EventHandler<TimeOfDay>>,
    /// Element id of the field. Generated when not given.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("time", id);
    let mut value = use_controlled(value, || None);
    let mut open = use_signal(|| false);
    let mut typing = use_signal(|| false);
    let cycle = hour_cycle.unwrap_or_default();
    let step = minute_step.unwrap_or(1).clamp(1, 30);
    let mut draft = use_signal(TimeOfDay::now);
    use_effect(move || {
        if open() {
            draft.set(value.peek().unwrap_or_else(TimeOfDay::now));
            typing.set(false);
        }
    });
    let mut commit = move |time: TimeOfDay| {
        value.set(Some(time));
        open.set(false);
        if let Some(onchange) = onchange {
            onchange.call(time);
        }
    };

    let style = style.unwrap_or_default().resolve(mode);
    let disabled = disabled.unwrap_or(false);
    let placeholder = placeholder.unwrap_or_else(|| strings.choose_time.clone());
    let text = value().map(|time| match format {
        Some(format) => format.call(time),
        None => format_time(&strings, time, cycle),
    });
    let describedby = described_by(&id, &helper, &error);
    let invalid = is_invalid(&error);
    let title = label.clone().unwrap_or_else(|| strings.choose_time.clone());
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
        clock_icon(),
    );

    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: required.unwrap_or(false),
            helper,
            error,
            class: merge_classes("g3-picker-field", class.as_deref()),
            match style {
                PickerStyle::Dialog => rsx! {
                    {trigger}
                    Modal {
                        open,
                        title: strings.choose_time.clone(),
                        mode,
                        class: "g3-picker-dialog",
                        actions: rsx! {
                            Button {
                                fill: ButtonFill::Clear,
                                size: ButtonSize::Sm,
                                mode,
                                class: "g3-time-mode-toggle",
                                aria_label: if typing() { strings.pick_time_on_dial.clone() } else { strings.type_time.clone() },
                                onclick: move |_| typing.toggle(),
                                if typing() {
                                    Clock { size: 20 }
                                } else {
                                    Keyboard { size: 20 }
                                }
                            }
                            Button { fill: ButtonFill::Clear, mode, onclick: move |_| open.set(false), "{strings.cancel}" }
                            Button { fill: ButtonFill::Clear, mode, onclick: move |_| commit(draft()), "{strings.confirm}" }
                        },
                        if open() {
                            TimeDialog { draft, cycle, typing }
                        }
                    }
                },
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
                            TimeWheels { draft, cycle, step }
                            div { class: "g3-picker-toolbar g3-picker-toolbar-end",
                                Button { fill: ButtonFill::Clear, mode, onclick: move |_| commit(draft()), "{strings.done}" }
                            }
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
                            Button { fill: ButtonFill::Clear, mode, onclick: move |_| commit(draft()), "{strings.done}" }
                        }
                        TimeWheels { draft, cycle, step }
                    }
                },
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TimePickerPlaygroundDemo() -> Element {
    let picked = use_signal(|| None::<TimeOfDay>);
    let style = use_signal(|| PickerStyle::Auto);
    let cycle = use_signal(|| HourCycle::H12);
    let step = use_signal(|| 5_usize);
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
                        crate::SelectOption::new(PickerStyle::Dialog, "Clock dialog"),
                        crate::SelectOption::new(PickerStyle::Popover, "Wheels in a popover"),
                    ],
                }
                crate::SegmentGroup { value: cycle, aria_label: "Clock",
                    crate::SegmentButton { value: HourCycle::H12, "12 hour" }
                    crate::SegmentButton { value: HourCycle::H24, "24 hour" }
                }
                crate::Select {
                    label: "Minutes on the wheel",
                    value: step,
                    options: vec![
                        crate::SelectOption::new(1_usize, "Every minute"),
                        crate::SelectOption::new(5_usize, "Every 5 minutes"),
                        crate::SelectOption::new(15_usize, "Every 15 minutes"),
                    ],
                }
            },
            TimePicker {
                label: "Tee time",
                value: picked,
                style: style(),
                hour_cycle: cycle(),
                minute_step: step() as u8,
                helper: match picked() {
                    Some(time) => format!("Chosen: {time}"),
                    None => "Nothing chosen yet".to_string(),
                },
            }
        }
    }
}

crate::g3_playground! {
    name: "TimePicker",
    description: "Pick a time with iOS wheels or a Material clock face.",
    components: ["TimePicker"],
    demo: TimePickerPlaygroundDemo,
    source: "src/components/time_picker.rs",
}
