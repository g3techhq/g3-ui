// Shared top-layer ordering. All blocking surfaces follow presentation order;
// nonblocking toasts remain above them, including toasts already showing.
//
// The top layer escapes every ancestor, including an AppWrapper that does not
// fill the window (a device preview, an embedded app). While such a layer is
// open it is confined to its app's box, so its scrim and sheet stay inside the
// app as they would in a full-window app.
const layerKey = Symbol.for("g3-ui.layers");
if (!window[layerKey]) {
    const entries = new Map();
    // The radius of whatever clips the app, so the confined layer follows its
    // rounded corners. Looks a few levels up for a clipping box that matches
    // the app's own.
    const innerRadius = (app, box) => {
        let node = app;
        for (let depth = 0; node && depth < 4; depth++, node = node.parentElement) {
            const style = getComputedStyle(node);
            if (style.overflowX === "visible") continue;
            const radius = parseFloat(style.borderTopLeftRadius) || 0;
            if (radius <= 0) continue;
            const outer = node.getBoundingClientRect();
            const top = parseFloat(style.borderTopWidth) || 0;
            const left = parseFloat(style.borderLeftWidth) || 0;
            const right = parseFloat(style.borderRightWidth) || 0;
            const bottom = parseFloat(style.borderBottomWidth) || 0;
            const near = (a, b) => Math.abs(a - b) <= 2;
            if (near(outer.left + left, box.left) && near(outer.top + top, box.top)
                && near(outer.right - right, box.right) && near(outer.bottom - bottom, box.bottom)) {
                return Math.max(0, radius - Math.max(top, left));
            }
            return 0;
        }
        return 0;
    };
    // The part of the window the user can see, in layout-viewport coordinates:
    // the visual viewport where there is one (pinch zoom, a docked keyboard).
    const visible = () => {
        const vv = window.visualViewport;
        if (vv) return { left: vv.offsetLeft, top: vv.offsetTop, right: vv.offsetLeft + vv.width, bottom: vv.offsetTop + vv.height };
        return { left: 0, top: 0, right: document.documentElement.clientWidth, bottom: window.innerHeight };
    };
    const confine = (entry) => {
        const el = entry.el;
        const app = el.parentElement?.closest(".g3-app");
        const box = app?.getBoundingClientRect();
        const width = document.documentElement.clientWidth;
        const height = window.innerHeight;
        const fills = !box || (box.left <= 0.5 && box.top <= 0.5
            && box.right >= width - 0.5 && box.bottom >= height - 0.5);
        if (fills) {
            el.removeAttribute("data-g3-confined");
            return;
        }
        // An app that starts below other content and runs past the window
        // keeps its layer on screen, so bottom sheets and toasts stay reachable.
        const view = visible();
        const left = Math.max(box.left, view.left);
        const top = Math.max(box.top, view.top);
        const right = Math.min(box.right, view.right);
        const bottom = Math.min(box.bottom, view.bottom);
        const clipped = left > box.left + 0.5 || top > box.top + 0.5
            || right < box.right - 0.5 || bottom < box.bottom - 0.5;
        el.setAttribute("data-g3-confined", "");
        el.style.setProperty("--g3-layer-top", `${top}px`);
        el.style.setProperty("--g3-layer-left", `${left}px`);
        el.style.setProperty("--g3-layer-width", `${Math.max(0, right - left)}px`);
        el.style.setProperty("--g3-layer-height", `${Math.max(0, bottom - top)}px`);
        // A clipped edge is the window's, not the app's rounded corner.
        el.style.setProperty("--g3-layer-radius", `${clipped ? 0 : innerRadius(app, box)}px`);
    };
    // Follows the app while the layer shows: the page around a device preview
    // can scroll or resize under an open sheet.
    const track = (entry) => {
        if (entry.tracking) return;
        const update = () => confine(entry);
        const listeners = new AbortController();
        const options = { capture: true, passive: true, signal: listeners.signal };
        document.addEventListener("scroll", update, options);
        window.addEventListener("resize", update, { signal: listeners.signal });
        window.visualViewport?.addEventListener("resize", update, { signal: listeners.signal });
        window.visualViewport?.addEventListener("scroll", update, { signal: listeners.signal });
        const observer = new ResizeObserver(update);
        const app = entry.el.parentElement?.closest(".g3-app");
        if (app) observer.observe(app);
        entry.tracking = () => { listeners.abort(); observer.disconnect(); };
    };
    const untrack = (entry) => {
        entry.tracking?.();
        entry.tracking = null;
    };
    window[layerKey] = {
        present(id, open, delay, priority) {
            const el = document.getElementById(id);
            if (!el) { window[layerKey].remove(id); return; }
            let entry = entries.get(id);
            if (!entry || entry.el !== el) {
                if (entry) { clearTimeout(entry.timer); untrack(entry); }
                entry = { el, open: false, priority, timer: 0, tracking: null };
                entries.set(id, entry);
            }
            clearTimeout(entry.timer);
            const wasOpen = entry.open;
            entry.open = open;
            entry.priority = priority;
            if (typeof el.showPopover !== 'function') { el.removeAttribute('popover'); return; }
            if (open) {
                confine(entry);
                track(entry);
                if (!wasOpen && el.matches(':popover-open')) el.hidePopover();
                if (!el.matches(':popover-open')) el.showPopover();
                const focused = document.activeElement;
                // Re-present higher-priority surfaces after the newly opened
                // surface. Reopening ordinary overlays also advances them.
                for (const [otherId, other] of entries) {
                    if (!other.el.isConnected) { clearTimeout(other.timer); untrack(other); entries.delete(otherId); continue; }
                    if (other.open && other.priority > priority && other.el.matches(':popover-open')) {
                        other.el.hidePopover(); other.el.showPopover();
                    }
                }
                if (focused?.isConnected && document.activeElement !== focused) focused.focus({ preventScroll: true });
            } else if (el.matches(':popover-open')) {
                entry.timer = setTimeout(() => {
                    if (!entry.open && el.isConnected && el.matches(':popover-open')) el.hidePopover();
                    if (!entry.open) { untrack(entry); entries.delete(id); }
                }, delay);
            } else {
                untrack(entry);
            }
        },
        remove(id) {
            const entry = entries.get(id);
            if (!entry) return;
            clearTimeout(entry.timer);
            untrack(entry);
            if (typeof entry.el.hidePopover === 'function' && entry.el.isConnected && entry.el.matches(':popover-open')) entry.el.hidePopover();
            entries.delete(id);
        },
    };
}
