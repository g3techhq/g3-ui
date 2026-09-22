//! A loading indicator.
use crate::theme::{classes, merge_classes, use_strings};
use dioxus::prelude::*;

/// Spinner size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SpinnerSize {
    /// 20px, for inline use in buttons and rows.
    Sm,
    /// 32px.
    Md,
    /// 60px, for whole-page loading.
    #[default]
    Lg,
}

/// A spinning loading indicator, announced to screen readers as a status.
#[component]
pub fn Spinner(
    /// Size. Defaults to [`SpinnerSize::Lg`].
    size: Option<SpinnerSize>,
    /// Fill the parent and centre the spinner in it.
    center: Option<bool>,
    /// What is loading, for screen readers. Defaults to
    /// [`Strings::loading`](crate::Strings::loading).
    label: Option<String>,
    /// Hide it from assistive technology, when something nearby already says
    /// the work is in progress, such as a button's `aria-busy`.
    decorative: Option<bool>,
    /// Extra classes for the wrapper.
    class: Option<String>,
) -> Element {
    let label = label.unwrap_or_else(|| use_strings().loading);
    let size_cls = match size.unwrap_or_default() {
        SpinnerSize::Sm => "g3-spinner-sm",
        SpinnerSize::Md => "g3-spinner-md",
        SpinnerSize::Lg => "",
    };
    let wrapper_cls = classes([
        "g3-spinner-wrapper",
        if center.unwrap_or(false) {
            "g3-spinner-centered"
        } else {
            ""
        },
    ]);
    rsx! {
        if decorative.unwrap_or(false) {
            div { class: merge_classes(wrapper_cls, class.as_deref()), aria_hidden: "true",
                div { class: classes(["g3-spinner", size_cls]) }
            }
        } else {
            div { class: merge_classes(wrapper_cls, class.as_deref()), role: "status",
                div { class: classes(["g3-spinner", size_cls]), aria_hidden: "true" }
                span { class: "g3-sr-only", "{label}" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SpinnerPlaygroundDemo() -> Element {
    let center = use_signal(|| true);
    let size = use_signal(|| SpinnerSize::Lg);
    let box_cls = if center() {
        "g3-playground-spinner-box g3-playground-spinner-box-centered"
    } else {
        "g3-playground-spinner-box"
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Checkbox { checked: center, label: "Center" }
                crate::SegmentGroup { value: size,
                    crate::SegmentButton { value: SpinnerSize::Sm, "Sm" }
                    crate::SegmentButton { value: SpinnerSize::Md, "Md" }
                    crate::SegmentButton { value: SpinnerSize::Lg, "Lg" }
                }
            },
            div { class: box_cls,
                Spinner { center: center(), size: size() }
            }
        }
    }
}

crate::g3_playground! {
    name: "Spinner",
    description: "Accessible loading indicator.",
    demo: SpinnerPlaygroundDemo,
    source: "src/components/spinner.rs",
}
