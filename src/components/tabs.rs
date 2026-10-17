//! Tabs that switch between panels.
use super::gesture::{GestureScript, use_gesture};
use super::hscroll::use_horizontal_scroll;
use super::keyboard::use_roving_selection;
use crate::state::{provide_live_context, use_element_id};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

struct TabsContext<T: 'static> {
    value: Signal<T>,
    onchange: Option<EventHandler<T>>,
    id: Signal<String>,
    mode: ComponentMode,
    previous: Signal<Option<u64>>,
    animating: Signal<bool>,
    animated: bool,
}

impl<T> Clone for TabsContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TabsContext<T> {}

impl<T> PartialEq for TabsContext<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
            && self.onchange == other.onchange
            && self.id == other.id
            && self.mode == other.mode
            && self.previous == other.previous
            && self.animating == other.animating
            && self.animated == other.animated
    }
}

#[derive(Clone, Copy)]
struct TabsMode(Signal<ComponentMode>);

const TABS_SCRIPT: GestureScript = GestureScript {
    name: "g3-ui.tabs",
    source: include_str!("tabs.js"),
};

impl<T: Hash> TabsContext<T> {
    fn key(&self, value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    fn ids(&self, value: &T) -> (String, String) {
        let key = self.key(value);
        let id = self.id.peek();
        (format!("{id}-tab-{key:x}"), format!("{id}-panel-{key:x}"))
    }
}

/// Tabs that show one panel at a time, with full tab semantics: arrow keys
/// move between tabs, and each tab controls its panel.
///
/// Put a [`TabList`] of [`Tab`]s and the matching [`TabPanel`]s inside.
/// Values must be hashable, to link each tab to its panel.
///
/// # Tabs or a segment?
///
/// Both look alike. The difference is what the choice does:
///
/// - **`Tabs`** switch between sections of content in place. Each tab is
///   tied to its panel, so screen readers announce "tab 2 of 3" and can jump
///   from the tab to the panel it shows. Panels that are not selected are not
///   rendered, unless `keep_mounted`.
/// - **[`SegmentGroup`](crate::SegmentGroup)** picks a value, like a
///   radio group styled as buttons: a view mode, a filter, a unit. It shows
///   nothing itself; your code reads the value and decides what changes.
///   Use it in toolbars and forms, or for routed tabs where each choice
///   navigates.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Clone, Copy, PartialEq, Eq, Hash)] enum Tab { Scores, Notes }
/// # #[component] fn Scores() -> Element { rsx! {} }
/// # #[component] fn Notes() -> Element { rsx! {} }
/// let tab = use_signal(|| Tab::Scores);
/// rsx! {
///     Tabs { value: tab,
///         TabList { aria_label: "Round",
///             Tab { value: Tab::Scores, "Scores" }
///             Tab { value: Tab::Notes, "Notes" }
///         }
///         TabPanel { value: Tab::Scores, Scores {} }
///         TabPanel { value: Tab::Notes, Notes {} }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Tabs<T: Clone + PartialEq + Hash + 'static>(
    /// The selected tab.
    value: Signal<T>,
    /// Called with the tab the user picks.
    onchange: Option<EventHandler<T>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Animate the newly selected panel. Defaults to `true`.
    animated: Option<bool>,
    /// Let a horizontal touch gesture select the adjacent tab. Defaults to
    /// `false`, since some panels own horizontal gestures of their own.
    swipe: Option<bool>,
    /// Extra classes for the wrapper.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("tabs", None);
    let id_signal = use_signal(|| id.clone());
    let previous = use_signal(|| None::<u64>);
    let animating = use_signal(|| false);
    let animated = animated.unwrap_or(true);
    let context = TabsContext {
        value,
        onchange,
        id: id_signal,
        mode,
        previous,
        animating,
        animated,
    };
    provide_live_context(context);
    let tab_mode = crate::state::use_synced_signal(mode);
    use_context_provider(|| TabsMode(tab_mode));
    let start_script = use_gesture(TABS_SCRIPT, id.clone(), |_| {});
    let swipe = swipe.unwrap_or(false);
    rsx! {
        div {
            id,
            class: merge_classes("g3-tabs", class.as_deref()),
            "data-animated": animated.to_string(),
            "data-direction": "forward",
            "data-swipe": swipe.to_string(),
            onmounted: move |_| start_script.call(()),
            {children}
        }
    }
}

/// The row of [`Tab`]s inside [`Tabs`].
#[component]
pub fn TabList(
    /// Accessible name of the tab list.
    aria_label: Option<String>,
    /// Let tabs keep their natural width and scroll sideways.
    scrollable: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the tab list.
    class: Option<String>,
    children: Element,
) -> Element {
    let group_mode = try_consume_context::<TabsMode>().map(|mode| mode.0());
    let mode = use_component_mode(mode.or(group_mode));
    let id = use_element_id("tablist", None);
    use_roving_selection(id.clone(), "[role=tab]", false);
    use_horizontal_scroll(id.clone(), scrollable.unwrap_or(false));
    let in_toolbar = try_consume_context::<super::HeaderToolbarContext>().is_some();
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
            role: "tablist",
            aria_label,
            {children}
        }
    }
}

/// One tab in a [`TabList`].
#[component]
pub fn Tab<T: Clone + PartialEq + Hash + 'static>(
    /// The panel value this tab selects.
    value: T,
    /// Disable the tab.
    disabled: Option<bool>,
    /// Accessible name, for a tab that shows only an icon.
    aria_label: Option<String>,
    /// Extra classes for the tab.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<Signal<TabsContext<T>>>()();
    let (tab_id, panel_id) = context.ids(&value);
    let mut selected_value = context.value;
    let selected = *selected_value.read() == value;
    rsx! {
        button {
            id: tab_id,
            class: merge_classes(
                context.mode.pick("g3-segment-btn-ios", "g3-segment-btn-md"),
                class.as_deref(),
            ),
            r#type: "button",
            role: "tab",
            aria_selected: selected.to_string(),
            aria_label,
            // Only the selected panel is sure to be mounted.
            aria_controls: selected.then_some(panel_id),
            tabindex: if selected { "0" } else { "-1" },
            disabled,
            onclick: move |_| {
                if *selected_value.peek() == value {
                    return;
                }
                let current_key = context.key(&selected_value.peek());
                if context.animated {
                    let mut previous = context.previous;
                    let mut animating = context.animating;
                    previous.set(Some(current_key));
                    animating.set(true);
                }
                selected_value.set(value.clone());
                if let Some(onchange) = context.onchange {
                    onchange.call(value.clone());
                }
            },
            {children}
        }
    }
}

/// The content shown while its [`Tab`] is selected.
#[component]
pub fn TabPanel<T: Clone + PartialEq + Hash + 'static>(
    /// The tab value this panel belongs to.
    value: T,
    /// Keep the panel rendered, hidden, while another tab is selected, so its
    /// state survives switching. Defaults to `false`.
    keep_mounted: Option<bool>,
    /// Extra classes for the panel.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<Signal<TabsContext<T>>>()();
    let (tab_id, panel_id) = context.ids(&value);
    let key = context.key(&value);
    let selected = *context.value.read() == value;
    let leaving =
        context.animated && *context.animating.read() && *context.previous.read() == Some(key);
    if !selected && !leaving && !keep_mounted.unwrap_or(false) {
        return rsx! {};
    }
    let state = if leaving {
        "leaving"
    } else if selected && context.animated && *context.animating.read() {
        "entering"
    } else if selected {
        "current"
    } else {
        "hidden"
    };
    rsx! {
        div {
            id: panel_id,
            class: merge_classes("g3-tab-panel", class.as_deref()),
            role: "tabpanel",
            "data-state": state,
            aria_labelledby: tab_id,
            aria_hidden: (!selected).then_some("true"),
            inert: (!selected).then_some(true),
            tabindex: "0",
            hidden: !selected && !leaving,
            onanimationend: move |event| {
                if selected
                    && event.data().animation_name().starts_with("g3-tab-panel-enter")
                    && *context.animating.peek()
                {
                    let mut animating = context.animating;
                    let mut previous = context.previous;
                    animating.set(false);
                    previous.set(None);
                }
            },
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TabsPlaygroundDemo() -> Element {
    let tab = use_signal(|| "scores");
    let mut taps = use_signal(|| 0_u32);
    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack {
                crate::Text { variant: crate::TextVariant::Caption, tone: crate::TextTone::Secondary,
                    "On a touch screen, swipe the panel left or right to move between tabs."
                }
                Tabs { value: tab, swipe: true,
                    TabList { aria_label: "Round",
                        Tab { value: "scores", "Scores" }
                        Tab { value: "players", "Players" }
                        Tab { value: "notes", "Notes" }
                    }
                    TabPanel { value: "scores",
                        crate::Card { title: "Scores", "Front nine: 38" }
                        crate::Button { onclick: move |_| taps += 1, "Panel action: {taps}" }
                    }
                    TabPanel { value: "players",
                        crate::Card { title: "Players", "Four players in this group." }
                    }
                    TabPanel { value: "notes", keep_mounted: true,
                        crate::TextArea { label: "Notes" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Tabs",
    description: "Switch between panels of content in place. Use SegmentGroup to pick a value instead.",
    components: ["Tabs", "TabList", "Tab", "TabPanel"],
    demo: TabsPlaygroundDemo,
    source: "src/components/tabs.rs",
}
