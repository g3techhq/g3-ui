//! g3-ui playground - component-owned demos rendered in responsive device contexts.
use dioxus::prelude::*;
use dioxus_code::{Code, CodeTheme, Language, SourceCode, Theme};
use g3_route_transitions::{Platform, animated_navigate, route_transitions, set_platform};
use g3_ui::{
    AppWrapper, ComponentMode, ComponentPlaygroundDemo, G3Theme, SegmentButton, SegmentGroup,
    Select, SelectOption, Sheet, SheetPlacement, component_playground_demos,
};
use gloo_timers::future::TimeoutFuture;
use manganis::{AssetOptions, asset};

mod transition_showcase;
use transition_showcase::{
    TransitionArticle, TransitionArticleDetail, TransitionDetail, TransitionHome,
    TransitionProfile, TransitionQueue, TransitionRatings,
};
#[allow(dead_code)]
const PLAYGROUND_CSS: Asset = asset!(
    "/assets/playground.css",
    AssetOptions::css().with_static_head(true)
);

#[route_transitions]
#[derive(Clone, Debug, PartialEq, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(Playground)]
        #[transition(root)]
        #[route("/")]
        Landing {},

        #[transition(root)]
        #[route("/components/:slug")]
        ComponentDemo { slug: String },

        #[transition(root)]
        #[route("/transitions")]
        TransitionHome {},

        #[transition(pushed)]
        #[route("/transitions/detail")]
        TransitionDetail {},

        #[transition(cover)]
        #[route("/transitions/queue")]
        TransitionQueue {},

        #[transition(base, replace, push(group = showcase_tabs, order = tab))]
        #[route("/transitions/ratings/:tab")]
        TransitionRatings { tab: u8 },

        #[transition(root)]
        #[route("/transitions/profile")]
        TransitionProfile {},

        #[transition(base)]
        #[route("/transitions/article")]
        TransitionArticle {},

        #[transition(morph)]
        #[route("/transitions/article/read")]
        TransitionArticleDetail {},

        #[transition(root)]
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlaygroundTheme {
    Blue,
    Green,
    Night,
    Forest,
}
impl PlaygroundTheme {
    fn from_value(value: &str) -> Self {
        match value {
            "Green" | "green" | "Fairway" | "fairway" => Self::Green,
            "Night" | "night" | "Twilight" | "twilight" => Self::Night,
            "Forest" | "forest" | "Terminal" | "terminal" => Self::Forest,
            _ => Self::Blue,
        }
    }
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
                focused: "#2563eb".into(),
                bg: "#eef5ff".into(),
                bg_secondary: "#dbeafe".into(),
                card_inset: "#f8fbff".into(),
                control: "#eaf2ff".into(),
                card_border: "rgba(37, 99, 235, 0.2)".into(),
                label_primary: "#102033".into(),
                label_secondary: "#496278".into(),
                text: "#102033".into(),
                text_secondary: "#53677f".into(),
                shadow: "rgba(30, 64, 175, 0.16)".into(),
                ..G3Theme::default_light()
            },
            Self::Green => G3Theme {
                focused: "#1f7a4d".into(),
                bg: "#edf7f0".into(),
                bg_secondary: "#dceee2".into(),
                card_inset: "#f5fbf7".into(),
                control: "#e5f3ea".into(),
                card_border: "rgba(31, 122, 77, 0.24)".into(),
                label_primary: "#12251a".into(),
                label_secondary: "#4d6657".into(),
                text: "#12251a".into(),
                text_secondary: "#52685c".into(),
                shadow: "rgba(24, 74, 47, 0.16)".into(),
                ..G3Theme::default_light()
            },
            Self::Night => G3Theme {
                focused: "#7dd3fc".into(),
                bg: "#07111f".into(),
                bg_secondary: "#0e1b2e".into(),
                card: "#111d2e".into(),
                card_inset: "#172438".into(),
                surface: "#101a2b".into(),
                control: "#1b2a40".into(),
                card_border: "rgba(148, 163, 184, 0.24)".into(),
                label_primary: "#e5edf6".into(),
                label_secondary: "#b8c7d8".into(),
                text: "#e5edf6".into(),
                text_secondary: "#9fb0c3".into(),
                shadow: "rgba(0, 0, 0, 0.54)".into(),
                ..G3Theme::default_dark()
            },
            Self::Forest => G3Theme {
                focused: "#34d399".into(),
                bg: "#050806".into(),
                bg_secondary: "#0b130d".into(),
                card: "#101811".into(),
                card_inset: "#162218".into(),
                surface: "#0e1610".into(),
                control: "#18241b".into(),
                card_border: "rgba(134, 239, 172, 0.2)".into(),
                label_primary: "#e6f4e8".into(),
                label_secondary: "#b8d4bd".into(),
                text: "#e6f4e8".into(),
                text_secondary: "#9eb5a4".into(),
                shadow: "rgba(0, 0, 0, 0.62)".into(),
                ..G3Theme::default_dark()
            },
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum PlaygroundViewport {
    Mobile,
    Desktop,
}
#[derive(Clone, Copy)]
struct PlaygroundSettings {
    mode_index: Signal<usize>,
    viewport_index: Signal<usize>,
    theme_value: Signal<String>,
}

impl PlaygroundViewport {
    fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Desktop,
            _ => Self::Mobile,
        }
    }
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
    rsx! {
        Router::<Route> {}
    }
}
#[component]
fn Playground() -> Element {
    let demos = component_playground_demos();
    let mode_index = use_signal(|| 0_usize);
    let viewport_index = use_signal(|| 0_usize);
    // Open from the first paint on a wide shell, where the drawer is a
    // persistent `Menu` that reserves its own space beside the page. Closed on
    // a phone, where it is a dismissible `Overlay` summoned from the header.
    let mut selector_open = use_signal(|| !initial_compact_shell().unwrap_or(true));
    let source_value = use_signal(Vec::<String>::new);
    let theme_value = use_signal(|| PlaygroundTheme::Blue.label().to_string());
    let mut playing = use_signal(|| false);
    let current_route: Route = use_route();
    let selected = selected_demo(&demos, &current_route);
    let showing_transitions = current_route.is_transition_showcase();
    let count = demos.len() + 1;
    let active_mode = if mode_index() == 1 {
        ComponentMode::Ios
    } else {
        ComponentMode::Md
    };
    let active_theme = PlaygroundTheme::from_value(&theme_value());
    let active_viewport = PlaygroundViewport::from_index(viewport_index());
    let compact_shell = use_compact_shell();
    let is_compact = compact_shell().unwrap_or(false);
    let selector_side_type = if is_compact {
        g3_ui::SideSheetType::Overlay
    } else {
        g3_ui::SideSheetType::Menu
    };
    set_platform(match active_mode {
        ComponentMode::Ios => Platform::Ios,
        ComponentMode::Md => Platform::Md,
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
        "Route transitions".to_string()
    } else {
        selected.descriptor.name.to_string()
    };
    let page_description = if showing_transitions {
        transition_showcase::DESCRIPTION
    } else {
        selected.descriptor.description
    };
    let selected_source = if showing_transitions {
        transition_showcase::SOURCE.to_string()
    } else {
        playground_demo_source(selected.source)
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
        .map(|theme| SelectOption::from((theme.label(), theme.label())))
        .collect::<Vec<_>>();
    provide_context(PlaygroundSettings {
        mode_index,
        viewport_index,
        theme_value,
    });
    rsx! {
        AppWrapper {
            theme: active_theme.to_theme(),
            mode: active_mode,
            class: "playground-root",
            layout: false,
            route_transition_root: false,
            div { class: "playground-page",
                g3_ui::Header {
                    title: page_title,
                    mode: active_mode,
                    class: "playground-header",
                    start_button: rsx! {
                        g3_ui::Button {
                            style: g3_ui::ButtonStyle::Clear,
                            size: g3_ui::ButtonSize::Sm,
                            aria_label: "Toggle component menu".to_string(),
                            class: "playground-menu-button",
                            onclick: move |_| selector_open.toggle(),
                            span { class: "playground-menu-icon", aria_hidden: "true",
                                span {}
                                span {}
                                span {}
                            }
                        }
                    },
                    end_button: rsx! {
                        Select { value: theme_value, mode: active_mode, options: theme_options }
                    },
                    toolbar: rsx! {
                        div { class: "playground-header-toggles",
                            SegmentGroup { active: mode_index, mode: active_mode,
                                SegmentButton { index: 0, mode: active_mode, "MD" }
                                SegmentButton { index: 1, mode: active_mode, "iOS" }
                            }
                            SegmentGroup { active: viewport_index, mode: active_mode,
                                SegmentButton { index: 0, mode: active_mode, "Mobile" }
                                SegmentButton { index: 1, mode: active_mode, "Desktop" }
                            }
                        }
                    },
                }
                main { class: "playground-main",
                    div { class: "playground-intro",
                        p { class: "playground-description", "{page_description}" }
                        if showing_transitions {
                            g3_ui::Button {
                                disabled: playing(),
                                aria_label: "Play every route transition".to_string(),
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
                    g3_ui::AccordionGroup { value: source_value, class: "source-panel",
                        g3_ui::AccordionItem {
                            value: "source".to_string(),
                            label: "Source".to_string(),
                            Code {
                                src: SourceCode::new(Language::Rust, selected_source.clone()),
                                theme: CodeTheme::system(Theme::GITHUB_LIGHT, Theme::GITHUB_DARK),
                            }
                        }
                    }
                }
            }
            Sheet {
                is_open: selector_open,
                placement: SheetPlacement::Left(selector_side_type),
                mode: active_mode,
                class: "playground-selector-sheet",
                div { class: "playground-sidebar-header",
                    h2 { "g3_ui" }
                    div { class: "subtitle", "{count} demos" }
                }
                PlaygroundNav {
                    demos: demos.clone(),
                    current_route,
                    selector_open,
                    dismiss_on_select: is_compact,
                }
            }
        }
    }
}

impl Route {
    fn is_transition_showcase(&self) -> bool {
        matches!(
            self,
            Self::TransitionHome {}
                | Self::TransitionDetail {}
                | Self::TransitionQueue {}
                | Self::TransitionRatings { .. }
                | Self::TransitionProfile {}
                | Self::TransitionArticle {}
                | Self::TransitionArticleDetail {}
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
    let mode = if (settings.mode_index)() == 1 {
        ComponentMode::Ios
    } else {
        ComponentMode::Md
    };
    (
        mode,
        PlaygroundTheme::from_value(&(settings.theme_value)()),
        PlaygroundViewport::from_index((settings.viewport_index)()),
    )
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
    animated_navigate(Route::TransitionHome {}).await;
    TimeoutFuture::new(1_400).await;
    for route in [
        Route::TransitionDetail {},
        Route::TransitionHome {},
        Route::TransitionQueue {},
        Route::TransitionHome {},
        Route::TransitionProfile {},
        Route::TransitionHome {},
        Route::TransitionRatings { tab: 0 },
        Route::TransitionRatings { tab: 1 },
        Route::TransitionRatings { tab: 2 },
        Route::TransitionRatings { tab: 0 },
        Route::TransitionArticle {},
        Route::TransitionArticleDetail {},
        Route::TransitionArticle {},
        Route::TransitionHome {},
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
                g3_ui::G3ThemeProvider { mode, theme: theme
                            .to_theme(),
                    RenderSelectedDemo { demo, mode, theme }
                }
            }
        }
    }
}
fn playground_demo_source(source: &str) -> String {
    let Some(marker_index) = source.find("PlaygroundDemo") else {
        return source.trim().to_string();
    };
    let function_start = source[..marker_index]
        .rfind("pub fn")
        .unwrap_or(marker_index);
    let start = source[..function_start]
        .rfind("#[component]")
        .unwrap_or(function_start);
    let Some(open_brace) = source[function_start..]
        .find('{')
        .map(|index| function_start + index)
    else {
        return source[start..].trim().to_string();
    };
    let mut depth = 0_i32;
    for (index, ch) in source[open_brace..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let end = open_brace + index + ch.len_utf8();
                    return source[start..end].trim().to_string();
                }
            }
            _ => {}
        }
    }
    source[start..].trim().to_string()
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
        .find(|demo| demo.descriptor.name == "Demo App");
    let mut component_demos = demos
        .into_iter()
        .filter(|demo| demo.descriptor.name != "Demo App")
        .collect::<Vec<_>>();
    component_demos.sort_by_key(|demo| demo.descriptor.name.to_ascii_lowercase());
    rsx! {
        g3_ui::List { class: "playground-nav", lines: g3_ui::ListLines::None,
            if let Some(demo) = demo_app {
                ComponentNavButton {
                    demo,
                    current_route: current_route.clone(),
                    selector_open,
                    dismiss_on_select,
                }
            }
            g3_ui::Item {
                kind: g3_ui::ItemKind::Button,
                selected: current_route.is_transition_showcase(),
                label: "Route transitions".to_string(),
                description: "Navigation showcase".to_string(),
                onclick: move |_| async move {
                    if dismiss_on_select {
                        selector_open.set(false);
                    }
                    animated_navigate(Route::TransitionHome {}).await;
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
        g3_ui::Item {
            kind: g3_ui::ItemKind::Button,
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
    use super::{PlaygroundViewport, demo_slug, playground_demo_source};
    #[test]
    fn viewport_picker_exposes_mobile_and_desktop_shell_widths() {
        assert_eq!(PlaygroundViewport::from_index(0).as_str(), "mobile");
        assert_eq!(PlaygroundViewport::from_index(1).as_str(), "desktop");
        assert_eq!(PlaygroundViewport::from_index(2).as_str(), "mobile");
    }
    #[test]
    fn source_panel_extracts_only_playground_demo_function() {
        let source = r#"
fn helper() {}
#[component]
pub fn ButtonPlaygroundDemo() -> Element {
    rsx! { div { "Button" } }
}
crate::g3_playground! { name: "Button" }
"#;
        let extracted = playground_demo_source(source);
        assert!(extracted.contains("pub fn ButtonPlaygroundDemo"));
        assert!(extracted.contains("Button"));
        assert!(!extracted.contains("fn helper"));
        assert!(!extracted.contains("g3_playground"));
    }
    #[test]
    fn component_names_have_stable_shareable_slugs() {
        assert_eq!(demo_slug("Demo App"), "demo-app");
        assert_eq!(demo_slug("SegmentGroup"), "segmentgroup");
        assert_eq!(demo_slug("Confirm Modal"), "confirm-modal");
    }
}
