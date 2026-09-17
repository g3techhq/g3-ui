//! Markup contracts for content and disclosure components.
use super::*;

#[test]
fn a_tappable_card_names_its_action_with_the_title() {
    fn app() -> Element {
        rsx! {
            Card { title: "Four-ball", subtitle: "8:10", onclick: |_| {}, selected: true,
                end: rsx! { InfoButton {} },
                "Body"
            }
            Card { aria_label: "Untitled", href: "/x", "Body" }
            Card { title: "Static", "Body" }
        }
    }
    let html = render(app);
    let action = element_with_class(&html, "g3-card-action");
    assert!(action.starts_with("<button"));
    assert!(action.contains("aria-pressed=\"true\""));
    assert!(
        html.contains("g3-card-action\" type=\"button\" aria-pressed=\"true\">Four-ball</button>")
            || html.contains(">Four-ball</button>")
    );
    assert!(html.contains("aria-label=\"Untitled\""));
    assert!(html.contains("href=\"/x\""));
    assert_eq!(html.matches("g3-card-interactive").count(), 2);
    // A control inside the card stays a separate button, not nested.
    assert_eq!(html.matches("<button").count(), 2);
}

#[test]
fn items_pick_their_element_from_their_props() {
    fn app() -> Element {
        rsx! {
            List { lines: ListLines::Full, aria_label: "Settings",
                ListHeader { "Account" }
                Item { label: "Static", metadata: "1" }
                Item { label: "Link", href: "/profile" }
                Item { label: "Check", checked: true, onclick: |_| {} }
                Item { label: "Current", selected: true, onclick: |_| {} }
            }
        }
    }
    let html = render(app);
    let list = element_with_class(&html, "g3-list");
    assert!(list.contains("role=\"list\""));
    assert!(list.contains("data-lines=\"full\""));
    assert_eq!(html.matches("role=\"listitem\"").count(), 5);
    assert!(html.contains("<h3 class=\"g3-list-header\">Account</h3>"));
    assert!(html.contains("href=\"/profile\""));
    assert!(html.contains("role=\"checkbox\""));
    assert!(html.contains("aria-checked=\"true\""));
    assert!(html.contains("aria-current=\"true\""));
}

#[test]
fn swipe_rows_keep_closed_actions_out_of_reach() {
    fn app() -> Element {
        rsx! {
            SwipeItem {
                end_actions: rsx! { SwipeAction { color: Color::Danger, "Delete" } },
                Item { label: "Round" }
            }
        }
    }
    let html = render(app);
    let actions = element_with_class(&html, "g3-swipe-actions");
    assert!(actions.contains("aria-hidden=\"true\""));
    assert!(actions.contains("inert"));
    let toggle = element_with_class(&html, "g3-swipe-toggle");
    assert!(toggle.contains("aria-expanded=\"false\""));
    assert!(html.contains("Show actions"));
    assert!(html.contains("g3-swipe-action-danger"));
}

#[test]
fn accordion_items_link_headers_and_panels_by_unique_ids() {
    fn app() -> Element {
        let open = use_signal(|| vec!["a b"]);
        rsx! {
            AccordionGroup { value: open,
                AccordionItem { value: "a b", label: "First", "One" }
                AccordionItem { value: "a-b", label: "Second", "Two" }
            }
        }
    }
    let html = render(app);
    let all_ids = ids(&html);
    let unique: std::collections::BTreeSet<_> = all_ids.iter().collect();
    assert_eq!(all_ids.len(), unique.len());
    assert_eq!(html.matches("aria-expanded=\"true\"").count(), 1);
    assert_eq!(
        html.matches("<h3 class=\"g3-accordion-heading\"").count(),
        2
    );
    assert_eq!(html.matches("role=\"region\"").count(), 2);
}

#[test]
fn segments_are_radio_groups_with_a_roving_tab_stop() {
    fn app() -> Element {
        let value = use_signal(|| 'b');
        rsx! {
            SegmentGroup { value, aria_label: "View",
                SegmentButton { value: 'a', "A" }
                SegmentButton { value: 'b', "B" }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("role=\"radiogroup\""));
    assert_eq!(html.matches("role=\"radio\"").count(), 2);
    assert_eq!(html.matches("tabindex=\"0\"").count(), 1);
    assert!(
        html.contains("aria-checked=\"true\" tabindex=\"0\"")
            || html.matches("aria-checked=\"true\"").count() == 1
    );
}

#[test]
fn tabs_link_each_tab_to_its_panel() {
    fn app() -> Element {
        let tab = use_signal(|| 1_u8);
        rsx! {
            Tabs { value: tab,
                TabList { aria_label: "Round",
                    Tab { value: 1_u8, "Scores" }
                    Tab { value: 2_u8, "Notes" }
                }
                TabPanel { value: 1_u8, "Score panel" }
                TabPanel { value: 2_u8, "Note panel" }
            }
        }
    }
    let html = render(app);
    assert!(html.contains("role=\"tablist\""));
    assert!(!html.contains("Note panel"));
    let panel = element_with_class(&html, "g3-tab-panel");
    let panel_id = panel
        .split("id=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap();
    let tab = html
        .split("<button")
        .find(|chunk| chunk.contains("aria-selected=\"true\""))
        .unwrap();
    assert!(tab.contains(&format!("aria-controls=\"{panel_id}\"")));
}

#[test]
fn avatars_are_named_once() {
    fn app() -> Element {
        rsx! {
            Avatar { name: "Alex Morgan", src: "/a.png" }
            Avatar { name: "Grace Park" }
            Avatar { name: "" }
        }
    }
    let html = render(app);
    assert!(html.contains("alt=\"Alex Morgan\""));
    assert_eq!(html.matches("role=\"img\"").count(), 1);
    assert!(html.contains("aria-label=\"Grace Park\""));
    assert!(html.contains(">GP</span>"));
}

#[test]
fn chips_are_buttons_only_when_pressable() {
    fn app() -> Element {
        rsx! {
            Chip { "Tag" }
            Chip { selected: true, onclick: |_| {}, "Filter" }
        }
    }
    let html = render(app);
    assert!(html.contains("<span class=\"g3-chip g3-chip-static\""));
    assert_eq!(html.matches("<button").count(), 1);
    assert!(html.contains("aria-pressed=\"true\""));
}

#[test]
fn progress_reports_values_only_when_known() {
    fn app() -> Element {
        rsx! {
            Progress { value: 9.0, max: 18.0, label: "Holes" }
            Progress { label: "Loading" }
        }
    }
    let html = render(app);
    assert_eq!(html.matches("aria-valuenow=").count(), 1);
    assert!(html.contains("aria-valuenow=\"9\""));
    assert!(html.contains("--g3-progress-value: 50%"));
    assert!(html.contains("g3-progress-indeterminate"));
}

#[test]
fn text_variants_choose_their_element() {
    fn app() -> Element {
        rsx! {
            Text { variant: TextVariant::Title, "T" }
            Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "C" }
            Text { variant: TextVariant::Label, color: Color::Danger, "L" }
            Divider { orientation: DividerOrientation::Vertical }
            Skeleton { width: "40%" }
        }
    }
    let html = render(app);
    assert!(html.contains("<h2 class=\"g3-text g3-text-title\">T</h2>"));
    assert!(html.contains("<p class=\"g3-text g3-text-caption g3-text-secondary\">C</p>"));
    assert!(html.contains("g3-text-danger"));
    assert!(html.contains("aria-orientation=\"vertical\""));
    assert!(html.contains("width: 40%;"));
}

#[test]
fn layout_helpers_render_their_settings() {
    fn app() -> Element {
        rsx! {
            Stack { horizontal: true, gap: Space::Sm, justify: StackJustify::Between, "a" }
            Grid { columns: GridColumns::Count(3), "b" }
            Tooltip { label: "Tip", button { "Trigger" } }
        }
    }
    let html = render(app);
    assert!(html.contains("g3-stack-row"));
    assert!(html.contains("data-justify=\"between\""));
    assert!(html.contains("repeat(3, minmax(0, 1fr))"));
    assert!(html.contains("role=\"tooltip\""));
}

#[test]
fn every_gallery_entry_is_named_and_described() {
    let descriptors = crate::component_descriptors();
    let names: std::collections::BTreeSet<_> = descriptors.iter().map(|d| d.name).collect();
    assert_eq!(names.len(), descriptors.len(), "duplicate gallery names");
    for descriptor in descriptors {
        assert!(
            !descriptor.description.is_empty(),
            "{} has no description",
            descriptor.name
        );
    }
}
