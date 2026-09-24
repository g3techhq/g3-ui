//! Swipeable list rows.
use super::Color;
use super::gesture::{Gesture, GestureScript, gesture_command, use_gesture};
use super::list::{InList, InSwipeRow};
use crate::state::use_element_id;
use crate::theme::{classes, merge_classes, use_strings};
use dioxus::prelude::*;

/// Which edge of a row a swipe uncovers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwipeSide {
    /// The leading edge, uncovered by dragging toward the trailing side.
    Start,
    /// The trailing edge, the usual home of destructive actions.
    End,
}

/// What swiping toward one edge does once it passes its threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SwipeBehavior {
    /// Uncover the actions and hold them open until one is pressed or the
    /// row is swiped closed.
    #[default]
    Reveal,
    /// Run a callback on release, as with swipe-to-archive. The first action
    /// stretches across the row as the swipe passes the threshold.
    Activate,
    /// Slide the row away and remove it.
    Dismiss,
}

impl SwipeBehavior {
    fn as_str(self) -> &'static str {
        match self {
            SwipeBehavior::Reveal => "reveal",
            SwipeBehavior::Activate => "activate",
            SwipeBehavior::Dismiss => "dismiss",
        }
    }
}

/// The live state of a swipe, passed to [`SwipeItem`]'s callbacks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeState {
    /// Which edge is being uncovered.
    pub side: SwipeSide,
    /// Horizontal displacement of the row, in pixels.
    pub offset: f64,
    /// `offset` as a fraction of the threshold distance; `1.0` is fully open.
    pub ratio: f64,
    /// Whether releasing now would activate or dismiss.
    pub committed: bool,
}

/// Runs every row's gesture inside the webview, so Rust hears only how each
/// one ended. See [`super::gesture`] for why.
const SWIPE_SCRIPT: GestureScript = GestureScript {
    name: "g3-ui.swipe",
    source: include_str!("swipe.js"),
};

/// What the script reports about one row.
#[derive(Clone, Copy, Debug, PartialEq)]
enum SwipeEvent {
    /// The row came to rest at this offset: closed, or open on one edge.
    Rest(f64),
    /// The row moved; only reported when someone listens.
    Move(SwipeState),
    Activate(SwipeState),
    Dismiss(SwipeState),
    LongPress,
}

impl SwipeEvent {
    /// Reads `[offset, ratio, committed]`.
    fn parse(gesture: &Gesture) -> Option<Self> {
        let (offset, ratio) = (gesture.value(0), gesture.value(1));
        let committed = gesture.value(2) != 0.0;
        let state = || {
            side_of(offset).map(|side| SwipeState {
                side,
                offset,
                ratio,
                committed,
            })
        };
        Some(match gesture.kind.as_str() {
            "rest" => SwipeEvent::Rest(offset),
            "move" => SwipeEvent::Move(state()?),
            "activate" => SwipeEvent::Activate(state()?),
            "dismiss" => SwipeEvent::Dismiss(state()?),
            "long-press" => SwipeEvent::LongPress,
            _ => return None,
        })
    }
}

fn side_of(offset: f64) -> Option<SwipeSide> {
    if offset > 0.0 {
        Some(SwipeSide::Start)
    } else if offset < 0.0 {
        Some(SwipeSide::End)
    } else {
        None
    }
}

/// A button uncovered by swiping a [`SwipeItem`].
#[component]
pub fn SwipeAction(
    /// Color. Defaults to [`Color::Neutral`].
    color: Option<Color>,
    /// Accessible name, for icon-only actions.
    aria_label: Option<String>,
    /// Called when pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Extra classes for the action.
    class: Option<String>,
    children: Element,
) -> Element {
    let color_cls = format!(
        "g3-swipe-action-{}",
        color.unwrap_or(Color::Neutral).as_str()
    );
    rsx! {
        button {
            class: merge_classes(classes(["g3-swipe-action", &color_cls]), class.as_deref()),
            r#type: "button",
            aria_label,
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            {children}
        }
    }
}

/// A list row with actions behind it, uncovered by swiping. Like Ionic's
/// `ion-item-sliding`.
///
/// Put an [`Item`](crate::Item) inside and [`SwipeAction`]s in
/// `start_actions` or `end_actions`. Each edge has its own behaviour, so a
/// row can archive on a swipe one way and reveal buttons the other way.
/// Keyboard and screen reader users reach the actions through a "Show
/// actions" button that appears on focus, or with the arrow keys; Escape
/// closes them.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let id = 7_u32;
/// # fn archive(_id: u32) {}
/// # fn pin(_id: u32) {}
/// # fn delete(_id: u32) {}
/// rsx! {
///     SwipeItem {
///         start_behavior: SwipeBehavior::Activate,
///         start_actions: rsx! { SwipeAction { color: Color::Success, "Archive" } },
///         on_activate: move |_| archive(id),
///         end_actions: rsx! {
///             SwipeAction { onclick: move |_| pin(id), "Pin" }
///             SwipeAction { color: Color::Danger, onclick: move |_| delete(id), "Delete" }
///         },
///         Item { label: "Round 12" }
///     }
/// }
/// # }
/// ```
#[component]
pub fn SwipeItem(
    /// Actions uncovered by swiping toward the trailing edge.
    start_actions: Option<Element>,
    /// Actions uncovered by swiping toward the leading edge.
    end_actions: Option<Element>,
    /// What swiping far toward the trailing edge does. Defaults to
    /// [`SwipeBehavior::Reveal`].
    start_behavior: Option<SwipeBehavior>,
    /// What swiping far toward the leading edge does. Defaults to
    /// [`SwipeBehavior::Reveal`].
    end_behavior: Option<SwipeBehavior>,
    /// Turn swiping off.
    disabled: Option<bool>,
    /// Let a mouse drag swipe too. Defaults to `true`; turn it off to keep
    /// the gesture for touch while desktop users use the actions button.
    mouse_swipe: Option<bool>,
    /// Called as the row moves.
    on_swipe: Option<EventHandler<SwipeState>>,
    /// Called when a [`SwipeBehavior::Activate`] swipe is released past its
    /// threshold. [`SwipeState::side`] says which edge.
    on_activate: Option<EventHandler<SwipeState>>,
    /// Called once a [`SwipeBehavior::Dismiss`] row has slid away and
    /// collapsed. Remove the row here.
    on_dismiss: Option<EventHandler<SwipeState>>,
    /// Called when the row is held without moving.
    on_long_press: Option<EventHandler<()>>,
    /// Extra classes for the row.
    class: Option<String>,
    children: Element,
) -> Element {
    let strings = use_strings();
    let row_id = use_element_id("swipe", None);
    let in_list = use_hook(|| try_consume_context::<InList>().is_some());
    use_context_provider(|| InSwipeRow);
    let start_behavior = start_behavior.unwrap_or_default();
    let end_behavior = end_behavior.unwrap_or_default();
    let has_start = start_actions.is_some();
    let has_end = end_actions.is_some();
    let disabled = disabled.unwrap_or(false);
    let mouse_swipe = mouse_swipe.unwrap_or(true);
    // Where the row rests, as the script last reported it. Only used to keep
    // closed actions out of reach; the live offset never leaves the webview.
    let mut rest = use_signal(|| 0.0_f64);

    let start_script =
        use_gesture(
            SWIPE_SCRIPT,
            row_id.clone(),
            move |gesture| match SwipeEvent::parse(&gesture) {
                Some(SwipeEvent::Rest(offset)) => rest.set(offset),
                Some(SwipeEvent::Move(state)) => {
                    if let Some(on_swipe) = on_swipe {
                        on_swipe.call(state);
                    }
                }
                Some(SwipeEvent::Activate(state)) => {
                    if let Some(on_activate) = on_activate {
                        on_activate.call(state);
                    }
                }
                Some(SwipeEvent::Dismiss(state)) => {
                    if let Some(on_dismiss) = on_dismiss {
                        on_dismiss.call(state);
                    }
                }
                Some(SwipeEvent::LongPress) => {
                    if let Some(on_long_press) = on_long_press {
                        on_long_press.call(());
                    }
                }
                None => {}
            },
        );

    let current = rest();
    let open = current != 0.0;
    // Opens or closes the row: `"start"`, `"end"` or `"close"`.
    let command = use_callback({
        let row_id = row_id.clone();
        move |to: &'static str| gesture_command(SWIPE_SCRIPT, &row_id, to)
    });

    rsx! {
        div {
            id: row_id,
            class: merge_classes("g3-swipe-item", class.as_deref()),
            role: in_list.then_some("listitem"),
            onmounted: move |_| start_script.call(()),
            // Read by the script at each press, so a change takes effect on
            // the next gesture.
            "data-start-behavior": start_behavior.as_str(),
            "data-end-behavior": end_behavior.as_str(),
            "data-has-start": has_start.to_string(),
            "data-has-end": has_end.to_string(),
            "data-disabled": disabled.then_some("true"),
            // A mouse cannot drag this row, so its actions button shows on hover.
            "data-mouse-swipe": (!mouse_swipe).then_some("false"),
            "data-long-press": on_long_press.is_some().then_some("true"),
            "data-report-swipe": on_swipe.is_some().then_some("true"),
            onkeydown: move |event| {
                if disabled {
                    return;
                }
                match event.key() {
                    Key::ArrowLeft => command.call("end"),
                    Key::ArrowRight => command.call("start"),
                    Key::Escape if open => {
                        event.stop_propagation();
                        command.call("close");
                    }
                    _ => {}
                }
            },
            if let Some(actions) = start_actions {
                div {
                    class: "g3-swipe-actions g3-swipe-actions-start",
                    aria_hidden: (current <= 0.0).to_string(),
                    inert: (current <= 0.0).then_some(true),
                    // Pressing a revealed action finishes the swipe.
                    onclick: move |_| command.call("close"),
                    {actions}
                }
            }
            if let Some(actions) = end_actions {
                div {
                    class: "g3-swipe-actions g3-swipe-actions-end",
                    aria_hidden: (current >= 0.0).to_string(),
                    inert: (current >= 0.0).then_some(true),
                    onclick: move |_| command.call("close"),
                    {actions}
                }
            }
            div {
                class: "g3-swipe-content",
                {children}
                if (has_start || has_end) && !disabled {
                    button {
                        r#type: "button",
                        class: "g3-swipe-toggle",
                        aria_expanded: open.to_string(),
                        onclick: move |_| {
                            if open {
                                command.call("close");
                            } else if has_end {
                                command.call("end");
                            } else {
                                command.call("start");
                            }
                        },
                        "{strings.show_actions}"
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
    fn side_follows_direction() {
        assert_eq!(side_of(12.0), Some(SwipeSide::Start));
        assert_eq!(side_of(-12.0), Some(SwipeSide::End));
        assert_eq!(side_of(0.0), None);
    }

    #[test]
    fn reports_name_their_side() {
        let parse = |kind: &str, values: [f64; 3]| {
            SwipeEvent::parse(&Gesture {
                kind: kind.into(),
                values: values.into(),
            })
        };
        let Some(SwipeEvent::Activate(state)) = parse("activate", [-70.0, -0.5, 1.0]) else {
            panic!("an activation is parsed");
        };
        assert_eq!(state.side, SwipeSide::End);
        assert!(state.committed);
        assert_eq!(parse("rest", [0.0, 0.0, 0.0]), Some(SwipeEvent::Rest(0.0)));
        // Only a row that moved has a side to report.
        assert_eq!(parse("activate", [0.0, 0.0, 1.0]), None);
        assert_eq!(parse("unknown", [4.0, 0.0, 0.0]), None);
    }

    #[test]
    fn script_stays_alive_and_never_waits_on_rust() {
        super::super::gesture::assert_gesture_script(SWIPE_SCRIPT);
    }
}
