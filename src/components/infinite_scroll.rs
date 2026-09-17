//! Loading more content as the user nears the end of a list.
use super::overlay::js_string;
use super::{Spinner, SpinnerSize};
use crate::state::{use_element_id, use_synced_signal};
use crate::theme::merge_classes;
use dioxus::prelude::*;

const OBSERVER_SCRIPT: &str = r#"
const sentinel = document.getElementById(__ID__);
if (sentinel && !sentinel.g3Observer) {
    sentinel.g3Observer = new IntersectionObserver((entries) => {
        if (entries.some((entry) => entry.isIntersecting)) dioxus.send(true);
    }, { rootMargin: "0px 0px __MARGIN__px 0px" });
    sentinel.g3Observer.observe(sentinel);
}
"#;

/// Calls `on_load` when the user scrolls near its position. Put it after the
/// last item of a list. Like Ionic's `ion-infinite-scroll`.
///
/// Set `loading` while a page loads, which shows a spinner and holds further
/// calls, and `complete` once there is nothing more to load.
///
/// ```rust,ignore
/// rsx! {
///     List { for round in rounds() { Item { label: round.name } } }
///     InfiniteScroll { loading: loading(), complete: done(), on_load: move |_| load_next_page() }
/// }
/// ```
#[component]
pub fn InfiniteScroll(
    /// Called when the user nears the end.
    on_load: EventHandler<()>,
    /// Whether a page is loading.
    loading: bool,
    /// Whether everything has loaded.
    complete: Option<bool>,
    /// How far before the end to start loading, in pixels. Defaults to 200.
    threshold: Option<u32>,
    /// Extra classes for the wrapper.
    class: Option<String>,
) -> Element {
    let id = use_element_id("infinite", None);
    let complete = complete.unwrap_or(false);
    let blocked = use_synced_signal(loading || complete);
    let margin = threshold.unwrap_or(200);
    {
        let id = id.clone();
        use_effect(move || {
            let script = OBSERVER_SCRIPT
                .replace("__ID__", &js_string(&id))
                .replace("__MARGIN__", &margin.to_string());
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(true) = eval.recv::<bool>().await {
                    if !*blocked.peek() {
                        on_load.call(());
                    }
                }
            });
        });
    }
    // An observer only reports changes. If the end is still in view after a
    // page loads, observing again reports it once more.
    {
        let id = id.clone();
        use_effect(move || {
            if !blocked() {
                let _ = document::eval(&format!(
                    "const s = document.getElementById({}); if (s && s.g3Observer) {{ s.g3Observer.unobserve(s); s.g3Observer.observe(s); }}",
                    js_string(&id)
                ));
            }
        });
    }
    rsx! {
        div {
            id,
            class: merge_classes("g3-infinite-scroll", class.as_deref()),
            aria_busy: loading.then_some("true"),
            if loading {
                Spinner { size: SpinnerSize::Md }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn InfiniteScrollPlaygroundDemo() -> Element {
    let mut count = use_signal(|| 12_usize);
    let mut loading = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            crate::List { variant: crate::ListVariant::Raised,
                for index in 1..=count() {
                    crate::Item { key: "{index}", label: format!("Round {index}") }
                }
            }
            InfiniteScroll {
                loading: loading(),
                complete: count() >= 60,
                on_load: move |_| {
                    loading.set(true);
                    spawn(async move {
                        dioxus_sdk_time::sleep(std::time::Duration::from_millis(600)).await;
                        count += 12;
                        loading.set(false);
                    });
                },
            }
        }
    }
}

crate::g3_playground! {
    name: "InfiniteScroll",
    description: "Loads more content as the user nears the end.",
    demo: InfiniteScrollPlaygroundDemo,
    source: "src/components/infinite_scroll.rs",
}
