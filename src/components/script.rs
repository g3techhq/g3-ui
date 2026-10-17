//! Scoped DOM sessions. Scripts finish by awaiting `dioxus.recv()` and clean
//! up their listeners, observers and timers in `finally` when disposed.
use crate::state::use_synced_signal;
use dioxus::core::Task;
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use std::cell::Cell;
use std::rc::Rc;

/// Install after rendering, replace when configuration changes, and dispose
/// on unmount. A current callback handle routes reports without stale props.
#[track_caller]
pub(crate) fn use_dom_script<T: DeserializeOwned + 'static>(
    source: Option<String>,
    handler: impl FnMut(T) + 'static,
) -> Signal<Option<document::Eval>> {
    #[cfg(debug_assertions)]
    let location = std::panic::Location::caller();
    let source = use_synced_signal(source);
    let handler = use_callback(handler);
    let session = use_hook(|| Rc::new(Cell::new(None::<document::Eval>)));
    let reader = use_hook(|| Rc::new(Cell::new(None::<Task>)));
    let mut current = use_signal(|| None);
    {
        let (session, reader) = (session.clone(), reader.clone());
        use_effect(move || {
            if let Some(eval) = session.take() {
                let _ = eval.send("dispose");
            }
            if let Some(task) = reader.take() {
                task.cancel();
            }
            if let Some(source) = source() {
                // Evaluators serialize the result as JSON; an implicit
                // `undefined` produces a communication error on the web.
                let mut eval = document::eval(&format!("{source}\nreturn null;"));
                session.set(Some(eval));
                current.set(Some(eval));
                reader.set(Some(spawn(async move {
                    while let Ok(report) = eval.recv::<T>().await {
                        handler.call(report);
                    }
                })));
                spawn(async move {
                    if let Err(_error) = eval.await {
                        #[cfg(debug_assertions)]
                        dioxus::logger::tracing::warn!(
                            "g3-ui DOM session at {location} failed: {_error}"
                        );
                    }
                });
            } else {
                current.set(None);
            }
        });
    }
    use_drop(move || {
        if let Some(eval) = session.take() {
            let _ = eval.send("dispose");
        }
        if let Some(task) = reader.take() {
            task.cancel();
        }
    });
    current
}
