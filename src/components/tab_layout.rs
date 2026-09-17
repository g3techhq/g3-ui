//! A page layout with persistent navigation.
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;
#[cfg(feature = "transitions")]
use g3_route_transitions::ROUTE_TRANSITION_BASE_REGION_CLASS;

/// A page with persistent navigation: a [`Header`](crate::Header), its
/// [`Content`](crate::Content), and a navigation component side by side. Like
/// Ionic's `ion-tabs`.
///
/// With an [`AdaptiveNav`](crate::AdaptiveNav) the navigation is a bottom bar
/// on compact shells and a side rail from `48rem`. A
/// [`NavBar`](crate::NavBar) is always a bottom bar and a
/// [`NavRail`](crate::NavRail) always a rail. Set `--g3-nav-rail-width` to
/// widen the rail.
///
/// ```rust,ignore
/// rsx! {
///     TabLayout {
///         Header { title: "Rounds" }
///         Content { RoundList {} }
///         AdaptiveNav {
///             NavItem { label: "Rounds", to: Route::Rounds {}, icon: rsx! { Flag {} } }
///             NavItem { label: "Profile", to: Route::Profile {}, group: NavItemGroup::Secondary,
///                 icon: rsx! { User {} } }
///         }
///     }
/// }
/// ```
#[component]
pub fn TabLayout(
    /// Whether this layout is the route-transition base region: the page shell
    /// that stays put during navigation and dims under a routed sheet.
    /// Defaults to `true`. Set `false` when a sheet route renders the layout.
    /// Needs the `transitions` feature.
    route_transition_base: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the layout element.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    #[cfg(feature = "transitions")]
    let base_cls = if route_transition_base.unwrap_or(true) {
        ROUTE_TRANSITION_BASE_REGION_CLASS
    } else {
        ""
    };
    #[cfg(not(feature = "transitions"))]
    let base_cls = {
        let _ = route_transition_base;
        ""
    };
    rsx! {
        div {
            class: merge_classes(classes(["g3-tab-layout", base_cls]), class.as_deref()),
            "data-g3-mode": mode.as_str(),
            {children}
        }
    }
}
