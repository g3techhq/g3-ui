//! Layout helpers: stacks and grids.
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// A spacing step, shared by [`Stack`] and [`Grid`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Space {
    /// No space.
    None,
    /// 4px.
    Xs,
    /// 8px.
    Sm,
    /// 12px.
    #[default]
    Md,
    /// 16px.
    Lg,
    /// 24px.
    Xl,
}

impl Space {
    fn as_str(self) -> &'static str {
        match self {
            Space::None => "none",
            Space::Xs => "xs",
            Space::Sm => "sm",
            Space::Md => "md",
            Space::Lg => "lg",
            Space::Xl => "xl",
        }
    }
}

/// Cross-axis alignment in a [`Stack`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum StackAlign {
    /// Fill the cross axis.
    #[default]
    Stretch,
    /// Align to the start.
    Start,
    /// Centre.
    Center,
    /// Align to the end.
    End,
    /// Align text baselines, for rows mixing sizes.
    Baseline,
}

/// Main-axis distribution in a [`Stack`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum StackJustify {
    /// Pack at the start.
    #[default]
    Start,
    /// Centre.
    Center,
    /// Pack at the end.
    End,
    /// Spread with the space between items.
    Between,
}

/// Lays children out in a column or row with even spacing.
///
/// ```rust,ignore
/// rsx! {
///     Stack { horizontal: true, justify: StackJustify::Between, align: StackAlign::Center,
///         Text { variant: TextVariant::Heading, "Players" }
///         Button { fill: ButtonFill::Clear, "Add" }
///     }
/// }
/// ```
#[component]
pub fn Stack(
    /// Lay out in a row instead of a column.
    horizontal: Option<bool>,
    /// Space between children. Defaults to [`Space::Md`].
    gap: Option<Space>,
    /// Cross-axis alignment. Defaults to [`StackAlign::Stretch`].
    align: Option<StackAlign>,
    /// Main-axis distribution. Defaults to [`StackJustify::Start`].
    justify: Option<StackJustify>,
    /// Wrap onto more lines when the row is full.
    wrap: Option<bool>,
    /// Extra classes for the stack.
    class: Option<String>,
    children: Element,
) -> Element {
    let align = match align.unwrap_or_default() {
        StackAlign::Stretch => "stretch",
        StackAlign::Start => "start",
        StackAlign::Center => "center",
        StackAlign::End => "end",
        StackAlign::Baseline => "baseline",
    };
    let justify = match justify.unwrap_or_default() {
        StackJustify::Start => "start",
        StackJustify::Center => "center",
        StackJustify::End => "end",
        StackJustify::Between => "between",
    };
    let cls = classes([
        "g3-stack",
        if horizontal.unwrap_or(false) {
            "g3-stack-row"
        } else {
            ""
        },
        if wrap.unwrap_or(false) {
            "g3-stack-wrap"
        } else {
            ""
        },
    ]);
    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            "data-gap": gap.unwrap_or_default().as_str(),
            "data-align": align,
            "data-justify": justify,
            {children}
        }
    }
}

/// How a [`Grid`] sizes its columns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridColumns {
    /// A fixed number of equal columns.
    Count(u16),
    /// As many columns as fit, each at least this many rem wide.
    Fit(f32),
}

impl Default for GridColumns {
    fn default() -> Self {
        GridColumns::Fit(12.0)
    }
}

/// Lays children out in a responsive grid of equal columns.
///
/// ```rust,ignore
/// rsx! { Grid { columns: GridColumns::Fit(14.0), for course in courses() { CourseCard { course } } } }
/// ```
#[component]
pub fn Grid(
    /// Column sizing. Defaults to as many 12rem columns as fit.
    columns: Option<GridColumns>,
    /// Space between cells. Defaults to [`Space::Md`].
    gap: Option<Space>,
    /// Extra classes for the grid.
    class: Option<String>,
    children: Element,
) -> Element {
    let template = match columns.unwrap_or_default() {
        GridColumns::Count(count) => format!("repeat({}, minmax(0, 1fr))", count.max(1)),
        GridColumns::Fit(min) => format!("repeat(auto-fill, minmax(min({min}rem, 100%), 1fr))"),
    };
    rsx! {
        div {
            class: merge_classes("g3-grid", class.as_deref()),
            "data-gap": gap.unwrap_or_default().as_str(),
            style: "--g3-grid-columns: {template};",
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn LayoutPlaygroundDemo() -> Element {
    use crate::{Card, Text, TextTone, TextVariant};
    let columns = use_signal(|| 2_i64);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::Stepper { label: "Grid columns", value: columns, min: 1, max: 4 }
            },
            Stack { gap: Space::Lg,
                Stack { horizontal: true, justify: StackJustify::Between, align: StackAlign::Baseline,
                    Text { variant: TextVariant::Title, "Leaderboard" }
                    Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Updated just now" }
                }
                Text { variant: TextVariant::Overline, tone: TextTone::Tertiary, "Courses" }
                Grid { columns: GridColumns::Count(columns() as u16),
                    for name in ["Pebble Creek", "Oak Hollow", "Cedar Ridge", "Lakeside"] {
                        Card { key: "{name}", inset: true, title: name, "18 holes" }
                    }
                }
                Text { color: crate::Color::Danger, "Payment failed for one entry." }
            }
        }
    }
}

crate::g3_playground! {
    name: "Layout",
    description: "Stacks, grids, and text styles.",
    demo: LayoutPlaygroundDemo,
    source: "src/components/layout.rs",
}
