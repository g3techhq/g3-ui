//! The scrollable content area of a page.
use crate::components::Spinner;
use crate::theme::{merge_classes, use_strings};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use g3_route_transitions::ROUTE_TRANSITION_SEGMENT_CLASS;

/// The scrollable content area of a page, below its [`Header`](crate::Header).
/// Like Ionic's `ion-content`.
///
/// It catches suspended children with a loading spinner and failed children
/// with an error message, so one failing resource does not take down the page.
///
/// ```rust,ignore
/// rsx! {
///     Header { title: "Rounds" }
///     Content {
///         fab: rsx! { Fab { FabButton { aria_label: "New round", Plus {} } } },
///         RoundList {}
///     }
/// }
/// ```
#[component]
pub fn Content(
    /// Pad the content. Defaults to `true`; the padding grows on wide shells.
    padding: Option<bool>,
    /// Leave space after the last child so it clears a bottom tab bar or FAB.
    /// Defaults to `true`.
    footer_space: Option<bool>,
    /// A [`Fab`](crate::Fab) pinned over the content rather than scrolling
    /// with it.
    fab: Option<Element>,
    /// Shown while a child is suspended. Defaults to a centred [`Spinner`].
    loading: Option<Element>,
    /// Shown when a child fails to render. Defaults to
    /// [`Strings::load_error`](crate::Strings::load_error).
    error: Option<Callback<ErrorContext, Element>>,
    /// Extra classes for the outer element.
    class: Option<String>,
    children: Element,
) -> Element {
    let strings = use_strings();
    let padding = padding.unwrap_or(true);
    #[cfg(feature = "transitions")]
    let scroll_cls = merge_classes("g3-content-scroll", Some(ROUTE_TRANSITION_SEGMENT_CLASS));
    #[cfg(not(feature = "transitions"))]
    let scroll_cls = "g3-content-scroll";
    let load_error = strings.load_error;
    rsx! {
        div { class: merge_classes("g3-content", class.as_deref()),
            div { class: scroll_cls, "data-padding": padding.to_string(),
                ErrorBoundary {
                    handle_error: move |context: ErrorContext| match error {
                        Some(error) => error.call(context),
                        None => rsx! {
                            div { class: "g3-content-error", role: "alert", "{load_error}" }
                        },
                    },
                    SuspenseBoundary {
                        fallback: move |_| match loading.clone() {
                            Some(loading) => loading,
                            None => rsx! {
                                Spinner { center: true }
                            },
                        },
                        {children}
                        if footer_space.unwrap_or(true) {
                            div { class: "g3-content-footer-spacer" }
                        }
                    }
                }
            }
            {fab}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ContentPlaygroundDemo() -> Element {
    let footer_space = use_signal(|| false);
    let padding = use_signal(|| true);
    let show_fab = use_signal(|| true);
    let mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Checkbox { checked: footer_space, label: "Footer space" }
                crate::Checkbox { checked: padding, label: "Padding" }
                crate::Checkbox { checked: show_fab, label: "FAB" }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                crate::Header { title: "Content" }
                Content {
                    footer_space: footer_space(),
                    padding: padding(),
                    fab: show_fab().then(|| rsx! {
                        crate::Fab {
                            crate::FabButton { aria_label: "Add", "+" }
                        }
                    }),
                    for index in 1..=12 {
                        crate::Card { title: format!("Hole {index}"),
                            "Scrollable content with enough rows to overflow."
                        }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Content",
    description: "Scrollable page content with loading and error fallbacks.",
    demo: ContentPlaygroundDemo,
    source: "src/components/content.rs",
}
