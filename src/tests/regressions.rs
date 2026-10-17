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
    let id = mutations
        .edits
        .iter()
        .find_map(|edit| match edit {
            Mutation::NewEventListener { name, id } if name == "click" => Some(*id),
            _ => None,
        })
        .expect("picker trigger");
    click(dom, id);
    pump(dom);
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
