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
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[component] fn FilterForm() -> Element { rsx! {} }
/// let mut open = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| open.set(true), "Filters" }
///     BottomSheet { open, title: "Filters", detents: vec![0.5, 1.0],
///         FilterForm {}
///     }
/// }
/// # }
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
    /// Heights the sheet can rest at, as fractions of the app's height, such
    /// as `vec![0.25, 0.5, 1.0]`. `1.0` fills the app below the status bar.
    /// They are sorted, so the order given does not matter.
    detents: Option<Vec<f64>>,
    /// Index into the sorted `detents` of the current height. Pass a signal to
    /// choose the starting height or to move the sheet from code; it follows
    /// the handle too. Starts at the lowest detent otherwise.
    detent: Option<Signal<usize>>,
    /// The lowest detent index at which the backdrop shows. Below it the
    /// page stays visible and usable, as for a map with a results sheet.
    /// Defaults to `0`, a backdrop at every height.
    backdrop_detent: Option<usize>,
    /// What sits behind the sheet. Defaults to [`SheetBackdrop::Dismiss`].
    backdrop: Option<SheetBackdrop>,
    /// Content height cap when there are no detents, as a CSS length.
    /// Defaults to `min(50dvh, 26rem)`.
    max_height: Option<String>,
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
                backdrop_detent: backdrop_detent.unwrap_or(0),
                max_height,
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
fn BottomSheetPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let backdrop = use_signal(|| SheetBackdrop::Dismiss);
    let preset = use_signal(|| 0_usize);
    let detent = use_signal(|| 0_usize);
    let presets: [(&str, Vec<f64>); 4] = [
        ("Fit content", vec![]),
        ("Half / full", vec![0.5, 1.0]),
        ("Peek / half / full", vec![0.2, 0.5, 1.0]),
        ("Quarter steps", vec![0.25, 0.5, 0.75, 1.0]),
    ];
    let detents = presets[preset()].1.clone();
    let count = detents.len();
    let heights = detents
        .iter()
        .map(|fraction| format!("{:.0}%", fraction * 100.0))
        .collect::<Vec<_>>();
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::SegmentGroup { value: backdrop, aria_label: "Backdrop",
                    crate::SegmentButton { value: SheetBackdrop::Dismiss, "Backdrop" }
                    crate::SegmentButton { value: SheetBackdrop::None, "No backdrop" }
                }
                crate::Select {
                    label: "Detents",
                    value: preset,
                    options: presets
                        .iter()
                        .enumerate()
                        .map(|(index, (label, _))| crate::SelectOption::new(index, *label))
                        .collect::<Vec<_>>(),
                }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                crate::Header { title: "Bottom sheet" }
                crate::Content { footer_space: false,
                    crate::Stack {
                        crate::Button {
                            onclick: move |_| open.set(true),
                            "Open sheet"
                        }
                        crate::Text { tone: crate::TextTone::Secondary,
                            if count > 0 {
                                "Drag the handle between heights, or tap it to step through them."
                            } else {
                                "The sheet is as tall as its content. Drag the handle down to close it."
                            }
                        }
                    }
                }
                BottomSheet {
                    open,
                    title: "Round settings",
                    backdrop: backdrop(),
                    detents,
                    detent,
                    if count > 0 {
                        crate::Text { tone: crate::TextTone::Secondary,
                            "Resting at {heights[detent().min(count - 1)]}"
                        }
                    }
                    crate::List { lines: crate::ListLines::Full,
                        for label in ["Scorecard", "Players", "Handicaps", "Tees", "Side games", "Weather", "Notes"] {
                            crate::Item { key: "{label}", label, onclick: |_| {} }
                        }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "BottomSheet",
    description: "A sheet that rises from the bottom and can rest at several heights.",
    demo: BottomSheetPlaygroundDemo,
    source: "src/components/bottom_sheet.rs",
}
