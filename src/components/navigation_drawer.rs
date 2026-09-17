//! Persistent navigation beside the page.
use super::sheet::{FrameKind, SheetBackdrop, SheetEdge, SheetFrame};
use crate::theme::ComponentMode;
use dioxus::prelude::*;

/// Navigation that stays open beside the page, like Ionic's split pane. The
/// page narrows to make room instead of being covered.
///
/// It is not a dialog: it has no backdrop, does not trap focus, does not lock
/// scrolling, and is not counted by [`open_sheet_count`](crate::open_sheet_count).
/// A menu button in the page usually toggles `open`.
///
/// Render it and one root page element as direct children of
/// [`AppWrapper`](crate::AppWrapper).
///
/// The rows of a drawer are its navigation, and they read best pressing from
/// edge to edge, so pass `padding: false` when the drawer is a list.
///
/// ```rust,ignore
/// rsx! {
///     AppWrapper {
///         NavigationDrawer { open: drawer_open, padding: false,
///             List { Item { label: "Rounds", to: Route::Rounds {} } }
///         }
///         Page {}
///     }
/// }
/// ```
#[component]
pub fn NavigationDrawer(
    /// Whether the drawer is shown.
    open: Signal<bool>,
    /// Which side it sits on. Defaults to [`SheetEdge::Start`].
    edge: Option<SheetEdge>,
    /// Accessible name. Defaults to
    /// [`Strings::navigation_drawer`](crate::Strings::navigation_drawer).
    aria_label: Option<String>,
    /// Drawer width as a CSS length. Defaults to `min(18rem, 72%)` of the
    /// shell.
    width: Option<String>,
    /// Pad the sheet's content. Defaults to `true`. Set it to `false` for a
    /// sheet whose rows run to its edges, the way
    /// [`Content`](crate::Content) does for a page.
    padding: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the drawer.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        SheetFrame {
            open,
            kind: FrameKind::Drawer {
                edge: edge.unwrap_or_default(),
                width,
            },
            backdrop: SheetBackdrop::None,
            aria_label,
            padding,
            mode,
            class,
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn NavigationDrawerPlaygroundDemo() -> Element {
    use dioxus_icons::lucide::Menu;
    let mut open = use_signal(|| true);
    let edge = use_signal(|| SheetEdge::Start);
    let padding = use_signal(|| false);
    let toggle = rsx! {
        crate::Button {
            fill: crate::ButtonFill::Clear,
            size: crate::ButtonSize::Sm,
            aria_label: "Toggle navigation",
            onclick: move |_| open.toggle(),
            Menu { size: 22 }
        }
    };
    let at_start = edge() == SheetEdge::Start;
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Checkbox { checked: open, label: "Open" }
                crate::SegmentGroup { value: edge, aria_label: "Side",
                    crate::SegmentButton { value: SheetEdge::Start, "Start (left)" }
                    crate::SegmentButton { value: SheetEdge::End, "End (right)" }
                }
                crate::Checkbox { checked: padding, label: "Pad the content" }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                NavigationDrawer { open, edge: edge(), padding: padding(),
                    crate::List { lines: crate::ListLines::None,
                        crate::Item { label: "Rounds", selected: true, onclick: |_| {} }
                        crate::Item { label: "Players", onclick: |_| {} }
                        crate::Item { label: "Settings", onclick: |_| {} }
                    }
                }
                div { class: "g3-sheet-demo-content-root",
                    crate::Header {
                        title: "Rounds",
                        start: at_start.then(|| toggle.clone()),
                        end: (!at_start).then_some(toggle),
                    }
                    crate::Content { footer_space: false,
                        crate::Card { title: "Split layout", "The page narrows beside the drawer." }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "NavigationDrawer",
    description: "Persistent navigation beside the page.",
    demo: NavigationDrawerPlaygroundDemo,
    source: "src/components/navigation_drawer.rs",
}
