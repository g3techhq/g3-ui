const root = document.getElementById(__ROOT__);
const label = document.getElementById(__LABEL__);
if (root) {
    const owner = {};
    root.g3RefreshSession = owner;
    const THRESHOLD = __THRESHOLD__;
    const ELASTIC = __ELASTIC__;
    const TEXT = { pull: __PULL__, release: __RELEASE__, refreshing: __REFRESHING__ };
    let dragging = false;
    let pending = false;
    let settleTimer = 0;
    let engaged = false;
    let startX = 0;
    let startY = 0;
    let pull = 0;
    const scrollParent = () => {
        let el = root.parentElement;
        while (el) {
            const overflow = getComputedStyle(el).overflowY;
            if ((overflow === "auto" || overflow === "scroll") && el.scrollHeight > el.clientHeight) {
                return el;
            }
            el = el.parentElement;
        }
        return null;
    };
    const available = () => !pending && root.dataset.refreshing !== "true" && root.dataset.disabled !== "true";
    const atTop = () => {
        const parent = scrollParent();
        return !parent || parent.scrollTop <= 0;
    };
    const setPull = (distance) => {
        pull = distance;
        root.style.setProperty("--g3-refresher-pull", distance + "px");
        root.style.setProperty("--g3-refresher-progress", Math.min(distance / THRESHOLD, 1.4));
        root.setAttribute("data-state", distance > 0 ? "pulling" : "idle");
        if (label) label.textContent = distance >= THRESHOLD ? TEXT.release : TEXT.pull;
    };
    // The script edits the label outside Dioxus, so it restores it itself once
    // the app's `refreshing` flag clears; a fast refresh may never re-render it.
    const settle = () => {
        if (root.dataset.refreshing === "true") {
            if (label) label.textContent = TEXT.refreshing;
            settleTimer = setTimeout(settle, 100);
            return;
        }
        pending = false;
        root.setAttribute("data-state", "idle");
        root.style.setProperty("--g3-refresher-progress", "0");
        if (label) label.textContent = TEXT.pull;
    };
    const down = (x, y) => {
        if (dragging || !available() || !atTop()) return;
        dragging = true;
        engaged = false;
        startX = x;
        startY = y;
    };
    const move = (x, y, event) => {
        if (!dragging) return;
        const dy = y - startY;
        // A sideways drag belongs to whatever scrolls sideways under the
        // finger, such as a chip strip.
        if (!engaged && Math.abs(x - startX) > Math.abs(dy)) {
            dragging = false;
            return;
        }
        if (dy <= 0) {
            if (engaged) {
                engaged = false;
                setPull(0);
            }
            return;
        }
        if (!engaged) {
            if (!atTop()) return;
            engaged = true;
        }
        if (event && event.cancelable) event.preventDefault();
        setPull(Math.min(dy > THRESHOLD ? THRESHOLD + (dy - THRESHOLD) * ELASTIC : dy, THRESHOLD * 2.5));
    };
    const up = () => {
        if (!dragging) return;
        dragging = false;
        if (!engaged) return;
        engaged = false;
        if (pull >= THRESHOLD) {
            root.style.setProperty("--g3-refresher-pull", "0px");
            root.style.setProperty("--g3-refresher-progress", "1");
            root.setAttribute("data-state", "refreshing");
            if (label) label.textContent = TEXT.refreshing;
            pending = true;
            dioxus.send(true);
            settleTimer = setTimeout(settle, 100);
        } else {
            setPull(0);
        }
    };
    const cancel = () => { dragging = false; engaged = false; setPull(0); };
    // A pull starts on the refresher but may end anywhere: moves and releases
    // are heard on the window, so dragging past the refresher (or out of the
    // window) still lets go instead of leaving the indicator stuck.
    const listeners = new AbortController();
    const alive = () => {
        if (root.isConnected) return true;
        listeners.abort();
        return false;
    };
    const opts = { signal: listeners.signal };
    root.addEventListener("pointerdown", (e) => { if (e.pointerType === "mouse" && e.button === 0) down(e.clientX, e.clientY); }, opts);
    window.addEventListener("pointermove", (e) => { if (e.pointerType === "mouse" && alive()) move(e.clientX, e.clientY, null); }, opts);
    window.addEventListener("pointerup", (e) => { if (e.pointerType === "mouse" && alive()) up(); }, opts);
    window.addEventListener("pointercancel", () => { if (alive()) cancel(); }, opts);
    window.addEventListener("blur", () => { if (alive()) cancel(); }, opts);
    document.addEventListener("mouseleave", () => { if (alive()) cancel(); }, opts);
    root.addEventListener("touchstart", (e) => { if (e.touches.length === 1) down(e.touches[0].clientX, e.touches[0].clientY); else cancel(); }, { passive: true, signal: listeners.signal });
    root.addEventListener("touchmove", (e) => move(e.touches[0].clientX, e.touches[0].clientY, e), { passive: false, signal: listeners.signal });
    root.addEventListener("touchend", (e) => e.touches.length ? cancel() : up(), opts);
    root.addEventListener("touchcancel", cancel, opts);
    try { await dioxus.recv(); } finally {
        listeners.abort(); clearTimeout(settleTimer);
        if (root.g3RefreshSession === owner) { cancel(); if (label) label.textContent = TEXT.pull; delete root.g3RefreshSession; }
    }
}
