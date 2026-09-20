//! Lists whose rows can be dragged, or moved with the keyboard, into a new
//! order.
use crate::state::{provide_live_context, use_live_context};
use crate::theme::{merge_classes, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::GripVertical;
use std::collections::HashMap;
use std::rc::Rc;

/// Which edge of a row holds a [`ReorderHandle`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ReorderHandlePosition {
    /// Leading edge.
    #[default]
    Start,
    /// Trailing edge, matching Ionic's default.
    End,
}

/// A drag in progress: which row, where it started, and where it would land.
#[derive(Clone, Copy, PartialEq, Debug)]
struct Drag {
    from: usize,
    target: usize,
    start_y: f64,
    delta: f64,
    /// Height of the dragged row, which the rows it passes move by.
    height: f64,
}

#[derive(Clone, Copy, PartialEq)]
struct ReorderContext {
    drag: Signal<Option<Drag>>,
    /// Vertical middle of each row when the drag began, in viewport pixels.
    middles: Signal<Vec<f64>>,
    rows: Signal<HashMap<usize, Rc<MountedData>>>,
    announcement: Signal<String>,
    onreorder: EventHandler<(usize, usize)>,
    disabled: Signal<bool>,
}

/// Where a row dragged from `from` would land, given the pointer's `y` and
/// the middle of every row before the drag began.
fn target_index(from: usize, y: f64, middles: &[f64]) -> usize {
    let mut target = from;
    for (index, middle) in middles.iter().enumerate() {
        if index < from && y < *middle {
            target = target.min(index);
        }
        if index > from && y > *middle {
            target = target.max(index);
        }
    }
    target
}

/// How far a row that is not being dragged moves aside for the one that is.
fn shift(index: usize, drag: &Drag) -> f64 {
    if drag.from < drag.target && index > drag.from && index <= drag.target {
        -drag.height
    } else if drag.target < drag.from && index >= drag.target && index < drag.from {
        drag.height
    } else {
        0.0
    }
}

/// A list whose rows can be put in a new order. Like Ionic's
/// `ion-reorder-group`.
///
/// Wrap each row in a [`ReorderItem`] with its position, and put a
/// [`ReorderHandle`] in the row. Dragging the handle moves the row, and the
/// others make room; the arrow keys move it one place while the handle has
/// focus. Either way `onreorder` is called with the row's old and new
/// positions, and the list is reordered by the caller, which owns the data:
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let mut holes = use_signal(|| vec!["Front nine", "Back nine", "Par threes"]);
/// rsx! {
///     ReorderList {
///         onreorder: move |(from, to): (usize, usize)| {
///             holes.with_mut(|holes| {
///                 let hole = holes.remove(from);
///                 holes.insert(to, hole);
///             });
///         },
///         List { variant: ListVariant::Raised,
///             for (index, hole) in holes().into_iter().enumerate() {
///                 ReorderItem { key: "{hole}", index,
///                     Item {
///                         label: hole,
///                         start: rsx! { ReorderHandle { label: format!("Move {hole}") } },
///                     }
///                 }
///             }
///         }
///     }
/// }
/// # }
/// ```
#[component]
pub fn ReorderList(
    /// Called with `(from, to)` when a row is moved.
    onreorder: EventHandler<(usize, usize)>,
    /// Turn reordering off, leaving the rows where they are.
    disabled: Option<bool>,
    /// Extra classes for the list wrapper.
    class: Option<String>,
    /// The list, whose rows are [`ReorderItem`]s.
    children: Element,
) -> Element {
    let rows = use_signal(HashMap::<usize, Rc<MountedData>>::new);
    let disabled = crate::state::use_synced_signal(disabled.unwrap_or(false));
    let mut drag = use_signal(|| None::<Drag>);
    let middles = use_signal(Vec::<f64>::new);
    let announcement = use_signal(String::new);
    let context = ReorderContext {
        drag,
        middles,
        rows,
        announcement,
        onreorder,
        disabled,
    };
    provide_live_context(context);

    let finish = move |_| {
        if let Some(done) = drag.take()
            && done.target != done.from
        {
            (context.onreorder)((done.from, done.target));
        }
    };

    rsx! {
        div {
            class: merge_classes("g3-reorder", class.as_deref()),
            "data-dragging": drag.read().is_some().then_some("true"),
            onpointermove: move |event: PointerEvent| {
                let Some(mut current) = *drag.peek() else {
                    return;
                };
                let y = event.client_coordinates().y;
                current.delta = y - current.start_y;
                current.target = target_index(current.from, y, &middles.peek());
                drag.set(Some(current));
            },
            onpointerup: finish,
            onpointercancel: move |_| drag.set(None),
            {children}
            span { class: "g3-sr-only", role: "status", aria_live: "polite",
                "{announcement}"
            }
        }
    }
}

/// One row of a [`ReorderList`], at position `index`.
#[component]
pub fn ReorderItem(
    /// The row's position in the list.
    index: usize,
    /// Extra classes for the row wrapper.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_live_context::<ReorderContext>();
    let mut rows = context.rows;
    // Rows are keyed, so a reorder moves them without remounting. Their
    // position is a signal, so the handle inside follows it, and each row
    // re-registers its element under its new position.
    let position = crate::state::use_synced_signal(index);
    use_context_provider(|| ReorderPosition(position));
    let mut mounted = use_signal(|| None::<Rc<MountedData>>);
    use_effect(move || {
        let index = position();
        if let Some(element) = mounted() {
            rows.write().insert(index, element);
        }
    });
    let drag = *context.drag.read();
    let (offset, dragging) = match drag {
        Some(drag) if drag.from == index => (drag.delta, true),
        Some(drag) => (shift(index, &drag), false),
        None => (0.0, false),
    };
    use_drop(move || {
        let index = *position.peek();
        let ours = match (rows.peek().get(&index), mounted.peek().as_ref()) {
            (Some(registered), Some(element)) => Rc::ptr_eq(registered, element),
            _ => false,
        };
        if ours {
            rows.write().remove(&index);
        }
    });
    rsx! {
        div {
            class: merge_classes("g3-reorder-item", class.as_deref()),
            "data-dragging": dragging.then_some("true"),
            style: (offset != 0.0).then(|| format!("transform: translateY({offset}px);")),
            onmounted: move |event: MountedEvent| mounted.set(Some(event.data())),
            {children}
        }
    }
}

/// The position a [`ReorderItem`] gives its handle.
#[derive(Clone, Copy)]
struct ReorderPosition(Signal<usize>);

/// The grip that moves a row of a [`ReorderList`]. Drag it, or focus it and
/// press the up and down arrows.
#[component]
pub fn ReorderHandle(
    /// Accessible name, naming the row, such as "Move Dune". Defaults to
    /// [`Strings::reorder`](crate::Strings::reorder).
    label: Option<String>,
    /// Edge spacing for the handle. Place the handle in the matching `Item`
    /// slot. Defaults to [`ReorderHandlePosition::Start`] for compatibility
    /// with the original handle API.
    position: Option<ReorderHandlePosition>,
    /// Extra classes for the handle.
    class: Option<String>,
) -> Element {
    let context = use_live_context::<ReorderContext>();
    let ReorderPosition(row_position) = use_context::<ReorderPosition>();
    let index = row_position();
    let mut announcement = context.announcement;
    let label = label.unwrap_or_else(|| use_strings().reorder);
    let disabled = (context.disabled)();
    let position = position.unwrap_or_default();
    let rows = context.rows;
    let mut drag = context.drag;
    let mut middles = context.middles;

    let start = move |event: PointerEvent| {
        if disabled {
            return;
        }
        event.prevent_default();
        event.stop_propagation();
        let start_y = event.client_coordinates().y;
        // The drag starts at once, so a quick flick is not lost while the
        // rows are measured; the measurements land a moment later.
        middles.set(Vec::new());
        drag.set(Some(Drag {
            from: index,
            target: index,
            start_y,
            delta: 0.0,
            height: 0.0,
        }));
        spawn(async move {
            // Measured as the drag begins, so rows of any height make room
            // for each other correctly.
            let mounted: Vec<(usize, Rc<MountedData>)> =
                rows.peek().iter().map(|(i, m)| (*i, m.clone())).collect();
            let mut measured = vec![0.0; mounted.len()];
            let mut height = 0.0;
            for (row, mount) in mounted {
                if let Ok(rect) = mount.get_client_rect().await {
                    if row < measured.len() {
                        measured[row] = rect.origin.y + rect.size.height / 2.0;
                    }
                    if row == index {
                        height = rect.size.height;
                    }
                }
            }
            middles.set(measured);
            // The pointer may already have moved: place the row for where it
            // is now, if the drag has not ended meanwhile.
            let current = *drag.peek();
            if let Some(mut current) = current
                && current.from == index
            {
                current.height = height;
                current.target =
                    target_index(index, current.start_y + current.delta, &middles.peek());
                drag.set(Some(current));
            }
        });
    };

    let count = rows.read().len();
    let key = move |event: KeyboardEvent| {
        if disabled {
            return;
        }
        let to = match event.key() {
            Key::ArrowUp if index > 0 => index - 1,
            Key::ArrowDown if index + 1 < count => index + 1,
            _ => return,
        };
        event.prevent_default();
        (context.onreorder)((index, to));
        // The handle keeps focus and its name; only the new place is news.
        announcement.set(format!("{} / {count}", to + 1));
    };

    rsx! {
        button {
            r#type: "button",
            class: merge_classes("g3-reorder-handle", class.as_deref()),
            "data-position": match position {
                ReorderHandlePosition::Start => "start",
                ReorderHandlePosition::End => "end",
            },
            aria_label: label.clone(),
            aria_disabled: disabled.then_some("true"),
            onpointerdown: start,
            onkeydown: key,
            // A press on the handle is for dragging, not for the row.
            onclick: move |event| event.stop_propagation(),
            GripVertical { size: 20 }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ReorderPlaygroundDemo() -> Element {
    let mut rows = use_signal(|| {
        vec![
            "Front nine",
            "Back nine",
            "Par threes",
            "Longest drive",
            "Closest to the pin",
        ]
    });
    let position = use_signal(|| ReorderHandlePosition::End);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::SegmentGroup { value: position, aria_label: "Handle edge",
                    crate::SegmentButton { value: ReorderHandlePosition::Start, "Start" }
                    crate::SegmentButton { value: ReorderHandlePosition::End, "End" }
                }
            },
            ReorderList {
                onreorder: move |(from, to): (usize, usize)| {
                    rows.with_mut(|rows| {
                        let row = rows.remove(from);
                        rows.insert(to, row);
                    });
                },
                crate::List { variant: crate::ListVariant::Raised,
                    for (index, row) in rows().into_iter().enumerate() {
                        ReorderItem { key: "{row}", index,
                            crate::Item {
                                label: row,
                                description: format!("Position {}", index + 1),
                                start: (position() == ReorderHandlePosition::Start).then(|| rsx! {
                                    ReorderHandle { label: format!("Move {row}"), position: ReorderHandlePosition::Start }
                                }),
                                end: (position() == ReorderHandlePosition::End).then(|| rsx! {
                                    ReorderHandle { label: format!("Move {row}"), position: ReorderHandlePosition::End }
                                }),
                            }
                        }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Reorder",
    description: "Rows moved by dragging a handle, or with the arrow keys.",
    demo: ReorderPlaygroundDemo,
    source: "src/components/reorder.rs",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_row_lands_past_the_middles_it_crosses() {
        let middles = [25.0, 75.0, 125.0, 175.0];
        // Dragging row 1 (middle 75) down past row 2's middle.
        assert_eq!(target_index(1, 130.0, &middles), 2);
        assert_eq!(target_index(1, 180.0, &middles), 3);
        // Up past row 0's middle.
        assert_eq!(target_index(1, 20.0, &middles), 0);
        // Short of any middle, it stays.
        assert_eq!(target_index(1, 90.0, &middles), 1);
    }

    #[test]
    fn rows_between_make_room() {
        let down = Drag {
            from: 1,
            target: 3,
            start_y: 0.0,
            delta: 0.0,
            height: 50.0,
        };
        assert_eq!(shift(2, &down), -50.0);
        assert_eq!(shift(3, &down), -50.0);
        assert_eq!(shift(0, &down), 0.0);
        let up = Drag {
            from: 3,
            target: 1,
            start_y: 0.0,
            delta: 0.0,
            height: 50.0,
        };
        assert_eq!(shift(1, &up), 50.0);
        assert_eq!(shift(2, &up), 50.0);
        assert_eq!(shift(0, &up), 0.0);
    }
}
