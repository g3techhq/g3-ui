# Changelog

All notable changes to `g3-ui` are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- `Chip`, `Toggle`, `Checkbox`, `Radio`, `Range`, `Progress`, `Spinner`,
  `SegmentGroup`, `FabButton` and `FabMenu` take a `color` role, as Ionic's
  controls do. `Card` and `Header` take `color` to fill with it, and their
  content (title, icons, buttons, a toolbar segment) switches to the matching
  contrast. The stylesheet exposes the role as `--g3-tint`, `--g3-on-tint`
  and `--g3-tint-ink`.
- `Badge`, `Progress`, `Spinner` and `Range` follow the mode like Ionic: an
  iOS badge is a pill and a Material one is slightly rounded; a Material
  progress bar is square; the iOS spinner is Ionic's eight fading lines; the
  iOS range has a 4px bar and a large knob, the Material one a 2px bar and a
  small tinted knob. Each takes a `mode` prop.
- Form controls report the same states: `Toggle`, `RadioGroup`, `Range`,
  `Stepper` and `Rating` take `error`; `Toggle`, `Checkbox` and `RadioGroup`
  take `required` (`aria-required`); `RadioGroup` takes an `id`. `Toggle`
  and `Checkbox` take `name`: a checked one submits `name=on` with its form,
  so a Dioxus `onsubmit` sees it in the form values.
- `Tab`, `MenuItem` and pressable `Chip`s take `aria_label` for icon-only
  content; `Searchbar` takes an `id`; `Card`, `MenuItem`, `NavItem` and
  `FabButton` take `new_tab` like `Button` and `Item`.
- **Breaking:** `ReorderList`'s `onreorder: (usize, usize)` is now
  `on_reorder: ReorderEvent`, following the rule that a component's own
  events are `on_*` and carry a named struct. `ReorderEvent::apply` makes the
  move in a `Vec`. Migration: `on_reorder: move |moved: ReorderEvent|
  items.with_mut(|items| moved.apply(items))`.
- **Breaking:** `heading_level` on `Card`, `EmptyState`, `ListHeader`,
  `AccordionItem` and `Shelf` takes a `HeadingLevel` (`H1`..`H6`) instead of a
  `u8` that was silently clamped. Migration: `heading_level: 3` becomes
  `heading_level: HeadingLevel::H3`.
- **Breaking:** `Grid` drops `wide_columns` and `wide_gap`; use
  `medium_columns` and the new `medium_gap` (from 48rem) and
  `expanded_gap` (from 64rem). The `--g3-grid-columns-wide` property and
  `data-wide-gap` attribute are gone.
- **Breaking:** `Theme::tokens()` returns a `Vec`, so adding a token is not a
  breaking change again.
- `Menu`'s docs point Ionic users to `NavigationDrawer`, and `ShellSize`'s
  docs explain when to use `use_layout_size` instead.
- `set_mode` is process-wide instead of per thread, so a server that sets it
  in `main` renders every request in that mode. Document passing `mode` to
  `AppWrapper` in server-rendered apps, that `Theme` and `Look` values are
  written as unescaped CSS (build user-chosen themes with
  `Theme::from_seeds`), and the nav rail's local-storage key.
- Only the outermost `AppWrapper` links the stylesheet and viewport tag. A
  nested wrapper (a demo frame, a themed island) no longer adds another copy
  of `g3-ui.css` to the cascade.
- An unlabelled `Toggle`, `Checkbox` or `Radio` takes taps across a 44px
  square around its mark, and a `Range` input box is 44px tall, so both meet
  touch-target guidance without changing the layout.
- `aria-controls` names only elements that exist: a `Select`, `Menu` or
  `Popover` trigger points at its panel only while it is open, a tab only at
  the selected panel, and the calendar title only at the open year grid.
  The `Select` trigger is now a `combobox`, so `required` is announced as
  `aria-required`.

- Add `ThemeSeeds` and `Theme::from_seeds` to build a whole theme at runtime
  from an accent, a page background and an optional second accent, such as a
  color the user picks: the background picks light or dark and tints the
  neutrals, text reads at 4.5:1 on every surface, and an accent that falls
  short is moved only as far as it must be. `ThemeSeeds::generate` also
  returns an `Adjustment` for each moved seed. The new `g3_ui::palette`
  module exposes the color math (`Rgba`, `contrast`, `TEXT_CONTRAST`). The
  playground's theme generator now uses it.
- Add `Strings::from_lookup` to fill `Strings` from any translation system:
  it asks for each text by a message id (the field name, or `months_1` and
  the like for list entries) and keeps English for any it is not given.
- Playground: the layout hooks guide's width ranges are a g3 `Table`, with
  the widths pinned as row headers when it scrolls on a phone.
- Playground: the theme generator asks for the page background before the
  accent.
- Playground themes: Night and Forest are lifted off near-black, with page,
  cards, raised surfaces and fields stepping lighter in turn and the text
  colors in order of emphasis. Blue and Green get cards and fields tinted to
  match their page. Add Light and Dark (the library defaults), Sand, Ocean,
  Graphite and Aurora. Every preset's text and accent read at 4.5:1 on its
  surfaces, which a test checks.
- Theme generator: load any preset into the editor from a row of swatches to
  use or copy its values, or generate a whole theme from an accent, a page
  background and an optional second accent. The background picks light or
  dark and tints the neutrals; text is set to 4.5:1, and an accent that
  falls short is moved only as far as it must be, with a note saying so.
- Add `NavRailBehavior` and a `behavior` prop to `NavRail` and `AdaptiveNav`.
  `Toggle` (the default) keeps the edge button that expands and collapses the
  rail; `Hover` keeps the rail icons-only and expands it over the page while
  hovered or focused from the keyboard, without moving the page. Its rows keep
  one layout, so they slide out of and back into their icons as it widens.
- A scrollable `SegmentGroup` or `Tabs` strip slides its indicator between
  options again, in both modes, instead of highlighting the selected button.
- A labelled `SegmentGroup` no longer adds its standalone top margin inside
  the field, so its label sits as close to it as other controls' labels.
- Playground: remove the shadow behind device previews. Demo controls sit in
  a flat `Card` with one spacing rhythm, and every segment group has a
  visible label. The props reference uses `Table`, scrolling sideways with
  the prop names pinned on phones, and inline code is a neutral chip instead
  of accent-colored text.
- A toggled rail now remembers its state across routes and visits, in local
  storage. Add `remember` (default `true`) to turn this off; a rail given an
  `expanded` signal leaves it to the signal's owner.
- Keep overlays inside an `AppWrapper` that does not fill the window, such as
  a device preview: the top-layer scrim and sheet take the app's box and
  rounded corners instead of covering the whole page.
- The playground opens each new page scrolled to the top, and its docs and
  launch configuration run plain `dx serve`.
- Generate playground prop references from component signatures and rustdoc at
  build time, including required props and enum variants. Show them between
  the device preview and source, with stacked prop rows on narrow screens.
- Show the Demo app in the overview's device viewer, with mode and size
  controls. Phone previews sit beside the intro on wide pages; larger previews
  use a full row and retain app state when resized.
- Rewrite the playground overview around component capabilities for Dioxus
  developers. Remove slogans, the preview subtitle, and promotional counters.
- Use g3-ui buttons for playground page links, remove redundant preview copy,
  and group Demo app and Route transitions under Examples in navigation and
  search. Add a crates.io button to the route transitions example.
- Add writable `use_mode()` and make `use_strings()` return `Signal<Strings>`.
  Children can change mode and labels directly, without parent-owned signals.
  Provider overrides isolate edits, omitted values share parent state, and
  changed props replace it. Migration: read labels with `strings.read().field`
  or `strings()`; call hooks unconditionally before branching.
- Simplify guide language and remove redundant preview headings. Correct native
  dependency setup: g3-ui has no default features to disable. Fix the layout
  hook example's width reset when returning to all available space.
- Make `use_theme()` return shared writable `Signal<Theme>` state. Writes update
  provider CSS and readers; explicit subtree themes stay isolated. Migration:
  use `theme.read().field` or `theme()` to read, and `theme.write().field = ...`
  or `theme.set(...)` in handlers to change it. Changed theme props replace the
  current state; unrelated renders preserve hook edits.
- Add an overview landing page explaining g3-ui, with a working component
  preview. Keep Getting started focused on installing and launching the app.
- Give documentation search a wider desktop dialog, fixed input/footer, clear
  result rows, and a single results area with a subtle scrollbar.
- Replace the generic guide body with individually composed sections, setup
  steps, and comparison tables. Explain theme value ownership and demonstrate
  local layout width versus whole-app width with an interactive constraint.
- Show copyable Cargo.toml setup snippets with TOML syntax highlighting.
- Run compiled guide, pattern, and hook examples in interactive device viewers;
  restore the preview controls where they now affect live examples.
- Give the theme generator one shared page scroll for colors, preview, and
  exported code, removing the separate sticky result scroller.
- Document changing theme, mode, and localized strings through writable hooks
  and reactive providers. Put guide/tool titles in the shared header
  and remove the remaining guide footer links.
- Replace playground header text links with GitHub and Cargo/package API icons,
  keep them accessible on phones, and remove duplicate guide links and component
  source/preview-size captions.
- Expand the theme and mode hook guide with compiled examples of reactive
  providers, custom colors, localized actions, mode overrides, and scoped themes.
- Keep additional syntax token kinds readable when the playground and system
  use different light/dark themes.
- Show mode and size controls on interactive previews, distinguish landscape phone
  previews from desktop workspaces, fix tablet/landscape frame proportions, and
  restore compact, inset iOS header segments.
- Keep segment keyboard focus rings inside the clipped group, avoiding a stray
  colored edge. Introductory and hook guides omit irrelevant preview controls.
- Add a searchable theme generator with all 17 color tokens, optional surface
  styles, live component previews, shared custom themes, and Rust configuration export.
- Show guide examples and component source in shared, framed code panels with
  syntax colors that follow the selected playground theme.
- Add Compact/Medium/Expanded layout sizes, available-width LayoutScope,
  single-variant AdaptiveLayout, persistent PageLayout slots, and intermediate
  Grid column settings. Existing ShellSize and wide column behavior remain compatible.
- Keep nested grids from inheriting a parent grid's responsive column overrides.
- Scale wide content padding with its container rather than the browser window,
  so embedded previews and narrow app panes retain usable content width.
- Add searchable getting-started, platform, layout, and hook guides to the
  playground, GitHub source links, and tablet, landscape, and resizable previews.
  Preview width and theme changes now preserve mounted demo state.

- Reject staged local shared-library dependencies in the commit hook.

- Publishing waits for the quality gate on the selected commit.
- Check the declared Rust 1.88 minimum in CI and before publishing.

### Fixed

- Present modals, alerts, popovers, sheets and toasts in a shared top-layer order;
  toasts remain above blocking overlays. Nested Escape/focus return follows the
  active surface, with hidden-page initialization and live sheet modality.
- Preserve button taps inside swipeable tabs; track tab swipes in the WebView,
  keep leaving panels inert, follow replacement selection signals, propagate
  Tabs mode to TabList, and derive animation direction from current DOM order.
- Cancel stale debounced input notifications after clearing or external changes;
  readonly inputs no longer expose a clear action.
- Use the local device date/time on native targets and reject trailing fields
  when parsing `TimeOfDay`.
- Enforce unavailable dates and bounds in all date picker styles and at commit;
  calendar navigation can focus unavailable dates without selecting them. Wheel
  time selections round down to `minute_step` consistently before confirmation.
- Follow changed alert input defaults, picker bounds, toast durations, sheet
  drag settings, refresh strings/thresholds and infinite-scroll thresholds.
- Dispose DOM listeners, observers and timers on reconfiguration/unmount;
  cancelled refresh gestures no longer trigger refresh. Scope script failures
  to development diagnostics and remove image-state writes during rendering.
- Keep horizontal tab-list Up/Down keys available for scrolling. Respect logical
  RTL edges in swipe, rating and horizontal strip interactions, and allow page zoom.
- Keep automatic tooltip descriptions current when their trigger is replaced,
  and remove only library-owned descriptions during cleanup.
- Closed overlay surfaces and backdrops no longer intercept the next click
  while their top-layer wrapper remains for its exit animation.
- Keep sheets, action sheets and toasts on screen in an `AppWrapper` that
  starts below other content and runs past the window: the confined top layer
  is clipped to the visible viewport (`visualViewport` where present) instead
  of taking the app's whole box below the fold. The playground's Mobile
  preview also fits the space under its header.

### Added

- `use_overlay_controller()` / `OverlayController::{count,dismiss_top}` for
  Android Back handling across overlay types.
- `Calendar::reference_date` and `DatePicker::reference_date` for deterministic
  server/client date rendering.
- Localized rating speech and date-format templates in `Strings`.
- Mounted-state regressions, production-script lifecycle tests in `just pre-push`,
  and a browser regression recipe covering modes, sizes, themes and touch taps.
- Document signal-driven forms, notification semantics, browser capabilities,
  and Android runtime/performance validation.

### Migration

- Full `Strings` struct literals must add `rating_unrated`, `rating_value`,
  `date_long`, `date_short` and `date_month_year`; prefer `..Strings::default()`.
- RTL `SwipeState::offset` and `ratio` use logical signs: positive uncovers
  Start and negative End, matching LTR behavior. Physical transforms remain internal.
- Off-step wheel times now commit the displayed rounded value; applications that
  need arbitrary minutes should use a one-minute step or the dialog style.

## [0.5.1] - 2026-10-08

### Fixed

- A toast is opaque. It sat on `--g3-color-card`, so a theme with a translucent
  card showed the page through it; it now uses `--g3-color-surface`, the token
  sheets and menus float on.

## [0.5.0] - 2026-10-08

### Added

- `Look`, set on `Theme::look`: a page gradient (`background`), card shape
  (`card_radius`, `card_border`, `card_shadow`, `card_sheen`,
  `card_backdrop_filter`) and the fill of solid accent buttons (`accent_fill`),
  written as `--g3-page-background`, `--g3-card-*` and `--g3-accent-fill`.
  Unset fields keep the platform look. The shell, `TabLayout`, `Content` and
  the route-transition layers all paint the page background, so a gradient
  shows through while pages slide.
- `Theme::accent_secondary` (`--g3-color-accent-secondary`), a second brand
  color for gradients such as `accent_fill`. It defaults to the accent, and
  `Theme::with_accent` moves it too while they are still the same.
- `Theme::adaptive` keeps both looks when they differ (`Theme::look_dark`),
  since a gradient cannot be a `light-dark()` value; `Theme::look_css` writes
  the `prefers-color-scheme` rule.
- A `ThemeProvider`, or an `AppWrapper` inside another, replaces the look of
  its parent instead of inheriting the properties it leaves unset.
- Playground themes **Glass** (light) and **Ember** (dark) show a `Look`.

### Changed

- A plain `List` inside a `Card` no longer repaints the card color on its rows:
  they show the card's own surface. Nothing changes for an opaque card, but a
  translucent or sheened one no longer shows a slab behind the list.
- `Theme` has three new public fields (`accent_secondary`, `look`,
  `look_dark`), and `Theme::tokens` has a seventeenth entry. A `Theme` built
  with a full struct literal, without `..Theme::default_*()`, needs them.
- `Shelf` titles use `Text`'s `Overline` size, weight and tracking, so a shelf
  heading and an overline heading above a list read as one style.

### Added

- `NavRail` and `AdaptiveNav` rails start expanded: a labelled row per
  destination instead of icons alone, with a button at the foot that collapses
  them to icons (and back). `default_expanded` sets the starting state,
  `expanded` takes a `Signal<bool>` for an app that remembers the choice, and
  `collapsible: false` removes the button. `Strings` gains
  `collapse_navigation` and `expand_navigation`. Rails used to be icons only, so
  pass `default_expanded: false` to keep that look.
- `--g3-toast-top` and `--g3-toast-bottom`: where a top or bottom toast sits. An
  app with a tab bar or other chrome along an edge sets one so its toasts clear
  it instead of covering it. Unset, a toast keeps its place: `1rem` in from the
  edge, or the safe area when that is larger.

### Fixed

- A touch swipe on a `SwipeItem` no longer snaps back after its first frame. A
  touch starts out captured by whatever the finger pressed, and the row taking
  capture over fired a `lostpointercapture` that was read as a release. Only the
  row losing its own capture ends a gesture now.
- A `Header` toolbar stays centred on a wide shell. The single-row header used
  `auto` columns for the start and end slots, so a wide end (a search box) pushed
  the segmented control off-centre. The start slot and title now share one
  column, mirrored by the end slot's, via a new `g3-header-lead` wrapper.

## [0.4.4] - 2026-09-28

### Fixed

- A phone's bottom tab bar no longer vanishes while a routed sheet rises
  over it or falls away. `TabLayout` now carries g3-route-transitions'
  `ROUTE_TRANSITION_PAGE_FRAME_CLASS`, so the layout, bar included, is what
  dims under the sheet. The `transitions` feature requires
  g3-route-transitions 0.4.3.

## [0.4.3] - 2026-09-28

### Changed

- The `transitions` feature requires g3-route-transitions 0.4.2, whose
  navigation no longer hangs on a page the browser does not paint.

### Fixed

- A tappable row inside a `SwipeItem` (an `Item` with `onclick`, a `Card`
  with `onclick`) opens again when tapped. The swipe script captured the
  pointer on every press, so the release, and the click after it, landed on
  the row instead of the control under the finger. It now captures once the
  pointer moves past the tap slop.
- `AppWrapper` renders its stylesheet `document::Link` on the web client too.
  Rendering it only on native builds meant a fullstack server wrote one more
  head-element hydration entry than the wasm client read, so every later entry
  was off by one and each page load logged `Error deserializing data: ...
  expected string` for the first `ErrorBoundary` (every `Content`).

## [0.4.2] - 2026-09-24

### Added

- `ReorderList` lays out grid cells as well as rows (`layout:
  ReorderLayout::Grid`): a dragged cell steps into the next slot, which may be
  at the other end of the row above or below.

### Changed

- Swipe rows, reorder lists, star ratings and the clock face now follow the
  pointer in the page itself and report only what the gesture decided. On
  native renderers every Rust event handler is a round trip that blocks the
  WebView, so a drag handled in Rust lagged the finger, badly on Android.
- A dropped reorder item glides from the pointer into its slot, and a
  cancelled drag glides back, instead of jumping.
- A shelf dragged with a mouse coasts to a stop at the speed it was let go,
  and a wheel eases it along; neither snaps to an item. Snapping is now for
  touch alone, where it is mandatory so a fling lands on an item in one
  motion.

### Fixed

- A dragged reorder item trailed behind the pointer: it kept the list's
  transform transition, whose rule outranked its own.
- `AppWrapper` adds `viewport-fit=cover` to the viewport, without which a
  WebView reports every safe-area inset as 0 and Android 15's edge-to-edge
  status bar covers the header.

## [0.4.1] - 2026-09-22

### Fixed

- A dark theme painted a near-white page behind the app and flashed near-white
  through every route transition. The theme is applied inline on the shell,
  but `g3-route-transitions` paints `html, body` from `--route-transition-bg`,
  which g3-ui maps to `--g3-color-bg`, and every `::view-transition-*`
  pseudo-element is a child of `:root` and resolves that variable there too.
  Both therefore took the stylesheet's light default no matter which theme was
  active. `AppWrapper` now mirrors the active theme's page background and
  `color-scheme` onto the document element, so the canvas and every transition
  snapshot match the theme. A nested `AppWrapper`, such as a demo frame inside
  a gallery, leaves the document element to the outermost one.
- A navigation rail is now the persistent route-transition chrome that
  `AdaptiveNav` and `NavRail` always documented it to be. The class was never
  applied, so the rail sat inside the base region: a routed sheet covered it
  and it popped back when the transition ended. `NavRail` is persistent at
  every width and `AdaptiveNav` only on wide shells, so a phone's bottom bar
  still rises under a sheet. Needs the `transitions` feature.

## [0.4.0] - 2026-09-21

0.4 reworks the public API for use as a general-purpose library. Most
components are renamed, several are split, and the stylesheet moves into a
cascade layer. The README has a table of the most common renames.

### Removed

- **Breaking:** the `G3`-prefixed aliases. Use the unprefixed names; import
  one under an alias if it collides with a local name.
- **Breaking:** `g3_ui::prelude` no longer re-exports `dioxus::prelude`.
- **Breaking:** the `*_styles.rs` class-name constants. Class names are
  internal; style components through their `class` prop, `data-state`, and
  ARIA attributes.
- **Breaking:** `SheetButton` (use `InfoButton { sheet, .. }`), `ItemKind`
  (an `Item` now infers its element from its props), `RefresherState`,
  `can_refresh`, and `Card`'s `RightSlot` and `top_margin`.
- **Breaking:** `InputType`'s `Date`, `Time`, `DateTime`, and `Month`. They
  opened the browser's own picker, which ignores the mode and the theme. Use
  `DatePicker` and `TimePicker`, which also carry a typed value instead of a
  string; a date and a time field side by side cover `datetime-local`.

### Fixed

- Every documentation example compiles. They were `rust,ignore`, so nothing
  checked them against the API, and 0.4 renamed half of it. Each now carries
  hidden scaffolding - the imports, a wrapper, and stand-ins for the names it
  invents - so it builds as a doctest while the rendered page shows only the
  example. The one exception is the route-transitions snippet in the README,
  which needs the optional `transitions` feature and its crate.
- Package only the crate's own root files. The `include` patterns were
  unanchored, so `deploy/README.md` shipped in the crate.
- The stylesheet names its layer order (`theme, base, g3, components,
  utilities`), so Tailwind's reset no longer flattens the components.
- `RadioGroup` and `AccordionGroup` follow a new `value` signal passed on a
  later render.
- An `Item` with controls in `end` keeps them out of its own button, so no
  button is nested in another.
- Cards in a `Stack` or `Grid` are spaced by its gap alone; a card's own
  margin no longer adds to it.
- `Rating` stars shrink to fit a narrow column, and a press still lands on
  the star under it.
- A `SwipeItem` holds content of any height and collapses from that height
  when dismissed. A vertical scroll no longer runs on under a sideways drag,
  and with `mouse_swipe: false` its actions button shows on hover.
- Rows in a `ReorderList` are separated like any other list's rows.
- A `Badge` with an `aria_label` keeps an icon beside its text, and `Avatar`
  initials skip leading punctuation, so "@ada" reads "A".
- A `RouteTransitionPage` inside a `TabLayout` fills the space beside the
  navigation.

### Changed

- **Breaking:** `to` and `BackButton`'s `default_to` take a `Destination`
  rather than an `Option<NavigationTarget>`, so a route goes straight in -
  `Item { label: "Profile", to: Route::Profile {} }` - the way Dioxus's own
  `Link` takes one. That form is what the documentation always showed, but it
  did not compile: Dioxus converts into an optional prop only for
  `Option<String>`, so every other caller had to spell out
  `NavigationTarget::from(..)`.
- A dismissed `SwipeItem` leaves by the row's own width instead of a fixed
  430px, so a wide desktop row no longer stops partway across.
- A `Searchbar` in a `Header`'s `toolbar` keeps the room a field needs from
  the header's edges. A segmented strip still sits flush, as it should.
- `BottomSheet`, `SideSheet`, and `NavigationDrawer` take `padding`, the
  same way `Content` does, so a drawer whose rows are its navigation can run
  them to its edges.
- The playground's two showcases are coherent mini apps. `DemoApp` gives each
  tab one job - a feed that refreshes, a booking form, an edge-to-edge inbox,
  and a settings page - and puts the segmented control and the searchbar in
  the header where they belong. `RouteTransitions` is a listening app whose
  every screen says which transition brought it in.
- **Breaking:** renamed components:
  - `Body` → `Content`, with `loading` and `error` fallbacks.
  - `Navbar` → `TabLayout`.
  - `Field` → `Input` and `TextArea`.
  - `Line` → `Divider`.
  - `ItemDivider` → `ListHeader`.
  - `FabContainer` → `FabMenu`.
- **Breaking:** `Sheet` is split by placement. `BottomSheet` rises from the
  bottom and supports `detents`. `SideSheet` takes a logical `SheetEdge`
  and a `SideSheetBehavior` of `Overlay`, `Push`, or `Reveal`.
  `NavigationDrawer` replaces the `Menu` side-sheet type; it is persistent
  navigation, not a dialog.
- **Breaking:** `NavbarTabBar` and `NavbarTab` are replaced by
  `AdaptiveNav`, which is a bottom bar on phones and a rail on wide shells,
  the fixed forms `NavBar` and `NavRail`, and `NavItem`. The old
  `visibility: RailOnly` is `AdaptiveNav { compact: AdaptiveNavCompact::Hidden }`,
  and `desktop_placement: Bottom` is `group: NavItemGroup::Secondary`.
  `NavItem` takes `to` or `href` and marks the current item with
  `aria-current`.
- **Breaking:** `Card`'s `inset: bool` is now `variant: CardVariant`:
  `Raised` (the default, with a shadow), `Flat` (a border and no shadow), or
  `Filled` (a tinted surface, the old `inset`).
- **Breaking:** `List`'s `inset: bool` is now `variant: ListVariant`:
  `EdgeToEdge` (the default) or a rounded group drawn like a card:
  `Raised` (the old `inset: true`), `Flat`, or `Filled`.
- **Breaking:** `SwipeItem`'s `behavior` is split into `start_behavior` and
  `end_behavior`, so each edge can reveal, activate, or dismiss on its own.
  Revealed actions are as wide as their buttons, and pressing one closes
  the row.
- **Breaking:** `Theme::warning` defaults to yellow (`#f5b400`) instead of
  orange, with a new `on_warning` field for text drawn on it. Warning text
  and outline buttons darken it so it stays readable.
- **Breaking:** consistent prop names across components:
  - `is_open` → `open`.
  - `active` and `index` → `value`.
  - `Button`'s `style: ButtonStyle` → `fill: ButtonFill`.
  - `disable_text_selection` → `text_selection`.
  - `FabSize::Normal` → `FabSize::Regular`.
  - `SwipeState::full` → `committed`.
- **Breaking:** color variants share one `Color` enum (`Accent`,
  `Neutral`, `Success`, `Warning`, `Danger`) across `Button`, `Badge`,
  `Toast`, `MenuItem`, `SwipeAction`, and `Text`.
- **Breaking:** `Select`, `RadioGroup`/`Radio`, `SegmentGroup`/`SegmentButton`,
  and `AccordionGroup` are generic over their value type instead of using
  strings or indexes.
- **Breaking:** value signals are optional. Without one, a component keeps
  its own state. Change callbacks receive the new value (`EventHandler<T>`),
  not a DOM event.
- **Breaking:** `Theme` fields are renamed to match their tokens: `focused` →
  `accent`, `card_border` → `border`, and `label_secondary` →
  `text_tertiary`. `label_primary` and `card_inset` are removed. New fields
  are `on_accent`, `surface`, `control`, `success`, `warning`, and `danger`.
  `with_focused` is now `with_accent`.
- **Breaking:** CSS custom properties use a `--g3-` prefix
  (`--g3-color-accent`, `--g3-transition-normal`, `--g3-nav-rail-width`).
- **Breaking:** the stylesheet is in the `g3` cascade layer and no longer uses
  `!important`, so any unlayered app rule overrides it. Component state is
  exposed through `data-state` and ARIA attributes rather than modifier
  classes. Overlays use a shared z-index scale (`--g3-z-*`).
- **Breaking:** `ConfirmModal`'s `on_confirm` is an `EventHandler<()>`, and
  the modal is an `alertdialog`.
- `Card` is a single stretched link or button when it has `onclick`, `to`,
  or `href`, so controls inside it stay separately focusable.
- `Chip` renders a plain `span` unless it is pressable.
- `Button`, `FabButton`, and `Input` pass other HTML attributes through to
  their element.
- Changing `AppWrapper`'s or `ThemeProvider`'s `mode` now updates the
  subtree.
- The `transitions` feature now depends on `g3-route-transitions` 0.4 and
  uses its region vocabulary. `AppWrapper` marks the shell with
  `ROUTE_TRANSITION_OVERLAY_REGION_CLASS` and loads `RouteTransitionStyles`.
  `TabLayout` marks itself with `ROUTE_TRANSITION_BASE_REGION_CLASS`.
- **Breaking:** renamed `AppWrapper`'s `route_transition_root` prop to
  `route_transition_overlay`. "Root" now names a route layer
  (`layer = stack_root`) in `g3-route-transitions`, and this prop controls
  the overlay region.
- The minimum supported Rust version is 1.88.

### Added

- `DatePicker` and `TimePicker`: form fields that open an iOS wheel picker
  or a Material calendar or clock dialog, chosen by `PickerStyle`. Both also
  open in a popover anchored to the field, which becomes a sheet on phones.
  The Material time dialog has a typing mode.
- `Calendar`: a month grid for picking a date, usable on its own. Arrow keys
  move by day, Home and End reach the ends of the week, and Page Up and Page
  Down change month or year; the title opens a year list.
- `CalendarDate` and `TimeOfDay` carry the values, parsing from and printing
  as ISO 8601, with `today()`, `now()`, and calendar arithmetic. `HourCycle`
  picks a 12- or 24-hour clock.
- `Strings` gained month and weekday names, the first day of the week, AM and
  PM, and the picker labels.
- `use_toast`, `use_alert`, and `use_action_sheet` open overlays from code.
  `AppWrapper` hosts them. Alerts and action sheets return a future that
  resolves to the user's answer.
- New components:
  - Overlays: `Alert`, `ActionSheet`, `Popover`, and `Menu`/`MenuItem`.
  - Navigation: `Tabs`/`TabList`/`Tab`/`TabPanel` and a router-aware
    `BackButton`.
  - Forms: `Range`, `Stepper`, and `Searchbar`.
  - Content and layout: `Text`, `Img`, `Tooltip`, `Stack`, and `Grid`.
  - Feedback: `InfiniteScroll`.
- `Theme::system()` and `Theme::adaptive(light, dark)` follow the operating
  system's color scheme through CSS `light-dark()`.
- `Strings` holds every built-in label, for translation.
- `use_theme`, `use_strings`, and `use_component_mode` read the values in
  effect.
- `BottomSheet` takes `backdrop_detent`: below that detent the page stays
  visible and usable. Detents are fractions of the app's height, so `1.0`
  fills it; they were fractions of the browser window. Detents can change
  while the sheet is mounted.
- `Content` takes `width: ContentWidth` (`Full`, `Readable` at 40rem, `Wide`
  at 60rem) to narrow its children on wide shells while the scrollbar stays at the page
  edge, and `on_refresh`/`refreshing` for pull to refresh.
- `Select` takes `width: SelectWidth`: `Fill` (the default), `Fit` to its
  chosen option, or a fixed `Sm`, `Md`, or `Lg`.
- Popovers, menus, and `Select` lists flip to the other side of their
  trigger when they would run past the edge of the app.
- `Grid` takes `wide_columns` and `wide_gap` for shells `48rem` and wider.
- Scrollable `SegmentGroup`s scroll by mouse drag and wheel as well as
  touch, fade the edge that has more buttons, and keep the selected button in
  view.
- `Button` takes `loading`, `button_type`, `to`, `href`, `new_tab`, `start`,
  and `end`.
- `Input` takes `clearable`, `start`, `end`, and `debounce_ms`, and supports
  date and time types.
- `SwipeItem` works with a mouse and the keyboard, and has a "Show actions"
  button for assistive technology.
- `Modal` takes a `role` (`ModalRole`) and `size` (`ModalSize`).
- `Toast` takes an `action`, `closable`, and `ToastDuration::Persistent`.
- Sheets, modals, and popovers trap focus, close on Escape, restore focus
  when they close, and lock page scrolling while open.
- Arrow keys move between radios, segments, and tabs.
- Every public item is documented, and docs.rs builds with the
  `transitions` feature.
- `Table`: a data table in a focusable, scrollable region, with an optional
  sticky first column. Cells take a `Color` and a muted tone.
- `ReorderList`, `ReorderItem`, and `ReorderHandle`: rows reordered by
  dragging the handle, or with the arrow keys, announced to screen readers.
- `Divider` takes a `label`, drawn between two rules, as in "or continue
  with email".
- `Card` takes `start`, content before the title, such as an avatar.
- `Item` takes `wrap`, which lets its label and description wrap instead of
  being cut short.
- `ToastOptions::replace` shows a toast at once, in place of the one showing
  and any waiting.
- `Color::as_str` is public, for styling your own elements by color.

### Fixed

- Sheets animate in and out over 0.5s on a softer curve instead of 0.25s,
  which read as popping in. The side sheet backdrop fades instead of sliding
  in with the sheet.
- A `BottomSheet` with detents slides in instead of appearing at the end
  of its entrance: the rules sizing it also delayed it becoming visible. It
  slides in at its resting height, and later detent changes still animate.
- A popover shown as a bottom sheet on a phone slides and fades on the
  sheet timing.
- An `AppWrapper` with `layout: false` is still the container its wide
  layout rules measure, so a `Select` in such a page opens as a menu on wide
  screens instead of a full-width sheet.
- A swipe released outside its row finishes instead of leaving the row
  stuck mid-drag.
- Accordion panels open without a jump: the padding no longer snaps in
  before the height animates.
- Wide `Content` shows a thin scrollbar without arrow buttons.
- A `Refresher` pull released outside the refresher, or outside the
  window, lets go instead of staying pulled. The refresher fills its scroll
  area, so a pull can start below short content, and it no longer becomes
  the containing block for sheets inside it.
- Neutral outline buttons, such as `ConfirmModal`'s Cancel, use the border
  color instead of a solid text-colored outline.

- The desktop rail no longer dims and scales with the page when a routed
  sheet opens, and the sheet no longer leaves an empty rail-width gap. With
  the `transitions` feature, the rail is route-transition persistent chrome,
  which stays in place above the rising sheet.
- `SideSheet` push and reveal and `NavigationDrawer` resize the page even
  when `AppWrapper` has `layout: false`.
- Generated element ids are unique, so labels, descriptions, and
  accordion panels no longer collide when a component is used twice.

### Accessibility

Every playground demo passes an axe-core audit in both modes.

- Toasts pause their timer while the pointer is over them or focus is
  inside, and a toast with an action defaults to the long duration.
- A loading `Button` stays focusable: it is `aria-disabled` and busy
  rather than `disabled`, so pressing it does not drop focus to the page.
  Its spinner is no longer a second live region.
- Links that open a new tab say so to screen readers.
- A `SwipeItem` inside a `List` is the list item; the row inside it no
  longer claims to be another one.
- `Card`, `AccordionItem`, and `ListHeader` take `heading_level`, and
  default to `h2` instead of `h3` so a page's outline does not skip a
  level.
- `NavItem`s keep their names in a rail, where the visible label is
  hidden.
- `Menu` and `Popover` set `aria-haspopup`, `aria-expanded`, and
  `aria-controls` on their trigger. Menus and `Select` lists jump to the
  next item starting with a typed letter, and `Select` opens with the
  arrow keys.
- `Stepper` supports Home, End, PageUp, and PageDown.
- `Tooltip` describes its trigger automatically, stays up while hovered,
  and hides on Escape.
- `Badge` takes an `aria_label` for counts that need context.
- The pull-to-refresh prompts are no longer announced on every movement;
  only the refresh is.
- `Content`'s scroll area can take keyboard focus, so a page with nothing
  focusable can still be scrolled from the keyboard.
- Fields, checkboxes, radios, and steppers use a new
  `--g3-color-control-border` with at least 3:1 contrast. The dark preset's
  accent and danger colors meet 4.5:1 as text, and its fills carry dark
  text (`on_accent` is `#04162b`).
- Focus rings for ranges, steppers, and chips; Windows High Contrast
  support; reduced motion is respected when segments scroll.

### Documentation

- The README covers the 0.4 API, overlays opened from code, styling, and
  upgrading from 0.3.
- The route-transition section lists the region each component provides,
  where to add `RouteTransitionPage`, and the layout rules for sheet routes,
  segmented screens, and nested wrappers.

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
- Stop the swipe action color bleeding through as a hairline along the bottom of the last swipeable row: the dragged content composites separately from the actions beneath it, and on a fractionally-tall row the two rasterize to different device pixels. Rows above the last were only ever covered by their own divider.
- Hold a swiped row's action color underneath until the row has slid back over it. The row reported itself closed the instant the drag was released, which hid the actions immediately and left the row animating home across bare card.
- Link the playground stylesheet into the head at build time. Loading it at runtime left a window where the library stylesheet had applied but the playground's had not, so the device frame rendered unstyled while the app booted. The page background during that window is white, so it reads as the browser's own blank page rather than a color of its own.
- Drop the compact-shell/wide-shell chips above each playground preview; the header toggle already names the width.
- 25 mobile-first components with iOS and Material Design variants of each.
- Configurable `Theme` of 17 CSS custom-property tokens, with built-in light
  and dark themes.
- Optional `transitions` feature integrating `g3-route-transitions`.
- Safe-area insets and `prefers-reduced-motion` handled in the stylesheet.
