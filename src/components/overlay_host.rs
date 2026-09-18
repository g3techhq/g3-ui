//! Overlays opened from code: toasts, alerts, and action sheets.
use super::{
    ActionSheet, ActionSheetButton, Alert, AlertButton, AlertInput, AlertResult, Color, Toast,
    ToastDuration, ToastPosition,
};
use crate::theme::use_strings;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll, Waker};
use std::time::Duration;

/// Time an overlay needs to animate out before it leaves the queue.
const EXIT_DELAY: Duration = Duration::from_millis(300);

struct Slot<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

/// A value that arrives later, when the user answers an overlay.
struct Reply<T>(Rc<RefCell<Slot<T>>>);

impl<T> Future for Reply<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let mut slot = self.0.borrow_mut();
        match slot.value.take() {
            Some(value) => Poll::Ready(value),
            None => {
                slot.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

/// The sending half of a [`Reply`].
struct Responder<T>(Rc<RefCell<Slot<T>>>);

impl<T> Responder<T> {
    fn send(&self, value: T) {
        let mut slot = self.0.borrow_mut();
        slot.value = Some(value);
        if let Some(waker) = slot.waker.take() {
            waker.wake();
        }
    }
}

fn reply<T>() -> (Responder<T>, Reply<T>) {
    let slot = Rc::new(RefCell::new(Slot {
        value: None,
        waker: None,
    }));
    (Responder(slot.clone()), Reply(slot))
}

/// A toast to show with [`Toaster::show`].
#[derive(Clone, Debug, PartialEq)]
pub struct ToastOptions {
    /// The message.
    pub message: String,
    /// Colour. Defaults to [`Color::Neutral`].
    pub color: Color,
    /// Where it appears.
    pub position: ToastPosition,
    /// How long it stays up.
    pub duration: ToastDuration,
    /// Take the place of the toast showing, and any waiting, rather than
    /// queueing behind them.
    pub replace: bool,
}

impl ToastOptions {
    /// A neutral toast at the bottom for the default time.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            color: Color::Neutral,
            position: ToastPosition::default(),
            duration: ToastDuration::default(),
            replace: false,
        }
    }

    /// Set the colour.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the position.
    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    /// Set how long it stays up.
    pub fn duration(mut self, duration: ToastDuration) -> Self {
        self.duration = duration;
        self
    }

    /// Show it at once, in place of the toast showing and any waiting. Use it
    /// for feedback on an action that can be repeated quickly, such as saving
    /// several items in a row: queued, the confirmations would trail behind
    /// the actions long after they happened.
    pub fn replace(mut self) -> Self {
        self.replace = true;
        self
    }
}

impl<S: Into<String>> From<S> for ToastOptions {
    fn from(message: S) -> Self {
        Self::new(message)
    }
}

/// Identifies a toast shown with [`Toaster::show`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToastId(u64);

/// An alert to show with [`Alerts::show`].
#[derive(Clone, Debug, PartialEq)]
pub struct AlertOptions {
    /// The question or headline.
    pub title: String,
    /// Supporting text.
    pub message: Option<String>,
    /// The buttons.
    pub buttons: Vec<AlertButton>,
    /// A text field, for prompts.
    pub input: Option<AlertInput>,
}

/// An action sheet to show with [`ActionSheets::show`].
#[derive(Clone, Debug, PartialEq)]
pub struct ActionSheetOptions {
    /// Heading.
    pub title: Option<String>,
    /// Supporting text.
    pub message: Option<String>,
    /// The choices.
    pub buttons: Vec<ActionSheetButton>,
}

struct Entry<O, R> {
    id: u64,
    options: O,
    responder: Option<Rc<Responder<R>>>,
}

impl<O: Clone, R> Clone for Entry<O, R> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            options: self.options.clone(),
            responder: self.responder.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct OverlayQueues {
    toasts: Signal<Vec<Entry<ToastOptions, ()>>>,
    alerts: Signal<Vec<Entry<AlertOptions, AlertResult>>>,
    sheets: Signal<Vec<Entry<ActionSheetOptions, Option<usize>>>>,
    next_id: Signal<u64>,
}

impl OverlayQueues {
    fn next(&self) -> u64 {
        let mut next_id = self.next_id;
        next_id.with_mut(|id| {
            *id += 1;
            *id
        })
    }
}

/// Create the queues for an [`AppWrapper`](crate::AppWrapper) and make them
/// available to its children.
pub(crate) fn use_provide_overlay_queues() -> OverlayQueues {
    use_context_provider(|| OverlayQueues {
        toasts: Signal::new(Vec::new()),
        alerts: Signal::new(Vec::new()),
        sheets: Signal::new(Vec::new()),
        next_id: Signal::new(0),
    })
}

fn use_queues() -> OverlayQueues {
    use_hook(|| {
        try_consume_context::<OverlayQueues>().unwrap_or_else(|| {
            dioxus::logger::tracing::warn!(
                "g3-ui: overlays opened from code need an AppWrapper above the calling component"
            );
            OverlayQueues {
                toasts: Signal::new(Vec::new()),
                alerts: Signal::new(Vec::new()),
                sheets: Signal::new(Vec::new()),
                next_id: Signal::new(0),
            }
        })
    })
}

/// Shows toasts from code. Get one with [`use_toast`].
///
/// Toasts queue up and show one at a time.
#[derive(Clone, Copy)]
pub struct Toaster {
    queues: OverlayQueues,
}

impl Toaster {
    /// Queue a toast.
    pub fn show(&self, options: impl Into<ToastOptions>) -> ToastId {
        let id = self.queues.next();
        let options = options.into();
        let mut toasts = self.queues.toasts;
        let mut queue = toasts.write();
        if options.replace {
            queue.clear();
        }
        queue.push(Entry {
            id,
            options,
            responder: None,
        });
        ToastId(id)
    }

    /// Queue a success toast.
    pub fn success(&self, message: impl Into<String>) -> ToastId {
        self.show(ToastOptions::new(message).color(Color::Success))
    }

    /// Queue an error toast.
    pub fn error(&self, message: impl Into<String>) -> ToastId {
        self.show(ToastOptions::new(message).color(Color::Danger))
    }

    /// Remove a toast, whether showing or still queued.
    pub fn dismiss(&self, id: ToastId) {
        let mut toasts = self.queues.toasts;
        toasts.write().retain(|entry| entry.id != id.0);
    }
}

/// Show toasts from event handlers.
///
/// Needs an [`AppWrapper`](crate::AppWrapper) above the calling component,
/// which renders them.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let toaster = use_toast();
/// rsx! { Button { onclick: move |_| { toaster.success("Saved"); }, "Save" } }
/// # }
/// ```
pub fn use_toast() -> Toaster {
    Toaster {
        queues: use_queues(),
    }
}

/// Shows alerts from code and waits for the answer. Get one with
/// [`use_alert`].
#[derive(Clone, Copy)]
pub struct Alerts {
    queues: OverlayQueues,
    strings: Signal<crate::Strings>,
}

impl Alerts {
    /// Show an alert and wait for the user's answer.
    pub fn show(&self, options: AlertOptions) -> impl Future<Output = AlertResult> + use<> {
        let (responder, reply) = reply();
        let id = self.queues.next();
        let mut alerts = self.queues.alerts;
        alerts.write().push(Entry {
            id,
            options,
            responder: Some(Rc::new(responder)),
        });
        reply
    }

    /// Ask a yes-or-no question. Resolves to `true` when the user confirms.
    pub fn confirm<T: Into<String>, M: Into<String>>(
        &self,
        title: T,
        message: M,
    ) -> impl Future<Output = bool> + use<T, M> {
        let strings = self.strings();
        let buttons = vec![
            AlertButton::cancel(strings.cancel),
            AlertButton::new(strings.confirm),
        ];
        let answer = self.show(AlertOptions {
            title: title.into(),
            message: Some(message.into()),
            buttons: buttons.clone(),
            input: None,
        });
        async move { answer.await.confirmed(&buttons) }
    }

    /// Ask for a line of text. Resolves to `None` when the user cancels.
    pub fn prompt<T: Into<String>>(
        &self,
        title: T,
        input: AlertInput,
    ) -> impl Future<Output = Option<String>> + use<T> {
        let strings = self.strings();
        let buttons = vec![
            AlertButton::cancel(strings.cancel),
            AlertButton::new(strings.confirm),
        ];
        let answer = self.show(AlertOptions {
            title: title.into(),
            message: None,
            buttons: buttons.clone(),
            input: Some(input),
        });
        async move {
            let result = answer.await;
            if result.confirmed(&buttons) {
                result.value
            } else {
                None
            }
        }
    }

    fn strings(&self) -> crate::Strings {
        self.strings.peek().clone()
    }
}

/// Show alerts from event handlers and await the answer.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # fn delete_round() {}
/// let alerts = use_alert();
/// rsx! {
///     Button {
///         onclick: move |_| async move {
///             if alerts.confirm("Delete round?", "This cannot be undone.").await {
///                 delete_round();
///             }
///         },
///         "Delete"
///     }
/// }
/// # }
/// ```
pub fn use_alert() -> Alerts {
    Alerts {
        queues: use_queues(),
        strings: crate::state::use_synced_signal(use_strings()),
    }
}

/// Shows action sheets from code and waits for the choice. Get one with
/// [`use_action_sheet`].
#[derive(Clone, Copy)]
pub struct ActionSheets {
    queues: OverlayQueues,
}

impl ActionSheets {
    /// Show an action sheet. Resolves to the chosen index, or `None` when
    /// cancelled.
    pub fn show(&self, options: ActionSheetOptions) -> impl Future<Output = Option<usize>> + use<> {
        let (responder, reply) = reply();
        let id = self.queues.next();
        let mut sheets = self.queues.sheets;
        sheets.write().push(Entry {
            id,
            options,
            responder: Some(Rc::new(responder)),
        });
        reply
    }
}

/// Show action sheets from event handlers and await the choice.
pub fn use_action_sheet() -> ActionSheets {
    ActionSheets {
        queues: use_queues(),
    }
}

/// Renders the first queued toast, alert, and action sheet.
#[component]
pub(crate) fn OverlayHost(queues: OverlayQueues) -> Element {
    let toast = queues.toasts.read().first().cloned();
    let alert = queues.alerts.read().first().cloned();
    let sheet = queues.sheets.read().first().cloned();
    rsx! {
        div { class: "g3-toast-host",
            if let Some(entry) = toast {
                QueuedToast {
                    key: "{entry.id}",
                    id: entry.id,
                    options: entry.options,
                    queue: queues.toasts,
                }
            }
        }
        if let Some(entry) = alert {
            QueuedAlert {
                key: "{entry.id}",
                id: entry.id,
                options: entry.options,
                queue: queues.alerts,
            }
        }
        if let Some(entry) = sheet {
            QueuedSheet {
                key: "{entry.id}",
                id: entry.id,
                options: entry.options,
                queue: queues.sheets,
            }
        }
    }
}

impl PartialEq for OverlayQueues {
    fn eq(&self, other: &Self) -> bool {
        self.toasts == other.toasts
            && self.alerts == other.alerts
            && self.sheets == other.sheets
            && self.next_id == other.next_id
    }
}

/// Answer an entry and drop it from its queue once it has animated out.
fn finish<O: 'static, R: 'static>(mut queue: Signal<Vec<Entry<O, R>>>, id: u64, answer: R) {
    let responder = queue
        .peek()
        .iter()
        .find(|entry| entry.id == id)
        .and_then(|entry| entry.responder.clone());
    if let Some(responder) = responder {
        responder.send(answer);
    }
    spawn(async move {
        dioxus_sdk_time::sleep(EXIT_DELAY).await;
        queue.write().retain(|entry| entry.id != id);
    });
}

#[component]
fn QueuedToast(
    id: u64,
    options: ToastOptions,
    queue: Signal<Vec<Entry<ToastOptions, ()>>>,
) -> Element {
    let open = use_signal(|| true);
    rsx! {
        Toast {
            open,
            message: options.message,
            color: options.color,
            position: options.position,
            duration: options.duration,
            on_dismiss: move |_| finish(queue, id, ()),
        }
    }
}

#[component]
fn QueuedAlert(
    id: u64,
    options: AlertOptions,
    queue: Signal<Vec<Entry<AlertOptions, AlertResult>>>,
) -> Element {
    let open = use_signal(|| true);
    rsx! {
        Alert {
            open,
            title: options.title,
            message: options.message,
            buttons: options.buttons,
            input: options.input,
            on_result: move |result| finish(queue, id, result),
        }
    }
}

#[component]
fn QueuedSheet(
    id: u64,
    options: ActionSheetOptions,
    queue: Signal<Vec<Entry<ActionSheetOptions, Option<usize>>>>,
) -> Element {
    let open = use_signal(|| true);
    let cancel_label = use_strings().cancel;
    rsx! {
        ActionSheet {
            open,
            title: options.title,
            message: options.message,
            buttons: options.buttons,
            cancel_label,
            on_select: move |choice| finish(queue, id, choice),
        }
    }
}
