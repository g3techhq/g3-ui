// Marks every Img on the page loaded or failed from inside the webview.
//
// On native renderers each Rust event handler is a synchronous request from
// the webview to Rust, and the page cannot paint until it returns. A list of
// images firing one `load` each would stall a page transition, so the
// wrapper's `data-state` is set here, and the stylesheet does the rest.
// `load` and `error` do not bubble, so they are heard while capturing.
const key = Symbol.for("g3-ui.img");
window[key]?.dispose?.();
const listeners = new AbortController();

const mark = (img, state) => {
    const box = img.parentElement;
    if (box?.classList.contains("g3-img") && box.dataset.state !== state) box.dataset.state = state;
};

const settled = (state) => (event) => {
    if (event.target instanceof HTMLImageElement) mark(event.target, state);
};
document.addEventListener("load", settled("loaded"), { capture: true, signal: listeners.signal });
document.addEventListener("error", settled("error"), { capture: true, signal: listeners.signal });

// Images that settled before this script ran, such as a server-rendered
// page's. A lazy image that has not started has no `currentSrc` yet, and
// `decode` tells a broken image from one without an intrinsic size.
for (const img of document.querySelectorAll(".g3-img > img")) {
    if (!img.complete || !img.currentSrc) continue;
    if (img.naturalWidth > 0) mark(img, "loaded");
    else img.decode().then(() => mark(img, "loaded"), () => img.isConnected && img.complete && mark(img, "error"));
}

let released;
const done = new Promise((resolve) => { released = resolve; });
window[key] = { dispose() { listeners.abort(); released(); } };
await done;
