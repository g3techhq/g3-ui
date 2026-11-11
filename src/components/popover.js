const el = document.getElementById(__ID__);
if (el) {
    const base = __PLACEMENT__;
    let cap = null;
    const fit = (event) => {
    el.setAttribute("data-placement", base);
    const anchor = el.closest(".g3-popover-anchor");
    const app = el.closest(".g3-app");
    const appBox = app ? app.getBoundingClientRect() : { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    // Compact select sheets keep their bottom-sheet geometry. Every anchored
    // panel uses viewport coordinates; the top-layer wrapper then guarantees
    // that a scroll pane or transformed route cannot clip it.
    const sheet = el.classList.contains("g3-popover-sheet");
    const rem = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
    const compactSheet = sheet && appBox.right - appBox.left < 48 * rem;
    if (sheet) el.dataset.g3SheetLayout = compactSheet ? "sheet" : "popover";
    el.toggleAttribute("data-g3-fixed-position", !compactSheet);
    if (anchor && !compactSheet) {
        // The stylesheet's own height cap, read while no room limit is set.
        // Lifting the limit to read it would lay the panel out taller and
        // cut the list's scroll position, so the limit is lifted only on the
        // first fit and when the window resizes, which can change the cap.
        if (cap === null || event?.type === "resize") el.style.removeProperty("--g3-popover-room");
        const style = getComputedStyle(el);
        if (!el.style.getPropertyValue("--g3-popover-room")) cap = parseFloat(style.maxHeight) || Infinity;
        const anchorBox = anchor.getBoundingClientRect();
        el.style.setProperty("--g3-popover-anchor-width", `${anchorBox.width}px`);
        void el.offsetWidth;
        const bounds = {
            left: Math.max(appBox.left, 0) + 8,
            right: Math.min(appBox.right, innerWidth) - 8,
            top: Math.max(appBox.top, 0) + 8,
            bottom: Math.min(appBox.bottom, innerHeight) - 8,
        };
        // Layout sizes, not the drawn box: the entrance animation scales the
        // panel, so measuring the drawn box placed it for a smaller panel and
        // it jumped on the next scroll. The height is what the panel wants,
        // whatever room limit it has now.
        const borders = parseFloat(style.borderTopWidth) + parseFloat(style.borderBottomWidth);
        const wanted = Math.min(el.scrollHeight + borders, cap);
        const menuBox = { width: el.offsetWidth, height: wanted };
        let [vertical, horizontal] = base.split("-");
        const rtl = style.direction === "rtl";
        const leftFor = (side) => (side === "start") !== rtl
            ? anchorBox.left
            : anchorBox.right - menuBox.width;
        const topFor = (side) => side === "bottom"
            ? anchorBox.bottom + 6
            : anchorBox.top - menuBox.height - 6;
        let left = leftFor(horizontal);
        if (left < bounds.left || left + menuBox.width > bounds.right) {
            horizontal = horizontal === "start" ? "end" : "start";
            left = leftFor(horizontal);
        }
        // Opens on its own side if it fits, else on the other side if it fits
        // there or there is more room there. Too tall for either, it scrolls
        // within that room rather than covering its trigger, down to 10rem.
        const room = {
            bottom: bounds.bottom - anchorBox.bottom - 6,
            top: anchorBox.top - 6 - bounds.top,
        };
        const other = vertical === "bottom" ? "top" : "bottom";
        if (wanted > room[vertical] && (wanted <= room[other] || room[other] > room[vertical])) {
            vertical = other;
        }
        menuBox.height = Math.min(wanted, Math.max(room[vertical], 10 * rem));
        if (menuBox.height < wanted) {
            el.style.setProperty("--g3-popover-room", `${menuBox.height}px`);
        } else {
            el.style.removeProperty("--g3-popover-room");
        }
        const top = topFor(vertical);
        el.style.setProperty("--g3-popover-fixed-left", `${Math.max(bounds.left, Math.min(left, bounds.right - menuBox.width))}px`);
        el.style.setProperty("--g3-popover-fixed-top", `${Math.max(bounds.top, Math.min(top, bounds.bottom - menuBox.height))}px`);
        el.setAttribute("data-placement", `${vertical}-${horizontal}`);
    } else {
        el.style.removeProperty("--g3-popover-room");
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
