//! Mounted state and markup regressions from the October component audit.
use super::*;
use dioxus::core::{ElementId, Mutation, NoOpMutations};
use std::any::Any;
use std::rc::Rc;
use std::time::Duration;

fn pump(dom: &mut VirtualDom) {
    for _ in 0..5 {
        dom.process_events();
        dom.render_immediate(&mut NoOpMutations);
    }
}

fn open_picker(dom: &mut VirtualDom) {
    dioxus::html::set_event_converter(Box::new(dioxus::html::SerializedHtmlEventConverter));
    let mutations = dom.rebuild_to_vec();
    let id = listener_target(&mutations.edits, "click").expect("picker trigger");
    click(dom, id);
    pump(dom);
}

/// The element the first `name` listener is attached to.
///
/// Edits are a stack machine: a listener carries no id and lands on the node
/// on top of the stack, so this replays the stack (as the interpreter's
/// `core.ts` does), tracking only the id registered to each entry. Taking the
/// last `PushId` instead would pick the wrong element after a `Child` or
/// `CreateElement` moved the top.
fn listener_target(edits: &[Mutation], name: &str) -> Option<ElementId> {
    fn pop(stack: &mut Vec<Option<ElementId>>, n: usize) {
        stack.truncate(stack.len().saturating_sub(n));
    }
    let mut stack = Vec::new();
    for edit in edits {
        match edit {
            Mutation::PushId { id } => stack.push(Some(*id)),
            Mutation::SetId { id } => *stack.last_mut()? = Some(*id),
            Mutation::Child { .. } | Mutation::Clone => *stack.last_mut()? = None,
            Mutation::CreateElement { .. } | Mutation::CreateText { .. } => stack.push(None),
            Mutation::Pop | Mutation::Remove => pop(&mut stack, 1),
            Mutation::AppendChildren { m }
            | Mutation::InsertAfter { m }
            | Mutation::InsertBefore { m } => pop(&mut stack, *m),
            // The replaced target leaves the stack with its replacements.
            Mutation::ReplaceWith { m } => pop(&mut stack, m + 1),
            Mutation::NewEventListener { name: listener } if listener == name => {
                return *stack.last()?;
            }
            _ => {}
        }
    }
    None
}

fn click(dom: &VirtualDom, id: ElementId) {
    let event = Event::new(
        Rc::new(dioxus::html::PlatformEventData::new(Box::<
            dioxus::html::SerializedMouseData,
        >::default())) as Rc<dyn Any>,
        true,
    );
    dom.runtime().handle_event("click", event, id);
}

#[test]
fn tabs_follow_replacement_signals_and_propagate_mode() {
    static SECOND: GlobalSignal<bool> = Signal::global(|| false);

    fn app() -> Element {
        let first = use_signal(|| 1_i32);
        let second = use_signal(|| 2_i32);
        rsx! {
            Tabs { value: if SECOND() { second } else { first }, mode: ComponentMode::Ios, animated: false,
                TabList { Tab { value: 1_i32, "One" } Tab { value: 2_i32, "Two" } }
                TabPanel { value: 1_i32, "Panel one" }
                TabPanel { value: 2_i32, "Panel two" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    assert!(dioxus_ssr::render(&dom).contains("Panel one"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *SECOND.write() = true);
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("Panel two"));
    assert!(!html.contains("Panel one"));
    assert!(html.contains("g3-segment-ios"));
    assert!(!html.contains("g3-segment-md"));
}

#[test]
fn retained_tab_panels_are_inert_when_not_selected() {
    fn app() -> Element {
        let value = use_signal(|| 1_i32);
        rsx! { Tabs { value,
            TabPanel { value: 2_i32, keep_mounted: true, Button { "Hidden action" } }
        } }
    }
    let html = render(app);
    let panel = element_with_class(&html, "g3-tab-panel");
    assert!(panel.contains("inert"));
    assert!(panel.contains("aria-hidden=\"true\""));
}

#[test]
fn readonly_inputs_do_not_offer_clear_actions() {
    fn app() -> Element {
        let value = use_signal(|| "Read only".to_string());
        rsx! { Input { value, readonly: true, clearable: true } }
    }
    assert!(!render(app).contains("g3-field-clear"));
}

#[test]
fn unavailable_calendar_days_remain_keyboard_focusable() {
    fn app() -> Element {
        rsx! { Calendar { reference_date: CalendarDate::new(2026, 9, 19).unwrap(), is_date_disabled: |_| true } }
    }
    let html = render(app);
    let today = element_with_class(&html, "g3-calendar-today");
    assert!(today.contains("tabindex=\"0\""));
    assert!(today.contains("aria-disabled=\"true\""));
    assert!(!today.contains(" disabled"));
}

#[test]
fn disabled_date_cannot_be_confirmed_in_calendar_or_wheels() {
    fn dialog() -> Element {
        rsx! { DatePicker { style: PickerStyle::Dialog, reference_date: CalendarDate::new(2026, 9, 19).unwrap(), is_date_disabled: |_| true } }
    }

    fn wheels() -> Element {
        rsx! { DatePicker { style: PickerStyle::Wheels, reference_date: CalendarDate::new(2026, 9, 19).unwrap(), is_date_disabled: |_| true } }
    }
    for app in [dialog as fn() -> Element, wheels] {
        let mut dom = VirtualDom::new(app);
        open_picker(&mut dom);
        let html = dioxus_ssr::render(&dom);
        let action = html
            .split(if html.contains("g3-wheel") {
                ">Done<"
            } else {
                ">Confirm<"
            })
            .next()
            .expect("confirmation");
        let tag = &action[action.rfind("<button").expect("confirmation button")..];
        assert!(tag.contains("disabled"), "{html}");
        if html.contains("g3-wheel") {
            assert!(html.contains("data-disabled=\"true\""));
        }
    }
}

#[test]
fn immediate_time_confirmation_matches_the_wheel_label() {
    fn app() -> Element {
        let time = use_signal(|| TimeOfDay::new(14, 59));
        rsx! { TimePicker { value: time, minute_step: 5, style: PickerStyle::Wheels, hour_cycle: HourCycle::H24 } }
    }
    let mut dom = VirtualDom::new(app);
    open_picker(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(html.contains("aria-valuetext=\"55\""), "{html}");
}

#[derive(Clone, Copy)]
struct Open(Signal<bool>);

#[test]
fn reused_alerts_reset_from_current_input_props() {
    static INITIAL: GlobalSignal<String> = Signal::global(|| "First".to_string());

    fn app() -> Element {
        let open = use_signal(|| true);
        use_hook(|| dioxus::core::provide_root_context(Open(open)));
        rsx! { Alert { open, title: "Prompt", buttons: vec![AlertButton::new("Save")],
            input: AlertInput { label: "Name".into(), value: INITIAL(), ..AlertInput::default() }
        } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("value=\"First\""));
    dom.runtime().in_scope(ScopeId::ROOT, || {
        try_consume_context::<Open>().unwrap().0.set(false);
        *INITIAL.write() = "Second".into();
    });
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        try_consume_context::<Open>().unwrap().0.set(true)
    });
    pump(&mut dom);
    assert!(dioxus_ssr::render(&dom).contains("value=\"Second\""));
}

#[tokio::test(start_paused = true)]
async fn toast_duration_updates_cancel_and_start_timers() {
    static DURATION: GlobalSignal<ToastDuration> = Signal::global(|| ToastDuration::Short);

    fn app() -> Element {
        let open = use_signal(|| true);
        use_hook(|| dioxus::core::provide_root_context(Open(open)));
        rsx! { Toast { open, duration: DURATION(), message: "Timer" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        *DURATION.write() = ToastDuration::Persistent
    });
    pump(&mut dom);
    for _ in 0..40 {
        tokio::time::advance(Duration::from_millis(100)).await;
        pump(&mut dom);
    }
    assert!(
        dom.runtime()
            .in_scope(ScopeId::ROOT, || try_consume_context::<Open>().unwrap().0())
    );
    dom.runtime().in_scope(ScopeId::ROOT, || {
        *DURATION.write() = ToastDuration::Custom(Duration::from_millis(200))
    });
    pump(&mut dom);
    for _ in 0..3 {
        tokio::time::advance(Duration::from_millis(100)).await;
        pump(&mut dom);
    }
    assert!(
        !dom.runtime()
            .in_scope(ScopeId::ROOT, || try_consume_context::<Open>().unwrap().0())
    );
}

#[derive(Clone, Copy)]
struct StackControls {
    sheet: Signal<bool>,
    modal: Signal<bool>,
    controller: OverlayController,
}

#[test]
fn back_dismisses_only_the_latest_surface_and_unmount_cleans_the_stack() {
    static PRESENT: GlobalSignal<bool> = Signal::global(|| true);

    fn app() -> Element {
        let sheet = use_signal(|| false);
        let modal = use_signal(|| false);
        let controller = use_overlay_controller();
        use_hook(|| {
            dioxus::core::provide_root_context(StackControls {
                sheet,
                modal,
                controller,
            })
        });
        rsx! {
            if PRESENT() {
                BottomSheet { open: sheet, title: "Sheet" }
                Modal { open: modal, title: "Modal" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        try_consume_context::<StackControls>()
            .unwrap()
            .sheet
            .set(true);
    });
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        try_consume_context::<StackControls>()
            .unwrap()
            .modal
            .set(true);
    });
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        let controls = try_consume_context::<StackControls>().unwrap();
        assert_eq!(controls.controller.count(), 2);
        assert!(controls.controller.dismiss_top());
        assert!(!(controls.modal)());
        assert!((controls.sheet)());
    });
    pump(&mut dom);
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *PRESENT.write() = false);
    pump(&mut dom);
    dom.runtime().in_scope(ScopeId::ROOT, || {
        assert_eq!(
            try_consume_context::<StackControls>()
                .unwrap()
                .controller
                .count(),
            0
        )
    });
}

#[test]
fn modal_and_toast_wrappers_share_the_top_layer_contract() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! { Modal { open, title: "Dialog" } Toast { open, message: "Notice" } }
    }
    let html = render(app);
    assert_eq!(html.matches("popover=\"manual\"").count(), 2);
}

#[test]
fn overlay_side_sheets_use_the_top_layer_and_layout_sheets_keep_their_sibling_contract() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            SideSheet { open, title: "Overlay" }
            SideSheet { open, behavior: SideSheetBehavior::Push, title: "Push" }
            SideSheet { open, behavior: SideSheetBehavior::Reveal, title: "Reveal" }
        }
    }
    let html = render(app);
    assert_eq!(html.matches("popover=\"manual\"").count(), 1);
    assert!(html.contains("g3-sheet-push"));
    assert!(html.contains("g3-sheet-reveal"));
}

#[test]
fn rating_and_date_formats_follow_localized_templates() {
    fn app() -> Element {
        let value = use_signal(|| 3.5);
        let date = use_signal(|| CalendarDate::new(2026, 9, 19));
        rsx! { ThemeProvider { strings: Strings {
            rating_value: "{value} / {max}".into(), date_short: "{day} {month} {year}".into(), ..Strings::default()
        }, Rating { value, readonly: true } DatePicker { value: date } } }
    }
    let html = render(app);
    assert!(html.contains("aria-label=\"3.5 / 5\""));
    assert!(html.contains("19 Sep 2026"));
}

#[test]
fn a_bottom_bar_marks_its_own_app_shell_for_toasts() {
    static SHOW_BAR: GlobalSignal<bool> = Signal::global(|| true);

    fn app() -> Element {
        rsx! {
            AppWrapper { class: "outer",
                if SHOW_BAR() {
                    NavBar { NavItem { label: "Rounds", onclick: |_| {} } }
                }
                AppWrapper { class: "inner", mode: ComponentMode::Ios,
                    AdaptiveNav { NavItem { label: "Book", onclick: |_| {} } }
                }
                AppWrapper { class: "bare" }
            }
        }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    // A NavBar is a bar at every width; an AdaptiveNav only when compact.
    // Each shell counts only the bars inside it, not those of nested apps.
    assert!(element_with_class(&html, "outer").contains("data-g3-bar=\"always\""));
    assert!(element_with_class(&html, "inner").contains("data-g3-bar=\"compact\""));
    assert!(!element_with_class(&html, "bare").contains("data-g3-bar"));
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *SHOW_BAR.write() = false);
    pump(&mut dom);
    let html = dioxus_ssr::render(&dom);
    assert!(!element_with_class(&html, "outer").contains("data-g3-bar"));
}

/// On native renderers every Rust event handler is a synchronous request
/// that blocks the webview, so per-element `load`, `error` and `mounted`
/// handlers stall page transitions. Images and gesture components keep
/// those in the page.
#[test]
fn images_and_gesture_components_register_no_blocking_events() {
    fn app() -> Element {
        let tab = use_signal(|| 1_i32);
        let stars = use_signal(|| 3.0);
        rsx! {
            Img { src: "/a.png", alt: "A", fallback: rsx! { span { "Missing" } } }
            Img { src: "/b.png", alt: "B", eager: true }
            SwipeItem { end_actions: rsx! { SwipeAction { "Delete" } }, Item { label: "Row" } }
            ReorderList { on_reorder: |_| {},
                List {
                    ReorderItem { index: 0, Item { label: "One" } }
                    ReorderItem { index: 1, Item { label: "Two" } }
                }
            }
            Tabs { value: tab, swipe: true,
                TabList { Tab { value: 1_i32, "One" } }
                TabPanel { value: 1_i32, "Panel" }
            }
            Rating { label: "Your rating", value: stars }
        }
    }
    let mut dom = VirtualDom::new(app);
    let mutations = dom.rebuild_to_vec();
    let listeners: Vec<&str> = mutations
        .edits
        .iter()
        .filter_map(|edit| match edit {
            Mutation::NewEventListener { name, .. } => Some(name.as_str()),
            _ => None,
        })
        .collect();
    for blocking in ["load", "error", "mounted"] {
        assert!(
            !listeners.contains(&blocking),
            "a `{blocking}` handler was registered: {listeners:?}"
        );
    }
}

#[test]
fn an_img_starts_loading_and_carries_its_fallback_hidden() {
    fn app() -> Element {
        rsx! {
            Img { src: "/a.png", alt: "A", aspect_ratio: "4 / 3",
                fallback: rsx! { span { "Missing" } },
            }
            Img { src: "/b.png", alt: "" }
        }
    }
    let html = render(app);
    let first = element_with_class(&html, "g3-img");
    assert!(first.contains("data-state=\"loading\""));
    assert!(first.contains("aspect-ratio: 4 / 3;"));
    assert!(html.contains("loading=\"lazy\""));
    // The fallback waits beside the image; the stylesheet shows it on error.
    assert!(html.contains("<span class=\"g3-img-fallback\"><span>Missing</span></span>"));
    assert_eq!(html.matches("g3-img-fallback").count(), 1);
    assert_eq!(html.matches("<img").count(), 2);
}

#[test]
fn an_img_with_a_new_src_starts_loading_again() {
    static SRC: GlobalSignal<&'static str> = Signal::global(|| "/a.png");

    fn app() -> Element {
        rsx! { Img { src: SRC(), alt: "A" } }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dom.runtime()
        .in_scope(ScopeId::ROOT, || *SRC.write() = "/b.png");
    dom.mark_dirty(ScopeId::ROOT);
    let mut edits = Vec::new();
    for _ in 0..5 {
        dom.process_events();
        edits.extend(dom.render_immediate_to_vec().edits);
    }
    // The wrapper is replaced rather than patched, so the `data-state` the
    // page set for the old image does not carry over. The fork's edits write
    // a replacement as a new node from the template and a `Remove` of the old
    // one (or `ReplaceWith`); patching would only set `src` on the old one.
    let rebuilt = edits.iter().any(|edit| matches!(edit, Mutation::Clone));
    let removed = edits
        .iter()
        .any(|edit| matches!(edit, Mutation::Remove | Mutation::ReplaceWith { .. }));
    assert!(rebuilt && removed, "{edits:?}");
    assert!(dioxus_ssr::render(&dom).contains("src=\"/b.png\""));
}
