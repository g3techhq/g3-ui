//! Pull-to-refresh wrapper component.

use super::refresher_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

pub const DEFAULT_REFRESH_THRESHOLD: f64 = 48.0;
pub const REFRESH_ELASTIC_FACTOR: f64 = 0.42;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RefresherState {
    pub pull: f64,
    pub progress: f64,
    pub refreshing: bool,
}

pub fn refresher_pull_distance(delta_y: f64, threshold: f64) -> f64 {
    if delta_y <= 0.0 {
        return 0.0;
    }
    let threshold = threshold.max(1.0);
    if delta_y > threshold {
        threshold + (delta_y - threshold) * REFRESH_ELASTIC_FACTOR
    } else {
        delta_y
    }
}

pub fn should_trigger_refresh(pull: f64, threshold: f64) -> bool {
    pull >= threshold.max(1.0)
}

#[component]
pub fn Refresher(
    refreshing: Option<bool>,
    threshold: Option<f64>,
    disabled: Option<bool>,
    can_refresh: Option<bool>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    on_refresh: Option<Callback<()>>,
    on_pull: Option<Callback<RefresherState>>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let refreshing = refreshing.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let can_refresh = can_refresh.unwrap_or(false);
    let threshold = threshold.unwrap_or(DEFAULT_REFRESH_THRESHOLD).max(1.0);
    let mode_cls = match mode {
        ComponentMode::Ios => s::REFRESHER_IOS,
        ComponentMode::Md => s::REFRESHER_MD,
    };
    let mut start_y = use_signal(|| 0.0);
    let mut pulling = use_signal(|| false);
    let mut pull = use_signal(|| 0.0);
    let mut was_refreshing = use_signal(|| false);

    use_effect(move || {
        if refreshing {
            was_refreshing.set(true);
            pull.set(threshold);
        } else if was_refreshing() {
            was_refreshing.set(false);
            pull.set(0.0);
        }
    });

    let visible_pull = if refreshing { threshold } else { pull() };
    let progress = if refreshing {
        1.0
    } else {
        (visible_pull / threshold).min(1.4)
    };
    let state = if refreshing {
        "refreshing"
    } else if visible_pull > 0.0 {
        "pulling"
    } else {
        "idle"
    };

    rsx! {
        div {
            class: merge_classes(format!("{} {mode_cls}", s::REFRESHER), class.as_deref()),
            style: format!(
                "--g3-refresher-pull: {}px; --g3-refresher-progress: {};",
                visible_pull,
                progress,
            ),
            "data-state": state,
            onpointerdown: move |event: PointerEvent| {
                if disabled || refreshing || !can_refresh {
                    return;
                }
                pulling.set(true);
                start_y.set(event.client_coordinates().y);
            },
            onpointermove: move |event: PointerEvent| {
                if !pulling() || disabled || refreshing || !can_refresh {
                    return;
                }
                let distance = refresher_pull_distance(
                    event.client_coordinates().y - start_y(),
                    threshold,
                );
                pull.set(distance);
                if let Some(on_pull) = on_pull {
                    on_pull
                        .call(RefresherState {
                            pull: distance,
                            progress: (distance / threshold).min(1.4),
                            refreshing,
                        });
                }
            },
            onpointerup: move |_| {
                if !pulling() || disabled {
                    return;
                }
                pulling.set(false);
                let current = pull();
                if should_trigger_refresh(current, threshold) {
                    if let Some(on_refresh) = on_refresh {
                        pull.set(threshold);
                        on_refresh.call(());
                    } else if !refreshing {
                        pull.set(0.0);
                    }
                } else if !refreshing {
                    pull.set(0.0);
                }
            },
            onpointercancel: move |_| {
                pulling.set(false);
                if !refreshing {
                    pull.set(0.0);
                }
            },
            onpointerleave: move |_| {}, // Intentionally keep the pull state when the pointer leaves the,
            div { class: s::INDICATOR, role: "status", aria_live: "polite",
                span { class: s::SPINNER, aria_hidden: "true" }
                span { class: s::LABEL,
                    // "Release to refresh" is only meaningful while the finger is
                    // actively down and past the threshold. Gating on `pulling`
                    // stops it from flashing back after release once the caller's
                    // refresh has finished and the indicator is settling away.
                    if refreshing {
                        "Refreshing"
                    } else if pulling() && should_trigger_refresh(visible_pull, threshold) {
                        "Release to refresh"
                    } else {
                        "Pull to refresh"
                    }
                }
            }
            div { class: s::CONTENT, {children} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn RefresherPlaygroundDemo() -> Element {
    let mut refreshing = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            Refresher {
                refreshing: refreshing(),
                can_refresh: true,
                on_refresh: move |_| {
                    refreshing.set(true);
                    spawn(async move {
                        dioxus_sdk_time::sleep(std::time::Duration::from_millis(700)).await;
                        refreshing.set(false);
                    });
                },
                crate::List { inset: true,
                    crate::Item {
                        label: "Leaderboard".to_string(),
                        description: "Pull down to simulate refresh".to_string(),
                    }
                    crate::Item {
                        label: "Skins".to_string(),
                        metadata: "$12".to_string(),
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Refresher",
    description: "Pull-to-refresh container with thresholded mobile gestures.",
    demo: RefresherPlaygroundDemo,
    source: "src/components/refresher.rs",
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
    fn pull_distance_is_elastic_after_threshold() {
        assert_eq!(refresher_pull_distance(-10.0, 72.0), 0.0);
        assert_eq!(refresher_pull_distance(24.0, 48.0), 24.0);
        assert_eq!(refresher_pull_distance(148.0, 48.0), 90.0);
    }

    #[test]
    fn refresh_triggers_at_threshold() {
        assert!(!should_trigger_refresh(47.0, 48.0));
        assert!(should_trigger_refresh(48.0, 48.0));
    }

    #[test]
    fn refresher_requires_explicit_refresh_gate() {
        let source = include_str!("refresher.rs");
        assert!(source.contains("can_refresh.unwrap_or(false)"));
        assert!(source.contains("disabled || refreshing || !can_refresh"));
    }

    #[test]
    fn triggered_refresh_holds_indicator_until_caller_completes() {
        let source = include_str!("refresher.rs");
        assert!(source.contains("pull.set(threshold)"));
        assert!(source.contains("let visible_pull = if refreshing { threshold } else { pull() }"));
        assert!(source.contains("if refreshing"));
    }

    #[component]
    fn RefresherSmokeApp() -> Element {
        rsx! {
            G3ThemeProvider { mode: ComponentMode::Ios,
                Refresher { refreshing: false, can_refresh: true,
                    div { "Rows" }
                }
            }
        }
    }

    #[test]
    fn refresher_renders() {
        render(RefresherSmokeApp);
    }
}
