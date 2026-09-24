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
    let lastX = 0;
    let lastTime = 0;
    let velocity = 0;
    let coastFrame = 0;
    let wheelFrame = 0;
    let wheelTarget = 0;
    let wheelTimer;
    let wheelSettled = false;
    const clamp = (value) => Math.max(0, Math.min(strip.scrollWidth - strip.clientWidth, value));
    const stopCoasting = () => {
        if (coastFrame) cancelAnimationFrame(coastFrame);
        coastFrame = 0;
    };
    const stopWheel = () => {
        if (wheelFrame) cancelAnimationFrame(wheelFrame);
        clearTimeout(wheelTimer);
        wheelFrame = 0;
    };
    // A mouse or wheel scrolls freely and never snaps: settling on an item
    // after the pointer let go read as the row jumping. Snapping is left to
    // the stylesheet, which applies it to touch alone.
    //
    // A released drag carries on at the speed it was let go, slowing under
    // friction, so the motion never changes speed abruptly.
    const coast = (speed) => {
        stopCoasting();
        if (calm.matches || Math.abs(speed) < 0.05) {
            delete strip.dataset.dragging;
            return;
        }
        let last = performance.now();
        const step = (now) => {
            const elapsed = Math.min(48, now - last);
            last = now;
            speed *= Math.pow(0.994, elapsed);
            const before = strip.scrollLeft;
            strip.scrollLeft = clamp(before + speed * elapsed);
            // Stopped by friction, or by an end of the row.
            if (Math.abs(speed) < 0.02 || strip.scrollLeft === before) {
                coastFrame = 0;
                delete strip.dataset.dragging;
                return;
            }
            coastFrame = requestAnimationFrame(step);
        };
        coastFrame = requestAnimationFrame(step);
    };
    // Eases toward `wheelTarget`, covering a share of the way each frame. The
    // step is at least a pixel, since a smaller one can round away to nothing
    // and leave the glide running forever.
    const glide = () => {
        const distance = wheelTarget - strip.scrollLeft;
        if (Math.abs(distance) <= 1) {
            strip.scrollLeft = wheelTarget;
            wheelFrame = 0;
            if (wheelSettled) delete strip.dataset.dragging;
            return;
        }
        strip.scrollLeft += Math.sign(distance) * Math.max(1, Math.abs(distance) * 0.2);
        wheelFrame = requestAnimationFrame(glide);
    };
    strip.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        stopCoasting();
        stopWheel();
        down = true;
        dragged = false;
        startX = event.clientX;
        startLeft = strip.scrollLeft;
        lastX = event.clientX;
        lastTime = performance.now();
        velocity = 0;
    });
    window.addEventListener("pointermove", (event) => {
        if (!down) return;
        const dx = event.clientX - startX;
        if (!dragged && Math.abs(dx) < 5) return;
        dragged = true;
        strip.dataset.dragging = "true";
        event.preventDefault();
        strip.scrollLeft = startLeft - dx;
        const now = performance.now();
        const elapsed = Math.max(1, now - lastTime);
        const instant = (lastX - event.clientX) / elapsed;
        velocity = velocity * 0.72 + instant * 0.28;
        lastX = event.clientX;
        lastTime = now;
    });
    window.addEventListener("pointerup", () => {
        if (!down) return;
        down = false;
        if (!dragged) {
            delete strip.dataset.dragging;
            return;
        }
        // A pointer held still before letting go has no speed left, even
        // though the last move it reported did.
        coast(performance.now() - lastTime > 80 ? 0 : velocity);
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
        const raw = Math.abs(event.deltaY) > Math.abs(event.deltaX)
            ? event.deltaY
            : event.deltaX;
        const max = strip.scrollWidth - strip.clientWidth;
        if (max <= 0) return;
        const delta = event.deltaMode === 1 ? raw * LINE
            : event.deltaMode === 2 ? raw * strip.clientWidth * 0.9
            : raw;
        // Against a stop, the page keeps the scroll.
        if ((delta < 0 && strip.scrollLeft <= 0) || (delta > 0 && strip.scrollLeft >= max - 1)) return;
        event.preventDefault();
        stopCoasting();
        strip.dataset.dragging = "true";
        if (!wheelFrame) wheelTarget = strip.scrollLeft;
        wheelTarget = clamp(wheelTarget + delta);
        wheelSettled = false;
        if (!wheelFrame) wheelFrame = requestAnimationFrame(glide);
        clearTimeout(wheelTimer);
        // The glide finishes where the wheel sent it; only then does the row
        // stop counting as scrolled by hand.
        wheelTimer = setTimeout(() => {
            wheelSettled = true;
            if (!wheelFrame) wheelFrame = requestAnimationFrame(glide);
        }, 120);
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

    #[test]
    fn a_mouse_drag_coasts_to_a_stop_instead_of_snapping() {
        assert!(SCRIPT.contains("const coast"));
        assert!(SCRIPT.contains("wheelTarget"));
        // The stylesheet snaps touch alone; the script must not snap either,
        // or a released drag jumps to the nearest item.
        assert!(!SCRIPT.contains("nearestSnap"));
    }
}
