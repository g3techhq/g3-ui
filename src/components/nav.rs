//! Persistent app navigation: bottom bars, side rails, and their items.
use super::pressable::Destination;
use crate::components::overlay::js_string;
use crate::components::pressable::{Pressable, Target};
use crate::state::use_synced_signal;
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

/// How a side rail shows its labels: a [`NavRail`], or an [`AdaptiveNav`] on
/// a wide shell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum NavRailBehavior {
    /// A button on the rail's edge expands and collapses it, and the page
    /// makes room for the labels.
    #[default]
    Toggle,
    /// The rail stays icons only and expands over the page while the pointer
    /// is on it or the keyboard is inside it. The page does not move.
    Hover,
}

/// Where a toggled rail's state is kept between routes and visits.
const EXPANDED_STORAGE_KEY: &str = "g3-ui.nav-expanded";

/// The last toggled rail state in this app. Each route renders its own rail,
/// so a new rail starts from this rather than from local storage, which only
/// answers asynchronously and would flash the default first. `None` until the
/// user toggles a rail or storage has been read.
pub(crate) static REMEMBERED_EXPANDED: GlobalSignal<Option<bool>> = Signal::global(|| None);

const READ_EXPANDED_SCRIPT: &str = r#"
let value = null;
try {
    const stored = localStorage.getItem(__KEY__);
    if (stored === "true" || stored === "false") value = stored === "true";
} catch {}
dioxus.send(value);
"#;

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
    behavior: Option<NavRailBehavior>,
    remember: Option<bool>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let has_rail = !matches!(kind, NavKind::Bar);
    let hover = behavior.unwrap_or_default() == NavRailBehavior::Hover;
    // Only the rail's own toggled state is remembered: a caller that passes
    // `expanded` keeps it wherever it likes.
    let remembers = has_rail && !hover && expanded.is_none() && remember.unwrap_or(true);
    // Called unconditionally: a hook may not sit behind a match arm. A bar has
    // no expanded state, so its signal goes unused.
    let mut own = use_signal(|| {
        let fallback = default_expanded.unwrap_or(true);
        if remembers {
            REMEMBERED_EXPANDED.peek().unwrap_or(fallback)
        } else {
            fallback
        }
    });
    let remembers_signal = use_synced_signal(remembers);
    // The first rail in a visit reads what an earlier visit stored. Later
    // rails start from `REMEMBERED_EXPANDED`, which is already set.
    use_effect(move || {
        if !remembers_signal() || REMEMBERED_EXPANDED.peek().is_some() {
            return;
        }
        spawn(async move {
            let script = READ_EXPANDED_SCRIPT.replace("__KEY__", &js_string(EXPANDED_STORAGE_KEY));
            let mut eval = document::eval(&script);
            if let Ok(Some(stored)) = eval.recv::<Option<bool>>().await
                && REMEMBERED_EXPANDED.peek().is_none()
            {
                *REMEMBERED_EXPANDED.write() = Some(stored);
                own.set(stored);
            }
        });
    });
    let mut expanded = expanded.unwrap_or(own);
    let is_expanded = has_rail && !hover && expanded();
    let collapsible = has_rail && !hover && collapsible.unwrap_or(true);
    let strings = use_strings()();
    let aria_label = aria_label.unwrap_or_else(|| strings.primary_navigation.clone());
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
        if has_rail && hover {
            "g3-nav-hover"
        } else {
            ""
        },
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
                    onclick: move |_| {
                        let next = !expanded();
                        expanded.set(next);
                        if remembers_signal() {
                            *REMEMBERED_EXPANDED.write() = Some(next);
                            document::eval(&format!(
                                "try {{ localStorage.setItem({}, \"{next}\"); }} catch {{}}",
                                js_string(EXPANDED_STORAGE_KEY),
                            ));
                        }
                    },
                    {toggle_icon}
                }
            }
        }
    }
}

/// Navigation that is a bottom bar on compact shells and a side rail on wide
/// ones. Put it last inside a [`TabLayout`](crate::TabLayout).
///
/// The rail starts expanded, with a label beside each icon, and a button on
/// its edge collapses it to icons alone. The choice is remembered across
/// routes and visits. Set `behavior` to [`NavRailBehavior::Hover`] for a rail
/// that stays icons only and expands over the page on hover instead.
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
    /// the state; the toggle writes it. Without one the nav keeps its own,
    /// starting from the remembered state (see `remember`), or else from
    /// `default_expanded`.
    expanded: Option<Signal<bool>>,
    /// Whether the rail starts expanded when it owns its state. Defaults to
    /// `true`.
    default_expanded: Option<bool>,
    /// Show the button that expands and collapses the rail. Defaults to
    /// `true`.
    collapsible: Option<bool>,
    /// How the rail shows its labels. Defaults to
    /// [`NavRailBehavior::Toggle`]. With [`NavRailBehavior::Hover`],
    /// `expanded`, `default_expanded`, `collapsible` and `remember` do not
    /// apply.
    behavior: Option<NavRailBehavior>,
    /// Remember the toggled state in local storage, so it carries across
    /// routes and visits. Defaults to `true`. Applies only to a rail that owns
    /// its state; one given `expanded` leaves it to the signal's owner. The
    /// key is `g3-ui.nav-expanded`, shared by every rail on the origin.
    remember: Option<bool>,
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
            behavior,
            remember,
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
///
/// By default a button on its edge expands and collapses the labels, and the
/// choice is remembered across routes and visits. With
/// [`NavRailBehavior::Hover`] it stays icons only and expands over the page
/// while hovered.
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
    /// the state; the toggle writes it. Without one the nav keeps its own,
    /// starting from the remembered state (see `remember`), or else from
    /// `default_expanded`.
    expanded: Option<Signal<bool>>,
    /// Whether the rail starts expanded when it owns its state. Defaults to
    /// `true`.
    default_expanded: Option<bool>,
    /// Show the button that expands and collapses the rail. Defaults to
    /// `true`.
    collapsible: Option<bool>,
    /// How the rail shows its labels. Defaults to
    /// [`NavRailBehavior::Toggle`]. With [`NavRailBehavior::Hover`],
    /// `expanded`, `default_expanded`, `collapsible` and `remember` do not
    /// apply.
    behavior: Option<NavRailBehavior>,
    /// Remember the toggled state in local storage, so it carries across
    /// routes and visits. Defaults to `true`. Applies only to a rail that owns
    /// its state; one given `expanded` leaves it to the signal's owner. The
    /// key is `g3-ui.nav-expanded`, shared by every rail on the origin.
    remember: Option<bool>,
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
            behavior,
            remember,
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
    let behavior = use_signal(|| NavRailBehavior::Toggle);
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
                crate::SegmentGroup { value: layout, label: "Layout",
                    crate::SegmentButton { value: 0_usize, "Adaptive" }
                    crate::SegmentButton { value: 1_usize, "Bar" }
                    crate::SegmentButton { value: 2_usize, "Rail" }
                }
                if layout() != 1 {
                    crate::SegmentGroup { value: behavior, label: "Rail behavior",
                        crate::SegmentButton { value: NavRailBehavior::Toggle, "Toggle" }
                        crate::SegmentButton { value: NavRailBehavior::Hover, "Hover" }
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
                            NavRail { behavior: behavior(), {items} }
                        },
                        _ => rsx! {
                            AdaptiveNav { behavior: behavior(), {items} }
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
