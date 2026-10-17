use dioxus::prelude::*;
use g3_ui::prelude::*;

#[component]
pub fn GettingStartedExample() -> Element {
    let mut started = use_signal(|| false);
    rsx! {
        AppWrapper {
            Header { title: "My app" }
            Content { footer_space: false,
                Card { title: "Welcome",
                    p { "Your first g3-ui screen." }
                    Button { onclick: move |_| started.set(true), "Get started" }
                    if started() { p { role: "status", "You're ready to build." } }
                }
            }
        }
    }
}

#[component]
pub fn ResponsiveLayoutsExample() -> Element {
    let mut filters = use_signal(|| false);
    let draft = use_signal(String::new);
    rsx! {
        AppWrapper {
            Header { title: "Responsive layouts" }
            Content { footer_space: false,
                LayoutScope {
                    Stack { gap: Space::Lg,
                        AdaptiveLayout {
                            compact: rsx! { Button { onclick: move |_| filters.toggle(), "Filters" } },
                            medium: rsx! { Button { onclick: move |_| filters.toggle(), "Tablet filters" } },
                            expanded: rsx! { Button { onclick: move |_| filters.toggle(), "All filters" } },
                        }
                        if filters() { Card { title: "Filters", "Shared state survives layout changes." } }
                        Grid { columns: GridColumns::Fit(18.0),
                            for title in ["Latest", "Popular", "Saved"] {
                                Card { key: "{title}", title, "Cards keep a readable minimum width." }
                            }
                        }
                        PageLayout {
                            start: rsx! { Card { title: "Activity", "Recent updates" } },
                            end: rsx! { Card { title: "Related", "More to explore" } },
                            Card { title: "Main content",
                                Input { label: "Draft survives resizing", value: draft }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn PlatformsExample() -> Element {
    let name = use_signal(String::new);
    let mut saved = use_signal(|| false);
    rsx! {
        AppWrapper {
            Header { title: "Web view ready" }
            Content { footer_space: false,
                Card { title: "Touch and keyboard input",
                    Stack { gap: Space::Md,
                        Input { label: "Your name", value: name }
                        Button { onclick: move |_| saved.set(true), "Save name" }
                        if saved() { p { role: "status", "Saved: {name}" } }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LayoutHooksExample() -> Element {
    let mut limit_width = use_signal(|| false);
    rsx! {
        AppWrapper {
            Header { title: "Available space" }
            Content { footer_space: false,
                Stack { gap: Space::Md,
                    Button { onclick: move |_| limit_width.toggle(),
                        if limit_width() { "Use all available space" } else { "Limit content to 32rem" }
                    }
                    div { style: if limit_width() { "width: 100%; max-width: 32rem;" } else { "width: 100%; max-width: none;" },
                        LayoutScope { LayoutHookValues {} }
                    }
                }
            }
        }
    }
}

#[component]
pub fn LayoutHookValues() -> Element {
    let size = use_layout_size();
    let shell = use_shell_size();
    rsx! {
        Stack { gap: Space::Md,
            Card { title: "This content area",
                p { "use_layout_size(): {size:?}" }
                p { "Measures the nearest LayoutScope." }
            }
            Card { title: "The whole app",
                p { "use_shell_size(): {shell:?}" }
                p { "Measures AppWrapper, including space outside this content area." }
            }
        }
    }
}

#[component]
pub fn OverlayHooksExample() -> Element {
    rsx! {
        AppWrapper {
            Header { title: "Overlay hooks" }
            Content { footer_space: false, OverlayHookActions {} }
        }
    }
}

#[component]
pub fn OverlayHookActions() -> Element {
    let toaster = use_toast();
    let alerts = use_alert();
    let sheets = use_action_sheet();
    let controller = use_overlay_controller();
    let mut sheet_open = use_signal(|| false);
    let mut result = use_signal(|| "No action selected.".to_string());
    rsx! {
        Stack { gap: Space::Md,
            Button { onclick: move |_| { toaster.show(ToastOptions::new("Saved")); }, "Show toast" }
            Button { onclick: move |_| async move {
                let confirmed = alerts.confirm("Save changes?", "Your changes are ready.").await;
                result.set(if confirmed { "Confirmed" } else { "Cancelled" }.into());
            }, "Ask for confirmation" }
            Button { onclick: move |_| async move {
                let choice = sheets.show(ActionSheetOptions {
                    title: Some("Choose an action".into()), message: None,
                    buttons: vec![ActionSheetButton::new("Share"), ActionSheetButton::destructive("Delete")],
                }).await;
                result.set(format!("Action result: {choice:?}"));
            }, "Show action sheet" }
            Button { onclick: move |_| sheet_open.set(true), "Open controlled sheet" }
            p { role: "status", "{result}" }
            p { "Open overlays: {controller.count()}" }
        }
        BottomSheet { open: sheet_open, title: "Controlled sheet",
            p { "Android Back can dismiss the top overlay before navigating." }
            Button { onclick: move |_| { controller.dismiss_top(); }, "Simulate Android Back" }
        }
    }
}
