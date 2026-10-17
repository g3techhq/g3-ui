const el = document.getElementById(__ID__);
if (el) {
    const listeners = new AbortController();
    const opts = { signal: listeners.signal };
    const calm = window.matchMedia("(prefers-reduced-motion: reduce)");
    const step = () => (el.querySelector(".g3-wheel-item") || {}).offsetHeight || 36;
    let quietUntil = 0;
    let timer = 0;
    const settle = () => {
        const items = [...el.querySelectorAll(".g3-wheel-item")];
        const requested = Math.max(0, Math.min(items.length - 1, Math.round(el.scrollTop / step())));
        const allowed = items.map((item, index) => ({ item, index })).filter(({item}) => item.dataset.disabled !== "true");
        if (!allowed.length) return;
        const index = allowed.reduce((best, next) => Math.abs(next.index - requested) < Math.abs(best.index - requested) ? next : best).index;
        el.scrollTo({ top: index * step(), behavior: calm.matches ? "auto" : "smooth" });
        dioxus.send(index);
    };
    el.addEventListener("scroll", () => {
        if (performance.now() < quietUntil) return;
        clearTimeout(timer);
        timer = setTimeout(settle, 140);
    }, { passive: true, signal: listeners.signal });

    // Touch and trackpads scroll natively; a mouse drags.
    // A press only becomes a drag once it moves, so a plain click still
    // reaches the item under it.
    let pressed = false;
    let dragging = false;
    let startY = 0;
    let startTop = 0;
    el.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        pressed = true;
        dragging = false;
        startY = event.clientY;
        startTop = el.scrollTop;
    }, opts);
    el.addEventListener("pointermove", (event) => {
        if (!pressed) return;
        const dy = event.clientY - startY;
        if (!dragging && Math.abs(dy) < 4) return;
        if (!dragging) {
            dragging = true;
            el.dataset.dragging = "true";
            try { el.setPointerCapture(event.pointerId); } catch (error) {}
        }
        el.scrollTop = startTop - dy;
    }, opts);
    const release = () => {
        pressed = false;
        if (!dragging) return;
        dragging = false;
        delete el.dataset.dragging;
        clearTimeout(timer);
        timer = setTimeout(settle, 60);
    };
    el.addEventListener("pointerup", release, opts);
    el.addEventListener("pointercancel", release, opts);

    try { while (true) {
        const message = await dioxus.recv();
        if (message === "dispose") break;
        const [index, animate] = message;
        const top = index * step();
        if (Math.abs(el.scrollTop - top) > 1) {
            const smooth = animate && !calm.matches;
            quietUntil = performance.now() + (smooth ? 450 : 80);
            el.scrollTo({ top, behavior: smooth ? "smooth" : "auto" });
        }
    } } finally { listeners.abort(); clearTimeout(timer); delete el.dataset.dragging; }
}
