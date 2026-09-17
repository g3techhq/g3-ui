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
            Input { aria_label: "When", input_type: InputType::DateTime }
            TextArea { label: "Notes", rows: 4 }
        }
    }
    let html = render(app);
    assert!(html.contains("type=\"datetime-local\""));
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
            Searchbar { show_cancel: true }
        }
    }
    let html = render(app);
    assert!(element_with_class(&html, "g3-searchbar").contains("role=\"search\""));
    assert!(html.contains("type=\"search\""));
    assert!(html.contains("aria-label=\"Search\""));
    assert!(html.contains(">Cancel</button>"));
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
