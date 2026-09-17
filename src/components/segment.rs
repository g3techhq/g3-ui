//! Segmented controls.
use super::HeaderToolbarContext;
use super::keyboard::use_roving_selection;
use super::overlay::js_string;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Sideways scrolling for a scrollable group: touch scrolls natively; this
/// adds mouse dragging, vertical wheels, edge fades while more is hidden, and
/// keeps the selected button in view.
const SCROLL_SCRIPT: &str = r#"
const group = document.getElementById(__ID__);
if (group && group.dataset.g3Scroll !== "true") {
    group.dataset.g3Scroll = "true";
    const edges = () => {
        const max = group.scrollWidth - group.clientWidth;
        group.dataset.overflowStart = String(group.scrollLeft > 1);
        group.dataset.overflowEnd = String(group.scrollLeft < max - 1);
    };
    const reveal = (smooth) => {
        const selected = group.querySelector("[aria-checked=true], [aria-selected=true]");
        if (!selected) return;
        // The group is positioned, so this is already relative to it.
        const left = selected.offsetLeft;
        const right = left + selected.offsetWidth;
        if (left < group.scrollLeft + 16) {
            group.scrollTo({ left: left - 16, behavior: smooth ? "smooth" : "auto" });
        } else if (right > group.scrollLeft + group.clientWidth - 16) {
            group.scrollTo({ left: right - group.clientWidth + 16, behavior: smooth ? "smooth" : "auto" });
        }
    };
    let down = false;
    let dragged = false;
    let startX = 0;
    let startLeft = 0;
    group.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        down = true;
        dragged = false;
        startX = event.clientX;
        startLeft = group.scrollLeft;
    });
    window.addEventListener("pointermove", (event) => {
        if (!down) return;
        const dx = event.clientX - startX;
        if (!dragged && Math.abs(dx) < 5) return;
        dragged = true;
        group.dataset.dragging = "true";
        group.scrollLeft = startLeft - dx;
    });
    window.addEventListener("pointerup", () => {
        down = false;
        delete group.dataset.dragging;
    });
    // A drag must not also pick the button it ended on.
    group.addEventListener("click", (event) => {
        if (dragged) {
            dragged = false;
            event.preventDefault();
            event.stopPropagation();
        }
    }, true);
    group.addEventListener("wheel", (event) => {
        if (Math.abs(event.deltaY) <= Math.abs(event.deltaX)) return;
        if (group.scrollWidth <= group.clientWidth) return;
        event.preventDefault();
        group.scrollLeft += event.deltaY;
    }, { passive: false });
    group.addEventListener("scroll", edges, { passive: true });
    new ResizeObserver(edges).observe(group);
    new MutationObserver(() => { reveal(true); edges(); })
        .observe(group, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-checked", "aria-selected"] });
    reveal(false);
    edges();
}
"#;

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
    {
        let id = id.clone();
        use_effect(use_reactive!(|scrollable| {
            if scrollable {
                document::eval(&SCROLL_SCRIPT.replace("__ID__", &js_string(&id)));
            }
        }));
    }
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
                    if scrollable { "g3-segment-scrollable" } else { "" },
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
    demo: SegmentPlaygroundDemo,
    source: "src/components/segment.rs",
}
