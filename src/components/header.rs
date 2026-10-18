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

/// Where a [`Header`]'s toolbar goes on a wide header.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ToolbarPlacement {
    /// In its own row, moving into the title row once the header is `64rem`
    /// wide. Suits a toolbar of a few segments or a search field.
    #[default]
    Auto,
    /// Always in its own row below the title, as in Ionic. For a toolbar too
    /// wide for the middle of a single row, such as several segment groups.
    Below,
}

/// The top bar of a page: a title between `start` and `end` slots, with an
/// optional toolbar row below. Like Ionic's `ion-header` with an
/// `ion-toolbar`.
///
/// Where the header itself is `64rem` or wider, the toolbar moves into the
/// title row, centred between the slots and at most `30rem` wide. It measures
/// its own width, not the shell's, so a navigation rail beside it or a narrow
/// sheet keeps the two rows. A toolbar too wide for that middle column takes
/// [`ToolbarPlacement::Below`] to keep its own row at every width.
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
    /// The page title, rendered as the page's `h1`. With no title, title
    /// content or title end, the title column stays for layout but is not a
    /// heading: an empty `h1` would announce a blank page title.
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
    /// Where the toolbar goes on a wide header. Defaults to
    /// [`ToolbarPlacement::Auto`], which moves it into the title row.
    toolbar_placement: Option<ToolbarPlacement>,
    /// Fill with a color role, like Ionic's `color` on a toolbar: the bar
    /// takes the color and its content the matching contrast. Unset keeps the
    /// theme surface.
    color: Option<crate::Color>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the `header` element.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let title = title.filter(|title| !title.trim().is_empty());
    let has_title = title.is_some() || title_content.is_some() || title_end.is_some();
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
        header {
            class: merge_classes(header_cls, class.as_deref()),
            "data-color": color.map(crate::Color::as_str),
            "data-toolbar-placement": (toolbar_placement == Some(ToolbarPlacement::Below)).then_some("below"),
            // Lays the row and toolbar out as one row on a wide header. The
            // header is the size container, so the grid sits one level in.
            div { class: "g3-header-bar",
                div { class: "g3-header-row",
                    div { class: "g3-header-lead",
                        div { class: "g3-header-slot g3-header-start-slot", {start} }
                        if has_title {
                            h1 { class: "g3-header-title",
                                if let Some(content) = title_content {
                                    {content}
                                } else if let Some(title) = title {
                                    span { class: "g3-header-title-text", "{title}" }
                                }
                                {title_end}
                            }
                        } else {
                            div { class: "g3-header-title" }
                        }
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
}

#[cfg(feature = "playground")]
#[component]
fn HeaderPlaygroundDemo() -> Element {
    let color = use_signal(|| crate::Color::Accent);
    let filled = use_signal(|| false);
    let tab = use_signal(|| 0_usize);
    let title = use_signal(|| "Pending Game".to_string());
    let toolbar = use_signal(|| true);
    let toolbar_below = use_signal(|| false);
    let show_start = use_signal(|| true);
    let show_end = use_signal(|| true);
    let mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Input { label: "Title", value: title }
                crate::Checkbox { checked: filled, label: "Color fill" }
                if filled() {
                    crate::components::color::ColorSelect { value: color }
                }
                crate::Checkbox { checked: show_start, label: "Back button" }
                crate::Checkbox { checked: show_end, label: "End action" }
                crate::Checkbox { checked: toolbar, label: "Toolbar" }
                if toolbar() {
                    crate::Checkbox { checked: toolbar_below, label: "Toolbar below at every width" }
                }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                Header {
                    color: filled().then_some(color()),
                    title: title(),
                    start: show_start().then(|| rsx! {
                        crate::BackButton { onclick: |_| {} }
                    }),
                    end: show_end().then(|| rsx! {
                        crate::Button { fill: crate::ButtonFill::Clear, "Create" }
                    }),
                    toolbar_placement: if toolbar_below() { ToolbarPlacement::Below } else { ToolbarPlacement::Auto },
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
