//! The root of a g3-ui app.
use super::overlay::js_string;
use super::overlay_host::{OverlayHost, use_provide_overlay_queues};
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
///
/// This is always the whole app's width. For three stages, or for the width
/// left inside a [`LayoutScope`](crate::LayoutScope), use
/// [`use_layout_size`](crate::use_layout_size).
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
const SHELL_SIZE_SCRIPT: &str = super::responsive::SIZE_SCRIPT;

/// Mirrors the active theme's page background and color scheme onto the
/// document element.
///
/// The theme is otherwise inline on the shell, which is enough for everything
/// the shell paints. The document canvas is not: it shows through wherever the
/// shell does not paint, and rules that resolve against `:root` never see the
/// shell's values. Two of those matter. `g3-route-transitions` paints
/// `html, body` from `--route-transition-bg`, which g3-ui maps to
/// `--g3-color-bg`; and every `::view-transition-*` pseudo-element is a child
/// of `:root`, so it resolves the same variable there. Left alone, both take
/// the stylesheet's light default, so a dark theme paints a near-white page
/// behind the app and flashes near-white through every route transition.
const ROOT_THEME_SCRIPT: &str = r#"
const root = document.documentElement;
root.style.setProperty("--g3-color-bg", __BG__);
root.style.setProperty("color-scheme", __SCHEME__);
"#;

/// The bottom bars inside one app shell, counted so its bottom toasts can sit
/// above them. `always` counts [`NavBar`](crate::NavBar)s; `compact` counts
/// [`AdaptiveNav`](crate::AdaptiveNav)s, which are a bar only on a compact
/// shell.
#[derive(Clone, Copy)]
pub(crate) struct BottomBars {
    always: Signal<u32>,
    compact: Signal<u32>,
}

impl BottomBars {
    fn count(self, always: bool) -> Signal<u32> {
        if always { self.always } else { self.compact }
    }
}

/// Counts the calling nav in its app shell while it is a bottom bar:
/// `Some(true)` at every width, `Some(false)` on compact shells, `None` never.
pub(crate) fn use_bottom_bar(bar: Option<bool>) {
    let bars = use_hook(try_consume_context::<BottomBars>);
    let bar = crate::state::use_synced_signal(bar);
    let held = use_hook(|| std::rc::Rc::new(std::cell::Cell::new(None::<bool>)));
    {
        let held = held.clone();
        use_effect(move || {
            let next = bar();
            let Some(bars) = bars else { return };
            let previous = held.replace(next);
            if previous == next {
                return;
            }
            if let Some(always) = previous {
                bars.count(always).with_mut(|n| *n = n.saturating_sub(1));
            }
            if let Some(always) = next {
                bars.count(always).with_mut(|n| *n += 1);
            }
        });
    }
    use_drop(move || {
        // The shell may be going away with this nav; its counts then no
        // longer matter.
        if let (Some(bars), Some(always)) = (bars, held.take())
            && let Ok(mut n) = bars.count(always).try_write()
        {
            *n = n.saturating_sub(1);
        }
    });
}

/// Marks that an enclosing [`AppWrapper`] already owns the document element, so
/// a nested wrapper - a demo frame inside a gallery, say - leaves it alone.
#[derive(Clone, Copy)]
struct RootShellClaimed;

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
    /// [`get_mode`](crate::get_mode). Seeds writable [`crate::use_mode`] state;
    /// later prop changes replace it. Omit to share the enclosing provider's mode.
    ///
    /// Set it in an app rendered on a server. Without it the server renders
    /// Material while an iPhone's browser detects iOS, and the hydrated page
    /// keeps the server's markup.
    mode: Option<ComponentMode>,
    /// Color tokens. Defaults to an enclosing provider's theme, then
    /// [`Theme::default_light`]. Seeds the writable [`crate::use_theme`] state;
    /// later prop changes replace that state and re-theme the app in place.
    theme: Option<Theme>,
    /// Component text. Defaults to an enclosing provider's strings, then
    /// English. Seeds writable [`crate::use_strings`] state; later prop changes
    /// replace it. Omit to share the enclosing provider's labels.
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
    let bars = use_context_provider(|| BottomBars {
        always: Signal::new(0),
        compact: Signal::new(0),
    });
    // Read by the stylesheet, which lifts bottom toasts above the bar.
    let bar_attr = if (bars.always)() > 0 {
        Some("always")
    } else if (bars.compact)() > 0 {
        Some("compact")
    } else {
        None
    };
    let shell_id = use_element_id("app", None);
    let mut shell_size = use_signal(ShellSize::default);
    use_context_provider(|| shell_size);
    let mut layout_size = use_signal(super::responsive::LayoutSize::default);
    use_context_provider(|| layout_size);
    super::script::use_dom_script(
        Some(SHELL_SIZE_SCRIPT.replace("__ID__", &js_string(&shell_id))),
        move |report: u8| {
            let next = if report >= 1 {
                ShellSize::Wide
            } else {
                ShellSize::Compact
            };
            if *shell_size.peek() != next {
                shell_size.set(next);
            }
            layout_size.set(super::responsive::LayoutSize::from_report(report));
        },
    );
    // Checked before the marker is provided, so only the outermost wrapper
    // claims the document element.
    let is_root_shell = use_hook(|| try_consume_context::<RootShellClaimed>().is_none());
    use_context_provider(|| RootShellClaimed);
    // Reruns when the theme's page color or scheme changes, and only then.
    let root_colors =
        crate::state::use_synced_signal((theme.bg.clone(), theme.color_scheme.clone()));
    use_effect(move || {
        if !is_root_shell {
            return;
        }
        let (root_bg, root_scheme) = root_colors();
        let script = ROOT_THEME_SCRIPT
            .replace("__BG__", &js_string(&root_bg))
            .replace("__SCHEME__", &js_string(&root_scheme));
        spawn(async move {
            let _ = document::eval(&script).await;
        });
    });
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
    let look_css = theme.scoped_look_css(&shell_id, !is_root_shell);
    let shell = rsx! {
        div {
            id: shell_id,
            class: merge_classes(shell_cls, class.as_deref()),
            style: theme.style_attr(!is_root_shell),
            "data-g3-mode": mode.as_str(),
            "data-g3-bar": bar_attr,
            if let Some(css) = look_css {
                style { dangerous_inner_html: css }
            }
            {children}
            OverlayHost { queues }
        }
    };
    // Head elements come from the outermost wrapper only: a nested one, such
    // as a demo frame, would link the whole stylesheet into the cascade again.
    // `is_root_shell` is decided by context, so the server and the hydrating
    // client render the same head.
    if !is_root_shell {
        return shell;
    }
    #[cfg(feature = "transitions")]
    return rsx! {
        StylesheetLink {}
        ViewportMeta {}
        RouteTransitionStyles { {shell} }
    };
    #[cfg(not(feature = "transitions"))]
    rsx! {
        StylesheetLink {}
        ViewportMeta {}
        {shell}
    }
}

/// Repeats Dioxus's own viewport tag with `viewport-fit=cover` added; the
/// later tag wins. Without it a WebView reports every
/// `env(safe-area-inset-*)` as 0, and on Android 15, which draws apps
/// edge-to-edge, the status bar then covers the header.
#[component]
fn ViewportMeta() -> Element {
    rsx! {
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1.0, viewport-fit=cover",
        }
    }
}

/// Links [`UI_CSS`](crate::UI_CSS) for desktop and mobile, where the build has
/// not already put it in the document head.
///
/// Rendered on every build, web included, even though a web page already has
/// the static-head link. Each head element takes a slot in the fullstack
/// hydration data: a fullstack server (native) that renders this link while
/// the wasm client does not shifts every later slot by one, so the client
/// reads a head element's `true` where an `ErrorBoundary` expects its
/// `Option<CapturedError>`. The markup must match; the hydrating client
/// skips re-inserting a link the server already wrote.
#[component]
fn StylesheetLink() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: UI_CSS }
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
