//! The overview explains the library; Getting started covers installation.
use super::*;

#[component]
pub fn Landing() -> Element {
    let settings = use_context::<PlaygroundSettings>();
    let viewport = (settings.viewport)();
    rsx! {
        article { class: "playground-landing",
            section { class: "landing-hero", "data-viewport": viewport.as_str(),
                div {
                    h2 { "iOS and Material components" }
                    p { class: "landing-lead", "Form controls, navigation, overlays, and responsive layouts for Dioxus." }
                    p { "Use the same components in browsers and Android, iOS, and desktop web views." }
                    g3_ui::Stack { horizontal: true, wrap: true, class: "landing-actions",
                        Button { size: ButtonSize::Lg, to: Route::Guide { slug: "getting-started".into() }, "Get started" }
                        Button { fill: ButtonFill::Outline, size: ButtonSize::Lg, to: Route::ComponentDemo { slug: "button".into() }, "Components" }
                    }
                }
                LandingPreview {}
            }
            section { class: "landing-benefits", aria_label: "Features",
                g3_ui::Grid { columns: g3_ui::GridColumns::Fit(25.0),
                    g3_ui::Card { title: "Platform modes", heading_level: g3_ui::HeadingLevel::H3,
                        p { "Switch between iOS and Material styling for the app, a subtree, or an individual component. Controls support touch input and keyboard navigation." }
                    }
                    g3_ui::Card { title: "Responsive layouts", heading_level: g3_ui::HeadingLevel::H3,
                        p { "Fit grid columns to the available width, stack side panels, or mount different controls for compact, medium, and expanded layouts." }
                        Button { fill: ButtonFill::Outline, to: Route::Guide { slug: "responsive-layouts".into() }, "Responsive layouts" }
                    }
                    g3_ui::Card { title: "Themes", heading_level: g3_ui::HeadingLevel::H3,
                        p { "Set color and surface tokens, switch light and dark themes at runtime, and override the theme for a subtree. Edit theme, mode, and component labels through writable hooks." }
                        Button { fill: ButtonFill::Outline, to: Route::ThemeGenerator {}, "Theme generator" }
                    }
                    g3_ui::Card { title: "Overlay hooks", heading_level: g3_ui::HeadingLevel::H3,
                        p { "Show toasts, await alert confirmations or action sheet choices, and dismiss the top overlay before navigating back." }
                        Button { fill: ButtonFill::Outline, to: Route::Guide { slug: "hooks-overlays".into() }, "Overlay hooks" }
                    }
                }
            }
            section { class: "landing-next",
                h2 { "Installation" }
                p { "Add g3-ui to Cargo.toml and wrap the app in AppWrapper." }
                Button { to: Route::Guide { slug: "getting-started".into() }, "Set up g3-ui" }
            }
        }
    }
}

#[component]
fn LandingPreview() -> Element {
    let (mode, theme, viewport) = active_playground_settings();
    let demo = use_hook(|| {
        component_playground_demos()
            .into_iter()
            .find(|demo| demo.descriptor.name == "DemoApp")
            .expect("the landing page requires the Demo app gallery entry")
    });
    rsx! {
        div { class: "landing-device-stage",
            PlaygroundViewportDemo { demo, mode, theme, viewport }
        }
    }
}
