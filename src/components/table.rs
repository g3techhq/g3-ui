//! A bordered data table that scrolls sideways when it is wider than the page.
use crate::state::use_element_id;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// A table of data, such as a scorecard, a leaderboard or a price list.
///
/// Write the rows as ordinary `thead`, `tbody`, `tr`, `th` and `td` elements;
/// `Table` styles them and scrolls the whole table sideways when it is wider
/// than the space it has. With `sticky_first_column`, the first cell of every
/// row stays put while the rest scroll under it, so a wide table keeps its
/// row names in view.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     Table { caption: "Front nine", sticky_first_column: true,
///         thead {
///             tr {
///                 th { scope: "col", "Hole" }
///                 th { scope: "col", "1" }
///                 th { scope: "col", "2" }
///             }
///         }
///         tbody {
///             tr {
///                 th { scope: "row", "Par" }
///                 td { "4" }
///                 td { "3" }
///             }
///             tr {
///                 th { scope: "row", "You" }
///                 td { "data-color": "success", "3" }
///                 td { "data-muted": "true", "–" }
///             }
///         }
///     }
/// }
/// # }
/// ```
///
/// A cell takes two attributes for emphasis. `data-color` tints it with a
/// [`Color`](crate::Color) name (`"accent"`, `"success"`, `"warning"`,
/// `"danger"`, or [`Color::as_str`](crate::Color::as_str)), for a good or
/// bad result; say what the tint means in the
/// cell's text too, since color alone does not reach everyone.
/// `data-muted="true"` greys it, for a cell that does not apply.
#[component]
pub fn Table(
    /// Visible title above the table, which also names it.
    caption: Option<String>,
    /// Accessible name when there is no caption.
    aria_label: Option<String>,
    /// Keep the first cell of each row in place while the rest scroll.
    sticky_first_column: Option<bool>,
    /// Stretch to the full width available. By default the table is as wide
    /// as its content, up to the width available.
    fill: Option<bool>,
    /// Extra classes for the scrolling frame.
    class: Option<String>,
    /// The rows: `thead`, `tbody` and `tr` elements.
    children: Element,
) -> Element {
    let id = use_element_id("table", None);
    let caption_id = format!("{id}-caption");
    // The frame scrolls, so it has to be reachable by keyboard, and a focused
    // region needs a name. It shares the table's.
    let table_label = if caption.is_none() {
        aria_label.clone()
    } else {
        None
    };
    let (labelled_by, label) = match &caption {
        Some(_) => (Some(caption_id.clone()), None),
        None => (None, aria_label.clone()),
    };
    rsx! {
        div {
            class: merge_classes("g3-table-frame", class.as_deref()),
            "data-fill": fill.unwrap_or(false).then_some("true"),
            role: "region",
            tabindex: "0",
            aria_labelledby: labelled_by,
            aria_label: label,
            table {
                id,
                class: "g3-table",
                "data-sticky-first": sticky_first_column.unwrap_or(false).then_some("true"),
                aria_label: table_label,
                if let Some(caption) = caption {
                    caption { id: caption_id, class: "g3-table-caption", "{caption}" }
                }
                {children}
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TablePlaygroundDemo() -> Element {
    let sticky = use_signal(|| true);
    let fill = use_signal(|| false);
    let pars = [4, 3, 5, 4, 4, 3, 4, 5, 4];
    let scores = [4, 2, 6, 4, 5, 3, 4, 4, 4];
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: sticky, label: "Sticky first column" }
                crate::Checkbox { checked: fill, label: "Fill width" }
            },
            Table {
                caption: "Front nine",
                sticky_first_column: sticky(),
                fill: fill(),
                thead {
                    tr {
                        th { scope: "col", "Hole" }
                        for hole in 1..=9 {
                            th { key: "{hole}", scope: "col", "{hole}" }
                        }
                        th { scope: "col", "Out" }
                    }
                }
                tbody {
                    tr {
                        th { scope: "row", "Par" }
                        for (hole, par) in pars.iter().enumerate() {
                            td { key: "{hole}", "{par}" }
                        }
                        td { "{pars.iter().sum::<i32>()}" }
                    }
                    tr {
                        th { scope: "row", "Alex" }
                        for (hole, (score, par)) in scores.iter().zip(pars).enumerate() {
                            td {
                                key: "{hole}",
                                "data-color": if *score < par { Some("success") } else if *score > par { Some("danger") } else { None },
                                "{score}"
                            }
                        }
                        td { "{scores.iter().sum::<i32>()}" }
                    }
                    tr {
                        th { scope: "row", "Sam" }
                        for hole in 0..9 {
                            td { key: "{hole}", "data-muted": "true", "–" }
                        }
                        td { "data-muted": "true", "–" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Table",
    description: "Rows and columns of data, scrolling sideways when wide.",
    demo: TablePlaygroundDemo,
    source: "src/components/table.rs",
}
