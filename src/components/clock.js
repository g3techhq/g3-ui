// Drives every ClockDial on the page from inside the webview, so dragging the
// hand never waits on Rust (see gesture.rs). Rust hears each value the drag
// lands on, at most sixty a turn, and when the press ends. The hand is drawn
// by Rust from the value, which the caller owns.
const key = Symbol.for("g3-ui.clock");
window[key]?.dispose?.();

// The dial is drawn at this size, and its rings at these radii.
const SIZE = 256;
const OUTER_RADIUS = 100;
const INNER_RADIUS = 64;

let active = null;

// The value under a point on the dial, given relative to its centre in the
// dial's own pixels.
const valueAt = (mode, dx, dy) => {
    const angle = ((Math.atan2(dx, -dy) * 180) / Math.PI + 360) % 360;
    if (mode === "minutes") return Math.round(angle / 6) % 60;
    const hour = Math.round(angle / 30) % 12;
    if (mode === "hours24" && Math.hypot(dx, dy) < (OUTER_RADIUS + INNER_RADIUS) / 2) {
        return hour === 0 ? 0 : hour + 12;
    }
    return hour === 0 ? 12 : hour;
};

const pick = (event) => {
    const { face } = active;
    const rect = face.getBoundingClientRect();
    if (!rect.width) return;
    // Measured from the face itself, whichever number was pressed, and scaled
    // in case the dial is drawn smaller than its design size.
    const scale = SIZE / rect.width;
    const dx = (event.clientX - rect.left - rect.width / 2) * scale;
    const dy = (event.clientY - rect.top - rect.height / 2) * scale;
    const value = valueAt(face.dataset.mode, dx, dy);
    if (value !== active.value) {
        active.value = value;
        dioxus.send([face.id, "pick", [value]]);
    }
};

const onPointerDown = (event) => {
    const face = event.target instanceof Element && event.target.closest(".g3-clock");
    if (!face || active) return;
    // Keeps the pointer coming to the dial while it drags past the edge.
    try { face.setPointerCapture(event.pointerId); } catch (error) {}
    face.dataset.dragging = "true";
    active = { face, pointerId: event.pointerId, value: null };
    pick(event);
};

const onPointerMove = (event) => {
    if (active && event.pointerId === active.pointerId) pick(event);
};

const end = (kind) => (event) => {
    if (!active || event.pointerId !== active.pointerId) return;
    const { face } = active;
    active = null;
    delete face.dataset.dragging;
    dioxus.send([face.id, kind, []]);
};
const onPointerUp = end("release");
const onPointerCancel = end("cancel");

document.addEventListener("pointerdown", onPointerDown);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("pointerup", onPointerUp);
document.addEventListener("pointercancel", onPointerCancel);

let released;
const done = new Promise((resolve) => {
    released = resolve;
});
window[key] = {
    dispose() {
        document.removeEventListener("pointerdown", onPointerDown);
        document.removeEventListener("pointermove", onPointerMove);
        document.removeEventListener("pointerup", onPointerUp);
        document.removeEventListener("pointercancel", onPointerCancel);
        released();
    },
};
// Native renderers close an eval's channel as soon as its script returns.
await done;
