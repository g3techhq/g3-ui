# g3-ui production readiness — October 16, 2026

Reviewed: the working tree on top of `43ce21e` (crate 0.5.1). That is 70
modified tracked files (+3,124 / −1,472) and about 35 untracked files: the
audit fixes, the responsive work, the playground docs site and the palette.
Another session was still editing this tree during the review (the last write
was `src/theme.rs` at 23:08), so line numbers may drift. No source was changed
during this review.

## Verdict

**The library is close. The current state is not releasable.**

The engineering is production grade. The architecture is sound and the
conventions are enforced by tests. Every automated gate passes, and all 15
findings from the October 15 audit are fixed and covered by tests. The
existing consumers can depend on it for web and Chromium web views.

Four things block calling it production-ready today:

1. **None of it is committed.** About 4,600 changed lines across ~105 files
   have not been reviewed in a PR or run through CI.
2. **The real-browser suite is red.** One failure is test drift. The other is
   a real overlay placement bug (below).
3. **The version is wrong.** `use_theme()` and `use_strings()` now return
   `Signal<_>`, which is a breaking change, so the next release is 0.6.0, not
   0.5.x. The four consumer apps need the Definition of Done #4 check against
   it.
4. **There is no device evidence.** Nothing has run on an Android System
   WebView or an iOS WKWebView. Both earlier reviews say so too. For a
   mobile-first library, that is the largest remaining gap.

## What was run

| Check | Result |
| --- | --- |
| `just pre-push` | Pass. 164 tests (default) and 165 (`transitions`), 90 doctests (2 ignored), 19 playground tests, 15 production-script tests, typos. |
| `cargo deny check` | Pass: advisories, bans, licenses and sources. Warnings only: duplicate versions (`syn`, `thiserror`, `winnow`, …) and 3 unused license allowances. |
| `cargo package --allow-dirty` | Pass. 106 files, 278 KiB compressed. |
| `cargo check --lib --target wasm32-unknown-unknown --all-features` | Pass. |
| Playground check for `wasm32` | Pass. |
| `cargo +1.88 check --features transitions` (MSRV) | Pass. |
| `cargo doc --features transitions` | Pass, with 1 warning: unresolved link to `ThemeSeeds::generate` at `src/theme.rs:340`. |
| `just browser-test` (Playwright, headless Chromium) | **Fail.** See "Playground". |
| Automated sweep of 46 component pages × MD/iOS at 1440×900 | No page errors, no unnamed controls, no duplicate ids. `aria-controls` dangles on every closed `Select`. |
| Android / iOS device | Not available; not run. |

Every check used the parent `.cargo/config.toml` `[patch.crates-io]`, which
points `g3-route-transitions` at the sibling checkout. The October 15 record
has the registry-only package check.

## Release blockers

### B1. The browser suite fails

`node playground/browser-tests.cjs` fails three ways.

1. **The dev server boots the app twice (environment).** The `dx serve`
   running on :8080 serves a `g3-ui-playground.js` whose loader bootstrap
   block appears twice (lines 2293 and 2309). WebAssembly is instantiated
   twice, the 140 MB debug wasm is fetched twice, and two complete app trees
   mount in `#main`, with 42 duplicate ids on the landing page. The deployed
   site (g3ui.g3tech.net) has a single bootstrap and a single tree. This is a
   stale incremental `dx` build, not g3-ui code. A clean `dx serve` should fix
   it. A one-line guard in the suite (`#main` has exactly one child) would
   catch it next time.
2. **Test drift at `browser-tests.cjs:67`.** `getByRole('heading', { name:
   'Overlay hooks' })` now matches three headings (the header `h1`, the guide
   `h2`, and the preview's `h1`). Scope it to `getByRole('banner')`. With that
   change and the loader deduplicated, the responsive, search, theme-generator
   and nested-grid sections pass.
3. **A real bug: bottom sheets render off-screen in the Mobile preview.** On
   `/components/alert`, "Stacked overlays" opens a sheet whose buttons sit
   below the viewport. At 1440×900 the frame spans y 176–1020 and the sheet's
   button is at y 916–952 (viewport 900). At 390×844 the frame spans y 241–1041
   and the button is at y 937. The page cannot scroll while the sheet is open.
   This hits anyone opening a sheet demo on a laptop. The cause has two parts:
   B2 below, plus a phone preview taller than the space under the playground
   header.

CI runs this suite against the production container, so failures 2 and 3
would fail CI.

### B2. The overlay layer can leave the viewport (library, P2)

`src/components/layers.js:37-52` sizes the top-layer wrapper to the enclosing
`.g3-app` box. It falls back to the full window only when that box covers the
whole viewport (`fills`, line 41). An `AppWrapper` that starts below other
content and extends past the bottom of the window does not "fill". The layer
then takes its full box, partly below the fold, and bottom sheets, action
sheets and toasts anchored to the bottom are unreachable. The playground is
one case. An app embedding a g3 shell under its own page header is another.
**Fix:** intersect the app box with the viewport (`visualViewport` where
present) before writing `--g3-layer-*`, and add a browser test with an offset
shell.

### B3. Version and consumers

Bump to 0.6.0. Check g3-stack, greenside-partee, tawny and media-mancer
through a temporary `[patch.crates-io]` path override (all four pass `mode`
explicitly; see P3-3). The 0.6.0 break is also the cheapest moment for the
API fixes below.

### B4. Device pass

Run the Android checklist in `docs/testing.md` on a real device or emulator.
It covers taps inside swipe regions, swipe versus scroll, nested overlays,
Back through `dismiss_top()`, IME resize, rotation, TalkBack and frame
timing. Run one WKWebView pass too: the iOS mode exists for it, and the
Safari 17.5 baseline in the README is a claim that has not been tested.

## Library findings

| # | Sev | Finding | Where |
| --- | --- | --- | --- |
| 1 | P2 | The overlay layer leaves the viewport for an offset shell (B2). | `layers.js:37-52` |
| 2 | P3 | `aria-controls` points at an element that is not mounted: the closed `Select` listbox, the closed `Menu` panel, and unmounted tab panels. Render it only while the target exists. | `select.rs:192`, `popover.js:54`, `tabs.rs` |
| 3 | P3 | Hydration footgun: `AppWrapper` with no `mode` falls back to `get_mode()`. That reads the user agent on wasm (iOS → `Ios`) but is `Md` on a server, so a fullstack app hydrates `data-g3-mode="md"` on iPhones. `GLOBAL_MODE` is also thread-local, so on a server it varies by worker thread. All current consumers pass `mode`. Document "pass `mode` in fullstack apps", or make the SSR default explicit. | `theme.rs:961-1000` |
| 4 | P3 | A required `Select` is not announced. Its trigger is a `button`, which cannot take `aria-required`. The APG select-only combobox (`role="combobox"`) can. | `select.rs:185` |
| 5 | P3 | Each `AppWrapper` renders its own `StylesheetLink` and `ViewportMeta`. The playground head has `g3-ui.css` three times and five viewport metas, and every copy of the 7,300-line sheet joins the cascade. Render both only in the root shell (`is_root_shell` is the same on server and client, so hydration is unaffected). | `app_wrapper.rs` |
| 6 | P3 | `Theme` and `Look` values are written raw into `style` and a `<style>` rule. Only `</` is escaped, so a value containing `}` escapes `#id { … }`. That is harmless for themes the developer writes. Document that end-user themes must go through `ThemeSeeds` (validated) or be sanitized. | `theme.rs:385-428` |
| 7 | P3 | Touch targets: the compact `Toggle` row is 22 px tall (WCAG 2.5.8 asks for 24), and the `Range` input box is 16 px tall. | CSS |
| 8 | P3 | Broken intra-doc link. Add `RUSTDOCFLAGS="-D warnings" cargo doc` to CI so docs.rs warnings fail the build. | `theme.rs:340` |
| 9 | P3 | `AdaptiveNav`/`NavRail` persist their expanded state in `localStorage` by default (`remember: true`) under one fixed key. Apps sharing an origin share it. Consider defaulting to off, or keying it by app. | `nav.rs:62,195` |

## API and naming

The API is coherent overall. Every prop is documented and typed, and
`Option<T>` defaults are stated. The issues below are consistency problems
against AGENTS.md's own rules. Several are breaking, so batch them into 0.6.0
or decide to leave them.

**Event naming is mixed.** Some handlers are DOM-style (`onclick`,
`onchange`, `oninput`, `onreorder`). Others are snake case (`on_select`,
`on_result`, `on_dismiss`, `on_confirm`, `on_cancel`, `on_refresh`,
`on_load`, `on_submit`, `on_swipe`, `on_activate`, `on_long_press`). A
defensible rule is "events that mirror a DOM event are lowercase; component
events are `on_*`". Under it, only `onreorder` is wrong (→ `on_reorder`). Write
the rule into AGENTS.md either way.

**Some components break the controlled-state rule.**
- `Tabs.value` and `SegmentGroup.value` require a `Signal<T>`; AGENTS.md says
  `Option<Signal<T>>` through `use_controlled`. A required value is defensible
  when `T` has no default, but then say so in the rule.
- Only `Select` has a `default_value`.
- `FabList.open` is a plain `bool`, `FabMenu.open` is `Option<Signal<bool>>`,
  and the overlays take a required `Signal<bool>`.

**Some props break "no magic numbers or strings".**
- `Rating.size: Option<u32>` is in pixels, while every other size is an enum.
- `heading_level: Option<u8>` is silently clamped to 1–6 (`text.rs:112`) on
  `Card`, `EmptyState`, `ListHeader`, `AccordionItem` and `Shelf`. A
  `HeadingLevel` enum removes the clamp.
- `width`, `max_height` and `aspect_ratio` take CSS strings. That is
  acceptable, but document it.

**`mode` is missing where Ionic's look differs by mode:** `Chip`, `Range`,
`Progress`, `Spinner` (Ionic's default spinner differs per mode), `Badge`,
`Stepper`, `Rating`, `NavItem`, `MenuItem`. These follow the ancestor through
CSS, but cannot be overridden one at a time.

**`color` is missing where Ionic has it:** `Chip`, `Toggle`, `Checkbox`,
`Radio`, `Range`, `Progress`, `Spinner`, `SegmentGroup`, `FabButton`, `Card`,
and the header toolbar.

**Form props are uneven.**
- `error` is missing on `RadioGroup`, `Toggle`, `Range`, `Stepper` and
  `Rating`.
- `required` is missing on `Checkbox`, `RadioGroup` and `Toggle`.
- `name` exists only on `RadioGroup`. `Input` and `TextArea` can pass it
  through `attributes`; `Checkbox`, `Toggle` and `Select` cannot, which limits
  native form posts and autofill.
- `id` is missing on `Searchbar`.
- `aria_label` is missing on `Tab` (icon-only tabs), `MenuItem` and `Chip`.
- `new_tab` exists only on `Button` and `Item`. `Card`, `MenuItem`, `NavItem`
  and `FabButton` accept `href` without it.

**Some concepts are duplicated.**
- `ShellSize`/`use_shell_size` (two stages) overlaps
  `LayoutSize`/`use_layout_size` (three stages).
- `Grid.wide_columns`/`wide_gap` are documented as legacy next to
  `medium_columns`/`expanded_columns`, with no `medium_gap` or `expanded_gap`.
- Deprecate `ShellSize` and `wide_*` in 0.6.0.

**Names compared with Ionic.** `Menu` is a popover menu here, but `IonMenu` is
the side drawer, which g3-ui calls `NavigationDrawer`. Ionic users will
reach for the wrong one, and the playground's own comment
(`playground/src/main.rs:186`) still calls the drawer "Menu". Consider
`PopoverMenu`/`DropdownMenu`. `TextArea` is `IonTextarea`. The other renames
(`SwipeItem` for `ItemSliding`, `TabList`/`Tab` for `TabBar`/`TabButton`,
`Progress` for `ProgressBar`) are reasonable but worth one table in the README
for Ionic users.

**Small API rough edges.**
- `ReorderList.onreorder` takes `(usize, usize)`. A named `{ from, to }` event
  would document itself.
- `open_sheet_count()` is a free function next to `OverlayController::count()`.
- `Theme::tokens()` returns `[_; 17]`, so adding a token breaks the API;
  return a slice or an iterator.
- `Theme.color_scheme` is a `String` where an enum would do.

**Ionic components not yet covered:** Loading, Footer, standalone
Toolbar/Title/Buttons, Note/Label, ItemDivider/ItemGroup, Breadcrumbs,
InputOtp. None of these blocks a release.

## Playground

**What's good:**
- 47 demos, prop references generated from signatures and rustdoc, search,
  guides and a theme generator.
- The sweep found no runtime errors or unnamed controls in either mode.
- The production site is healthy.

**What needs fixing:**
- B1.3: sheets render off-screen in the Mobile preview. Cap the preview
  height to the space available, or let the page scroll to the frame, in
  addition to fixing B2.
- B1.1: restart `dx serve` cleanly and add the single-root guard.
- `<html lang>` is empty, the title is "g3_ui Playground" (`Dioxus.toml`),
  and there is no meta description.
- `CHANGELOG.md [Unreleased]` is about 170 lines, mostly playground copy and
  layout notes. Consumers read it for migrations. Move playground-only entries
  out, or under their own heading, so the hook and nav migrations stand out.

## What is solid

- **Interaction architecture.** Continuous gestures run in the page, Rust
  hears only outcomes, and capture starts after the slop. This matches the
  AGENTS.md rule, and script tests check it.
- **Scoped DOM sessions** (`script.rs`) dispose of listeners, observers and
  timers.
- **Overlays share top-layer presentation order**, with `dismiss_top()` for
  Back.
- **Hydration-safe element ids.**
- **The stylesheet:**
  - one `@layer g3` with Tailwind-aware layer order;
  - tokens only, with no colour literals outside the `:root` fallback;
  - no `!important`;
  - container queries only, with no viewport media queries;
  - 16 reduced-motion blocks, a forced-colors block and 52 `:focus-visible`
    rules.
- **Docs.** Every component and prop is documented, and the README and
  guides compile as doctests.
- **Supply chain.** `cargo deny` is clean, and the MSRV and wasm builds pass.

## Path to a release

1. Once the other session finishes, land the working tree as reviewable
   Conventional Commits through a PR, so CI runs.
2. Fix B2 (viewport clamp) and the playground preview height. Update
   `browser-tests.cjs:67`, restart `dx serve`, and get `just browser-test`
   green.
3. Decide which API fixes go into 0.6.0. At minimum: `onreorder`, the
   `HeadingLevel` enum, `Rating.size`, the `ShellSize`/`wide_*`
   deprecations, and the `Menu` name.
4. Bump to 0.6.0 and patch-check the four consumers.
5. Do an Android device pass and a WKWebView pass per `docs/testing.md`.
6. With the maintainer's go-ahead, publish through `publish-crate.yml`.
