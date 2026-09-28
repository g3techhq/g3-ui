# AGENTS.md

Instructions for coding agents working in this repository (Claude Code, Codex,
Cursor, Copilot, and anything else that reads `AGENTS.md`). Useful for people
too. Follow these over your defaults.

## What this is

**g3-ui** is the component library of the g3 stack: mobile-first
[Dioxus](https://dioxuslabs.com/) components that match
[Ionic](https://ionicframework.com/)'s design and behaviour, in iOS and
Material modes, for web and for desktop, Android and iOS web views. Its code
structure follows eq_ui. Apps built on it: media-mancer, greenside-partee,
tawny, and the g3-stack template.

| Piece | Version | Reference |
| --- | --- | --- |
| Dioxus | 0.7.9 | [dioxuslabs.com/learn/0.7](https://dioxuslabs.com/learn/0.7/), and g3-stack's `docs/dioxus/patterns.md` |
| g3-route-transitions (optional, `transitions`) | 0.4 | [README](https://github.com/g3techhq/g3-route-transitions) |
| Rust | 1.88+, edition 2024 | `rust-toolchain.toml` |

**Your training data is probably wrong about Dioxus 0.7** (signals, props,
assets and `document::eval` all changed). Prefer the patterns already in
`src/` over what you remember.

## Map

```
src/lib.rs              Public exports and the crate docs
src/prelude.rs          What `use g3_ui::prelude::*` brings in
src/theme.rs            Theme, ComponentMode, Strings, merge_classes, ambient context
src/state.rs            use_controlled, use_synced_signal, live context, element ids
src/components/         One file per component (plus its .js where the DOM needs one)
src/components/mod.rs   Exports, and the playground `gallery!` list
src/tests/              Markup and ARIA tests rendered with dioxus-ssr
assets/g3-ui.css        The one stylesheet, inside `@layer g3`
playground/             The live gallery (g3ui.g3tech.net), a separate crate
CHANGELOG.md            Every user-visible change, under [Unreleased] until a release
```

## Commands

```bash
just check        # the crate, with and without `transitions`, and the playground
just test         # nextest (both feature sets) + doc tests
just lint-strict  # clippy with warnings as errors, as CI runs it
just format       # rustfmt (crate and playground)
just pre-push     # everything above plus typos
just ci           # pre-push + cargo-deny + package check
```

## Definition of done

1. `just pre-push` passes.
2. A user-visible change has a line under `## [Unreleased]` in `CHANGELOG.md`.
3. A new or changed component has markup tests in `src/tests/` and is shown
   in the playground.
4. A visual or gesture change was looked at in the playground, in both modes,
   at a phone width and a wide one. A change a consuming app depends on was
   also checked in that app, through a `[patch.crates-io]` path override that
   is removed afterwards.

If you could not do one of these, say which and why.

---

## Rules

### Components

- **Match the equivalent Ionic component** in look and behaviour, in both
  `ios` and `md` modes. A component uses its `mode` prop, then the nearest
  provider, then the global default.
- **Names are plain Ionic-style names** (`Button`, `BottomSheet`, `TabLayout`),
  unprefixed, with no aliases.
- **Every prop is typed**: enums for variants, sizes and positions, no magic
  strings. Optional props are `Option<T>` with the default documented on the
  prop. Every component and every prop has a doc comment.
- **Every component takes `class: Option<String>`**, merged with
  `merge_classes()`, and most take `mode`.
- **Slots follow Ionic:** `start`, `end`, `title`, `toolbar`; body content is
  `children`.
- **Names:** `open` for visibility, `value` for selection, `disabled`,
  `aria_label`, `mode`, `class`. Colors use the shared `Color` enum.
- **State ownership:** a component with a primary value takes an optional
  `Signal<T>` through `state::use_controlled`, so it keeps its own state when
  none is passed. Overlays take a required `open: Signal<bool>`. Pair the
  signal with an optional `onchange` that receives the new value, not a DOM
  event. A control that follows something else offers `defer_selection`.
- **Full WAI-ARIA** on interactive components: roles, states, keyboard
  navigation, roving tabindex for composite widgets, decorative parts
  `aria-hidden`, live regions for dynamic content.
- New component checklist: file in `src/components/`, `g3_playground!` in it
  and a line in `gallery!`, markup tests, docs on every prop, ARIA, both modes,
  builds for wasm32 and native (check timer feature gates).

### Dioxus inside components

Consuming apps render these components on busy pages, so the library holds
itself to the rules the apps follow (g3-stack's `docs/dioxus/patterns.md`):

- **Hooks run unconditionally, in the same order, before any early return.**
  `use_effect` keeps its first closure.
- **No `use_reactive!`.** A plain prop that a hook must follow goes through
  `use_synced_signal`, and the effect reads the signal. `use_synced_signal` is
  the one sanctioned signal write during render (see its doc comment); do not
  write a signal in a component body anywhere else.
- **A context whose value follows props** uses `provide_live_context` /
  `use_live_context`, never a `use_context_provider` initialized once from
  props.
- **Handlers passed as props are called with `.call()`** and read what they
  need when they run. An effect that stores a handler (a script's message
  loop) calls it through the `EventHandler` handle, which Dioxus re-points on
  every render, so it stays current.
- **Never hold a signal borrow across `.await`.**
- **Gate calls, never markup.** The server renders what the web client
  hydrates; a tree that differs between `cfg`s breaks hydration. Every head
  element (`document::Link`) renders on every build.
- **Element ids come from `use_element_id`**, so server and client agree.

### Behaviour that needs the DOM

- Prefer Rust and CSS. Where the DOM is needed (focus trapping, gestures,
  scroll locking), the script runs in the web view and reports to Rust only
  how a gesture ended: a Rust handler per pointer move is too slow on Android.
  Escape every Rust string passed into a script with `overlay::js_string`.
- **A script must not break the click that ends a tap.** Pointer capture on
  press retargets the release and the click to the capturing element, so a
  button inside it never fires. Capture once the pointer has moved past the
  tap slop (see `swipe.js`).
- Code that waits a frame must tolerate a hidden page, where
  `requestAnimationFrame` never fires.

### Styling and theming

- **One stylesheet.** Components write their `g3-` class names inline; the
  rules live in `assets/g3-ui.css` inside `@layer g3`, which is all a consumer
  links. A test fails if a component emits a `g3-` class the stylesheet does
  not define.
- **Colors come from `--g3-color-*` tokens** or `color-mix()` over them. The
  only literals are intentional third-party brand colors and neutral
  `rgba(0,0,0,..)` shadows and scrims.
- State is styled from `data-state` and ARIA attributes, not modifier classes.
  No `!important`; the layer lets any unlayered app rule win.
- Theme switching is reactive through the prop: `AppWrapper` writes the theme
  as inline custom properties, so a new `Theme` re-themes without a remount.
  `color-scheme` belongs to the theme, never to a mode selector.
- Responsive rules query `@container g3-app-shell`, never the viewport.

### Releases

- Every user-visible change goes under `## [Unreleased]` in `CHANGELOG.md` in
  the same commit, including renames with their migration.
- Releases publish from `publish-crate.yml` (crates.io trusted publishing).
  Do not run `cargo publish` by hand, and do not release without the
  maintainer's go-ahead.
- Commits follow Conventional Commits; lefthook checks them.

---

## Verifying in the playground

- Start the `playground` configuration in `.claude/launch.json`
  (`dx serve --port 8080` in `playground/`), or reuse one already running.
  If port 8080 belongs to another project, use the next free port.
- Look at the component in both modes, light and dark, at a phone width and a
  wide one, and with the keyboard.
- For a change an app depends on, patch the app to this checkout:
  `[patch.crates-io] g3-ui = { path = "../g3-ui" }`, run its checks and UI
  tests, then remove the patch and restore its `Cargo.lock`.

## Where to look

- `src/lib.rs` crate docs and each component's rustdoc
- [docs.rs/g3-ui](https://docs.rs/g3-ui), and the playground at
  [g3ui.g3tech.net](https://g3ui.g3tech.net)
- `CHANGELOG.md` for what changed between versions
- `docs/history/` for the original design plan and spec
