//! Markup contracts for form controls and actions.
use super::*;

#[test]
fn inputs_with_the_same_label_get_distinct_linked_ids() {
    fn app() -> Element {
        rsx! {
            Input { label: "Name" }
            Input { label: "Name", helper: "Shown to others", error: "Required", required: true }
        }
    }
    let html = render(app);
    let ids: Vec<_> = ids(&html)
        .into_iter()
        .filter(|id| !id.ends_with("-label") && !id.ends_with("-helper") && !id.ends_with("-error"))
        .collect();
    assert_eq!(ids.len(), 2);
    assert_ne!(ids[0], ids[1]);
    assert!(!ids[0].contains(' '));
    for id in &ids {
        assert!(html.contains(&format!("for=\"{id}\"")));
    }
    let second = element_with_id(&html, &ids[1]);
    assert!(second.contains(&format!(
        "aria-describedby=\"{0}-helper {0}-error\"",
        ids[1]
    )));
    assert!(second.contains("aria-invalid=\"true\""));
    assert!(html.contains("g3-field-required"));
}

#[test]
fn inputs_use_typed_kinds() {
    fn app() -> Element {
        rsx! {
            Input { aria_label: "When", input_type: InputType::Tel }
            TextArea { label: "Notes", rows: 4 }
        }
    }
    let html = render(app);
    assert!(html.contains("type=\"tel\""));
    assert!(html.contains("aria-label=\"When\""));
    assert!(html.contains("<textarea"));
}

#[test]
fn typing_limits_count_characters_and_ignore_minimums() {
    use crate::components::field_limits as within;
    assert!(within("é".repeat(3).as_str(), None, Some(3)));
    assert!(!within("é".repeat(4).as_str(), None, Some(3)));
    assert!(within("1", Some(10.0), None));
    assert!(!within("11", Some(10.0), None));
    assert!(within("-", Some(10.0), None));
}

#[test]
fn select_shows_the_chosen_label_not_the_value() {
    fn app() -> Element {
        let value = use_signal(|| "gb".to_string());
        rsx! {
            Select {
                label: "Country",
                value,
                options: vec![SelectOption::new("us".to_string(), "United States"), SelectOption::new("gb".to_string(), "United Kingdom")],
            }
        }
    }
    let html = render(app);
    assert!(html.contains("g3-select-value\">United Kingdom</span>"));
    assert!(!html.contains("g3-select-value\">gb"));
    let trigger = element_with_class(&html, "g3-select");
    assert!(trigger.contains("aria-haspopup=\"listbox\""));
    assert!(trigger.contains("aria-expanded=\"false\""));
}

#[test]
fn select_shows_a_placeholder_for_an_unknown_value() {
    fn app() -> Element {
        rsx! {
            Select::<u8> {
                aria_label: "Tee time",
                placeholder: "Choose",
                default_value: 0,
                options: vec![SelectOption::new(8, "8:00")],
            }
        }
    }
    let html = render(app);
    assert!(element_with_class(&html, "g3-select-placeholder").contains("g3-select-value"));
    assert!(html.contains(">Choose</span>"));
}

#[test]
fn checkbox_reports_mixed_and_bare_states() {
    fn app() -> Element {
        rsx! {
            Checkbox { label: "All", indeterminate: true }
            Checkbox { aria_label: "Row" }
        }
    }
    let html = render(app);
    assert!(html.contains("role=\"checkbox\""));
    assert!(html.contains("aria-checked=\"mixed\""));
    assert!(html.contains("g3-control-bare"));
    assert!(html.contains("aria-label=\"Row\""));
}

#[test]
fn toggle_is_a_labelled_switch() {
    fn app() -> Element {
        let on = use_signal(|| true);
        rsx! {
            Toggle { checked: on, label: "Alerts", disabled: true }
        }
    }
    let html = render(app);
    let toggle = element_with_class(&html, "g3-toggle");
    assert!(toggle.contains("role=\"switch\""));
    assert!(toggle.contains("aria-checked=\"true\""));
    assert!(toggle.contains("disabled"));
    assert!(html.contains(">Alerts</span>"));
}

#[test]
fn radios_share_a_name_and_mark_the_selection() {
    fn app() -> Element {
        let value = use_signal(|| Some(2));
        rsx! {
            RadioGroup { value, label: "Holes",
                Radio { value: 1, label: "Nine" }
                Radio { value: 2, label: "Eighteen" }
            }
        }
    }
    let html = render(app);
    let group = element_with_class(&html, "g3-radio-group");
    assert!(group.contains("role=\"radiogroup\""));
    assert!(group.contains("aria-labelledby="));
    let names: Vec<_> = html
        .split("name=\"")
        .skip(1)
        .map(|chunk| chunk.split('"').next().unwrap())
        .collect();
    assert_eq!(names.len(), 2);
    assert_eq!(names[0], names[1]);
    assert_eq!(html.matches(" checked").count(), 1);
    assert!(!html.contains("role=\"radio\""));
}

#[test]
fn stepper_and_range_expose_their_values() {
    fn app() -> Element {
        let count = use_signal(|| 8_i64);
        rsx! {
            Stepper { label: "Players", value: count, min: 1, max: 8 }
            Range { aria_label: "Volume", show_value: true }
        }
    }
    let html = render(app);
    let stepper = element_with_class(&html, "g3-stepper");
    assert!(stepper.contains("role=\"spinbutton\""));
    assert!(stepper.contains("aria-valuenow=\"8\""));
    assert!(stepper.contains("aria-valuemax=\"8\""));
    assert!(html.contains("type=\"range\""));
}

#[test]
fn searchbar_is_a_search_landmark() {
    fn app() -> Element {
        rsx! {
            Searchbar {}
        }
    }
    let html = render(app);
    assert!(element_with_class(&html, "g3-searchbar").contains("role=\"search\""));
    assert!(html.contains("type=\"search\""));
    assert!(html.contains("aria-label=\"Search\""));
    assert!(!html.contains(">Cancel</button>"));
}

#[test]
fn buttons_choose_their_element_and_type() {
    fn app() -> Element {
        rsx! {
            Button { button_type: ButtonType::Submit, "Save" }
            Button { href: "/help", fill: ButtonFill::Clear, color: Color::Danger, "Help" }
            Button { loading: true, aria_label: "Saving", "Save" }
            Button { disabled: true, href: "/x", "Gone" }
        }
    }
    let html = render(app);
    assert!(html.contains("type=\"submit\""));
    let link = element_with_class(&html, "g3-btn-danger");
    assert!(link.starts_with("<a"));
    assert!(link.contains("href=\"/help\""));
    assert!(link.contains("g3-btn-clear"));
    let loading = element_with_class(&html, "g3-btn-loading");
    assert!(loading.contains("aria-busy=\"true\""));
    assert!(loading.contains("disabled"));
    assert!(loading.contains("aria-label=\"Saving\""));
    assert!(html.contains("aria-disabled=\"true\""));
    assert!(!html.contains("href=\"/x\""));
}

#[test]
fn fab_menu_toggles_its_list() {
    fn app() -> Element {
        rsx! {
            FabMenu { aria_label: "Create", icon: rsx! { "+" },
                FabButton { aria_label: "Round", "R" }
            }
        }
    }
    let html = render(app);
    let main = element_with_class(&html, "g3-fab");
    assert!(main.contains("aria-expanded=\"false\""));
    assert!(main.contains("aria-label=\"Create\""));
    let list = element_with_class(&html, "g3-fab-list");
    assert!(list.contains("aria-hidden=\"true\""));
    assert!(list.contains("inert"));
    assert!(element_with_class(&html, "g3-fab-container").contains("g3-fab-vertical-bottom"));
}

#[test]
fn a_calendar_is_a_grid_of_days() {
    fn app() -> Element {
        let day = use_signal(|| CalendarDate::new(2026, 9, 19));
        rsx! {
            Calendar { value: day, min: CalendarDate::new(2026, 9, 10).unwrap() }
        }
    }
    let html = render(app);
    assert!(element_with_class(&html, "g3-calendar-grid").contains("role=\"grid\""));
    assert!(html.contains("September 2026"));
    // Every day of the month, and the leading blanks to its first weekday.
    assert_eq!(html.matches("g3-calendar-day").count(), 30);
    assert!(html.contains("aria-label=\"Saturday, September 19, 2026\""));
    let selected = html
        .split("aria-selected=\"true\"")
        .nth(1)
        .expect("a selected day");
    assert!(selected.contains("tabindex=\"0\""));
    // Days before the minimum cannot be picked.
    assert!(html.contains("aria-label=\"Tuesday, September 1, 2026\" disabled"));
}

#[test]
fn pickers_are_fields_that_open_a_dialog() {
    fn app() -> Element {
        let day = use_signal(|| CalendarDate::new(2026, 9, 19));
        let time = use_signal(|| TimeOfDay::new(14, 5));
        rsx! {
            DatePicker { label: "Tee day", value: day }
            TimePicker { label: "Tee time", value: time }
            TimePicker { aria_label: "Round start", hour_cycle: HourCycle::H24 }
        }
    }
    let html = render(app);
    let triggers = html.matches("aria-haspopup=\"dialog\"").count();
    assert_eq!(triggers, 3);
    assert!(html.contains(">Sep 19, 2026</span>"));
    assert!(html.contains(">2:05 PM</span>"));
    // An empty picker shows its placeholder, not a made-up time.
    assert!(html.contains("g3-select-placeholder\">Select time"));
    assert!(html.contains("aria-label=\"Round start\""));
}

#[test]
fn a_rating_is_a_slider_and_a_display_is_an_image() {
    fn input() -> Element {
        let stars = use_signal(|| 3.5);
        rsx! { Rating { label: "Your rating", value: stars, half: true } }
    }
    let html = render(input);
    assert!(html.contains(r#"role="slider""#));
    assert!(html.contains(r#"aria-valuemin="0""#));
    assert!(html.contains(r#"aria-valuemax="5""#));
    assert!(html.contains(r#"aria-valuenow="3.5""#));
    assert!(html.contains(r#"aria-valuetext="3.5 of 5 stars""#));
    assert!(html.contains(r#"tabindex="0""#));
    assert!(html.contains("-label\""), "named by its visible label");
    assert_eq!(html.matches("g3-rating-star").count(), 5);

    fn display() -> Element {
        rsx! { Rating { aria_label: "Average", value: use_signal(|| 3.7), readonly: true } }
    }
    let html = render(display);
    assert!(
        html.contains(r#"role="img""#),
        "a display is a picture of a number"
    );
    assert!(!html.contains("role=\"slider\""));
    assert!(!html.contains("tabindex"), "nothing to focus in a display");
    // A display keeps the fraction rather than snapping it to a step.
    assert!(html.contains(r#"aria-label="Average: 3.7 of 5 stars""#));
}

#[test]
fn a_labelled_rating_display_still_says_its_value() {
    // aria-labelledby would replace aria-label, dropping the number, so a
    // display with a visible label folds that label into its own name.
    fn app() -> Element {
        rsx! { Rating { label: "Average of 214 ratings", value: use_signal(|| 3.7), readonly: true } }
    }
    let html = render(app);
    assert!(html.contains(r#"aria-label="Average of 214 ratings: 3.7 of 5 stars""#));
    assert!(!html.contains("aria-labelledby"));
}

#[test]
fn a_labelled_segment_group_is_named_by_its_visible_label() {
    fn labelled() -> Element {
        let theme = use_signal(|| 0_u8);
        rsx! {
            SegmentGroup { value: theme, label: "Theme",
                SegmentButton { value: 0_u8, "Light" }
                SegmentButton { value: 1_u8, "Dark" }
            }
        }
    }
    let html = render(labelled);
    assert!(
        html.contains("g3-field-label"),
        "the label shows like other form labels"
    );
    assert!(html.contains("aria-labelledby="), "and names the group");
    assert!(!html.contains("<label"), "a radio group is not labelable");

    fn toolbar() -> Element {
        let tab = use_signal(|| 0_u8);
        rsx! {
            SegmentGroup { value: tab, aria_label: "Round",
                SegmentButton { value: 0_u8, "Card" }
            }
        }
    }
    let html = render(toolbar);
    assert!(html.contains(r#"aria-label="Round""#));
    assert!(!html.contains("g3-field"), "no wrapper without a label");
}

/// A group handed a different `value` signal follows it. RadioGroup used to
/// provide its context once, so after the switch its radios went on showing,
/// and writing to, the first signal: in greenside-partee, picking the second
/// game's type changed the first game's.
#[test]
fn a_radio_group_follows_a_new_value_signal() {
    static SECOND: GlobalSignal<bool> = Signal::global(|| false);
    fn app() -> Element {
        let first = use_signal(|| Some(1));
        let second = use_signal(|| Some(2));
        rsx! {
            RadioGroup { value: if SECOND() { second } else { first }, aria_label: "Pick",
                Radio { value: 1, label: "One" }
                Radio { value: 2, label: "Two" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let checked = |dom: &VirtualDom| {
        let html = dioxus_ssr::render(dom);
        let one = html.find(">One<").expect("one");
        let two = html.find(">Two<").expect("two");
        let at = html.find("checked").expect("a checked radio");
        if at < one {
            1
        } else if at < two {
            2
        } else {
            0
        }
    };
    assert_eq!(checked(&dom), 1);
    dom.in_runtime(|| *SECOND.write() = true);
    // The group re-renders first and updates its context, which marks the
    // radios dirty for the pass after.
    for _ in 0..2 {
        dom.process_events();
        dom.render_immediate(&mut dioxus::core::NoOpMutations);
    }
    assert_eq!(checked(&dom), 2);
}
