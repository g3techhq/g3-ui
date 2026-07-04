//! Body component - scrollable page content with error/loading boundaries.

use super::body_styles as s;
use crate::components::Spinner;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use dx_route_transitions::ROUTE_TRANSITION_SEGMENT_CLASS;

#[component]
pub fn Body(
    children: Element,
    has_footer_space: Option<bool>,
    fab: Option<Element>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);

    let body_cls = match mode {
        ComponentMode::Ios => format!("{} {} bg-white", s::BODY_BASE, s::BODY_IOS),
        ComponentMode::Md => format!("{} {} bg-gray-100", s::BODY_BASE, s::BODY_MD),
    };

    #[cfg(feature = "transitions")]
    let content_cls = merge_classes(s::BODY_CONTENT, Some(ROUTE_TRANSITION_SEGMENT_CLASS));
    #[cfg(not(feature = "transitions"))]
    let content_cls = s::BODY_CONTENT.to_string();

    rsx! {
        div {
            class: merge_classes(body_cls, class.as_deref()),
            div { class: content_cls,
                ErrorBoundary {
                    handle_error: |_| rsx! {
                        div {
                            class: "flex items-center justify-center py-8 text-red-500",
                            role: "alert",
                            "Failed to load resource. Please try again."
                        }
                    },
                    SuspenseBoundary {
                        fallback: |_| rsx! {
                            Spinner { center: true }
                        },
                        {children}
                        if has_footer_space.unwrap_or(true) {
                            div { class: s::FOOTER_SPACER }
                        }
                    }
                }
            }
            if let Some(fab) = fab {
                {fab}
            }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn BodyPlaygroundDemo() -> Element {
    let mut footer_space = use_signal(|| false);
    let mut show_fab = use_signal(|| true);
    let playground_mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: footer_space(), onchange: move |_| footer_space.toggle() } span { "Footer space" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: show_fab(), onchange: move |_| show_fab.toggle() } span { "FAB" } }
            },
            crate::AppWrapper { mode: playground_mode, class: "g3-playground-device-app",
                crate::Header { title: "Body" }
                Body {
                    has_footer_space: footer_space(),
                    fab: show_fab().then_some(rsx! {
                        crate::Fab { vertical: crate::FabVertical::Bottom, horizontal: crate::FabHorizontal::End,
                            crate::FabButton { onclick: |_| {}, "+" }
                        }
                    }),
                    for index in 1..=12 {
                        crate::Card { title: format!("Hole {index}"), "Scrollable content row with enough body content to show overflow." }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Body",
    g3_name: "G3Body",
    description: "Scrollable page body with loading and error boundaries.",
    demo: BodyPlaygroundDemo,
}
