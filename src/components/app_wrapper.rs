//! The root of a g3-ui app.
use super::overlay_host::{OverlayHost, use_provide_overlay_queues};
#[cfg(not(target_arch = "wasm32"))]
use crate::UI_CSS;
use crate::theme::{ComponentMode, Strings, Theme, classes, merge_classes, use_provide_ambient};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use g3_route_transitions::{ROUTE_TRANSITION_OVERLAY_REGION_CLASS, RouteTransitionStyles};

/// The root of a g3-ui app: loads the stylesheet, applies the mode, theme, and
/// strings, and lays out a full-height, responsive app shell.
///
/// Put everything else inside it. It also hosts the overlays opened from code
/// with [`use_toast`](crate::use_toast), [`use_alert`](crate::use_alert), and
/// [`use_action_sheet`](crate::use_action_sheet).
///
/// The shell measures its own width with a container query. At `48rem` and
/// wider, a [`TabLayout`](crate::TabLayout) moves its navigation to a side
/// rail, bottom sheets float, and dialogs widen.
///
/// ```rust,ignore
/// rsx! {
///     AppWrapper { theme: Theme::system(),
///         Header { title: "Games" }
///         Content { Card { title: "Pending game", "Invite players." } }
///     }
/// }
/// ```
#[component]
pub fn AppWrapper(
    /// Platform look. Defaults to an enclosing provider's mode, then
    /// [`get_mode`](crate::get_mode).
    mode: Option<ComponentMode>,
    /// Colour tokens. Defaults to an enclosing provider's theme, then
    /// [`Theme::default_light`]. Changing it re-themes the app in place.
    theme: Option<Theme>,
    /// Component text. Defaults to an enclosing provider's strings, then
    /// English.
    strings: Option<Strings>,
    /// Apply the full-height app shell layout. Defaults to `true`. Set `false`
    /// for a page that scrolls normally and only needs the stylesheet, mode,
    /// and theme.
    layout: Option<bool>,
    /// Allow text selection in the shell. Defaults to `true`. Turn it off in
    /// apps built around drag gestures; inputs stay selectable.
    text_selection: Option<bool>,
    /// Whether this wrapper is the route-transition overlay region, the part
    /// that rises for routed sheets. Defaults to `true`. Set `false` on an
    /// outer wrapper that contains the real app's wrapper; a page may have
    /// only one overlay region. Needs the `transitions` feature.
    route_transition_overlay: Option<bool>,
    /// Extra classes for the shell element.
    class: Option<String>,
    children: Element,
) -> Element {
    let (mode, theme) = use_provide_ambient(mode, theme, strings);
    let queues = use_provide_overlay_queues();
    let layout = layout.unwrap_or(true);
    let overlay_region = cfg!(feature = "transitions") && route_transition_overlay.unwrap_or(true);
    #[cfg(feature = "transitions")]
    let overlay_cls = if overlay_region {
        ROUTE_TRANSITION_OVERLAY_REGION_CLASS
    } else {
        ""
    };
    #[cfg(not(feature = "transitions"))]
    let overlay_cls = {
        let _ = overlay_region;
        ""
    };
    let shell_cls = classes([
        "g3-app",
        if layout { "g3-app-shell" } else { "" },
        if text_selection.unwrap_or(true) {
            ""
        } else {
            "g3-no-select"
        },
        overlay_cls,
    ]);
    let shell = rsx! {
        div {
            class: merge_classes(shell_cls, class.as_deref()),
            style: theme.to_style_attr(),
            "data-g3-mode": mode.as_str(),
            {children}
            OverlayHost { queues }
        }
    };
    #[cfg(feature = "transitions")]
    return rsx! {
        StylesheetLink {}
        RouteTransitionStyles { {shell} }
    };
    #[cfg(not(feature = "transitions"))]
    rsx! {
        StylesheetLink {}
        {shell}
    }
}

/// Links [`UI_CSS`](crate::UI_CSS) where the build has not already put it in
/// the document head: desktop and mobile. On the web a second link would only
/// add a duplicate request.
#[component]
fn StylesheetLink() -> Element {
    #[cfg(target_arch = "wasm32")]
    {
        rsx! {}
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        rsx! {
            document::Link { rel: "stylesheet", href: UI_CSS }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn AppWrapperPlaygroundDemo() -> Element {
    let mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame { app: false,
            AppWrapper { mode, class: "g3-playground-device-app",
                crate::Header { title: "Games" }
                crate::Content { footer_space: false,
                    crate::Card { title: "Shell", "Mode and theme come from the playground controls." }
                    crate::Card { title: "Pending", "A full app shell with header and content." }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "AppWrapper",
    description: "Root app shell: stylesheet, mode, theme, strings, and overlay host.",
    demo: AppWrapperPlaygroundDemo,
    source: "src/components/app_wrapper.rs",
}
