//! The scrollable content area of a page.
use crate::components::{Refresher, Spinner};
use crate::theme::{merge_classes, use_strings};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use g3_route_transitions::ROUTE_TRANSITION_SEGMENT_CLASS;

/// How wide [`Content`] lets its children grow.
///
/// A limit only shows once the page is wider than it. The children are
/// centred in a column of at most that width, and the rest of the page is
/// empty margin; the scroll area, and its scrollbar, still span the page. On
/// a phone every option looks the same.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ContentWidth {
    /// No limit: children span the page, less its padding. Suits lists,
    /// maps, and tables that use every column.
    #[default]
    Full,
    /// At most 40rem (640px), about 70 characters of body text a line. Suits
    /// articles, settings, and forms, which are hard to read when stretched
    /// across a monitor.
    Readable,
    /// At most 60rem (960px). Suits dashboards and card grids that should
    /// grow on a laptop but not sprawl across a large monitor.
    Wide,
}

impl ContentWidth {
    fn as_str(self) -> &'static str {
        match self {
            ContentWidth::Full => "full",
            ContentWidth::Readable => "readable",
            ContentWidth::Wide => "wide",
        }
    }
}

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
    /// The widest the children may grow; see [`ContentWidth`]. Defaults to
    /// [`ContentWidth::Full`]. The scroll area, and its scrollbar, always span
    /// the whole page.
    width: Option<ContentWidth>,
    /// Leave space after the last child so it clears a bottom tab bar or FAB.
    /// Defaults to `true`.
    footer_space: Option<bool>,
    /// A [`Fab`](crate::Fab) pinned over the content rather than scrolling
    /// with it.
    fab: Option<Element>,
    /// Turn on pull to refresh: called when the user pulls down from the top
    /// of the page. See [`Refresher`].
    on_refresh: Option<EventHandler<()>>,
    /// Whether a refresh started by `on_refresh` is still running. The
    /// indicator stays until this is `false`.
    refreshing: Option<bool>,
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
            div {
                class: scroll_cls,
                "data-padding": padding.to_string(),
                "data-width": width.unwrap_or_default().as_str(),
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
                        if let Some(on_refresh) = on_refresh {
                            Refresher { refreshing: refreshing.unwrap_or(false), on_refresh,
                                {children}
                                if footer_space.unwrap_or(true) {
                                    div { class: "g3-content-footer-spacer" }
                                }
                            }
                        } else {
                            {children}
                            if footer_space.unwrap_or(true) {
                                div { class: "g3-content-footer-spacer" }
                            }
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
    let width = use_signal(|| ContentWidth::Readable);
    let pull = use_signal(|| true);
    let mut refreshing = use_signal(|| false);
    let mut column = use_signal(|| 0.0_f64);
    let mode = crate::use_component_mode(None);
    let explain = match width() {
        ContentWidth::Full => "Full: no limit. The cards span the page, less its padding.",
        ContentWidth::Readable => {
            "Readable: at most 640px, centred. Visible on the Desktop viewport, which is about 790px wide."
        }
        ContentWidth::Wide => {
            "Wide: at most 960px, centred. The Desktop viewport is narrower than that, so it matches Full here; on a large monitor it would not."
        }
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Checkbox { checked: footer_space, label: "Footer space" }
                crate::Checkbox { checked: padding, label: "Padding" }
                crate::Checkbox { checked: show_fab, label: "FAB" }
                crate::Checkbox { checked: pull, label: "Pull to refresh" }
                crate::SegmentGroup { value: width, aria_label: "Width",
                    crate::SegmentButton { value: ContentWidth::Full, "Full" }
                    crate::SegmentButton { value: ContentWidth::Readable, "Readable" }
                    crate::SegmentButton { value: ContentWidth::Wide, "Wide" }
                }
                crate::Text { variant: crate::TextVariant::Caption, tone: crate::TextTone::Secondary,
                    "{explain} The scrollbar stays at the page edge either way."
                }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                crate::Header { title: "Content" }
                Content {
                    footer_space: footer_space(),
                    padding: padding(),
                    width: width(),
                    refreshing: refreshing(),
                    on_refresh: pull().then(|| EventHandler::new(move |_| {
                        refreshing.set(true);
                        spawn(async move {
                            dioxus_sdk_time::sleep(std::time::Duration::from_millis(1000)).await;
                            refreshing.set(false);
                        });
                    })),
                    fab: show_fab().then(|| rsx! {
                        crate::Fab {
                            crate::FabButton { aria_label: "Add", "+" }
                        }
                    }),
                    div {
                        class: "playground-content-ruler",
                        onresize: move |event| {
                            if let Ok(size) = event.data().get_content_box_size() {
                                column.set(size.width);
                            }
                        },
                        if column() > 0.0 {
                            "Content column: {column():.0}px wide"
                        } else {
                            "Content column"
                        }
                    }
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
