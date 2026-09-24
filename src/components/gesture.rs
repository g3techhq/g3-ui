//! Pointer gestures that run inside the webview.
//!
//! On native renderers every Rust event handler is a synchronous request from
//! the webview to Rust: the page cannot paint or scroll until it returns.
//! Bubbling listeners are also delegated once per event name on the root, so a
//! single Rust `onpointermove` makes every pointer move in the app wait on
//! one. On Android that is too slow for a drag to follow the finger at all.
//!
//! So a component that tracks a drag ships a script which follows the pointer
//! in the page, and Rust hears only what the gesture decided. Each script is
//! installed once per window, on a long-lived eval, and serves every instance
//! of its component; its reports name the element they concern and are routed
//! back to it here.
use super::overlay::js_string;
use dioxus::core::{provide_root_context, spawn_forever};
use dioxus::prelude::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// A gesture script: the `Symbol.for` name it registers under, and its source.
///
/// The script sends `[id, kind, [numbers...]]` for each report, and must end
/// by awaiting a promise that only its `dispose` resolves, because native
/// renderers close an eval's channel as soon as its script returns. A script
/// that takes commands from Rust exposes `command(id, what)` on
/// `window[Symbol.for(name)]`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct GestureScript {
    pub name: &'static str,
    pub source: &'static str,
}

/// One report from a gesture script.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Gesture {
    pub kind: String,
    pub values: Vec<f64>,
}

impl Gesture {
    /// The `index`th number, or 0 when the script sent fewer.
    pub fn value(&self, index: usize) -> f64 {
        self.values.get(index).copied().unwrap_or_default()
    }
}

/// Routes every script's reports in a window to the element they name.
#[derive(Clone, Default)]
struct GestureBridge {
    handlers: Rc<RefCell<HashMap<String, Callback<Gesture>>>>,
    running: Rc<RefCell<HashSet<&'static str>>>,
}

impl GestureBridge {
    fn start(&self, script: GestureScript) {
        if !self.running.borrow_mut().insert(script.name) {
            return;
        }
        let bridge = self.clone();
        spawn_forever(async move {
            let mut eval = document::eval(script.source);
            while let Ok((id, kind, values)) = eval.recv::<(String, String, Vec<f64>)>().await {
                let handler = bridge.handlers.borrow().get(&id).copied();
                if let Some(handler) = handler {
                    handler.call(Gesture { kind, values });
                }
            }
            // The page went away; the next element to mount starts it again.
            bridge.running.borrow_mut().remove(script.name);
        });
    }
}

/// Registers `handler` for reports about the element `id`, for as long as the
/// calling component lives. Call the returned callback from the element's
/// `onmounted` to make sure the script is running; it is never started during
/// server rendering, where nothing mounts.
pub(crate) fn use_gesture(
    script: GestureScript,
    id: String,
    handler: impl FnMut(Gesture) + 'static,
) -> Callback<()> {
    let bridge = use_hook(|| {
        try_consume_context::<GestureBridge>()
            .unwrap_or_else(|| provide_root_context(GestureBridge::default()))
    });
    let handler = use_callback(handler);
    use_hook({
        let (bridge, id) = (bridge.clone(), id.clone());
        move || bridge.handlers.borrow_mut().insert(id, handler)
    });
    use_drop({
        let (bridge, id) = (bridge.clone(), id);
        move || {
            bridge.handlers.borrow_mut().remove(&id);
        }
    });
    use_callback(move |()| bridge.start(script))
}

/// Sends `what` to the element `id` through its script's `command`.
pub(crate) fn gesture_command(script: GestureScript, id: &str, what: &str) {
    document::eval(&format!(
        "window[Symbol.for({})]?.command({}, {});",
        js_string(script.name),
        js_string(id),
        js_string(what),
    ));
}

/// The assertions every gesture script has to pass.
#[cfg(test)]
pub(crate) fn assert_gesture_script(script: GestureScript) {
    let source = script.source;
    assert!(
        source.contains(&format!("Symbol.for(\"{}\")", script.name)),
        "{} registers under its name",
        script.name
    );
    // Native renderers close an eval's channel when its script returns.
    assert!(
        source.trim_end().ends_with("await done;"),
        "{} stays alive",
        script.name
    );
    assert!(
        source.contains("released();"),
        "{} can be disposed",
        script.name
    );
    // Every move is handled in the page; Rust only hears outcomes.
    assert!(
        !source.contains("dioxus.recv"),
        "{} never waits on Rust",
        script.name
    );
}
