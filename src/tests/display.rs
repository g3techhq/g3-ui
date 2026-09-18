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
    assert!(html.contains("<h2 class=\"g3-list-header\">Account</h2>"));
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
        html.matches("<h2 class=\"g3-accordion-heading\"").count(),
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

#[test]
fn card_and_list_variants_set_their_surface() {
    fn app() -> Element {
        rsx! {
            Card { title: "Raised", "a" }
            Card { variant: CardVariant::Flat, title: "Flat", "b" }
            Card { variant: CardVariant::Filled, title: "Filled", "c" }
            List { variant: ListVariant::Flat, Item { label: "Row" } }
        }
    }
    let html = render(app);
    assert_eq!(html.matches("g3-card-flat").count(), 1);
    assert_eq!(html.matches("g3-card-filled").count(), 1);
    let list = element_with_class(&html, "g3-list");
    assert!(list.contains("g3-list-grouped") && list.contains("g3-list-flat"));
}

#[test]
fn swipe_edges_keep_their_own_behaviour() {
    fn app() -> Element {
        rsx! {
            SwipeItem {
                start_behavior: SwipeBehavior::Activate,
                start_actions: rsx! { SwipeAction { "Archive" } },
                end_actions: rsx! { SwipeAction { "Pin" } SwipeAction { "Delete" } },
                Item { label: "Round" }
            }
        }
    }
    let html = render(app);
    let row = element_with_class(&html, "g3-swipe-item");
    assert!(row.contains("data-start-behavior=\"activate\""));
    assert!(row.contains("data-end-behavior=\"reveal\""));
}

#[test]
fn content_limits_width_and_hosts_a_refresher() {
    fn app() -> Element {
        rsx! {
            Content { width: ContentWidth::Readable, refreshing: true, on_refresh: |_| {}, "Page" }
        }
    }
    let html = render(app);
    assert!(element_with_class(&html, "g3-content-scroll").contains("data-width=\"readable\""));
    assert!(element_with_class(&html, "g3-refresher").contains("aria-busy=\"true\""));
}

#[test]
fn grids_carry_wide_settings() {
    fn app() -> Element {
        rsx! {
            Grid { columns: GridColumns::Count(1), wide_columns: GridColumns::Count(3), wide_gap: Space::Xl, "a" }
        }
    }
    let html = render(app);
    let grid = element_with_class(&html, "g3-grid");
    assert!(grid.contains("--g3-grid-columns-wide: repeat(3, minmax(0, 1fr));"));
    assert!(grid.contains("data-wide-gap=\"xl\""));
}

#[test]
fn an_empty_state_announces_only_a_failure() {
    fn empty() -> Element {
        rsx! {
            EmptyState { title: "No rounds yet", heading_level: 3, "Rounds you play show up here." }
        }
    }
    let html = render(empty);
    assert!(html.contains("g3-empty-state"));
    assert!(html.contains(r#"data-color="neutral""#));
    assert!(
        html.contains("<h3"),
        "the title is a heading at the level asked for"
    );
    assert!(
        !html.contains("role="),
        "an empty list is part of the page, not an event"
    );

    fn failed() -> Element {
        rsx! {
            EmptyState { title: "Couldn't load rounds", color: Color::Danger, "Try again." }
        }
    }
    let html = render(failed);
    assert!(html.contains(r#"role="alert""#), "a failure is announced");
    assert!(html.contains(r#"data-color="danger""#));
    assert!(
        !html.contains("g3-empty-state-icon"),
        "no icon wrapper when no icon is given"
    );
}

#[test]
fn a_shelf_is_a_named_focusable_group() {
    fn titled() -> Element {
        rsx! {
            Shelf { title: "Nearby courses", gap: Space::Sm,
                Card { title: "Pebble Creek", "18 holes" }
            }
        }
    }
    let html = render(titled);
    assert!(html.contains("g3-shelf-track"));
    assert!(html.contains(r#"role="group""#));
    assert!(html.contains(r#"tabindex="0""#), "a keyboard can scroll it");
    assert!(html.contains(r#"data-gap="sm""#));
    assert!(html.contains(r#"data-snap="true""#), "snaps by default");
    // The visible title names the row, so there is no separate label.
    assert!(html.contains("aria-labelledby="));
    assert!(!html.contains("aria-label=\""));

    fn untitled() -> Element {
        rsx! {
            Shelf { aria_label: "Filters", snap: false,
                Chip { "Nearby" }
            }
        }
    }
    let html = render(untitled);
    assert!(html.contains(r#"aria-label="Filters""#));
    assert!(!html.contains("aria-labelledby="));
    assert!(!html.contains("g3-shelf-header"), "no empty heading row");
    assert!(html.contains(r#"data-snap="false""#));
}

#[test]
fn a_table_scrolls_in_a_named_focusable_frame() {
    fn captioned() -> Element {
        rsx! {
            Table { caption: "Front nine", sticky_first_column: true,
                tbody { tr { th { scope: "row", "Par" } td { "4" } } }
            }
        }
    }
    let html = render(captioned);
    let frame = element_with_class(&html, "g3-table-frame");
    assert!(frame.contains("role=\"region\""));
    assert!(
        frame.contains("tabindex=\"0\""),
        "a scrolling frame is reachable by keyboard"
    );
    assert!(
        frame.contains("aria-labelledby="),
        "the frame is named by the caption"
    );
    assert!(html.contains("<caption"));
    assert!(html.contains("data-sticky-first=\"true\""));
    assert!(!html.contains("data-fill"));

    fn unnamed_caption() -> Element {
        rsx! {
            Table { aria_label: "Results", fill: true,
                tbody { tr { td { "1" } } }
            }
        }
    }
    let html = render(unnamed_caption);
    assert!(!html.contains("<caption"));
    assert_eq!(html.matches("aria-label=\"Results\"").count(), 2);
    assert!(html.contains("data-fill=\"true\""));
    assert!(!html.contains("data-sticky-first"));
}

#[test]
fn a_labelled_divider_reads_its_label() {
    fn app() -> Element {
        rsx! {
            Divider { label: "or continue with email", spaced: true }
        }
    }
    let html = render(app);
    assert!(!html.contains("<hr"), "a separator's content goes unread");
    assert!(html.contains("g3-divider-labelled"));
    assert!(html.contains(">or continue with email</span>"));
    assert_eq!(html.matches("aria-hidden=\"true\"").count(), 2);
}

#[test]
fn a_row_with_a_control_at_its_end_does_not_nest_buttons() {
    fn app() -> Element {
        rsx! {
            List { aria_label: "People",
                Item {
                    label: "Alex",
                    onclick: |_| {},
                    end: rsx! { Button { "Follow" } },
                }
                Item { label: "Sam", onclick: |_| {} }
            }
        }
    }
    let html = render(app);
    assert!(
        element_with_class(&html, "g3-item-split").starts_with("<div"),
        "the row itself is not the button"
    );
    let split = &html[html.find("g3-item-split").expect("split row")..];
    let action = split
        .find("g3-item-action")
        .expect("the text is the action");
    let closes = split[action..]
        .find("</button>")
        .expect("action button closes")
        + action;
    let follow = split.find(">Follow<").expect("follow button");
    assert!(
        follow > closes,
        "the Follow button sits after the row action, not inside it"
    );
    // A row with nothing at its end stays one button.
    assert!(
        html.contains("g3-item g3-item-md g3-item-button") || html.contains("g3-item-button\"")
    );
    assert_eq!(html.matches("g3-item-split").count(), 1);
}
