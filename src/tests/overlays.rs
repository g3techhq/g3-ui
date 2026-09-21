//! Markup contracts for sheets, dialogs, menus, and toasts.
use super::*;

#[test]
fn closed_sheets_render_nothing_until_opened() {
    fn app() -> Element {
        let open = use_signal(|| false);
        rsx! {
            BottomSheet { open, "Hidden" }
            Modal { open, title: "Hidden", "Hidden" }
        }
    }
    let html = render(app);
    assert!(!html.contains("Hidden"));
}

#[test]
fn open_bottom_sheet_is_a_labelled_modal_dialog() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            BottomSheet { open, title: "Filters", detents: vec![0.9, 0.4], "Body" }
        }
    }
    let html = render(app);
    let layer = element_with_class(&html, "g3-overlay-layer");
    assert!(layer.contains("popover=\"manual\""));
    let sheet = element_with_class(&html, "g3-sheet");
    assert!(sheet.contains("role=\"dialog\""));
    assert!(sheet.contains("aria-modal=\"true\""));
    assert!(sheet.contains("data-state=\"open\""));
    assert!(sheet.contains("tabindex=\"-1\""));
    // Detents are sorted, and the first is the starting height.
    assert!(sheet.contains("--g3-sheet-detent: 0.4;"));
    assert!(sheet.contains("data-detent=\"0\""));
    let labelledby = sheet
        .split("aria-labelledby=\"")
        .nth(1)
        .and_then(|rest| rest.split('"').next())
        .expect("labelled by its title");
    assert!(element_with_id(&html, labelledby).starts_with("<h2"));
    assert!(html.contains("data-g3-sheet-dismiss"));
    assert!(html.contains("aria-label=\"Drag to close\""));
    let backdrop = element_with_class(&html, "g3-sheet-backdrop");
    assert!(backdrop.contains("tabindex=\"-1\""));
    assert!(backdrop.contains("aria-hidden=\"true\""));
}

#[test]
fn a_sheet_without_backdrop_is_not_modal() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            BottomSheet { open, backdrop: SheetBackdrop::None, draggable: false, "Comments" }
        }
    }
    let html = render(app);
    assert!(!html.contains("g3-sheet-backdrop"));
    assert!(!html.contains("g3-sheet-handle"));
    let sheet = element_with_class(&html, "g3-sheet");
    assert!(sheet.contains("aria-modal=\"false\""));
    assert!(sheet.contains("aria-label=\"Sheet\""));
}

#[test]
fn side_sheets_use_logical_edges_and_behaviours() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            SideSheet { open, edge: SheetEdge::End, behavior: SideSheetBehavior::Push, width: "20rem", "Menu" }
        }
    }
    let html = render(app);
    let sheet = html
        .split("<div")
        .find(|tag| tag.contains("role=\"dialog\""))
        .expect("dialog");
    assert!(sheet.contains("g3-sheet-end"));
    assert!(sheet.contains("g3-sheet-push"));
    assert!(sheet.contains("--g3-side-sheet-width: 20rem;"));
    // The scrim fades; only the behaviour class reaches it, not the slide.
    let backdrop = element_with_class(&html, "g3-sheet-backdrop");
    assert!(backdrop.contains("g3-sheet-push"));
    assert!(!backdrop.contains("g3-sheet-side"));
}

#[test]
fn navigation_drawer_is_navigation_not_a_dialog() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            NavigationDrawer { open, "Links" }
        }
    }
    let html = render(app);
    let drawer = element_with_class(&html, "g3-drawer");
    assert!(drawer.contains("role=\"navigation\""));
    assert!(drawer.contains("aria-label=\"Menu\""));
    assert!(!drawer.contains("aria-modal"));
    assert!(!html.contains("g3-sheet-backdrop"));
    assert!(!html.contains("data-g3-sheet-dismiss"));
}

#[test]
fn confirm_modal_is_a_labelled_alert_dialog() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            ConfirmModal {
                open,
                title: "Delete round?",
                message: "This cannot be undone.",
                destructive: true,
                on_confirm: |_| {},
            }
        }
    }
    let html = render(app);
    let dialog = element_with_class(&html, "g3-modal");
    assert!(dialog.contains("role=\"alertdialog\""));
    assert!(dialog.contains("aria-modal=\"true\""));
    assert!(dialog.contains("aria-labelledby="));
    assert!(dialog.contains("aria-describedby="));
    assert!(html.contains(">Cancel</span>"));
    assert!(html.contains(">Confirm</span>"));
    assert!(html.contains("g3-btn-danger"));
}

#[test]
fn alert_puts_cancel_buttons_first() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            Alert {
                open,
                title: "Leave?",
                buttons: vec![AlertButton::destructive("Leave"), AlertButton::cancel("Stay")],
            }
        }
    }
    let html = render(app);
    assert!(html.find(">Stay<").unwrap() < html.find(">Leave<").unwrap());
}

#[test]
fn action_sheet_lists_choices_and_a_cancel_button() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            ActionSheet {
                open,
                title: "Round",
                buttons: vec![ActionSheetButton::new("Share"), ActionSheetButton::destructive("Delete").disabled()],
            }
        }
    }
    let html = render(app);
    assert!(html.contains("g3-action-sheet-button-danger"));
    assert!(html.contains(">Cancel</button>"));
    assert!(element_with_class(&html, "g3-action-sheet-group").contains("role=\"group\""));
}

#[test]
fn open_menus_expose_menu_items() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            Menu { open, aria_label: "Actions", trigger: rsx! { button { "More" } },
                MenuItem { "Rename" }
                MenuItem { color: Color::Danger, "Delete" }
            }
        }
    }
    let html = render(app);
    let menu = element_with_class(&html, "g3-menu");
    assert!(menu.contains("role=\"menu\""));
    assert!(menu.contains("data-state=\"open\""));
    assert_eq!(html.matches("role=\"menuitem\"").count(), 2);
    assert!(html.contains("g3-popover-backdrop"));
    let layer = element_with_class(&html, "g3-overlay-layer");
    assert!(layer.contains("popover=\"manual\""));
}

#[test]
fn toasts_keep_an_empty_live_region_while_closed() {
    fn app() -> Element {
        let closed = use_signal(|| false);
        let open = use_signal(|| true);
        rsx! {
            Toast { open: closed, message: "Hidden" }
            Toast { open, message: "Payment failed", color: Color::Danger, duration: ToastDuration::Persistent }
        }
    }
    let html = render(app);
    assert!(!html.contains("Hidden"));
    assert!(html.contains("role=\"status\""));
    let urgent = element_with_class(&html, "g3-toast-danger");
    assert!(urgent.contains("role=\"alert\""));
    assert!(urgent.contains("aria-live=\"assertive\""));
    assert!(urgent.contains("data-timer=\"none\""));
    assert!(html.contains("Payment failed"));
    assert!(html.contains("aria-label=\"Dismiss\""));
}

#[test]
fn app_wrapper_hosts_code_opened_overlays() {
    fn app() -> Element {
        rsx! {
            AppWrapper { Page {} }
        }
    }
    #[component]
    fn Page() -> Element {
        let toaster = use_toast();
        use_hook(|| {
            toaster.show("First");
            toaster.success("Second");
        });
        rsx! { "Page" }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dom.render_immediate(&mut dioxus::core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    // Toasts show one at a time.
    assert!(html.contains("First"));
    assert!(!html.contains("Second"));
    assert!(html.contains("g3-toast-host"));
}

#[test]
fn a_replacing_toast_takes_the_place_of_the_one_showing() {
    fn app() -> Element {
        rsx! {
            AppWrapper { Page {} }
        }
    }
    #[component]
    fn Page() -> Element {
        let toaster = use_toast();
        use_hook(|| {
            toaster.show("First");
            toaster.show(ToastOptions::new("Second").replace());
        });
        rsx! { "Page" }
    }
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    dom.render_immediate(&mut dioxus::core::NoOpMutations);
    let html = dioxus_ssr::render(&dom);
    assert!(!html.contains("First"));
    assert!(html.contains("Second"));
}

#[test]
fn a_low_detent_leaves_the_page_usable() {
    fn app() -> Element {
        let open = use_signal(|| true);
        rsx! {
            BottomSheet { open, detents: vec![0.2, 1.0], backdrop_detent: 1, "Results" }
        }
    }
    let html = render(app);
    let sheet = element_with_class(&html, "g3-sheet");
    assert!(sheet.contains("aria-modal=\"false\""));
    assert!(sheet.contains("data-detents=\"[0.2, 1.0]\""));
    assert!(element_with_class(&html, "g3-sheet-backdrop").contains("data-state=\"closed\""));
}
