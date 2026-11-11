const sentinel = document.getElementById(__ID__);
if (sentinel) {
    const top = __TOP__;
    // The nearest ancestor that scrolls: a page's content, or a table that
    // scrolls inside its own frame.
    const scroller = () => {
        for (let el = sentinel.parentElement; el; el = el.parentElement) {
            const overflow = getComputedStyle(el).overflowY;
            if (overflow === "auto" || overflow === "scroll" || overflow === "overlay") return el;
        }
        return null;
    };
    const root = scroller();
    const canLoad = () => sentinel.dataset.loading !== "true"
        && sentinel.dataset.complete !== "true";
    // Loading above the view would push what the user is reading down, so
    // keep their distance from the bottom while rows arrive.
    let fromBottom = null;
    const keepPlace = () => {
        if (fromBottom === null || !root) return;
        root.scrollTop = root.scrollHeight - fromBottom;
    };
    let observer;
    let listObserver;
    const rearm = () => {
        if (!sentinel.isConnected || !canLoad()) return;
        sentinel.dataset.g3Pending = "false";
        observer.unobserve(sentinel);
        observer.observe(sentinel);
    };
    observer = new IntersectionObserver((entries) => {
        if (entries.some((entry) => entry.isIntersecting)
            && canLoad()
            && sentinel.dataset.g3Pending !== "true") {
            sentinel.dataset.g3Pending = "true";
            if (top && root) {
                fromBottom = root.scrollHeight - root.scrollTop;
                root.style.overflowAnchor = "none";
            }
            dioxus.send(true);
        }
    }, { root, rootMargin: top ? "__MARGIN__px 0px 0px 0px" : "0px 0px __MARGIN__px 0px" });
    observer.observe(sentinel);
    const stateObserver = new MutationObserver(() => {
        keepPlace();
        if (sentinel.dataset.loading !== "true" && fromBottom !== null) {
            // The page has landed; hold the place through the next layout.
            requestAnimationFrame(() => {
                keepPlace();
                fromBottom = null;
                if (root) root.style.overflowAnchor = "";
                rearm();
            });
            return;
        }
        rearm();
    });
    stateObserver.observe(sentinel, {
        attributes: true,
        attributeFilter: ["data-loading", "data-complete"],
    });
    // A synchronous page (or a very fast cached request) may never toggle
    // `loading`. Re-arm when the surrounding list changes so a sentinel that
    // remains visible can request enough pages to fill the viewport.
    const list = sentinel.parentElement;
    if (list) {
        listObserver = new MutationObserver(() => { keepPlace(); rearm(); });
        listObserver.observe(list, { childList: true, subtree: true });
    }
    try { await dioxus.recv(); } finally {
        observer.disconnect(); stateObserver.disconnect();
        listObserver?.disconnect();
        if (root) root.style.overflowAnchor = "";
        delete sentinel.dataset.g3Pending;
    }
}
