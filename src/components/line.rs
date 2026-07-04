//! Line (divider/separator) component.

use super::line_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Orientation of the line separator.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum LineOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn Line(orientation: Option<LineOrientation>, class: Option<String>) -> Element {
    let orient = orientation.unwrap_or_default();
    let base_cls = match orient {
        LineOrientation::Horizontal => s::LINE_H,
        LineOrientation::Vertical => s::LINE_V,
    };

    let cls = merge_classes(format!("{} {base_cls}", s::LINE), class.as_deref());
    let aria_orientation = match orient {
        LineOrientation::Horizontal => "horizontal",
        LineOrientation::Vertical => "vertical",
    };

    rsx! {
        hr {
            class: cls,
            role: "separator",
            aria_orientation,
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn LinePlaygroundDemo() -> Element {
    let mut vertical = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: vertical(), onchange: move |_| vertical.toggle() } span { "Vertical" } }
            },
            div { class: if vertical() { "h-24" } else { "w-full" },
                Line { orientation: if vertical() { LineOrientation::Vertical } else { LineOrientation::Horizontal } }
            }
        }
    }
}
crate::g3_playground! {
    name: "Line",
    g3_name: "G3Line",
    description: "Horizontal or vertical separator.",
    demo: LinePlaygroundDemo,
}
