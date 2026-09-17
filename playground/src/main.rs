//! g3-ui playground - component-owned demos rendered in responsive device contexts.
use dioxus::prelude::*;
use dioxus_code::{Code, CodeTheme, Language, SourceCode, Theme};
use g3_route_transitions::{
    Platform, RouteTransitions, animated_navigate, set_platform, use_browser_history_transitions,
};
use g3_ui::{
    AppWrapper, Button, ButtonFill, ButtonSize, ComponentMode, ComponentPlaygroundDemo, Header,
    Item, List, ListLines, NavigationDrawer, SegmentButton, SegmentGroup, Select, SelectOption,
    SideSheet, Theme as G3Theme, ThemeProvider, component_playground_demos,
};
use gloo_timers::future::TimeoutFuture;
use manganis::{AssetOptions, asset};

mod transition_showcase;
use transition_showcase::{
    TransitionAlbum, TransitionEpisode, TransitionLibrary, TransitionListen, TransitionProfile,
    TransitionQueue, TransitionRadio,
};
#[allow(dead_code)]
const PLAYGROUND_CSS: Asset = asset!(
    "/assets/playground.css",
    AssetOptions::css().with_static_head(true)
);

#[derive(Clone, Debug, PartialEq, Routable, RouteTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(Playground)]
        #[transition(layer = stack_root)]
        #[route("/")]
        Landing {},

        #[transition(layer = stack_root)]
        #[route("/components/:slug")]
        ComponentDemo { slug: String },

        #[transition(layer = stack_root)]
        #[route("/transitions")]
        TransitionListen {},

        #[transition(layer = stack_page)]
        #[route("/transitions/album")]
        TransitionAlbum {},

        #[transition(layer = sheet)]
        #[route("/transitions/queue")]
        TransitionQueue {},

        #[transition(history = replace, peers(group = showcase_tabs, order = tab))]
        #[route("/transitions/library/:tab")]
        TransitionLibrary { tab: u8 },

        #[transition(layer = stack_root)]
        #[route("/transitions/profile")]
        TransitionProfile {},

        #[transition(forward_to = TransitionEpisode)]
        #[route("/transitions/radio")]
        TransitionRadio {},

        #[transition(layer = stack_page)]
        #[route("/transitions/radio/episode")]
        TransitionEpisode {},

        #[transition(layer = stack_root)]
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaygroundTheme {
    Blue,
    Green,
    Night,
    Forest,
}
impl PlaygroundTheme {
    fn label(self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Green => "Green",
            Self::Night => "Night",
            Self::Forest => "Forest",
        }
    }
    fn all() -> [Self; 4] {
        [Self::Blue, Self::Green, Self::Night, Self::Forest]
    }
    /// Real `g3_ui::Theme` for this brand - fed straight into `AppWrapper`'s
    /// `theme` prop, the same way any consumer of the library themes its app.
    fn to_theme(self) -> G3Theme {
        match self {
            Self::Blue => G3Theme {
                accent: "#2563eb".into(),
                bg: "#eef5ff".into(),
                bg_secondary: "#dbeafe".into(),
                control: "#eaf2ff".into(),
                border: "rgba(37, 99, 235, 0.2)".into(),
                text_tertiary: "#496278".into(),
                text: "#102033".into(),
                text_secondary: "#53677f".into(),
                shadow: "rgba(30, 64, 175, 0.16)".into(),
                ..G3Theme::default_light()
            },
            Self::Green => G3Theme {
                accent: "#1f7a4d".into(),
                bg: "#edf7f0".into(),
                bg_secondary: "#dceee2".into(),
                control: "#e5f3ea".into(),
                border: "rgba(31, 122, 77, 0.24)".into(),
                text_tertiary: "#4d6657".into(),
                text: "#12251a".into(),
                text_secondary: "#52685c".into(),
                shadow: "rgba(24, 74, 47, 0.16)".into(),
                ..G3Theme::default_light()
            },
            Self::Night => G3Theme {
                accent: "#7dd3fc".into(),
                bg: "#07111f".into(),
                bg_secondary: "#0e1b2e".into(),
                card: "#111d2e".into(),
                surface: "#101a2b".into(),
                control: "#1b2a40".into(),
                border: "rgba(148, 163, 184, 0.24)".into(),
                text_tertiary: "#b8c7d8".into(),
                text: "#e5edf6".into(),
                text_secondary: "#9fb0c3".into(),
                shadow: "rgba(0, 0, 0, 0.54)".into(),
                ..G3Theme::default_dark()
            },
            Self::Forest => G3Theme {
                accent: "#34d399".into(),
                bg: "#050806".into(),
                bg_secondary: "#0b130d".into(),
                card: "#101811".into(),
                surface: "#0e1610".into(),
                control: "#18241b".into(),
                border: "rgba(134, 239, 172, 0.2)".into(),
                text_tertiary: "#b8d4bd".into(),
                text: "#e6f4e8".into(),
                text_secondary: "#9eb5a4".into(),
                shadow: "rgba(0, 0, 0, 0.62)".into(),
                ..G3Theme::default_dark()
            },
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlaygroundViewport {
    Mobile,
    Desktop,
}
#[derive(Clone, Copy)]
struct PlaygroundSettings {
    mode: Signal<ComponentMode>,
    viewport: Signal<PlaygroundViewport>,
    theme: Signal<PlaygroundTheme>,
}

impl PlaygroundViewport {
    fn as_str(self) -> &'static str {
        match self {
            Self::Mobile => "mobile",
            Self::Desktop => "desktop",
        }
    }
}
/// Width at which the playground chrome itself becomes a phone layout. Kept in
/// sync with the `max-width: 760px` block in `playground.css`.
const COMPACT_SHELL_QUERY: &str = "(max-width: 760px)";
const COMPACT_SHELL_SCRIPT: &str = r#"
const query = window.matchMedia("__QUERY__");
let last = null;
const publish = () => {
    if (query.matches === last) return;
    last = query.matches;
    dioxus.send(last);
};
publish();
query.addEventListener("change", publish);
window.addEventListener("resize", publish);
"#;
/// Answers [`COMPACT_SHELL_QUERY`] during the render that asks, before any
/// `eval` could round-trip. The drawer's resting state depends on the answer,
/// and a rail that is part of the wide layout has to be laid out with the first
/// paint rather than animated in over the top of it. `None` off the web, and if
/// the query cannot be evaluated - callers then wait for [`use_compact_shell`].
#[cfg(target_arch = "wasm32")]
fn initial_compact_shell() -> Option<bool> {
    web_sys::window()?
        .match_media(COMPACT_SHELL_QUERY)
        .ok()
        .flatten()
        .map(|query| query.matches())
}
#[cfg(not(target_arch = "wasm32"))]
fn initial_compact_shell() -> Option<bool> {
    None
}
/// Tracks whether the playground page - not the simulated device inside it -
/// is rendering at phone width. A persistent `Menu` rail has no room to reserve
/// on a phone, so the component drawer switches to `Overlay` there.
///
/// Seeded from [`initial_compact_shell`], then kept current by a media-query
/// listener. `None` only where neither can answer, which leaves callers to fall
/// back rather than guess a width.
fn use_compact_shell() -> Signal<Option<bool>> {
    let mut compact = use_signal(initial_compact_shell);
    use_future(move || async move {
        let script = COMPACT_SHELL_SCRIPT.replace("__QUERY__", COMPACT_SHELL_QUERY);
        let mut eval = document::eval(&script);
        while let Ok(matches) = eval.recv::<bool>().await {
            compact.set(Some(matches));
        }
    });
    compact
}
fn main() {
    g3_ui::init_auto_mode();
    dioxus::launch(App);
}
#[component]
fn App() -> Element {
    use_browser_history_transitions::<Route>();
    rsx! {
        Router::<Route> {}
    }
}
#[component]
fn Playground() -> Element {
    let demos = component_playground_demos();
    let mode = use_signal(|| ComponentMode::Md);
    let viewport = use_signal(|| PlaygroundViewport::Mobile);
    // Open from the first paint on a wide shell, where the drawer is a
    // persistent `Menu` that reserves its own space beside the page. Closed on
    // a phone, where it is a dismissible `Overlay` summoned from the header.
    let mut selector_open = use_signal(|| !initial_compact_shell().unwrap_or(true));
    let source_value = use_signal(Vec::<String>::new);
    let theme = use_signal(|| PlaygroundTheme::Blue);
    let mut playing = use_signal(|| false);
    let current_route: Route = use_route();
    let selected = selected_demo(&demos, &current_route);
    let showing_transitions = current_route.is_transition_showcase();
    let count = demos.len() + 1;
    let active_mode = mode();
    let active_viewport = viewport();
    let compact_shell = use_compact_shell();
    let is_compact = compact_shell().unwrap_or(false);
    set_platform(match active_mode {
        ComponentMode::Ios => Platform::Ios,
        ComponentMode::Md => Platform::Material,
    });
    // The wide-shell drawer is a persistent `Menu` that reserves its own space
    // beside the page, so it is part of the layout rather than something to
    // summon - it opens with the page. A phone has no room to reserve, so the
    // same drawer is a dismissible `Overlay` there and stays shut. This only
    // runs on a real measurement, and only re-runs when the breakpoint is
    // crossed, so toggling the drawer by hand still sticks at either width.
    use_effect(move || {
        if let Some(compact) = compact_shell() {
            selector_open.set(!compact);
        }
    });
    g3_ui::set_mode(active_mode);
    let page_title = if showing_transitions {
        "RouteTransitions".to_string()
    } else {
        selected.descriptor.name.to_string()
    };
    let page_description = if showing_transitions {
        transition_showcase::DESCRIPTION
    } else {
        selected.descriptor.description
    };
    let source_panels = if showing_transitions {
        vec![(
            "Transition showcase routes".to_string(),
            transition_showcase::SOURCE.to_string(),
        )]
    } else {
        source_panels(&selected)
    };
    let play_all = move |_| {
        if playing() {
            return;
        }
        playing.set(true);
        spawn(async move {
            play_transition_tour().await;
            playing.set(false);
        });
    };
    let theme_options = PlaygroundTheme::all()
        .iter()
        .copied()
        .map(|theme| SelectOption::new(theme, theme.label()))
        .collect::<Vec<_>>();
    provide_context(PlaygroundSettings {
        mode,
        viewport,
        theme,
    });
    let selector = rsx! {
        div { class: "playground-sidebar-header",
            h2 { "g3_ui" }
            div { class: "subtitle", "{count} demos" }
        }
        PlaygroundNav {
            demos: demos.clone(),
            current_route: current_route.clone(),
            selector_open,
            dismiss_on_select: is_compact,
        }
    };
    rsx! {
        AppWrapper {
            theme: theme().to_theme(),
            mode: active_mode,
            class: "playground-root",
            layout: false,
            route_transition_overlay: false,
            div { class: "playground-page",
                Header {
                    title: page_title,
                    mode: active_mode,
                    class: "playground-header",
                    start: rsx! {
                        Button {
                            fill: ButtonFill::Clear,
                            size: ButtonSize::Sm,
                            aria_label: "Toggle component menu",
                            aria_expanded: selector_open(),
                            class: "playground-menu-button",
                            onclick: move |_| selector_open.toggle(),
                            span { class: "playground-menu-icon", aria_hidden: "true",
                                span {}
                                span {}
                                span {}
                            }
                        }
                    },
                    end: rsx! {
                        Select {
                            value: theme,
                            aria_label: "Theme",
                            width: g3_ui::SelectWidth::Fit,
                            options: theme_options,
                        }
                    },
                    toolbar: rsx! {
                        div { class: "playground-header-toggles",
                            SegmentGroup { value: mode, aria_label: "Platform",
                                SegmentButton { value: ComponentMode::Md, "MD" }
                                SegmentButton { value: ComponentMode::Ios, "iOS" }
                            }
                            SegmentGroup { value: viewport, aria_label: "Viewport",
                                SegmentButton { value: PlaygroundViewport::Mobile, "Mobile" }
                                SegmentButton { value: PlaygroundViewport::Desktop, "Desktop" }
                            }
                        }
                    },
                }
                main { class: "playground-main",
                    div { class: "playground-intro",
                        p { class: "playground-description", "{page_description}" }
                        if showing_transitions {
                            Button {
                                disabled: playing(),
                                aria_label: "Play every route transition",
                                onclick: play_all,
                                if playing() { "Playing tour…" } else { "Play transition tour" }
                            }
                        }
                    }
                    div {
                        class: "playground-stage",
                        "data-playground-viewport": active_viewport.as_str(),
                        Outlet::<Route> {}
                    }
                    g3_ui::AccordionGroup { value: source_value, multiple: true, class: "source-panel",
                        for (title, code) in source_panels {
                            g3_ui::AccordionItem {
                                key: "{title}",
                                value: title.clone(),
                                label: title.clone(),
                                Code {
                                    src: SourceCode::new(Language::Rust, code),
                                    theme: CodeTheme::system(Theme::GITHUB_LIGHT, Theme::GITHUB_DARK),
                                }
                            }
                        }
                    }
                }
            }
            // A phone has no room to keep the menu beside the page, so it
            // becomes a dismissible sheet there.
            if is_compact {
                SideSheet {
                    open: selector_open,
                    aria_label: "Components",
                    class: "playground-selector-sheet",
                    {selector}
                }
            } else {
                NavigationDrawer {
                    open: selector_open,
                    aria_label: "Components",
                    class: "playground-selector-sheet",
                    {selector}
                }
            }
        }
    }
}

impl Route {
    fn is_transition_showcase(&self) -> bool {
        matches!(
            self,
            Self::TransitionListen {}
                | Self::TransitionAlbum {}
                | Self::TransitionQueue {}
                | Self::TransitionLibrary { .. }
                | Self::TransitionProfile {}
                | Self::TransitionRadio {}
                | Self::TransitionEpisode {}
        )
    }
}

fn demo_slug(name: &str) -> String {
    let mut slug = String::with_capacity(name.len());
    let mut separator_pending = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            if separator_pending && !slug.is_empty() {
                slug.push('-');
            }
            separator_pending = false;
            slug.push(ch.to_ascii_lowercase());
        } else {
            separator_pending = true;
        }
    }
    slug
}

fn selected_demo(demos: &[ComponentPlaygroundDemo], route: &Route) -> ComponentPlaygroundDemo {
    let requested_slug = match route {
        Route::ComponentDemo { slug } => Some(slug.as_str()),
        _ => None,
    };
    requested_slug
        .and_then(|slug| {
            demos
                .iter()
                .copied()
                .find(|demo| demo_slug(demo.descriptor.name) == slug)
        })
        .unwrap_or_else(|| demos[0])
}

fn active_playground_settings() -> (ComponentMode, PlaygroundTheme, PlaygroundViewport) {
    let settings = use_context::<PlaygroundSettings>();
    ((settings.mode)(), (settings.theme)(), (settings.viewport)())
}

#[component]
fn Landing() -> Element {
    let navigator = use_navigator();
    use_effect(move || {
        navigator.replace(Route::ComponentDemo {
            slug: "demo-app".to_string(),
        });
    });
    rsx! {}
}

#[component]
fn NotFound(segments: Vec<String>) -> Element {
    let _ = segments;
    let navigator = use_navigator();
    use_effect(move || {
        navigator.replace(Route::ComponentDemo {
            slug: "demo-app".to_string(),
        });
    });
    rsx! {}
}

#[component]
fn ComponentDemo(slug: String) -> Element {
    let demos = component_playground_demos();
    let demo = demos
        .iter()
        .copied()
        .find(|demo| demo_slug(demo.descriptor.name) == slug)
        .unwrap_or_else(|| demos[0]);
    let (mode, theme, viewport) = active_playground_settings();
    rsx! {
        PlaygroundViewportDemo {
            key: "{viewport.as_str()}-{mode.as_str()}-{theme.label()}-{demo.descriptor.name}",
            demo,
            mode,
            theme,
            viewport,
        }
    }
}

async fn pause_between_transitions() {
    TimeoutFuture::new(1_100).await;
}

async fn play_transition_tour() {
    animated_navigate(Route::TransitionListen {}).await;
    TimeoutFuture::new(1_400).await;
    for route in [
        Route::TransitionAlbum {},
        Route::TransitionListen {},
        Route::TransitionQueue {},
        Route::TransitionListen {},
        Route::TransitionProfile {},
        Route::TransitionListen {},
        Route::TransitionLibrary { tab: 0 },
        Route::TransitionLibrary { tab: 1 },
        Route::TransitionLibrary { tab: 2 },
        Route::TransitionLibrary { tab: 0 },
        Route::TransitionRadio {},
        Route::TransitionEpisode {},
        Route::TransitionRadio {},
        Route::TransitionListen {},
    ] {
        animated_navigate(route).await;
        pause_between_transitions().await;
    }
    TimeoutFuture::new(1_400).await;
}

#[component]
fn PlaygroundViewportDemo(
    demo: ComponentPlaygroundDemo,
    mode: ComponentMode,
    theme: PlaygroundTheme,
    viewport: PlaygroundViewport,
) -> Element {
    let label = match viewport {
        PlaygroundViewport::Desktop => "Compact desktop",
        PlaygroundViewport::Mobile => "Mobile",
    };
    rsx! {
        section {
            class: "playground-viewport-card playground-viewport-{viewport.as_str()}",
            aria_label: format!("{label} component preview"),
            div {
                class: "playground-viewport-demo",
                "data-g3-mode": mode.as_str(),
                ThemeProvider { mode, theme: theme.to_theme(),
                    RenderSelectedDemo { demo, mode, theme }
                }
            }
        }
    }
}
/// The source panels under a demo: the demo itself, then each component it
/// shows, titled so the reader knows which is which.
fn source_panels(demo: &ComponentPlaygroundDemo) -> Vec<(String, String)> {
    let mut panels = Vec::new();
    let demo_code = g3_ui::function_source(demo.source, &format!("fn {}", demo.demo_name))
        .unwrap_or(demo.source);
    panels.push((
        format!("{} demo source", demo.descriptor.name),
        demo_code.trim().to_string(),
    ));
    for name in demo.components {
        if let Some(code) = g3_ui::component_source(name) {
            panels.push((format!("{name} source"), code.trim().to_string()));
        }
    }
    panels
}
#[component]
fn PlaygroundNav(
    demos: Vec<ComponentPlaygroundDemo>,
    current_route: Route,
    mut selector_open: Signal<bool>,
    dismiss_on_select: bool,
) -> Element {
    let demo_app = demos
        .iter()
        .copied()
        .find(|demo| demo.descriptor.name == "DemoApp");
    let mut component_demos = demos
        .into_iter()
        .filter(|demo| demo.descriptor.name != "DemoApp")
        .collect::<Vec<_>>();
    component_demos.sort_by_key(|demo| demo.descriptor.name.to_ascii_lowercase());
    rsx! {
        List { class: "playground-nav", lines: ListLines::None,
            if let Some(demo) = demo_app {
                ComponentNavButton {
                    demo,
                    current_route: current_route.clone(),
                    selector_open,
                    dismiss_on_select,
                }
            }
            Item {
                selected: current_route.is_transition_showcase(),
                label: "RouteTransitions",
                onclick: move |_| async move {
                    if dismiss_on_select {
                        selector_open.set(false);
                    }
                    animated_navigate(Route::TransitionListen {}).await;
                },
            }
            for demo in component_demos {
                ComponentNavButton {
                    demo,
                    current_route: current_route.clone(),
                    selector_open,
                    dismiss_on_select,
                }
            }
        }
    }
}
#[component]
fn RenderSelectedDemo(
    demo: ComponentPlaygroundDemo,
    mode: ComponentMode,
    theme: PlaygroundTheme,
) -> Element {
    let _ = (mode, theme);
    (demo.render)()
}
#[component]
fn ComponentNavButton(
    demo: ComponentPlaygroundDemo,
    current_route: Route,
    mut selector_open: Signal<bool>,
    dismiss_on_select: bool,
) -> Element {
    let slug = demo_slug(demo.descriptor.name);
    let is_selected = matches!(
        &current_route,
        Route::ComponentDemo { slug: selected_slug } if selected_slug == &slug
    );
    rsx! {
        Item {
            selected: is_selected,
            label: demo.descriptor.name.to_string(),
            onclick: move |_| {
                let slug = slug.clone();
                if dismiss_on_select {
                    selector_open.set(false);
                }
                spawn(async move {
                    animated_navigate(Route::ComponentDemo { slug }).await;
                });
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{PlaygroundViewport, demo_slug, source_panels};
    #[test]
    fn viewport_picker_exposes_mobile_and_desktop_shell_widths() {
        assert_eq!(PlaygroundViewport::Mobile.as_str(), "mobile");
        assert_eq!(PlaygroundViewport::Desktop.as_str(), "desktop");
    }
    #[test]
    fn every_demo_lists_its_demo_and_component_sources() {
        for demo in g3_ui::component_playground_demos() {
            let panels = source_panels(&demo);
            let (title, code) = &panels[0];
            assert!(title.ends_with("demo source"), "{title}");
            assert!(
                code.contains(&format!("fn {}", demo.demo_name))
                    && !code.contains("g3_playground!"),
                "{} demo source is not just the demo function",
                demo.descriptor.name
            );
            assert_eq!(
                panels.len(),
                demo.components.len() + 1,
                "{} names a component with no source",
                demo.descriptor.name
            );
            for (name, (_, code)) in demo.components.iter().zip(&panels[1..]) {
                assert!(code.contains(&format!("pub fn {name}")), "{name}");
            }
        }
    }
    #[test]
    fn component_names_have_stable_shareable_slugs() {
        assert_eq!(demo_slug("DemoApp"), "demoapp");
        assert_eq!(demo_slug("SegmentGroup"), "segmentgroup");
        assert_eq!(demo_slug("Confirm Modal"), "confirm-modal");
    }
}
