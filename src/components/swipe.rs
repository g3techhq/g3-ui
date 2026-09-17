//! Swipeable list rows.
use super::Color;
use crate::theme::{classes, merge_classes, use_strings};
use dioxus::prelude::*;
use std::time::Duration;

/// Which edge of a row a swipe uncovers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SwipeSide {
    /// The leading edge, uncovered by dragging toward the trailing side.
    Start,
    /// The trailing edge, the usual home of destructive actions.
    End,
}

/// What a swipe does once it passes its threshold.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SwipeBehavior {
    /// Uncover the actions and hold them open until one is pressed or the
    /// row is swiped closed.
    #[default]
    Reveal,
    /// Run the first action directly, as with swipe-to-archive.
    Activate,
    /// Slide the row away and remove it.
    Dismiss,
}

/// The live state of a swipe, passed to [`SwipeItem`]'s callbacks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeState {
    /// Which edge is being uncovered.
    pub side: SwipeSide,
    /// Horizontal displacement of the row, in pixels.
    pub offset: f64,
    /// `offset` as a fraction of the actions' width; `1.0` is fully open.
    pub ratio: f64,
    /// Whether releasing now would activate or dismiss.
    pub committed: bool,
}

const REVEAL_WIDTH: f64 = 88.0;
const ACTIVATE_WIDTH: f64 = 136.0;
const DISMISS_WIDTH: f64 = 104.0;
const FULL_SWIPE_MARGIN: f64 = 30.0;
const ELASTIC_FACTOR: f64 = 0.55;
const ACTIVATE_RATIO: f64 = 0.48;
const ACTIVATE_SOFTENING: f64 = 0.72;
const DISMISS_OFFSET: f64 = 430.0;
const DISMISS_EXIT_MS: u64 = 560;
const DISMISS_COLLAPSE_MS: u64 = 180;
const LONG_PRESS_MS: u64 = 500;
const LONG_PRESS_SLOP: f64 = 8.0;
/// Matches the browser's own slop: below it every gesture looks diagonal, and
/// claiming one would steal ordinary scrolls.
const HORIZONTAL_SLOP: f64 = 10.0;

fn action_width(behavior: SwipeBehavior) -> f64 {
    match behavior {
        SwipeBehavior::Reveal => REVEAL_WIDTH,
        SwipeBehavior::Activate => ACTIVATE_WIDTH,
        SwipeBehavior::Dismiss => DISMISS_WIDTH,
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

fn elastic_offset(raw: f64, width: f64) -> f64 {
    let limit = width.max(1.0);
    if raw > limit {
        limit + (raw - limit) * ELASTIC_FACTOR
    } else if raw < -limit {
        -limit + (raw + limit) * ELASTIC_FACTOR
    } else {
        raw
    }
}

fn activate_offset(raw: f64, width: f64) -> f64 {
    let limit = width.max(1.0);
    let soften_from = limit * ACTIVATE_SOFTENING;
    let distance = raw.abs();
    if distance <= soften_from {
        return raw;
    }
    let extra = distance - soften_from;
    let remaining = (limit - soften_from).max(1.0);
    raw.signum() * (soften_from + remaining * (extra / (extra + remaining)))
}

/// Where the row sits for a raw drag distance.
fn drag_offset(raw: f64, behavior: SwipeBehavior, has_start: bool, has_end: bool) -> f64 {
    let available = match side_of(raw) {
        Some(SwipeSide::Start) => has_start,
        Some(SwipeSide::End) => has_end,
        None => false,
    };
    if !available {
        return 0.0;
    }
    let width = action_width(behavior);
    match behavior {
        SwipeBehavior::Reveal => raw.clamp(-width, width),
        SwipeBehavior::Activate => activate_offset(raw, width),
        SwipeBehavior::Dismiss => elastic_offset(raw, width),
    }
}

/// Whether releasing at `offset` commits the behaviour.
fn is_committed(offset: f64, behavior: SwipeBehavior) -> bool {
    let width = action_width(behavior);
    match behavior {
        SwipeBehavior::Activate => offset.abs() >= width * ACTIVATE_RATIO,
        SwipeBehavior::Reveal | SwipeBehavior::Dismiss => offset.abs() >= width + FULL_SWIPE_MARGIN,
    }
}

/// Where a released Reveal row settles: open past halfway, closed otherwise.
fn settle_reveal(offset: f64) -> f64 {
    if offset.abs() > REVEAL_WIDTH / 2.0 {
        REVEAL_WIDTH.copysign(offset)
    } else {
        0.0
    }
}

fn state_for(offset: f64, behavior: SwipeBehavior, committed: bool) -> Option<SwipeState> {
    side_of(offset).map(|side| SwipeState {
        side,
        offset,
        ratio: offset / action_width(behavior),
        committed,
    })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Idle,
    Exiting,
    Collapsing,
}

impl Phase {
    fn as_str(self) -> &'static str {
        match self {
            Phase::Idle => "idle",
            Phase::Exiting => "exiting",
            Phase::Collapsing => "collapsing",
        }
    }
}

/// A button uncovered by swiping a [`SwipeItem`].
#[component]
pub fn SwipeAction(
    /// Colour. Defaults to [`Color::Neutral`].
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
/// `start_actions` or `end_actions`. Keyboard and screen reader users reach
/// the actions through a "Show actions" button that appears on focus, or
/// with the arrow keys; Escape closes them.
///
/// ```rust,ignore
/// rsx! {
///     SwipeItem {
///         behavior: SwipeBehavior::Dismiss,
///         end_actions: rsx! { SwipeAction { color: Color::Danger, onclick: move |_| delete(id), "Delete" } },
///         on_dismiss: move |_| delete(id),
///         Item { label: "Round 12" }
///     }
/// }
/// ```
#[component]
pub fn SwipeItem(
    /// Actions uncovered by swiping toward the trailing edge.
    start_actions: Option<Element>,
    /// Actions uncovered by swiping toward the leading edge.
    end_actions: Option<Element>,
    /// What a full swipe does. Defaults to [`SwipeBehavior::Reveal`].
    behavior: Option<SwipeBehavior>,
    /// Turn swiping off.
    disabled: Option<bool>,
    /// Let a mouse drag swipe too. Defaults to `true`; turn it off to keep
    /// the gesture for touch while desktop users use the actions button.
    mouse_swipe: Option<bool>,
    /// Called as the row moves.
    on_swipe: Option<EventHandler<SwipeState>>,
    /// Called when an [`SwipeBehavior::Activate`] swipe commits.
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
    let behavior = behavior.unwrap_or_default();
    let width = action_width(behavior);
    let has_start = start_actions.is_some();
    let has_end = end_actions.is_some();
    let disabled = disabled.unwrap_or(false);
    let mouse_swipe = mouse_swipe.unwrap_or(true);
    let mut start = use_signal(|| (0.0, 0.0));
    let mut offset = use_signal(|| 0.0_f64);
    let mut dragging = use_signal(|| false);
    // Set once a drag is clearly sideways: the row then tracks the finger with
    // no transition, and touchmove is cancelled so the page cannot scroll.
    let mut horizontal = use_signal(|| false);
    let mut moved = use_signal(|| false);
    let mut press_generation = use_signal(|| 0_u64);
    let mut phase = use_signal(|| Phase::Idle);
    // A click that ends a swipe must not also activate the row, so the
    // content ignores pointers briefly after one.
    let mut suppress_click = use_signal(|| false);

    let current = offset();
    let start_hidden = current <= 0.0;
    let end_hidden = current >= 0.0;
    let open = current != 0.0;

    let mut close = move || {
        offset.set(0.0);
        dragging.set(false);
        horizontal.set(false);
    };
    let reveal_side = move |side: SwipeSide| {
        let mut offset = offset;
        match side {
            SwipeSide::Start if has_start => offset.set(width),
            SwipeSide::End if has_end => offset.set(-width),
            _ => {}
        }
    };

    rsx! {
        div {
            class: merge_classes("g3-swipe-item", class.as_deref()),
            style: format!(
                "--g3-swipe-offset: {current}px; --g3-swipe-progress: {}; --g3-swipe-action-width: {width}px;",
                (current / width).abs().min(1.4),
            ),
            "data-behavior": match behavior {
                SwipeBehavior::Reveal => "reveal",
                SwipeBehavior::Activate => "activate",
                SwipeBehavior::Dismiss => "dismiss",
            },
            "data-state": phase().as_str(),
            "data-dragging": (dragging() && horizontal()).then_some("true"),
            onkeydown: move |event| {
                if disabled || phase() != Phase::Idle {
                    return;
                }
                match event.key() {
                    Key::ArrowLeft => reveal_side(SwipeSide::End),
                    Key::ArrowRight => reveal_side(SwipeSide::Start),
                    Key::Escape if open => {
                        event.stop_propagation();
                        close();
                    }
                    _ => {}
                }
            },
            ontouchmove: move |event| {
                if horizontal() {
                    event.prevent_default();
                }
            },
            onpointerdown: move |event| {
                if disabled
                    || phase() != Phase::Idle
                    || (!mouse_swipe && event.data.pointer_type() == "mouse")
                {
                    return;
                }
                let point = event.client_coordinates();
                start.set((point.x, point.y));
                dragging.set(true);
                horizontal.set(false);
                moved.set(false);
                let generation = press_generation.with_mut(|g| {
                    *g += 1;
                    *g
                });
                if let Some(on_long_press) = on_long_press {
                    spawn(async move {
                        dioxus_sdk_time::sleep(Duration::from_millis(LONG_PRESS_MS)).await;
                        if press_generation() == generation && dragging() && !moved() {
                            on_long_press.call(());
                        }
                    });
                }
            },
            onpointermove: move |event| {
                if !dragging() || disabled || phase() != Phase::Idle {
                    return;
                }
                let point = event.client_coordinates();
                let (x, y) = start();
                let (dx, dy) = (point.x - x, point.y - y);
                if dx.hypot(dy) > LONG_PRESS_SLOP {
                    moved.set(true);
                }
                if !horizontal() {
                    if dx.abs() > HORIZONTAL_SLOP && dx.abs() > dy.abs() {
                        horizontal.set(true);
                    } else {
                        return;
                    }
                }
                let next = drag_offset(dx, behavior, has_start, has_end);
                offset.set(next);
                if let (Some(on_swipe), Some(state)) =
                    (on_swipe, state_for(next, behavior, is_committed(next, behavior)))
                {
                    on_swipe.call(state);
                }
            },
            onpointerup: move |_| {
                if !dragging() || phase() != Phase::Idle {
                    return;
                }
                dragging.set(false);
                let was_horizontal = horizontal();
                horizontal.set(false);
                if !was_horizontal {
                    return;
                }
                suppress_click.set(true);
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(300)).await;
                    suppress_click.set(false);
                });
                let released = offset();
                let committed = is_committed(released, behavior);
                match behavior {
                    SwipeBehavior::Reveal => offset.set(settle_reveal(released)),
                    SwipeBehavior::Activate => {
                        if committed
                            && let Some(state) = state_for(released, behavior, true)
                            && let Some(on_activate) = on_activate
                        {
                            on_activate.call(state);
                        }
                        offset.set(0.0);
                    }
                    SwipeBehavior::Dismiss => {
                        let Some(state) = state_for(released, behavior, true).filter(|_| committed) else {
                            offset.set(0.0);
                            return;
                        };
                        offset.set(DISMISS_OFFSET.copysign(released));
                        phase.set(Phase::Exiting);
                        spawn(async move {
                            dioxus_sdk_time::sleep(Duration::from_millis(DISMISS_EXIT_MS)).await;
                            phase.set(Phase::Collapsing);
                            dioxus_sdk_time::sleep(Duration::from_millis(DISMISS_COLLAPSE_MS)).await;
                            if let Some(on_dismiss) = on_dismiss {
                                on_dismiss.call(state);
                            }
                        });
                    }
                }
            },
            onpointercancel: move |_| {
                if phase() == Phase::Idle {
                    press_generation.with_mut(|g| *g += 1);
                    close();
                }
            },
            if let Some(actions) = start_actions {
                div {
                    class: "g3-swipe-actions g3-swipe-actions-start",
                    aria_hidden: start_hidden.to_string(),
                    inert: start_hidden.then_some(true),
                    {actions}
                }
            }
            if let Some(actions) = end_actions {
                div {
                    class: "g3-swipe-actions g3-swipe-actions-end",
                    aria_hidden: end_hidden.to_string(),
                    inert: end_hidden.then_some(true),
                    {actions}
                }
            }
            div {
                class: "g3-swipe-content",
                style: suppress_click().then_some("pointer-events: none;"),
                {children}
                if (has_start || has_end) && !disabled {
                    button {
                        r#type: "button",
                        class: "g3-swipe-toggle",
                        aria_expanded: open.to_string(),
                        onclick: move |_| {
                            if open {
                                close();
                            } else if has_end {
                                reveal_side(SwipeSide::End);
                            } else {
                                reveal_side(SwipeSide::Start);
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
    fn elastic_offset_slows_past_the_action_width() {
        assert_eq!(elastic_offset(44.0, 88.0), 44.0);
        assert_eq!(elastic_offset(188.0, 88.0), 143.0);
        assert_eq!(elastic_offset(-188.0, 88.0), -143.0);
    }

    #[test]
    fn full_swipe_needs_the_width_plus_a_margin() {
        assert!(!is_committed(133.0, SwipeBehavior::Dismiss));
        assert!(is_committed(134.0, SwipeBehavior::Dismiss));
        assert!(is_committed(-134.0, SwipeBehavior::Dismiss));
    }

    #[test]
    fn activation_commits_early() {
        assert!(!is_committed(64.0, SwipeBehavior::Activate));
        assert!(is_committed(66.0, SwipeBehavior::Activate));
    }

    #[test]
    fn side_follows_direction() {
        assert_eq!(side_of(12.0), Some(SwipeSide::Start));
        assert_eq!(side_of(-12.0), Some(SwipeSide::End));
        assert_eq!(side_of(0.0), None);
    }

    #[test]
    fn reveal_stops_at_the_action_width() {
        assert_eq!(drag_offset(188.0, SwipeBehavior::Reveal, true, true), 88.0);
        assert_eq!(
            drag_offset(-188.0, SwipeBehavior::Reveal, true, true),
            -88.0
        );
    }

    #[test]
    fn activation_softens_toward_its_limit() {
        assert_eq!(activate_offset(80.0, 136.0), 80.0);
        let long = activate_offset(400.0, 136.0);
        assert!(long > 130.0 && long < 136.0);
    }

    #[test]
    fn missing_sides_do_not_move() {
        assert_eq!(drag_offset(44.0, SwipeBehavior::Dismiss, false, true), 0.0);
        assert_eq!(drag_offset(-44.0, SwipeBehavior::Dismiss, true, false), 0.0);
        assert_eq!(drag_offset(44.0, SwipeBehavior::Dismiss, true, false), 44.0);
    }

    #[test]
    fn reveal_settles_open_past_halfway() {
        assert_eq!(settle_reveal(50.0), 88.0);
        assert_eq!(settle_reveal(-50.0), -88.0);
        assert_eq!(settle_reveal(30.0), 0.0);
    }
}
