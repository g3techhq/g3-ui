//! The clock face of a Material time picker.
use super::keyboard::use_roving_selection;
use super::overlay::js_string;
use crate::state::use_element_id;
use dioxus::prelude::*;

/// The dial is drawn at this size, in CSS pixels.
const SIZE: f64 = 256.0;
const OUTER_RADIUS: f64 = 100.0;
const INNER_RADIUS: f64 = 64.0;

/// What a [`ClockDial`] picks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DialMode {
    /// Hours 1 to 12.
    Hours12,
    /// Hours 0 to 23: 1 to 12 on the outer ring, 13 to 00 on the inner.
    Hours24,
    /// Minutes 0 to 59, labelled every five.
    Minutes,
}

/// Where `value` sits on the dial: its angle in degrees clockwise from the
/// top, and its distance from the centre.
fn position(mode: DialMode, value: u8) -> (f64, f64) {
    match mode {
        DialMode::Hours12 => (f64::from(value % 12) * 30.0, OUTER_RADIUS),
        DialMode::Hours24 if (1..=12).contains(&value) => {
            (f64::from(value % 12) * 30.0, OUTER_RADIUS)
        }
        DialMode::Hours24 => (f64::from(value % 12) * 30.0, INNER_RADIUS),
        DialMode::Minutes => (f64::from(value) * 6.0, OUTER_RADIUS),
    }
}

/// The value under a point on the dial, given relative to its centre.
fn value_at(mode: DialMode, dx: f64, dy: f64) -> u8 {
    let angle = dx.atan2(-dy).to_degrees().rem_euclid(360.0);
    match mode {
        DialMode::Hours12 => match ((angle / 30.0).round() as u8) % 12 {
            0 => 12,
            hour => hour,
        },
        DialMode::Hours24 => {
            let hour = ((angle / 30.0).round() as u8) % 12;
            let inner = dx.hypot(dy) < (OUTER_RADIUS + INNER_RADIUS) / 2.0;
            match (inner, hour) {
                (false, 0) => 12,
                (false, hour) => hour,
                (true, 0) => 0,
                (true, hour) => hour + 12,
            }
        }
        DialMode::Minutes => ((angle / 6.0).round() as u8) % 60,
    }
}

/// The numbers printed on the dial, with their values.
fn marks(mode: DialMode) -> Vec<(u8, String)> {
    match mode {
        DialMode::Hours12 => (1..=12).map(|hour| (hour, hour.to_string())).collect(),
        DialMode::Hours24 => (1..=12)
            .map(|hour| (hour, hour.to_string()))
            .chain((13..=24).map(|hour| (hour % 24, format!("{:02}", hour % 24))))
            .collect(),
        DialMode::Minutes => (0..12)
            .map(|step| (step * 5, format!("{:02}", step * 5)))
            .collect(),
    }
}

/// Keeps pointer events coming to the dial while a finger or mouse drags
/// past its edge.
const CAPTURE_SCRIPT: &str = r#"
try { document.getElementById(__ID__).setPointerCapture(__POINTER__); } catch (error) {}
"#;

/// A clock face that picks an hour or a minute, as in Material's time
/// picker. Drag or tap anywhere on the face; the numbers are also a radio
/// group for keyboards and screen readers.
#[component]
pub(crate) fn ClockDial(
    mode: DialMode,
    value: u8,
    onchange: EventHandler<u8>,
    /// Called when a drag or tap ends, to move on from hours to minutes.
    on_release: EventHandler<()>,
    aria_label: String,
    /// Accessible name of each number, such as "5 minutes".
    mark_label: Callback<u8, String>,
) -> Element {
    let id = use_element_id("clock", None);
    let face_id = format!("{id}-face");
    use_roving_selection(id.clone(), "[role=radio]", false);
    let mut pressed = use_signal(|| false);
    let pick = move |event: PointerEvent| {
        let point = event.element_coordinates();
        let picked = value_at(mode, point.x - SIZE / 2.0, point.y - SIZE / 2.0);
        if picked != value {
            onchange.call(picked);
        }
    };
    let (angle, radius) = position(mode, value);
    // The keyboard stop: the selected mark, or for minutes between the
    // labels, the nearest one.
    let focus_value = match mode {
        DialMode::Minutes => ((u16::from(value) + 2) / 5 % 12 * 5) as u8,
        _ => value,
    };
    let between_marks = mode == DialMode::Minutes && value % 5 != 0;

    rsx! {
        div {
            class: "g3-clock",
            "data-dragging": pressed().then_some("true"),
            onpointerdown: {
                let face_id = face_id.clone();
                move |event: PointerEvent| {
                    pressed.set(true);
                    document::eval(
                        &CAPTURE_SCRIPT
                            .replace("__ID__", &js_string(&face_id))
                            .replace("__POINTER__", &event.data.pointer_id().to_string()),
                    );
                    pick(event);
                }
            },
            onpointermove: move |event| {
                if pressed() {
                    pick(event);
                }
            },
            onpointerup: move |_| {
                if pressed() {
                    pressed.set(false);
                    on_release.call(());
                }
            },
            onpointercancel: move |_| pressed.set(false),
            id: face_id,
            div { class: "g3-clock-center", aria_hidden: "true" }
            div {
                class: "g3-clock-hand",
                aria_hidden: "true",
                style: "--g3-clock-angle: {angle}deg; --g3-clock-length: {radius}px;",
                if between_marks {
                    span { class: "g3-clock-hand-dot" }
                }
            }
            div {
                id,
                class: "g3-clock-marks",
                role: "radiogroup",
                aria_label,
                for (mark, text) in marks(mode) {
                    {
                        let (angle, radius) = position(mode, mark);
                        let checked = mark == value;
                        rsx! {
                            button {
                                key: "{mark}-{radius}",
                                r#type: "button",
                                class: if radius < OUTER_RADIUS { "g3-clock-mark g3-clock-mark-inner" } else { "g3-clock-mark" },
                                role: "radio",
                                aria_checked: checked.to_string(),
                                aria_label: mark_label.call(mark),
                                tabindex: if mark == focus_value { "0" } else { "-1" },
                                style: "--g3-clock-angle: {angle}deg; --g3-clock-length: {radius}px;",
                                onclick: move |_| {
                                    if mark != value {
                                        onchange.call(mark);
                                    }
                                },
                                "{text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(mode: DialMode, degrees: f64, radius: f64) -> u8 {
        let radians = degrees.to_radians();
        value_at(mode, radius * radians.sin(), -radius * radians.cos())
    }

    #[test]
    fn hours_follow_the_clock_face() {
        assert_eq!(at(DialMode::Hours12, 0.0, OUTER_RADIUS), 12);
        assert_eq!(at(DialMode::Hours12, 90.0, OUTER_RADIUS), 3);
        assert_eq!(at(DialMode::Hours12, 184.0, OUTER_RADIUS), 6);
        assert_eq!(at(DialMode::Hours12, 350.0, OUTER_RADIUS), 12);
    }

    #[test]
    fn a_24_hour_dial_uses_its_inner_ring_for_the_afternoon() {
        assert_eq!(at(DialMode::Hours24, 0.0, OUTER_RADIUS), 12);
        assert_eq!(at(DialMode::Hours24, 0.0, INNER_RADIUS), 0);
        assert_eq!(at(DialMode::Hours24, 90.0, INNER_RADIUS), 15);
        assert_eq!(at(DialMode::Hours24, 90.0, OUTER_RADIUS), 3);
        assert_eq!(position(DialMode::Hours24, 15), (90.0, INNER_RADIUS));
        assert_eq!(position(DialMode::Hours24, 0), (0.0, INNER_RADIUS));
    }

    #[test]
    fn minutes_resolve_to_single_minutes() {
        assert_eq!(at(DialMode::Minutes, 0.0, OUTER_RADIUS), 0);
        assert_eq!(at(DialMode::Minutes, 42.0, OUTER_RADIUS), 7);
        assert_eq!(at(DialMode::Minutes, 359.0, OUTER_RADIUS), 0);
        assert_eq!(at(DialMode::Minutes, 356.0, OUTER_RADIUS), 59);
        assert_eq!(marks(DialMode::Minutes).len(), 12);
        assert_eq!(marks(DialMode::Hours24).len(), 24);
    }
}
