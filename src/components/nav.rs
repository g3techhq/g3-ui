//! Persistent app navigation: bottom bars, side rails, and their items.
use super::pressable::Destination;
use crate::components::pressable::{Pressable, Target};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::{ChevronLeft, ChevronRight};

#[cfg(feature = "transitions")]
use crate::components::use_shell_size;
#[cfg(feature = "transitions")]
use g3_route_transitions::ROUTE_TRANSITION_PERSISTENT_CLASS;

/// What an [`AdaptiveNav`] does on a compact shell (narrower than `48rem`).
/// On wide shells it is always a rail.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum AdaptiveNavCompact {
    /// Show a bottom bar.
    #[default]
    Bar,
    /// Show nothing. For full-screen routes, such as routed sheets, that cover
    /// the bottom bar on phones but keep the rail beside them on wide shells.
    Hidden,
}

/// Where a [`NavItem`] sits in a rail. Bottom bars keep declaration order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum NavItemGroup {
    /// Main destinations, at the top of the rail.
    #[default]
    Primary,
    /// Account and settings destinations, grouped at the bottom of the rail.
    Secondary,
}

#[derive(Clone, Copy, PartialEq)]
enum NavKind {
    Bar,
    Rail,
    Adaptive(AdaptiveNavCompact),
}

#[component]
fn NavContainer(
    kind: NavKind,
    aria_label: Option<String>,
    mode: Option<ComponentMode>,
    class: Option<String>,
    expanded: Option<Signal<bool>>,
    default_expanded: Option<bool>,
    collapsible: Option<bool>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    // Called unconditionally: a hook may not sit behind a match arm. A bar has
    // no expanded state, so its signal goes unused.
    let own = use_signal(|| default_expanded.unwrap_or(true));
    let mut expanded = expanded.unwrap_or(own);
    let has_rail = !matches!(kind, NavKind::Bar);
    let is_expanded = has_rail && expanded();
    let collapsible = has_rail && collapsible.unwrap_or(true);
    let strings = use_strings();
    let aria_label = aria_label.unwrap_or_else(|| use_strings().primary_navigation);
    let kind_cls = match kind {
        NavKind::Bar => "",
        NavKind::Rail => "g3-nav-rail",
        NavKind::Adaptive(AdaptiveNavCompact::Bar) => "g3-nav-adaptive",
        NavKind::Adaptive(AdaptiveNavCompact::Hidden) => "g3-nav-adaptive g3-nav-compact-hidden",
    };
    // A rail stays reachable while a routed sheet is up, so it is captured as
    // persistent chrome and paints above the sheet. A bottom bar is not: on a
    // phone a sheet is meant to cover it, so it stays inside the base region
    // and rises under the sheet as before. An adaptive nav is a rail only on a
    // wide shell, so it follows the measured shell size.
    #[cfg(feature = "transitions")]
    let persistent_cls = {
        // Called unconditionally: a hook may not sit behind a match arm.
        let wide = use_shell_size().is_wide();
        let is_rail = match kind {
            NavKind::Bar => false,
            NavKind::Rail => true,
            NavKind::Adaptive(_) => wide,
        };
        if is_rail {
            ROUTE_TRANSITION_PERSISTENT_CLASS
        } else {
            ""
        }
    };
    #[cfg(not(feature = "transitions"))]
    let persistent_cls = "";
    let cls = classes([
        "g3-nav",
        mode.pick("g3-nav-ios", "g3-nav-md"),
        kind_cls,
        if is_expanded { "g3-nav-expanded" } else { "" },
        persistent_cls,
    ]);
    let (toggle_label, toggle_icon) = if is_expanded {
        (
            strings.collapse_navigation,
            rsx! { ChevronLeft { size: 16 } },
        )
    } else {
        (
            strings.expand_navigation,
            rsx! { ChevronRight { size: 16 } },
        )
    };
    rsx! {
        nav { class: merge_classes(cls, class.as_deref()), aria_label,
            {children}
            // Hidden by CSS wherever the nav is a bottom bar, so the server
            // and the client render the same markup at every width.
            if collapsible {
                button {
                    r#type: "button",
                    class: "g3-nav-toggle",
                    aria_label: toggle_label,
                    aria_expanded: is_expanded.to_string(),
                    onclick: move |_| expanded.set(!expanded()),
                    {toggle_icon}
                }
            }
        }
    }
}

/// Navigation that is a bottom bar on compact shells and a side rail on wide
/// ones. Put it last inside a [`TabLayout`](crate::TabLayout).
///
/// The rail starts expanded, with a label beside each icon, and a button at its
/// foot collapses it to icons alone.
///
/// With the `transitions` feature, the rail is persistent route-transition
/// chrome inside an [`AppWrapper`](crate::AppWrapper): it stays still while
/// pages slide, as long as the routes on both sides render it.
#[component]
pub fn AdaptiveNav(
    /// What to show on compact shells. Defaults to a bottom bar.
    compact: Option<AdaptiveNavCompact>,
    /// Accessible name. Defaults to
    /// [`Strings::primary_navigation`](crate::Strings::primary_navigation).
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the `nav` element.
    class: Option<String>,
    /// Whether the rail shows labels beside its icons. Pass a signal to own
    /// the state, for example to remember it between visits; the toggle
    /// writes it. Without one the nav keeps its own, starting from
    /// `default_expanded`.
    expanded: Option<Signal<bool>>,
    /// Whether the rail starts expanded when it owns its state. Defaults to
    /// `true`.
    default_expanded: Option<bool>,
    /// Show the button that expands and collapses the rail. Defaults to
    /// `true`.
    collapsible: Option<bool>,
    children: Element,
) -> Element {
    rsx! {
        NavContainer {
            kind: NavKind::Adaptive(compact.unwrap_or_default()),
            aria_label,
            mode,
            class,
            expanded,
            default_expanded,
            collapsible,
            {children}
        }
    }
}

/// A bottom tab bar at every shell width. Put it last inside a
/// [`TabLayout`](crate::TabLayout).
#[component]
pub fn NavBar(
    /// Accessible name. Defaults to
    /// [`Strings::primary_navigation`](crate::Strings::primary_navigation).
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the `nav` element.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        NavContainer { kind: NavKind::Bar, aria_label, mode, class, {children} }
    }
}

/// A side navigation rail at every shell width. Put it inside a
/// [`TabLayout`](crate::TabLayout).
#[component]
pub fn NavRail(
    /// Accessible name. Defaults to
    /// [`Strings::primary_navigation`](crate::Strings::primary_navigation).
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the `nav` element.
    class: Option<String>,
    /// Whether the rail shows labels beside its icons. Pass a signal to own
    /// the state, for example to remember it between visits; the toggle
    /// writes it. Without one the nav keeps its own, starting from
    /// `default_expanded`.
    expanded: Option<Signal<bool>>,
    /// Whether the rail starts expanded when it owns its state. Defaults to
    /// `true`.
    default_expanded: Option<bool>,
    /// Show the button that expands and collapses the rail. Defaults to
    /// `true`.
    collapsible: Option<bool>,
    children: Element,
) -> Element {
    rsx! {
        NavContainer {
            kind: NavKind::Rail,
            aria_label,
            mode,
            class,
            expanded,
            default_expanded,
            collapsible,
            {children}
        }
    }
}

/// One destination in a [`NavBar`], [`NavRail`], or [`AdaptiveNav`].
///
/// Give it `to` for a router destination; it is then marked current
/// automatically when its route is active. Use `href` for a plain link, or
/// `onclick` with `selected` to manage the current destination yourself.
#[component]
pub fn NavItem(
    /// Visible label. In a collapsed rail it appears as a tooltip.
    label: String,
    /// Icon shown above the label.
    icon: Option<Element>,
    /// Router destination.
    #[props(default, into)]
    to: Destination,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Mark this as the current destination. Router links mark themselves.
    selected: Option<bool>,
    /// Short badge text on the icon, such as an unread count. Hidden when
    /// empty.
    badge: Option<String>,
    /// Where the item sits in a rail. Defaults to
    /// [`NavItemGroup::Primary`].
    group: Option<NavItemGroup>,
    /// Disable the item.
    disabled: Option<bool>,
    /// Called on activation, before any navigation.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Extra classes for the item.
    class: Option<String>,
) -> Element {
    let target = Target::from_props(href, to, false);
    let group_cls = match group.unwrap_or_default() {
        NavItemGroup::Primary => "",
        NavItemGroup::Secondary => "g3-nav-item-secondary",
    };
    let badge = badge.filter(|badge| !badge.is_empty());
    // The rail hides the visible label, which also hides it from assistive
    // technology, so the name is given directly.
    let name = match &badge {
        Some(badge) => format!("{label}, {badge}"),
        None => label.clone(),
    };
    rsx! {
        Pressable {
            class: merge_classes(classes(["g3-nav-item", group_cls]), class.as_deref()),
            target,
            disabled: disabled.unwrap_or(false),
            onclick,
            aria_current: selected.unwrap_or(false).then_some("page"),
            aria_label: name,
            span { class: "g3-nav-item-icon", aria_hidden: "true",
                {icon}
                if let Some(badge) = &badge {
                    span { class: "g3-nav-item-badge", "{badge}" }
                }
            }
            span { class: "g3-nav-item-label", aria_hidden: "true", "{label}" }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn NavPlaygroundDemo() -> Element {
    use dioxus_icons::lucide::{CalendarDays, CircleUserRound, Trophy};
    let mode = crate::use_component_mode(None);
    let mut active = use_signal(|| 0_usize);
    let layout = use_signal(|| 0_usize);
    let items = rsx! {
        NavItem {
            label: "Games",
            selected: active() == 0,
            badge: "3",
            icon: rsx! {
                Trophy { size: 20 }
            },
            onclick: move |_| active.set(0),
        }
        NavItem {
            label: "Tourneys",
            selected: active() == 1,
            icon: rsx! {
                CalendarDays { size: 20 }
            },
            onclick: move |_| active.set(1),
        }
        NavItem {
            label: "Account",
            selected: active() == 2,
            group: NavItemGroup::Secondary,
            icon: rsx! {
                CircleUserRound { size: 20 }
            },
            onclick: move |_| active.set(2),
        }
    };
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                div {
                    span { "Layout" }
                    crate::SegmentGroup { value: layout,
                        crate::SegmentButton { value: 0_usize, "Adaptive" }
                        crate::SegmentButton { value: 1_usize, "Bar" }
                        crate::SegmentButton { value: 2_usize, "Rail" }
                    }
                }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                crate::TabLayout {
                    crate::Header { title: "Navigation" }
                    crate::Content { footer_space: false,
                        crate::Card { title: "Content", "Page content beside persistent navigation." }
                    }
                    match layout() {
                        1 => rsx! {
                            NavBar { {items} }
                        },
                        2 => rsx! {
                            NavRail { {items} }
                        },
                        _ => rsx! {
                            AdaptiveNav { {items} }
                        },
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Navigation",
    description: "Bottom bars and side rails, adaptive to the shell width.",
    components: ["TabLayout", "AdaptiveNav", "NavBar", "NavRail", "NavItem"],
    demo: NavPlaygroundDemo,
    source: "src/components/nav.rs",
}
