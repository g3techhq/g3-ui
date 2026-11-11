//! Loading more content as the user nears the end of a list.
use super::overlay::js_string;
use super::{Spinner, SpinnerSize};
use crate::state::{use_element_id, use_synced_signal};
use crate::theme::merge_classes;
use dioxus::prelude::*;

const OBSERVER_SCRIPT: &str = include_str!("infinite_scroll.js");

/// Which end of a list loads more.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InfiniteScrollEdge {
    /// After the last item, for a feed that grows downwards.
    #[default]
    Bottom,
    /// Before the first item, for a list whose newest items are at the
    /// bottom, such as a ledger or a chat. The rows already shown stay where
    /// they are on screen while earlier ones arrive above them.
    Top,
}

/// Calls `on_load` when the user scrolls near its position. Put it after the
/// last item of a list, or before the first with
/// [`InfiniteScrollEdge::Top`]. Like Ionic's `ion-infinite-scroll`.
///
/// Set `loading` while a page loads, which shows a spinner and holds further
/// calls, and `complete` once there is nothing more to load. It watches the
/// nearest element that scrolls, so it works inside a table that scrolls in
/// its own frame: give `table_columns` and put it in the `tbody`, where it
/// is a row spanning that many columns.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Clone, PartialEq)] struct Round { name: String }
/// # let rounds = use_signal(Vec::<Round>::new);
/// # let loading = use_signal(|| false);
/// # let done = use_signal(|| false);
/// # fn load_next_page() {}
/// rsx! {
///     List { for round in rounds() { Item { label: round.name } } }
///     InfiniteScroll { loading: loading(), complete: done(), on_load: move |_| load_next_page() }
/// }
/// # }
/// ```
#[component]
pub fn InfiniteScroll(
    /// Called when the user nears the end.
    on_load: EventHandler<()>,
    /// Whether a page is loading.
    loading: bool,
    /// Whether everything has loaded.
    complete: Option<bool>,
    /// How far before the end to start loading, in pixels. Defaults to 200.
    threshold: Option<u32>,
    /// Which end loads more. Defaults to [`InfiniteScrollEdge::Bottom`].
    edge: Option<InfiniteScrollEdge>,
    /// Inside a table body: render as a row spanning this many columns.
    table_columns: Option<u32>,
    /// Extra classes for the wrapper.
    class: Option<String>,
) -> Element {
    let id = use_element_id("infinite", None);
    let complete = complete.unwrap_or(false);
    let blocked = use_synced_signal(loading || complete);
    let margin = threshold.unwrap_or(200);
    let script = OBSERVER_SCRIPT
        .replace("__ID__", &js_string(&id))
        .replace("__MARGIN__", &margin.to_string())
        .replace(
            "__TOP__",
            if edge.unwrap_or_default() == InfiniteScrollEdge::Top {
                "true"
            } else {
                "false"
            },
        );
    super::script::use_dom_script(Some(script), move |load: bool| {
        if load && !*blocked.peek() {
            on_load.call(());
        }
    });

    let class = merge_classes("g3-infinite-scroll", class.as_deref());
    if let Some(columns) = table_columns {
        return rsx! {
            tr {
                id,
                class,
                "data-loading": loading.to_string(),
                "data-complete": complete.to_string(),
                aria_busy: loading.then_some("true"),
                td { colspan: "{columns.max(1)}",
                    // A flex box centres the spinner whatever the table's
                    // cell alignment.
                    div { class: "g3-infinite-scroll-cell",
                        if loading {
                            Spinner { size: SpinnerSize::Md }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        div {
            id,
            class,
            "data-loading": loading.to_string(),
            "data-complete": complete.to_string(),
            aria_busy: loading.then_some("true"),
            if loading {
                Spinner { size: SpinnerSize::Md }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn InfiniteScrollPlaygroundDemo() -> Element {
    const ROUNDS: usize = 60;
    let mut count = use_signal(|| 12_usize);
    let mut loading = use_signal(|| false);
    let top = use_signal(|| false);
    let in_table = use_signal(|| false);
    let edge = if top() {
        InfiniteScrollEdge::Top
    } else {
        InfiniteScrollEdge::Bottom
    };
    // Loading at the top keeps the newest round last and brings earlier ones
    // in above it, as a ledger or a chat does.
    let rounds: Vec<usize> = if top() {
        (ROUNDS + 1 - count()..=ROUNDS).collect()
    } else {
        (1..=count()).collect()
    };
    let complete = count() >= ROUNDS;
    let on_load = move |_| {
        loading.set(true);
        spawn(async move {
            dioxus_sdk_time::sleep(std::time::Duration::from_millis(600)).await;
            count += 12;
            loading.set(false);
        });
    };

    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::Checkbox { checked: top, label: "Load earlier rounds at the top" }
                crate::Checkbox { checked: in_table, label: "In a table" }
            },
            if in_table() {
                crate::Table {
                    caption: "Rounds",
                    fill: true,
                    lines: crate::TableLines::Rows,
                    align: crate::TableAlign::Start,
                    thead {
                        tr {
                            th { scope: "col", "Round" }
                            th { scope: "col", "data-align": "end", "Score" }
                        }
                    }
                    tbody {
                        if top() {
                            InfiniteScroll { edge, table_columns: 2, loading: loading(), complete, on_load }
                        }
                        for round in rounds {
                            tr { key: "{round}",
                                th { scope: "row", "Round {round}" }
                                td { "data-align": "end", "{68 + round % 9}" }
                            }
                        }
                        if !top() {
                            InfiniteScroll { edge, table_columns: 2, loading: loading(), complete, on_load }
                        }
                    }
                }
            } else {
                if top() {
                    InfiniteScroll { edge, loading: loading(), complete, on_load }
                }
                crate::List { variant: crate::ListVariant::Raised,
                    for round in rounds {
                        crate::Item { key: "{round}", label: format!("Round {round}") }
                    }
                }
                if !top() {
                    InfiniteScroll { edge, loading: loading(), complete, on_load }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "InfiniteScroll",
    description: "Loads more content as the user nears the end.",
    demo: InfiniteScrollPlaygroundDemo,
    source: "src/components/infinite_scroll.rs",
}
