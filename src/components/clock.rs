//! The clock face of a Material time picker.
use super::gesture::{GestureScript, use_gesture};
use super::keyboard::use_roving_selection;
use crate::state::use_element_id;
use dioxus::prelude::*;

/// Radii of the dial's rings, in CSS pixels. `clock.js` shares them, with
/// the dial's size, to turn a pointer into a value.
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

impl DialMode {
    /// How `clock.js` knows the mode.
    fn as_str(self) -> &'static str {
        match self {
            DialMode::Hours12 => "hours12",
            DialMode::Hours24 => "hours24",
            DialMode::Minutes => "minutes",
        }
    }
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

/// Follows a press or drag on the face in the webview, and reports each value
/// it lands on. See [`super::gesture`] for why.
const CLOCK_SCRIPT: GestureScript = GestureScript {
    name: "g3-ui.clock",
    source: include_str!("clock.js"),
};

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
    // The script turns the pointer into values; each one it lands on arrives
    // here, and so does the end of the press.
    let start_script = use_gesture(CLOCK_SCRIPT, face_id.clone(), move |gesture| match gesture
        .kind
        .as_str()
    {
        "pick" => {
            let picked = gesture.value(0) as u8;
            if picked != value {
                onchange.call(picked);
            }
        }
        "release" => on_release.call(()),
        _ => {}
    });
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
            // Read by the script at each press; it marks the face while
            // dragging.
            "data-mode": mode.as_str(),
            onmounted: move |_| start_script.call(()),
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

    #[test]
    fn a_24_hour_dial_puts_the_afternoon_on_its_inner_ring() {
        assert_eq!(position(DialMode::Hours24, 3), (90.0, OUTER_RADIUS));
        assert_eq!(position(DialMode::Hours24, 15), (90.0, INNER_RADIUS));
        assert_eq!(position(DialMode::Hours24, 0), (0.0, INNER_RADIUS));
        assert_eq!(position(DialMode::Minutes, 7), (42.0, OUTER_RADIUS));
    }

    #[test]
    fn marks_label_every_hour_and_every_five_minutes() {
        assert_eq!(marks(DialMode::Minutes).len(), 12);
        assert_eq!(marks(DialMode::Hours24).len(), 24);
    }

    /// The script reads the rings' radii from its own copy.
    #[test]
    fn script_shares_the_dial_geometry() {
        super::super::gesture::assert_gesture_script(CLOCK_SCRIPT);
        assert!(
            CLOCK_SCRIPT
                .source
                .contains(&format!("OUTER_RADIUS = {OUTER_RADIUS}"))
        );
        assert!(
            CLOCK_SCRIPT
                .source
                .contains(&format!("INNER_RADIUS = {INNER_RADIUS}"))
        );
    }
}
