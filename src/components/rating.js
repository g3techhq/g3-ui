// Drives every interactive Rating on the page from inside the webview, so a
// drag across the stars never waits on Rust (see gesture.rs). Rust hears each
// value the drag lands on, which is at most one per half star, and when the
// gesture ends. The stars are drawn by Rust from the value, which the caller
// owns.
const key = Symbol.for("g3-ui.rating");
window[key]?.dispose?.();

let active = null;

// The value a pointer at `clientX` picks. Snaps to whole stars, or halves
// when the row allows them, and never goes below the smallest step: clearing
// is the keyboard's Home.
const valueAt = (row, clientX) => {
    const stars = row.querySelectorAll(":scope > .g3-rating-star");
    const max = stars.length;
    if (!max) return null;
    const first = stars[0].getBoundingClientRect();
    const size = first.width;
    // Measured, so the stars may shrink to fit and the gap may change.
    const pitch = max > 1 ? stars[1].getBoundingClientRect().left - first.left : size;
    const offset = clientX - first.left;
    const index = Math.min(Math.max(Math.floor(offset / pitch), 0), max - 1);
    const half = row.dataset.half === "true";
    const step = half && offset - index * pitch < size / 2 ? 0.5 : 1;
    return Math.min(Math.max(index + step, half ? 0.5 : 1), max);
};

const pick = (event) => {
    const value = valueAt(active.row, event.clientX);
    if (value !== null && value !== active.value) {
        active.value = value;
        dioxus.send([active.row.id, "input", [value]]);
    }
};

const finish = (event) => {
    if (!active || event.pointerId !== active.pointerId) return;
    const { row } = active;
    active = null;
    dioxus.send([row.id, "commit", []]);
};

// Capturing, so the press is claimed before a swipe row around the stars
// sees it.
const onPointerDown = (event) => {
    const star = event.target instanceof Element && event.target.closest(".g3-rating-star");
    const row = star?.closest('.g3-rating[data-interactive="true"]');
    if (!row || active) return;
    event.preventDefault();
    if (!row.g3RatingLeave) {
        // Leaving the stars ends the drag. `pointerleave` does not bubble, so
        // it is heard on the row itself.
        row.g3RatingLeave = true;
        row.addEventListener("pointerleave", finish);
    }
    active = { row, pointerId: event.pointerId, value: null };
    pick(event);
};

const onPointerMove = (event) => {
    if (active && event.pointerId === active.pointerId) pick(event);
};

document.addEventListener("pointerdown", onPointerDown, true);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("pointerup", finish);
document.addEventListener("pointercancel", finish);

let released;
const done = new Promise((resolve) => {
    released = resolve;
});
window[key] = {
    dispose() {
        document.removeEventListener("pointerdown", onPointerDown, true);
        document.removeEventListener("pointermove", onPointerMove);
        document.removeEventListener("pointerup", finish);
        document.removeEventListener("pointercancel", finish);
        released();
    },
};
// Native renderers close an eval's channel as soon as its script returns.
await done;
