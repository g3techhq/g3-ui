//! SheetButton component - info button that opens a sheet with description.

use super::InfoButton;
use super::Sheet;
use crate::theme::ComponentMode;
use dioxus::prelude::*;

#[component]
pub fn SheetButton(
    description: String,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        InfoButton {
            onclick: move |event: Event<MouseData>| {
                event.stop_propagation();
                is_open.set(true);
            },
        }
        Sheet {
            is_open,
            class,
            mode,
            "{description}"
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn SheetButtonPlaygroundDemo() -> Element {
    let mut description =
        use_signal(|| "Handicaps adjust player scoring for the match.".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-control", span { "Description" } textarea { value: "{description()}", oninput: move |event: Event<FormData>| description.set(event.value()) } }
            },
            SheetButton { description: description() }
        }
    }
}
crate::g3_playground! {
    name: "SheetButton",
    g3_name: "G3SheetButton",
    description: "Information button that opens a sheet.",
    demo: SheetButtonPlaygroundDemo,
}
