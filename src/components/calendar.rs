//! A month calendar for picking a date.
use super::datetime::CalendarDate;
use super::overlay::js_string;
use crate::state::{use_controlled, use_element_id};
use crate::theme::{
    ComponentMode, Strings, classes, merge_classes, use_component_mode, use_strings,
};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, ChevronLeft, ChevronRight};

/// "Saturday, September 19, 2026", for accessible names.
pub(crate) fn long_date(strings: &Strings, date: CalendarDate) -> String {
    format!(
        "{}, {} {}, {}",
        name_at(&strings.weekdays, usize::from(date.weekday())),
        name_at(&strings.months, usize::from(date.month() - 1)),
        date.day(),
        date.year()
    )
}

/// "Sep 19, 2026", for a field showing the chosen date.
pub(crate) fn short_date(strings: &Strings, date: CalendarDate) -> String {
    format!(
        "{} {}, {}",
        name_at(&strings.months_short, usize::from(date.month() - 1)),
        date.day(),
        date.year()
    )
}

/// "September 2026".
pub(crate) fn month_year(strings: &Strings, date: CalendarDate) -> String {
    format!(
        "{} {}",
        name_at(&strings.months, usize::from(date.month() - 1)),
        date.year()
    )
}

/// A name from a localised list, tolerating a list that is too short.
pub(crate) fn name_at(names: &[String], index: usize) -> String {
    names
        .get(index)
        .cloned()
        .unwrap_or_else(|| (index + 1).to_string())
}

/// The days shown for `month`: leading blanks up to the first weekday, then
/// each day of the month.
fn month_cells(month: CalendarDate, first_weekday: u8) -> Vec<Option<CalendarDate>> {
    let first = month.first_of_month();
    let lead = (7 + first.weekday() - first_weekday % 7) % 7;
    let mut cells = vec![None; usize::from(lead)];
    let mut day = first;
    while day.month() == first.month() {
        cells.push(Some(day));
        day = day.add_days(1);
    }
    cells
}

#[derive(Clone, Copy, PartialEq)]
enum View {
    Days,
    Years,
}

/// Moves keyboard focus onto a day once it has rendered.
const FOCUS_SCRIPT: &str = r#"
const el = document.getElementById(__ID__);
if (el) el.focus({ preventScroll: false });
"#;

/// Brings the chosen year into view in the year list.
const YEAR_SCRIPT: &str = r#"
const list = document.getElementById(__ID__);
const year = list && list.querySelector("[aria-current=true]");
if (year) year.scrollIntoView({ block: "center" });
if (year) year.focus({ preventScroll: true });
"#;

/// A month calendar for picking a date. Like the calendar in iOS's inline
/// date picker and Material's date picker.
///
/// The arrow keys move between days, Home and End go to the start and end of
/// the week, and Page Up and Page Down change the month (with Shift, the
/// year). The month title opens a list of years.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let day = use_signal(|| None::<CalendarDate>);
/// rsx! {
///     Calendar { value: day, min: CalendarDate::today() }
/// }
/// # }
/// ```
#[component]
pub fn Calendar(
    /// The chosen date. Kept internally when not given.
    value: Option<Signal<Option<CalendarDate>>>,
    /// The earliest date that can be picked.
    min: Option<CalendarDate>,
    /// The latest date that can be picked.
    max: Option<CalendarDate>,
    /// Returns `true` for dates that cannot be picked, such as weekends.
    is_date_disabled: Option<Callback<CalendarDate, bool>>,
    /// Called with the date the user picks.
    onchange: Option<EventHandler<CalendarDate>>,
    /// Accessible name of the calendar. Defaults to
    /// [`Strings::choose_date`](crate::Strings::choose_date).
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the calendar.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let id = use_element_id("calendar", None);
    let mut value = use_controlled(value, || None);
    let today = use_hook(CalendarDate::today);
    let start = use_hook(|| value.peek().unwrap_or(today).clamp_to(min, max));
    let mut focused = use_signal(|| start);
    let mut shown = use_signal(|| start.first_of_month());
    let mut view = use_signal(|| View::Days);
    let mut focus_pending = use_signal(|| false);

    // Follow a value set from outside.
    use_effect(move || {
        if let Some(date) = value() {
            if date.first_of_month() != *shown.peek() {
                shown.set(date.first_of_month());
            }
            if date != *focused.peek() {
                focused.set(date);
            }
        }
    });

    let disabled = move |date: CalendarDate| {
        min.is_some_and(|min| date < min)
            || max.is_some_and(|max| date > max)
            || is_date_disabled.is_some_and(|check| check.call(date))
    };
    let mut pick = move |date: CalendarDate| {
        if disabled(date) {
            return;
        }
        value.set(Some(date));
        focused.set(date);
        if let Some(onchange) = onchange {
            onchange.call(date);
        }
    };
    let mut move_focus = move |date: CalendarDate| {
        let date = date.clamp_to(min, max);
        focused.set(date);
        if date.first_of_month() != *shown.peek() {
            shown.set(date.first_of_month());
        }
        focus_pending.set(true);
    };

    {
        let id = id.clone();
        use_effect(move || {
            let date = focused();
            if focus_pending() && view() == View::Days {
                focus_pending.set(false);
                let day_id = format!("{id}-{date}");
                document::eval(&FOCUS_SCRIPT.replace("__ID__", &js_string(&day_id)));
            }
        });
    }
    {
        let id = id.clone();
        use_effect(move || {
            if view() == View::Years {
                document::eval(&YEAR_SCRIPT.replace("__ID__", &js_string(&format!("{id}-years"))));
            }
        });
    }

    let month = shown();
    let title_id = format!("{id}-title");
    let prev_month = month.add_months(-1);
    let next_month = month.add_months(1);
    let can_go_back = min.is_none_or(|min| prev_month.add_months(1).add_days(-1) >= min);
    let can_go_forward = max.is_none_or(|max| next_month <= max);
    let first_weekday = strings.first_weekday % 7;
    let cells = month_cells(month, first_weekday);
    let weeks = cells.chunks(7).map(<[_]>::to_vec).collect::<Vec<_>>();
    let chosen = value();
    let focus_date = focused();
    let years = {
        let low = min.map_or(today.year() - 100, CalendarDate::year);
        let high = max.map_or(today.year() + 50, CalendarDate::year);
        (low..=high).collect::<Vec<_>>()
    };
    let cls = classes([
        "g3-calendar",
        mode.pick("g3-calendar-ios", "g3-calendar-md"),
    ]);

    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            role: "group",
            aria_label: aria_label.unwrap_or_else(|| strings.choose_date.clone()),
            div { class: "g3-calendar-header",
                button {
                    id: title_id.clone(),
                    r#type: "button",
                    class: "g3-calendar-title",
                    aria_live: "polite",
                    aria_expanded: (view() == View::Years).to_string(),
                    aria_controls: format!("{id}-years"),
                    onclick: move |_| {
                        view.set(if view() == View::Days { View::Years } else { View::Days });
                    },
                    "{month_year(&strings, month)}"
                    span { class: "g3-calendar-title-icon", aria_hidden: "true",
                        ChevronDown { size: 16 }
                    }
                }
                if view() == View::Days {
                    div { class: "g3-calendar-nav",
                        button {
                            r#type: "button",
                            class: "g3-calendar-step",
                            aria_label: strings.previous_month.clone(),
                            disabled: !can_go_back,
                            onclick: move |_| {
                                shown.set(prev_month);
                                focused.set(focus_date.add_months(-1).clamp_to(min, max));
                            },
                            ChevronLeft { size: 20 }
                        }
                        button {
                            r#type: "button",
                            class: "g3-calendar-step",
                            aria_label: strings.next_month.clone(),
                            disabled: !can_go_forward,
                            onclick: move |_| {
                                shown.set(next_month);
                                focused.set(focus_date.add_months(1).clamp_to(min, max));
                            },
                            ChevronRight { size: 20 }
                        }
                    }
                }
            }
            if view() == View::Years {
                div {
                    id: format!("{id}-years"),
                    class: "g3-calendar-years",
                    role: "group",
                    aria_label: strings.year.clone(),
                    for year in years {
                        button {
                            key: "{year}",
                            r#type: "button",
                            class: "g3-calendar-year",
                            aria_current: (year == month.year()).then_some("true"),
                            onclick: move |_| {
                                let target = focus_date
                                    .add_months((year - focus_date.year()) * 12)
                                    .clamp_to(min, max);
                                shown.set(target.first_of_month());
                                focused.set(target);
                                view.set(View::Days);
                                focus_pending.set(true);
                            },
                            "{year}"
                        }
                    }
                }
            } else {
                table {
                    class: "g3-calendar-grid",
                    role: "grid",
                    aria_labelledby: title_id.clone(),
                    onkeydown: move |event| {
                        let date = focused();
                        let shift = event.modifiers().shift();
                        let next = match event.key() {
                            Key::ArrowLeft => date.add_days(-1),
                            Key::ArrowRight => date.add_days(1),
                            Key::ArrowUp => date.add_days(-7),
                            Key::ArrowDown => date.add_days(7),
                            Key::Home => date.add_days(-i64::from((7 + date.weekday() - first_weekday) % 7)),
                            Key::End => date.add_days(i64::from(6 - (7 + date.weekday() - first_weekday) % 7)),
                            Key::PageUp => date.add_months(if shift { -12 } else { -1 }),
                            Key::PageDown => date.add_months(if shift { 12 } else { 1 }),
                            _ => return,
                        };
                        event.prevent_default();
                        move_focus(next);
                    },
                    thead {
                        tr {
                            for offset in 0..7_u8 {
                                th {
                                    key: "{offset}",
                                    scope: "col",
                                    abbr: name_at(&strings.weekdays, usize::from((first_weekday + offset) % 7)),
                                    "{name_at(&strings.weekdays_narrow, usize::from((first_weekday + offset) % 7))}"
                                }
                            }
                        }
                    }
                    tbody {
                        for (row, week) in weeks.into_iter().enumerate() {
                            tr { key: "{month}-{row}",
                                for (column, cell) in week.into_iter().enumerate() {
                                    match cell {
                                        None => rsx! {
                                            td { key: "{column}", role: "gridcell" }
                                        },
                                        Some(date) => {
                                            let off = disabled(date);
                                            let selected = chosen == Some(date);
                                            rsx! {
                                                td {
                                                    key: "{column}",
                                                    role: "gridcell",
                                                    aria_selected: selected.to_string(),
                                                    button {
                                                        id: format!("{id}-{date}"),
                                                        r#type: "button",
                                                        class: classes([
                                                            "g3-calendar-day",
                                                            if date == today { "g3-calendar-today" } else { "" },
                                                        ]),
                                                        tabindex: if date == focus_date { "0" } else { "-1" },
                                                        aria_label: long_date(&strings, date),
                                                        aria_current: (date == today).then_some("date"),
                                                        disabled: off,
                                                        onclick: move |_| pick(date),
                                                        "{date.day()}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_cells_start_on_the_first_weekday() {
        let september = CalendarDate::new(2026, 9, 1).unwrap(); // a Tuesday
        let sunday_first = month_cells(september, 0);
        assert_eq!(
            sunday_first
                .iter()
                .take_while(|cell| cell.is_none())
                .count(),
            2
        );
        assert_eq!(sunday_first.len(), 2 + 30);
        let monday_first = month_cells(september, 1);
        assert_eq!(
            monday_first
                .iter()
                .take_while(|cell| cell.is_none())
                .count(),
            1
        );
    }

    #[test]
    fn dates_read_in_full() {
        let strings = Strings::default();
        let date = CalendarDate::new(2026, 9, 19).unwrap();
        assert_eq!(long_date(&strings, date), "Saturday, September 19, 2026");
        assert_eq!(short_date(&strings, date), "Sep 19, 2026");
        assert_eq!(month_year(&strings, date), "September 2026");
    }
}
