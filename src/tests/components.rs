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
