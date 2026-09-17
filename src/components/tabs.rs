//! Tabs that switch between panels.
use super::keyboard::use_roving_selection;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;
use std::hash::{DefaultHasher, Hash, Hasher};

struct TabsContext<T: 'static> {
    value: Signal<T>,
    onchange: Option<EventHandler<T>>,
    id: Signal<String>,
    mode: ComponentMode,
}

impl<T> Clone for TabsContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TabsContext<T> {}

impl<T: Hash> TabsContext<T> {
    fn ids(&self, value: &T) -> (String, String) {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        let key = hasher.finish();
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
/// ```rust,ignore
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
/// ```
#[component]
pub fn Tabs<T: Clone + PartialEq + Hash + 'static>(
    /// The selected tab.
    value: Signal<T>,
    /// Called with the tab the user picks.
    onchange: Option<EventHandler<T>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the wrapper.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("tabs", None);
    let id_signal = use_signal(|| id.clone());
    let context = TabsContext {
        value,
        onchange,
        id: id_signal,
        mode,
    };
    let mut provided = use_context_provider(|| Signal::new(context));
    if provided.peek().onchange != onchange || provided.peek().mode != mode {
        provided.set(context);
    }
    rsx! {
        div { id, class: merge_classes("g3-tabs", class.as_deref()), {children} }
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
    let mode = use_component_mode(mode);
    let id = use_element_id("tablist", None);
    use_roving_selection(id.clone(), "[role=tab]", false);
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
            aria_controls: panel_id,
            tabindex: if selected { "0" } else { "-1" },
            disabled,
            onclick: move |_| {
                if *selected_value.peek() == value {
                    return;
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
    let selected = *context.value.read() == value;
    if !selected && !keep_mounted.unwrap_or(false) {
        return rsx! {};
    }
    rsx! {
        div {
            id: panel_id,
            class: merge_classes("g3-tab-panel", class.as_deref()),
            role: "tabpanel",
            aria_labelledby: tab_id,
            tabindex: "0",
            hidden: !selected,
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn TabsPlaygroundDemo() -> Element {
    let tab = use_signal(|| "scores");
    rsx! {
        crate::PlaygroundDemoFrame {
            Tabs { value: tab,
                TabList { aria_label: "Round",
                    Tab { value: "scores", "Scores" }
                    Tab { value: "players", "Players" }
                    Tab { value: "notes", "Notes" }
                }
                TabPanel { value: "scores",
                    crate::Card { title: "Scores", "Front nine: 38" }
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

crate::g3_playground! {
    name: "Tabs",
    description: "Tabs with panels and full keyboard support.",
    demo: TabsPlaygroundDemo,
    source: "src/components/tabs.rs",
}
