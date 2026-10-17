use super::*;

#[test]
fn adaptive_switches_remount_variants_but_not_a_reused_fallback() {
    use dioxus::core::NoOpMutations;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static SIZE: GlobalSignal<LayoutSize> = Signal::global(LayoutSize::default);
    static MOUNTS: AtomicUsize = AtomicUsize::new(0);
    #[component]
    fn Counted() -> Element {
        let mount = use_hook(|| MOUNTS.fetch_add(1, Ordering::SeqCst) + 1);
        rsx! { "mounted-{mount}" }
    }
    fn app() -> Element {
        let mut size = use_context_provider(|| Signal::new(LayoutSize::Compact));
        use_effect(move || size.set(SIZE()));
        rsx! { AdaptiveLayout {
            compact: rsx! { Counted {} },
            expanded: rsx! { Counted {} },
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    assert!(dioxus_ssr::render(&dom).contains("mounted-1"));
    for (size, mount) in [
        (LayoutSize::Medium, 1),
        (LayoutSize::Expanded, 2),
        (LayoutSize::Compact, 3),
    ] {
        dom.runtime()
            .in_scope(ScopeId::ROOT, || *SIZE.write() = size);
        for _ in 0..5 {
            dom.process_events();
            dom.render_immediate(&mut NoOpMutations);
        }
        assert!(dioxus_ssr::render(&dom).contains(&format!("mounted-{mount}")));
    }
}

#[test]
fn adaptive_slots_select_only_one_tree_and_fall_back_downward() {
    for (size, expected) in [
        (LayoutSize::Compact, "phone-only"),
        (LayoutSize::Medium, "tablet-only"),
        (LayoutSize::Expanded, "desktop-only"),
    ] {
        let mut dom = VirtualDom::new_with_props(
            |size: LayoutSize| {
                let size = use_signal(|| size);
                use_context_provider(|| size);
                rsx! { AdaptiveLayout {
                    compact: rsx! { "phone-only" },
                    medium: rsx! { "tablet-only" },
                    expanded: rsx! { "desktop-only" },
                } }
            },
            size,
        );
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(expected));
        for other in ["phone-only", "tablet-only", "desktop-only"] {
            assert_eq!(html.contains(other), other == expected);
        }
    }
    fn app() -> Element {
        let size = use_signal(|| LayoutSize::Expanded);
        use_context_provider(|| size);
        rsx! { AdaptiveLayout {
            compact: rsx! { "fallback-phone" },
            medium: rsx! { "fallback-tablet" },
        } }
    }
    assert!(render(app).contains("fallback-tablet"));
}

#[test]
fn adaptive_server_default_is_compact_and_scope_does_not_inherit_outer_width() {
    fn app() -> Element {
        let size = use_signal(|| LayoutSize::Expanded);
        use_context_provider(|| size);
        rsx! { LayoutScope { class: "local-content",
            AdaptiveLayout {
                compact: rsx! { "initial-compact" },
                expanded: rsx! { "outer-expanded" },
            }
        } }
    }
    let html = render(app);
    assert!(html.contains("initial-compact"));
    assert!(!html.contains("outer-expanded"));
    assert!(element_with_class(&html, "g3-layout-scope").contains("local-content"));
}

#[test]
fn page_slots_have_explicit_reading_order_and_no_duplicate_controls() {
    fn app() -> Element {
        rsx! { PageLayout {
            order: PageLayoutOrder::StartEndMain,
            start: rsx! { button { "hero-control" } },
            end: rsx! { button { "rating-control" } },
            button { "main-control" }
        } }
    }
    let html = render(app);
    assert!(html.find("hero-control") < html.find("rating-control"));
    assert!(html.find("rating-control") < html.find("main-control"));
    assert_eq!(html.matches("<button").count(), 3);
    assert_eq!(html.matches("g3-layout-scope").count(), 1);
}

#[test]
fn grid_exposes_additive_intermediate_columns() {
    fn app() -> Element {
        rsx! { Grid {
            columns: GridColumns::Count(1),
            medium_columns: GridColumns::Count(2),
            expanded_columns: GridColumns::Count(3),
            "items"
        } }
    }
    let html = render(app);
    assert!(html.contains("--g3-grid-columns-medium: repeat(2"));
    assert!(html.contains("--g3-grid-columns-expanded: repeat(3"));
}
