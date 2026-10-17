//! g3-ui playground - component-owned demos rendered in responsive device contexts.
use dioxus::prelude::*;
use dioxus_code::{Code, CodeTheme, Language, SourceCode, Theme};
use dioxus_icons::lucide::{Package, Search};
use g3_route_transitions::{
    Platform, RouteTransitions, animated_navigate, set_platform, use_browser_history_transitions,
};
use g3_ui::{
    AppWrapper, Button, ButtonFill, ButtonSize, ComponentMode, ComponentPlaygroundDemo, Header,
    Item, List, ListLines, Look, NavigationDrawer, SegmentButton, SegmentGroup, Select,
    SelectOption, SideSheet, Theme as G3Theme, ThemeProvider, component_playground_demos,
};
use gloo_timers::future::TimeoutFuture;
use manganis::{AssetOptions, asset};

mod code_block;
mod component_props;
mod docs;
mod landing;
mod theme_generator;
mod themes;
mod transition_showcase;
use code_block::CodeBlock;
use docs::Guide;
use landing::Landing;
use theme_generator::ThemeGenerator;
use themes::PlaygroundTheme;
use transition_showcase::{
    TransitionAlbum, TransitionEpisode, TransitionLibrary, TransitionListen, TransitionProfile,
    TransitionQueue, TransitionRadio,
};
#[allow(dead_code)]
const PLAYGROUND_CSS: Asset = asset!(
    "/assets/playground.css",
    AssetOptions::css().with_static_head(true)
);
const FAVICON: Asset = asset!("/assets/favicon.svg");

#[derive(Clone, Debug, PartialEq, Routable, RouteTransitions)]
#[rustfmt::skip]
enum Route {
    #[layout(Playground)]
        #[transition(layer = stack_root)]
        #[route("/")]
        Landing {},

        #[transition(layer = stack_root)]
        #[route("/docs/:slug")]
        Guide { slug: String },

        #[transition(layer = stack_root)]
        #[route("/theme-generator")]
        ThemeGenerator {},

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
enum PlaygroundViewport {
    Mobile,
    Tablet,
    Landscape,
    Desktop,
    Resizable,
}
#[derive(Clone, Copy)]
struct PlaygroundSettings {
    mode: Signal<ComponentMode>,
    viewport: Signal<PlaygroundViewport>,
    theme: Signal<PlaygroundTheme>,
    custom_theme: Signal<G3Theme>,
}

impl PlaygroundViewport {
    fn as_str(self) -> &'static str {
        match self {
            Self::Mobile => "mobile",
            Self::Tablet => "tablet",
            Self::Landscape => "landscape",
            Self::Desktop => "desktop",
            Self::Resizable => "resizable",
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
        document::Link { rel: "icon", r#type: "image/svg+xml", href: FAVICON }
        Router::<Route> {}
    }
}
#[component]
fn Playground() -> Element {
    let demos = component_playground_demos();
    let mode = use_signal(|| ComponentMode::Md);
    let viewport = use_signal(|| PlaygroundViewport::Mobile);
    let preview_width = use_signal(|| 800.0);
    // Open from the first paint on a wide shell, where the drawer is a
    // persistent `Menu` that reserves its own space beside the page. Closed on
    // a phone, where it is a dismissible `Overlay` summoned from the header.
    let mut selector_open = use_signal(|| !initial_compact_shell().unwrap_or(true));
    let source_value = use_signal(Vec::<String>::new);
    let theme = use_signal(|| PlaygroundTheme::Blue);
    let custom_theme = use_signal(|| PlaygroundTheme::Blue.to_theme(&G3Theme::default_light()));
    let mut playing = use_signal(|| false);
    let mut search_open = use_signal(|| false);
    let search_value = use_signal(String::new);
    let mut shortcut = use_signal(|| None::<document::Eval>);
    use_effect(move || {
        let eval = document::eval(
            r#"
            const onKey = event => {
                if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === 'k') {
                    event.preventDefault();
                    document.getElementById('playground-search-trigger')?.click();
                }
            };
            document.addEventListener('keydown', onKey);
            try { await dioxus.recv(); } finally { document.removeEventListener('keydown', onKey); }
            return null;
        "#,
        );
        shortcut.set(Some(eval));
    });
    use_drop(move || {
        if let Some(eval) = *shortcut.peek() {
            let _ = eval.send("dispose");
        }
    });
    let current_route: Route = use_route();
    let selected = selected_demo(&demos, &current_route);
    let showing_transitions = current_route.is_transition_showcase();
    let showing_preview = current_route.has_preview_controls();
    let guide = match &current_route {
        Route::Guide { slug } => docs::page(slug),
        _ => None,
    };
    let showing_guide = matches!(current_route, Route::Guide { .. });
    let showing_generator = matches!(current_route, Route::ThemeGenerator {});
    let showing_landing = matches!(current_route, Route::Landing {});
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
    let page_title = if showing_landing {
        "g3-ui".to_string()
    } else if showing_generator {
        "Theme generator".to_string()
    } else if showing_guide {
        guide
            .map_or("Page not found", |page| page.title)
            .to_string()
    } else if showing_transitions {
        "Route transitions".to_string()
    } else if selected.descriptor.name == "DemoApp" {
        "Demo app".to_string()
    } else {
        selected.descriptor.name.to_string()
    };
    let page_description = if showing_generator {
        "Edit theme colors and copy the Rust configuration."
    } else if showing_guide {
        guide.map_or("Find guides, hooks, and examples.", |page| page.summary)
    } else if showing_transitions {
        transition_showcase::DESCRIPTION
    } else {
        selected.descriptor.description
    };
    let source_panels = if !showing_preview || showing_generator || showing_guide || showing_landing
    {
        vec![]
    } else if showing_transitions {
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
        custom_theme,
    });
    let selector = rsx! {
        div { class: "playground-sidebar-header",
            h2 { "g3-ui" }
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
            theme: theme().to_theme(&custom_theme()),
            mode: active_mode,
            class: "playground-root",
            layout: false,
            route_transition_overlay: false,
            div { class: if showing_guide || showing_generator || showing_landing { "playground-page playground-document-page" } else { "playground-page" },
                Header {
                    title: page_title,
                    mode: active_mode,
                    class: "playground-header",
                    start: rsx! {
                        Button {
                            fill: ButtonFill::Clear,
                            size: ButtonSize::Sm,
                            aria_label: "Toggle navigation menu",
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
                        button {
                            id: "playground-search-trigger",
                            class: "playground-search-trigger",
                            onclick: move |_| search_open.set(true),
                            aria_label: "Search documentation",
                            span { class: "playground-search-icon", aria_hidden: "true", Search { size: 20 } }
                            span { class: "playground-search-label", "Search" }
                            span { class: "playground-search-shortcut", "Ctrl K" }
                        }
                        a { class: "playground-header-link", href: "https://github.com/g3techhq/g3-ui", target: "_blank", rel: "noopener noreferrer", aria_label: "GitHub repository", title: "GitHub repository",
                            span { aria_hidden: "true", svg { width: "20", height: "20", view_box: "0 0 16 16", fill: "currentColor",
                                path { d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27s1.36.09 2 .27c1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z" }
                            } }
                        }
                        a { class: "playground-header-link", href: "https://docs.rs/g3-ui/latest/g3_ui/", target: "_blank", rel: "noopener noreferrer", aria_label: "API reference", title: "API reference (docs.rs)",
                            span { aria_hidden: "true", Package { size: 20 } }
                        }
                        Select {
                            value: theme,
                            aria_label: "Theme",
                            class: "playground-theme-picker",
                            width: g3_ui::SelectWidth::Fit,
                            options: theme_options,
                        }
                    },
                    toolbar: showing_preview.then(|| rsx! {
                        div { class: "playground-header-toggles",
                            SegmentGroup { value: mode, aria_label: "Platform",
                                SegmentButton { value: ComponentMode::Md, "MD" }
                                SegmentButton { value: ComponentMode::Ios, "iOS" }
                            }
                            SegmentGroup { value: viewport, aria_label: "Viewport",
                                SegmentButton { value: PlaygroundViewport::Mobile, "Mobile" }
                                SegmentButton { value: PlaygroundViewport::Tablet, "Tablet" }
                                SegmentButton { value: PlaygroundViewport::Landscape, aria_label: "Landscape phone", "Landscape" }
                                SegmentButton { value: PlaygroundViewport::Desktop, "Desktop" }
                                SegmentButton { value: PlaygroundViewport::Resizable, "Resize" }
                            }
                        }
                    }),
                }
                main { class: "playground-main",
                    // Keys only take effect in a list, so this is a list of one.
                    for page in std::iter::once(current_route.page_key()) {
                        ScrollToTop { key: "{page}" }
                    }
                    div { class: "playground-intro",
                        p { class: "playground-description", "{page_description}" }
                        if showing_transitions {
                            div { class: "playground-intro-actions",
                                Button {
                                    disabled: playing(),
                                    aria_label: "Play every route transition",
                                    onclick: play_all,
                                    if playing() { "Playing tour…" } else { "Play transition tour" }
                                }
                                Button {
                                    href: "https://crates.io/crates/g3-route-transitions",
                                    new_tab: true,
                                    fill: ButtonFill::Outline,
                                    aria_label: "g3-route-transitions on crates.io",
                                    start: rsx! { span { aria_hidden: "true", Package { size: 20 } } },
                                    "crates.io"
                                }
                            }
                        }
                    }
                    if showing_preview && active_viewport == PlaygroundViewport::Resizable {
                        g3_ui::Range {
                            label: "Preview width (pixels)",
                            value: preview_width,
                            min: 320.0,
                            max: 1440.0,
                            step: 1.0,
                            show_value: true,
                            helper: "Drag the slider or use arrow keys to check intermediate widths.",
                        }
                    }
                    div {
                        class: "playground-stage",
                        style: "--playground-preview-width: {preview_width()}px;",
                        "data-playground-viewport": active_viewport.as_str(),
                        Outlet::<Route> {}
                    }
                    if matches!(current_route, Route::ComponentDemo { .. }) {
                        component_props::PropsReference { components: selected.components.to_vec() }
                    }
                    if showing_preview && !showing_generator && !showing_guide && !showing_landing { g3_ui::AccordionGroup { value: source_value, multiple: true, class: "source-panel",
                        for (title, code) in source_panels {
                            g3_ui::AccordionItem {
                                key: "{title}",
                                value: title.clone(),
                                label: title.clone(),
                                CodeBlock { src: code }
                            }
                        }
                    } }
                }
            }
            // A phone has no room to keep the menu beside the page, so it
            // becomes a dismissible sheet there.
            if is_compact {
                SideSheet {
                    open: selector_open,
                    aria_label: "Navigation",
                    padding: false,
                    class: "playground-selector-sheet",
                    {selector}
                }
            } else {
                NavigationDrawer {
                    open: selector_open,
                    aria_label: "Navigation",
                    padding: false,
                    class: "playground-selector-sheet",
                    {selector}
                }
            }
            DocsSearch { open: search_open, value: search_value, selector_open }
        }
    }
}

/// Scrolls the page back to the top when it mounts. Keyed by
/// [`Route::page_key`] in a list of one, so it remounts - and scrolls - only when the reader
/// moves to another page. `.playground-main` is the scroll container and lives
/// in the layout, so without this a new page opens at the old one's offset.
#[component]
fn ScrollToTop() -> Element {
    use_effect(|| {
        document::eval(
            "document.querySelector('.playground-main')?.scrollTo({ top: 0, left: 0, behavior: 'instant' });",
        );
    });
    rsx! {}
}

impl Route {
    /// Names the page a route belongs to. The transition showcase is one page
    /// whose routes navigate inside the device, so they share a key.
    fn page_key(&self) -> String {
        match self {
            Self::Landing {} => "landing".into(),
            Self::Guide { slug } => format!("guide/{slug}"),
            Self::ThemeGenerator {} => "theme-generator".into(),
            Self::ComponentDemo { slug } => format!("component/{slug}"),
            Self::NotFound { .. } => "not-found".into(),
            _ => "transitions".into(),
        }
    }

    fn has_preview_controls(&self) -> bool {
        matches!(
            self,
            Self::Landing {} | Self::ComponentDemo { .. } | Self::ThemeGenerator {}
        ) || matches!(self, Self::Guide { slug } if docs::page(slug).is_some())
            || self.is_transition_showcase()
    }

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

fn active_playground_settings() -> (ComponentMode, G3Theme, PlaygroundViewport) {
    let settings = use_context::<PlaygroundSettings>();
    (
        (settings.mode)(),
        (settings.theme)().to_theme(&(settings.custom_theme)()),
        (settings.viewport)(),
    )
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
            key: "{demo.descriptor.name}",
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
    theme: G3Theme,
    viewport: PlaygroundViewport,
) -> Element {
    let label = match viewport {
        PlaygroundViewport::Desktop => "Desktop",
        PlaygroundViewport::Mobile => "Mobile",
        PlaygroundViewport::Tablet => "Tablet",
        PlaygroundViewport::Landscape => "Landscape phone",
        PlaygroundViewport::Resizable => "Resizable",
    };
    rsx! {
        section {
            class: "playground-viewport-card playground-viewport-{viewport.as_str()}",
            aria_label: format!("{label} component preview"),
            div {
                class: "playground-viewport-demo",
                "data-g3-mode": mode.as_str(),
                ThemeProvider { mode, theme: theme.clone(),
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

#[derive(Clone, PartialEq)]
struct SearchEntry {
    title: String,
    description: String,
    section: &'static str,
    route: Route,
}

fn search_entries(query: &str) -> Vec<SearchEntry> {
    let query = query.trim().to_lowercase();
    let mut entries: Vec<_> = docs::PAGES
        .iter()
        .filter(|page| {
            format!(
                "{} {} {} {}",
                page.title, page.summary, page.section, page.keywords
            )
            .to_lowercase()
            .contains(&query)
        })
        .map(|page| SearchEntry {
            title: page.title.into(),
            description: page.summary.into(),
            section: page.section,
            route: Route::Guide {
                slug: page.slug.into(),
            },
        })
        .collect();
    if "why g3-ui overview home mobile first dioxus ionic android ios desktop".contains(&query) {
        entries.insert(
            0,
            SearchEntry {
                title: "g3-ui overview".into(),
                description:
                    "iOS and Material components, responsive layouts, themes, and overlays.".into(),
                section: "Overview",
                route: Route::Landing {},
            },
        );
    }
    entries.extend(
        component_playground_demos()
            .into_iter()
            .filter(|demo| {
                format!(
                    "{} {} {} {} {}",
                    demo.descriptor.name,
                    if demo.descriptor.name == "DemoApp" {
                        "Demo app"
                    } else {
                        demo.descriptor.name
                    },
                    demo.descriptor.description,
                    demo.components.join(" "),
                    if demo.descriptor.name == "DemoApp" {
                        "Examples"
                    } else {
                        "Components"
                    },
                )
                .to_lowercase()
                .contains(&query)
            })
            .map(|demo| SearchEntry {
                title: if demo.descriptor.name == "DemoApp" {
                    "Demo app".into()
                } else {
                    demo.descriptor.name.into()
                },
                description: demo.descriptor.description.into(),
                section: if demo.descriptor.name == "DemoApp" {
                    "Examples"
                } else {
                    "Components"
                },
                route: Route::ComponentDemo {
                    slug: demo_slug(demo.descriptor.name),
                },
            }),
    );
    if "route transitions routetransitions animation navigation examples".contains(&query) {
        entries.push(SearchEntry {
            title: "Route transitions".into(),
            description: transition_showcase::DESCRIPTION.into(),
            section: "Examples",
            route: Route::TransitionListen {},
        });
    }
    if "theme generator colors palette custom branding".contains(&query) {
        entries.push(SearchEntry {
            title: "Theme generator".into(),
            description: "Edit theme colors and preview the result.".into(),
            section: "Tools",
            route: Route::ThemeGenerator {},
        });
    }
    entries
}

#[component]
fn DocsSearch(
    mut open: Signal<bool>,
    value: Signal<String>,
    mut selector_open: Signal<bool>,
) -> Element {
    let query = value();
    let results = search_entries(&query);
    let no_matches = results.is_empty();
    rsx! {
        g3_ui::Modal { open, title: "Search documentation", size: g3_ui::ModalSize::Wide, class: "playground-search",
            actions: rsx! {
                span { class: "playground-search-hint", "Tab to a result · Enter to open · Esc to close" }
                Button { fill: ButtonFill::Clear, onclick: move |_| open.set(false), "Close search" }
            },
            g3_ui::Searchbar { value, aria_label: "Search guides, hooks, and components", placeholder: "Search the library" }
            p { class: "playground-search-count", role: "status", aria_live: "polite",
                "{results.len()} "
                if results.len() == 1 { "result. " } else { "results. " }
            }
            for (query, results) in std::iter::once((query, results)) {
              div { key: "{query}", class: "playground-search-results",
              ul {
                for entry in results {
                    li {
                        key: "{entry.section}-{entry.title}",
                      button {
                        class: "playground-search-result",
                        aria_label: format!("{} · {}", entry.section, entry.title),
                        onclick: move |_| {
                            let route = entry.route.clone();
                            open.set(false);
                            if initial_compact_shell().unwrap_or(false) { selector_open.set(false); }
                            spawn(async move { animated_navigate(route).await; });
                        },
                        span { class: "playground-search-result-category", "{entry.section}" }
                        span { class: "playground-search-result-title", "{entry.title}" }
                        span { class: "playground-search-result-description", "{entry.description}" }
                      }
                    }
                }
              }
              if no_matches {
                  div { class: "playground-search-empty",
                      h3 { "No matches" }
                      p { "Search by component, hook, or topic." }
                  }
              }
            }
            }
        }
    }
}

#[component]
fn DocNavButton(
    slug: String,
    title: String,
    current_route: Route,
    mut selector_open: Signal<bool>,
    dismiss_on_select: bool,
) -> Element {
    let selected = matches!(&current_route, Route::Guide { slug: current } if current == &slug);
    rsx! { Item { label: title, selected,
        onclick: move |_| {
            let slug = slug.clone();
            if dismiss_on_select { selector_open.set(false); }
            spawn(async move { animated_navigate(Route::Guide { slug }).await; });
        },
    } }
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
            Item {
                label: "Overview",
                selected: matches!(current_route, Route::Landing {}),
                onclick: move |_| async move {
                    if dismiss_on_select { selector_open.set(false); }
                    animated_navigate(Route::Landing {}).await;
                },
            }
            for section in ["Guides", "Patterns", "Hooks"] {
                g3_ui::ListHeader { "{section}" }
                for page in docs::PAGES.iter().filter(|page| page.section == section) {
                    DocNavButton { slug: page.slug.to_string(), title: page.title.to_string(), current_route: current_route.clone(), selector_open, dismiss_on_select }
                }
            }
            g3_ui::ListHeader { "Examples" }
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
                label: "Route transitions",
                onclick: move |_| async move {
                    if dismiss_on_select {
                        selector_open.set(false);
                    }
                    animated_navigate(Route::TransitionListen {}).await;
                },
            }
            g3_ui::ListHeader { "Tools" }
            Item {
                label: "Theme generator",
                selected: matches!(current_route, Route::ThemeGenerator {}),
                onclick: move |_| async move {
                    if dismiss_on_select { selector_open.set(false); }
                    animated_navigate(Route::ThemeGenerator {}).await;
                },
            }
            g3_ui::ListHeader { "Components" }
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
    theme: G3Theme,
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
            label: if demo.descriptor.name == "DemoApp" { "Demo app".to_string() } else { demo.descriptor.name.to_string() },
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
    fn preview_controls_only_appear_on_interactive_routes() {
        use super::Route;
        for page in super::docs::PAGES {
            assert!(
                Route::Guide {
                    slug: page.slug.into()
                }
                .has_preview_controls()
            );
        }
        assert!(
            !Route::Guide {
                slug: "missing-guide".into()
            }
            .has_preview_controls()
        );
        assert!(Route::Landing {}.has_preview_controls());
        assert!(!Route::NotFound { segments: vec![] }.has_preview_controls());
        assert!(Route::ThemeGenerator {}.has_preview_controls());
        assert!(
            Route::ComponentDemo {
                slug: "button".into()
            }
            .has_preview_controls()
        );
        assert!(Route::TransitionListen {}.has_preview_controls());
    }
    #[test]
    fn viewport_picker_exposes_mobile_and_desktop_shell_widths() {
        assert_eq!(PlaygroundViewport::Mobile.as_str(), "mobile");
        assert_eq!(PlaygroundViewport::Tablet.as_str(), "tablet");
        assert_eq!(PlaygroundViewport::Landscape.as_str(), "landscape");
        assert_eq!(PlaygroundViewport::Desktop.as_str(), "desktop");
        assert_eq!(PlaygroundViewport::Resizable.as_str(), "resizable");
    }
    #[test]
    fn documentation_and_public_names_share_the_search_index() {
        assert!(
            super::search_entries("use_layout_size")
                .iter()
                .any(|entry| entry.title == "Layout hooks")
        );
        assert!(
            super::search_entries("AdaptiveLayout")
                .iter()
                .any(|entry| entry.title == "Responsive")
        );
        assert!(super::search_entries("getting started").iter().any(|entry| matches!(&entry.route, super::Route::Guide { slug } if slug == "getting-started")));
        assert!(
            super::search_entries("  USE_TOAST ")
                .iter()
                .any(|entry| entry.title == "Overlay hooks")
        );
        assert!(super::search_entries("no-such-page-12345").is_empty());
        assert!(
            super::search_entries("overview")
                .iter()
                .any(|entry| matches!(entry.route, super::Route::Landing {}))
        );
        assert!(
            super::search_entries("theme generator")
                .iter()
                .any(|entry| matches!(entry.route, super::Route::ThemeGenerator {}))
        );
    }
    #[test]
    fn documentation_slugs_and_source_links_are_valid_and_unique() {
        let mut slugs = std::collections::HashSet::new();
        for page in super::docs::PAGES {
            assert!(slugs.insert(page.slug));
            assert!(!page.summary.is_empty() && !page.keywords.is_empty());
            assert_eq!(super::docs::page(page.slug).unwrap().title, page.title);
        }
        for demo in g3_ui::component_playground_demos() {
            assert!(demo.source_path.starts_with("src/components/"));
        }
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
    fn a_new_page_scrolls_to_the_top_but_the_transition_showcase_does_not() {
        use super::Route;
        let button = Route::ComponentDemo {
            slug: "button".into(),
        };
        let sheet = Route::ComponentDemo {
            slug: "sidesheet".into(),
        };
        assert_ne!(button.page_key(), sheet.page_key());
        assert_ne!(Route::Landing {}.page_key(), button.page_key());
        assert_eq!(
            Route::TransitionListen {}.page_key(),
            Route::TransitionAlbum {}.page_key()
        );
    }

    #[test]
    fn component_names_have_stable_shareable_slugs() {
        assert_eq!(demo_slug("DemoApp"), "demoapp");
        assert_eq!(demo_slug("SegmentGroup"), "segmentgroup");
        assert_eq!(demo_slug("Confirm Modal"), "confirm-modal");
    }
}
