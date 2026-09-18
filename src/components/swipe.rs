//! Swipeable list rows.
use super::Color;
use super::list::{InList, InSwipeRow};
use super::overlay::js_string;
use crate::state::use_element_id;
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

/// Width of one revealed action, and the fallback before the actions are
/// measured.
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

/// How one edge of a row behaves: what a swipe does, and how far it travels
/// before that happens.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Edge {
    behavior: SwipeBehavior,
    width: f64,
}

impl Edge {
    /// `revealed` is the measured width of the edge's actions.
    fn new(behavior: SwipeBehavior, revealed: f64) -> Self {
        let width = match behavior {
            SwipeBehavior::Reveal => revealed.max(1.0),
            SwipeBehavior::Activate => ACTIVATE_WIDTH,
            SwipeBehavior::Dismiss => DISMISS_WIDTH,
        };
        Self { behavior, width }
    }

    /// Where the row sits for a raw drag distance toward this edge.
    fn offset(self, raw: f64) -> f64 {
        match self.behavior {
            SwipeBehavior::Reveal => raw.clamp(-self.width, self.width),
            SwipeBehavior::Activate => activate_offset(raw, self.width),
            SwipeBehavior::Dismiss => elastic_offset(raw, self.width),
        }
    }

    /// Whether releasing at `offset` commits the behaviour.
    fn committed(self, offset: f64) -> bool {
        match self.behavior {
            SwipeBehavior::Activate => offset.abs() >= self.width * ACTIVATE_RATIO,
            SwipeBehavior::Reveal | SwipeBehavior::Dismiss => {
                offset.abs() >= self.width + FULL_SWIPE_MARGIN
            }
        }
    }

    /// Where a released Reveal row settles: open past halfway, closed
    /// otherwise.
    fn settle(self, offset: f64) -> f64 {
        if offset.abs() > self.width / 2.0 {
            self.width.copysign(offset)
        } else {
            0.0
        }
    }

    fn state(self, offset: f64, committed: bool) -> Option<SwipeState> {
        side_of(offset).map(|side| SwipeState {
            side,
            offset,
            ratio: offset / self.width,
            committed,
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

/// The edge a drag of `raw` pixels moves toward, if it has actions.
fn edge_for(raw: f64, start: Option<Edge>, end: Option<Edge>) -> Option<Edge> {
    match side_of(raw)? {
        SwipeSide::Start => start,
        SwipeSide::End => end,
    }
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
    // Revealed actions are as wide as their buttons, measured once mounted.
    let mut start_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let mut end_mount = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let start_width = use_signal(|| REVEAL_WIDTH);
    let end_width = use_signal(|| REVEAL_WIDTH);
    let measure = move || {
        spawn(async move {
            for (mount, mut width) in [(start_mount, start_width), (end_mount, end_width)] {
                let Some(mount) = mount.peek().clone() else {
                    continue;
                };
                if let Ok(rect) = mount.get_client_rect().await
                    && rect.width() > 0.0
                    && (rect.width() - *width.peek()).abs() > 0.5
                {
                    width.set(rect.width());
                }
            }
        });
    };

    let start_edge = has_start.then(|| Edge::new(start_behavior, start_width()));
    let end_edge = has_end.then(|| Edge::new(end_behavior, end_width()));

    let current = offset();
    let start_hidden = current <= 0.0;
    let end_hidden = current >= 0.0;
    let open = current != 0.0;
    let current_width = edge_for(current, start_edge, end_edge).map_or(REVEAL_WIDTH, |e| e.width);

    let mut close = move || {
        offset.set(0.0);
        dragging.set(false);
        horizontal.set(false);
    };
    let reveal_side = move |side: SwipeSide| {
        let mut offset = offset;
        match side {
            SwipeSide::Start if has_start => offset.set(*start_width.peek()),
            SwipeSide::End if has_end => offset.set(-*end_width.peek()),
            _ => {}
        }
    };

    // Ends a gesture, however it ends: released over the row, or anywhere
    // else once the pointer is captured.
    let release = use_callback(move |()| {
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
        let Some(edge) = edge_for(released, start_edge, end_edge) else {
            offset.set(0.0);
            return;
        };
        let committed = edge.committed(released);
        match edge.behavior {
            SwipeBehavior::Reveal => offset.set(edge.settle(released)),
            SwipeBehavior::Activate => {
                if committed
                    && let Some(state) = edge.state(released, true)
                    && let Some(on_activate) = on_activate
                {
                    on_activate.call(state);
                }
                offset.set(0.0);
            }
            SwipeBehavior::Dismiss => {
                let Some(state) = edge.state(released, true).filter(|_| committed) else {
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
    });

    rsx! {
        div {
            id: row_id.clone(),
            class: merge_classes("g3-swipe-item", class.as_deref()),
            role: in_list.then_some("listitem"),
            style: format!(
                "--g3-swipe-offset: {current}px; --g3-swipe-progress: {}; --g3-swipe-exit: {};",
                (current / current_width).abs().min(1.4),
                // Which way a dismissed row leaves. The distance is the row's
                // own width, in CSS, so it clears a desktop row as surely as a
                // phone one.
                if current < 0.0 { -1 } else { 1 },
            ),
            "data-start-behavior": start_behavior.as_str(),
            "data-end-behavior": end_behavior.as_str(),
            "data-committed": edge_for(current, start_edge, end_edge)
                .is_some_and(|edge| edge.behavior != SwipeBehavior::Reveal && edge.committed(current))
                .then_some("true"),
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
                measure();
                let point = event.client_coordinates();
                // Resume from where an open row rests.
                start.set((point.x - offset(), point.y));
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
                    if (dx - offset()).abs() > HORIZONTAL_SLOP && (dx - offset()).abs() > dy.abs() {
                        horizontal.set(true);
                        // Keep receiving the drag after the pointer leaves the row.
                        document::eval(&format!(
                            "try {{ document.getElementById({}).setPointerCapture({}); }} catch (error) {{}}",
                            js_string(&row_id),
                            event.data.pointer_id(),
                        ));
                    } else {
                        return;
                    }
                }
                let Some(edge) = edge_for(dx, start_edge, end_edge) else {
                    offset.set(0.0);
                    return;
                };
                let next = edge.offset(dx);
                offset.set(next);
                if let (Some(on_swipe), Some(state)) = (on_swipe, edge.state(next, edge.committed(next))) {
                    on_swipe.call(state);
                }
            },
            onpointerup: move |_| release(()),
            // A captured pointer keeps reporting to the row wherever it goes.
            // Leaving is the fallback for when capture was not granted.
            onpointerleave: move |_| release(()),
            onlostpointercapture: move |_| release(()),
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
                    onmounted: move |event| {
                        start_mount.set(Some(event.data()));
                        measure();
                    },
                    // Pressing a revealed action finishes the swipe.
                    onclick: move |_| close(),
                    {actions}
                }
            }
            if let Some(actions) = end_actions {
                div {
                    class: "g3-swipe-actions g3-swipe-actions-end",
                    aria_hidden: end_hidden.to_string(),
                    inert: end_hidden.then_some(true),
                    onmounted: move |event| {
                        end_mount.set(Some(event.data()));
                        measure();
                    },
                    onclick: move |_| close(),
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

    const REVEAL: Edge = Edge {
        behavior: SwipeBehavior::Reveal,
        width: 88.0,
    };

    #[test]
    fn elastic_offset_slows_past_the_action_width() {
        assert_eq!(elastic_offset(44.0, 88.0), 44.0);
        assert_eq!(elastic_offset(188.0, 88.0), 143.0);
        assert_eq!(elastic_offset(-188.0, 88.0), -143.0);
    }

    #[test]
    fn full_swipe_needs_the_width_plus_a_margin() {
        let dismiss = Edge::new(SwipeBehavior::Dismiss, 0.0);
        assert!(!dismiss.committed(133.0));
        assert!(dismiss.committed(134.0));
        assert!(dismiss.committed(-134.0));
    }

    #[test]
    fn activation_commits_early() {
        let activate = Edge::new(SwipeBehavior::Activate, 0.0);
        assert!(!activate.committed(64.0));
        assert!(activate.committed(66.0));
    }

    #[test]
    fn side_follows_direction() {
        assert_eq!(side_of(12.0), Some(SwipeSide::Start));
        assert_eq!(side_of(-12.0), Some(SwipeSide::End));
        assert_eq!(side_of(0.0), None);
    }

    #[test]
    fn reveal_stops_at_the_measured_width() {
        assert_eq!(REVEAL.offset(188.0), 88.0);
        assert_eq!(REVEAL.offset(-188.0), -88.0);
        let two_buttons = Edge::new(SwipeBehavior::Reveal, 176.0);
        assert_eq!(two_buttons.offset(-300.0), -176.0);
    }

    #[test]
    fn activation_softens_toward_its_limit() {
        assert_eq!(activate_offset(80.0, 136.0), 80.0);
        let long = activate_offset(400.0, 136.0);
        assert!(long > 130.0 && long < 136.0);
    }

    #[test]
    fn each_edge_keeps_its_own_behaviour() {
        let activate = Edge::new(SwipeBehavior::Activate, 0.0);
        assert_eq!(edge_for(44.0, Some(activate), Some(REVEAL)), Some(activate));
        assert_eq!(edge_for(-44.0, Some(activate), Some(REVEAL)), Some(REVEAL));
        assert_eq!(edge_for(-44.0, Some(activate), None), None);
        assert_eq!(edge_for(0.0, Some(activate), Some(REVEAL)), None);
    }

    #[test]
    fn reveal_settles_open_past_halfway() {
        assert_eq!(REVEAL.settle(50.0), 88.0);
        assert_eq!(REVEAL.settle(-50.0), -88.0);
        assert_eq!(REVEAL.settle(30.0), 0.0);
    }
}
