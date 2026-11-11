// Runs a theme change inside a view transition whose new state is clipped
// to a circle growing from the origin. Sends "swap" when the change should
// be made, waits for "applied" before the new state is captured, and sends
// "done" once the reveal has finished.
const origin = __ORIGIN__;
const root = document.documentElement;
const reduced = globalThis.matchMedia?.("(prefers-reduced-motion: reduce)").matches;

if (!origin || typeof document.startViewTransition !== "function" || reduced) {
    dioxus.send("swap");
    await dioxus.recv();
    dioxus.send("done");
} else {
    // To the farthest corner, so the circle ends covering the window.
    const radius = Math.hypot(
        Math.max(origin.x, innerWidth - origin.x),
        Math.max(origin.y, innerHeight - origin.y),
    );
    root.style.setProperty("--g3-reveal-x", `${origin.x}px`);
    root.style.setProperty("--g3-reveal-y", `${origin.y}px`);
    root.style.setProperty("--g3-reveal-r", `${radius}px`);
    root.dataset.g3ThemeReveal = "";
    // The page is revealed as one picture, so regions with view transition
    // names of their own (g3-route-transitions' regions, g3-ui's persistent
    // rail) give them up until the reveal ends. Inline, because their names
    // come from an unlayered sheet that g3-ui's layered rules cannot beat.
    const regions = [...document.querySelectorAll(
        ".route-transition-base-region, .route-transition-persistent-region, "
        + ".route-transition-page-frame, .route-transition-page-region, "
        + ".route-transition-segment-region, .route-transition-overlay-region, .g3-nav",
    )].map((element) => [element, element.style.viewTransitionName]);
    for (const [element] of regions) element.style.viewTransitionName = "none";
    // Both have to reach computed style before the old state is captured.
    void root.offsetHeight;

    const transition = document.startViewTransition(async () => {
        // Watch first, then ask: the theme can render before the ack arrives.
        // A change lands in steps (the shell's colors, then the root's color
        // scheme, then anything restyled by mode), so the new state is taken
        // once style changes have stopped for a moment, not at the first.
        const rendered = new Promise((resolve) => {
            let quiet = null;
            const done = () => {
                observer.disconnect();
                clearTimeout(quiet);
                resolve();
            };
            const observer = new MutationObserver(() => {
                clearTimeout(quiet);
                quiet = setTimeout(done, 60);
            });
            observer.observe(root, {
                attributes: true,
                subtree: true,
                attributeFilter: ["style", "class", "data-g3-mode"],
            });
            setTimeout(done, 700);
        });
        dioxus.send("swap");
        await dioxus.recv();
        // No animation frames tick inside this callback, so it waits on DOM
        // changes and timers only.
        await rendered;
    });
    // The browser skips a reveal whose new look takes too long to render (a
    // slow device, a background tab). The change still lands, so a skipped
    // reveal is not an error.
    transition.ready.catch(() => {});
    try {
        await transition.finished.catch(() => {});
    } finally {
        delete root.dataset.g3ThemeReveal;
        for (const [element, name] of regions) element.style.viewTransitionName = name;
        root.style.removeProperty("--g3-reveal-x");
        root.style.removeProperty("--g3-reveal-y");
        root.style.removeProperty("--g3-reveal-r");
        dioxus.send("done");
    }
}
