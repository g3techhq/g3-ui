# Changelog

All notable changes to `g3-ui` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-09-13

### Added

- `--g3-sheet-max-height` sets how tall a bottom sheet's content may grow. It
  defaults to the existing cap, `min(50dvh, 26rem)`, or `min(70dvh, 30rem)` on
  wide shells. Set it from the sheet's `class` to size one sheet; overriding
  the cap with a selector had to out-rank g3-ui's own rule, and because
  `g3-ui.css` loads after an app's stylesheet, an equal-weight override lost.
- `Sheet` takes a `backdrop: SheetBackdrop`. `Dismiss`, the default, keeps the
  current dimming scrim that closes the sheet when tapped. `None` renders no
  scrim, for a sheet over a page that is still in use: the page stays visible,
  interactive and scrollable, the sheet is no longer announced as modal, and a
  bottom sheet closes only by dragging its handle.
- Every open dismissible sheet renders a hidden `[data-g3-sheet-dismiss]`
  control, and `open_sheet_count()` reports how many are open. Together they let
  an app close the topmost sheet from Android Back or any other app-level path
  without depending on the backdrop, which a `SheetBackdrop::None` sheet does
  not have. The count is reactive and includes sheets whose `is_open` signal a
  page keeps to itself; persistent `Menu` side sheets are not counted.

### Fixed

- Bottom sheet content taller than the cap now scrolls instead of being cut
  off. The content box is a flex column, so an inset `List` or other child that
  clips its overflow shrank to fit the cap: its last rows were hidden inside it
  and the sheet had nothing to scroll. Direct children of the content box no
  longer shrink.
- Cards nested inside another card or placed on a bottom sheet now receive a
  theme-derived contrasting surface automatically. The tint uses the shared
  text and card color tokens, so it darkens light themes and lightens dark
  themes without consumer-specific selectors.
- The sheet backdrop no longer dismisses on `pointerdown`, only on `click`.
  Closing on the press unmounted the scrim before the press completed, so the
  browser delivered the click to whatever the sheet had been covering: tapping
  the backdrop to dismiss a sheet also activated the control underneath it.

## [0.2.0] - 2026-09-08

### Added

- Every playground demo now has a stable `/components/:slug` URL, and a
  `/transitions` showcase demonstrates route pushes, full-screen covers, fades, segmented
  filmstrips, and morphs with the actual g3-ui app components.
- `AppWrapper::route_transition_root` lets documentation shells and other
  non-navigating outer wrappers opt out of the cover snapshot while a nested
  real app continues to use route transitions.
- `ComponentMode` now derives `Debug`, `Eq`, and `Hash`. Without `Debug` a
  consumer cannot print the mode or use it in `assert_eq!`, which makes an
  app-level setting that stores the mode awkward to test.

### Changed

- **Breaking (CSS):** drop the Tailwind dependency. Components emitted plain
  Tailwind utilities, so `assets/g3-ui.css` alone was never enough - a consumer
  had to run Tailwind over g3-ui's own source to get working components, which
  the README never said, and `fill-gray`/`text-focused` only ever existed in the
  playground's `@theme`, so no consumer could get them at all. Every class is
  now defined in `g3-ui.css` and namespaced `g3-`; the `info-btn*` and
  `selected` classes are renamed `g3-info-btn*` and `g3-card-selected`. Link
  `g3-ui.css` and nothing else.

### Fixed

- Ship the styles the playground asks for. `tailwind_output` wrote a stylesheet
  that `[web.resource] style` linked as a bare `<link>` rather than a bundled
  asset, so `dx bundle` never copied it: the deployed site requested
  `/assets/tailwind.css`, nginx's `try_files` answered with `index.html`, and
  `nosniff` made the browser refuse it. The playground had been running without
  Tailwind the whole time, which is how the gap stayed invisible.
- Restore `Fab` positioning. `fab.rs` hardcoded Tailwind classes instead of the
  `g3-fab-vertical-*`/`g3-fab-horizontal-*` constants beside it, whose rules
  already existed, so with no Tailwind the container got no offsets at all and
  fell wherever flow put it.
- Stop `Spinner` printing its screen-reader label on screen. The label used a
  bare `sr-only` class that nothing defined, so "Loading" rendered as visible
  text next to the spinner. `g3-sr-only` now hides it while keeping it in the
  accessibility tree.
- Keep the `Select` chevron from being squeezed by a long value; `shrink-0` was
  a Tailwind class that never applied.
- Stop the iOS `Header` from swallowing the overlays inside it. `.g3-header-ios`
  carried a `backdrop-filter` over a fully opaque `--color-card` background, so
  the blur was invisible but still made the header the containing block for
  every `position: fixed` descendant. A `Select` in a header slot rendered its
  sheet against the bottom of the header instead of the viewport. The other
  translucent iOS surfaces keep their blur; they pair it with a `color-mix`
  background and hold no fixed children.
- Stop a push or reveal side sheet trapping the app shell's overlays. The
  content beside such a sheet was given `translate: 0 0` whenever the sheet
  merely existed, open or not, which made it the containing block for every
  `Select` sheet, modal, and toast in the app. The closed state now leaves
  `translate` unset; `none` interpolates as zero, so the push animation is
  unchanged.
- Paint a sheet that mounts open as open, instead of painting one closed frame
  and animating it in. `presented_open` waited on a `requestAnimationFrame`
  round trip regardless of the sheet's initial state, so a persistent `Menu`
  rail slid in on every load and shifted the page beside it. A sheet that
  mounts closed is unchanged and still animates when it is opened.
- Open sheets without a round trip to JavaScript. The entrance was a CSS
  transition, which only runs once its start value has been painted - never true
  for a sheet inserted on open - so the component asked JS for a
  `requestAnimationFrame` through `document::eval` and waited for the answer
  before anything moved. That also meant a sheet opened in a background tab or a
  minimised window, where frame callbacks are paused, stayed parked closed. The
  entrance is now a keyframe animation, which needs no painted start value: it
  runs on the frame the class lands. `SHEET_PRESENT_SCRIPT` and the present
  signal are gone, leaving the drag handle as the component's only `eval`. The
  exit is still a transition, which is sound because the open state has been
  painted by then, and `reveal` side sheets - which do not move - are excluded.
- Link the library stylesheet once on the web. `AppWrapper` linked it at runtime
  as well as through `with_static_head(true)`, so a web build requested and
  parsed `g3-ui.css` twice. The runtime link now applies only to desktop and
  mobile, which have no build-time head to write into.
- Keep the playground's sticky header pinned once a demo is taller than the
  window. `.playground-root` set only `min-height`, so the `height: 100%` chain
  below it resolved to `auto`: `.playground-main` never became the scroller,
  the document scrolled instead, and the header - sticky inside the
  `overflow: hidden` page rather than to the viewport - scrolled away with it.
- Open the playground's component drawer with the page on a wide shell, where
  it is a persistent `Menu` rail that reserves its own space. It is read from
  the media query during the first render rather than a round trip later, so
  the rail is laid out rather than animated in, and a phone - where the drawer
  is a dismissible `Overlay` - never flashes it open.

## [0.1.0] - 2026-09-06

Initial release.

- Fix the responsive `Navbar` rail layout when the `transitions` feature is enabled.
- Add `NavbarTabDesktopPlacement` for grouping profile/settings tabs at the bottom of the desktop rail.
- Place header toolbars inline with the title and actions on wide app shells.
- Keep Material header tab indicators flush with the header's bottom edge on wide shells, matching compact ones. The wide-shell bottom inset is iOS-only, whose pill sits deliberately clear of the edge.
- Keep compact desktop headers on the two-row layout so titles remain readable and iOS segments retain bottom spacing.
- Add Mobile and Desktop viewport modes with compact- and wide-shell previews to every playground demo, selected from the header beside the design-language toggle.
- Add typed overlay, push, reveal, and persistent menu side-sheet behaviors for both left and right placements.
- Make sheet backdrops fill and dim the complete app shell, dismiss immediately from mouse/touch input, and keep contained playground demos from locking the outer page scroll.
- Make the site-wide playground drawer a persistent left menu on wide shells, falling back to a dismissible overlay once the playground page itself is phone width, keep that rail open when a demo is picked while still dismissing the phone overlay, and prevent placement classes from constraining backdrop coverage.
- Keep push sheets dimmed and dismissible at every width, add flat menus that resize rather than clip the page, and cast reveal elevation from the moving page onto the sheet.
- Keep pages scrollable beside menus, use square menu edges, and align push and overlay scrim opacity.
- Center desktop demo/source surfaces and flatten the playground component drawer styling.
- Simplify shell labels and use a persistent menu-style playground drawer.
- Give wide app shells compact floating bottom sheets, aligned select menus, centred toasts, and roomier modal dialogs.
- Update the full app demo to show secondary navigation at the bottom of the desktop rail.
- Separate menu side sheets from the page they sit beside with a hairline edge and a soft shadow, so a flush header no longer reads as one surface.
- Put each playground demo's controls above its preview so the case is set up before it is read, and keep a wide control group inside the card instead of spilling out of it.
- Make mode and theme switchable at runtime: `G3Mode` now carries a `Signal<ComponentMode>` and the ambient theme is published as a `Signal<Theme>`, so changing either re-renders components that are already mounted. Previously both were read once per component and never again, and `AppWrapper` read its own published value back instead of an outer one. **Breaking:** `G3Mode.mode` changed type; construct it from a signal.
- Add `use_ambient_theme` for reading the theme an enclosing `AppWrapper` or `G3ThemeProvider` set.
- Replace the JavaScript first-paint guard with `AssetOptions::css().with_static_head(true)`, which puts the stylesheet `<link>` in the document head at build time and lets the browser block first paint on it. The old guard hid the shell behind `visibility: hidden` and `transition: none !important` until a polled round trip reported the stylesheet had applied; when that round trip never completed the guard never lifted, which left every transition in the app dead - swipe rows snapped back instead of animating. `AppWrapper` still links the stylesheet at runtime as well, because desktop and mobile bundles only collect assets something links at runtime - a statically-headed asset alone never reaches them. **Breaking:** `G3PreloadStyle` and the `g3-preload` class are gone.
- Stop the swipe action colour bleeding through as a hairline along the bottom of the last swipeable row: the dragged content composites separately from the actions beneath it, and on a fractionally-tall row the two rasterize to different device pixels. Rows above the last were only ever covered by their own divider.
- Hold a swiped row's action colour underneath until the row has slid back over it. The row reported itself closed the instant the drag was released, which hid the actions immediately and left the row animating home across bare card.
- Link the playground stylesheet into the head at build time. Loading it at runtime left a window where the library stylesheet had applied but the playground's had not, so the device frame rendered unstyled while the app booted. The page background during that window is white, so it reads as the browser's own blank page rather than a colour of its own.
- Drop the compact-shell/wide-shell chips above each playground preview; the header toggle already names the width.
- 25 mobile-first components with iOS and Material Design variants of each.
- Configurable `Theme` of 17 CSS custom-property tokens, with built-in light
  and dark themes.
- Optional `transitions` feature integrating `g3-route-transitions`.
- Safe-area insets and `prefers-reduced-motion` handled in the stylesheet.
