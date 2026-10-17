# Runtime validation

`just pre-push` runs compilation, strict lints, mounted Dioxus/SSR tests,
production JavaScript behavior/disposal tests, doctests and spelling. The
script tests execute the shipped sources against event/observer doubles;
they exercise cancellation and cleanup but cannot model native browser capture.

With the playground already serving on port 8080, run:

```powershell
npm ci --prefix playground
npx --prefix playground playwright install chromium
just browser-test
```

`PLAYGROUND_URL` overrides the origin. The browser recipe does not start a
server. It tests MD/iOS, 390x844/1440x900, light/dark, reduced motion and touch
input. It checks a real touch tap inside swipeable tabs, keyboard tab
selection, nested-overlay hit testing, focus return and toast presentation.
Screenshots are saved under `playground/target/browser-tests/`, including
failures. CI runs this against the production playground container.

Also inspect date/time wheels and calendars, below-backdrop sheet detents,
popover positioning, RTL strips/rating/swipe, ordinary motion, zoom and
keyboard focus. Script tests cover refresh cancellation and observer/listener
cleanup; repeat mount/unmount cycles in the browser to inspect retained nodes.

## Android

A library ARM64 check establishes compilation only. On an attached device or
emulator, record Android System WebView version, hardware, release/debug mode
and screen/font settings. Test taps within swiping regions, scroll versus
swipe arbitration, cancellation, nested overlays, Back, rotation, IME resizing,
background/resume, large fonts, TalkBack and a hardware keyboard. Confirm
`use_overlay_controller().dismiss_top()` consumes Back before navigation.

For performance comparisons, use the same release build and workload: long
lists, repeated route mount/unmount, reorder, rating/clock dragging and blur
surfaces. Collect frame timing and memory before/after, then profile CPU for
repeatable slow paths. InfiniteScroll does not virtualize mounted rows. Avoid
claiming a speedup from compilation or source inspection alone.

## Consumers

Use a temporary `[patch.crates-io] g3-ui = { path = "../g3-ui" }` override,
or an existing local-development override, while checking a consuming app.
Preserve existing overrides and restore its Cargo.lock byte-for-byte after
the check. Run affected UI flows when its services are available. Report
missing devices/services separately from passing compile checks.
