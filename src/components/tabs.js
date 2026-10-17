// Touch recognition stays in the page. A press remains a tap until horizontal
// intent is established; nested controls and gesture owners keep their input.
const key = Symbol.for("g3-ui.tabs");
window[key]?.dispose?.();
const listeners = new AbortController();
const opts = { signal: listeners.signal };
let active = null;
let suppress = null;
document.addEventListener("pointerdown", (event) => {
    if (event.pointerType === "mouse" || event.defaultPrevented || active || !event.isPrimary) return;
    const target = event.target instanceof Element && event.target;
    const root = target && target.closest('.g3-tabs[data-swipe="true"]');
    if (!root || target.closest('input,textarea,select,[contenteditable="true"],[role="slider"],.g3-swipe-item,.g3-reorder,.g3-clock,.g3-wheel,.g3-shelf-track,.g3-segment-scrollable')) return;
    active = { root, pointerId: event.pointerId, x: event.clientX, y: event.clientY, dx: 0, captured: false };
}, opts);
document.addEventListener("pointermove", (event) => {
    const drag = active;
    if (!drag || event.pointerId !== drag.pointerId) return;
    if (!drag.root.isConnected || drag.root.dataset.swipe !== "true") { active = null; return; }
    drag.dx = event.clientX - drag.x;
    const dy = event.clientY - drag.y;
    if (!drag.captured) {
        if (Math.abs(dy) > 10 && Math.abs(dy) >= Math.abs(drag.dx)) { active = null; return; }
        if (Math.abs(drag.dx) <= 10 || Math.abs(drag.dx) <= Math.abs(dy)) return;
        drag.captured = true;
        try { drag.root.setPointerCapture(event.pointerId); } catch (_) {}
    }
    if (event.cancelable) event.preventDefault();
}, opts);
const end = (event, cancelled) => {
    const drag = active;
    if (!drag || event.pointerId !== drag.pointerId) return;
    active = null;
    if (!drag.captured) return;
    suppress = { root: drag.root, until: performance.now() + 300 };
    if (cancelled || !drag.root.isConnected || Math.abs(drag.dx) < 48) return;
    const tabs = [...drag.root.querySelectorAll('[role="tab"]')]
        .filter((tab) => tab.closest('.g3-tabs') === drag.root && !tab.disabled && tab.getAttribute('aria-disabled') !== 'true');
    const index = tabs.findIndex((tab) => tab.getAttribute('aria-selected') === 'true');
    const rtl = getComputedStyle(drag.root).direction === 'rtl';
    const direction = (drag.dx < 0 ? 1 : -1) * (rtl ? -1 : 1);
    if (index >= 0) tabs[index + direction]?.click();
};
document.addEventListener('pointerup', (event) => end(event, false), opts);
document.addEventListener('pointercancel', (event) => end(event, true), opts);
document.addEventListener('lostpointercapture', (event) => {
    if (event.target === active?.root) end(event, true);
}, opts);
document.addEventListener('click', (event) => {
    if (event.isTrusted && suppress && performance.now() < suppress.until && suppress.root.contains(event.target)) {
        event.preventDefault();
        event.stopPropagation();
    }
}, { capture: true, signal: listeners.signal });
let released;
const done = new Promise((resolve) => { released = resolve; });
window[key] = { dispose() { listeners.abort(); active = null; suppress = null; released(); } };
await done;
