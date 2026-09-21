//! Hooks shared by the components: optional controlled state and element ids.
use dioxus::core::provide_root_context;
use dioxus::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// A component's primary value, either owned by the caller or kept locally.
///
/// When the caller passes a signal the component reads and writes it directly.
/// Otherwise the component keeps its own, seeded from `initial`.
pub(crate) fn use_controlled<T: 'static>(
    value: Option<Signal<T>>,
    initial: impl FnOnce() -> T,
) -> Signal<T> {
    let local = use_signal(initial);
    value.unwrap_or(local)
}

/// Provides `context` to descendants and keeps it current as the owner's
/// props change. `use_context_provider` alone runs once, so a group handed a
/// different `value` signal or `onchange` later would go on writing to the
/// first one. Read it with [`use_live_context`].
pub(crate) fn provide_live_context<C: Copy + PartialEq + 'static>(context: C) {
    let mut provided = use_context_provider(|| Signal::new(context));
    if *provided.peek() != context {
        provided.set(context);
    }
}

/// The current value of a context given by [`provide_live_context`].
pub(crate) fn use_live_context<C: Copy + 'static>() -> C {
    use_context::<Signal<C>>()()
}

/// Counts out the ids below, once per document. It lives in the root scope
/// rather than in a static, so each render of a page starts from the same
/// number.
#[derive(Clone)]
struct ElementIds(Rc<Cell<u64>>);

/// An element id that is unique in the document and stable for the life of
/// the component. `explicit` wins when it is non-empty.
///
/// A server renders the page and the client that hydrates it renders the
/// same tree again. The markup keeps the ids of the first render, so the
/// second one has to arrive at the same names or every script the client
/// runs looks up an element that is not there: a shelf never picks up its
/// drag handling, an infinite scroll never arms, a refresher never binds,
/// until a later client-side navigation rebuilds the page. A counter held by
/// the document counts the same way in both renders, where a process-wide
/// one carries over from whatever the server rendered before, and a scope id
/// shifts by any scope that exists on only one side.
pub(crate) fn use_element_id(prefix: &'static str, explicit: Option<String>) -> String {
    let generated = use_hook(|| {
        let ids = try_consume_context::<ElementIds>()
            .unwrap_or_else(|| provide_root_context(ElementIds(Rc::new(Cell::new(1)))));
        let number = ids.0.get();
        ids.0.set(number + 1);
        format!("g3-{prefix}-{number}")
    });
    explicit
        .filter(|id| !id.trim().is_empty())
        .unwrap_or(generated)
}

/// A signal that tracks a prop value, so effects and memos can depend on it.
/// `peek` keeps the owner from subscribing to its own signal.
pub(crate) fn use_synced_signal<T: PartialEq + Clone + 'static>(value: T) -> Signal<T> {
    let mut signal = use_signal(|| value.clone());
    if *signal.peek() != value {
        signal.set(value);
    }
    signal
}
