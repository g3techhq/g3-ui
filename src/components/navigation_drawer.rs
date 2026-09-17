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
/// ```rust,ignore
/// rsx! {
///     AppWrapper {
///         NavigationDrawer { open: drawer_open,
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
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                NavigationDrawer { open, edge: edge(),
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
