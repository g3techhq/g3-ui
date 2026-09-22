//! Floating panels anchored to a trigger: popovers and menus.
use super::Color;
use super::overlay::{js_string, use_overlay_focus_with, use_top_layer};
use super::pressable::Destination;
use crate::components::pressable::{Pressable, Target};
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// Where a [`Popover`] or [`Menu`] opens relative to its trigger.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum PopoverPlacement {
    /// Below the trigger, aligned to its leading edge.
    #[default]
    BottomStart,
    /// Below the trigger, aligned to its trailing edge.
    BottomEnd,
    /// Above the trigger, aligned to its leading edge.
    TopStart,
    /// Above the trigger, aligned to its trailing edge.
    TopEnd,
}

impl PopoverPlacement {
    fn as_str(self) -> &'static str {
        match self {
            PopoverPlacement::BottomStart => "bottom-start",
            PopoverPlacement::BottomEnd => "bottom-end",
            PopoverPlacement::TopStart => "top-start",
            PopoverPlacement::TopEnd => "top-end",
        }
    }
}

/// Flips an open popover to the other side of its trigger when the requested
/// placement would run past the edge of the app, as a menu at the end of a
/// toolbar would. Sheets are left alone.
const FIT_SCRIPT: &str = r#"
const el = document.getElementById(__ID__);
if (el) {
    const layer = el.closest(".g3-overlay-layer");
    if (layer && typeof layer.showPopover === "function" && !layer.matches(":popover-open")) {
        layer.showPopover();
    } else if (layer && typeof layer.showPopover !== "function") {
        layer.removeAttribute("popover");
    }
    const base = el.dataset.basePlacement || el.getAttribute("data-placement");
    el.dataset.basePlacement = base;
    el.setAttribute("data-placement", base);
    const anchor = el.closest(".g3-popover-anchor");
    const app = el.closest(".g3-app");
    const appBox = app ? app.getBoundingClientRect() : { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    // Compact select sheets keep their bottom-sheet geometry. Every anchored
    // panel uses viewport coordinates; the top-layer wrapper then guarantees
    // that a scroll pane or transformed route cannot clip it.
    const sheet = el.classList.contains("g3-popover-sheet");
    const compactSheet = sheet && appBox.right - appBox.left < 768;
    if (sheet) el.dataset.g3SheetLayout = compactSheet ? "sheet" : "popover";
    el.toggleAttribute("data-g3-fixed-position", !compactSheet);
    if (anchor && !compactSheet) {
        const anchorBox = anchor.getBoundingClientRect();
        el.style.setProperty("--g3-popover-anchor-width", `${anchorBox.width}px`);
        void el.offsetWidth;
        const bounds = {
            left: Math.max(appBox.left, 0) + 8,
            right: Math.min(appBox.right, innerWidth) - 8,
            top: Math.max(appBox.top, 0) + 8,
            bottom: Math.min(appBox.bottom, innerHeight) - 8,
        };
        const menuBox = el.getBoundingClientRect();
        let [vertical, horizontal] = base.split("-");
        const rtl = getComputedStyle(el).direction === "rtl";
        const leftFor = (side) => (side === "start") !== rtl
            ? anchorBox.left
            : anchorBox.right - menuBox.width;
        const topFor = (side) => side === "bottom"
            ? anchorBox.bottom + 6
            : anchorBox.top - menuBox.height - 6;
        let left = leftFor(horizontal);
        let top = topFor(vertical);
        if (left < bounds.left || left + menuBox.width > bounds.right) {
            horizontal = horizontal === "start" ? "end" : "start";
            left = leftFor(horizontal);
        }
        if (top < bounds.top || top + menuBox.height > bounds.bottom) {
            vertical = vertical === "bottom" ? "top" : "bottom";
            top = topFor(vertical);
        }
        el.style.setProperty("--g3-popover-fixed-left", `${Math.max(bounds.left, Math.min(left, bounds.right - menuBox.width))}px`);
        el.style.setProperty("--g3-popover-fixed-top", `${Math.max(bounds.top, Math.min(top, bounds.bottom - menuBox.height))}px`);
        el.setAttribute("data-placement", `${vertical}-${horizontal}`);
    }
}
"#;

/// Describes the panel on the control that opens it: what kind of popup it
/// is, whether it is open, and which element it is. The trigger is the
/// caller's own element, so these are set on it from the page. Attributes the
/// caller already set are left alone.
const TRIGGER_SCRIPT: &str = r#"
const anchor = document.getElementById(__ANCHOR__);
const trigger = anchor && [...anchor.children].find((node) => !node.matches(".g3-popover, .g3-popover-backdrop"));
const control = trigger && (trigger.matches("button, a, [role=button], [tabindex]") ? trigger : trigger.querySelector("button, a, [role=button], [tabindex]"));
if (control) {
    if (!control.hasAttribute("aria-haspopup") || control.dataset.g3Popup === "true") {
        control.setAttribute("aria-haspopup", __POPUP__);
        control.dataset.g3Popup = "true";
    }
    control.setAttribute("aria-expanded", __OPEN__);
    control.setAttribute("aria-controls", __PANEL__);
}
"#;

#[component]
pub(crate) fn PopoverFrame(
    open: Signal<bool>,
    trigger: Element,
    placement: PopoverPlacement,
    sheet_on_compact: bool,
    role: &'static str,
    roving: Option<&'static str>,
    id: Option<String>,
    aria_label: Option<String>,
    aria_labelledby: Option<String>,
    on_dismiss: Option<EventHandler<()>>,
    mode: Option<ComponentMode>,
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let id = use_element_id("popover", id);
    let anchor_id = format!("{id}-anchor");
    let dismiss = use_callback(move |()| {
        let mut open = open;
        open.set(false);
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.call(());
        }
    });
    use_overlay_focus_with(open.into(), id.clone(), false, roving, dismiss);
    let layer_id = format!("{id}-layer");
    use_top_layer(
        open.into(),
        layer_id.clone(),
        std::time::Duration::from_millis(300),
    );
    let mut ever_opened = use_signal(|| false);
    let starts_open = open();
    let mut backdrop_present = use_signal(move || starts_open);
    {
        let id = id.clone();
        use_effect(move || {
            let is_open = open();
            if is_open {
                ever_opened.set(true);
                backdrop_present.set(true);
                document::eval(&FIT_SCRIPT.replace("__ID__", &js_string(&id)));
            } else if backdrop_present() {
                spawn(async move {
                    dioxus_sdk_time::sleep(std::time::Duration::from_millis(300)).await;
                    if !open() {
                        backdrop_present.set(false);
                    }
                });
            }
            let popup = match role {
                "menu" | "listbox" => role,
                _ => "dialog",
            };
            document::eval(
                &TRIGGER_SCRIPT
                    .replace("__ANCHOR__", &js_string(&format!("{id}-anchor")))
                    .replace("__POPUP__", &js_string(popup))
                    .replace("__OPEN__", &js_string(&is_open.to_string()))
                    .replace("__PANEL__", &js_string(&id)),
            );
        });
    }
    let is_open = open();
    let state = if is_open { "open" } else { "closed" };
    let cls = classes([
        "g3-popover",
        mode.pick("g3-popover-ios", "g3-popover-md"),
        if sheet_on_compact {
            "g3-popover-sheet"
        } else {
            ""
        },
    ]);
    rsx! {
        span { id: anchor_id, class: "g3-popover-anchor",
            {trigger}
            if is_open || ever_opened() || backdrop_present() {
                div {
                    id: layer_id,
                    class: "g3-overlay-layer",
                    popover: "manual",
                    if is_open || backdrop_present() {
                        div {
                            class: "g3-popover-backdrop",
                            "data-state": state,
                            aria_hidden: "true",
                            onclick: move |_| dismiss.call(()),
                        }
                    }
                    if is_open || ever_opened() {
                        div {
                            id,
                            class: merge_classes(cls, class.as_deref()),
                            role,
                            aria_label,
                            aria_labelledby,
                            tabindex: "-1",
                            "data-state": state,
                            "data-placement": placement.as_str(),
                            aria_hidden: (!is_open).then_some("true"),
                            inert: (!is_open).then_some(true),
                            {children}
                        }
                    }
                }
            }
        }
    }
}

/// A panel of content floating beside a trigger, such as a filter form or a
/// profile card. Like Ionic's `ion-popover`.
///
/// The trigger is any element; give it `aria_haspopup: "dialog"` and
/// `aria_expanded` so assistive technology knows what it opens. Escape and a
/// tap outside close the popover. With `sheet_on_compact`, it shows as a
/// bottom sheet on shells narrower than `48rem`.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let mut open = use_signal(|| false);
/// # #[component] fn FilterForm() -> Element { rsx! {} }
/// rsx! {
///     Popover { open,
///         trigger: rsx! {
///             Button { aria_haspopup: "dialog", aria_expanded: open().to_string(),
///                 onclick: move |_| open.toggle(), "Filters" }
///         },
///         FilterForm {}
///     }
/// }
/// # }
/// ```
#[component]
pub fn Popover(
    /// Whether the popover is open.
    open: Signal<bool>,
    /// The element the popover is anchored to.
    trigger: Element,
    /// Where it opens. Defaults to [`PopoverPlacement::BottomStart`].
    placement: Option<PopoverPlacement>,
    /// Show as a bottom sheet on compact shells.
    sheet_on_compact: Option<bool>,
    /// Accessible name of the panel.
    aria_label: Option<String>,
    /// Called after the user dismisses the popover.
    on_dismiss: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the panel.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        PopoverFrame {
            open,
            trigger,
            placement: placement.unwrap_or_default(),
            sheet_on_compact: sheet_on_compact.unwrap_or(false),
            role: "dialog",
            aria_label,
            on_dismiss,
            mode,
            class,
            div { class: "g3-popover-content", {children} }
        }
    }
}

#[derive(Clone, Copy)]
struct MenuContext {
    open: Signal<bool>,
}

/// A menu of actions anchored to a trigger. Like a desktop dropdown menu.
///
/// Arrow keys, Home, and End move between items; Escape closes the menu.
/// Choosing an item closes it.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let mut open = use_signal(|| false);
/// # #[component] fn Ellipsis() -> Element { rsx! {} }
/// # fn rename() {}
/// # fn delete() {}
/// rsx! {
///     Menu { open,
///         trigger: rsx! {
///             Button { fill: ButtonFill::Clear, aria_label: "More", aria_haspopup: "menu",
///                 aria_expanded: open().to_string(), onclick: move |_| open.toggle(), Ellipsis {} }
///         },
///         MenuItem { onclick: move |_| rename(), "Rename" }
///         MenuItem { color: Color::Danger, onclick: move |_| delete(), "Delete" }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Menu(
    /// Whether the menu is open.
    open: Signal<bool>,
    /// The element the menu is anchored to.
    trigger: Element,
    /// Where it opens. Defaults to [`PopoverPlacement::BottomStart`].
    placement: Option<PopoverPlacement>,
    /// Accessible name of the menu.
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the menu panel.
    class: Option<String>,
    children: Element,
) -> Element {
    use_context_provider(|| MenuContext { open });
    rsx! {
        PopoverFrame {
            open,
            trigger,
            placement: placement.unwrap_or_default(),
            sheet_on_compact: false,
            role: "menu",
            roving: Some("[role=menuitem]"),
            aria_label,
            mode,
            class: merge_classes("g3-menu", class.as_deref()),
            {children}
        }
    }
}

/// One action in a [`Menu`].
#[component]
pub fn MenuItem(
    /// Content before the label, usually an icon.
    start: Option<Element>,
    /// Content after the label, such as a keyboard shortcut.
    end: Option<Element>,
    /// Color. Only [`Color::Danger`] and [`Color::Accent`] change the item.
    color: Option<Color>,
    /// Disable the item.
    disabled: Option<bool>,
    /// Router destination.
    #[props(default, into)]
    to: Destination,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Called when chosen.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Extra classes for the item.
    class: Option<String>,
    children: Element,
) -> Element {
    let context = try_use_context::<MenuContext>();
    let color_cls = match color {
        Some(Color::Danger) => "g3-menu-item-danger",
        Some(Color::Accent) => "g3-menu-item-accent",
        _ => "",
    };
    rsx! {
        Pressable {
            class: merge_classes(classes(["g3-menu-item", color_cls]), class.as_deref()),
            target: Target::from_props(href, to, false),
            disabled: disabled.unwrap_or(false),
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
                if let Some(MenuContext { mut open }) = context {
                    open.set(false);
                }
            },
            attributes: vec![
                Attribute::new("role", "menuitem", None, false),
                Attribute::new("tabindex", "-1", None, false),
            ],
            if let Some(start) = start {
                span { class: "g3-menu-item-start", aria_hidden: "true", {start} }
            }
            span { class: "g3-menu-item-label", {children} }
            if let Some(end) = end {
                span { class: "g3-menu-item-end", {end} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn MenuPlaygroundDemo() -> Element {
    let show_popover = use_signal(|| false);
    let sheet_on_compact = use_signal(|| true);
    let mut menu_open = use_signal(|| false);
    let mut popover_open = use_signal(|| false);
    let mut last = use_signal(|| "Nothing yet".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::SegmentGroup { value: show_popover, aria_label: "Component",
                    crate::SegmentButton { value: false, "Menu" }
                    crate::SegmentButton { value: true, "Popover" }
                }
                if show_popover() {
                    crate::Checkbox { checked: sheet_on_compact, label: "Sheet on phones" }
                    crate::Text {
                        variant: crate::TextVariant::Caption,
                        tone: crate::TextTone::Secondary,
                        "A popover holds extra detail about its trigger. On a phone it can open as a bottom sheet instead."
                    }
                } else {
                    crate::Text {
                        variant: crate::TextVariant::Caption,
                        tone: crate::TextTone::Secondary,
                        "A menu lists actions for its trigger. Arrow keys move between items."
                    }
                }
            },
            if show_popover() {
                Popover {
                    open: popover_open,
                    sheet_on_compact: sheet_on_compact(),
                    aria_label: "Player",
                    trigger: rsx! {
                        crate::Button {
                            fill: crate::ButtonFill::Outline,
                            aria_haspopup: "dialog",
                            aria_expanded: popover_open().to_string(),
                            onclick: move |_| popover_open.toggle(),
                            "Alex Morgan"
                        }
                    },
                    crate::Stack { gap: crate::Space::Xs,
                        crate::Text { variant: crate::TextVariant::Heading, "Alex Morgan" }
                        crate::Text { tone: crate::TextTone::Secondary, "Handicap 12 · 18 rounds this season" }
                    }
                }
            } else {
                crate::Stack { align: crate::StackAlign::Center,
                    Menu {
                        open: menu_open,
                        trigger: rsx! {
                            crate::Button {
                                fill: crate::ButtonFill::Outline,
                                aria_haspopup: "menu",
                                aria_expanded: menu_open().to_string(),
                                onclick: move |_| menu_open.toggle(),
                                "Actions"
                            }
                        },
                        MenuItem { onclick: move |_| last.set("Rename".into()), "Rename" }
                        MenuItem { onclick: move |_| last.set("Duplicate".into()), "Duplicate" }
                        MenuItem {
                            color: Color::Danger,
                            onclick: move |_| last.set("Delete".into()),
                            "Delete"
                        }
                    }
                    crate::Text { tone: crate::TextTone::Secondary, "Last action: {last}" }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Menu",
    description: "Menus of actions and popovers of detail, anchored to the control that opens them.",
    components: ["Menu", "MenuItem", "Popover"],
    demo: MenuPlaygroundDemo,
    source: "src/components/popover.rs",
}
