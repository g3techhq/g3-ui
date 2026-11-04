// Keeps an open bottom sheet above the on-screen keyboard. iOS lays the
// keyboard over the page without resizing it, so a sheet pinned to the
// window's bottom would sit under the keyboard with the field being typed in.
// The visual viewport is the part left visible: the sheet's layer is told how
// far that ends above the window's bottom and how tall it is, the stylesheet
// lifts and caps the sheet by them, and the focused field scrolls into view.
// Android resizes the page for its keyboard instead, so there this measures
// nothing and changes nothing.
const sheet = document.getElementById(__SHEET__);
const viewport = window.visualViewport;
if (sheet && viewport) {
    const listeners = new AbortController();
    const options = { signal: listeners.signal };
    const layer = sheet.closest(".g3-overlay-layer") || sheet;

    // Keeps the focused field inside the sheet's scrolling content.
    const reveal = () => {
        const focused = document.activeElement;
        if (!focused || focused === sheet || !sheet.contains(focused)) return;
        const content = focused.closest(".g3-sheet-content");
        if (!content) return;
        const field = focused.getBoundingClientRect();
        const box = content.getBoundingClientRect();
        const margin = 12;
        if (field.bottom > box.bottom - margin) {
            content.scrollTop += field.bottom - box.bottom + margin;
        } else if (field.top < box.top + margin) {
            content.scrollTop -= box.top + margin - field.top;
        }
    };

    const update = () => {
        // A layer confined to an app smaller than the window already ends at
        // the visible bottom (layers.js); pinch zoom is not a keyboard.
        const confined = layer.hasAttribute("data-g3-confined");
        const zoomed = Math.abs((viewport.scale || 1) - 1) > 0.01;
        const inset = confined || zoomed
            ? 0
            : Math.round(window.innerHeight - viewport.height - viewport.offsetTop);
        if (inset > 0) {
            layer.style.setProperty("--g3-keyboard-inset", `${inset}px`);
            layer.style.setProperty("--g3-visible-height", `${Math.round(viewport.height)}px`);
            layer.setAttribute("data-g3-keyboard", "");
            reveal();
        } else if (layer.hasAttribute("data-g3-keyboard")) {
            clear();
        }
    };

    const clear = () => {
        layer.removeAttribute("data-g3-keyboard");
        layer.style.removeProperty("--g3-keyboard-inset");
        layer.style.removeProperty("--g3-visible-height");
    };

    viewport.addEventListener("resize", update, options);
    viewport.addEventListener("scroll", update, options);
    // Moving to another field while the keyboard stays up resizes nothing.
    sheet.addEventListener("focusin", update, options);
    update();
    try { await dioxus.recv(); } finally { listeners.abort(); clear(); }
}
