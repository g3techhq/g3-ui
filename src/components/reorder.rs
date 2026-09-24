//! Lists whose rows can be dragged, or moved with the keyboard, into a new
//! order.
use super::gesture::{GestureScript, use_gesture};
use crate::state::{provide_live_context, use_element_id, use_live_context};
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

/// How the items of a [`ReorderList`] are laid out, which decides where a
/// dragged item lands and how the others make room.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ReorderLayout {
    /// One item per row, top to bottom. A dragged row takes the place of
    /// each row whose middle it passes, and the rows between move up or down
    /// by its height, so rows of any height work.
    #[default]
    Rows,
    /// Items that wrap across rows in reading order, such as the cells of a
    /// [`Grid`](crate::Grid). A dragged item lands on the cell whose middle
    /// is nearest the pointer, and the cells between each step into the
    /// neighbouring slot, wrapping between rows. Suits cells of one size.
    Grid,
}

impl ReorderLayout {
    fn as_str(self) -> &'static str {
        match self {
            ReorderLayout::Rows => "rows",
            ReorderLayout::Grid => "grid",
        }
    }
}

/// Follows a dragged item in the webview, moving the others aside, and
/// reports where it was dropped. See [`super::gesture`] for why.
const REORDER_SCRIPT: GestureScript = GestureScript {
    name: "g3-ui.reorder",
    source: include_str!("reorder.js"),
};

#[derive(Clone, Copy, PartialEq)]
struct ReorderContext {
    /// Every item's element, by position, so the keyboard knows how many
    /// places there are.
    rows: Signal<HashMap<usize, Rc<MountedData>>>,
    announcement: Signal<String>,
    onreorder: EventHandler<(usize, usize)>,
    disabled: Signal<bool>,
    layout: Signal<ReorderLayout>,
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
///
/// With [`ReorderLayout::Grid`] the items can wrap across rows, as the cells
/// of a [`Grid`](crate::Grid) do, and the left and right arrows move a cell
/// too. The list does not lay its items out itself, so any grid works:
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let mut posters = use_signal(|| vec!["Dune", "Heat", "Alien", "Up"]);
/// rsx! {
///     ReorderList {
///         layout: ReorderLayout::Grid,
///         onreorder: move |(from, to): (usize, usize)| {
///             posters.with_mut(|posters| {
///                 let poster = posters.remove(from);
///                 posters.insert(to, poster);
///             });
///         },
///         Grid { columns: GridColumns::Count(2),
///             for (index, title) in posters().into_iter().enumerate() {
///                 ReorderItem { key: "{title}", index,
///                     Card { title: title.to_string(),
///                         ReorderHandle { label: format!("Move {title}") }
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
    /// How the items are laid out. Defaults to [`ReorderLayout::Rows`].
    layout: Option<ReorderLayout>,
    /// Turn reordering off, leaving the rows where they are.
    disabled: Option<bool>,
    /// Extra classes for the list wrapper.
    class: Option<String>,
    /// The list, whose rows are [`ReorderItem`]s.
    children: Element,
) -> Element {
    let id = use_element_id("reorder", None);
    let rows = use_signal(HashMap::<usize, Rc<MountedData>>::new);
    let disabled = crate::state::use_synced_signal(disabled.unwrap_or(false));
    let announcement = use_signal(String::new);
    let layout = crate::state::use_synced_signal(layout.unwrap_or_default());
    let context = ReorderContext {
        rows,
        announcement,
        onreorder,
        disabled,
        layout,
    };
    provide_live_context(context);

    // The script moves the rows while they are dragged; a drop that changes
    // the order arrives here once.
    let start_script = use_gesture(REORDER_SCRIPT, id.clone(), move |gesture| {
        if gesture.kind == "reorder" && !disabled() {
            let (from, to) = (gesture.value(0) as usize, gesture.value(1) as usize);
            if from != to {
                onreorder((from, to));
            }
        }
    });

    rsx! {
        div {
            id,
            class: merge_classes("g3-reorder", class.as_deref()),
            // Read by the script at each press.
            "data-layout": layout().as_str(),
            onmounted: move |_| start_script.call(()),
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
            // The script orders the rows by this, and waits for it to change
            // before letting go of a dropped row.
            "data-index": "{index}",
            onmounted: move |event: MountedEvent| mounted.set(Some(event.data())),
            {children}
        }
    }
}

/// The position a [`ReorderItem`] gives its handle.
#[derive(Clone, Copy)]
struct ReorderPosition(Signal<usize>);

/// The grip that moves a row of a [`ReorderList`]. Drag it, or focus it and
/// press the up and down arrows; in a [`ReorderLayout::Grid`], left and
/// right move it back and on too.
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

    let count = rows.read().len();
    let grid = (context.layout)() == ReorderLayout::Grid;
    let key = move |event: KeyboardEvent| {
        if disabled {
            return;
        }
        // In a grid the item moves along the reading order, so left and
        // right move it too.
        let back = matches!(event.key(), Key::ArrowUp) || (grid && event.key() == Key::ArrowLeft);
        let on = matches!(event.key(), Key::ArrowDown) || (grid && event.key() == Key::ArrowRight);
        let to = match (back, on) {
            (true, _) if index > 0 => index - 1,
            (_, true) if index + 1 < count => index + 1,
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
            // Also how the script knows not to start a drag.
            aria_disabled: disabled.then_some("true"),
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
            "Best round",
        ]
    });
    let position = use_signal(|| ReorderHandlePosition::End);
    let layout = use_signal(ReorderLayout::default);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::SegmentGroup { value: layout, aria_label: "Layout",
                    crate::SegmentButton { value: ReorderLayout::Rows, "Rows" }
                    crate::SegmentButton { value: ReorderLayout::Grid, "Grid" }
                }
                if layout() == ReorderLayout::Rows {
                    crate::SegmentGroup { value: position, aria_label: "Handle edge",
                        crate::SegmentButton { value: ReorderHandlePosition::Start, "Start" }
                        crate::SegmentButton { value: ReorderHandlePosition::End, "End" }
                    }
                }
            },
            ReorderList {
                layout: layout(),
                onreorder: move |(from, to): (usize, usize)| {
                    rows.with_mut(|rows| {
                        let row = rows.remove(from);
                        rows.insert(to, row);
                    });
                },
                if layout() == ReorderLayout::Grid {
                    crate::Grid { columns: crate::GridColumns::Count(3), gap: crate::Space::Sm,
                        for (index, row) in rows().into_iter().enumerate() {
                            ReorderItem { key: "{row}", index,
                                crate::Card {
                                    title: row.to_string(),
                                    subtitle: format!("Position {}", index + 1),
                                    ReorderHandle { label: format!("Move {row}") }
                                }
                            }
                        }
                    }
                } else {
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
}

crate::g3_playground! {
    name: "Reorder",
    description: "Rows or grid cells moved by dragging a handle, or with the arrow keys.",
    components: ["ReorderList", "ReorderItem", "ReorderHandle"],
    demo: ReorderPlaygroundDemo,
    source: "src/components/reorder.rs",
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_stays_alive_and_never_waits_on_rust() {
        super::super::gesture::assert_gesture_script(REORDER_SCRIPT);
    }
}
