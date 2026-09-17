//! Persistent app navigation: bottom bars, side rails, and their items.
use crate::components::pressable::{Pressable, Target};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;

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
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let aria_label = aria_label.unwrap_or_else(|| use_strings().primary_navigation);
    let kind_cls = match kind {
        NavKind::Bar => "",
        NavKind::Rail => "g3-nav-rail",
        NavKind::Adaptive(AdaptiveNavCompact::Bar) => "g3-nav-adaptive",
        NavKind::Adaptive(AdaptiveNavCompact::Hidden) => "g3-nav-adaptive g3-nav-compact-hidden",
    };
    let cls = classes(["g3-nav", mode.pick("g3-nav-ios", "g3-nav-md"), kind_cls]);
    rsx! {
        nav { class: merge_classes(cls, class.as_deref()), aria_label, {children} }
    }
}

/// Navigation that is a bottom bar on compact shells and a side rail on wide
/// ones. Put it last inside a [`TabLayout`](crate::TabLayout).
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
    children: Element,
) -> Element {
    rsx! {
        NavContainer {
            kind: NavKind::Adaptive(compact.unwrap_or_default()),
            aria_label,
            mode,
            class,
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
    children: Element,
) -> Element {
    rsx! {
        NavContainer { kind: NavKind::Rail, aria_label, mode, class, {children} }
    }
}

/// One destination in a [`NavBar`], [`NavRail`], or [`AdaptiveNav`].
///
/// Give it `to` for a router destination; it is then marked current
/// automatically when its route is active. Use `href` for a plain link, or
/// `onclick` with `selected` to manage the current destination yourself.
#[component]
pub fn NavItem(
    /// Visible label. In a rail it appears as a tooltip.
    label: String,
    /// Icon shown above the label.
    icon: Option<Element>,
    /// Router destination.
    #[props(into)]
    to: Option<NavigationTarget>,
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
    rsx! {
        Pressable {
            class: merge_classes(classes(["g3-nav-item", group_cls]), class.as_deref()),
            target,
            disabled: disabled.unwrap_or(false),
            onclick,
            aria_current: selected.unwrap_or(false).then_some("page"),
            span { class: "g3-nav-item-icon", aria_hidden: "true",
                {icon}
                if let Some(badge) = &badge {
                    span { class: "g3-nav-item-badge", "{badge}" }
                }
            }
            span { class: "g3-nav-item-label", "{label}" }
            if let Some(badge) = badge {
                span { class: "g3-sr-only", ", {badge}" }
            }
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
    demo: NavPlaygroundDemo,
    source: "src/components/nav.rs",
}
