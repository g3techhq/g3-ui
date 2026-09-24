// Drives every SwipeItem on the page from inside the webview.
//
// On native renderers each Rust event handler is a synchronous request from
// the webview to Rust, and the page cannot paint or scroll until it returns.
// Tracking a finger from Rust costs one of those per pointer move, which on
// Android is too slow for the row to follow the finger at all, and a Rust
// touchmove listener makes every scroll in the app wait on one. So the gesture
// runs here, and Rust hears only how each one ended. Each row describes itself
// in data attributes, which are read afresh at every press.
const key = Symbol.for("g3-ui.swipe");
window[key]?.dispose?.();

const REVEAL_WIDTH = 88;
const ACTIVATE_WIDTH = 136;
const DISMISS_WIDTH = 104;
const FULL_SWIPE_MARGIN = 30;
const ELASTIC_FACTOR = 0.55;
const ACTIVATE_RATIO = 0.48;
const ACTIVATE_SOFTENING = 0.72;
const DISMISS_OFFSET = 430;
const DISMISS_EXIT_MS = 560;
const DISMISS_COLLAPSE_MS = 180;
const LONG_PRESS_MS = 500;
const LONG_PRESS_SLOP = 8;
// Matches the browser's own slop: below it every gesture looks diagonal, and
// claiming one would steal ordinary scrolls.
const HORIZONTAL_SLOP = 10;
const SUPPRESS_CLICK_MS = 300;

// Where each row rests between gestures: 0, or open by an edge's width.
const rests = new WeakMap();
let active = null;
let suppress = null;

const send = (row, kind, offset, edge, committed) => {
    dioxus.send([row.id, kind, [offset, edge ? offset / edge.width : 0, committed ? 1 : 0]]);
};

// The edge a drag of `raw` pixels moves toward, if it has actions.
const edgeFor = (row, raw) => {
    const side = raw > 0 ? "start" : raw < 0 ? "end" : null;
    if (!side || row.dataset[side === "start" ? "hasStart" : "hasEnd"] !== "true") return null;
    const behavior = row.dataset[side === "start" ? "startBehavior" : "endBehavior"];
    let width = behavior === "activate" ? ACTIVATE_WIDTH : DISMISS_WIDTH;
    if (behavior === "reveal") {
        // Revealed actions are as wide as their buttons.
        const actions = row.querySelector(`:scope > .g3-swipe-actions-${side}`);
        width = Math.max(actions?.getBoundingClientRect().width || REVEAL_WIDTH, 1);
    }
    return { behavior, width };
};

// Where the row sits for a raw drag distance toward `edge`.
const offsetFor = (edge, raw) => {
    const limit = Math.max(edge.width, 1);
    if (edge.behavior === "reveal") return Math.min(Math.max(raw, -edge.width), edge.width);
    if (edge.behavior === "dismiss") {
        if (raw > limit) return limit + (raw - limit) * ELASTIC_FACTOR;
        if (raw < -limit) return -limit + (raw + limit) * ELASTIC_FACTOR;
        return raw;
    }
    const softenFrom = limit * ACTIVATE_SOFTENING;
    const distance = Math.abs(raw);
    if (distance <= softenFrom) return raw;
    const extra = distance - softenFrom;
    const remaining = Math.max(limit - softenFrom, 1);
    return Math.sign(raw) * (softenFrom + remaining * (extra / (extra + remaining)));
};

const committedAt = (edge, offset) => edge.behavior === "activate"
    ? Math.abs(offset) >= edge.width * ACTIVATE_RATIO
    : Math.abs(offset) >= edge.width + FULL_SWIPE_MARGIN;

const paint = (row, offset, dragging) => {
    const edge = edgeFor(row, offset);
    const width = edge?.width ?? REVEAL_WIDTH;
    row.style.setProperty("--g3-swipe-offset", `${offset}px`);
    row.style.setProperty("--g3-swipe-progress", String(Math.min(Math.abs(offset / width), 1.4)));
    // Which way a dismissed row leaves.
    row.style.setProperty("--g3-swipe-exit", offset < 0 ? "-1" : "1");
    if (dragging) row.dataset.dragging = "true";
    else delete row.dataset.dragging;
    // Rust hears only where a row comes to rest, so its `aria-hidden` still
    // says closed for the whole drag - and for all of an activate swipe, which
    // always rests shut. The edge being uncovered is shown from here instead.
    if (offset > 0) row.dataset.exposed = "start";
    else if (offset < 0) row.dataset.exposed = "end";
    else delete row.dataset.exposed;
    if (edge && edge.behavior !== "reveal" && committedAt(edge, offset)) row.dataset.committed = "true";
    else delete row.dataset.committed;
};

// Settle a row at rest and tell Rust, which keeps the hidden actions inert.
const rest = (row, offset) => {
    paint(row, offset, false);
    if ((rests.get(row) ?? 0) !== offset) {
        rests.set(row, offset);
        send(row, "rest", offset, edgeFor(row, offset), false);
    }
};

const idle = (row) => (row.dataset.state ?? "idle") === "idle";

// A drag that is clearly sideways must not scroll the page. Attached to the
// row itself, so only rows are blocking touch regions rather than the whole
// document, and the handler never leaves the webview.
const onTouchMove = (event) => {
    if (active?.horizontal && event.cancelable) event.preventDefault();
};

const onPointerDown = (event) => {
    // A reorder handle or a rating star in the row claimed this press first.
    if (event.defaultPrevented) return;
    const row = event.target instanceof Element && event.target.closest(".g3-swipe-item");
    if (!row || active || row.dataset.disabled === "true" || !idle(row)) return;
    if (row.dataset.mouseSwipe === "false" && event.pointerType === "mouse") return;
    if (!row.g3SwipeTouch) {
        row.g3SwipeTouch = true;
        row.addEventListener("touchmove", onTouchMove, { passive: false });
    }
    const base = rests.get(row) ?? 0;
    active = {
        row,
        pointerId: event.pointerId,
        x: event.clientX,
        y: event.clientY,
        base,
        offset: base,
        horizontal: false,
        moved: false,
        timer: null,
    };
    // Capture at once. Touch browsers can dispatch a leave as soon as the
    // finger moves off the original hit-test box; if capture waits for
    // horizontal intent, that leave ends the gesture and the row snaps back.
    try { row.setPointerCapture(event.pointerId); } catch (error) {}
    if (row.dataset.longPress === "true") {
        const gesture = active;
        gesture.timer = setTimeout(() => {
            if (active === gesture && !gesture.moved) send(row, "long-press", 0, null, false);
        }, LONG_PRESS_MS);
    }
};

const onPointerMove = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId || !idle(gesture.row)) return;
    const moveX = event.clientX - gesture.x;
    const dy = event.clientY - gesture.y;
    if (Math.hypot(moveX, dy) > LONG_PRESS_SLOP) gesture.moved = true;
    if (!gesture.horizontal) {
        if (Math.abs(moveX) > HORIZONTAL_SLOP && Math.abs(moveX) > Math.abs(dy)) {
            gesture.horizontal = true;
        } else {
            return;
        }
    }
    // Resume from where an open row rests.
    const raw = gesture.base + moveX;
    const edge = edgeFor(gesture.row, raw);
    gesture.offset = edge ? offsetFor(edge, raw) : 0;
    paint(gesture.row, gesture.offset, true);
    if (edge && gesture.row.dataset.reportSwipe === "true") {
        send(gesture.row, "move", gesture.offset, edge, committedAt(edge, gesture.offset));
    }
};

const release = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId) return;
    active = null;
    clearTimeout(gesture.timer);
    const { row, offset } = gesture;
    if (!gesture.horizontal) return;
    // The click that ends a swipe must not also activate the row.
    suppress = { row, until: performance.now() + SUPPRESS_CLICK_MS };
    const edge = edgeFor(row, offset);
    if (!edge) return rest(row, 0);
    const committed = committedAt(edge, offset);
    if (edge.behavior === "reveal") {
        return rest(row, Math.abs(offset) > edge.width / 2 ? Math.sign(offset) * edge.width : 0);
    }
    if (edge.behavior === "activate") {
        if (committed) send(row, "activate", offset, edge, true);
        return rest(row, 0);
    }
    if (!committed) return rest(row, 0);
    // Collapse from the row's own height, whatever it holds.
    row.style.setProperty("--g3-swipe-row-height", `${row.getBoundingClientRect().height}px`);
    paint(row, DISMISS_OFFSET * Math.sign(offset), false);
    row.dataset.state = "exiting";
    setTimeout(() => {
        row.dataset.state = "collapsing";
        setTimeout(() => send(row, "dismiss", offset, edge, true), DISMISS_COLLAPSE_MS);
    }, DISMISS_EXIT_MS);
};

const onPointerCancel = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId) return;
    active = null;
    clearTimeout(gesture.timer);
    if (idle(gesture.row)) rest(gesture.row, 0);
};

const onClick = (event) => {
    if (!suppress || performance.now() > suppress.until) return;
    if (event.target instanceof Node && suppress.row.contains(event.target)) {
        event.preventDefault();
        event.stopPropagation();
    }
};

document.addEventListener("pointerdown", onPointerDown);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("pointerup", release);
document.addEventListener("lostpointercapture", release);
document.addEventListener("pointercancel", onPointerCancel);
// Capturing, so it runs before the renderer's own click listener.
document.addEventListener("click", onClick, true);

let released;
const done = new Promise((resolve) => {
    released = resolve;
});
window[key] = {
    // Opening or closing from the keyboard, the actions button, or a pressed
    // action.
    command(id, to) {
        const row = document.getElementById(id);
        if (!row || !idle(row)) return;
        if (to === "close") return rest(row, 0);
        // Whatever the edge's swipe does, this uncovers its buttons.
        if (row.dataset[to === "start" ? "hasStart" : "hasEnd"] !== "true") return;
        const actions = row.querySelector(`:scope > .g3-swipe-actions-${to}`);
        const width = Math.max(actions?.getBoundingClientRect().width || REVEAL_WIDTH, 1);
        rest(row, to === "start" ? width : -width);
    },
    dispose() {
        document.removeEventListener("pointerdown", onPointerDown);
        document.removeEventListener("pointermove", onPointerMove);
        document.removeEventListener("pointerup", release);
        document.removeEventListener("lostpointercapture", release);
        document.removeEventListener("pointercancel", onPointerCancel);
        document.removeEventListener("click", onClick, true);
        released();
    },
};
// Native renderers close an eval's channel as soon as its script returns.
await done;
