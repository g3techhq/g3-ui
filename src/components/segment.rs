//! Segmented controls.
use super::HeaderToolbarContext;
use super::keyboard::use_roving_selection;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

struct SegmentContext<T: 'static> {
    value: Signal<T>,
    onchange: Option<EventHandler<T>>,
    defer_selection: bool,
    mode: ComponentMode,
}

// Written out: a derive would demand `T: Copy`.
impl<T> Clone for SegmentContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for SegmentContext<T> {}

/// A row of mutually exclusive buttons, like Ionic's `ion-segment`: a sliding
/// pill on iOS and an underlined tab strip on Material Design.
///
/// It behaves as a radio group: arrow keys move the selection. For buttons
/// that show and hide panels, use [`Tabs`](crate::Tabs), which adds the tab
/// semantics. Inside a [`Header`](crate::Header) toolbar it uses the toolbar
/// layout.
///
/// A segment always has a selection, so it takes a signal:
///
/// ```rust,ignore
/// let view = use_signal(|| View::Card);
/// rsx! {
///     SegmentGroup { value: view, aria_label: "View",
///         SegmentButton { value: View::Card, "Card" }
///         SegmentButton { value: View::Stats, "Stats" }
///     }
/// }
/// ```
#[component]
pub fn SegmentGroup<T: Clone + PartialEq + 'static>(
    /// The selected value.
    value: Signal<T>,
    /// Called with the value the user picks.
    onchange: Option<EventHandler<T>>,
    /// Only report picks through `onchange` and leave `value` alone, for a
    /// selection that follows something else such as the current route.
    defer_selection: Option<bool>,
    /// Accessible name of the group.
    aria_label: Option<String>,
    /// Let buttons keep their natural width and scroll sideways when they
    /// overflow.
    scrollable: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the group.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("segment", None);
    use_roving_selection(id.clone(), "[role=radio]", false);
    let in_toolbar = try_consume_context::<HeaderToolbarContext>().is_some();
    let context = SegmentContext {
        value,
        onchange,
        defer_selection: defer_selection.unwrap_or(false),
        mode,
    };
    let mut provided = use_context_provider(|| Signal::new(context));
    if *provided.peek() != context {
        provided.set(context);
    }
    rsx! {
        div {
            id,
            class: merge_classes(
                classes([
                    mode.pick("g3-segment-ios", "g3-segment-md"),
                    if in_toolbar { "g3-segment-toolbar" } else { "g3-segment-standalone" },
                    if scrollable.unwrap_or(false) { "g3-segment-scrollable" } else { "" },
                ]),
                class.as_deref(),
            ),
            role: "radiogroup",
            aria_label,
            {children}
        }
    }
}

impl<T: PartialEq> PartialEq for SegmentContext<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.onchange == other.onchange
            && self.defer_selection == other.defer_selection
            && self.mode == other.mode
    }
}

/// One choice in a [`SegmentGroup`].
#[component]
pub fn SegmentButton<T: Clone + PartialEq + 'static>(
    /// The value this button selects.
    value: T,
    /// Disable the button.
    disabled: Option<bool>,
    /// Accessible name, for icon-only buttons.
    aria_label: Option<String>,
    /// Extra classes for the button.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<Signal<SegmentContext<T>>>()();
    let mut group_value = context.value;
    let selected = *group_value.read() == value;
    let disabled = disabled.unwrap_or(false);
    rsx! {
        button {
            class: merge_classes(
                context.mode.pick("g3-segment-btn-ios", "g3-segment-btn-md"),
                class.as_deref(),
            ),
            r#type: "button",
            role: "radio",
            aria_checked: selected.to_string(),
            aria_label,
            tabindex: if selected { "0" } else { "-1" },
            disabled,
            onclick: move |_| {
                if disabled || *group_value.peek() == value {
                    return;
                }
                if let Some(onchange) = context.onchange {
                    onchange.call(value.clone());
                }
                if !context.defer_selection {
                    group_value.set(value.clone());
                }
            },
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SegmentPlaygroundDemo() -> Element {
    let view = use_signal(|| 0_usize);
    let scrollable = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: scrollable, label: "Scrollable" }
            },
            div { class: "playground-stack",
                SegmentGroup { value: view, aria_label: "Round view", scrollable: scrollable(),
                    SegmentButton { value: 0_usize, "Scorecard" }
                    SegmentButton { value: 1_usize, "Leaderboard" }
                    SegmentButton { value: 2_usize, "Stats" }
                    if scrollable() {
                        SegmentButton { value: 3_usize, "Course notes" }
                        SegmentButton { value: 4_usize, "Weather" }
                    }
                }
                crate::Card { title: "Selected", "Segment {view}" }
            }
        }
    }
}

crate::g3_playground! {
    name: "SegmentGroup",
    description: "Segmented control with platform styling and arrow-key selection.",
    demo: SegmentPlaygroundDemo,
    source: "src/components/segment.rs",
}
