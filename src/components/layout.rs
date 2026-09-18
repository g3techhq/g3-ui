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
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     Stack { horizontal: true, justify: StackJustify::Between, align: StackAlign::Center,
///         Text { variant: TextVariant::Heading, "Players" }
///         Button { fill: ButtonFill::Clear, "Add" }
///     }
/// }
/// # }
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

impl GridColumns {
    fn template(self) -> String {
        match self {
            GridColumns::Count(count) => format!("repeat({}, minmax(0, 1fr))", count.max(1)),
            GridColumns::Fit(min) => {
                format!("repeat(auto-fill, minmax(min({min}rem, 100%), 1fr))")
            }
        }
    }
}

/// Lays children out in a grid of equal columns.
///
/// `columns` and `gap` apply on phone-width shells. `wide_columns` and
/// `wide_gap` take over once the app shell is `48rem` or wider, the same
/// breakpoint where navigation becomes a rail.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let courses = use_signal(Vec::<String>::new);
/// # #[component] fn CourseCard(course: String) -> Element { rsx! {} }
/// rsx! {
///     Grid {
///         columns: GridColumns::Count(1),
///         wide_columns: GridColumns::Count(3),
///         wide_gap: Space::Xl,
///         for course in courses() { CourseCard { course } }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Grid(
    /// Column sizing. Defaults to as many 12rem columns as fit.
    columns: Option<GridColumns>,
    /// Column sizing on wide shells. Defaults to `columns`.
    wide_columns: Option<GridColumns>,
    /// Space between cells. Defaults to [`Space::Md`].
    gap: Option<Space>,
    /// Space between cells on wide shells. Defaults to `gap`.
    wide_gap: Option<Space>,
    /// Extra classes for the grid.
    class: Option<String>,
    children: Element,
) -> Element {
    let mut style = format!(
        "--g3-grid-columns: {};",
        columns.unwrap_or_default().template()
    );
    if let Some(wide) = wide_columns {
        style.push_str(&format!("--g3-grid-columns-wide: {};", wide.template()));
    }
    rsx! {
        div {
            class: merge_classes("g3-grid", class.as_deref()),
            "data-gap": gap.unwrap_or_default().as_str(),
            "data-wide-gap": wide_gap.map(Space::as_str),
            style,
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn LayoutPlaygroundDemo() -> Element {
    use crate::{Card, CardVariant, Text, TextTone, TextVariant};
    let compact = use_signal(|| true);
    let columns = use_signal(|| 1_i64);
    let wide_columns = use_signal(|| 3_i64);
    let gap = use_signal(|| Space::Md);
    let wide_gap = use_signal(|| Space::Xl);
    let space_options = || {
        vec![
            crate::SelectOption::new(Space::None, "None"),
            crate::SelectOption::new(Space::Xs, "Extra small"),
            crate::SelectOption::new(Space::Sm, "Small"),
            crate::SelectOption::new(Space::Md, "Medium"),
            crate::SelectOption::new(Space::Lg, "Large"),
            crate::SelectOption::new(Space::Xl, "Extra large"),
        ]
    };
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::SegmentGroup { value: compact, aria_label: "Shell width",
                    crate::SegmentButton { value: true, "Phone settings" }
                    crate::SegmentButton { value: false, "Wide settings" }
                }
                if compact() {
                    crate::Stepper { label: "Columns", value: columns, min: 1, max: 4 }
                    crate::Select { label: "Gap", value: gap, options: space_options() }
                } else {
                    crate::Stepper { label: "Wide columns", value: wide_columns, min: 1, max: 6 }
                    crate::Select { label: "Wide gap", value: wide_gap, options: space_options() }
                }
                Text { variant: TextVariant::Caption, tone: TextTone::Secondary,
                    "Switch the viewport to Desktop to see the wide settings."
                }
            },
            Stack { gap: Space::Lg,
                Stack { horizontal: true, justify: StackJustify::Between, align: StackAlign::Baseline,
                    Text { variant: TextVariant::Title, "Leaderboard" }
                    Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Updated just now" }
                }
                Text { variant: TextVariant::Overline, tone: TextTone::Tertiary, "Courses" }
                Grid {
                    columns: GridColumns::Count(columns() as u16),
                    wide_columns: GridColumns::Count(wide_columns() as u16),
                    gap: gap(),
                    wide_gap: wide_gap(),
                    for name in ["Pebble Creek", "Oak Hollow", "Cedar Ridge", "Lakeside", "Pine Valley", "Harbor Links"] {
                        Card { key: "{name}", variant: CardVariant::Filled, title: name, "18 holes" }
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
    components: ["Stack", "Grid", "Text"],
    demo: LayoutPlaygroundDemo,
    source: "src/components/layout.rs",
}
