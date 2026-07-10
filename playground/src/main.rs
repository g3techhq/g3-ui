//! g3_ui playground - component-owned demos rendered in mobile context.

use dioxus::prelude::*;
use dioxus_code::{Code, CodeTheme, Language, SourceCode, Theme};
use g3_ui::{
    AppWrapper, ComponentMode, ComponentPlaygroundDemo, G3Theme, SegmentButton, SegmentGroup,
    Select, SelectOption, Sheet, SheetPlacement, component_playground_demos,
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

fn main() {
    g3_ui::init_auto_mode();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
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
    let source_value = use_signal(Vec::<String>::new);
    let theme_value = use_signal(|| PlaygroundTheme::Blue.label().to_string());
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

    let selected_source = playground_demo_source(selected.source);

    let theme_options = PlaygroundTheme::all()
        .iter()
        .copied()
        .map(|theme| SelectOption::from((theme.label(), theme.label())))
        .collect::<Vec<_>>();

    rsx! {
        AppWrapper {
            theme: active_theme.to_theme(),
            mode: active_mode,
            class: "playground-root",
            layout: false,
            g3_ui::Header {
                title: selected.descriptor.name.to_string(),
                mode: active_mode,
                class: "playground-header",
                start_button: rsx! {
                    g3_ui::Button {
                        style: g3_ui::ButtonStyle::Clear,
                        size: g3_ui::ButtonSize::Sm,
                        aria_label: "Open component menu".to_string(),
                        class: "playground-menu-button",
                        onclick: move |_| selector_open.set(true),
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
                    SegmentGroup { active: mode_index, mode: active_mode,
                        SegmentButton { index: 0, mode: active_mode, "MD" }
                        SegmentButton { index: 1, mode: active_mode, "iOS" }
                    }
                },
            }
            Sheet {
                is_open: selector_open,
                placement: SheetPlacement::Left,
                mode: active_mode,
                class: "playground-selector-sheet",
                div { class: "playground-sidebar-header",
                    h2 { "g3_ui" }
                    div { class: "subtitle", "{count} demos" }
                }
                PlaygroundNav {
                    demos: demos.clone(),
                    selected_index,
                    selector_open,
                }
            }
            main { class: "playground-main",
                p { class: "playground-description", "{selected.descriptor.description}" }
                div { class: "playground-stage",
                    div {
                        key: "{active_mode.as_str()}-{active_theme.label()}-{selected.descriptor.name}",
                        "data-g3-mode": active_mode.as_str(),
                        g3_ui::G3ThemeProvider { mode: active_mode, theme: active_theme.to_theme(),
                            RenderSelectedDemo {
                                key: "{active_mode.as_str()}-{active_theme.label()}-{selected.descriptor.name}",
                                demo: selected,
                                mode: active_mode,
                                theme: active_theme,
                            }
                        }
                    }
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
    mut selected_index: Signal<usize>,
    mut selector_open: Signal<bool>,
) -> Element {
    rsx! {
        g3_ui::List { class: "playground-nav", lines: g3_ui::ListLines::None,
            for (idx , demo) in demos.iter().copied().enumerate() {
                ComponentNavButton {
                    demo,
                    idx,
                    selected_index,
                    selector_open,
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
    idx: usize,
    mut selected_index: Signal<usize>,
    mut selector_open: Signal<bool>,
) -> Element {
    let is_selected = selected_index() == idx;

    rsx! {
        g3_ui::Item {
            kind: g3_ui::ItemKind::Button,
            selected: is_selected,
            label: demo.descriptor.name.to_string(),
            onclick: move |_| {
                selected_index.set(idx);
                selector_open.set(false);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::playground_demo_source;

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
}
