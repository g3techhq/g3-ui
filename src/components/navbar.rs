//! Navbar component - persistent base layout for bottom-tab style app navigation.

use super::navbar_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use dx_route_transitions::ROUTE_TRANSITION_BASE_CLASS;

#[component]
pub fn Navbar(children: Element, class: Option<String>, mode: Option<ComponentMode>) -> Element {
    let mode = use_component_mode(mode);

    let navbar_cls = match mode {
        ComponentMode::Ios => format!("{} {}", s::NAVBAR_BASE, s::NAVBAR_IOS),
        ComponentMode::Md => format!("{} {}", s::NAVBAR_BASE, s::NAVBAR_MD),
    };
    #[cfg(feature = "transitions")]
    let navbar_cls = merge_classes(navbar_cls, Some(ROUTE_TRANSITION_BASE_CLASS));
    let navbar_cls = merge_classes(navbar_cls, class.as_deref());

    rsx! {
        div {
            class: navbar_cls,
            "data-g3-mode": mode.as_str(),
            {children}
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn NavbarPlaygroundDemo() -> Element {
    let playground_mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame { app: false,
            crate::AppWrapper { mode: playground_mode, class: "g3-playground-device-app",
                Navbar {
                    crate::Header { title: "Navbar" }
                    crate::Body { has_footer_space: false,
                        crate::Card { title: "Content", "Route content sits above a persistent navigation bar." }
                    }
                    nav { class: "flex justify-around border-t-2 border-t-focused bg-white flex-0 z-40",
                        span { class: "flex flex-col items-center py-2 text-xs text-focused", "Games" }
                        span { class: "flex flex-col items-center py-2 text-xs text-light", "Tourneys" }
                        span { class: "flex flex-col items-center py-2 text-xs text-light", "Account" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Navbar",
    g3_name: "G3Navbar",
    description: "Persistent navigation layout that owns the base transition layer.",
    demo: NavbarPlaygroundDemo,
    source: "src/components/navbar.rs",
}
