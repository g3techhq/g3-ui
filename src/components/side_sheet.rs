//! A sheet that slides in from a side of the screen.
use super::sheet::{FrameKind, SheetBackdrop, SheetEdge, SheetFrame, SideSheetBehavior};
use crate::theme::ComponentMode;
use dioxus::prelude::*;

/// A sheet that slides in from a side, for filters, inspectors, and
/// temporary navigation. Like Ionic's `ion-menu`.
///
/// For [`SideSheetBehavior::Push`] and [`SideSheetBehavior::Reveal`], render
/// the sheet and one root page element as direct children of
/// [`AppWrapper`](crate::AppWrapper); that page element is what moves.
///
/// For navigation that stays open beside the page, use
/// [`NavigationDrawer`](crate::NavigationDrawer).
///
/// ```rust,ignore
/// rsx! {
///     AppWrapper {
///         SideSheet { open: filters_open, edge: SheetEdge::End, title: "Filters",
///             FilterForm {}
///         }
///         Page {}
///     }
/// }
/// ```
#[component]
pub fn SideSheet(
    /// Whether the sheet is open.
    open: Signal<bool>,
    /// Which side it slides in from. Defaults to [`SheetEdge::Start`].
    edge: Option<SheetEdge>,
    /// How the page reacts. Defaults to [`SideSheetBehavior::Overlay`].
    behavior: Option<SideSheetBehavior>,
    /// Heading shown at the top, which also names the dialog.
    title: Option<String>,
    /// Accessible name when there is no `title`. Defaults to
    /// [`Strings::sheet`](crate::Strings::sheet).
    aria_label: Option<String>,
    /// What sits beside the sheet. Defaults to [`SheetBackdrop::Dismiss`].
    backdrop: Option<SheetBackdrop>,
    /// Sheet width as a CSS length. Defaults to `min(18rem, 72%)` of the
    /// shell.
    width: Option<String>,
    /// Called after the user dismisses the sheet.
    on_dismiss: Option<EventHandler<()>>,
    /// Pad the sheet's content. Defaults to `true`. Set it to `false` for a
    /// sheet whose rows run to its edges, the way
    /// [`Content`](crate::Content) does for a page.
    padding: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the sheet surface.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        SheetFrame {
            open,
            kind: FrameKind::Side {
                edge: edge.unwrap_or_default(),
                behavior: behavior.unwrap_or_default(),
                width,
            },
            backdrop: backdrop.unwrap_or_default(),
            title,
            aria_label,
            on_dismiss,
            padding,
            mode,
            class,
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SideSheetPlaygroundDemo() -> Element {
    use dioxus_icons::lucide::Menu;
    let mut open = use_signal(|| false);
    let edge = use_signal(|| SheetEdge::Start);
    let behavior = use_signal(|| SideSheetBehavior::Overlay);
    let padding = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::SegmentGroup { value: edge, aria_label: "Edge",
                    crate::SegmentButton { value: SheetEdge::Start, "Start" }
                    crate::SegmentButton { value: SheetEdge::End, "End" }
                }
                crate::SegmentGroup { value: behavior, aria_label: "Behavior",
                    crate::SegmentButton { value: SideSheetBehavior::Overlay, "Overlay" }
                    crate::SegmentButton { value: SideSheetBehavior::Push, "Push" }
                    crate::SegmentButton { value: SideSheetBehavior::Reveal, "Reveal" }
                }
                crate::Checkbox { checked: open, label: "Open" }
                crate::Checkbox { checked: padding, label: "Pad the content" }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                SideSheet {
                    open,
                    edge: edge(),
                    behavior: behavior(),
                    padding: padding(),
                    title: "Round menu",
                    crate::List { lines: crate::ListLines::Full,
                        crate::Item { label: "Scorecard", metadata: "12 / 18" }
                        crate::Item { label: "Players", metadata: "4" }
                        crate::Item { label: "Round settings" }
                    }
                }
                div { class: "g3-sheet-demo-content-root",
                    crate::Header {
                        title: "Side sheets",
                        start: rsx! {
                            crate::Button {
                                fill: crate::ButtonFill::Clear,
                                size: crate::ButtonSize::Sm,
                                aria_label: "Toggle menu",
                                onclick: move |_| open.toggle(),
                                Menu { size: 22 }
                            }
                        },
                    }
                    crate::Content { footer_space: false,
                        crate::Card { title: "Round settings", "Push and Reveal move this page." }
                        crate::Button { onclick: move |_| open.set(true), "Open sheet" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "SideSheet",
    description: "A sheet that slides in from a side: overlay, push, or reveal.",
    demo: SideSheetPlaygroundDemo,
    source: "src/components/side_sheet.rs",
}
