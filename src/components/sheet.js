const dialog = document.getElementById(__DIALOG__);
const handle = document.getElementById(__HANDLE__);
if (dialog && handle) {
    const listeners = new AbortController();
    const opts = { signal: listeners.signal };
    // Read on each gesture, so a sheet whose detents change keeps up.
    const readDetents = () => JSON.parse(dialog.dataset.detents || "[]");
    let detents = readDetents();
    const DISMISS = __DISMISS__;
    let dragging = false;
    let dragged = false;
    let startY = 0;
    let deltaY = 0;
    let startHeight = 0;
    let fraction = 1;

    handle.addEventListener("pointerdown", (event) => {
        dragging = true;
        dragged = false;
        detents = readDetents();
        startY = event.clientY;
        deltaY = 0;
        startHeight = dialog.getBoundingClientRect().height;
        fraction = parseFloat(dialog.style.getPropertyValue("--g3-sheet-detent")) || 1;
        dialog.dataset.dragging = "true";
        try { handle.setPointerCapture(event.pointerId); } catch (error) {}
        event.preventDefault();
    }, opts);

    handle.addEventListener("pointermove", (event) => {
        if (!dragging) return;
        deltaY = event.clientY - startY;
        if (Math.abs(deltaY) > 4) dragged = true;
        if (detents.length > 0) {
            dialog.style.height = Math.max(0, startHeight - deltaY) + "px";
        } else {
            dialog.style.setProperty("--g3-sheet-drag-y", Math.max(0, deltaY) + "px");
        }
        event.preventDefault();
    }, opts);

    const end = () => {
        if (!dragging) return;
        dragging = false;
        if (!dragged) {
            delete dialog.dataset.dragging;
            dialog.style.removeProperty("height");
            dialog.style.setProperty("--g3-sheet-drag-y", "0px");
            return;
        }
        if (detents.length === 0) {
            delete dialog.dataset.dragging;
            dialog.style.setProperty("--g3-sheet-drag-y", "0px");
            if (deltaY > DISMISS) dioxus.send(-1);
            return;
        }
        const ceiling = startHeight / fraction;
        const height = startHeight - deltaY;
        const lowest = detents[0] * ceiling;
        if (height < lowest - DISMISS) {
            delete dialog.dataset.dragging;
            dialog.style.removeProperty("height");
            dioxus.send(-1);
            return;
        }
        let nearest = 0;
        detents.forEach((detent, index) => {
            if (Math.abs(detent * ceiling - height) < Math.abs(detents[nearest] * ceiling - height)) {
                nearest = index;
            }
        });
        // Select the CSS-sized destination while retaining the temporary pixel
        // height from the drag. Re-enable transitions, flush that starting
        // geometry, and only then release the pixel height so the sheet settles
        // smoothly while its visible and hit-test edges stay together.
        dialog.style.setProperty("--g3-sheet-detent", detents[nearest]);
        dialog.style.setProperty("--g3-sheet-drag-y", "0px");
        delete dialog.dataset.dragging;
        dialog.getBoundingClientRect();
        dialog.style.removeProperty("height");
        dioxus.send(nearest);
    };
    handle.addEventListener("pointerup", end, opts);
    const cancel = () => { dragging = false; dragged = false; delete dialog.dataset.dragging; dialog.style.removeProperty("height"); dialog.style.setProperty("--g3-sheet-drag-y", "0px"); };
    handle.addEventListener("pointercancel", cancel, opts);
    window.addEventListener("blur", cancel, opts);

    // Keyboard and plain taps: cycle detents, or close a sheet without any.
    handle.addEventListener("click", (event) => {
        if (dragged) {
            dragged = false;
            event.preventDefault();
            return;
        }
        dioxus.send(readDetents().length > 0 ? -2 : -1);
    }, opts);
    try { await dioxus.recv(); } finally { listeners.abort(); cancel(); }
}
