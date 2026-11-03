// System Back for open overlays. One stack for the whole app, in presentation
// order, so Back only ever closes the surface on top.
//
// Web: a sheet pushes a history entry at the same URL when it opens, and
// browser Back leaving that entry closes the sheet. The popstate is stopped
// in the capture phase, ahead of the router's own listeners, so the router
// never sees a step that does not change the route.
//
// Native: g3-native-plugins raises a cancelable `g3nativeback` on window. The
// top surface claims it synchronously, before the router integration's
// deferred handler pops a route.
const backKey = Symbol.for("g3-ui.back");
if (!window[backKey]) {
    const layers = [];
    const nav = window.navigation?.entries ? window.navigation : null;
    // Entries of sheets that have closed, by Navigation API key. They stay in
    // history at the page's URL, and traversals step over them.
    const spent = new Set();
    // Popstates from our own history.back(), where there is no Navigation API.
    let ignore = 0;
    let lastKey = nav?.currentEntry?.key ?? null;

    const indexOf = (key) => nav.entries().findIndex((entry) => entry.key === key);
    const isCurrent = (layer) => {
        if (location.href !== layer.url) return false;
        return nav
            ? nav.currentEntry?.key === layer.key
            : history.state?.g3UiSheet === layer.token;
    };

    nav?.addEventListener("currententrychange", (event) => {
        // Traversals are settled in the popstate handler instead.
        if (event.navigationType !== "traverse") lastKey = nav.currentEntry?.key ?? null;
    });

    const onPopState = (event) => {
        const left = lastKey;
        lastKey = nav?.currentEntry?.key ?? null;
        if (ignore > 0) {
            ignore -= 1;
            event.stopImmediatePropagation();
            return;
        }

        // Sheets whose entries this traversal went back past.
        const passed = layers.filter((layer) => {
            if (!layer.pushed) return false;
            if (nav) {
                const index = indexOf(layer.key);
                return index >= 0 && index > (nav.currentEntry?.index ?? Infinity);
            }
            return layer === layers.findLast((other) => other.pushed)
                && history.length === layer.length
                && history.state?.g3UiSheet !== layer.token;
        });
        if (passed.length > 0) {
            // Only a step within the page is ours; a jump to another route
            // still belongs to the router, and the sheets close with it.
            if (passed.every((layer) => layer.url === location.href)) {
                event.stopImmediatePropagation();
            }
            for (const layer of passed.reverse()) {
                layers.splice(layers.indexOf(layer), 1);
                if (layer.key) spent.add(layer.key);
                layer.dismiss();
            }
            return;
        }

        // Step over a spent entry, in the direction the user was going: it
        // shows the same page as its neighbour, so stopping there would make
        // Back look like it did nothing.
        if (nav) {
            const current = nav.currentEntry;
            const from = left ? nav.entries()[indexOf(left)] : null;
            if (current && from && (spent.has(current.key) || spent.has(left)) && from.url === current.url) {
                event.stopImmediatePropagation();
                history.go(current.index < from.index ? -1 : 1);
            }
        } else if (history.state?.g3UiSheet && !layers.some((layer) => layer.token === history.state.g3UiSheet)) {
            // Without keys, only an entry still carrying its marker is known.
            // Nothing was pushed over it, so it was reached going forward.
            event.stopImmediatePropagation();
            history.forward();
        }
    };
    window.addEventListener("popstate", onPopState, { capture: true });

    window.addEventListener("g3nativeback", (event) => {
        if (event.defaultPrevented) return;
        const top = layers.at(-1);
        // A dialog or menu on top keeps Back for the app; never close the
        // sheet beneath it.
        if (!top?.closesOnBack) return;
        event.preventDefault();
        layers.pop();
        top.dismiss();
    });

    window[backKey] = {
        open(token, closesOnBack, push, dismiss) {
            const layer = { token, closesOnBack, dismiss, pushed: false };
            if (closesOnBack && push) {
                // Same URL: the router keeps the route, only Back changes.
                history.pushState({ g3UiSheet: token }, "", location.href);
                layer.pushed = true;
                layer.url = location.href;
                layer.length = history.length;
                layer.key = nav?.currentEntry?.key ?? null;
                lastKey = layer.key ?? lastKey;
            }
            layers.push(layer);
        },
        close(token) {
            const index = layers.findIndex((layer) => layer.token === token);
            if (index < 0) return;
            const [layer] = layers.splice(index, 1);
            if (!layer.pushed) return;
            if (nav) {
                // Never history.back() here: a row that closes its sheet and
                // navigates would race the route push. Mark the entry spent
                // and let traversals step over it.
                spent.add(layer.key);
            } else if (isCurrent(layer)) {
                ignore += 1;
                history.back();
            }
        },
    };
}

// One open surface. The token is made here, so a surface reopened in the
// same tick never shares one with the session it replaces.
{
    const back = window[backKey];
    const token = crypto.randomUUID?.() ?? `${Date.now()}-${Math.random()}`;
    back.open(token, __CLOSES_ON_BACK__, __PUSH_HISTORY__, () => dioxus.send("dismiss"));
    try {
        await dioxus.recv();
    } finally {
        back.close(token);
    }
}
