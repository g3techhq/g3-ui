//! The root of a g3-ui app.
use super::overlay::js_string;
use super::overlay_host::{OverlayHost, use_provide_overlay_queues};
#[cfg(not(target_arch = "wasm32"))]
use crate::UI_CSS;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, Strings, Theme, classes, merge_classes, use_provide_ambient};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use g3_route_transitions::{ROUTE_TRANSITION_OVERLAY_REGION_CLASS, RouteTransitionStyles};

/// How wide the app shell is, in the two sizes the stylesheet lays out for.
///
/// The same `48rem` line the CSS uses, reported to Rust so a component can
/// branch on it where a container query cannot reach: choosing between two
/// components, picking how many items to ask the server for, deciding whether
/// a control is worth showing at all. Layout alone is better left to CSS,
/// which needs no measurement and cannot flicker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ShellSize {
    /// Narrower than `48rem`: a phone, or a window cut down to one.
    #[default]
    Compact,
    /// `48rem` and wider.
    Wide,
}

impl ShellSize {
    /// Whether this is [`ShellSize::Wide`].
    pub fn is_wide(self) -> bool {
        matches!(self, Self::Wide)
    }

    /// Whether this is [`ShellSize::Compact`].
    pub fn is_compact(self) -> bool {
        matches!(self, Self::Compact)
    }
}

/// The width class of the enclosing [`AppWrapper`]'s shell.
///
/// Reads [`ShellSize::Compact`] until the shell has been measured, and
/// anywhere outside an `AppWrapper`, so a server render and the first client
/// render agree on the narrower layout.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let size = use_shell_size();
/// rsx! {
///     if size.is_wide() {
///         Card { title: "Rounds", "Side by side." }
///     } else {
///         Card { title: "Rounds", "One column." }
///     }
/// }
/// # }
/// ```
pub fn use_shell_size() -> ShellSize {
    use_hook(try_consume_context::<Signal<ShellSize>>)
        .map(|size| size())
        .unwrap_or_default()
}

/// Watches the shell and reports which side of `48rem` it is on.
///
/// `rem` is read from the document rather than assumed to be 16px, so this
/// agrees with the container query even on a page that has resized its root.
const SHELL_SIZE_SCRIPT: &str = r#"
const shell = document.getElementById(__ID__);
if (shell && !shell.g3SizeObserver) {
    let last = null;
    const report = () => {
        const rem = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
        const wide = shell.getBoundingClientRect().width >= 48 * rem;
        if (wide !== last) {
            last = wide;
            dioxus.send(wide);
        }
    };
    shell.g3SizeObserver = new ResizeObserver(report);
    shell.g3SizeObserver.observe(shell);
    report();
}
"#;

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
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     AppWrapper { theme: Theme::system(),
///         Header { title: "Games" }
///         Content { Card { title: "Pending game", "Invite players." } }
///     }
/// }
/// # }
/// ```
#[component]
pub fn AppWrapper(
    /// Platform look. Defaults to an enclosing provider's mode, then
    /// [`get_mode`](crate::get_mode).
    mode: Option<ComponentMode>,
    /// Color tokens. Defaults to an enclosing provider's theme, then
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
    let shell_id = use_element_id("app", None);
    let mut shell_size = use_signal(ShellSize::default);
    use_context_provider(|| shell_size);
    {
        let shell_id = shell_id.clone();
        use_effect(move || {
            let script = SHELL_SIZE_SCRIPT.replace("__ID__", &js_string(&shell_id));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(wide) = eval.recv::<bool>().await {
                    shell_size.set(if wide {
                        ShellSize::Wide
                    } else {
                        ShellSize::Compact
                    });
                }
            });
        });
    }
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
            id: shell_id,
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
