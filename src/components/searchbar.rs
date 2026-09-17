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
/// ```rust,ignore
/// rsx! {
///     Header { title: "Players",
///         toolbar: rsx! { Searchbar { value: query, onchange: move |q| filter(q) } } }
/// }
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
    /// Show a Cancel button beside the field, as iOS does.
    show_cancel: Option<bool>,
    /// Called with the query after typing pauses.
    onchange: Option<EventHandler<String>>,
    /// Called with the query when the user presses Enter.
    on_submit: Option<EventHandler<String>>,
    /// Called when the user presses Cancel. The query is cleared first.
    on_cancel: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the search form.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let mut value = use_controlled(value, String::new);
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
            if show_cancel.unwrap_or(false) {
                button {
                    r#type: "button",
                    class: "g3-searchbar-cancel",
                    onclick: move |_| {
                        value.set(String::new());
                        if let Some(onchange) = onchange {
                            onchange.call(String::new());
                        }
                        if let Some(on_cancel) = on_cancel {
                            on_cancel.call(());
                        }
                    },
                    "{strings.cancel}"
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SearchbarPlaygroundDemo() -> Element {
    let query = use_signal(String::new);
    let mut committed = use_signal(String::new);
    let show_cancel = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: show_cancel, label: "Cancel button" }
            },
            div { class: "playground-stack",
                Searchbar {
                    value: query,
                    show_cancel: show_cancel(),
                    onchange: move |q| committed.set(q),
                }
                p { "Filtering by: \"{committed}\"" }
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
