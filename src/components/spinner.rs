//! A loading indicator.
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};

/// How many lines the iOS spinner draws.
const LINES: i32 = 8;
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
    /// Color role. Defaults to the accent ring in Material and the secondary
    /// text color for the iOS lines, as in Ionic.
    color: Option<crate::Color>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the wrapper.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let label = label.unwrap_or_else(|| strings.read().loading.clone());
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
    // Ionic's platform defaults: fading lines on iOS, a turning ring in
    // Material.
    let indicator = match mode {
        ComponentMode::Ios => rsx! {
            svg {
                class: classes(["g3-spinner", "g3-spinner-lines", size_cls]),
                view_box: "0 0 64 64",
                "aria-hidden": "true",
                for index in 0..LINES {
                    line {
                        key: "{index}",
                        x1: "32",
                        y1: "6",
                        x2: "32",
                        y2: "18",
                        style: "transform: rotate({index * 360 / LINES}deg); animation-delay: {index * 1000 / LINES - 1000}ms;",
                    }
                }
            }
        },
        ComponentMode::Md => rsx! {
            div { class: classes(["g3-spinner", "g3-spinner-ring", size_cls]), aria_hidden: "true" }
        },
    };
    rsx! {
        if decorative.unwrap_or(false) {
            div {
                class: merge_classes(&wrapper_cls, class.as_deref()),
                "data-color": color.map(crate::Color::as_str),
                aria_hidden: "true",
                {indicator}
            }
        } else {
            div {
                class: merge_classes(&wrapper_cls, class.as_deref()),
                "data-color": color.map(crate::Color::as_str),
                role: "status",
                {indicator}
                span { class: "g3-sr-only", "{label}" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SpinnerPlaygroundDemo() -> Element {
    let color = use_signal(|| crate::Color::Accent);
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
                crate::components::color::ColorSelect { value: color }
                crate::Checkbox { checked: center, label: "Center" }
                crate::SegmentGroup { value: size, label: "Size",
                    crate::SegmentButton { value: SpinnerSize::Sm, "Sm" }
                    crate::SegmentButton { value: SpinnerSize::Md, "Md" }
                    crate::SegmentButton { value: SpinnerSize::Lg, "Lg" }
                }
            },
            div { class: box_cls,
                Spinner { center: center(), size: size(), color: color() }
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
