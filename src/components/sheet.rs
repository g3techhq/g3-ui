//! What bottom sheets, side sheets, and navigation drawers share: the
//! entrance, the backdrop, scroll locking, focus handling, and the app-wide
//! count of open sheets.
use super::overlay::{
    use_lock_body_scroll, use_overlay_focus, use_overlay_registration, use_top_layer,
};
use crate::state::{use_element_id, use_synced_signal};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Dismissible sheets open right now. Global on purpose: the question it
/// answers, "is a sheet up?", is app-wide.
static OPEN_SHEETS: GlobalSignal<usize> = Signal::global(|| 0);

/// How many dismissible sheets are open in the app.
///
/// Reactive: a component that reads it re-renders as sheets open and close.
/// Navigation drawers are not counted. For Android Back, prefer
/// [`crate::use_overlay_controller`], which also covers dialogs and popovers
/// and dismisses the most recently presented surface without DOM selectors.
pub fn open_sheet_count() -> usize {
    OPEN_SHEETS()
}

/// What sits behind an open sheet.
///
/// Ionic's `showBackdrop` and `backdropDismiss` joined: the scrim is what a
/// tap outside the sheet lands on, so removing one removes the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SheetBackdrop {
    /// A dimming scrim. Tapping it closes the sheet, the page behind cannot
    /// scroll, and keyboard focus stays inside the sheet.
    #[default]
    Dismiss,
    /// No scrim, for a sheet over a page that stays in use, such as comments
    /// under a playing video. The page stays interactive and scrollable; the
    /// sheet closes from its handle, Escape, or its owner.
    None,
}

/// A side of the screen, in reading direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SheetEdge {
    /// The leading edge: left in left-to-right documents.
    #[default]
    Start,
    /// The trailing edge: right in left-to-right documents.
    End,
}

impl SheetEdge {
    fn as_str(self) -> &'static str {
        match self {
            SheetEdge::Start => "start",
            SheetEdge::End => "end",
        }
    }
}

/// How a [`SideSheet`](crate::SideSheet) moves the page beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SideSheetBehavior {
    /// The sheet slides over the page.
    #[default]
    Overlay,
    /// The sheet slides in and pushes the page aside by the same distance.
    Push,
    /// The sheet stays still and the page slides away to reveal it.
    Reveal,
}

#[derive(Clone, PartialEq)]
pub(crate) enum FrameKind {
    Bottom {
        draggable: bool,
        detents: Vec<f64>,
        detent: Signal<usize>,
        backdrop_detent: usize,
        max_height: Option<String>,
    },
    Side {
        edge: SheetEdge,
        behavior: SideSheetBehavior,
        width: Option<String>,
    },
    Drawer {
        edge: SheetEdge,
        width: Option<String>,
    },
}

const DISMISS_DISTANCE: f64 = 96.0;
const BACKDROP_EXIT_DELAY: std::time::Duration = std::time::Duration::from_millis(300);

/// Pointer and keyboard behaviour of a bottom sheet's handle. Messages back to
/// Rust: `-1` dismiss, `-2` cycle detents, `n >= 0` settle on detent `n`.
const DRAG_SCRIPT: &str = include_str!("sheet.js");

#[component]
pub(crate) fn SheetFrame(
    open: Signal<bool>,
    kind: FrameKind,
    backdrop: SheetBackdrop,
    title: Option<String>,
    aria_label: Option<String>,
    on_dismiss: Option<EventHandler<()>>,
    padding: Option<bool>,
    mode: Option<ComponentMode>,
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings()();
    let id = use_element_id("sheet", None);
    let title_id = format!("{id}-title");
    let handle_id = format!("{id}-handle");
    let layer_id = format!("{id}-layer");
    let is_drawer = matches!(kind, FrameKind::Drawer { .. });
    let is_bottom = matches!(kind, FrameKind::Bottom { .. });
    let layered = use_synced_signal(
        is_bottom
            || matches!(
                kind,
                FrameKind::Side {
                    behavior: SideSheetBehavior::Overlay,
                    ..
                }
            ),
    );
    let has_backdrop = !is_drawer && backdrop == SheetBackdrop::Dismiss;
    // A sheet resting below its backdrop detent leaves the page usable.
    let below_backdrop = match &kind {
        FrameKind::Bottom {
            detents,
            detent,
            backdrop_detent,
            ..
        } if !detents.is_empty() => detent() < *backdrop_detent,
        _ => false,
    };
    let modal = has_backdrop && !below_backdrop;
    let has_backdrop_signal = use_synced_signal(modal);
    let open_prop = use_synced_signal(open);
    let is_open = open();
    // Keep the initially rendered backdrop in the markup even below its
    // activation detent; it is inert there and retires after the fade window.
    let mut backdrop_present = use_signal(|| is_open);
    use_effect(move || {
        let open = open_prop();
        if open() && has_backdrop_signal() {
            backdrop_present.set(true);
        } else if backdrop_present() {
            spawn(async move {
                dioxus_sdk_time::sleep(BACKDROP_EXIT_DELAY).await;
                if !open() || !has_backdrop_signal() {
                    backdrop_present.set(false);
                }
            });
        }
    });

    let dismiss = use_callback(move |()| {
        if !open() {
            return;
        }
        let mut open = open;
        open.set(false);
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.call(());
        }
    });

    // A surface that mounts open is part of the initial layout (a drawer,
    // usually) and paints open without an entrance. Opening it later is a
    // presentation and animates.
    let mut ever_opened = use_signal(|| is_open);
    let mut has_been_closed = use_signal(|| !is_open);
    // Cleared when the entrance finishes, so a detent change while open
    // animates its height again.
    let mut entered = use_signal(|| false);
    use_effect(move || {
        let open = open_prop();
        if open() {
            ever_opened.set(true);
        } else {
            has_been_closed.set(true);
            entered.set(false);
        }
    });

    let locked = use_memo(move || open_prop()() && has_backdrop_signal());
    use_lock_body_scroll(locked.into());
    let layer_open = use_memo(move || open_prop()() && layered());
    use_top_layer(
        layer_open.into(),
        layer_id.clone(),
        std::time::Duration::from_millis(500),
    );

    if !is_drawer {
        use_overlay_focus(open.into(), id.clone(), modal, dismiss);
        use_open_count(open);
    }
    let registered = use_memo(move || open_prop()() && !is_drawer);
    use_overlay_registration(registered.into(), id.clone(), dismiss);

    let (draggable, detent_signal, count) = match &kind {
        FrameKind::Bottom {
            draggable: true,
            detents,
            detent,
            ..
        } => (true, Some(*detent), detents.len()),
        _ => (false, None, 0),
    };
    let detent_count = use_synced_signal(count);
    let script = (draggable && is_open).then(|| {
        DRAG_SCRIPT
            .replace("__DIALOG__", &super::overlay::js_string(&id))
            .replace("__HANDLE__", &super::overlay::js_string(&handle_id))
            .replace("__DISMISS__", &DISMISS_DISTANCE.to_string())
    });
    super::script::use_dom_script(script, move |message: i64| match (message, detent_signal) {
        (-1, _) => dismiss.call(()),
        (-2, Some(mut detent)) => {
            let count = detent_count.peek().max(1);
            let next = (*detent.peek() + 1) % count;
            detent.set(next);
        }
        (index, Some(mut detent)) if index >= 0 => {
            detent.set(index as usize);
        }
        _ => {}
    });

    if !is_open && !ever_opened() {
        return rsx! {};
    }

    let state = if is_open { "open" } else { "closed" };
    let enter_cls = if is_open && has_been_closed() && !entered() {
        "g3-sheet-enter"
    } else {
        ""
    };
    let (kind_cls, style, detent_attr, handle) = match &kind {
        FrameKind::Bottom {
            draggable,
            detents,
            detent,
            max_height,
            ..
        } => {
            let mut style = String::new();
            if let Some(max_height) = max_height {
                style.push_str(&format!("--g3-sheet-max-height: {max_height};"));
            }
            let detent_attr = if detents.is_empty() {
                None
            } else {
                let index = detent().min(detents.len() - 1);
                style.push_str(&format!("--g3-sheet-detent: {};", detents[index]));
                Some(index.to_string())
            };
            (
                "g3-sheet g3-sheet-bottom".to_string(),
                style,
                detent_attr,
                *draggable,
            )
        }
        FrameKind::Side {
            edge,
            behavior,
            width,
        } => {
            let behavior_cls = behavior_class(*behavior);
            (
                format!(
                    "g3-sheet g3-sheet-side g3-sheet-{} {behavior_cls}",
                    edge.as_str()
                ),
                width_style(width),
                None,
                false,
            )
        }
        FrameKind::Drawer { edge, width } => (
            format!("g3-drawer g3-drawer-{}", edge.as_str()),
            width_style(width),
            None,
            false,
        ),
    };
    // A custom width has to reach the page too, which moves or narrows by
    // it: the page is a sibling, so the width is set on the app shell that
    // holds this surface.
    let width_rule = match &kind {
        FrameKind::Side {
            width: Some(width), ..
        }
        | FrameKind::Drawer {
            width: Some(width), ..
        } => Some(
            format!(".g3-app:has(> #{id}) {{ --g3-side-sheet-width: {width}; }}")
                .replace("</", "<\\/"),
        ),
        _ => None,
    };
    // The scrim only needs the behaviour: the side-sheet classes would give
    // it the sheet's slide instead of a fade.
    let behavior_cls = match &kind {
        FrameKind::Side { behavior, .. } => behavior_class(*behavior),
        _ => "",
    };
    let surface_cls = classes([
        kind_cls.as_str(),
        if is_drawer {
            ""
        } else {
            mode.pick("g3-sheet-ios", "g3-sheet-md")
        },
        enter_cls,
    ]);
    let label = aria_label.or_else(|| {
        title.is_none().then(|| {
            if is_drawer {
                strings.navigation_drawer.clone()
            } else {
                strings.sheet.clone()
            }
        })
    });
    let labelledby = title.as_ref().map(|_| title_id.clone());

    let contents = rsx! {
        if has_backdrop && (modal && is_open || backdrop_present()) {
            button {
                r#type: "button",
                class: classes(["g3-sheet-backdrop", behavior_cls, enter_cls]),
                "data-state": if modal && is_open { "open" } else { "closed" },
                // The scrim is a pointer target only. Keyboard users close the
                // sheet with Escape, so it stays out of the tab order.
                tabindex: "-1",
                aria_hidden: "true",
                // Dismiss on click, not pointerdown: closing on the press
                // removes the scrim before the press ends, and the click lands
                // on whatever was underneath.
                onclick: move |_| dismiss.call(()),
            }
        }
        div {
            id: id.clone(),
            class: merge_classes(surface_cls, class.as_deref()),
            style,
            role: if is_drawer { "navigation" } else { "dialog" },
            aria_modal: (!is_drawer).then(|| modal.to_string()),
            aria_label: label,
            aria_labelledby: labelledby,
            aria_hidden: (!is_open).then_some("true"),
            inert: (!is_open).then_some(true),
            tabindex: (!is_drawer).then_some("-1"),
            "data-state": state,
            "data-detent": detent_attr,
            onanimationend: move |event| {
                // Animations inside the sheet bubble here too.
                if event.data().animation_name().starts_with("g3-sheet-enter") && !*entered.peek() {
                    entered.set(true);
                }
            },
            "data-detents": match &kind {
                FrameKind::Bottom { detents, .. } if !detents.is_empty() => Some(format!("{detents:?}")),
                _ => None,
            },
            if let Some(css) = width_rule {
                style { dangerous_inner_html: css }
            }
            if handle && is_bottom {
                button {
                    id: handle_id,
                    r#type: "button",
                    class: "g3-sheet-handle",
                    aria_label: strings.sheet_handle,
                    span { class: "g3-sheet-handle-bar" }
                }
            }
            // A close control for app-level dismissal such as Android Back.
            // Hidden, and never in the tab order.
            if is_open && !is_drawer {
                button {
                    r#type: "button",
                    class: "g3-sheet-dismiss",
                    tabindex: "-1",
                    aria_hidden: "true",
                    "data-g3-sheet-dismiss": "",
                    onclick: move |_| dismiss.call(()),
                    "{strings.close}"
                }
            }
            if let Some(title) = title {
                div { class: "g3-sheet-header",
                    h2 { id: title_id, class: "g3-sheet-title", "{title}" }
                }
            }
            div {
                class: "g3-sheet-content",
                "data-padding": padding.unwrap_or(true).to_string(),
                {children}
            }
        }
    };
    rsx! {
        if layered() {
            div {
                id: layer_id,
                class: "g3-overlay-layer",
                popover: "manual",
                {contents}
            }
        } else {
            {contents}
        }
    }
}

fn behavior_class(behavior: SideSheetBehavior) -> &'static str {
    match behavior {
        SideSheetBehavior::Overlay => "",
        SideSheetBehavior::Push => "g3-sheet-push",
        SideSheetBehavior::Reveal => "g3-sheet-reveal",
    }
}

fn width_style(width: &Option<String>) -> String {
    width
        .as_ref()
        .map(|width| format!("--g3-side-sheet-width: {width};"))
        .unwrap_or_default()
}

/// Keep [`OPEN_SHEETS`] in step with this sheet, including when it unmounts
/// while open because its page navigated away.
fn use_open_count(open: Signal<bool>) {
    let open = use_synced_signal(open);
    let counted = use_hook(|| Rc::new(Cell::new(false)));
    {
        let counted = counted.clone();
        use_effect(move || {
            let is_open = open()();
            if counted.get() != is_open {
                counted.set(is_open);
                let mut count = OPEN_SHEETS.write();
                *count = if is_open {
                    *count + 1
                } else {
                    count.saturating_sub(1)
                };
            }
        });
    }
    use_drop(move || {
        if counted.get() {
            let mut count = OPEN_SHEETS.write();
            *count = count.saturating_sub(1);
        }
    });
}
