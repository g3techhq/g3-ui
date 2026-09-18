//! Sideways scrolling for a strip that overflows its width: a scrollable
//! [`SegmentGroup`](crate::SegmentGroup) or a [`Shelf`](crate::Shelf).
use super::overlay::js_string;
use dioxus::prelude::*;

/// Touch already pans a strip natively. This adds the pointer cases a desktop
/// needs, marks the edges that have more past them, and keeps a selected item
/// in view.
///
/// Two things it takes care not to do. A wheel over a strip already against
/// one of its ends is left to the page, so a strip can never trap the page
/// beneath it. And wheel deltas are converted from lines and pages to pixels
/// first, since a three-line scroll read as three pixels looks like nothing
/// happened at all.
const SCRIPT: &str = r#"
const strip = document.getElementById(__ID__);
if (strip && strip.dataset.g3Scroll !== "true") {
    strip.dataset.g3Scroll = "true";
    const LINE = 16;
    const edges = () => {
        const max = strip.scrollWidth - strip.clientWidth;
        strip.dataset.overflowStart = String(strip.scrollLeft > 1);
        strip.dataset.overflowEnd = String(strip.scrollLeft < max - 1);
    };
    const calm = window.matchMedia("(prefers-reduced-motion: reduce)");
    const reveal = (animate) => {
        const smooth = animate && !calm.matches;
        const selected = strip.querySelector("[aria-checked=true], [aria-selected=true]");
        if (!selected) return;
        // The strip is positioned, so this is already relative to it.
        const left = selected.offsetLeft;
        const right = left + selected.offsetWidth;
        if (left < strip.scrollLeft + 16) {
            strip.scrollTo({ left: left - 16, behavior: smooth ? "smooth" : "auto" });
        } else if (right > strip.scrollLeft + strip.clientWidth - 16) {
            strip.scrollTo({ left: right - strip.clientWidth + 16, behavior: smooth ? "smooth" : "auto" });
        }
    };
    let down = false;
    let dragged = false;
    let startX = 0;
    let startLeft = 0;
    strip.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        down = true;
        dragged = false;
        startX = event.clientX;
        startLeft = strip.scrollLeft;
    });
    window.addEventListener("pointermove", (event) => {
        if (!down) return;
        const dx = event.clientX - startX;
        if (!dragged && Math.abs(dx) < 5) return;
        dragged = true;
        strip.dataset.dragging = "true";
        strip.scrollLeft = startLeft - dx;
    });
    window.addEventListener("pointerup", () => {
        down = false;
        delete strip.dataset.dragging;
    });
    // A drag must not also press whatever it ended on.
    strip.addEventListener("click", (event) => {
        if (dragged) {
            dragged = false;
            event.preventDefault();
            event.stopPropagation();
        }
    }, true);
    strip.addEventListener("wheel", (event) => {
        // A trackpad's own sideways gesture already scrolls the strip.
        if (Math.abs(event.deltaY) <= Math.abs(event.deltaX)) return;
        const max = strip.scrollWidth - strip.clientWidth;
        if (max <= 0) return;
        const delta = event.deltaMode === 1 ? event.deltaY * LINE
            : event.deltaMode === 2 ? event.deltaY * strip.clientWidth * 0.9
            : event.deltaY;
        // Against a stop, the page keeps the scroll.
        if ((delta < 0 && strip.scrollLeft <= 0) || (delta > 0 && strip.scrollLeft >= max - 1)) return;
        event.preventDefault();
        strip.scrollLeft = Math.max(0, Math.min(max, strip.scrollLeft + delta));
    }, { passive: false });
    strip.addEventListener("scroll", edges, { passive: true });
    new ResizeObserver(edges).observe(strip);
    new MutationObserver(() => { reveal(true); edges(); })
        .observe(strip, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-checked", "aria-selected"] });
    reveal(false);
    edges();
}
"#;

/// Gives the element with this id sideways scrolling, once it is mounted.
/// `enabled` lets a component switch it on only when it overflows by design.
pub(crate) fn use_horizontal_scroll(id: String, enabled: bool) {
    use_effect(use_reactive!(|enabled| {
        if enabled {
            document::eval(&SCRIPT.replace("__ID__", &js_string(&id)));
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::SCRIPT;

    #[test]
    fn a_wheel_at_a_stop_is_left_to_the_page() {
        // The early return has to come before preventDefault, or the strip
        // traps the page whenever it is scrolled to an end.
        let stop = SCRIPT.find("Against a stop").expect("stop check");
        let prevent = SCRIPT
            .rfind("event.preventDefault();")
            .expect("preventDefault");
        assert!(stop < prevent);
    }

    #[test]
    fn wheel_lines_and_pages_become_pixels() {
        assert!(SCRIPT.contains("event.deltaMode === 1"));
        assert!(SCRIPT.contains("event.deltaMode === 2"));
    }
}
