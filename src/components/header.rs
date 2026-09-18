//! The top bar of a page.
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Marks content rendered inside a header's toolbar row, so a
/// [`SegmentGroup`](crate::SegmentGroup) there can use its toolbar layout.
#[derive(Clone, Copy)]
pub(crate) struct HeaderToolbarContext;

#[component]
fn ToolbarContextProvider(children: Element) -> Element {
    use_hook(|| provide_context(HeaderToolbarContext));
    rsx! {
        {children}
    }
}

/// The top bar of a page: a title between `start` and `end` slots, with an
/// optional toolbar row below. Like Ionic's `ion-header` with an
/// `ion-toolbar`.
///
/// On shells `64rem` and wider, the toolbar moves into the title row.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Clone, Copy, PartialEq)] enum Tab { Card, Stats }
/// # let tab = use_signal(|| Tab::Card);
/// rsx! {
///     Header {
///         title: "Round",
///         start: rsx! { BackButton {} },
///         end: rsx! { Button { fill: ButtonFill::Clear, "Save" } },
///         toolbar: rsx! {
///             SegmentGroup { value: tab,
///                 SegmentButton { value: Tab::Card, "Card" }
///                 SegmentButton { value: Tab::Stats, "Stats" }
///             }
///         },
///     }
/// }
/// # }
/// ```
#[component]
pub fn Header(
    /// The page title, rendered as the page's `h1`.
    title: Option<String>,
    /// Custom title content, used instead of `title`.
    title_content: Option<Element>,
    /// Rendered right after the title, such as a status badge.
    title_end: Option<Element>,
    /// Leading slot, usually a [`BackButton`](crate::BackButton) or menu button.
    start: Option<Element>,
    /// Trailing slot for page actions.
    end: Option<Element>,
    /// A second row, usually a [`SegmentGroup`](crate::SegmentGroup) or
    /// [`Searchbar`](crate::Searchbar).
    toolbar: Option<Element>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the `header` element.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let header_cls = classes([
        "g3-header",
        mode.pick("g3-header-ios", "g3-header-md"),
        if toolbar.is_some() {
            "g3-header-with-toolbar"
        } else {
            ""
        },
    ]);
    rsx! {
        header { class: merge_classes(header_cls, class.as_deref()),
            div { class: "g3-header-row",
                div { class: "g3-header-slot g3-header-start-slot", {start} }
                h1 { class: "g3-header-title",
                    if let Some(content) = title_content {
                        {content}
                    } else if let Some(title) = title {
                        span { class: "g3-header-title-text", "{title}" }
                    }
                    {title_end}
                }
                div { class: "g3-header-slot g3-header-end-slot", {end} }
            }
            if let Some(toolbar) = toolbar {
                div { class: "g3-header-toolbar",
                    ToolbarContextProvider { {toolbar} }
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn HeaderPlaygroundDemo() -> Element {
    let tab = use_signal(|| 0_usize);
    let title = use_signal(|| "Pending Game".to_string());
    let toolbar = use_signal(|| true);
    let show_start = use_signal(|| true);
    let show_end = use_signal(|| true);
    let mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Input { label: "Title", value: title }
                crate::Checkbox { checked: show_start, label: "Back button" }
                crate::Checkbox { checked: show_end, label: "End action" }
                crate::Checkbox { checked: toolbar, label: "Toolbar" }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                Header {
                    title: title(),
                    start: show_start().then(|| rsx! {
                        crate::BackButton { onclick: |_| {} }
                    }),
                    end: show_end().then(|| rsx! {
                        crate::Button { fill: crate::ButtonFill::Clear, "Create" }
                    }),
                    toolbar: toolbar().then(|| rsx! {
                        crate::SegmentGroup { value: tab,
                            crate::SegmentButton { value: 0_usize, "Players" }
                            crate::SegmentButton { value: 1_usize, "Bets" }
                        }
                    }),
                }
                crate::Content { footer_space: false,
                    crate::Card { title: "Preview", "The toolbar stays attached to the header." }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Header",
    description: "Page top bar with start, title, end, and toolbar slots.",
    components: ["Header", "BackButton"],
    demo: HeaderPlaygroundDemo,
    source: "src/components/header.rs",
}
