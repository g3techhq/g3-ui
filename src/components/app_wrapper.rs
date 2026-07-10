//! AppWrapper component - root shell for the entire app layout.

use super::shell_styles as s;
use crate::UI_CSS;
use crate::theme::{
    ComponentMode, G3Mode, G3PreloadStyle, Theme, merge_classes, use_component_mode,
    use_css_preload_guard,
};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use dx_route_transitions::{ROUTE_TRANSITION_COVER_CLASS, RouteTransitionProvider};

#[component]
pub fn AppWrapper(
    children: Element,
    class: Option<String>,
    mode: Option<ComponentMode>,
    theme: Option<Theme>,
) -> Element {
    let mode = use_component_mode(mode);
    let theme_style = theme.as_ref().map(Theme::to_style_attr).unwrap_or_default();
    provide_context(G3Mode { mode });
    provide_context(theme.unwrap_or_default());

    // The stylesheet is attached at runtime (below), so on first load the DOM is
    // painted before it applies. Without this, overlays (sheets, modals) would
    // render fully unstyled (and, once the transform rules do land, visibly
    // slide out of their unstyled position). Hold a `g3-preload` class - backed
    // by the inline `G3PreloadStyle` guard, since g3_ui.css hasn't loaded yet
    // during this exact window - until the stylesheet is confirmed applied.
    let preloading = use_css_preload_guard();

    let mut shell_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::SHELL_BASE, s::SHELL_IOS),
        ComponentMode::Md => format!("{} {}", s::SHELL_BASE, s::SHELL_MD),
    };
    if preloading() {
        shell_cls = format!("{shell_cls} {}", s::SHELL_PRELOAD);
    }
    #[cfg(feature = "transitions")]
    let shell_cls = merge_classes(shell_cls, Some(ROUTE_TRANSITION_COVER_CLASS));
    let shell_cls = merge_classes(shell_cls, class.as_deref());

    let shell = rsx! {
        div {
            class: shell_cls,
            style: theme_style,
            "data-g3-mode": mode.as_str(),
            {children}
        }
    };

    #[cfg(feature = "transitions")]
    return rsx! {
        G3PreloadStyle {}
        document::Link { rel: "stylesheet", href: UI_CSS }
        RouteTransitionProvider { {shell} }
    };

    #[cfg(not(feature = "transitions"))]
    rsx! {
        G3PreloadStyle {}
        document::Link { rel: "stylesheet", href: UI_CSS }
        {shell}
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn AppWrapperPlaygroundDemo() -> Element {
    let playground_mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame { app: false,
            crate::AppWrapper { mode: playground_mode, class: "g3-playground-device-app",
                crate::Header { title: "Games" }
                crate::Body { has_footer_space: false,
                    crate::Card { title: "Shell", "Mode and theme come from the playground controls." }
                    crate::Card { title: "Pending", "A full app shell with header and body." }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "AppWrapper",
    description: "Root app shell and mode provider.",
    demo: AppWrapperPlaygroundDemo,
    source: "src/components/app_wrapper.rs",
}
