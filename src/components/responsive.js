const shell = document.getElementById(__ID__);
if (shell) {
    let last = null;
    const report = () => {
        const rem = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
        const width = shell.getBoundingClientRect().width / rem;
        const size = width >= 64 ? 2 : width >= 48 ? 1 : 0;
        if (size !== last) {
            last = size;
            dioxus.send(size);
        }
    };
    const observer = new ResizeObserver(report);
    observer.observe(shell);
    observer.observe(document.documentElement);
    report();
    try { await dioxus.recv(); } finally { observer.disconnect(); }
}
