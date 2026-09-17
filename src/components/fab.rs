//! Floating action buttons.
use crate::components::pressable::{Pressable, Target};
use crate::state::{use_controlled, use_element_id};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::X;

/// Vertical position of a [`Fab`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FabVertical {
    /// Top edge.
    Top,
    /// Centre.
    Center,
    /// Bottom edge.
    #[default]
    Bottom,
}

/// Horizontal position of a [`Fab`], in reading direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FabHorizontal {
    /// Leading edge.
    Start,
    /// Centre.
    Center,
    /// Trailing edge.
    #[default]
    End,
}

/// Which way a [`FabList`] opens from its button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FabListSide {
    /// Upward.
    #[default]
    Top,
    /// Downward.
    Bottom,
    /// Toward the leading edge.
    Start,
    /// Toward the trailing edge.
    End,
}

/// [`FabButton`] size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum FabSize {
    /// 56px.
    #[default]
    Regular,
    /// 40px, for the buttons in a [`FabList`].
    Small,
}

/// Positions floating action buttons over the page. Like Ionic's `ion-fab`.
/// Pass it to [`Content`](crate::Content)'s `fab` slot so it stays put while
/// the content scrolls.
///
/// ```rust,ignore
/// Content { fab: rsx! { Fab { FabButton { aria_label: "New round", onclick: new_round, Plus {} } } }, .. }
/// ```
#[component]
pub fn Fab(
    /// Vertical position. Defaults to [`FabVertical::Bottom`].
    vertical: Option<FabVertical>,
    /// Horizontal position. Defaults to [`FabHorizontal::End`].
    horizontal: Option<FabHorizontal>,
    /// Straddle the top or bottom edge, half over a header or footer.
    edge: Option<bool>,
    /// Extra classes for the container.
    class: Option<String>,
    children: Element,
) -> Element {
    let vertical_cls = match vertical.unwrap_or_default() {
        FabVertical::Top => "g3-fab-vertical-top",
        FabVertical::Center => "g3-fab-vertical-center",
        FabVertical::Bottom => "g3-fab-vertical-bottom",
    };
    let horizontal_cls = match horizontal.unwrap_or_default() {
        FabHorizontal::Start => "g3-fab-horizontal-start",
        FabHorizontal::Center => "g3-fab-horizontal-center",
        FabHorizontal::End => "g3-fab-horizontal-end",
    };
    let cls = classes([
        "g3-fab-container",
        vertical_cls,
        horizontal_cls,
        if edge.unwrap_or(false) {
            "g3-fab-container-edge"
        } else {
            ""
        },
    ]);
    rsx! {
        div { class: merge_classes(cls, class.as_deref()), {children} }
    }
}

/// A round floating action button. Like Ionic's `ion-fab-button`.
///
/// It usually holds only an icon, so give it an `aria_label`.
#[component]
pub fn FabButton(
    /// Accessible name. Required for icon-only buttons.
    aria_label: Option<String>,
    /// Size. Defaults to [`FabSize::Regular`].
    size: Option<FabSize>,
    /// Use a translucent surface on iOS.
    translucent: Option<bool>,
    /// Disable the button.
    disabled: Option<bool>,
    /// Router destination. Renders a link.
    #[props(into)]
    to: Option<NavigationTarget>,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Called when pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the button.
    class: Option<String>,
    /// Any other HTML attribute, such as `aria_expanded`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let cls = classes([
        "g3-fab",
        mode.pick("g3-fab-ios", "g3-fab-md"),
        if size.unwrap_or_default() == FabSize::Small {
            "g3-fab-small"
        } else {
            ""
        },
        if translucent.unwrap_or(false) && mode == ComponentMode::Ios {
            "g3-fab-translucent"
        } else {
            ""
        },
    ]);
    let mut attributes = attributes;
    if let Some(label) = aria_label {
        attributes.push(Attribute::new("aria-label", label, None, false));
    }
    rsx! {
        Pressable {
            class: merge_classes(cls, class.as_deref()),
            target: Target::from_props(href, to, false),
            disabled: disabled.unwrap_or(false),
            onclick,
            attributes,
            {children}
        }
    }
}

/// Secondary buttons that open from a [`FabButton`]. Like Ionic's
/// `ion-fab-list`. [`FabMenu`] wires one up for you.
#[component]
pub fn FabList(
    /// Whether the list is open.
    open: bool,
    /// Which way it opens. Defaults to [`FabListSide::Top`].
    side: Option<FabListSide>,
    /// Element id, for the button's `aria_controls`.
    id: Option<String>,
    /// Extra classes for the list.
    class: Option<String>,
    children: Element,
) -> Element {
    let side_cls = match side.unwrap_or_default() {
        FabListSide::Top => "g3-fab-list-top",
        FabListSide::Bottom => "g3-fab-list-bottom",
        FabListSide::Start => "g3-fab-list-start",
        FabListSide::End => "g3-fab-list-end",
    };
    rsx! {
        div {
            id,
            class: merge_classes(classes(["g3-fab-list", side_cls]), class.as_deref()),
            aria_hidden: (!open).to_string(),
            inert: (!open).then_some(true),
            {children}
        }
    }
}

/// A floating action button that opens a list of secondary actions, with the
/// open state and close icon handled for you. A speed dial.
///
/// ```rust,ignore
/// rsx! {
///     FabMenu { aria_label: "Create", icon: rsx! { Plus {} },
///         FabButton { size: FabSize::Small, aria_label: "Round", onclick: new_round, Flag {} }
///         FabButton { size: FabSize::Small, aria_label: "Player", onclick: new_player, User {} }
///     }
/// }
/// ```
#[component]
pub fn FabMenu(
    /// Accessible name of the main button.
    aria_label: String,
    /// Icon of the main button while closed.
    icon: Element,
    /// Whether the list is open. Kept internally when not given.
    open: Option<Signal<bool>>,
    /// Which way the list opens. Defaults to [`FabListSide::Top`].
    side: Option<FabListSide>,
    /// Vertical position. Defaults to [`FabVertical::Bottom`].
    vertical: Option<FabVertical>,
    /// Horizontal position. Defaults to [`FabHorizontal::End`].
    horizontal: Option<FabHorizontal>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the container.
    class: Option<String>,
    /// The secondary [`FabButton`]s.
    children: Element,
) -> Element {
    let mut open = use_controlled(open, || false);
    let list_id = use_element_id("fab-list", None);
    let close_label = use_strings().close;
    let is_open = open();
    rsx! {
        Fab { vertical, horizontal, class,
            FabButton {
                aria_label: if is_open { close_label } else { aria_label },
                aria_expanded: is_open.to_string(),
                aria_controls: list_id.clone(),
                mode,
                onclick: move |_| open.toggle(),
                if is_open {
                    X { size: 24 }
                } else {
                    {icon}
                }
            }
            FabList { open: is_open, side, id: list_id, {children} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn FabPlaygroundDemo() -> Element {
    use dioxus_icons::lucide::{Flag, Plus, UserPlus};
    let open = use_signal(|| true);
    let vertical = use_signal(|| FabVertical::Bottom);
    let horizontal = use_signal(|| FabHorizontal::End);
    let side = use_signal(|| FabListSide::Top);
    let mode = crate::use_component_mode(None);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            center: false,
            controls: rsx! {
                crate::SegmentGroup { value: vertical, aria_label: "Vertical",
                    crate::SegmentButton { value: FabVertical::Top, "Top" }
                    crate::SegmentButton { value: FabVertical::Center, "Center" }
                    crate::SegmentButton { value: FabVertical::Bottom, "Bottom" }
                }
                crate::SegmentGroup { value: horizontal, aria_label: "Horizontal",
                    crate::SegmentButton { value: FabHorizontal::Start, "Start" }
                    crate::SegmentButton { value: FabHorizontal::Center, "Center" }
                    crate::SegmentButton { value: FabHorizontal::End, "End" }
                }
                crate::SegmentGroup { value: side, aria_label: "List side",
                    crate::SegmentButton { value: FabListSide::Top, "Top" }
                    crate::SegmentButton { value: FabListSide::Bottom, "Bottom" }
                    crate::SegmentButton { value: FabListSide::Start, "Start" }
                    crate::SegmentButton { value: FabListSide::End, "End" }
                }
            },
            crate::AppWrapper { mode, class: "g3-playground-device-app",
                crate::Header { title: "Fab" }
                crate::Content {
                    fab: rsx! {
                        FabMenu {
                            aria_label: "Create",
                            icon: rsx! {
                                Plus { size: 24 }
                            },
                            open,
                            side: side(),
                            vertical: vertical(),
                            horizontal: horizontal(),
                            FabButton { size: FabSize::Small, aria_label: "New round",
                                Flag { size: 18 }
                            }
                            FabButton { size: FabSize::Small, aria_label: "Invite player",
                                UserPlus { size: 18 }
                            }
                        }
                    },
                    for index in 1..=8 {
                        crate::Card { title: format!("Round {index}"), "Scroll to see the button stay put." }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Fab",
    description: "Floating action buttons and a speed-dial menu.",
    components: ["FabMenu", "FabButton", "FabList", "Fab"],
    demo: FabPlaygroundDemo,
    source: "src/components/fab.rs",
}
