//! Progress bars.
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// A horizontal progress bar. Like Ionic's `ion-progress-bar`.
///
/// Give it a `value` for known progress; leave it out for an indeterminate,
/// animated bar.
///
/// ```rust,ignore
/// rsx! { Progress { value: holes_played as f64, max: 18.0, label: "Holes played" } }
/// ```
#[component]
pub fn Progress(
    /// Current progress, from `0` to `max`.
    value: Option<f64>,
    /// The value at completion. Defaults to `100`.
    max: Option<f64>,
    /// What is progressing, for screen readers.
    label: Option<String>,
    /// Text read out instead of the number, such as "7 of 18 holes".
    value_text: Option<String>,
    /// Extra classes for the bar.
    class: Option<String>,
) -> Element {
    let max = max.unwrap_or(100.0).max(f64::EPSILON);
    let value = value.map(|value| value.clamp(0.0, max));
    let percent = value.map_or(0.0, |value| value / max * 100.0);
    let cls = classes([
        "g3-progress",
        if value.is_none() {
            "g3-progress-indeterminate"
        } else {
            ""
        },
    ]);
    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            role: "progressbar",
            aria_label: label,
            aria_valuemin: value.map(|_| "0"),
            aria_valuemax: value.map(|_| max.to_string()),
            aria_valuenow: value.map(|value| value.to_string()),
            aria_valuetext: value_text,
            style: "--g3-progress-value: {percent}%;",
            div { class: "g3-progress-track",
                div { class: "g3-progress-fill" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ProgressPlaygroundDemo() -> Element {
    let value = use_signal(|| 45.0);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Range { label: "Value", value, show_value: true }
            },
            div { class: "playground-stack",
                Progress { value: value(), label: "Upload" }
                Progress { label: "Loading" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Progress",
    description: "Determinate and indeterminate progress bars.",
    demo: ProgressPlaygroundDemo,
    source: "src/components/progress.rs",
}
