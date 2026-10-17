const el = document.getElementById(__ID__);
if (el) {
    const base = __PLACEMENT__;
    const fit = () => {
    el.setAttribute("data-placement", base);
    const anchor = el.closest(".g3-popover-anchor");
    const app = el.closest(".g3-app");
    const appBox = app ? app.getBoundingClientRect() : { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    // Compact select sheets keep their bottom-sheet geometry. Every anchored
    // panel uses viewport coordinates; the top-layer wrapper then guarantees
    // that a scroll pane or transformed route cannot clip it.
    const sheet = el.classList.contains("g3-popover-sheet");
    const compactSheet = sheet && appBox.right - appBox.left < 48 * (parseFloat(getComputedStyle(document.documentElement).fontSize) || 16);
    if (sheet) el.dataset.g3SheetLayout = compactSheet ? "sheet" : "popover";
    el.toggleAttribute("data-g3-fixed-position", !compactSheet);
    if (anchor && !compactSheet) {
        const anchorBox = anchor.getBoundingClientRect();
        el.style.setProperty("--g3-popover-anchor-width", `${anchorBox.width}px`);
        void el.offsetWidth;
        const bounds = {
            left: Math.max(appBox.left, 0) + 8,
            right: Math.min(appBox.right, innerWidth) - 8,
            top: Math.max(appBox.top, 0) + 8,
            bottom: Math.min(appBox.bottom, innerHeight) - 8,
        };
        const menuBox = el.getBoundingClientRect();
        let [vertical, horizontal] = base.split("-");
        const rtl = getComputedStyle(el).direction === "rtl";
        const leftFor = (side) => (side === "start") !== rtl
            ? anchorBox.left
            : anchorBox.right - menuBox.width;
        const topFor = (side) => side === "bottom"
            ? anchorBox.bottom + 6
            : anchorBox.top - menuBox.height - 6;
        let left = leftFor(horizontal);
        let top = topFor(vertical);
        if (left < bounds.left || left + menuBox.width > bounds.right) {
            horizontal = horizontal === "start" ? "end" : "start";
            left = leftFor(horizontal);
        }
        if (top < bounds.top || top + menuBox.height > bounds.bottom) {
            vertical = vertical === "bottom" ? "top" : "bottom";
            top = topFor(vertical);
        }
        el.style.setProperty("--g3-popover-fixed-left", `${Math.max(bounds.left, Math.min(left, bounds.right - menuBox.width))}px`);
        el.style.setProperty("--g3-popover-fixed-top", `${Math.max(bounds.top, Math.min(top, bounds.bottom - menuBox.height))}px`);
        el.setAttribute("data-placement", `${vertical}-${horizontal}`);
    }
    };
    const listeners = new AbortController();
    const opts = { signal: listeners.signal };
    const observer = new ResizeObserver(fit);
    observer.observe(el);
    const anchor = el.closest('.g3-popover-anchor');
    if (anchor) observer.observe(anchor);
    const app = el.closest('.g3-app');
    if (app) observer.observe(app);
    window.addEventListener('resize', fit, opts);
    window.visualViewport?.addEventListener('resize', fit, opts);
    document.addEventListener('scroll', fit, { capture: true, passive: true, signal: listeners.signal });
    fit();
    try { await dioxus.recv(); } finally { listeners.abort(); observer.disconnect(); }
}
