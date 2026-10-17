//! Pull to refresh.
use super::overlay::js_string;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;

const ELASTIC_FACTOR: f64 = 0.42;

/// The gesture runs in the page: it needs non-passive touch listeners to stop
/// the page scrolling while the user pulls. It reports a completed pull with
/// `true`.
const DRAG_SCRIPT: &str = include_str!("refresher.js");

/// Pull-to-refresh around scrollable content. Like Ionic's `ion-refresher`.
///
/// Pulling past the threshold at the top of the scroll area calls
/// `on_refresh`. Set `refreshing` while the refresh runs; the indicator stays
/// until it is `false` again.
///
/// Usually you set [`Content`](crate::Content)'s `on_refresh` and
/// `refreshing` instead, which wraps the whole page in a refresher. Use this
/// component directly to refresh only part of a page. It grows to fill its
/// scroll area, so a pull can start anywhere below the content too.
///
/// A pull needs a pointer. Offer another way to refresh as well, such as a
/// button in the header, for keyboard and switch users.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[component] fn RoundList() -> Element { rsx! {} }
/// # async fn reload() {}
/// let mut refreshing = use_signal(|| false);
/// rsx! {
///     Refresher {
///         refreshing: refreshing(),
///         on_refresh: move |_| async move {
///             refreshing.set(true);
///             reload().await;
///             refreshing.set(false);
///         },
///         RoundList {}
///     }
/// }
/// # }
/// ```
#[component]
pub fn Refresher(
    /// Whether a refresh is running.
    refreshing: bool,
    /// Called when the user completes a pull.
    on_refresh: EventHandler<()>,
    /// Pull distance, in pixels, that triggers a refresh. Defaults to 48.
    threshold: Option<f64>,
    /// Turn pulling off.
    disabled: Option<bool>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the wrapper.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let strings = use_strings()();
    let root_id = use_element_id("refresher", None);
    let label_id = format!("{root_id}-label");
    let threshold = threshold
        .filter(|value| value.is_finite())
        .unwrap_or(48.0)
        .max(1.0);
    let script = DRAG_SCRIPT
        .replace("__ROOT__", &js_string(&root_id))
        .replace("__LABEL__", &js_string(&label_id))
        .replace("__THRESHOLD__", &threshold.to_string())
        .replace("__ELASTIC__", &ELASTIC_FACTOR.to_string())
        .replace("__PULL__", &js_string(&strings.pull_to_refresh))
        .replace("__RELEASE__", &js_string(&strings.release_to_refresh))
        .replace("__REFRESHING__", &js_string(&strings.refreshing));
    let blocked = crate::state::use_synced_signal(refreshing || disabled.unwrap_or(false));
    super::script::use_dom_script(Some(script), move |refresh: bool| {
        if refresh && !*blocked.peek() {
            on_refresh.call(());
        }
    });
    let cls = classes(["g3-refresher", mode.pick("", "g3-refresher-md")]);
    rsx! {
        div {
            id: root_id,
            class: merge_classes(cls, class.as_deref()),
            "data-state": if refreshing { "refreshing" } else { "idle" },
            "data-refreshing": refreshing.to_string(),
            "data-disabled": disabled.unwrap_or(false).to_string(),
            aria_busy: refreshing.then_some("true"),
            // Only the refresh itself is announced. The pull prompts change on
            // every movement and would be read over and over.
            span { class: "g3-sr-only", role: "status",
                if refreshing {
                    "{strings.refreshing}"
                }
            }
            div { class: "g3-refresher-indicator", aria_hidden: "true",
                span { class: "g3-refresher-spinner" }
                span { id: label_id, class: "g3-refresher-label",
                    if refreshing {
                        "{strings.refreshing}"
                    } else {
                        "{strings.pull_to_refresh}"
                    }
                }
            }
            div { class: "g3-refresher-content", {children} }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn RefresherPlaygroundDemo() -> Element {
    let mut refreshing = use_signal(|| false);
    let mut count = use_signal(|| 0);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            crate::AppWrapper { class: "g3-playground-device-app",
                crate::Header { title: "Leaderboard" }
                crate::Content {
                    refreshing: refreshing(),
                    on_refresh: move |_| {
                        refreshing.set(true);
                        spawn(async move {
                            dioxus_sdk_time::sleep(std::time::Duration::from_millis(1200)).await;
                            count += 1;
                            refreshing.set(false);
                        });
                    },
                    crate::List { variant: crate::ListVariant::Raised,
                        crate::Item { label: "Pull down anywhere to refresh" }
                        crate::Item { label: "Refreshed", metadata: "{count} times" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Refresher",
    description: "Pull-to-refresh with a threshold and elastic pull.",
    components: ["Refresher", "Content"],
    demo: RefresherPlaygroundDemo,
    source: "src/components/refresher.rs",
}
