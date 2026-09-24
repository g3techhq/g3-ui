// Drives every ReorderList on the page from inside the webview, so a dragged
// item follows the pointer without waiting on Rust (see gesture.rs). Rust
// hears one thing per drag: the item's old and new positions, if it moved.
// A list's `data-layout` says whether its items are rows or grid cells.
const key = Symbol.for("g3-ui.reorder");
window[key]?.dispose?.();

// Longest a dropped row waits for the caller to reorder the list before it
// gives up and slides back.
const SETTLE_MS = 400;

// How long a dropped item takes to glide from the pointer into its slot;
// matches `--g3-reorder-drop` in the stylesheet.
const DROP_MS = 200;

let active = null;

// The list's own rows, in order, skipping any in a list nested inside it.
const rowsOf = (list) => [...list.querySelectorAll(".g3-reorder-item")]
    .filter((row) => row.closest(".g3-reorder") === list)
    .sort((a, b) => Number(a.dataset.index) - Number(b.dataset.index));

// Where an item dragged from `from` would land, given the pointer and the
// middle of every item before the drag began.
//
// Rows: past the middle of each row the pointer has crossed, so rows of any
// height work. Grid: the cell whose middle is nearest, wherever it wraps.
const targetIndex = (drag, x, y) => {
    if (drag.grid) {
        let target = drag.from;
        let nearest = Infinity;
        drag.centers.forEach(([cx, cy], index) => {
            const distance = (cx - x) ** 2 + (cy - y) ** 2;
            if (distance < nearest) {
                nearest = distance;
                target = index;
            }
        });
        return target;
    }
    let target = drag.from;
    drag.centers.forEach(([, middle], index) => {
        if (index < drag.from && y < middle) target = Math.min(target, index);
        if (index > drag.from && y > middle) target = Math.max(target, index);
    });
    return target;
};

// How far an item that is not being dragged moves aside for the one that
// is: toward the place the dragged item left, when the drag passes over it.
// A row moves by the dragged row's height; a cell steps into the next slot,
// which may be at the other end of the row above or below.
const shift = (index, drag) => {
    let slot;
    if (drag.from < drag.target && index > drag.from && index <= drag.target) slot = index - 1;
    else if (drag.target < drag.from && index >= drag.target && index < drag.from) slot = index + 1;
    else return [0, 0];
    if (!drag.grid) return [0, slot < index ? -drag.height : drag.height];
    const [ox, oy] = drag.centers[index];
    const [sx, sy] = drag.centers[slot];
    return [sx - ox, sy - oy];
};

const paint = (drag) => {
    drag.rows.forEach((row, index) => {
        const [x, y] = index === drag.from ? [drag.grid ? drag.dx : 0, drag.dy] : shift(index, drag);
        if (x || y) row.style.transform = `translate3d(${x}px, ${y}px, 0)`;
        else row.style.removeProperty("transform");
    });
};

const clear = (drag) => {
    for (const row of drag.rows) {
        row.style.removeProperty("transform");
        delete row.dataset.dragging;
        delete row.dataset.settling;
    }
    delete drag.list.dataset.dragging;
};

// Where the dragged item's slot is, measured from where it started.
const slotOffset = (drag) => {
    if (drag.grid) {
        const [ox, oy] = drag.centers[drag.from];
        const [tx, ty] = drag.centers[drag.target];
        return [tx - ox, ty - oy];
    }
    const from = drag.rects[drag.from];
    const to = drag.rects[drag.target];
    return [0, drag.target > drag.from ? to.bottom - from.bottom : to.top - from.top];
};

// Glides the dragged item from the pointer to `[x, y]`, then calls `then`.
// Without it, a dropped item jumped from under the pointer into its slot.
const glide = (drag, [x, y], then) => {
    const row = drag.rows[drag.from];
    const still = (!drag.grid || drag.dx === x) && drag.dy === y;
    if (still || matchMedia("(prefers-reduced-motion: reduce)").matches) return then();
    let finished = false;
    const finish = () => {
        if (finished) return;
        finished = true;
        row.removeEventListener("transitionend", onEnd);
        clearTimeout(timer);
        then();
    };
    const onEnd = (event) => {
        if (event.target === row && event.propertyName === "transform") finish();
    };
    row.addEventListener("transitionend", onEnd);
    // In case the transition never runs, as when the row is hidden.
    const timer = setTimeout(finish, DROP_MS + 80);
    row.dataset.settling = "true";
    if (x || y) row.style.transform = `translate3d(${x}px, ${y}px, 0)`;
    else row.style.removeProperty("transform");
};

// Capturing, and stopped here, so the press is the handle's alone: neither a
// swipe row around it nor anything in Rust hears it.
const onPointerDown = (event) => {
    const handle = event.target instanceof Element && event.target.closest(".g3-reorder-handle");
    const list = handle?.closest(".g3-reorder");
    const row = handle?.closest(".g3-reorder-item");
    // A list still settling its last drop takes no new drag until it has.
    if (!list || !row || active || list.dataset.dragging === "true"
        || handle.getAttribute("aria-disabled") === "true") return;
    event.preventDefault();
    event.stopPropagation();
    const rows = rowsOf(list);
    const from = rows.indexOf(row);
    if (from < 0) return;
    // Measured as the drag begins, so items of any size make room for each
    // other correctly.
    const rects = rows.map((each) => each.getBoundingClientRect());
    active = {
        list,
        rows,
        grid: list.dataset.layout === "grid",
        pointerId: event.pointerId,
        from,
        target: from,
        startX: event.clientX,
        startY: event.clientY,
        dx: 0,
        dy: 0,
        height: rects[from].height,
        rects,
        centers: rects.map((rect) => [rect.left + rect.width / 2, rect.top + rect.height / 2]),
    };
    try { handle.setPointerCapture(event.pointerId); } catch (error) {}
    list.dataset.dragging = "true";
    row.dataset.dragging = "true";
};

const onPointerMove = (event) => {
    const drag = active;
    if (!drag || event.pointerId !== drag.pointerId) return;
    drag.dx = event.clientX - drag.startX;
    drag.dy = event.clientY - drag.startY;
    drag.target = targetIndex(drag, event.clientX, event.clientY);
    paint(drag);
};

const onPointerUp = (event) => {
    const drag = active;
    if (!drag || event.pointerId !== drag.pointerId) return;
    active = null;
    if (drag.target === drag.from) return putBack(drag);
    glide(drag, slotOffset(drag), () => {
        // Hold every row where the drop left it until the caller's reorder
        // reaches the page, then let go in the same frame, so the dropped
        // row never flashes back to where it started.
        let settled = false;
        const settle = () => {
            if (settled) return;
            settled = true;
            observer.disconnect();
            clearTimeout(timer);
            clear(drag);
        };
        const observer = new MutationObserver(settle);
        observer.observe(drag.list, {
            subtree: true,
            childList: true,
            attributes: true,
            attributeFilter: ["data-index"],
        });
        const timer = setTimeout(settle, SETTLE_MS);
        dioxus.send([drag.list.id, "reorder", [drag.from, drag.target]]);
    });
};

// Slides everything back to where the drag began.
const putBack = (drag) => {
    drag.target = drag.from;
    drag.rows.forEach((row, index) => {
        if (index !== drag.from) row.style.removeProperty("transform");
    });
    glide(drag, [0, 0], () => clear(drag));
};

const onPointerCancel = (event) => {
    const drag = active;
    if (!drag || event.pointerId !== drag.pointerId) return;
    active = null;
    putBack(drag);
};

document.addEventListener("pointerdown", onPointerDown, true);
document.addEventListener("pointermove", onPointerMove);
document.addEventListener("pointerup", onPointerUp);
document.addEventListener("pointercancel", onPointerCancel);

let released;
const done = new Promise((resolve) => {
    released = resolve;
});
window[key] = {
    dispose() {
        document.removeEventListener("pointerdown", onPointerDown, true);
        document.removeEventListener("pointermove", onPointerMove);
        document.removeEventListener("pointerup", onPointerUp);
        document.removeEventListener("pointercancel", onPointerCancel);
        released();
    },
};
// Native renderers close an eval's channel as soon as its script returns.
await done;
