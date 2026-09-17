//! Pull to refresh.
use super::overlay::js_string;
use crate::state::use_element_id;
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode, use_strings};
use dioxus::prelude::*;

const ELASTIC_FACTOR: f64 = 0.42;

/// The gesture runs in the page: it needs non-passive touch listeners to stop
/// the page scrolling while the user pulls. It reports a completed pull with
/// `true`.
const DRAG_SCRIPT: &str = r#"
const root = document.getElementById(__ROOT__);
const label = document.getElementById(__LABEL__);
if (root && root.dataset.g3Bound !== "true") {
    root.dataset.g3Bound = "true";
    const THRESHOLD = __THRESHOLD__;
    const ELASTIC = __ELASTIC__;
    const TEXT = { pull: __PULL__, release: __RELEASE__, refreshing: __REFRESHING__ };
    let dragging = false;
    let engaged = false;
    let startX = 0;
    let startY = 0;
    let pull = 0;
    const scrollParent = () => {
        let el = root.parentElement;
        while (el) {
            const overflow = getComputedStyle(el).overflowY;
            if ((overflow === "auto" || overflow === "scroll") && el.scrollHeight > el.clientHeight) {
                return el;
            }
            el = el.parentElement;
        }
        return null;
    };
    const available = () => root.dataset.refreshing !== "true" && root.dataset.disabled !== "true";
    const atTop = () => {
        const parent = scrollParent();
        return !parent || parent.scrollTop <= 0;
    };
    const setPull = (distance) => {
        pull = distance;
        root.style.setProperty("--g3-refresher-pull", distance + "px");
        root.style.setProperty("--g3-refresher-progress", Math.min(distance / THRESHOLD, 1.4));
        root.setAttribute("data-state", distance > 0 ? "pulling" : "idle");
        if (label) label.textContent = distance >= THRESHOLD ? TEXT.release : TEXT.pull;
    };
    // The script edits the label outside Dioxus, so it restores it itself once
    // the app's `refreshing` flag clears; a fast refresh may never re-render it.
    const settle = () => {
        if (root.dataset.refreshing === "true") {
            if (label) label.textContent = TEXT.refreshing;
            setTimeout(settle, 100);
            return;
        }
        root.setAttribute("data-state", "idle");
        root.style.setProperty("--g3-refresher-progress", "0");
        if (label) label.textContent = TEXT.pull;
    };
    const down = (x, y) => {
        if (!available() || !atTop()) return;
        dragging = true;
        engaged = false;
        startX = x;
        startY = y;
    };
    const move = (x, y, event) => {
        if (!dragging) return;
        const dy = y - startY;
        // A sideways drag belongs to whatever scrolls sideways under the
        // finger, such as a chip strip.
        if (!engaged && Math.abs(x - startX) > Math.abs(dy)) {
            dragging = false;
            return;
        }
        if (dy <= 0) {
            if (engaged) {
                engaged = false;
                setPull(0);
            }
            return;
        }
        if (!engaged) {
            if (!atTop()) return;
            engaged = true;
        }
        if (event && event.cancelable) event.preventDefault();
        setPull(Math.min(dy > THRESHOLD ? THRESHOLD + (dy - THRESHOLD) * ELASTIC : dy, THRESHOLD * 2.5));
    };
    const up = () => {
        if (!dragging) return;
        dragging = false;
        if (!engaged) return;
        engaged = false;
        if (pull >= THRESHOLD) {
            root.style.setProperty("--g3-refresher-pull", "0px");
            root.style.setProperty("--g3-refresher-progress", "1");
            root.setAttribute("data-state", "refreshing");
            if (label) label.textContent = TEXT.refreshing;
            dioxus.send(true);
            setTimeout(settle, 100);
        } else {
            setPull(0);
        }
    };
    // A pull starts on the refresher but may end anywhere: moves and releases
    // are heard on the window, so dragging past the refresher (or out of the
    // window) still lets go instead of leaving the indicator stuck.
    const listeners = new AbortController();
    const alive = () => {
        if (root.isConnected) return true;
        listeners.abort();
        return false;
    };
    const opts = { signal: listeners.signal };
    root.addEventListener("pointerdown", (e) => { if (e.pointerType === "mouse" && e.button === 0) down(e.clientX, e.clientY); }, opts);
    window.addEventListener("pointermove", (e) => { if (e.pointerType === "mouse" && alive()) move(e.clientX, e.clientY, null); }, opts);
    window.addEventListener("pointerup", (e) => { if (e.pointerType === "mouse" && alive()) up(); }, opts);
    window.addEventListener("pointercancel", () => { if (alive()) up(); }, opts);
    window.addEventListener("blur", () => { if (alive()) up(); }, opts);
    document.addEventListener("mouseleave", () => { if (alive()) up(); }, opts);
    root.addEventListener("touchstart", (e) => down(e.touches[0].clientX, e.touches[0].clientY), { passive: true, signal: listeners.signal });
    root.addEventListener("touchmove", (e) => move(e.touches[0].clientX, e.touches[0].clientY, e), { passive: false, signal: listeners.signal });
    root.addEventListener("touchend", up, opts);
    root.addEventListener("touchcancel", up, opts);
}
"#;

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
/// ```rust,ignore
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
    let strings = use_strings();
    let root_id = use_element_id("refresher", None);
    let label_id = format!("{root_id}-label");
    let threshold = threshold.unwrap_or(48.0).max(1.0);
    {
        let root_id = root_id.clone();
        let label_id = label_id.clone();
        let strings = strings.clone();
        use_effect(move || {
            let script = DRAG_SCRIPT
                .replace("__ROOT__", &js_string(&root_id))
                .replace("__LABEL__", &js_string(&label_id))
                .replace("__THRESHOLD__", &threshold.to_string())
                .replace("__ELASTIC__", &ELASTIC_FACTOR.to_string())
                .replace("__PULL__", &js_string(&strings.pull_to_refresh))
                .replace("__RELEASE__", &js_string(&strings.release_to_refresh))
                .replace("__REFRESHING__", &js_string(&strings.refreshing));
            spawn(async move {
                let mut eval = document::eval(&script);
                while let Ok(true) = eval.recv::<bool>().await {
                    on_refresh.call(());
                }
            });
        });
    }
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
