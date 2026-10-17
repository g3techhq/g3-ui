const sentinel = document.getElementById(__ID__);
if (sentinel) {
    const root = sentinel.closest(".g3-content-scroll");
    const canLoad = () => sentinel.dataset.loading !== "true"
        && sentinel.dataset.complete !== "true";
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
            dioxus.send(true);
        }
    }, { root, rootMargin: "0px 0px __MARGIN__px 0px" });
    observer.observe(sentinel);
    const stateObserver = new MutationObserver(rearm);
    stateObserver.observe(sentinel, {
        attributes: true,
        attributeFilter: ["data-loading", "data-complete"],
    });
    // A synchronous page (or a very fast cached request) may never toggle
    // `loading`. Re-arm when the surrounding list changes so a sentinel that
    // remains visible can request enough pages to fill the viewport.
    const list = sentinel.parentElement;
    if (list) {
        listObserver = new MutationObserver(rearm);
        listObserver.observe(list, { childList: true, subtree: true });
    }
    try { await dioxus.recv(); } finally {
        observer.disconnect(); stateObserver.disconnect();
        listObserver?.disconnect();
        delete sentinel.dataset.g3Pending;
    }
}
