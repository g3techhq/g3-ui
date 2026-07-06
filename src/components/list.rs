//! General mobile list, item, and swipe row components.

use super::list_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeSide {
    Start,
    End,
}

pub const DEFAULT_SWIPE_ACTION_WIDTH: f64 = 88.0;
pub const FULL_SWIPE_MARGIN: f64 = 30.0;
pub const ELASTIC_FACTOR: f64 = 0.55;
pub const LONG_PRESS_MS: u64 = 500;
pub const LONG_PRESS_CANCEL_DISTANCE: f64 = 8.0;

pub fn elastic_swipe_offset(raw_offset: f64, action_width: f64) -> f64 {
    let limit = action_width.max(1.0);
    if raw_offset > limit {
        limit + (raw_offset - limit) * ELASTIC_FACTOR
    } else if raw_offset < -limit {
        -limit + (raw_offset + limit) * ELASTIC_FACTOR
    } else {
        raw_offset
    }
}

pub fn should_full_swipe(offset: f64, action_width: f64) -> bool {
    offset.abs() >= action_width.max(1.0) + FULL_SWIPE_MARGIN
}

pub fn swipe_side(offset: f64) -> Option<SwipeSide> {
    if offset > 0.0 {
        Some(SwipeSide::Start)
    } else if offset < 0.0 {
        Some(SwipeSide::End)
    } else {
        None
    }
}

pub fn swipe_ratio(offset: f64, action_width: f64) -> f64 {
    offset / action_width.max(1.0)
}

pub fn should_cancel_long_press(delta_x: f64, delta_y: f64) -> bool {
    delta_x.hypot(delta_y) > LONG_PRESS_CANCEL_DISTANCE
}

pub fn is_swipe_side_available(
    offset: f64,
    has_start_actions: bool,
    has_end_actions: bool,
) -> bool {
    match swipe_side(offset) {
        Some(SwipeSide::Start) => has_start_actions,
        Some(SwipeSide::End) => has_end_actions,
        None => false,
    }
}

pub fn constrained_swipe_offset(
    raw_offset: f64,
    action_width: f64,
    has_start_actions: bool,
    has_end_actions: bool,
) -> f64 {
    if is_swipe_side_available(raw_offset, has_start_actions, has_end_actions) {
        elastic_swipe_offset(raw_offset, action_width)
    } else {
        0.0
    }
}

fn swipe_state(offset: f64, action_width: f64, full: bool) -> Option<SwipeState> {
    swipe_side(offset).map(|side| SwipeState {
        side,
        offset,
        ratio: swipe_ratio(offset, action_width),
        full,
    })
}

fn settled_swipe_offset(offset: f64, action_width: f64) -> f64 {
    if offset.abs() > action_width.max(1.0) / 2.0 {
        match swipe_side(offset) {
            Some(SwipeSide::Start) => action_width.max(1.0),
            Some(SwipeSide::End) => -action_width.max(1.0),
            None => 0.0,
        }
    } else {
        0.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ListLines {
    Full,
    #[default]
    Inset,
    None,
}

impl ListLines {
    fn class(self) -> &'static str {
        match self {
            Self::Full => s::ITEM_LINES_FULL,
            Self::Inset => s::ITEM_LINES_INSET,
            Self::None => s::ITEM_LINES_NONE,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub enum ItemKind {
    #[default]
    Static,
    Button,
    Link(String),
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemDetail {
    #[default]
    Auto,
    Show,
    Hide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeState {
    pub side: SwipeSide,
    pub offset: f64,
    pub ratio: f64,
    pub full: bool,
}

#[component]
pub fn List(
    inset: Option<bool>,
    lines: Option<ListLines>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let mode_cls = match mode {
        ComponentMode::Ios => s::LIST_IOS,
        ComponentMode::Md => s::LIST_MD,
    };
    let inset_cls = if inset.unwrap_or(false) {
        s::LIST_INSET
    } else {
        ""
    };
    let lines = match lines.unwrap_or_default() {
        ListLines::Full => "full",
        ListLines::Inset => "inset",
        ListLines::None => "none",
    };
    rsx! { div { class: merge_classes(format!("{} {mode_cls} {inset_cls}", s::LIST), class.as_deref()), role: "list", "data-lines": lines, {children} } }
}

#[component]
pub fn Item(
    kind: Option<ItemKind>,
    lines: Option<ListLines>,
    selected: Option<bool>,
    disabled: Option<bool>,
    detail: Option<ItemDetail>,
    start: Option<Element>,
    end: Option<Element>,
    overline: Option<String>,
    label: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Option<Element>,
) -> Element {
    let mode = use_component_mode(mode);
    let kind = match (kind.unwrap_or_default(), onclick.is_some()) {
        (ItemKind::Static, true) => ItemKind::Button,
        (kind, _) => kind,
    };
    let disabled = disabled.unwrap_or(false);
    let selected = selected.unwrap_or(false);
    let detail = detail.unwrap_or_default();
    let mode_cls = match mode {
        ComponentMode::Ios => s::ITEM_IOS,
        ComponentMode::Md => s::ITEM_MD,
    };
    let interactive = !matches!(kind, ItemKind::Static) || onclick.is_some();
    let show_detail = matches!(detail, ItemDetail::Show)
        || (matches!(detail, ItemDetail::Auto) && interactive && mode == ComponentMode::Ios);
    let cls = merge_classes(
        format!(
            "{} {mode_cls} {} {} {} {}",
            s::ITEM,
            lines.map(ListLines::class).unwrap_or(""),
            if interactive { s::ITEM_BUTTON } else { "" },
            if selected { s::ITEM_SELECTED } else { "" },
            if disabled { s::ITEM_DISABLED } else { "" }
        ),
        class.as_deref(),
    );
    let aria_disabled = disabled.then(|| "true".to_string());
    match kind {
        ItemKind::Link(href) if !disabled => {
            rsx! { div { class: s::ITEM_ROW, role: "listitem", a { class: cls, href, if let Some(start) = start { span { class: s::ITEM_START, {start} } } span { class: s::ITEM_MAIN, if let Some(overline) = overline { span { class: s::ITEM_OVERLINE, "{overline}" } } if let Some(label) = label { span { class: s::ITEM_LABEL, "{label}" } } if let Some(description) = description { span { class: s::ITEM_DESCRIPTION, "{description}" } } if let Some(children) = children { {children} } } if let Some(metadata) = metadata { span { class: s::ITEM_METADATA, "{metadata}" } } if let Some(end) = end { span { class: s::ITEM_END, {end} } } if show_detail { span { aria_hidden: "true", ChevronRight { class: s::ITEM_DETAIL, size: 18 } } } } } }
        }
        ItemKind::Link(_) => {
            rsx! { div { class: s::ITEM_ROW, role: "listitem", div { class: cls, role: "link", aria_disabled, if let Some(start) = start { span { class: s::ITEM_START, {start} } } span { class: s::ITEM_MAIN, if let Some(overline) = overline { span { class: s::ITEM_OVERLINE, "{overline}" } } if let Some(label) = label { span { class: s::ITEM_LABEL, "{label}" } } if let Some(description) = description { span { class: s::ITEM_DESCRIPTION, "{description}" } } if let Some(children) = children { {children} } } if let Some(metadata) = metadata { span { class: s::ITEM_METADATA, "{metadata}" } } if let Some(end) = end { span { class: s::ITEM_END, {end} } } if show_detail { span { aria_hidden: "true", ChevronRight { class: s::ITEM_DETAIL, size: 18 } } } } } }
        }
        ItemKind::Button => {
            rsx! { div { class: s::ITEM_ROW, role: "listitem", button { class: cls, r#type: "button", disabled, onclick: move |event| if let Some(onclick) = onclick { onclick.call(event); }, if let Some(start) = start { span { class: s::ITEM_START, {start} } } span { class: s::ITEM_MAIN, if let Some(overline) = overline { span { class: s::ITEM_OVERLINE, "{overline}" } } if let Some(label) = label { span { class: s::ITEM_LABEL, "{label}" } } if let Some(description) = description { span { class: s::ITEM_DESCRIPTION, "{description}" } } if let Some(children) = children { {children} } } if let Some(metadata) = metadata { span { class: s::ITEM_METADATA, "{metadata}" } } if let Some(end) = end { span { class: s::ITEM_END, {end} } } if show_detail { span { aria_hidden: "true", ChevronRight { class: s::ITEM_DETAIL, size: 18 } } } } } }
        }
        ItemKind::Static => {
            rsx! { div { class: s::ITEM_ROW, role: "listitem", div { class: cls, if let Some(start) = start { span { class: s::ITEM_START, {start} } } span { class: s::ITEM_MAIN, if let Some(overline) = overline { span { class: s::ITEM_OVERLINE, "{overline}" } } if let Some(label) = label { span { class: s::ITEM_LABEL, "{label}" } } if let Some(description) = description { span { class: s::ITEM_DESCRIPTION, "{description}" } } if let Some(children) = children { {children} } } if let Some(metadata) = metadata { span { class: s::ITEM_METADATA, "{metadata}" } } if let Some(end) = end { span { class: s::ITEM_END, {end} } } if show_detail { span { aria_hidden: "true", ChevronRight { class: s::ITEM_DETAIL, size: 18 } } } } } }
        }
    }
}
#[component]
pub fn ItemDivider(class: Option<String>, children: Element) -> Element {
    rsx! { div { class: merge_classes(s::ITEM_DIVIDER, class.as_deref()), role: "separator", {children} } }
}

#[component]
pub fn SwipeAction(
    side: SwipeSide,
    destructive: Option<bool>,
    accent: Option<bool>,
    class: Option<String>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Element,
) -> Element {
    let variant = if destructive.unwrap_or(false) {
        s::SWIPE_ACTION_DESTRUCTIVE
    } else if accent.unwrap_or(false) {
        s::SWIPE_ACTION_ACCENT
    } else {
        ""
    };
    let side = match side {
        SwipeSide::Start => "start",
        SwipeSide::End => "end",
    };
    rsx! { button { class: merge_classes(format!("{} {variant}", s::SWIPE_ACTION), class.as_deref()), r#type: "button", "data-side": side, onclick: move |event| if let Some(onclick) = onclick { onclick.call(event); }, {children} } }
}

#[component]
pub fn SwipeItem(
    start_actions: Option<Element>,
    end_actions: Option<Element>,
    action_width: Option<f64>,
    disabled: Option<bool>,
    class: Option<String>,
    on_drag: Option<Callback<SwipeState>>,
    on_full_swipe: Option<Callback<SwipeState>>,
    on_long_press: Option<Callback<()>>,
    children: Element,
) -> Element {
    let action_width = action_width.unwrap_or(DEFAULT_SWIPE_ACTION_WIDTH);
    let has_start_actions = start_actions.is_some();
    let has_end_actions = end_actions.is_some();
    let disabled = disabled.unwrap_or(false);
    let mut start_x = use_signal(|| 0.0);
    let mut start_y = use_signal(|| 0.0);
    let mut offset = use_signal(|| 0.0);
    let mut dragging = use_signal(|| false);
    let mut long_press_generation = use_signal(|| 0_u64);
    let on_drag_move = on_drag;
    let on_long_press_down = on_long_press;
    let on_drag_up = on_drag;
    let on_full_swipe_up = on_full_swipe;
    let start_actions_hidden = offset() <= 0.0;
    let end_actions_hidden = offset() >= 0.0;
    let start_actions_inert = start_actions_hidden.then(|| "".to_string());
    let end_actions_inert = end_actions_hidden.then(|| "".to_string());

    rsx! {
        div {
            class: merge_classes(s::SWIPE_ITEM, class.as_deref()),
            style: format!("--g3-swipe-offset: {}px; --g3-swipe-progress: {}; --g3-swipe-action-width: {}px;", offset(), swipe_ratio(offset(), action_width).abs().min(1.4), action_width),
            onpointerdown: move |event: PointerEvent| {
                if disabled { return; }
                dragging.set(true);
                start_x.set(event.client_coordinates().x);
                start_y.set(event.client_coordinates().y);
                let generation = long_press_generation.with_mut(|value| { *value += 1; *value });
                if let Some(on_long_press) = on_long_press_down {
                    spawn(async move {
                        dioxus_sdk_time::sleep(Duration::from_millis(LONG_PRESS_MS)).await;
                        if long_press_generation() == generation && dragging() {
                            on_long_press.call(());
                        }
                    });
                }
            },
            onpointermove: move |event: PointerEvent| {
                if !dragging() || disabled { return; }
                let dx = event.client_coordinates().x - start_x();
                let dy = event.client_coordinates().y - start_y();
                if should_cancel_long_press(dx, dy) {
                    long_press_generation.with_mut(|value| *value += 1);
                }
                let next = constrained_swipe_offset(dx, action_width, has_start_actions, has_end_actions);
                offset.set(next);
                if let Some(state) = swipe_state(next, action_width, should_full_swipe(next, action_width)) {
                    if let Some(on_drag) = on_drag_move {
                        on_drag.call(state);
                    }
                }
            },
            onpointerup: move |_| {
                if disabled { return; }
                dragging.set(false);
                long_press_generation.with_mut(|value| *value += 1);
                let current = offset();
                if should_full_swipe(current, action_width) && is_swipe_side_available(current, has_start_actions, has_end_actions) {
                    if let Some(state) = swipe_state(current, action_width, true) {
                        if let Some(on_drag) = on_drag_up { on_drag.call(state); }
                        if let Some(on_full_swipe) = on_full_swipe_up { on_full_swipe.call(state); }
                    }
                    offset.set(0.0);
                } else {
                    offset.set(settled_swipe_offset(current, action_width));
                }
            },
            onpointercancel: move |_| {
                dragging.set(false);
                long_press_generation.with_mut(|value| *value += 1);
                offset.set(0.0);
            },
            onpointerleave: move |_| {
                if !dragging() { return; }
                dragging.set(false);
                long_press_generation.with_mut(|value| *value += 1);
                offset.set(settled_swipe_offset(offset(), action_width));
            },
            if let Some(start_actions) = start_actions { div { class: format!("{} {}", s::SWIPE_ACTIONS, s::SWIPE_ACTIONS_START), aria_hidden: start_actions_hidden.to_string(), inert: start_actions_inert, {start_actions} } }
            if let Some(end_actions) = end_actions { div { class: format!("{} {}", s::SWIPE_ACTIONS, s::SWIPE_ACTIONS_END), aria_hidden: end_actions_hidden.to_string(), inert: end_actions_inert, {end_actions} } }
            div { class: s::SWIPE_CONTENT, {children} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn ListPlaygroundDemo() -> Element {
    let mut last_action = use_signal(|| "Swipe or long-press the middle row".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            div { class: "g3-list-demo-stack",
                List { inset: true,
                    ItemDivider { "Round" }
                    Item { start: rsx! { crate::Avatar { fallback: "MW" } }, label: "Matthew Weisfeld", description: "Walking 18 holes", metadata: "9:40" }
                    SwipeItem {
                        start_actions: rsx! { SwipeAction { side: SwipeSide::Start, accent: true, onclick: move |_| last_action.set("Pinned".to_string()), "Pin" } },
                        end_actions: rsx! { SwipeAction { side: SwipeSide::End, destructive: true, onclick: move |_| last_action.set("Removed".to_string()), "Delete" } },
                        on_full_swipe: move |state: SwipeState| last_action.set(format!("Full swipe: {:?}", state.side)),
                        on_long_press: move |_| last_action.set("Long press".to_string()),
                        Item { kind: ItemKind::Button, label: "Swipe actions", description: "Drag left or right", onclick: |_| {} }
                    }
                    Item { kind: ItemKind::Link("https://example.com".to_string()), label: "Link row", description: "Opens a destination" }
                    Item { kind: ItemKind::Button, label: "Button row", description: "Tap action", detail: ItemDetail::Show, onclick: move |_| last_action.set("Tapped button row".to_string()) }
                }
                crate::Badge { color: crate::StatusColor::Neutral, "{last_action()}" }
            }
        }
    }
}

crate::g3_playground! {
    name: "List",
    g3_name: "G3List / G3Item",
    description: "Mobile list rows with slots, dividers, and swipe actions.",
    demo: ListPlaygroundDemo,
    source: "src/components/list.rs",
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::G3ThemeProvider;

    fn render(app: fn() -> Element) {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
    }

    #[test]
    fn elastic_swipe_offset_slows_after_action_width() {
        assert_eq!(elastic_swipe_offset(44.0, 88.0), 44.0);
        assert_eq!(elastic_swipe_offset(188.0, 88.0), 143.0);
        assert_eq!(elastic_swipe_offset(-188.0, 88.0), -143.0);
    }

    #[test]
    fn full_swipe_requires_action_width_plus_margin() {
        assert!(!should_full_swipe(117.0, 88.0));
        assert!(should_full_swipe(118.0, 88.0));
        assert!(should_full_swipe(-118.0, 88.0));
    }

    #[test]
    fn swipe_side_follows_offset_direction() {
        assert_eq!(swipe_side(12.0), Some(SwipeSide::Start));
        assert_eq!(swipe_side(-12.0), Some(SwipeSide::End));
        assert_eq!(swipe_side(0.0), None);
    }

    #[test]
    fn swipe_ratio_follows_action_width() {
        assert_eq!(swipe_ratio(44.0, 88.0), 0.5);
        assert_eq!(swipe_ratio(-88.0, 88.0), -1.0);
    }

    #[test]
    fn long_press_cancels_after_movement_threshold() {
        assert!(!should_cancel_long_press(4.0, 4.0));
        assert!(should_cancel_long_press(9.0, 0.0));
    }

    #[test]
    fn constrained_swipe_offset_ignores_missing_action_sides() {
        assert_eq!(constrained_swipe_offset(44.0, 88.0, false, true), 0.0);
        assert_eq!(constrained_swipe_offset(-44.0, 88.0, true, false), 0.0);
        assert_eq!(constrained_swipe_offset(44.0, 88.0, true, false), 44.0);
        assert_eq!(constrained_swipe_offset(-44.0, 88.0, false, true), -44.0);
    }

    #[test]
    fn item_lines_can_defer_to_parent_list() {
        let source = include_str!("list.rs");
        let stylesheet = include_str!("../../assets/g3_ui.css");
        assert!(source.contains("lines.map(ListLines::class).unwrap_or"));
        assert!(stylesheet.contains(".g3-list[data-lines=\"full\"] .g3-item"));
        assert!(stylesheet.contains(
            ":not(.g3-item-lines-full):not(.g3-item-lines-inset):not(.g3-item-lines-none)"
        ));
    }

    #[test]
    fn swipe_actions_are_hidden_from_keyboard_when_closed() {
        let source = include_str!("list.rs");
        assert!(source.contains("start_actions_hidden"));
        assert!(source.contains("inert: start_actions_inert"));
        assert!(source.contains("aria_hidden: start_actions_hidden.to_string()"));
    }

    #[test]
    fn item_controls_keep_native_roles() {
        let source = include_str!("list.rs");
        assert!(!source.contains("a { class: cls, href, role: \"listitem\""));
        assert!(!source.contains("button { class: cls, r#type: \"button\", role: \"listitem\""));
        assert!(source.contains("div { class: s::ITEM_ROW, role: \"listitem\""));
    }

    #[test]
    fn default_item_with_onclick_promotes_to_button_semantics() {
        let source = include_str!("list.rs");
        assert!(source.contains("(ItemKind::Static, true) => ItemKind::Button"));
        assert!(source.contains("button { class: cls"));
    }

    #[test]
    fn disabled_links_drop_anchor_navigation() {
        let source = include_str!("list.rs");
        assert!(source.contains("ItemKind::Link(href) if !disabled"));
        assert!(source.contains("ItemKind::Link(_)"));
        assert!(source.contains("role: \"link\", aria_disabled"));
    }

    #[test]
    fn full_swipe_requires_available_action_side() {
        let source = include_str!("list.rs");
        assert!(
            source.contains("should_full_swipe(current, action_width) && is_swipe_side_available")
        );
        assert!(!is_swipe_side_available(44.0, false, true));
        assert!(is_swipe_side_available(-44.0, false, true));
    }

    #[test]
    fn pointer_leave_cleans_up_drag_state() {
        let source = include_str!("list.rs");
        assert!(source.contains("onpointerleave"));
        assert!(source.contains("if !dragging() { return; }"));
        assert!(source.contains("offset.set(settled_swipe_offset(offset(), action_width))"));
    }

    #[test]
    fn settled_swipe_offset_opens_after_midpoint() {
        assert_eq!(settled_swipe_offset(45.0, 88.0), 88.0);
        assert_eq!(settled_swipe_offset(-45.0, 88.0), -88.0);
        assert_eq!(settled_swipe_offset(40.0, 88.0), 0.0);
    }

    #[component]
    fn ListSmokeApp() -> Element {
        rsx! {
            G3ThemeProvider { mode: ComponentMode::Ios,
                List { inset: true,
                    Item { label: "Static", description: "Description" }
                    SwipeItem {
                        start_actions: rsx! { SwipeAction { side: SwipeSide::Start, accent: true, "Pin" } },
                        end_actions: rsx! { SwipeAction { side: SwipeSide::End, destructive: true, "Delete" } },
                        Item { label: "Swipe", description: "Drag row" }
                    }
                }
            }
        }
    }

    #[test]
    fn list_family_renders() {
        render(ListSmokeApp);
    }
}
