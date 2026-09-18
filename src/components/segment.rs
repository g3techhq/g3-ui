//! Segmented controls.
use super::HeaderToolbarContext;
use super::hscroll::use_horizontal_scroll;
use super::keyboard::use_roving_selection;
use crate::state::{provide_live_context, use_element_id, use_live_context};
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
/// It behaves as a radio group: it picks a value, and arrow keys move the
/// selection. For buttons that switch between panels of content, use
/// [`Tabs`](crate::Tabs), which ties each button to its panel for screen
/// readers; see its docs for when to use which. Inside a [`Header`](crate::Header) toolbar it uses the toolbar
/// layout.
///
/// A segment always has a selection, so it takes a signal:
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Clone, Copy, PartialEq)] enum View { Card, Stats }
/// let view = use_signal(|| View::Card);
/// rsx! {
///     SegmentGroup { value: view, aria_label: "View",
///         SegmentButton { value: View::Card, "Card" }
///         SegmentButton { value: View::Stats, "Stats" }
///     }
/// }
/// # }
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
    /// Visible label above the group, which also names it, as on the other
    /// form controls. Leave it out in a toolbar, where the group names itself
    /// through `aria_label`.
    label: Option<String>,
    /// Accessible name of the group when there is no visible label.
    aria_label: Option<String>,
    /// Let buttons keep their natural width and scroll sideways when they
    /// overflow: by touch, mouse drag, or wheel. Otherwise buttons share the
    /// width equally and long labels are truncated.
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
    let scrollable = scrollable.unwrap_or(false);
    use_horizontal_scroll(id.clone(), scrollable);
    let in_toolbar = try_consume_context::<HeaderToolbarContext>().is_some();
    let context = SegmentContext {
        value,
        onchange,
        defer_selection: defer_selection.unwrap_or(false),
        mode,
    };
    provide_live_context(context);
    let group = rsx! {
        div {
            id: id.clone(),
            class: merge_classes(
                classes([
                    mode.pick("g3-segment-ios", "g3-segment-md"),
                    if in_toolbar { "g3-segment-toolbar" } else { "g3-segment-standalone" },
                    if scrollable { "g3-segment-scrollable" } else { "" },
                ]),
                class.as_deref(),
            ),
            role: "radiogroup",
            aria_label: if label.is_none() { aria_label } else { None },
            aria_labelledby: label.as_ref().map(|_| format!("{id}-label")),
            {children}
        }
    };
    match label {
        // Not a <label>: a radio group is not a labelable element, so the name
        // comes from aria-labelledby and the text only has to be visible.
        Some(label) => rsx! {
            div { class: "g3-field",
                span { id: "{id}-label", class: "g3-field-label", "{label}" }
                {group}
            }
        },
        None => group,
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
    let context = use_live_context::<SegmentContext<T>>();
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
    let scrollable = use_signal(|| true);
    let labels = [
        "Scorecard",
        "Leaderboard",
        "Stats",
        "Course notes",
        "Weather",
        "Photos",
    ];
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: scrollable, label: "Scrollable" }
                crate::Text { variant: crate::TextVariant::Caption, tone: crate::TextTone::Secondary,
                    if scrollable() {
                        "Buttons keep their width. Swipe, drag, or use the wheel to see the rest."
                    } else {
                        "Buttons share the width and long labels are shortened."
                    }
                }
            },
            crate::Stack {
                SegmentGroup { value: view, aria_label: "Round view", scrollable: scrollable(),
                    for (index, label) in labels.iter().enumerate() {
                        SegmentButton { key: "{index}", value: index, "{label}" }
                    }
                }
                crate::Card { title: labels[view()], "Content for the selected segment." }
            }
        }
    }
}

crate::g3_playground! {
    name: "SegmentGroup",
    description: "Pick one value from a row of buttons. Use Tabs to switch between panels of content instead.",
    components: ["SegmentGroup", "SegmentButton"],
    demo: SegmentPlaygroundDemo,
    source: "src/components/segment.rs",
}
