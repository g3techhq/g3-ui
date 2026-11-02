//! Spinning columns shown in place.
use super::SelectOption;
use super::wheel::WheelColumn;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Spinning columns shown in place, for a short run of choices that reads
/// better as a wheel than a list, such as a score. Like Ionic's `ion-picker`.
/// Each column is a [`PickerColumn`].
///
/// [`DatePicker`](crate::DatePicker) and [`TimePicker`](crate::TimePicker)
/// open the same wheels from a field; use this where the wheel is the main
/// control, in a sheet or a card.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let strokes = use_signal(|| 4_u8);
/// rsx! {
///     Picker { aria_label: "Score",
///         PickerColumn {
///             value: strokes,
///             aria_label: "Strokes",
///             options: (1..=10_u8).map(|n| SelectOption::new(n, n.to_string())).collect::<Vec<_>>(),
///         }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Picker(
    /// Accessible name for the group of columns.
    aria_label: Option<String>,
    /// Extra classes for the picker.
    class: Option<String>,
    /// The [`PickerColumn`]s, left to right.
    children: Element,
) -> Element {
    rsx! {
        div {
            class: merge_classes("g3-wheel", class.as_deref()),
            role: "group",
            aria_label,
            {children}
        }
    }
}

/// One spinning column of a [`Picker`]. Like Ionic's `ion-picker-column`.
///
/// Scrolling, flicking, dragging with a mouse, tapping an option or the arrow
/// keys choose. Screen readers meet a spin button.
#[component]
pub fn PickerColumn<T: Clone + PartialEq + 'static>(
    /// The chosen value. One that is not among the options shows the first.
    value: Signal<T>,
    /// The choices, top to bottom. Disabled ones are skipped.
    options: Vec<SelectOption<T>>,
    /// Accessible name, such as "Strokes".
    aria_label: String,
    /// Called with the value the column settles on.
    onchange: Option<EventHandler<T>>,
    /// Take a larger share of the width, for longer labels. Defaults to
    /// `false`.
    wide: Option<bool>,
    /// Extra classes for the column.
    class: Option<String>,
) -> Element {
    let index = {
        let current = value.read();
        options
            .iter()
            .position(|option| option.value == *current)
            .unwrap_or(0)
    };
    let labels = options
        .iter()
        .map(|option| option.label.clone())
        .collect::<Vec<_>>();
    let disabled_indices = options
        .iter()
        .enumerate()
        .filter_map(|(index, option)| option.disabled.then_some(index))
        .collect::<Vec<_>>();
    let values = options
        .into_iter()
        .map(|option| option.value)
        .collect::<Vec<_>>();
    let mut value = value;

    rsx! {
        WheelColumn {
            labels,
            index,
            aria_label,
            disabled_indices,
            wide: wide.unwrap_or(false),
            class,
            onchange: move |index: usize| {
                if let Some(next) = values.get(index).cloned() {
                    value.set(next.clone());
                    if let Some(onchange) = onchange {
                        onchange.call(next);
                    }
                }
            },
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn PickerPlaygroundDemo() -> Element {
    let strokes = use_signal(|| 4_u8);
    let par = 4_u8;

    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack {
                crate::Text { "Hole 7 · Par {par}" }
                Picker { aria_label: "Score",
                    PickerColumn {
                        value: strokes,
                        aria_label: "Strokes",
                        options: (1..=10_u8)
                            .map(|n| SelectOption::new(n, n.to_string()))
                            .collect::<Vec<_>>(),
                    }
                }
                crate::Text { tone: crate::TextTone::Secondary, "Chosen: {strokes}" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Picker",
    description: "Spinning wheel columns shown in place.",
    components: ["Picker", "PickerColumn"],
    demo: PickerPlaygroundDemo,
    source: "src/components/picker.rs",
}
