const el = document.getElementById(__ID__);
if (el) {
    const stackKey = Symbol.for('g3-ui.focus');
    const stack = window[stackKey] ||= [];
    const previous = document.activeElement;
    const session = { el };
    stack.push(session);
    const top = () => stack.at(-1) === session;
    const listeners = new AbortController();
    const roving = __ROVING__;
    const visible = (node) => !node.disabled && !node.hidden && !node.closest('[inert], [aria-hidden="true"]') && node.getClientRects().length > 0;
    const selector = 'a[href],button,input,select,textarea,[tabindex]';
    // Nonblocking toast actions remain reachable above a modal surface.
    const toast = (node) => node instanceof Element && node.closest('.g3-toast[data-state="open"]');
    const focusables = () => [...el.querySelectorAll(selector), ...document.querySelectorAll('.g3-toast[data-state="open"] :is(a[href],button,input,select,textarea,[tabindex])')].filter((node) => visible(node) && node.tabIndex >= 0);
    const items = () => [...el.querySelectorAll(roving)].filter((node) => visible(node) && node.getAttribute('aria-disabled') !== 'true');
    const trap = () => el.dataset.g3TrapFocus === 'true';
    const onKey = (event) => {
        if (!top() || (!el.contains(event.target) && !toast(event.target))) return;
        if (event.key === 'Escape') { event.stopPropagation(); event.preventDefault(); dioxus.send('escape'); return; }
        if (roving && event.key.length === 1 && event.key.trim() && !event.ctrlKey && !event.metaKey && !event.altKey) {
            const all = items();
            const current = all.indexOf(document.activeElement);
            const match = all.slice(current + 1).concat(all.slice(0, current + 1)).find((node) => node.textContent.trim().toLowerCase().startsWith(event.key.toLowerCase()));
            if (match) { event.preventDefault(); match.focus(); }
            return;
        }
        if (roving && ['ArrowDown', 'ArrowUp', 'Home', 'End'].includes(event.key)) {
            const all = items();
            if (!all.length) return;
            event.preventDefault();
            const current = all.indexOf(document.activeElement);
            const next = event.key === 'End' ? all.length - 1 : event.key === 'Home' ? 0
                : event.key === 'ArrowDown' ? (current + 1) % all.length : (current - 1 + all.length) % all.length;
            all[next].focus();
            return;
        }
        if (event.key !== 'Tab' || !trap()) return;
        const all = focusables();
        const current = all.indexOf(document.activeElement);
        if (!all.length) { event.preventDefault(); el.focus(); }
        else if (event.shiftKey && current <= 0) { event.preventDefault(); all.at(-1).focus(); }
        else if (!event.shiftKey && (current < 0 || current === all.length - 1)) { event.preventDefault(); all[0].focus(); }
    };
    document.addEventListener('keydown', onKey, { signal: listeners.signal });
    document.addEventListener('focusin', (event) => {
        if (top() && trap() && !el.contains(event.target) && !toast(event.target)) (focusables()[0] || el).focus({ preventScroll: true });
    }, { signal: listeners.signal });
    let alive = true;
    let initialized = false;
    const initialize = () => {
        if (!alive || initialized || !top() || !el.isConnected) return;
        initialized = true;
        if (el.contains(document.activeElement)) return;
        const all = focusables();
        const selected = roving && items().find((node) => node.getAttribute('aria-selected') === 'true' || node.getAttribute('aria-checked') === 'true');
        (all.find((node) => node.hasAttribute('autofocus')) || selected || all[0] || el).focus({ preventScroll: true });
    };
    const frame = requestAnimationFrame(initialize);
    const timer = setTimeout(initialize, 50); // Hidden pages may never run a frame.
    try { await dioxus.recv(); } finally {
        alive = false;
        listeners.abort(); cancelAnimationFrame(frame); clearTimeout(timer);
        const wasTop = top();
        const index = stack.indexOf(session);
        if (index >= 0) stack.splice(index, 1);
        const current = document.activeElement;
        const restore = wasTop && (el.contains(current) || current === document.body || !current?.isConnected || current.closest('[inert]'));
        if (restore && previous?.isConnected && !previous.closest('[inert]')) previous.focus({ preventScroll: true });
        else if (restore && stack.length) stack.at(-1).el.focus({ preventScroll: true });
    }
}
