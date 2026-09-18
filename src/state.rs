//! Hooks shared by the components: optional controlled state and element ids.
use dioxus::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

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

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// An element id that is unique in the document and stable for the life of
/// the component. `explicit` wins when it is non-empty.
pub(crate) fn use_element_id(prefix: &'static str, explicit: Option<String>) -> String {
    let generated = use_hook(|| format!("g3-{prefix}-{}", NEXT_ID.fetch_add(1, Ordering::Relaxed)));
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
