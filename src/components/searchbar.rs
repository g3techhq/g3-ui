//! A search field.
use crate::components::{Input, InputType};
use crate::state::use_controlled;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::Search;

/// A search field with an icon and a clear button. Like Ionic's
/// `ion-searchbar`.
///
/// `onchange` fires after typing pauses for `debounce_ms` (300 by default),
/// which suits live filtering. `on_submit` fires on Enter.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let query = use_signal(String::new);
/// # fn filter(_query: String) {}
/// rsx! {
///     Header { title: "Players",
///         toolbar: rsx! { Searchbar { value: query, onchange: move |q| filter(q) } } }
/// }
/// # }
/// ```
#[component]
pub fn Searchbar(
    /// The query. Kept internally when not given.
    value: Option<Signal<String>>,
    /// Hint shown while empty. Defaults to
    /// [`Strings::search`](crate::Strings::search).
    placeholder: Option<String>,
    /// Accessible name. Defaults to the placeholder.
    aria_label: Option<String>,
    /// Delay before `onchange`, in milliseconds. Defaults to 300.
    debounce_ms: Option<u64>,
    /// Called with the query after typing pauses.
    onchange: Option<EventHandler<String>>,
    /// Called with the query when the user presses Enter.
    on_submit: Option<EventHandler<String>>,
    /// A control after the field, such as a filter button. It is separated
    /// from the field by a vertical rule.
    end: Option<Element>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the search form.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let value = use_controlled(value, String::new);
    let placeholder = placeholder.unwrap_or(strings.search);
    let aria_label = aria_label.unwrap_or_else(|| placeholder.clone());
    let cls = classes([
        "g3-searchbar",
        mode.pick("g3-searchbar-ios", "g3-searchbar-md"),
    ]);
    rsx! {
        form {
            class: merge_classes(cls, class.as_deref()),
            role: "search",
            onsubmit: move |event| {
                event.prevent_default();
                if let Some(on_submit) = on_submit {
                    on_submit.call(value());
                }
            },
            Input {
                value,
                input_type: InputType::Search,
                placeholder,
                aria_label,
                clearable: true,
                debounce_ms: debounce_ms.unwrap_or(300),
                onchange,
                mode,
                start: rsx! {
                    Search { size: 18 }
                },
                autocomplete: "off",
            }
            if let Some(end) = end {
                span { class: "g3-searchbar-end", {end} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SearchbarPlaygroundDemo() -> Element {
    let query = use_signal(String::new);
    let mut committed = use_signal(String::new);
    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack {
                Searchbar {
                    value: query,
                    onchange: move |q| committed.set(q),
                }
                crate::Text { tone: crate::TextTone::Secondary,
                    if committed().is_empty() { "Type to filter." } else { "Filtering by \"{committed}\"" }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Searchbar",
    description: "Search field with debounced changes and a clear button.",
    demo: SearchbarPlaygroundDemo,
    source: "src/components/searchbar.rs",
}
