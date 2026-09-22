//! Arrow-key movement inside composite widgets.
use super::overlay::js_string;
use dioxus::prelude::*;

const ROVING_SCRIPT: &str = r#"
const root = document.getElementById(__ID__);
if (root && root.dataset.g3Roving !== "true") {
    root.dataset.g3Roving = "true";
    const selector = __SELECTOR__;
    const keys = __VERTICAL__
        ? { next: ["ArrowDown", "ArrowRight"], previous: ["ArrowUp", "ArrowLeft"] }
        : { next: ["ArrowRight", "ArrowDown"], previous: ["ArrowLeft", "ArrowUp"] };
    root.addEventListener("keydown", (event) => {
        const items = [...root.querySelectorAll(selector)]
            .filter((node) => !node.disabled && node.getAttribute("aria-disabled") !== "true");
        const current = items.indexOf(document.activeElement);
        if (current < 0) return;
        const rtl = getComputedStyle(root).direction === "rtl" && !__VERTICAL__;
        let next = -1;
        if (keys.next.includes(event.key)) next = rtl ? current - 1 : current + 1;
        else if (keys.previous.includes(event.key)) next = rtl ? current + 1 : current - 1;
        else if (event.key === "Home") next = 0;
        else if (event.key === "End") next = items.length - 1;
        else return;
        event.preventDefault();
        const target = items[(next + items.length) % items.length];
        target.focus();
        target.click();
    });
}
"#;

/// Arrow keys, Home, and End move focus between the elements matching
/// `selector` inside the element with id `id`, and activate the one they
/// land on, as radio groups and tab lists do. Pair with a roving `tabindex`
/// so only the selected item is in the tab order.
pub(crate) fn use_roving_selection(id: String, selector: &'static str, vertical: bool) {
    use_effect(move || {
        let script = ROVING_SCRIPT
            .replace("__ID__", &js_string(&id))
            .replace("__SELECTOR__", &js_string(selector))
            .replace("__VERTICAL__", if vertical { "true" } else { "false" });
        let _ = document::eval(&script);
    });
}
