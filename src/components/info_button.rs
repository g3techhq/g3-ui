//! A small "more information" button.
use super::BottomSheet;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::Info;

/// A round "i" button. Give it `sheet` content and it opens that content in a
/// [`BottomSheet`]; otherwise handle `onclick` yourself.
///
/// Presses do not reach an enclosing clickable card or list row.
///
/// ```rust,ignore
/// rsx! {
///     InfoButton { sheet_title: "Handicaps",
///         sheet: rsx! { p { "Handicaps adjust each player's score." } },
///     }
/// }
/// ```
#[component]
pub fn InfoButton(
    /// Content of the sheet the button opens.
    sheet: Option<Element>,
    /// Title of that sheet.
    sheet_title: Option<String>,
    /// Called when pressed, in addition to opening any sheet.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Accessible name. Defaults to
    /// [`Strings::more_information`](crate::Strings::more_information).
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the button.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let mut open = use_signal(|| false);
    let aria_label = aria_label.unwrap_or_else(|| use_strings().more_information);
    let has_sheet = sheet.is_some();
    let cls = classes([
        "g3-info-btn",
        mode.pick("g3-info-btn-ios", "g3-info-btn-md"),
    ]);
    rsx! {
        button {
            class: merge_classes(cls, class.as_deref()),
            r#type: "button",
            aria_label,
            aria_haspopup: has_sheet.then_some("dialog"),
            aria_expanded: has_sheet.then(|| open().to_string()),
            onpointerdown: |event| event.stop_propagation(),
            onclick: move |event| {
                event.stop_propagation();
                if has_sheet {
                    open.set(true);
                }
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            Info { class: "g3-info-btn-icon", size: 24 }
        }
        if let Some(sheet) = sheet {
            BottomSheet { open, title: sheet_title, mode, {sheet} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn InfoButtonPlaygroundDemo() -> Element {
    let description = use_signal(|| "Handicaps adjust player scoring for the match.".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::TextArea { label: "Sheet text", value: description }
            },
            InfoButton {
                sheet_title: "Handicaps",
                sheet: rsx! {
                    p { "{description}" }
                },
            }
        }
    }
}

crate::g3_playground! {
    name: "InfoButton",
    description: "Information button that can open a sheet.",
    demo: InfoButtonPlaygroundDemo,
    source: "src/components/info_button.rs",
}
