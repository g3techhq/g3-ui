//! A filter over a list or table that keeps several choices, drawn as a chip.
use super::popover::{PopoverFrame, PopoverPlacement};
use super::select::{SelectOption, SelectOptionRow};
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronDown, X};

/// A filter over a list or table that keeps several choices, as a pill:
/// "Category" while it lets everything through, "Category: Dining" or
/// "Category: Dining +2" once it narrows, with a button that clears it.
///
/// It is the many-choice partner of [`FilterSelect`](super::FilterSelect).
/// An empty `value` lets everything through; each option picked narrows to
/// it, and a second press lets it go. The list stays open while choices are
/// ticked, and the pill names them in the list's order. `searchable` puts a
/// search at the top of the list for long ones, matching as a searchable
/// [`Select`](super::Select) does. The page reads the value and filters its
/// data.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let categories = use_signal(Vec::<&str>::new);
/// rsx! {
///     FilterChecklist {
///         label: "Category",
///         value: categories,
///         options: vec![
///             SelectOption::new("dining", "Dining"),
///             SelectOption::new("groceries", "Groceries"),
///         ],
///     }
/// }
/// # }
/// ```
#[component]
pub fn FilterChecklist<T: Clone + PartialEq + 'static>(
    /// What it filters on, such as "Category". Shown on the pill and names it.
    label: String,
    /// The chosen values, in the order they were picked. Empty lets
    /// everything through.
    value: Signal<Vec<T>>,
    /// The choices.
    options: Vec<SelectOption<T>>,
    /// Put a search field at the top of the list. Defaults to `false`.
    searchable: Option<bool>,
    /// Called with the chosen values after each pick or clear.
    onchange: Option<EventHandler<Vec<T>>>,
    /// Disable the filter.
    disabled: Option<bool>,
    /// Element id of the list. Generated when omitted.
    id: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the pill's wrapper.
    class: Option<String>,
) -> Element {
    let mut value = value;
    let mode = use_component_mode(mode);
    let strings = use_strings()();
    let list_id = use_element_id("filter-checklist", id);
    let mut open = use_signal(|| false);
    let mut query = use_signal(String::new);
    // Each opening starts a fresh search.
    use_effect(move || {
        if !open() {
            query.set(String::new());
        }
    });
    let toggle = use_callback(move |picked: T| {
        let chosen = {
            let mut list = value.write();
            match list.iter().position(|it| *it == picked) {
                Some(at) => {
                    list.remove(at);
                }
                None => list.push(picked),
            }
            list.clone()
        };
        if let Some(onchange) = onchange {
            onchange.call(chosen);
        }
    });

    let searchable = searchable.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let chosen = value();
    let active = !chosen.is_empty();
    let short = |option: &SelectOption<T>| {
        option
            .trigger_label
            .clone()
            .unwrap_or_else(|| option.label.clone())
    };
    // Named in the list's order, not the order they were picked.
    let picked: Vec<&SelectOption<T>> = options
        .iter()
        .filter(|option| chosen.contains(&option.value))
        .collect();
    let name = match picked.as_slice() {
        [] => label.clone(),
        [one] => format!("{label}: {}", short(one)),
        [first, rest @ ..] => format!("{label}: {} +{}", short(first), rest.len()),
    };
    let typed = query();
    let typed = typed.trim().to_string();
    let shown: Vec<SelectOption<T>> = if searchable && !typed.is_empty() {
        options
            .iter()
            .filter(|option| option.matches(&typed))
            .cloned()
            .map(|option| option.indent(0))
            .collect()
    } else {
        options.clone()
    };
    let first_match = shown
        .iter()
        .find(|option| !option.disabled)
        .map(|option| option.value.clone());
    let clear_label = format!("{} {label}", strings.clear);
    let trigger = rsx! {
        button {
            r#type: "button",
            role: "combobox",
            class: classes(["g3-input", "g3-select", mode.pick("g3-input-ios", "g3-input-md")]),
            disabled,
            aria_haspopup: "listbox",
            aria_expanded: open().to_string(),
            aria_controls: open().then(|| list_id.clone()),
            aria_label: name.clone(),
            onclick: move |_| open.toggle(),
            // Arrow keys open the list, as a native select does. Typing on
            // a searchable one opens it with the search begun.
            onkeydown: move |event| {
                let plain = !event
                    .modifiers()
                    .intersects(Modifiers::CONTROL | Modifiers::META | Modifiers::ALT);
                if open() {
                    if let Key::Character(text) = event.key()
                        && searchable
                        && plain
                    {
                        event.prevent_default();
                        query.write().push_str(&text);
                    }
                    return;
                }
                match event.key() {
                    Key::Named(NamedKey::ArrowDown) | Key::Named(NamedKey::ArrowUp) => {
                        event.prevent_default();
                        open.set(true);
                    }
                    Key::Character(text) if searchable && plain && !text.trim().is_empty() => {
                        event.prevent_default();
                        query.set(text);
                        open.set(true);
                    }
                    _ => {}
                }
            },
            span { class: "g3-select-value", "{name}" }
            ChevronDown { class: "g3-select-icon", size: 16 }
        }
    };

    rsx! {
        div {
            class: merge_classes("g3-filter", class.as_deref()),
            "data-active": active.then_some("true"),
            PopoverFrame {
                open,
                trigger,
                placement: PopoverPlacement::BottomStart,
                sheet_on_compact: true,
                role: "listbox",
                roving: Some("[role=option]"),
                aria_multiselectable: true,
                id: list_id.clone(),
                aria_label: label.clone(),
                mode,
                class: "g3-select-list",
                if searchable {
                    div { class: "g3-select-search",
                        input {
                            r#type: "search",
                            class: classes(["g3-input", "g3-select-search-input", mode.pick("g3-input-ios", "g3-input-md")]),
                            "data-g3-autofocus": "true",
                            autocomplete: "off",
                            spellcheck: "false",
                            aria_label: strings.search.clone(),
                            aria_controls: list_id.clone(),
                            placeholder: strings.search.clone(),
                            value: "{query}",
                            oninput: move |event| query.set(event.value()),
                            // Enter ticks or unticks the first match and
                            // keeps the list open for the next.
                            onkeydown: {
                                let first_match = first_match.clone();
                                move |event: KeyboardEvent| {
                                    if event.key() != Key::Named(NamedKey::Enter) {
                                        return;
                                    }
                                    event.prevent_default();
                                    if let Some(first) = first_match.clone() {
                                        toggle.call(first);
                                    }
                                }
                            },
                        }
                    }
                }
                for option in shown.iter().cloned() {
                    SelectOptionRow {
                        selected: chosen.contains(&option.value),
                        option,
                        onpick: move |picked: T| toggle.call(picked),
                    }
                }
                if searchable && shown.is_empty() {
                    div { class: "g3-select-empty", role: "status", "{strings.no_matches}" }
                }
            }
            if active && !disabled {
                button {
                    r#type: "button",
                    class: "g3-filter-clear",
                    aria_label: clear_label,
                    onclick: move |_| {
                        value.write().clear();
                        if let Some(onchange) = onchange {
                            onchange.call(Vec::new());
                        }
                    },
                    X { size: 14 }
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn FilterChecklistPlaygroundDemo() -> Element {
    let categories = use_signal(|| vec![2_u8]);
    let payees = use_signal(Vec::<u8>::new);

    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack { horizontal: true, wrap: true, align: crate::StackAlign::Center,
                FilterChecklist {
                    label: "Category",
                    value: categories,
                    options: vec![
                        SelectOption::new(0, "Auto"),
                        SelectOption::new(1, "Fuel").indent(1),
                        SelectOption::new(2, "Dining"),
                        SelectOption::new(3, "Groceries"),
                        SelectOption::new(4, "Utilities"),
                    ],
                }
                FilterChecklist {
                    label: "Payee",
                    value: payees,
                    searchable: true,
                    options: vec![
                        SelectOption::new(0, "Corner Bakery"),
                        SelectOption::new(1, "City Water"),
                        SelectOption::new(2, "Fresh Market"),
                        SelectOption::new(3, "Northside Gas"),
                        SelectOption::new(4, "Riverside Grill"),
                        SelectOption::new(5, "Sunrise Coffee"),
                    ],
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "FilterChecklist",
    description: "A filter that keeps several choices, drawn as a pill with a clear button.",
    demo: FilterChecklistPlaygroundDemo,
    source: "src/components/filter_checklist.rs",
}
