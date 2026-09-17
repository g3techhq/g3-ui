//! A sheet that rises from the bottom of the screen.
use super::sheet::{FrameKind, SheetBackdrop, SheetFrame};
use crate::state::use_controlled;
use crate::theme::ComponentMode;
use dioxus::prelude::*;

/// A sheet that rises from the bottom of the screen. Like Ionic's sheet
/// modal. On wide shells it floats as a centred card.
///
/// The owner holds the `open` signal. The sheet sets it to `false` when the
/// user dismisses it (backdrop tap, handle drag, Escape) and then calls
/// `on_dismiss`.
///
/// Without `detents` the sheet is as tall as its content, up to
/// `max_height`. With `detents` it has fixed heights, as fractions of the
/// screen, that the handle moves between:
///
/// ```rust,ignore
/// let open = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| open.set(true), "Filters" }
///     BottomSheet { open, title: "Filters", detents: vec![0.5, 1.0],
///         FilterForm {}
///     }
/// }
/// ```
#[component]
pub fn BottomSheet(
    /// Whether the sheet is open.
    open: Signal<bool>,
    /// Heading shown at the top, which also names the dialog.
    title: Option<String>,
    /// Accessible name when there is no `title`. Defaults to
    /// [`Strings::sheet`](crate::Strings::sheet).
    aria_label: Option<String>,
    /// Show a drag handle that closes the sheet or moves it between detents.
    /// Defaults to `true`.
    draggable: Option<bool>,
    /// Heights the sheet can rest at, as ascending fractions of the available
    /// height, such as `vec![0.4, 0.9]`.
    detents: Option<Vec<f64>>,
    /// Index into `detents` of the current height. Starts at the first detent
    /// when not given.
    detent: Option<Signal<usize>>,
    /// What sits behind the sheet. Defaults to [`SheetBackdrop::Dismiss`].
    backdrop: Option<SheetBackdrop>,
    /// Content height cap when there are no detents, as a CSS length.
    /// Defaults to `min(50dvh, 26rem)`.
    max_height: Option<String>,
    /// Called after the user dismisses the sheet.
    on_dismiss: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the sheet surface.
    class: Option<String>,
    children: Element,
) -> Element {
    let detent = use_controlled(detent, || 0);
    let mut detents = detents.unwrap_or_default();
    detents.retain(|fraction| fraction.is_finite() && *fraction > 0.0);
    detents.sort_by(f64::total_cmp);
    rsx! {
        SheetFrame {
            open,
            kind: FrameKind::Bottom {
                draggable: draggable.unwrap_or(true),
                detents,
                detent,
                max_height,
            },
            backdrop: backdrop.unwrap_or_default(),
            title,
            aria_label,
            on_dismiss,
            mode,
            class,
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn BottomSheetPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let backdrop = use_signal(|| SheetBackdrop::Dismiss);
    let with_detents = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::SegmentGroup { value: backdrop, aria_label: "Backdrop",
                    crate::SegmentButton { value: SheetBackdrop::Dismiss, "Dismiss" }
                    crate::SegmentButton { value: SheetBackdrop::None, "None" }
                }
                crate::Checkbox { checked: with_detents, label: "Detents (40% / 90%)" }
                crate::Checkbox { checked: open, label: "Open" }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                crate::Header { title: "Bottom sheet" }
                crate::Content { footer_space: false,
                    crate::Button { onclick: move |_| open.set(true), "Open sheet" }
                }
                BottomSheet {
                    open,
                    title: "Round settings",
                    backdrop: backdrop(),
                    detents: with_detents().then(|| vec![0.4, 0.9]),
                    crate::List { lines: crate::ListLines::Full,
                        crate::Item { label: "Scorecard", metadata: "12 / 18" }
                        crate::Item { label: "Players", metadata: "4" }
                        crate::Item { label: "Handicaps" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "BottomSheet",
    description: "A sheet that rises from the bottom, with optional detents.",
    demo: BottomSheetPlaygroundDemo,
    source: "src/components/bottom_sheet.rs",
}
