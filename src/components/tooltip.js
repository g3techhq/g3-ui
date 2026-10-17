const tip = document.getElementById(__ID__);
const anchor = tip?.parentElement;
if (anchor) {
    const key = Symbol.for('g3-ui.tooltip-description');
    const session = {};
    tip[key] = session;
    let control = null;
    const release = () => {
        const owned = control?.[key];
        if (owned?.session !== session) return;
        if (control.getAttribute('aria-describedby') === owned.id) control.removeAttribute('aria-describedby');
        delete control[key];
    };
    const connect = () => {
        if (tip[key] !== session) return;
        const next = anchor.querySelector('a[href],button,input,select,textarea,[tabindex]');
        if (next !== control) { release(); control = next; }
        if (!control || control === tip) return;
        const previous = control[key];
        // Preserve a description explicitly supplied or changed by the app.
        if (control.hasAttribute('aria-describedby') && !(previous?.tip === tip && control.getAttribute('aria-describedby') === previous.id)) return;
        control[key] = { session, tip, id: tip.id };
        control.setAttribute('aria-describedby', tip.id);
    };
    const observer = new MutationObserver(connect);
    observer.observe(anchor, { childList: true, subtree: true });
    connect();
    try { await dioxus.recv(); } finally {
        observer.disconnect(); release();
        if (tip[key] === session) delete tip[key];
    }
}
