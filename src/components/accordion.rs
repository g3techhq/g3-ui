//! Expandable sections.
use crate::state::{use_controlled, use_element_id, use_synced_signal};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronDown;

struct AccordionContext<T: 'static> {
    value: Signal<Vec<T>>,
    multiple: Signal<bool>,
    disabled: Signal<bool>,
    onchange: Option<EventHandler<Vec<T>>>,
}

impl<T> Clone for AccordionContext<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for AccordionContext<T> {}

/// The open values after pressing `item`.
fn toggled<T: Clone + PartialEq>(current: &[T], item: &T, multiple: bool) -> Vec<T> {
    if current.contains(item) {
        current
            .iter()
            .filter(|value| *value != item)
            .cloned()
            .collect()
    } else if multiple {
        let mut next = current.to_vec();
        next.push(item.clone());
        next
    } else {
        vec![item.clone()]
    }
}

/// A group of expandable sections. Like Ionic's `ion-accordion-group`.
///
/// `value` lists the open items. By default opening one closes the others.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[component] fn Rules() -> Element { rsx! {} }
/// # #[component] fn Prizes() -> Element { rsx! {} }
/// rsx! {
///     AccordionGroup::<&str> { multiple: true,
///         AccordionItem { value: "rules", label: "Rules", Rules {} }
///         AccordionItem { value: "prizes", label: "Prizes", Prizes {} }
///     }
/// }
/// # }
/// ```
#[component]
pub fn AccordionGroup<T: Clone + PartialEq + 'static>(
    /// The open items. Kept internally, starting closed, when not given.
    value: Option<Signal<Vec<T>>>,
    /// Allow several items open at once.
    multiple: Option<bool>,
    /// Disable every item.
    disabled: Option<bool>,
    /// Called with the open items after a change.
    onchange: Option<EventHandler<Vec<T>>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the group.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let value = use_controlled(value, Vec::new);
    let multiple = use_synced_signal(multiple.unwrap_or(false));
    let disabled = use_synced_signal(disabled.unwrap_or(false));
    use_context_provider(|| AccordionContext {
        value,
        multiple,
        disabled,
        onchange,
    });
    rsx! {
        div {
            class: merge_classes(
                classes(["g3-accordion-group", mode.pick("g3-accordion-group-ios", "g3-accordion-group-md")]),
                class.as_deref(),
            ),
            {children}
        }
    }
}

/// One expandable section of an [`AccordionGroup`].
#[component]
pub fn AccordionItem<T: Clone + PartialEq + 'static>(
    /// Identifies this item in the group's open list.
    value: T,
    /// Header text.
    label: Option<String>,
    /// Secondary header text.
    description: Option<String>,
    /// Heading level of the header, 1 to 6. Defaults to 2; set it so the
    /// page's headings do not skip a level.
    heading_level: Option<u8>,
    /// Custom header content, used instead of `label` and `description`.
    header: Option<Element>,
    /// Disable this item.
    disabled: Option<bool>,
    /// Extra classes for the item.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<AccordionContext<T>>();
    let id = use_element_id("accordion", None);
    let panel_id = format!("{id}-panel");
    let button_id = format!("{id}-button");
    let mut open_values = context.value;
    let expanded = open_values.read().contains(&value);
    let disabled = disabled.unwrap_or(false) || (context.disabled)();
    let state = if expanded { "open" } else { "closed" };
    rsx! {
        div {
            class: merge_classes("g3-accordion-item", class.as_deref()),
            "data-state": state,
            "data-disabled": disabled.then_some("true"),
            super::text::Heading { level: heading_level.unwrap_or(2), class: "g3-accordion-heading",
                button {
                    id: button_id.clone(),
                    class: "g3-accordion-header",
                    r#type: "button",
                    disabled,
                    aria_expanded: expanded.to_string(),
                    aria_controls: panel_id.clone(),
                    onclick: move |_| {
                        let next = toggled(&open_values.peek(), &value, (context.multiple)());
                        open_values.set(next.clone());
                        if let Some(onchange) = context.onchange {
                            onchange.call(next);
                        }
                    },
                    span { class: "g3-accordion-header-text",
                        if let Some(header) = header {
                            {header}
                        } else {
                            if let Some(label) = label {
                                span { class: "g3-accordion-label", "{label}" }
                            }
                            if let Some(description) = description {
                                span { class: "g3-accordion-description", "{description}" }
                            }
                        }
                    }
                    span { class: "g3-accordion-chevron", aria_hidden: "true",
                        ChevronDown { size: 18 }
                    }
                }
            }
            div {
                id: panel_id,
                class: "g3-accordion-panel",
                role: "region",
                "data-state": state,
                aria_labelledby: button_id,
                inert: (!expanded).then_some(true),
                aria_hidden: (!expanded).then_some("true"),
                div { class: "g3-accordion-content",
                    div { class: "g3-accordion-body", {children} }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::toggled;

    #[test]
    fn single_mode_keeps_one_item_open() {
        assert_eq!(toggled(&["a"], &"b", false), vec!["b"]);
        assert_eq!(toggled(&["a"], &"a", false), Vec::<&str>::new());
    }

    #[test]
    fn multiple_mode_adds_and_removes() {
        assert_eq!(toggled(&["a"], &"b", true), vec!["a", "b"]);
        assert_eq!(toggled(&["a", "b"], &"a", true), vec!["b"]);
    }
}

#[cfg(feature = "playground")]
#[component]
fn AccordionPlaygroundDemo() -> Element {
    let multiple = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: multiple, label: "Multiple open" }
            },
            AccordionGroup::<&'static str> { multiple: multiple(),
                AccordionItem { value: "rules", label: "Rules", description: "Stroke play, full handicap",
                    "Lowest net score wins. Ties go to a scorecard playoff."
                }
                AccordionItem { value: "prizes", label: "Prizes",
                    "Closest to the pin on 7 and 16."
                }
                AccordionItem { value: "weather", label: "Weather policy", disabled: true,
                    "Play continues in light rain."
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Accordion",
    description: "Expandable sections, one or several open at a time.",
    components: ["AccordionGroup", "AccordionItem"],
    demo: AccordionPlaygroundDemo,
    source: "src/components/accordion.rs",
}
