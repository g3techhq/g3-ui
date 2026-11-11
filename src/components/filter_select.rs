//! A filter over a list or table, drawn as a chip.
use super::select::{Select, SelectOption, SelectWidth};
use crate::theme::{merge_classes, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::X;

/// A filter over a list or table, as a pill: "Status" while it lets
/// everything through, "Status: Not cleared" once it narrows, with a button
/// that resets it.
///
/// It is a [`Select`] in a chip's clothes, for a row of filters above a
/// table. The value that lets everything through is `default_value`, or the
/// first option's. While another value is chosen the pill is tinted and shows
/// a clear button. Put several side by side in a wrapping row; the page reads
/// each value and filters its data.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let status = use_signal(|| "all");
/// rsx! {
///     FilterSelect {
///         label: "Status",
///         value: status,
///         options: vec![
///             SelectOption::new("all", "All"),
///             SelectOption::new("open", "Not cleared"),
///         ],
///     }
/// }
/// # }
/// ```
#[component]
pub fn FilterSelect<T: Clone + PartialEq + 'static>(
    /// What it filters on, such as "Status". Shown on the pill and names it.
    label: String,
    /// The chosen value.
    value: Signal<T>,
    /// The choices.
    options: Vec<SelectOption<T>>,
    /// The value that lets everything through, which the clear button
    /// restores. Defaults to the first option's.
    default_value: Option<T>,
    /// Called with the value the user picks or clears to.
    onchange: Option<EventHandler<T>>,
    /// Disable the filter.
    disabled: Option<bool>,
    /// Extra classes for the pill's wrapper.
    class: Option<String>,
) -> Element {
    let strings = use_strings()();
    let mut value = value;
    let Some(all) = default_value.or_else(|| options.first().map(|option| option.value.clone()))
    else {
        return rsx! {};
    };
    let current = value();
    let active = current != all;
    let disabled = disabled.unwrap_or(false);
    // The pill names the filter, then what it is narrowed to.
    let options: Vec<SelectOption<T>> = options
        .into_iter()
        .map(|option| {
            let shown = if option.value == all {
                label.clone()
            } else {
                let short = option.trigger_label.as_deref().unwrap_or(&option.label);
                format!("{label}: {short}")
            };
            option.trigger_label(shown)
        })
        .collect();
    let name = options
        .iter()
        .find(|option| option.value == current)
        .and_then(|option| option.trigger_label.clone())
        .unwrap_or_else(|| label.clone());
    let clear_label = format!("{} {label}", strings.clear);

    rsx! {
        div {
            class: merge_classes("g3-filter", class.as_deref()),
            "data-active": active.then_some("true"),
            Select {
                value,
                options,
                aria_label: name,
                width: SelectWidth::Fit,
                disabled,
                onchange: move |picked: T| {
                    if let Some(onchange) = onchange {
                        onchange.call(picked);
                    }
                },
            }
            if active && !disabled {
                button {
                    r#type: "button",
                    class: "g3-filter-clear",
                    aria_label: clear_label,
                    onclick: move |_| {
                        value.set(all.clone());
                        if let Some(onchange) = onchange {
                            onchange.call(all.clone());
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
fn FilterSelectPlaygroundDemo() -> Element {
    let status = use_signal(|| 0_u8);
    let kind = use_signal(|| 0_u8);
    let dates = use_signal(|| 0_u8);

    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack { horizontal: true, wrap: true, align: crate::StackAlign::Center,
                FilterSelect {
                    label: "Status",
                    value: status,
                    options: vec![
                        SelectOption::new(0, "All"),
                        SelectOption::new(1, "Not cleared"),
                        SelectOption::new(2, "Not reconciled"),
                    ],
                }
                FilterSelect {
                    label: "Type",
                    value: kind,
                    options: vec![
                        SelectOption::new(0, "All"),
                        SelectOption::new(1, "Payments"),
                        SelectOption::new(2, "Deposits"),
                    ],
                }
                FilterSelect {
                    label: "Dates",
                    value: dates,
                    options: vec![
                        SelectOption::new(0, "All dates"),
                        SelectOption::new(1, "This month"),
                        SelectOption::new(2, "Last month"),
                        SelectOption::new(3, "This year"),
                    ],
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "FilterSelect",
    description: "A filter over a list or table, drawn as a pill with a clear button.",
    demo: FilterSelectPlaygroundDemo,
    source: "src/components/filter_select.rs",
}
