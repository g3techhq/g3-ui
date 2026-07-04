//! g3_ui playground - component-owned demos rendered in mobile context.

use dioxus::prelude::*;
use dioxus_code::{Code, CodeTheme, Language, SourceCode, Theme};
use g3_ui::{
    ComponentMode, ComponentPlaygroundDemo, SegmentButton, SegmentGroup, Select, SelectOption,
    Sheet, SheetPlacement, UI_CSS, component_playground_demos,
};
use manganis::asset;

const PLAYGROUND_CSS: Asset = asset!("/assets/playground.css");

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

    fn as_str(self) -> &'static str {
        match self {
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Night => "night",
            Self::Forest => "forest",
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
}

fn main() {
    g3_ui::init_auto_mode();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: UI_CSS }
        document::Link { rel: "stylesheet", href: PLAYGROUND_CSS }
        Playground {}
    }
}

#[component]
fn Playground() -> Element {
    let demos = component_playground_demos();
    let selected_index = use_signal(|| 0_usize);
    let mode_index = use_signal(|| 0_usize);
    let mut selector_open = use_signal(|| false);
    let mut theme_value = use_signal(|| PlaygroundTheme::Blue.label().to_string());
    let selected = demos
        .get(selected_index())
        .copied()
        .unwrap_or_else(|| demos[0]);
    let count = demos.len();
    let active_mode = if mode_index() == 1 {
        ComponentMode::Ios
    } else {
        ComponentMode::Md
    };
    let active_theme = PlaygroundTheme::from_value(&theme_value());
    g3_ui::set_mode(active_mode);

    let theme_options = PlaygroundTheme::all()
        .iter()
        .copied()
        .map(|theme| SelectOption::from((theme.label(), theme.label())))
        .collect::<Vec<_>>();

    rsx! {
        g3_ui::G3ThemeProvider { mode: active_mode,
            div {
                class: "playground-root",
                "data-g3-mode": active_mode.as_str(),
                "data-playground-theme": active_theme.as_str(),
                header { class: "playground-header",
                    div { class: "playground-header-row",
                        button {
                            class: "playground-menu-button",
                            r#type: "button",
                            aria_label: "Open component menu",
                            onclick: move |_| selector_open.set(true),
                            span { class: "playground-menu-icon", aria_hidden: "true",
                                span {}
                                span {}
                                span {}
                            }
                        }
                        div { class: "playground-title-block",
                            h1 { "{selected.descriptor.g3_name}" }
                            p { class: "description", "{selected.descriptor.description}" }
                        }
                        div { class: "playground-top-controls",
                            div { class: "playground-top-control playground-mode-control",
                                span { "Mode" }
                                SegmentGroup { active: mode_index, toolbar: true, mode: active_mode,
                                    SegmentButton { index: 0, mode: active_mode, "MD" }
                                    SegmentButton { index: 1, mode: active_mode, "iOS" }
                                }
                            }
                            div { class: "playground-top-control playground-theme-control",
                                span { "Theme" }
                                Select {
                                    value: theme_value,
                                    mode: active_mode,
                                    options: theme_options,
                                    onchange: move |next| theme_value.set(next),
                                }
                            }
                        }
                    }
                }
                Sheet { is_open: selector_open, placement: SheetPlacement::Left, mode: active_mode, class: "playground-selector-sheet",
                    div { class: "playground-sidebar-header",
                        h2 { "g3_ui" }
                        div { class: "subtitle", "{count} demos" }
                    }
                    PlaygroundNav { demos: demos.clone(), selected_index, selector_open }
                }
                main { class: "playground-main",
                    div { class: "playground-stage",
                        div {
                            key: "{active_mode.as_str()}-{active_theme.as_str()}-{selected.descriptor.name}",
                            "data-g3-mode": active_mode.as_str(),
                            "data-playground-theme": active_theme.as_str(),
                            g3_ui::G3ThemeProvider { mode: active_mode,
                                RenderSelectedDemo { key: "{active_mode.as_str()}-{active_theme.as_str()}-{selected.descriptor.name}", demo: selected, mode: active_mode, theme: active_theme }
                            }
                        }
                    }
                    details { class: "source-panel",
                        summary { "Source" }
                        Code {
                            src: SourceCode::new(Language::Rust, selected.source.to_string()),
                            theme: CodeTheme::system(Theme::GITHUB_LIGHT, Theme::GITHUB_DARK),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PlaygroundNav(
    demos: Vec<ComponentPlaygroundDemo>,
    mut selected_index: Signal<usize>,
    mut selector_open: Signal<bool>,
) -> Element {
    rsx! {
        nav { class: "playground-nav",
            for (idx, demo) in demos.iter().copied().enumerate() {
                ComponentNavButton { demo, idx, selected_index, selector_open }
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
    idx: usize,
    mut selected_index: Signal<usize>,
    mut selector_open: Signal<bool>,
) -> Element {
    let is_selected = selected_index() == idx;
    let class = if is_selected {
        "nav-button nav-button-selected"
    } else {
        "nav-button nav-button-default"
    };

    rsx! {
        button {
            class,
            r#type: "button",
            onclick: move |_| {
                selected_index.set(idx);
                selector_open.set(false);
            },
            "{demo.descriptor.name}"
        }
    }
}
