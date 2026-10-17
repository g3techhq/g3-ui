//! Writable provider state must update CSS and readers together.
use super::*;
use dioxus::core::NoOpMutations;
use std::{cell::RefCell, rc::Rc};

type NamedThemes = Vec<(String, Signal<Theme>)>;

#[derive(Clone, Default)]
struct CapturedThemes(Rc<RefCell<NamedThemes>>);

#[component]
fn ThemeReader(label: String) -> Element {
    let theme = use_theme();
    let captures = use_context::<CapturedThemes>();
    use_hook(|| captures.0.borrow_mut().push((label.clone(), theme)));
    let accent = theme.read().accent.clone();
    rsx! { span { id: label, "{accent}" } }
}

fn pump(dom: &mut VirtualDom) {
    for _ in 0..8 {
        dom.process_events();
        dom.render_immediate(&mut NoOpMutations);
    }
}

fn captured(dom: &VirtualDom, name: &str) -> Signal<Theme> {
    dom.runtime().in_scope(ScopeId::ROOT, || {
        try_consume_context::<CapturedThemes>()
            .unwrap()
            .0
            .borrow()
            .iter()
            .find(|(label, _)| label == name)
            .unwrap()
            .1
    })
}

#[test]
fn theme_writes_update_css_survive_renders_and_follow_changed_props() {
    static ACCENT: GlobalSignal<String> = Signal::global(|| "#123456".into());
    static TICK: GlobalSignal<u32> = Signal::global(|| 0);
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedThemes::default()));
        rsx! { AppWrapper { theme: Theme::default_light().with_accent(ACCENT()),
            p { "Render {TICK}" }
            ThemeReader { label: "reader" }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let mut theme = captured(&dom, "reader");
    dom.runtime()
        .in_scope(ScopeId::ROOT, || theme.write().accent = "#16a34a".into());
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("--g3-color-accent: #16a34a;"), "{html}");
    assert!(html.contains("id=\"reader\">#16a34a</span>"));
    dom.runtime().in_scope(ScopeId::ROOT, || *TICK.write() += 1);
    pump(&mut dom);
    assert_eq!(theme.peek().accent, "#16a34a");
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *ACCENT.write() = "#abcdef".into());
    pump(&mut dom);
    assert_eq!(theme.peek().accent, "#abcdef");
    assert!(dioxus_ssr::render(&dom).contains("--g3-color-accent: #abcdef;"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || theme.set(Theme::default_dark()));
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("color-scheme: dark;"));
    assert!(html.contains("--g3-color-bg: #111827;"));
}

#[test]
fn inherited_theme_handles_are_shared_and_explicit_overrides_are_isolated() {
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedThemes::default()));
        rsx! { AppWrapper { theme: Theme::default_light().with_accent("#123456"),
            ThemeReader { label: "root" }
            ThemeProvider { mode: ComponentMode::Ios, ThemeReader { label: "inherited" } }
            ThemeProvider { theme: Theme::default_dark().with_accent("#facc15"),
                ThemeReader { label: "local" }
            }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    let root = captured(&dom, "root");
    let mut inherited = captured(&dom, "inherited");
    let mut local = captured(&dom, "local");
    assert_eq!(root, inherited);
    assert_ne!(root, local);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        inherited.write().accent = "#16a34a".into()
    });
    pump(&mut dom);
    assert_eq!(root.peek().accent, "#16a34a");
    assert_eq!(local.peek().accent, "#facc15");
    dom.runtime()
        .in_scope(ScopeId::ROOT, || local.write().accent = "#f97316".into());
    pump(&mut dom);
    assert_eq!(root.peek().accent, "#16a34a");
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("--g3-color-accent: #16a34a;"));
    assert!(html.contains("--g3-color-accent: #f97316;"));
    assert!(html.contains("id=\"local\">#f97316</span>"));
}

#[test]
fn adding_and_removing_an_override_updates_existing_readers() {
    static OVERRIDE: GlobalSignal<bool> = Signal::global(|| true);
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedThemes::default()));
        rsx! { AppWrapper { theme: Theme::default_light().with_accent("#123456"),
            ThemeProvider { theme: OVERRIDE().then(|| Theme::default_dark().with_accent("#facc15")),
                ThemeReader { label: "reader" }
            }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">#facc15</span>"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *OVERRIDE.write() = false);
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">#123456</span>"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *OVERRIDE.write() = true);
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">#facc15</span>"));
}

#[test]
fn theme_hook_has_writable_local_state_outside_a_provider() {
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedThemes::default()));
        rsx! { ThemeReader { label: "reader" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let mut theme = captured(&dom, "reader");
    dom.runtime()
        .in_scope(ScopeId::ROOT, || theme.write().accent = "#16a34a".into());
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">#16a34a</span>"));
}

#[test]
fn default_app_wrapper_owns_writable_theme_without_app_signals() {
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedThemes::default()));
        rsx! { AppWrapper { ThemeReader { label: "reader" } } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let mut theme = captured(&dom, "reader");
    dom.runtime()
        .in_scope(ScopeId::ROOT, || theme.write().accent = "#16a34a".into());
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("--g3-color-accent: #16a34a;"));
    assert!(html.contains("id=\"reader\">#16a34a</span>"));
}

type NamedAppearance = Vec<(String, Signal<ComponentMode>, Signal<Strings>)>;

#[derive(Clone, Default)]
struct CapturedAppearance(Rc<RefCell<NamedAppearance>>);

#[component]
fn AppearanceReader(label: String) -> Element {
    let mode = use_mode();
    let strings = use_strings();
    let captures = use_context::<CapturedAppearance>();
    use_hook(|| captures.0.borrow_mut().push((label.clone(), mode, strings)));
    let cancel = strings.read().cancel.clone();
    rsx! {
        span { id: label, "{mode():?}: {cancel}" }
        Button { "Inherited mode" }
        Button { mode: ComponentMode::Md, "Explicit Material" }
    }
}

fn appearance(dom: &VirtualDom, name: &str) -> (Signal<ComponentMode>, Signal<Strings>) {
    dom.runtime().in_scope(ScopeId::ROOT, || {
        let captures = try_consume_context::<CapturedAppearance>().unwrap();
        let values = captures.0.borrow();
        let (_, mode, strings) = values.iter().find(|(label, _, _)| label == name).unwrap();
        (*mode, *strings)
    })
}

#[test]
fn mode_and_strings_writes_update_readers_survive_renders_and_follow_changed_props() {
    static MODE: GlobalSignal<ComponentMode> = Signal::global(|| ComponentMode::Md);
    static CANCEL: GlobalSignal<String> = Signal::global(|| "Cancel".into());
    static TICK: GlobalSignal<u32> = Signal::global(|| 0);
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedAppearance::default()));
        rsx! { AppWrapper { mode: MODE(), strings: Strings { cancel: CANCEL(), ..Strings::default() },
            p { "Render {TICK}" }
            AppearanceReader { label: "reader" }
            Searchbar {}
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let (mut mode, mut strings) = appearance(&dom, "reader");
    // Writes before the initial provider effect must also survive.
    dom.runtime().in_scope(ScopeId::ROOT, || {
        *mode.write() = ComponentMode::Ios;
        let mut labels = strings.write();
        labels.cancel = "Cancelar".into();
        labels.search = "Buscar".into();
    });
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(
        html.contains("id=\"reader\">Ios: Cancelar</span>"),
        "{html}"
    );
    assert!(html.contains("data-g3-mode=\"ios\""));
    assert!(html.contains("g3-btn-ios"));
    assert!(html.contains("g3-btn-md")); // Explicit component mode still wins.
    assert!(html.contains("placeholder=\"Buscar\""));
    dom.runtime().in_scope(ScopeId::ROOT, || *TICK.write() += 1);
    pump(&mut dom);
    assert_eq!(*mode.peek(), ComponentMode::Ios);
    assert_eq!(strings.peek().cancel, "Cancelar");
    dom.runtime().in_scope(ScopeId::ROOT, || {
        *MODE.write() = ComponentMode::Ios;
        *CANCEL.write() = "Dismiss".into();
    });
    pump(&mut dom);
    // Changing props is observed even when the new mode equals current state.
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *MODE.write() = ComponentMode::Md);
    pump(&mut dom);
    assert_eq!(*mode.peek(), ComponentMode::Md);
    assert_eq!(strings.peek().cancel, "Dismiss");
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">Md: Dismiss</span>"));
}

#[test]
fn mode_and_strings_inherit_shared_handles_and_isolate_explicit_overrides() {
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedAppearance::default()));
        rsx! { AppWrapper {
            AppearanceReader { label: "root" }
            ThemeProvider { theme: Theme::default_dark(), AppearanceReader { label: "inherited" } }
            ThemeProvider { mode: ComponentMode::Md, strings: Strings::default(),
                AppearanceReader { label: "local" }
            }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let root = appearance(&dom, "root");
    let (mut mode, mut strings) = appearance(&dom, "inherited");
    let (mut local_mode, mut local_strings) = appearance(&dom, "local");
    assert_eq!(root, (mode, strings));
    assert_ne!(mode, local_mode);
    assert_ne!(strings, local_strings);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        mode.set(ComponentMode::Ios);
        strings.write().cancel = "Cancelar".into();
        local_strings.write().cancel = "Dismiss".into();
    });
    pump(&mut dom);
    assert_eq!(*local_mode.peek(), ComponentMode::Md);
    dom.runtime()
        .in_scope(ScopeId::ROOT, || local_mode.set(ComponentMode::Ios));
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("id=\"root\">Ios: Cancelar</span>"));
    assert!(html.contains("id=\"inherited\">Ios: Cancelar</span>"));
    assert!(html.contains("id=\"local\">Ios: Dismiss</span>"));
}

#[test]
fn mode_and_strings_readers_follow_added_and_removed_overrides() {
    static OVERRIDE: GlobalSignal<bool> = Signal::global(|| false);
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedAppearance::default()));
        rsx! { AppWrapper { mode: ComponentMode::Ios,
            strings: Strings { cancel: "Parent".into(), ..Strings::default() },
            ThemeProvider {
                mode: OVERRIDE().then_some(ComponentMode::Md),
                strings: OVERRIDE().then(|| Strings { cancel: "Local".into(), ..Strings::default() }),
                AppearanceReader { label: "reader" }
            }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">Ios: Parent</span>"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *OVERRIDE.write() = true);
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">Md: Local</span>"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *OVERRIDE.write() = false);
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">Ios: Parent</span>"));
}

#[test]
fn mode_and_strings_have_writable_local_defaults_outside_a_provider() {
    fn app() -> Element {
        use_hook(|| dioxus::core::provide_root_context(CapturedAppearance::default()));
        rsx! { AppearanceReader { label: "reader" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let (mut mode, mut strings) = appearance(&dom, "reader");
    dom.runtime().in_scope(ScopeId::ROOT, || {
        mode.set(ComponentMode::Ios);
        strings.write().cancel = "Cancelar".into();
    });
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("id=\"reader\">Ios: Cancelar</span>"));
}

#[test]
fn the_global_mode_is_shared_by_every_thread() {
    // A server sets it once in `main` and renders on worker threads.
    crate::set_mode(ComponentMode::Ios);
    let seen = std::thread::spawn(crate::get_mode).join().unwrap();
    assert_eq!(seen, ComponentMode::Ios);
}
