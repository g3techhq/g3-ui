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
const touchListeners = new AbortController();
const touchRows = new WeakSet();

const direction = (row) => getComputedStyle(row).direction === "rtl" ? -1 : 1;

const send = (row, kind, offset, edge, committed, dir = direction(row)) => {
    // Reports use logical displacement: positive exposes Start in either direction.
    offset *= dir;
    dioxus.send([row.id, kind, [offset, edge ? offset / edge.width : 0, committed ? 1 : 0]]);
};

// The edge a drag of `raw` pixels moves toward, if it has actions.
const edgeFor = (row, raw, dir = direction(row)) => {
    const logical = raw * dir;
    const side = logical > 0 ? "start" : logical < 0 ? "end" : null;
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

// Measure both edges before any writes. A move must never read computed style
// after painting the previous move: that forces style/layout on Android.
const geometryFor = (row) => {
    const dir = direction(row);
    return { dir, start: edgeFor(row, dir, dir), end: edgeFor(row, -dir, dir) };
};
const gestureEdge = (geometry, offset) => offset * geometry.dir > 0
    ? geometry.start : offset * geometry.dir < 0 ? geometry.end : null;

const paint = (row, offset, dragging, geometry = geometryFor(row)) => {
    const edge = gestureEdge(geometry, offset);
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
    if (offset * geometry.dir > 0) row.dataset.exposed = "start";
    else if (offset * geometry.dir < 0) row.dataset.exposed = "end";
    else delete row.dataset.exposed;
    if (edge && edge.behavior !== "reveal" && committedAt(edge, offset)) row.dataset.committed = "true";
    else delete row.dataset.committed;
};

// Settle a row at rest and tell Rust, which keeps the hidden actions inert.
const rest = (row, offset, geometry = geometryFor(row)) => {
    paint(row, offset, false, geometry);
    if ((rests.get(row) ?? 0) !== offset) {
        rests.set(row, offset);
        send(row, "rest", offset, gestureEdge(geometry, offset), false, geometry.dir);
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
    // Only the compatibility click from the old gesture is suppressed. A new
    // press, even on the same card before its settle finishes, is a fresh tap.
    suppress = null;
    // A reorder handle or a rating star in the row claimed this press first.
    if (event.defaultPrevented || event.isPrimary === false) return;
    if (event.pointerType === "mouse" && event.button !== 0) return;
    const row = event.target instanceof Element && event.target.closest(".g3-swipe-item");
    if (!row || active || row.dataset.disabled === "true" || !idle(row)) return;
    if (row.dataset.mouseSwipe === "false" && event.pointerType === "mouse") return;
    if (!touchRows.has(row)) {
        touchRows.add(row);
        row.addEventListener("touchmove", onTouchMove, { passive: false, signal: touchListeners.signal });
    }
    const resting = rests.get(row) ?? 0;
    // A new drag can catch a row halfway through its settle. Resume from the
    // visible transform rather than jumping to the eventual resting offset.
    const content = row.querySelector(":scope > .g3-swipe-content");
    const transform = content && getComputedStyle(content).transform;
    const matrix = transform?.match(/^matrix(3d)?\((.+)\)$/);
    const base = matrix ? Number(matrix[2].split(",")[matrix[1] ? 12 : 4]) : resting;
    active = {
        row,
        pointerId: event.pointerId,
        x: event.clientX,
        y: event.clientY,
        base,
        resting,
        geometry: geometryFor(row),
        frame: 0,
        offset: base,
        horizontal: false,
        moved: false,
        timer: null,
    };
    if (row.dataset.longPress === "true") {
        const gesture = active;
        gesture.timer = setTimeout(() => {
            if (active === gesture && !gesture.moved) longPress(gesture);
        }, LONG_PRESS_MS);
    }
};

// Once per gesture. Marked so the release that follows does not also tap the row:
// a held card would open its menu and then whatever it links to.
const longPress = (gesture) => {
    if (gesture.longPressed) return;
    gesture.longPressed = true;
    clearTimeout(gesture.timer);
    send(gesture.row, "long-press", 0, null, false);
};

const onPointerMove = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId || !idle(gesture.row)) return;
    const moveX = event.clientX - gesture.x;
    const dy = event.clientY - gesture.y;
    if (!gesture.moved && Math.hypot(moveX, dy) > LONG_PRESS_SLOP) {
        gesture.moved = true;
        clearTimeout(gesture.timer);
    }
    if (!gesture.horizontal) {
        if (Math.abs(dy) > HORIZONTAL_SLOP && Math.abs(dy) >= Math.abs(moveX)) {
            active = null;
            return;
        }
        if (Math.abs(moveX) > HORIZONTAL_SLOP && Math.abs(moveX) > Math.abs(dy)) {
            gesture.horizontal = true;
            // Capture only a swipe; ordinary taps retain their button target.
            try { gesture.row.setPointerCapture(event.pointerId); } catch (error) {}
        } else {
            return;
        }
    }
    // Resume from where an open row rests.
    const raw = gesture.base + moveX;
    const edge = gestureEdge(gesture.geometry, raw);
    gesture.offset = edge ? offsetFor(edge, raw) : 0;
    if (gesture.frame) return;
    gesture.frame = requestAnimationFrame(() => {
        gesture.frame = 0;
        if (active !== gesture) return;
        paint(gesture.row, gesture.offset, true, gesture.geometry);
        const edge = gestureEdge(gesture.geometry, gesture.offset);
        if (edge && gesture.row.dataset.reportSwipe === "true") {
            send(gesture.row, "move", gesture.offset, edge, committedAt(edge, gesture.offset), gesture.geometry.dir);
        }
    });
};

const release = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId) return;
    active = null;
    clearTimeout(gesture.timer);
    cancelAnimationFrame(gesture.frame);
    const { row, offset, geometry } = gesture;
    // The click that ends a swipe or a long press must not also activate the row.
    if (gesture.longPressed || gesture.horizontal) {
        suppress = { row, until: performance.now() + SUPPRESS_CLICK_MS };
    }
    if (!gesture.horizontal) return;
    const edge = gestureEdge(geometry, offset);
    if (!edge) return rest(row, 0, geometry);
    const committed = committedAt(edge, offset);
    if (edge.behavior === "reveal") {
        return rest(row, Math.abs(offset) > edge.width / 2 ? Math.sign(offset) * edge.width : 0, geometry);
    }
    if (edge.behavior === "activate") {
        if (committed) send(row, "activate", offset, edge, true, geometry.dir);
        return rest(row, 0, geometry);
    }
    if (!committed) return rest(row, 0, geometry);
    // Collapse from the row's own height, whatever it holds.
    row.style.setProperty("--g3-swipe-row-height", `${row.getBoundingClientRect().height}px`);
    paint(row, DISMISS_OFFSET * Math.sign(offset), false, geometry);
    row.dataset.state = "exiting";
    setTimeout(() => {
        row.dataset.state = "collapsing";
        setTimeout(() => send(row, "dismiss", offset, edge, true), DISMISS_COLLAPSE_MS);
    }, DISMISS_EXIT_MS);
};

// Only the row losing its own capture ends a gesture. A touch starts out
// captured by whatever the finger pressed, and the row taking capture over
// from it fires a loss on that element, which bubbles here mid-swipe. Taken
// for a release, it snapped every touch swipe back after its first frame.
const onLostPointerCapture = (event) => {
    if (event.target === active?.row) onPointerCancel(event);
};

const onPointerCancel = (event) => {
    const gesture = active;
    if (!gesture || event.pointerId !== gesture.pointerId) return;
    active = null;
    clearTimeout(gesture.timer);
    cancelAnimationFrame(gesture.frame);
    if (idle(gesture.row)) rest(gesture.row, gesture.resting, gesture.geometry);
};

const onClick = (event) => {
    if (event.detail === 0) return;
    if (!suppress || performance.now() > suppress.until) return;
    if (event.target instanceof Node && suppress.row.contains(event.target)) {
        event.preventDefault();
        event.stopPropagation();
        suppress = null;
    }
};

// A row that answers a long press owns it. Android's WebView otherwise opens
// its own menu or starts a text selection at about the same moment, and can
// cancel the pointer first, so the menu counts as the long press too. A right
// click or the menu key, with no press under way, is the same request.
const onContextMenu = (event) => {
    const row = event.target instanceof Element && event.target.closest('.g3-swipe-item[data-long-press="true"]');
    if (!row || row.dataset.disabled === "true") return;
    event.preventDefault();
    if (active?.row === row) {
        if (!active.moved) longPress(active);
    } else if (idle(row)) {
        send(row, "long-press", 0, null, false);
    }
};

document.addEventListener("pointerdown", onPointerDown);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("pointerup", release);
document.addEventListener("lostpointercapture", onLostPointerCapture);
document.addEventListener("pointercancel", onPointerCancel);
// Capturing, so it runs before the renderer's own click listener.
document.addEventListener("click", onClick, true);
document.addEventListener("contextmenu", onContextMenu);

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
        if (to === "left") to = direction(row) === 1 ? "end" : "start";
        if (to === "right") to = direction(row) === 1 ? "start" : "end";
        // Whatever the edge's swipe does, this uncovers its buttons.
        if (row.dataset[to === "start" ? "hasStart" : "hasEnd"] !== "true") return;
        const actions = row.querySelector(`:scope > .g3-swipe-actions-${to}`);
        const width = Math.max(actions?.getBoundingClientRect().width || REVEAL_WIDTH, 1);
        rest(row, (to === "start" ? width : -width) * direction(row));
    },
    dispose() {
        touchListeners.abort();
        if (active) onPointerCancel({ pointerId: active.pointerId });
        suppress = null;
        document.removeEventListener("pointerdown", onPointerDown);
        document.removeEventListener("pointermove", onPointerMove);
        document.removeEventListener("pointerup", release);
        document.removeEventListener("lostpointercapture", onLostPointerCapture);
        document.removeEventListener("pointercancel", onPointerCancel);
        document.removeEventListener("click", onClick, true);
        document.removeEventListener("contextmenu", onContextMenu);
        released();
    },
};
// Keep the shared session alive until its explicit disposer runs.
await done;
