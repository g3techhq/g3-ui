//! A row of stars for giving or showing a rating.
use super::color::Color;
use super::field::{FieldShell, described_by};
use crate::state::{use_controlled, use_element_id};
use dioxus::prelude::*;

/// A five-point star, on a 24 x 24 grid.
const STAR: &str = "M12 1.6l3.1 6.6 7.2.9-5.3 5 1.4 7.1L12 17.7l-6.4 3.5 1.4-7.1-5.3-5 7.2-.9z";

/// Space between stars, in pixels, which the pointer maths has to know.
const GAP: f64 = 2.0;

/// The value a press at `offset` pixels along the row picks, where each star
/// is `size` pixels wide. Snaps to whole stars, or halves when `half`, and
/// never goes below the smallest step: clearing is the keyboard's Home.
fn value_at(offset: f64, size: f64, max: u8, half: bool) -> f64 {
    let pitch = size + GAP;
    let index = (offset / pitch).floor().clamp(0.0, f64::from(max) - 1.0);
    let within = offset - index * pitch;
    let step = if half && within < size / 2.0 {
        0.5
    } else {
        1.0
    };
    let lowest = if half { 0.5 } else { 1.0 };
    (index + step).clamp(lowest, f64::from(max))
}

/// How much of star `index` (0-based) a value of `value` fills, 0 to 1.
fn fill_of(value: f64, index: u8) -> f64 {
    (value - f64::from(index)).clamp(0.0, 1.0)
}

/// Rounds a stored value to the steps the control offers.
fn snap(value: f64, max: u8, half: bool) -> f64 {
    let steps = if half { 2.0 } else { 1.0 };
    ((value * steps).round() / steps).clamp(0.0, f64::from(max))
}

/// How a value reads aloud: "3.5 of 5 stars", or "Not rated".
fn spoken(value: f64, max: u8) -> String {
    if value <= 0.0 {
        return "Not rated".to_string();
    }
    let number = if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    };
    format!("{number} of {max} stars")
}

/// A row of stars. Like a slider, it can be pressed, dragged across, or moved
/// with the arrow keys; `readonly` turns it into a display, which can show any
/// fraction, such as an average of 3.7.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # fn save(_stars: f64) {}
/// let stars = use_signal(|| 0.0);
/// rsx! {
///     Rating { label: "Your rating", value: stars, half: true, onchange: move |v| save(v) }
///     Rating { aria_label: "Average", value: use_signal(|| 3.7), readonly: true }
/// }
/// # }
/// ```
///
/// `oninput` fires with every value a drag passes through, for a live
/// preview; `onchange` fires once when it ends, so a caller saves one value
/// per gesture rather than every step.
#[component]
pub fn Rating(
    /// The value, in stars. Kept internally, starting at `0`, when not given.
    value: Option<Signal<f64>>,
    /// Number of stars. Defaults to `5`.
    max: Option<u8>,
    /// Allow half stars. Defaults to `false`.
    half: Option<bool>,
    /// Show the value without letting it change.
    readonly: Option<bool>,
    /// Disable the control.
    disabled: Option<bool>,
    /// Visible label, which also names the control.
    label: Option<String>,
    /// Accessible name when there is no visible label.
    aria_label: Option<String>,
    /// Help text below.
    helper: Option<String>,
    /// Width of one star, in pixels. Defaults to `24`. The stars shrink
    /// below it when the row would not otherwise fit, as ten stars on a
    /// phone may not.
    size: Option<u32>,
    /// Color of a filled star. Defaults to [`Color::Warning`], the gold
    /// ratings are usually shown in.
    color: Option<Color>,
    /// Called with every value while dragging.
    oninput: Option<EventHandler<f64>>,
    /// Called with the value once a press, drag, or key changes it.
    onchange: Option<EventHandler<f64>>,
    /// Element id. Generated when not given.
    id: Option<String>,
    /// Extra classes for the field wrapper.
    class: Option<String>,
) -> Element {
    let id = use_element_id("rating", id);
    let max = max.unwrap_or(5).max(1);
    let half = half.unwrap_or(false);
    let readonly = readonly.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let interactive = !readonly && !disabled;
    let size = f64::from(size.unwrap_or(24));
    // The row's drawn width, once known: the stars may be narrower than
    // `size`, and a press has to map to the star it lands on.
    let mut row_width = use_signal(|| None::<f64>);
    let star_size = move || match row_width() {
        Some(width) if width > 0.0 => ((width + GAP) / f64::from(max) - GAP).min(size),
        _ => size,
    };
    let mut value = use_controlled(value, || 0.0);
    let mut dragging = use_signal(|| false);
    let step = if half { 0.5 } else { 1.0 };

    let current = value();
    let shown = if readonly {
        current.clamp(0.0, f64::from(max))
    } else {
        snap(current, max, half)
    };

    let mut set = move |next: f64, live: bool| {
        if (next - *value.peek()).abs() < f64::EPSILON {
            return;
        }
        value.set(next);
        if live {
            if let Some(oninput) = oninput {
                oninput.call(next);
            }
        } else if let Some(onchange) = onchange {
            onchange.call(next);
        }
    };
    let mut commit = move || {
        if dragging() {
            dragging.set(false);
            if let Some(onchange) = onchange {
                onchange.call(*value.peek());
            }
        }
    };

    let labelled_by = label.as_ref().map(|_| format!("{id}-label"));
    // A display is an image, whose name is all a screen reader hears of it, so
    // the name has to carry the value. It cannot point at the visible label
    // with aria-labelledby: that replaces aria-label outright, and the value
    // would be lost.
    let display_name = match (&label, &aria_label) {
        (Some(name), _) | (None, Some(name)) => format!("{name}: {}", spoken(shown, max)),
        (None, None) => spoken(shown, max),
    };
    let describedby = described_by(&id, &helper, &None);
    let spoken_value = spoken(shown, max);
    let px = size.to_string();

    let stars = rsx! {
        for index in 0..max {
            span {
                key: "{index}",
                class: "g3-rating-star",
                style: "width: {px}px;",
                // Each star knows where it sits, so a press on it, or a drag
                // that the browser keeps delivering to it, maps to a place
                // along the whole row.
                onpointerdown: move |event: PointerEvent| {
                    if !interactive {
                        return;
                    }
                    event.prevent_default();
                    dragging.set(true);
                    let size = star_size();
                    let offset = f64::from(index) * (size + GAP) + event.element_coordinates().x;
                    set(value_at(offset, size, max, half), true);
                },
                onpointermove: move |event: PointerEvent| {
                    if !interactive || !dragging() {
                        return;
                    }
                    let size = star_size();
                    let offset = f64::from(index) * (size + GAP) + event.element_coordinates().x;
                    set(value_at(offset, size, max, half), true);
                },
                svg {
                    view_box: "0 0 24 24",
                    "aria-hidden": "true",
                    defs {
                        clipPath { id: "{id}-clip-{index}",
                            rect {
                                x: "0",
                                y: "0",
                                width: "{fill_of(shown, index) * 24.0}",
                                height: "24",
                            }
                        }
                    }
                    path { class: "g3-rating-empty", d: STAR }
                    path {
                        class: "g3-rating-fill",
                        d: STAR,
                        clip_path: "url(#{id}-clip-{index})",
                    }
                }
            }
        }
    };

    rsx! {
        FieldShell {
            id: id.clone(),
            label,
            required: false,
            helper,
            error: None,
            class,
            if readonly {
                // A display is a picture of a number, not a control.
                div {
                    id: id.clone(),
                    class: "g3-rating",
                    role: "img",
                    "data-color": color.unwrap_or(Color::Warning).as_str(),
                    aria_label: display_name,
                    aria_describedby: describedby.clone(),
                    {stars}
                }
            } else {
                div {
                    id: id.clone(),
                    class: "g3-rating",
                    role: "slider",
                    tabindex: if disabled { "-1" } else { "0" },
                    "data-color": color.unwrap_or(Color::Warning).as_str(),
                    "data-interactive": interactive.then_some("true"),
                    aria_label: if labelled_by.is_none() { aria_label } else { None },
                    aria_labelledby: labelled_by,
                    aria_describedby: describedby,
                    aria_valuemin: "0",
                    aria_valuemax: "{max}",
                    aria_valuenow: "{shown}",
                    aria_valuetext: spoken_value,
                    aria_disabled: disabled.then_some("true"),
                    onresize: move |event: ResizeEvent| {
                        if let Ok(box_size) = event.data().get_border_box_size() {
                            row_width.set(Some(box_size.width));
                        }
                    },
                    onpointerup: move |_| commit(),
                    onpointercancel: move |_| commit(),
                    onpointerleave: move |_| commit(),
                    onkeydown: move |event: KeyboardEvent| {
                        if !interactive {
                            return;
                        }
                        let now = snap(*value.peek(), max, half);
                        let next = match event.key() {
                            Key::ArrowRight | Key::ArrowUp => now + step,
                            Key::ArrowLeft | Key::ArrowDown => now - step,
                            Key::Home => 0.0,
                            Key::End => f64::from(max),
                            _ => return,
                        };
                        event.prevent_default();
                        set(next.clamp(0.0, f64::from(max)), false);
                    },
                    {stars}
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn RatingPlaygroundDemo() -> Element {
    let stars = use_signal(|| 3.5);
    let half = use_signal(|| true);
    let max = use_signal(|| 5_u8);
    let saved = use_signal(|| None::<f64>);
    let mut saved_setter = saved;
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: half, label: "Half stars" }
                crate::SegmentGroup { value: max, aria_label: "Stars",
                    crate::SegmentButton { value: 5_u8, "5 stars" }
                    crate::SegmentButton { value: 10_u8, "10 stars" }
                }
            },
            crate::Stack { gap: crate::Space::Lg,
                Rating {
                    label: "Your rating",
                    value: stars,
                    max: max(),
                    half: half(),
                    helper: match saved() {
                        Some(v) => format!("Saved {v} stars"),
                        None => "Tap, drag, or use the arrow keys.".to_string(),
                    },
                    onchange: move |v| saved_setter.set(Some(v)),
                }
                Rating {
                    label: "Average of 214 ratings",
                    value: use_signal(|| 3.7),
                    max: 5,
                    readonly: true,
                    size: 18,
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Rating",
    description: "Stars for giving a rating, or showing one.",
    demo: RatingPlaygroundDemo,
    source: "src/components/rating.rs",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_press_picks_the_star_under_it() {
        // 24px stars, 2px gaps: star 3 spans 78..102.
        assert_eq!(value_at(80.0, 24.0, 5, false), 4.0);
        assert_eq!(value_at(0.0, 24.0, 5, false), 1.0);
        // Its left half is a half star when halves are on.
        assert_eq!(value_at(80.0, 24.0, 5, true), 3.5);
        assert_eq!(value_at(95.0, 24.0, 5, true), 4.0);
    }

    #[test]
    fn a_drag_off_either_end_stops_at_the_ends() {
        assert_eq!(value_at(-40.0, 24.0, 5, true), 0.5);
        assert_eq!(value_at(900.0, 24.0, 5, true), 5.0);
    }

    #[test]
    fn a_display_fills_fractions() {
        assert!((fill_of(3.7, 3) - 0.7).abs() < 1e-9);
        assert_eq!(fill_of(3.7, 2), 1.0);
        assert_eq!(fill_of(3.7, 4), 0.0);
    }

    #[test]
    fn values_read_aloud() {
        assert_eq!(spoken(0.0, 5), "Not rated");
        assert_eq!(spoken(4.0, 5), "4 of 5 stars");
        assert_eq!(spoken(3.5, 10), "3.5 of 10 stars");
    }

    #[test]
    fn stored_values_snap_to_the_offered_steps() {
        assert_eq!(snap(3.7, 5, false), 4.0);
        assert_eq!(snap(3.7, 5, true), 3.5);
        assert_eq!(snap(9.0, 5, true), 5.0);
    }
}
