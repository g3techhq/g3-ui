//! AppWrapper component - root shell for the entire app layout.

use super::shell_styles as s;
use crate::UI_CSS;
use crate::theme::{
    ComponentMode, G3Mode, Theme, get_mode, merge_classes, use_ancestor_context, use_context_signal,
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
    /// Whether to apply the responsive app-shell layout (`flex flex-col h-dvh
    /// overflow-hidden` + mode class). Defaults to `true`. Set to `false` for
    /// a root that provides its own top-level layout (e.g. a desktop page
    /// that should scroll normally) and only wants `AppWrapper` for theme
    /// tokens, the stylesheet link, and the first-paint guard.
    layout: Option<bool>,
    /// Disable text selection/highlighting across the whole app shell
    /// (`input`/`textarea` are exempted so typing still works). Useful for
    /// apps with drag gestures (reorder, swipe, scrubbing) where an
    /// accidental text selection during a drag is visually distracting.
    /// Defaults to `false`.
    disable_text_selection: Option<bool>,
) -> Element {
    // AppWrapper both consumes and re-provides mode and theme, so it has to
    // resolve them from an *ancestor* rather than with the ordinary context
    // helpers - otherwise it reads back its own published value from its second
    // render onward and never sees an outer change. Reading through the
    // inherited signals still subscribes, so a live mode or theme switch
    // re-renders this shell and everything under it.
    let inherited_mode = use_ancestor_context::<G3Mode>();
    let inherited_theme = use_ancestor_context::<Signal<Theme>>();
    let mode = mode
        .or_else(|| inherited_mode.map(|context| (context.mode)()))
        .unwrap_or_else(get_mode);
    // Nested AppWrapper (a device-frame demo inside a themed page, for
    // instance) shouldn't lose its ambient theme just because it wasn't
    // re-specified: fall back to whatever Theme an outer AppWrapper already
    // provided before reaching for the library default. This has to resolve
    // to a concrete Theme either way (not fall through to an empty inline
    // style) - `[data-g3-mode]` sets its own default color vars, and those
    // beat an *inherited* value on any element that carries the attribute,
    // which every AppWrapper does.
    let effective_theme = theme
        .or_else(|| inherited_theme.map(|theme| theme()))
        .unwrap_or_default();
    let theme_style = effective_theme.to_style_attr();
    provide_context(G3Mode {
        mode: use_context_signal(mode),
    });
    provide_context(use_context_signal(effective_theme));

    let mut shell_cls = if layout.unwrap_or(true) {
        match mode {
            ComponentMode::Ios => format!("{} {}", s::SHELL_BASE, s::SHELL_IOS),
            ComponentMode::Md => format!("{} {}", s::SHELL_BASE, s::SHELL_MD),
        }
    } else {
        String::new()
    };
    if disable_text_selection.unwrap_or(false) {
        shell_cls = format!("{shell_cls} {}", s::SHELL_NO_SELECT);
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

    // `with_static_head` puts this in the document head for web builds, which
    // is what blocks first paint and prevents a flash of unstyled content.
    // Desktop and mobile bundles do not collect statically-headed assets at
    // all, though - only ones something links at runtime - so this tag is what
    // gets the stylesheet into those builds. On web it resolves to the same URL
    // the head already loaded, so it costs a cache hit and nothing else.
    #[cfg(feature = "transitions")]
    return rsx! {
        document::Link { rel: "stylesheet", href: UI_CSS }
        RouteTransitionProvider { {shell} }
    };

    #[cfg(not(feature = "transitions"))]
    rsx! {
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
