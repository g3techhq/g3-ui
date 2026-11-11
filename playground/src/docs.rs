//! Navigation metadata and shared demo/source tools. Each guide owns its layout.
use super::*;

mod charts;

mod examples;

mod pages;

mod theme_hooks;

const CHARTS_SOURCE: &str = include_str!("docs/charts.rs");

const EXAMPLES_SOURCE: &str = include_str!("docs/examples.rs");

const THEME_HOOKS_SOURCE: &str = include_str!("docs/theme_hooks.rs");

/// Only information shared by the menu, search, and header belongs here.
pub struct GuideLink {
    pub slug: &'static str,
    pub title: &'static str,
    pub section: &'static str,
    pub summary: &'static str,
    pub keywords: &'static str,
    pub render: fn() -> Element,
}

pub const PAGES: &[GuideLink] = &[
    GuideLink {
        slug: "getting-started",
        title: "Getting started",
        section: "Guides",
        summary: "Install g3-ui, add an AppWrapper, and launch the app.",
        keywords: "install setup Cargo.toml Rust Dioxus Ionic AppWrapper Header Content stylesheet init_auto_mode launch",
        render: || rsx! { pages::GettingStarted {} },
    },
    GuideLink {
        slug: "responsive-layouts",
        title: "Responsive layouts",
        section: "Patterns",
        summary: "Choose readable grids, stable page layouts, or different controls.",
        keywords: "Grid GridColumns Fit PageLayout AdaptiveLayout LayoutScope Compact Medium Expanded shared state responsive resizing",
        render: || rsx! { pages::ResponsiveLayouts {} },
    },
    GuideLink {
        slug: "platforms",
        title: "Web and native web views",
        section: "Guides",
        summary: "Use the same components in a browser or a native web view.",
        keywords: "Android iOS desktop web view native mobile Cargo.toml safe areas keyboard back performance transitions",
        render: || rsx! { pages::Platforms {} },
    },
    GuideLink {
        slug: "hooks-layout",
        title: "Layout hooks",
        section: "Hooks",
        summary: "Measure a content area or the whole app.",
        keywords: "use_layout_size use_shell_size LayoutScope AppWrapper Compact Medium Expanded Wide breakpoints width container",
        render: || rsx! { pages::LayoutHooks {} },
    },
    GuideLink {
        slug: "hooks-overlays",
        title: "Overlay hooks",
        section: "Hooks",
        summary: "Show a toast, ask a question, or dismiss the top overlay.",
        keywords: "use_toast Toaster ToastOptions use_alert Alerts confirm use_action_sheet ActionSheets ActionSheetOptions use_overlay_controller dismiss_top count Android Back async await",
        render: || rsx! { pages::OverlayHooks {} },
    },
    GuideLink {
        slug: "hooks-theme",
        title: "Theme and mode hooks",
        section: "Hooks",
        summary: "Read and change shared colors, platform mode, and component labels.",
        keywords: "use_theme use_mode use_strings use_component_mode reveal_theme ThemeProvider AppWrapper Signal mut set write accent colors theme mode dark light switch reveal circle view transition localization Strings scoped provider",
        render: || rsx! { pages::ThemeHooks {} },
    },
    GuideLink {
        slug: "charts",
        title: "Charts",
        section: "Companion crates",
        summary: "Draw donuts, sunbursts, target bars, line charts and sparklines with g3-charts.",
        keywords: "g3-charts g3_charts chart charts graph Donut pie Sunburst allocation TargetBars bullet grouped drift LineChart line time series Sparkline trend ChartStyles ChartColor ValueFormat Slice Node TargetRow Series Point SVG",
        render: || rsx! { pages::Charts {} },
    },
];

pub fn page(slug: &str) -> Option<&'static GuideLink> {
    PAGES.iter().find(|page| page.slug == slug)
}

#[component]
pub fn Guide(slug: String) -> Element {
    rsx! {
        article { class: "playground-doc",
            if let Some(page) = page(&slug) {
                {(page.render)()}
            } else {
                p { "This guide could not be found." }
                Button { fill: ButtonFill::Outline, to: Route::Guide { slug: "getting-started".into() }, "Getting started" }
            }
        }
    }
}

/// Executable examples are shared by the viewer and source panels, not page prose.
#[derive(Clone, Copy, PartialEq)]
enum Example {
    GettingStarted,
    ResponsiveLayouts,
    Platforms,
    LayoutHooks,
    OverlayHooks,
    ThemeHooks,
    Donut,
    Sunburst,
    TargetBars,
    LineChart,
    Sparkline,
}

/// Every example, for the test that checks each shows compiled code.
#[cfg(test)]
const EXAMPLES: [Example; 11] = [
    Example::GettingStarted,
    Example::ResponsiveLayouts,
    Example::Platforms,
    Example::LayoutHooks,
    Example::OverlayHooks,
    Example::ThemeHooks,
    Example::Donut,
    Example::Sunburst,
    Example::TargetBars,
    Example::LineChart,
    Example::Sparkline,
];

impl Example {
    /// Drawn with g3-charts, so its code imports that crate too.
    fn is_chart(self) -> bool {
        matches!(
            self,
            Self::Donut | Self::Sunburst | Self::TargetBars | Self::LineChart | Self::Sparkline
        )
    }

    fn source(self) -> &'static str {
        match self {
            Self::ThemeHooks => THEME_HOOKS_SOURCE,
            _ if self.is_chart() => CHARTS_SOURCE,
            _ => EXAMPLES_SOURCE,
        }
    }

    fn functions(self) -> &'static [&'static str] {
        match self {
            Self::GettingStarted => &["GettingStartedExample"],
            Self::ResponsiveLayouts => &["ResponsiveLayoutsExample"],
            Self::Platforms => &["PlatformsExample"],
            Self::LayoutHooks => &["LayoutHooksExample", "LayoutHookValues"],
            Self::OverlayHooks => &["OverlayHooksExample", "OverlayHookActions"],
            Self::ThemeHooks => &[
                "ThemeHooksExample",
                "ThemeColorControls",
                "AppearanceControls",
                "RevealedThemeSwitch",
                "ThemedAccent",
                "LocalizedActions",
                "ModeAwareButton",
                "ScopedThemeExample",
                "LocalThemeEditor",
            ],
            Self::Donut => &["DonutExample"],
            Self::Sunburst => &["SunburstExample"],
            Self::TargetBars => &["TargetBarsExample"],
            Self::LineChart => &["LineChartExample"],
            Self::Sparkline => &["SparklineExample"],
        }
    }

    fn render(self) -> Element {
        match self {
            Self::GettingStarted => rsx! { examples::GettingStartedExample {} },
            Self::ResponsiveLayouts => rsx! { examples::ResponsiveLayoutsExample {} },
            Self::Platforms => rsx! { examples::PlatformsExample {} },
            Self::LayoutHooks => rsx! { examples::LayoutHooksExample {} },
            Self::OverlayHooks => rsx! { examples::OverlayHooksExample {} },
            Self::ThemeHooks => rsx! { theme_hooks::ThemeHooksExample {} },
            Self::Donut => rsx! { charts::DonutExample {} },
            Self::Sunburst => rsx! { charts::SunburstExample {} },
            Self::TargetBars => rsx! { charts::TargetBarsExample {} },
            Self::LineChart => rsx! { charts::LineChartExample {} },
            Self::Sparkline => rsx! { charts::SparklineExample {} },
        }
    }

    fn code(self) -> String {
        if self.is_chart() {
            // The shared header imports only g3-ui; chart examples need both.
            let imports = "use dioxus::prelude::*;
use g3_charts::prelude::*;
use g3_ui::prelude::*;
";
            return imports.to_string() + &function_code(self.source(), self.functions(), false);
        }
        function_code(self.source(), self.functions(), true)
    }
}

fn function_code(source: &'static str, functions: &[&str], imports: bool) -> String {
    let mut code = if imports {
        "use dioxus::prelude::*;\nuse g3_ui::prelude::*;\n".to_string()
    } else {
        String::new()
    };
    for name in functions {
        let function = g3_ui::function_source(source, &format!("fn {name}"))
            .unwrap_or_else(|| panic!("guide example function missing: {name}"));
        code.push('\n');
        code.push_str(function);
        code.push('\n');
    }
    code
}

#[component]
fn LiveExample(example: Example, label: String) -> Element {
    let (mode, theme, viewport) = active_playground_settings();

    rsx! {
        div { class: "playground-doc-preview-stage",
            section {
                class: "playground-doc-preview playground-viewport-card playground-viewport-{viewport.as_str()}",
                aria_label: "{label} live preview",
                div { class: "playground-viewport-demo",
                    ThemeProvider { mode, theme,
                        g3_ui::PlaygroundDemoFrame { app: false, {example.render()} }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_displayed_example_uses_its_compiled_functions() {
        for example in EXAMPLES {
            for name in example.functions() {
                let source = g3_ui::function_source(example.source(), &format!("fn {name}"))
                    .unwrap_or_else(|| panic!("missing compiled function: {name}"));
                assert!(source.starts_with("#[component]"));
                assert!(example.code().contains(source));
            }
        }
    }
}
