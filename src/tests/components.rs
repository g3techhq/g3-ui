//! Markup contracts for each component.
use super::*;

mod shell {
    use super::*;

    #[test]
    fn app_wrapper_applies_theme_mode_and_layout() {
        fn app() -> Element {
            rsx! {
                AppWrapper { mode: ComponentMode::Ios, theme: Theme::default_dark(), "Body" }
            }
        }
        let html = render(app);
        let shell = element_with_class(&html, "g3-app-shell");
        assert!(shell.contains("data-g3-mode=\"ios\""));
        assert!(shell.contains("--g3-color-bg: #111827;"));
        assert!(shell.contains("color-scheme: dark;"));
    }

    #[cfg(feature = "transitions")]
    #[test]
    fn tab_layout_is_the_page_frame_for_route_transitions() {
        // So a phone's bottom bar stays under a rising sheet: see
        // g3-route-transitions' ROUTE_TRANSITION_PAGE_FRAME_CLASS.
        fn app() -> Element {
            rsx! {
                TabLayout { route_transition_base: false, "Body" }
            }
        }
        let html = render(app);
        let layout = element_with_class(&html, "g3-tab-layout");
        assert!(layout.contains(g3_route_transitions::ROUTE_TRANSITION_PAGE_FRAME_CLASS));
        assert!(!layout.contains("route-transition-base-region"));
    }

    #[test]
    fn app_wrapper_without_layout_has_no_shell_class() {
        fn app() -> Element {
            rsx! {
                AppWrapper { layout: false, text_selection: false, "Body" }
            }
        }
        let html = render(app);
        assert!(!html.contains("g3-app-shell"));
        assert!(html.contains("g3-no-select"));
    }

    #[test]
    fn theme_provider_applies_its_theme_without_a_box() {
        fn app() -> Element {
            rsx! {
                ThemeProvider { theme: Theme::default_light().with_accent("#22c55e"),
                    Spinner {}
                }
            }
        }
        let html = render(app);
        let provider = element_with_class(&html, "g3-theme-provider");
        assert!(provider.contains("--g3-color-accent: #22c55e;"));
    }

    #[test]
    fn nested_providers_inherit_unset_values() {
        fn app() -> Element {
            rsx! {
                ThemeProvider { mode: ComponentMode::Ios, theme: Theme::default_dark(),
                    AppWrapper { "Inner" }
                }
            }
        }
        let html = render(app);
        let shell = element_with_class(&html, "g3-app-shell");
        assert!(shell.contains("data-g3-mode=\"ios\""));
        assert!(shell.contains("color-scheme: dark;"));
    }

    #[test]
    fn strings_reach_components() {
        fn app() -> Element {
            rsx! {
                ThemeProvider {
                    strings: Strings {
                        loading: "Chargement".into(),
                        ..Strings::default()
                    },
                    Spinner {}
                }
            }
        }
        assert!(render(app).contains("Chargement"));
    }

    #[test]
    fn strings_from_a_lookup_reach_components() {
        fn app() -> Element {
            let strings = Strings::from_lookup(|id| (id == "loading").then(|| "Chargement".into()));
            rsx! {
                ThemeProvider { strings, Spinner {} }
            }
        }
        assert!(render(app).contains("Chargement"));
    }

    #[test]
    fn a_lookup_is_asked_for_every_string_once_and_defaults_to_english() {
        let mut ids = Vec::new();
        let strings = Strings::from_lookup(|id| {
            ids.push(id.to_string());
            None
        });
        assert_eq!(strings, Strings::default());
        let unique: BTreeSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), ids.len(), "an id was asked for twice");
        // 43 texts, 12 + 12 month names, 7 + 7 weekday names, first_weekday.
        assert_eq!(ids.len(), 82);
        for id in ["cancel", "sheet_handle", "months_1", "months_short_12"] {
            assert!(unique.contains(&id.to_string()), "{id} was not asked for");
        }
        for id in ["weekdays_7", "weekdays_narrow_1", "first_weekday"] {
            assert!(unique.contains(&id.to_string()), "{id} was not asked for");
        }

        let all = Strings::from_lookup(|id| Some(format!("<{id}>")));
        assert_eq!(all.date_month_year, "<date_month_year>");
        assert_eq!(all.months[11], "<months_12>");
        assert_eq!(all.weekdays_narrow[0], "<weekdays_narrow_1>");
        assert_eq!(all.first_weekday, 0, "not a number keeps Sunday");
    }

    #[test]
    fn a_looked_up_first_weekday_must_be_a_weekday() {
        let first = |text: &'static str| {
            Strings::from_lookup(|id| (id == "first_weekday").then(|| text.into())).first_weekday
        };
        assert_eq!(first(" 1 "), 1);
        assert_eq!(first("6"), 6);
        assert_eq!(first("7"), 0);
        assert_eq!(first("Monday"), 0);
    }

    #[test]
    fn header_renders_slots_and_a_single_heading() {
        fn app() -> Element {
            rsx! {
                Header {
                    title: "Round",
                    start: rsx! {
                        BackButton {}
                    },
                    end: rsx! {
                        span { "End" }
                    },
                    toolbar: rsx! {
                        span { "Tools" }
                    },
                }
            }
        }
        let html = render(app);
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(element_with_class(&html, "g3-header").contains("g3-header-with-toolbar"));
        assert!(html.contains("g3-header-toolbar"));
        assert!(element_with_class(&html, "g3-back-button").contains("aria-label=\"Back\""));
        // The single-row layout is a grid on the bar, inside the header that
        // it measures: the row comes first in the bar, then the toolbar.
        let bar = html
            .find("class=\"g3-header-bar\"")
            .expect("missing header bar");
        let row = html.find("g3-header-row").expect("missing header row");
        let toolbar = html.find("g3-header-toolbar").expect("missing toolbar");
        assert!(html.find("g3-header-with-toolbar").unwrap() < bar);
        assert!(bar < row && row < toolbar);
    }

    #[test]
    fn a_header_toolbar_can_stack_rows() {
        fn app() -> Element {
            let tab = use_signal(|| 0_u8);
            let query = use_signal(String::new);
            rsx! {
                Header {
                    title: "Players",
                    toolbar: rsx! {
                        Toolbar { Searchbar { value: query, aria_label: "Search players" } }
                        Toolbar {
                            SegmentGroup { value: tab, aria_label: "Show",
                                SegmentButton { value: 0_u8, "All" }
                                SegmentButton { value: 1_u8, "Friends" }
                            }
                        }
                    },
                }
            }
        }
        let html = render(app);
        assert_eq!(html.matches("g3-header-toolbar-row").count(), 2);
        // A segment group in a row still takes its toolbar layout.
        assert!(html.contains("g3-segment-toolbar"));
        let css = include_str!("../../assets/g3-ui.css");
        assert!(css.contains(".g3-header-toolbar:has(> .g3-header-toolbar-row)"));
    }

    #[test]
    fn a_header_without_a_title_has_no_empty_heading() {
        fn app() -> Element {
            rsx! {
                Header { start: rsx! { span { "Brand" } } }
                Header { title: "  " }
            }
        }
        let html = render(app);
        assert!(!html.contains("<h1"));
        // The column stays, so the slots keep their places.
        assert_eq!(
            html.matches("<div class=\"g3-header-title\"></div>")
                .count(),
            2
        );
    }

    #[test]
    fn a_page_holds_its_header_and_content() {
        fn app() -> Element {
            rsx! {
                Page { class: "rounds",
                    Header { title: "Rounds" }
                    Content { "Rows" }
                }
            }
        }
        let html = render(app);
        let page = element_with_class(&html, "g3-page");
        assert!(page.contains("rounds"));
        let (header, content) = (
            html.find("g3-header").unwrap(),
            html.find("g3-content").unwrap(),
        );
        assert!(html.find("g3-page").unwrap() < header && header < content);
    }

    #[test]
    fn content_pads_by_default_and_reports_it() {
        fn app() -> Element {
            rsx! {
                Content { padding: false, footer_space: false, "Rows" }
            }
        }
        let html = render(app);
        assert!(element_with_class(&html, "g3-content-scroll").contains("data-padding=\"false\""));
        assert!(!html.contains("g3-content-footer-spacer"));
    }

    #[test]
    fn nav_items_are_links_or_buttons_not_tabs() {
        fn app() -> Element {
            rsx! {
                TabLayout {
                    AdaptiveNav { compact: AdaptiveNavCompact::Hidden,
                        NavItem { label: "Home", selected: true, badge: "2" }
                        NavItem {
                            label: "Docs",
                            href: "/docs",
                            group: NavItemGroup::Secondary,
                        }
                    }
                }
            }
        }
        let html = render(app);
        assert!(!html.contains("role=\"tab"));
        let nav = element_with_class(&html, "g3-nav");
        assert!(nav.starts_with("<nav"));
        assert!(nav.contains("aria-label=\"Primary navigation\""));
        assert!(nav.contains("g3-nav-compact-hidden"));
        let home = element_with_class(&html, "g3-nav-item");
        assert!(home.starts_with("<button"));
        assert!(home.contains("aria-current=\"page\""));
        let docs = element_with_class(&html, "g3-nav-item-secondary");
        assert!(docs.starts_with("<a"));
        assert!(docs.contains("href=\"/docs\""));
        assert!(html.contains("g3-nav-item-badge"));
    }

    #[test]
    fn nav_bar_and_rail_have_fixed_layouts() {
        fn app() -> Element {
            rsx! {
                NavBar {
                    NavItem { label: "A" }
                }
                NavRail {
                    NavItem { label: "B" }
                }
            }
        }
        let html = render(app);
        let bar = element_with_class(&html, "g3-nav");
        assert!(!bar.contains("g3-nav-rail") && !bar.contains("g3-nav-adaptive"));
        assert_eq!(html.matches("g3-nav-rail").count(), 1);
    }

    #[test]
    fn nav_rail_starts_expanded_and_can_start_collapsed() {
        fn expanded() -> Element {
            rsx! {
                NavRail {
                    NavItem { label: "A" }
                }
            }
        }
        fn collapsed() -> Element {
            rsx! {
                NavRail { default_expanded: false,
                    NavItem { label: "A" }
                }
            }
        }
        fn fixed() -> Element {
            rsx! {
                NavRail { collapsible: false,
                    NavItem { label: "A" }
                }
            }
        }
        let html = render(expanded);
        assert!(html.contains("g3-nav-expanded"));
        assert!(html.contains("Collapse navigation"));
        let html = render(collapsed);
        assert!(!html.contains("g3-nav-expanded"));
        assert!(html.contains("Expand navigation"));
        assert!(!render(fixed).contains("g3-nav-toggle"));
    }

    #[test]
    fn a_hover_rail_is_icons_only_with_no_toggle() {
        fn app() -> Element {
            rsx! {
                NavRail { behavior: NavRailBehavior::Hover, default_expanded: true,
                    NavItem { label: "A" }
                }
                AdaptiveNav { behavior: NavRailBehavior::Hover,
                    NavItem { label: "B" }
                }
            }
        }
        let html = render(app);
        assert_eq!(html.matches("g3-nav-hover").count(), 2);
        assert!(!html.contains("g3-nav-expanded"));
        assert!(!html.contains("g3-nav-toggle"));
    }

    #[test]
    fn a_toggled_rail_starts_from_the_remembered_state() {
        use crate::components::nav::REMEMBERED_EXPANDED;
        fn remembered() -> Element {
            use_hook(|| *REMEMBERED_EXPANDED.write() = Some(false));
            rsx! {
                NavRail {
                    NavItem { label: "A" }
                }
            }
        }
        fn forgetful() -> Element {
            use_hook(|| *REMEMBERED_EXPANDED.write() = Some(false));
            rsx! {
                NavRail { remember: false,
                    NavItem { label: "A" }
                }
            }
        }
        fn owned() -> Element {
            use_hook(|| *REMEMBERED_EXPANDED.write() = Some(false));
            let expanded = use_signal(|| true);
            rsx! {
                NavRail { expanded,
                    NavItem { label: "A" }
                }
            }
        }
        // A new route's rail opens as the last one was left.
        assert!(!render(remembered).contains("g3-nav-expanded"));
        assert!(render(forgetful).contains("g3-nav-expanded"));
        assert!(render(owned).contains("g3-nav-expanded"));
    }

    #[test]
    fn spinner_announces_its_label_once() {
        fn app() -> Element {
            rsx! {
                Spinner { size: SpinnerSize::Sm, label: "Loading rounds" }
            }
        }
        let html = render(app);
        assert_eq!(html.matches("Loading rounds").count(), 1);
        assert!(html.contains("role=\"status\""));
        assert!(html.contains("g3-spinner-sm"));
    }

    #[test]
    fn the_shell_size_starts_compact_and_is_readable_anywhere() {
        fn app() -> Element {
            let outside = use_shell_size();
            rsx! {
                AppWrapper {
                    Card { title: "Inside", "{use_shell_size().is_wide()}" }
                }
                Card { title: "Outside", "{outside.is_compact()}" }
            }
        }
        // Nothing has been measured in a server render, and a shell may not
        // exist at all, so both answer with the narrow layout rather than
        // guessing at a width.
        let html = render(app);
        assert!(html.contains(">false<"));
        assert!(html.contains(">true<"));
        assert!(ShellSize::default().is_compact());
    }

    #[test]
    fn ids_survive_the_render_that_hydrates_a_server_rendered_page() {
        fn app() -> Element {
            rsx! {
                Shelf { title: "Rounds", div { "One" } }
                Refresher { refreshing: false, on_refresh: |_| {} }
                InfiniteScroll { loading: false, complete: false, on_load: |_| {} }
            }
        }
        // Hydration keeps the markup the server sent, so the client's render
        // of the same tree has to name the same elements. When it does not,
        // every script that looks its element up by id finds nothing: a shelf
        // stops dragging and an infinite scroll never arms.
        let server = crate::tests::ids_in(&render(app));
        let client = crate::tests::ids_in(&render(app));
        assert!(!server.is_empty());
        assert_eq!(server, client);
    }

    #[test]
    fn infinite_scroll_exposes_loading_state_to_its_observer() {
        fn app() -> Element {
            rsx! {
                InfiniteScroll { loading: true, complete: false, on_load: |_| {} }
            }
        }
        let html = render(app);
        let sentinel = element_with_class(&html, "g3-infinite-scroll");
        assert!(sentinel.contains("data-loading=\"true\""));
        assert!(sentinel.contains("data-complete=\"false\""));
        assert!(sentinel.contains("aria-busy=\"true\""));
    }
}

#[test]
fn accessibility_names_and_states_hold() {
    fn app() -> Element {
        rsx! {
            Button { loading: true, "Save" }
            Button { href: "https://example.com", new_tab: true, "Docs" }
            Badge { aria_label: "3 unread", "3" }
            NavBar {
                NavItem { label: "Inbox", badge: "3", href: "/inbox" }
            }
            List {
                SwipeItem {
                    end_actions: rsx! { SwipeAction { "Delete" } },
                    Item { label: "Round" }
                }
            }
        }
    }
    let html = render(app);
    // A loading button keeps focus: busy, not disabled, and its spinner is
    // not a second live region.
    let busy = element_with_class(&html, "g3-btn-loading");
    assert!(busy.contains("aria-disabled=\"true\""));
    assert!(!busy.contains(" disabled=\"true\"") && !busy.contains(" disabled "));
    assert!(!html.contains("role=\"status\""));
    assert!(html.contains("(opens in a new tab)"));
    assert!(html.contains(">3 unread</span>"));
    assert!(element_with_class(&html, "g3-nav-item").contains("aria-label=\"Inbox, 3\""));
    // The swipe row is the list item; the row inside it is not another.
    assert_eq!(html.matches("role=\"listitem\"").count(), 1);
    assert!(element_with_class(&html, "g3-swipe-item").contains("role=\"listitem\""));
}
