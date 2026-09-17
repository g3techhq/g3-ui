//! The spinning columns of an iOS-style picker.
use super::overlay::js_string;
use crate::state::{use_element_id, use_synced_signal};
use dioxus::prelude::*;

/// Keeps a column's scroll position and its value in step. Scrolling,
/// flicking, or dragging with a mouse reports the item that settles in the
/// middle; a new value from Rust scrolls to it.
const WHEEL_SCRIPT: &str = r#"
const el = document.getElementById(__ID__);
if (el && el.dataset.g3Wheel !== "true") {
    el.dataset.g3Wheel = "true";
    const calm = window.matchMedia("(prefers-reduced-motion: reduce)");
    const step = () => (el.querySelector(".g3-wheel-item") || {}).offsetHeight || 36;
    let quietUntil = 0;
    let timer = 0;
    const settle = () => {
        const count = el.querySelectorAll(".g3-wheel-item").length;
        const index = Math.max(0, Math.min(count - 1, Math.round(el.scrollTop / step())));
        el.scrollTo({ top: index * step(), behavior: calm.matches ? "auto" : "smooth" });
        dioxus.send(index);
    };
    el.addEventListener("scroll", () => {
        if (performance.now() < quietUntil) return;
        clearTimeout(timer);
        timer = setTimeout(settle, 140);
    }, { passive: true });

    // Touch and trackpads scroll natively; a mouse drags.
    // A press only becomes a drag once it moves, so a plain click still
    // reaches the item under it.
    let pressed = false;
    let dragging = false;
    let startY = 0;
    let startTop = 0;
    el.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        pressed = true;
        dragging = false;
        startY = event.clientY;
        startTop = el.scrollTop;
    });
    el.addEventListener("pointermove", (event) => {
        if (!pressed) return;
        const dy = event.clientY - startY;
        if (!dragging && Math.abs(dy) < 4) return;
        if (!dragging) {
            dragging = true;
            el.dataset.dragging = "true";
            try { el.setPointerCapture(event.pointerId); } catch (error) {}
        }
        el.scrollTop = startTop - dy;
    });
    const release = () => {
        pressed = false;
        if (!dragging) return;
        dragging = false;
        delete el.dataset.dragging;
        clearTimeout(timer);
        timer = setTimeout(settle, 60);
    };
    el.addEventListener("pointerup", release);
    el.addEventListener("pointercancel", release);

    while (true) {
        const message = await dioxus.recv();
        const [index, animate] = message;
        const top = index * step();
        if (Math.abs(el.scrollTop - top) > 1) {
            const smooth = animate && !calm.matches;
            quietUntil = performance.now() + (smooth ? 450 : 80);
            el.scrollTo({ top, behavior: smooth ? "smooth" : "auto" });
        }
    }
}
"#;

/// One column of a wheel picker, such as the hours of a time.
///
/// Screen readers and keyboards meet a spin button: arrow keys step, Home
/// and End jump to the ends, Page Up and Page Down move by five. The visible
/// list is for pointers and is hidden from assistive technology.
#[component]
pub(crate) fn WheelColumn(
    labels: Vec<String>,
    index: usize,
    onchange: EventHandler<usize>,
    aria_label: String,
    /// Text alignment and share of the width: month names need more room.
    #[props(default)]
    wide: bool,
) -> Element {
    let id = use_element_id("wheel", None);
    let scroll_id = format!("{id}-scroll");
    let last = labels.len().saturating_sub(1);
    let index = index.min(last);
    let current = use_synced_signal(index);
    let mut session = use_signal(|| None::<document::Eval>);
    let mut first = use_signal(|| true);

    {
        let scroll_id = scroll_id.clone();
        use_effect(move || {
            let mut eval = document::eval(&WHEEL_SCRIPT.replace("__ID__", &js_string(&scroll_id)));
            session.set(Some(eval));
            spawn(async move {
                while let Ok(reported) = eval.recv::<usize>().await {
                    if reported != *current.peek() {
                        onchange.call(reported);
                    }
                }
            });
        });
    }
    use_effect(use_reactive!(|index| {
        if let Some(eval) = *session.peek() {
            let smooth = !*first.peek();
            first.set(false);
            let _ = eval.send((index, smooth));
        }
    }));
    // The session exists only after the first effect ran; position it then.
    use_effect(move || {
        if let Some(eval) = session() {
            let _ = eval.send((*current.peek(), false));
        }
    });

    let step = move |to: usize| {
        let to = to.min(last);
        if to != index {
            onchange.call(to);
        }
    };
    let value_text = labels.get(index).cloned().unwrap_or_default();

    rsx! {
        div {
            class: if wide { "g3-wheel-column g3-wheel-column-wide" } else { "g3-wheel-column" },
            role: "spinbutton",
            tabindex: "0",
            aria_label,
            aria_valuemin: "0",
            aria_valuemax: last.to_string(),
            aria_valuenow: index.to_string(),
            aria_valuetext: value_text,
            onkeydown: move |event| {
                let to = match event.key() {
                    Key::ArrowUp => index.saturating_sub(1),
                    Key::ArrowDown => index + 1,
                    Key::PageUp => index.saturating_sub(5),
                    Key::PageDown => index + 5,
                    Key::Home => 0,
                    Key::End => last,
                    _ => return,
                };
                event.prevent_default();
                step(to);
            },
            div { id: scroll_id, class: "g3-wheel-scroll", aria_hidden: "true",
                div { class: "g3-wheel-spacer" }
                for (item, label) in labels.iter().enumerate() {
                    div {
                        key: "{item}",
                        class: "g3-wheel-item",
                        "data-selected": (item == index).then_some("true"),
                        onclick: move |_| step(item),
                        "{label}"
                    }
                }
                div { class: "g3-wheel-spacer" }
            }
        }
    }
}
