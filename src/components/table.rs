//! A bordered data table that scrolls sideways when it is wider than the page,
//! and, with a sticky header, down under its header row.
use crate::state::use_element_id;
use crate::theme::merge_classes;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowDown, ArrowUp, ArrowUpDown};

/// How a [`Table`] sits on its page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TableVariant {
    /// A quiet bordered surface.
    #[default]
    Flat,
    /// A card-like surface with elevation.
    Raised,
}

/// Which rules a [`Table`] draws between its cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TableLines {
    /// Rules between rows and between columns, as a scorecard has.
    #[default]
    Grid,
    /// Rules between rows only, as a ledger or a register has.
    Rows,
}

/// Where the text of a [`Table`]'s cells sits by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TableAlign {
    /// Centred, for short figures such as scores.
    #[default]
    Center,
    /// At the start of the cell, as a data grid reads. Header cells then
    /// read as column labels.
    Start,
}

/// How tightly a [`Table`] packs its rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TableDensity {
    /// Comfortable padding for touch.
    #[default]
    Standard,
    /// Tighter padding and slightly smaller type, for data-dense desktop
    /// screens.
    Compact,
}

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
/// `data-align` (`"start"`, `"center"` or `"end"`) places one cell's text
/// regardless of the table's `align`; `"end"` is for amounts, whose figures
/// are already tabular.
///
/// A row (`tr`) takes two attributes for a list of records.
/// `data-interactive="true"` gives it a pointer and a hover and pressed tint,
/// for a row that opens something on click; give it `tabindex: "0"` and a
/// key handler too, and a focus ring shows when it is reached by keyboard.
/// `aria-selected="true"` tints the selected row. `data-compact="true"`
/// packs one row tighter, such as a totals line.
///
/// For a long list, such as a register of thousands of rows, use
/// `sticky_header` with `lines: TableLines::Rows`, `align: TableAlign::Start`
/// and `density: TableDensity::Compact`. The table then scrolls inside its own
/// frame, which needs a bounded height: make it a flex child of a column
/// (it shrinks to the space left), or give it a `height` or `max-height`
/// through `class`. Without a bound it grows with its rows and the page
/// scrolls instead, taking the header with it.
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
    /// Surface treatment. Defaults to [`TableVariant::Flat`].
    variant: Option<TableVariant>,
    /// Cells hold prose, such as a reference of descriptions: they wrap and
    /// sit at the start and top of the cell, instead of staying on one
    /// centred line as figures do. Defaults to `false`.
    wrap: Option<bool>,
    /// Keep the header row in view while the body scrolls under it, inside
    /// the table's own frame, which then stays within the height it is given
    /// (a flex column's remaining space, or a height set through `class`)
    /// rather than growing the page. Defaults to `false`.
    sticky_header: Option<bool>,
    /// Which rules to draw between cells. Defaults to [`TableLines::Grid`].
    lines: Option<TableLines>,
    /// Where cell text sits unless a cell's `data-align` says otherwise.
    /// Defaults to [`TableAlign::Center`].
    align: Option<TableAlign>,
    /// Row padding and type size. Defaults to [`TableDensity::Standard`].
    density: Option<TableDensity>,
    /// Extra classes for the scrolling frame.
    class: Option<String>,
    /// The rows: `thead`, `tbody` and `tr` elements.
    children: Element,
) -> Element {
    let id = use_element_id("table", None);
    let caption_id = format!("{id}-caption");
    // The inner viewport scrolls, so it has to be reachable by keyboard, and
    // a focused region needs a name. The caption stays outside that viewport:
    // it titles the whole surface rather than travelling with its columns.
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
            "data-sticky-header": sticky_header.unwrap_or(false).then_some("true"),
            "data-variant": match variant.unwrap_or_default() {
                TableVariant::Flat => "flat",
                TableVariant::Raised => "raised",
            },
            if let Some(caption) = caption.as_ref() {
                div { id: caption_id.clone(), class: "g3-table-caption", "{caption}" }
            }

            div {
                class: "g3-table-scroll",
                role: "region",
                tabindex: "0",
                aria_labelledby: labelled_by,
                aria_label: label,
                table {
                    id,
                    class: "g3-table",
                    "data-sticky-first": sticky_first_column.unwrap_or(false).then_some("true"),
                    "data-wrap": wrap.unwrap_or(false).then_some("true"),
                    "data-lines": match lines.unwrap_or_default() {
                        TableLines::Grid => None,
                        TableLines::Rows => Some("rows"),
                    },
                    "data-align": match align.unwrap_or_default() {
                        TableAlign::Center => None,
                        TableAlign::Start => Some("start"),
                    },
                    "data-density": match density.unwrap_or_default() {
                        TableDensity::Standard => None,
                        TableDensity::Compact => Some("compact"),
                    },
                    aria_labelledby: caption.as_ref().map(|_| caption_id),
                    aria_label: table_label,
                    {children}
                }
            }
        }
    }
}

/// Which way a [`SortHeader`]'s column is sorted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SortDirection {
    /// Smallest, earliest or A first.
    Ascending,
    /// Largest, latest or Z first.
    Descending,
}

impl SortDirection {
    /// The `aria-sort` value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ascending => "ascending",
            Self::Descending => "descending",
        }
    }
}

/// A column header that sorts its [`Table`] when pressed.
///
/// Use it in place of a `th` in the header row. The label is a button; an
/// arrow shows the direction while the table is sorted by this column, and
/// a faint two-way arrow shows on hover while it is not. The table does not
/// sort its rows itself: `onsort` tells the page, which reorders its data
/// (often with a new query) and passes the new `sorted` back.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let mut by_amount = use_signal(|| false);
/// rsx! {
///     Table { aria_label: "Payments",
///         thead {
///             tr {
///                 SortHeader {
///                     sorted: (!by_amount()).then_some(SortDirection::Ascending),
///                     onsort: move |_| by_amount.set(false),
///                     "Date"
///                 }
///                 SortHeader { end: true,
///                     sorted: by_amount().then_some(SortDirection::Descending),
///                     onsort: move |_| by_amount.set(true),
///                     "Amount"
///                 }
///             }
///         }
///     }
/// }
/// # }
/// ```
#[component]
pub fn SortHeader(
    /// The direction while the table is sorted by this column; `None` while
    /// it is sorted by another.
    sorted: Option<SortDirection>,
    /// Called when the header is pressed.
    onsort: EventHandler<()>,
    /// Sit at the end of the cell, over a column of amounts. The arrow then
    /// goes before the label, so the label lines up with the figures.
    end: Option<bool>,
    /// Hover text, such as the full name of a short label.
    title: Option<String>,
    /// Extra classes for the cell.
    class: Option<String>,
    /// The label.
    children: Element,
) -> Element {
    rsx! {
        th {
            scope: "col",
            class: merge_classes("g3-table-sortable", class.as_deref()),
            "data-align": end.unwrap_or(false).then_some("end"),
            "data-sorted": sorted.is_some().then_some("true"),
            "aria-sort": sorted.map(SortDirection::as_str),
            title,
            button {
                r#type: "button",
                class: "g3-table-sort",
                onclick: move |_| onsort.call(()),
                span { class: "g3-table-sort-label", {children} }
                span { class: "g3-table-sort-icon", aria_hidden: "true",
                    match sorted {
                        Some(SortDirection::Ascending) => rsx! { ArrowUp { size: 14 } },
                        Some(SortDirection::Descending) => rsx! { ArrowDown { size: 14 } },
                        None => rsx! { ArrowUpDown { size: 14 } },
                    }
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TablePlaygroundDemo() -> Element {
    let sticky = use_signal(|| true);
    let sticky_header = use_signal(|| true);
    let fill = use_signal(|| false);
    let raised = use_signal(|| true);
    let rows_only = use_signal(|| true);
    let start = use_signal(|| true);
    let compact = use_signal(|| false);
    let mut selected = use_signal(|| Some(3_usize));
    let mut by_amount = use_signal(|| false);
    let pars = [4, 3, 5, 4, 4, 3, 4, 5, 4];
    let scores = [4, 2, 6, 4, 5, 3, 4, 4, 4];
    let payees = [
        "Harbor Grocers",
        "City Water",
        "Northwind Payroll",
        "Bell Telephone",
        "Oak Street Rent",
        "Fuel Depot",
    ];
    let variant = if raised() {
        TableVariant::Raised
    } else {
        TableVariant::Flat
    };
    let lines = if rows_only() {
        TableLines::Rows
    } else {
        TableLines::Grid
    };
    let align = if start() {
        TableAlign::Start
    } else {
        TableAlign::Center
    };
    let density = if compact() {
        TableDensity::Compact
    } else {
        TableDensity::Standard
    };
    // A register long enough to scroll: payees cycle, amounts follow a
    // pattern, and the running balance is worked out row by row.
    let mut balance = 1_250_000_i64;
    let mut register: Vec<(usize, String, &str, i64, i64)> = (0..60)
        .map(|row| {
            let amount = if row % 7 == 2 {
                215_000
            } else {
                -(((row as i64 * 3_719) % 48_000) + 1_250)
            };
            balance += amount;
            let date = format!("2026-{:02}-{:02}", 1 + row / 28, 1 + row % 28);
            (row, date, payees[row % payees.len()], amount, balance)
        })
        .collect();
    // Sorting by amount puts the largest first, whichever way the money went.
    if by_amount() {
        register.sort_by_key(|line| std::cmp::Reverse(line.3.abs()));
    }
    let money = |cents: i64| {
        let sign = if cents < 0 { "−" } else { "" };
        let cents = cents.abs();
        format!("{sign}{}.{:02}", cents / 100, cents % 100)
    };

    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: sticky, label: "Sticky first column" }
                crate::Checkbox { checked: sticky_header, label: "Sticky header" }
                crate::Checkbox { checked: fill, label: "Fill width" }
                crate::Checkbox { checked: raised, label: "Raised surface" }
                crate::Checkbox { checked: rows_only, label: "Row rules only (register)" }
                crate::Checkbox { checked: start, label: "Start aligned (register)" }
                crate::Checkbox { checked: compact, label: "Compact" }
            },
            crate::Stack {
                Table {
                    caption: "Front nine",
                    sticky_first_column: sticky(),
                    fill: fill(),
                    variant,
                    density,
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

                // A bounded column: the register shrinks to it and scrolls
                // inside its own frame, under its header.
                div { style: "display: flex; flex-direction: column; height: 20rem; min-width: 0;",
                    Table {
                        caption: "Checking register",
                        sticky_header: sticky_header(),
                        sticky_first_column: sticky(),
                        fill: true,
                        variant,
                        lines,
                        align,
                        density,
                        thead {
                            tr {
                                SortHeader {
                                    sorted: (!by_amount()).then_some(SortDirection::Ascending),
                                    onsort: move |_| by_amount.set(false),
                                    "Date"
                                }
                                th { scope: "col", "Payee" }
                                SortHeader { end: true,
                                    sorted: by_amount().then_some(SortDirection::Descending),
                                    onsort: move |_| by_amount.set(true),
                                    "Amount"
                                }
                                th { scope: "col", "data-align": "end", "Balance" }
                            }
                        }
                        tbody {
                            for (row, date, payee, amount, balance) in register {
                                tr {
                                    key: "{row}",
                                    "data-interactive": "true",
                                    tabindex: "0",
                                    aria_selected: (selected() == Some(row)).then_some("true"),
                                    onclick: move |_| selected.set(Some(row)),
                                    onkeydown: move |event| {
                                        if event.key() == Key::Named(NamedKey::Enter) || event.key() == Key::Character(" ".into()) {
                                            event.prevent_default();
                                            selected.set(Some(row));
                                        }
                                    },
                                    th { scope: "row", "{date}" }
                                    td { "{payee}" }
                                    td {
                                        "data-align": "end",
                                        "data-color": (amount > 0).then_some("success"),
                                        "{money(amount)}"
                                    }
                                    td { "data-align": "end", "{money(balance)}" }
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
    name: "Table",
    description: "Rows and columns of data, scrolling sideways when wide.",
    demo: TablePlaygroundDemo,
    source: "src/components/table.rs",
}
