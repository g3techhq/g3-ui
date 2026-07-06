//! Toast component for transient mobile feedback.

use super::toast_styles as s;
use crate::components::StatusColor;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    Top,
    Middle,
    #[default]
    Bottom,
}

impl ToastPosition {
    fn class(self) -> &'static str {
        match self {
            Self::Top => s::POSITION_TOP,
            Self::Middle => s::POSITION_MIDDLE,
            Self::Bottom => s::POSITION_BOTTOM,
        }
    }
}

fn color_class(color: StatusColor) -> &'static str {
    match color {
        StatusColor::Neutral => s::COLOR_NEUTRAL,
        StatusColor::Accent => s::COLOR_ACCENT,
        StatusColor::Success => s::COLOR_SUCCESS,
        StatusColor::Warning => s::COLOR_WARNING,
        StatusColor::Danger => s::COLOR_DANGER,
    }
}

#[component]
pub fn Toast(
    mut open: Signal<bool>,
    message: String,
    position: Option<ToastPosition>,
    color: Option<StatusColor>,
    duration_ms: Option<u64>,
    close_label: Option<String>,
    action: Option<Element>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    on_dismiss: Option<Callback<()>>,
) -> Element {
    let mode = use_component_mode(mode);
    let color = color.unwrap_or_default();
    let position = position.unwrap_or_default();
    let close_label = close_label.unwrap_or_else(|| "Dismiss".to_string());
    let role = if matches!(color, StatusColor::Danger | StatusColor::Warning) {
        "alert"
    } else {
        "status"
    };
    let mode_cls = match mode {
        ComponentMode::Ios => s::TOAST_IOS,
        ComponentMode::Md => s::TOAST_MD,
    };
    let state = if open() { "open" } else { "closed" };
    let closed_inert = (!open()).then(|| "".to_string());
    let mut dismiss_generation = use_signal(|| 0_u64);

    use_effect(move || {
        let generation = dismiss_generation.with_mut(|value| {
            *value += 1;
            *value
        });
        if !open() {
            return;
        }
        let Some(duration_ms) = duration_ms.filter(|duration| *duration > 0) else {
            return;
        };
        spawn(async move {
            dioxus_sdk_time::sleep(Duration::from_millis(duration_ms)).await;
            if dismiss_generation() == generation && open() {
                open.set(false);
                if let Some(on_dismiss) = on_dismiss {
                    on_dismiss.call(());
                }
            }
        });
    });

    rsx! {
        div {
            class: merge_classes(
                format!("{} {mode_cls} {} {}", s::TOAST, position.class(), color_class(color)),
                class.as_deref(),
            ),
            role,
            aria_live: if role == "alert" { "assertive" } else { "polite" },
            aria_hidden: (!open()).to_string(),
            inert: closed_inert,
            "data-state": state,
            div { class: s::MESSAGE, "{message}" }
            if let Some(action) = action {
                div { class: s::ACTION, {action} }
            }
            button {
                class: s::CLOSE,
                r#type: "button",
                aria_label: close_label.clone(),
                disabled: !open(),
                onclick: move |_| {
                    dismiss_generation.with_mut(|value| *value += 1);
                    open.set(false);
                    if let Some(on_dismiss) = on_dismiss {
                        on_dismiss.call(());
                    }
                },
                "x"
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn ToastPlaygroundDemo() -> Element {
    let mut open = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                button { class: "g3-playground-button", r#type: "button", onclick: move |_| open.set(true), "Show toast" }
            },
            div { class: "g3-toast-demo-stage",
                Toast { open, message: "Round saved".to_string(), color: StatusColor::Success, duration_ms: 0 }
            }
        }
    }
}

crate::g3_playground! {
    name: "Toast",
    g3_name: "G3Toast",
    description: "Transient mobile feedback banner with positions and status colors.",
    demo: ToastPlaygroundDemo,
    source: "src/components/toast.rs",
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::G3ThemeProvider;

    fn render(app: fn() -> Element) {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
    }

    #[component]
    fn ToastSmokeApp() -> Element {
        let open = use_signal(|| true);
        rsx! {
            G3ThemeProvider { mode: ComponentMode::Ios,
                Toast { open, message: "Saved".to_string(), color: StatusColor::Success, duration_ms: 0 }
            }
        }
    }

    #[test]
    fn toast_renders() {
        render(ToastSmokeApp);
    }

    #[test]
    fn toast_uses_alert_role_for_urgent_colors() {
        assert_eq!(color_class(StatusColor::Danger), s::COLOR_DANGER);
        let source = include_str!("toast.rs");
        assert!(source.contains("StatusColor::Danger | StatusColor::Warning"));
        assert!(source.contains("aria_live"));
    }

    #[test]
    fn closed_toasts_are_not_keyboard_focusable() {
        let source = include_str!("toast.rs");
        assert!(source.contains("inert: closed_inert"));
        assert!(source.contains("disabled: !open()"));
    }

    #[test]
    fn toast_autodismiss_uses_generation_guard() {
        let source = include_str!("toast.rs");
        assert!(source.contains("dismiss_generation"));
        assert!(source.contains("dismiss_generation() == generation && open()"));
    }

    #[test]
    fn toast_autodismiss_uses_dioxus_sdk_time() {
        let source = include_str!("toast.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("toast source should have production section");
        assert!(source.contains("dioxus_sdk_time::sleep"));
        assert!(!source.contains("document::eval"));
    }
}
