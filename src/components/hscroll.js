const strip = document.getElementById(__ID__);
if (strip) {
    const listeners = new AbortController();
    const opts = { signal: listeners.signal };
    const rtl = getComputedStyle(strip).direction === "rtl";
    const position = () => rtl ? -strip.scrollLeft : strip.scrollLeft;
    const setPosition = (value) => { strip.scrollLeft = rtl ? -value : value; };
    const LINE = 16;
    const edges = () => {
        const max = strip.scrollWidth - strip.clientWidth;
        strip.dataset.overflowStart = String(position() > 1);
        strip.dataset.overflowEnd = String(position() < max - 1);
    };
    const calm = window.matchMedia("(prefers-reduced-motion: reduce)");
    const reveal = (animate) => {
        const smooth = animate && !calm.matches;
        const selected = strip.querySelector("[aria-checked=true], [aria-selected=true]");
        if (!selected) return;
        // The strip is positioned, so this is already relative to it.
        const box = selected.getBoundingClientRect();
        const frame = strip.getBoundingClientRect();
        const left = position() + (rtl ? frame.right - box.right : box.left - frame.left);
        const right = left + selected.offsetWidth;
        if (left < position() + 16) {
            strip.scrollTo({ left: (left - 16) * (rtl ? -1 : 1), behavior: smooth ? "smooth" : "auto" });
        } else if (right > position() + strip.clientWidth - 16) {
            strip.scrollTo({ left: (right - strip.clientWidth + 16) * (rtl ? -1 : 1), behavior: smooth ? "smooth" : "auto" });
        }
    };
    let down = false;
    let dragged = false;
    let startX = 0;
    let startLeft = 0;
    let lastX = 0;
    let lastTime = 0;
    let velocity = 0;
    let coastFrame = 0;
    let wheelFrame = 0;
    let wheelTarget = 0;
    let wheelTimer;
    let wheelSettled = false;
    const clamp = (value) => Math.max(0, Math.min(strip.scrollWidth - strip.clientWidth, value));
    const stopCoasting = () => {
        if (coastFrame) cancelAnimationFrame(coastFrame);
        coastFrame = 0;
    };
    const stopWheel = () => {
        if (wheelFrame) cancelAnimationFrame(wheelFrame);
        clearTimeout(wheelTimer);
        wheelFrame = 0;
    };
    // A mouse or wheel scrolls freely and never snaps: settling on an item
    // after the pointer let go read as the row jumping. Snapping is left to
    // the stylesheet, which applies it to touch alone.
    //
    // A released drag carries on at the speed it was let go, slowing under
    // friction, so the motion never changes speed abruptly.
    const coast = (speed) => {
        stopCoasting();
        if (calm.matches || Math.abs(speed) < 0.05) {
            delete strip.dataset.dragging;
            return;
        }
        let last = performance.now();
        const step = (now) => {
            const elapsed = Math.min(48, now - last);
            last = now;
            speed *= Math.pow(0.994, elapsed);
            const before = position();
            setPosition(clamp(before + speed * elapsed));
            // Stopped by friction, or by an end of the row.
            if (Math.abs(speed) < 0.02 || position() === before) {
                coastFrame = 0;
                delete strip.dataset.dragging;
                return;
            }
            coastFrame = requestAnimationFrame(step);
        };
        coastFrame = requestAnimationFrame(step);
    };
    // Eases toward `wheelTarget`, covering a share of the way each frame. The
    // step is at least a pixel, since a smaller one can round away to nothing
    // and leave the glide running forever.
    const glide = () => {
        const distance = wheelTarget - position();
        if (Math.abs(distance) <= 1) {
            setPosition(wheelTarget);
            wheelFrame = 0;
            if (wheelSettled) delete strip.dataset.dragging;
            return;
        }
        setPosition(position() + Math.sign(distance) * Math.max(1, Math.abs(distance) * 0.2));
        wheelFrame = requestAnimationFrame(glide);
    };
    strip.addEventListener("pointerdown", (event) => {
        if (event.pointerType !== "mouse" || event.button !== 0) return;
        stopCoasting();
        stopWheel();
        down = true;
        dragged = false;
        startX = event.clientX;
        startLeft = position();
        lastX = event.clientX;
        lastTime = performance.now();
        velocity = 0;
    }, opts);
    window.addEventListener("pointermove", (event) => {
        if (!down) return;
        const dx = event.clientX - startX;
        if (!dragged && Math.abs(dx) < 5) return;
        dragged = true;
        strip.dataset.dragging = "true";
        event.preventDefault();
        setPosition(startLeft - dx * (rtl ? -1 : 1));
        const now = performance.now();
        const elapsed = Math.max(1, now - lastTime);
        const instant = (lastX - event.clientX) / elapsed * (rtl ? -1 : 1);
        velocity = velocity * 0.72 + instant * 0.28;
        lastX = event.clientX;
        lastTime = now;
    }, opts);
    window.addEventListener("pointerup", () => {
        if (!down) return;
        down = false;
        if (!dragged) {
            delete strip.dataset.dragging;
            return;
        }
        // A pointer held still before letting go has no speed left, even
        // though the last move it reported did.
        coast(performance.now() - lastTime > 80 ? 0 : velocity);
    }, opts);
    const cancel = () => { down = false; dragged = false; stopCoasting(); stopWheel(); delete strip.dataset.dragging; };
    window.addEventListener("pointercancel", cancel, opts);
    window.addEventListener("blur", cancel, opts);
    // A drag must not also press whatever it ended on.
    strip.addEventListener("click", (event) => {
        if (dragged) {
            dragged = false;
            event.preventDefault();
            event.stopPropagation();
        }
    }, { capture: true, signal: listeners.signal });
    strip.addEventListener("wheel", (event) => {
        const raw = Math.abs(event.deltaY) > Math.abs(event.deltaX)
            ? event.deltaY
            : event.deltaX * (rtl ? -1 : 1);
        const max = strip.scrollWidth - strip.clientWidth;
        if (max <= 0) return;
        const delta = event.deltaMode === 1 ? raw * LINE
            : event.deltaMode === 2 ? raw * strip.clientWidth * 0.9
            : raw;
        // Against a stop, the page keeps the scroll.
        if ((delta < 0 && position() <= 0) || (delta > 0 && position() >= max - 1)) return;
        event.preventDefault();
        stopCoasting();
        strip.dataset.dragging = "true";
        if (!wheelFrame) wheelTarget = position();
        wheelTarget = clamp(wheelTarget + delta);
        wheelSettled = false;
        if (!wheelFrame) wheelFrame = requestAnimationFrame(glide);
        clearTimeout(wheelTimer);
        // The glide finishes where the wheel sent it; only then does the row
        // stop counting as scrolled by hand.
        wheelTimer = setTimeout(() => {
            wheelSettled = true;
            if (!wheelFrame) wheelFrame = requestAnimationFrame(glide);
        }, 120);
    }, { passive: false, signal: listeners.signal });
    // A segment or tab strip slides its indicator under the selected button.
    // Buttons keep their natural widths here, so the stylesheet cannot work
    // out where that is: this records it. The first placement is not
    // animated; until then the selected button highlights itself.
    const indicator = strip.matches(".g3-segment-scrollable");
    const place = () => {
        if (!indicator) return;
        const selected = strip.querySelector(":scope > [aria-checked=true], :scope > [aria-selected=true]");
        if (!selected) { delete strip.dataset.indicatorPlaced; return; }
        strip.style.setProperty("--g3-segment-indicator-left", `${selected.offsetLeft}px`);
        strip.style.setProperty("--g3-segment-indicator-width", `${selected.offsetWidth}px`);
        if (!strip.hasAttribute("data-indicator-placed")) {
            void strip.offsetWidth;
            strip.dataset.indicatorPlaced = "";
        }
    };
    strip.addEventListener("scroll", edges, { passive: true, signal: listeners.signal });
    const resize = new ResizeObserver(() => { edges(); place(); });
    resize.observe(strip);
    const observeButtons = () => { for (const button of strip.children) resize.observe(button); };
    observeButtons();
    const mutations = new MutationObserver((records) => {
        if (records.some((record) => record.type === "childList")) observeButtons();
        place(); reveal(true); edges();
    });
    mutations.observe(strip, { subtree: true, childList: true, attributes: true, attributeFilter: ["aria-checked", "aria-selected"] });
    place();
    reveal(false);
    edges();
    try { await dioxus.recv(); } finally {
        listeners.abort(); resize.disconnect(); mutations.disconnect();
        stopCoasting(); stopWheel(); delete strip.dataset.dragging;
    }
}
