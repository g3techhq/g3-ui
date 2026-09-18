//! Brief, non-blocking messages.
use super::Color;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;
use dioxus_icons::lucide::X;
use std::time::Duration;

/// Where a toast appears.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ToastPosition {
    /// Near the top of the screen.
    Top,
    /// Centred, for messages that should interrupt.
    Middle,
    /// Above the bottom edge. The least obstructive.
    #[default]
    Bottom,
}

/// How long a toast stays up.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ToastDuration {
    /// Three seconds.
    #[default]
    Short,
    /// Six seconds, for longer messages or ones with an action.
    Long,
    /// A custom time.
    Custom(Duration),
    /// Until the user or the app closes it.
    Persistent,
}

impl ToastDuration {
    pub(crate) fn as_duration(self) -> Option<Duration> {
        match self {
            ToastDuration::Short => Some(Duration::from_secs(3)),
            ToastDuration::Long => Some(Duration::from_secs(6)),
            ToastDuration::Custom(duration) => Some(duration),
            ToastDuration::Persistent => None,
        }
    }
}

/// A brief message that does not block the page. Like Ionic's `ion-toast`.
///
/// To show toasts from event handlers without keeping a signal for each,
/// use [`use_toast`](crate::use_toast).
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let saved = use_signal(|| false);
/// # let undo = move |_: MouseEvent| {};
/// rsx! {
///     Toast { open: saved, message: "Round saved", color: Color::Success,
///         action: rsx! { Button { fill: ButtonFill::Clear, onclick: undo, "Undo" } } }
/// }
/// # }
/// ```
#[component]
pub fn Toast(
    /// Whether the toast is showing.
    open: Signal<bool>,
    /// The message. Use `children` for richer content.
    message: Option<String>,
    /// Where it appears. Defaults to [`ToastPosition::Bottom`].
    position: Option<ToastPosition>,
    /// Colour of the status dot and tint. Defaults to [`Color::Neutral`].
    /// [`Color::Danger`] and [`Color::Warning`] are announced assertively.
    color: Option<Color>,
    /// How long it stays up. Defaults to [`ToastDuration::Short`], or
    /// [`ToastDuration::Long`] when there is an `action`. The timer pauses
    /// while the pointer is over the toast or focus is inside it, so there is
    /// time to reach the action.
    duration: Option<ToastDuration>,
    /// A button beside the message, such as "Undo".
    action: Option<Element>,
    /// Show a close button. Defaults to `true`.
    closable: Option<bool>,
    /// Called whenever the toast closes, however it closed.
    on_dismiss: Option<EventHandler<()>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the toast.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings();
    let color = color.unwrap_or(Color::Neutral);
    let position = position.unwrap_or_default();
    let lifetime = duration
        .unwrap_or(if action.is_some() {
            ToastDuration::Long
        } else {
            ToastDuration::Short
        })
        .as_duration();
    let mut hovered = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let paused = hovered() || focused();
    let urgent = matches!(color, Color::Danger | Color::Warning);
    let is_open = open();

    // Auto-dismiss, restarted each time the toast opens. The generation keeps
    // a timer from an earlier opening from closing a later one.
    let mut generation = use_signal(|| 0_u64);
    use_effect(move || {
        let current = generation.with_mut(|g| {
            *g += 1;
            *g
        });
        if !open() {
            return;
        }
        if let Some(lifetime) = lifetime {
            spawn(async move {
                // Count only the time the toast is not paused.
                let tick = Duration::from_millis(100);
                let mut elapsed = Duration::ZERO;
                while elapsed < lifetime {
                    dioxus_sdk_time::sleep(tick).await;
                    if *generation.peek() != current || !*open.peek() {
                        return;
                    }
                    if !*hovered.peek() && !*focused.peek() {
                        elapsed += tick;
                    }
                }
                let mut open = open;
                open.set(false);
            });
        }
    });

    // Report every close, including ones the owner makes.
    let mut was_open = use_signal(|| is_open);
    use_effect(move || {
        let now = open();
        if *was_open.peek()
            && !now
            && let Some(on_dismiss) = on_dismiss
        {
            on_dismiss.call(());
        }
        if *was_open.peek() != now {
            was_open.set(now);
        }
    });

    let position_cls = match position {
        ToastPosition::Top => "g3-toast-top",
        ToastPosition::Middle => "g3-toast-middle",
        ToastPosition::Bottom => "g3-toast-bottom",
    };
    let color_cls = format!("g3-toast-{}", color.as_str());
    let cls = classes([
        "g3-toast",
        mode.pick("g3-toast-ios", "g3-toast-md"),
        position_cls,
        &color_cls,
    ]);
    let duration_ms = lifetime.map_or(0, |lifetime| lifetime.as_millis());
    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            role: if urgent { "alert" } else { "status" },
            aria_live: if urgent { "assertive" } else { "polite" },
            aria_atomic: "true",
            "data-state": if is_open { "open" } else { "closed" },
            "data-timer": if lifetime.is_some() { "active" } else { "none" },
            "data-paused": paused.then_some("true"),
            onpointerenter: move |_| hovered.set(true),
            onpointerleave: move |_| hovered.set(false),
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| focused.set(false),
            style: "--g3-toast-duration: {duration_ms}ms;",
            // Keep the live region in the page but empty while closed, so the
            // message is announced when it appears rather than when it mounts.
            if is_open {
                span { class: "g3-toast-indicator", aria_hidden: "true" }
                div { class: "g3-toast-message",
                    if let Some(message) = message {
                        "{message}"
                    }
                    {children}
                }
                if let Some(action) = action {
                    div { class: "g3-toast-action", {action} }
                }
                if closable.unwrap_or(true) {
                    button {
                        class: "g3-toast-close",
                        r#type: "button",
                        aria_label: strings.dismiss,
                        onclick: move |_| open.set(false),
                        X { class: "g3-toast-close-icon", size: 18 }
                    }
                }
                div { class: "g3-toast-timer", aria_hidden: "true" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ToastPlaygroundDemo() -> Element {
    let mut open = use_signal(|| false);
    let color = use_signal(|| Color::Success);
    let position = use_signal(|| ToastPosition::Bottom);
    let toaster = crate::use_toast();
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::SegmentGroup { value: position, aria_label: "Position",
                    crate::SegmentButton { value: ToastPosition::Top, "Top" }
                    crate::SegmentButton { value: ToastPosition::Middle, "Middle" }
                    crate::SegmentButton { value: ToastPosition::Bottom, "Bottom" }
                }
                crate::Select {
                    label: "Color",
                    value: color,
                    options: vec![
                        crate::SelectOption::new(Color::Neutral, "Neutral"),
                        crate::SelectOption::new(Color::Accent, "Accent"),
                        crate::SelectOption::new(Color::Success, "Success"),
                        crate::SelectOption::new(Color::Warning, "Warning"),
                        crate::SelectOption::new(Color::Danger, "Danger"),
                    ],
                }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                crate::Content {
                    div { class: "playground-stack",
                        crate::Button { onclick: move |_| open.set(true), "Show toast" }
                        crate::Button {
                            fill: crate::ButtonFill::Outline,
                            onclick: move |_| {
                                toaster.show(
                                    crate::ToastOptions::new("Queued from code")
                                        .color(color())
                                        .position(position()),
                                );
                            },
                            "Queue a toast with use_toast"
                        }
                    }
                }
                Toast {
                    open,
                    message: "Round saved",
                    color: color(),
                    position: position(),
                    duration: ToastDuration::Long,
                    action: rsx! {
                        crate::Button {
                            fill: crate::ButtonFill::Clear,
                            size: crate::ButtonSize::Sm,
                            onclick: move |_| open.set(false),
                            "Undo"
                        }
                    },
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Toast",
    description: "Brief messages, declared in markup or queued from code.",
    demo: ToastPlaygroundDemo,
    source: "src/components/toast.rs",
}
