//! Fab component - Floating Action Button with full Ionic parity.
//! Supports: Fab container, FabButton, FabList.

use super::fab_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
use dioxus_icons::lucide::X;

/// Fab container vertical alignment
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FabVertical {
    #[default]
    Top,
    Center,
    Bottom,
}

/// Fab container horizontal alignment
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FabHorizontal {
    #[default]
    Start,
    Center,
    End,
}

/// FabList side relative to the main FabButton
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FabListSide {
    #[default]
    Top,
    Bottom,
    Start,
    End,
}

/// Fab button size
#[derive(Clone, Copy, PartialEq, Default)]
pub enum FabSize {
    #[default]
    Normal,
    Small,
}

/// Fab container - wraps buttons and lists, handles fixed positioning.
#[component]
pub fn Fab(
    children: Element,
    #[props(extends=Input)] attributes: Vec<Attribute>,
    vertical: Option<FabVertical>,
    horizontal: Option<FabHorizontal>,
    edge: Option<bool>,
    class: Option<String>,
) -> Element {
    let vert = vertical.unwrap_or_default();
    let horiz = horizontal.unwrap_or_default();
    let is_edge = edge.unwrap_or(false);

    let vert_cls = match vert {
        FabVertical::Top => "top-0",
        FabVertical::Center => "top-[50%] -translate-y-[50%]",
        FabVertical::Bottom => "bottom-6",
    };
    let horiz_cls = match horiz {
        FabHorizontal::Start => "left-6",
        FabHorizontal::Center => "left-[50%] -translate-x-[50%]",
        FabHorizontal::End => "right-6",
    };
    let edge_cls = if is_edge {
        "mt-[-3.5rem] mb-[-3.5rem] "
    } else {
        ""
    };

    let container_cls = merge_classes(
        format!("absolute z-39 {vert_cls} {horiz_cls} {edge_cls} flex flex-col items-center"),
        class.as_deref(),
    );

    let final_attributes: Vec<Attribute> = attributes
        .into_iter()
        .filter(|attr| attr.name.as_ref() as &str != "onclick")
        .collect();

    rsx! {
        div {
            class: container_cls,
            ..final_attributes,
            {children}
        }
    }
}

/// FabButton - the primary circular action button.
#[component]
pub fn FabButton(
    children: Element,
    #[props(extends=Input)] attributes: Vec<Attribute>,
    activated: Option<bool>,
    close_icon: Option<Element>,
    size: Option<FabSize>,
    translucent: Option<bool>,
    onclick: Option<Callback<Event<MouseData>>>,
    href: Option<String>,
    target: Option<String>,
    disabled: Option<bool>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);
    let is_activated = activated.unwrap_or(false);
    let is_small = size.unwrap_or_default() == FabSize::Small;
    let is_translucent = translucent.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);

    let fab_cls = match mode {
        ComponentMode::Ios => s::FAB_IOS,
        ComponentMode::Md => s::FAB_MD,
    };

    let size_cls = if is_small { s::FAB_SMALL } else { "" };
    let translucent_cls = if is_translucent && mode == ComponentMode::Ios {
        s::FAB_TRANSLUCENT
    } else {
        ""
    };

    let btn_cls = merge_classes(
        format!("{} {fab_cls} {size_cls} {translucent_cls}", s::FAB),
        class.as_deref(),
    );

    let final_attributes: Vec<Attribute> = attributes
        .into_iter()
        .filter(|attr| attr.name.as_ref() as &str != "onclick")
        .collect();

    if let Some(ref href) = href {
        rsx! {
            a {
                href: href,
                target: target.clone(),
                class: btn_cls,
                aria_disabled: is_disabled.to_string(),
                ..final_attributes,
                if is_activated && close_icon.is_some() {
                    {close_icon}
                } else {
                    {children}
                }
            }
        }
    } else {
        rsx! {
            button {
                class: btn_cls,
                r#type: "button",
                disabled: is_disabled,
                onclick: move |event| {
                    if is_disabled { return; }
                    if let Some(ref handler) = onclick {
                        handler.call(event);
                    }
                },
                ..final_attributes,
                if is_activated && close_icon.is_some() {
                    {close_icon}
                } else {
                    {children}
                }
            }
        }
    }
}

/// FabList - expandable list of secondary fab buttons.
#[component]
pub fn FabList(
    children: Element,
    #[props(extends=Input)] attributes: Vec<Attribute>,
    activated: Option<bool>,
    side: Option<FabListSide>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);
    let is_activated = activated.unwrap_or(false);
    let side = side.unwrap_or_default();

    let list_cls = match mode {
        ComponentMode::Ios => s::FAB_LIST_IOS,
        ComponentMode::Md => s::FAB_LIST_MD,
    };

    let side_cls = match side {
        FabListSide::Top => s::FAB_LIST_TOP,
        FabListSide::Bottom => s::FAB_LIST_BOTTOM,
        FabListSide::Start => s::FAB_LIST_START,
        FabListSide::End => s::FAB_LIST_END,
    };

    let final_attributes: Vec<Attribute> = attributes.into_iter().collect();

    rsx! {
        div {
            class: merge_classes(
                format!("{} {} {} flex-col gap-2", s::FAB_LIST_BASE, list_cls, side_cls),
                class.as_deref(),
            ),
            aria_hidden: (!is_activated).to_string(),
            ..final_attributes,
            if !is_activated {
                div { style: "display: none;", {children} }
            } else {
                {children}
            }
        }
    }
}

/// Convenience component: Fab with integrated list.
/// Wraps Fab + FabButton + FabList into a single component with activation toggle.
#[component]
pub fn FabContainer(
    main_button: Element,
    list_buttons: Option<Element>,
    vertical: Option<FabVertical>,
    horizontal: Option<FabHorizontal>,
    edge: Option<bool>,
    list_side: Option<FabListSide>,
    class: Option<String>,
) -> Element {
    let mut is_activated = use_signal(|| false);

    let toggle = Callback::new(move |_| {
        is_activated.with_mut(|active| *active = !*active);
    });

    rsx! {
        Fab {
            vertical,
            horizontal,
            edge,
            class,
            FabButton {
                activated: is_activated(),
                onclick: toggle,
                close_icon: rsx! {
                    X { size: 24 }
                },
                {main_button}
            }
            if let Some(list) = list_buttons {
                FabList {
                    activated: is_activated(),
                    side: list_side,
                    {list}
                }
            }
        }
    }
}
#[cfg(feature = "playground")]
#[component]
pub fn FabPlaygroundDemo() -> Element {
    let mut activated = use_signal(|| true);
    let mut small = use_signal(|| false);
    let mut edge = use_signal(|| false);
    let mut vertical = use_signal(|| FabVertical::Bottom);
    let mut horizontal = use_signal(|| FabHorizontal::End);
    let mut list_side = use_signal(|| FabListSide::Top);

    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                div { class: "g3-playground-control",
                    span { "Vertical" }
                    div { class: "g3-playground-segments",
                        button { class: if vertical() == FabVertical::Top { "selected" } else { "" }, r#type: "button", onclick: move |_| vertical.set(FabVertical::Top), "Top" }
                        button { class: if vertical() == FabVertical::Center { "selected" } else { "" }, r#type: "button", onclick: move |_| vertical.set(FabVertical::Center), "Center" }
                        button { class: if vertical() == FabVertical::Bottom { "selected" } else { "" }, r#type: "button", onclick: move |_| vertical.set(FabVertical::Bottom), "Bottom" }
                    }
                }
                div { class: "g3-playground-control",
                    span { "Horizontal" }
                    div { class: "g3-playground-segments",
                        button { class: if horizontal() == FabHorizontal::Start { "selected" } else { "" }, r#type: "button", onclick: move |_| horizontal.set(FabHorizontal::Start), "Start" }
                        button { class: if horizontal() == FabHorizontal::Center { "selected" } else { "" }, r#type: "button", onclick: move |_| horizontal.set(FabHorizontal::Center), "Center" }
                        button { class: if horizontal() == FabHorizontal::End { "selected" } else { "" }, r#type: "button", onclick: move |_| horizontal.set(FabHorizontal::End), "End" }
                    }
                }
                div { class: "g3-playground-control",
                    span { "List side" }
                    div { class: "g3-playground-segments",
                        button { class: if list_side() == FabListSide::Top { "selected" } else { "" }, r#type: "button", onclick: move |_| list_side.set(FabListSide::Top), "Top" }
                        button { class: if list_side() == FabListSide::Bottom { "selected" } else { "" }, r#type: "button", onclick: move |_| list_side.set(FabListSide::Bottom), "Bottom" }
                        button { class: if list_side() == FabListSide::Start { "selected" } else { "" }, r#type: "button", onclick: move |_| list_side.set(FabListSide::Start), "Start" }
                        button { class: if list_side() == FabListSide::End { "selected" } else { "" }, r#type: "button", onclick: move |_| list_side.set(FabListSide::End), "End" }
                    }
                }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: activated(), onchange: move |_| activated.toggle() } span { "List active" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: small(), onchange: move |_| small.toggle() } span { "Small main button" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: edge(), onchange: move |_| edge.toggle() } span { "Edge" } }
            },
            crate::Card { title: "Actions", "FAB is anchored to the body, and the list opens from the selected side." }
            crate::Card { title: "Content", "The button stays over scrolling body content." }
            Fab {
                vertical: vertical(),
                horizontal: horizontal(),
                edge: edge(),
                FabButton { onclick: |_| {}, size: if small() { FabSize::Small } else { FabSize::Normal }, "+" }
                FabList { activated: activated(), side: list_side(),
                    FabButton { size: FabSize::Small, onclick: |_| {}, "A" }
                    FabButton { size: FabSize::Small, onclick: |_| {}, "B" }
                    FabButton { size: FabSize::Small, onclick: |_| {}, "C" }
                }
            }
        }
    }
}
crate::g3_playground! {
    name: "Fab",
    g3_name: "G3Fab",
    description: "Floating action button container, button, and expandable list.",
    demo: FabPlaygroundDemo,
}
