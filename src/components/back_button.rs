//! A header button that goes back.
use super::pressable::Destination;
use crate::components::pressable::{Pressable, Target};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus::router::RouterContext;
use dioxus_icons::lucide::{ArrowLeft, ChevronLeft};

/// A back button for a [`Header`](crate::Header)'s `start` slot. Like
/// Ionic's `ion-back-button`.
///
/// With no `onclick`, it goes back in the router's history, or navigates to
/// `default_to` when there is nothing to go back to. It shows a chevron and
/// label on iOS and an arrow on Material Design.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Routable, Clone, Debug, PartialEq)]
/// # enum Route {
/// #     #[route("/")]
/// #     Players {},
/// # }
/// # #[component] fn Players() -> Element { rsx! {} }
/// # rsx! {
/// Header { title: "Player", start: rsx! { BackButton { default_to: Route::Players {} } } }
/// # }
/// # }
/// ```
#[component]
pub fn BackButton(
    /// Visible label on iOS and the accessible name everywhere. Defaults to
    /// [`Strings::back`](crate::Strings::back).
    label: Option<String>,
    /// Where to go when the history has nothing to go back to.
    #[props(default, into)]
    default_to: Destination,
    /// Replaces the default back behaviour.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the button.
    class: Option<String>,
) -> Element {
    let mode = use_component_mode(mode);
    let label = label.unwrap_or_else(|| use_strings().back);
    let router = try_consume_context::<RouterContext>();
    let handle = move |event: MouseEvent| {
        if let Some(onclick) = onclick {
            onclick.call(event);
            return;
        }
        let Some(router) = router else {
            return;
        };
        if router.can_go_back() {
            router.go_back();
        } else if let Some(target) = default_to.clone().target() {
            router.replace(target);
        }
    };
    let cls = classes([
        "g3-back-button",
        mode.pick("g3-back-button-ios", "g3-back-button-md"),
    ]);
    rsx! {
        Pressable {
            class: merge_classes(cls, class.as_deref()),
            target: Target::Action,
            onclick: handle,
            aria_label: label.clone(),
            span { class: "g3-back-button-icon", aria_hidden: "true",
                match mode {
                    ComponentMode::Ios => rsx! {
                        ChevronLeft { size: 26 }
                    },
                    ComponentMode::Md => rsx! {
                        ArrowLeft { size: 22 }
                    },
                }
            }
            if mode == ComponentMode::Ios {
                span { class: "g3-back-button-label", aria_hidden: "true", "{label}" }
            }
        }
    }
}
