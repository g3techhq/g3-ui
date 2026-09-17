# g3-ui - Goals & Philosophies

> Reference document for AI agents working on g3-ui.
> Read this before making any changes to the crate.

---

## Identity

g3-ui is the Dioxus component library for the Greenside Partee project. It uses a flat component taxonomy and borrows its design philosophy and component behavior from **Ionic Framework**.

---

## Foundational Influences

### 1. eq_ui - Code Structure & Architecture

g3-ui adopts the following structural conventions from eq_ui:

- **Flat Component Module**: Components live together in `src/components/`; metadata groups them as Components or Utilities.
- **One stylesheet**: Components write their `g3-` class names inline. The matching rules live in `assets/g3-ui.css`, inside `@layer g3`, which is the only stylesheet a consumer links. A test fails if a component emits a `g3-` class that stylesheet does not define. Shared tokens live in `theme.rs`.
- **CSS Custom Properties**: Colors and theming via CSS variables defined in theme CSS files.
- **Theme Context**: `AppWrapper` and `ThemeProvider` provide mode, theme, and `Strings` as context; read them with `use_component_mode`, `use_theme`, and `use_strings`.
- **Playground Registration**: Each component module invokes `crate::g3_playground!` once and is listed in the `gallery!` macro in `src/components/mod.rs`.
- **Markup Tests**: `src/tests/` renders components with `dioxus-ssr` and checks their markup and ARIA contracts.
- **`class` Prop**: Every component accepts `class: Option<String>` for per-instance overrides, merged with `merge_classes()`.
- **WAI-ARIA**: Full ARIA roles/attributes on all interactive components.

### 2. Ionic Framework - Design & Behavior

g3-ui matches Ionic's component behaviors and design language:

- **Adaptive Styling**: Components have `ios` and `md` (Material Design) modes. A component uses its `mode` prop, then the nearest provider, then the global default. This is the clearest indicator of Ionic inspiration.
- **CSS Variables Theming**: Ionic uses CSS custom properties as its entire theming system. g3-ui inherits this approach through eq_ui's CSS variable system.
- **Shadow Parts**: Where applicable, components expose CSS shadow parts for deep customization.
- **Parallel Navigation**: Tabbed interfaces with independent navigation stacks per tab.
- **Platform-Specific Visuals**: iOS (rounded, thin borders, subtle shadows) vs MD (Material Design, card elevation, box shadows).
- **Component Slots**: Toolbar components use named slots (`start`, `title`, `end`, `secondary`) matching `ion-toolbar`.
- **Safe Area Awareness**: Mobile shells respect `env(safe-area-inset-*)` for iOS notch/home indicator padding.

---

## Core Goals

### 1. Match Ionic Design

When implementing a component, the visual target is to match the equivalent Ionic component. The iOS/MD mode distinction is the primary signal of this goal.

### 2. Follow eq_ui Structure

Code should mirror eq_ui patterns:
- Components in the flat `src/components/` module area
- Rules in `assets/g3-ui.css`
- `theme.rs` for shared tokens + `merge_classes()` utility
- Each component gets markup tests in `src/tests/`

### 3. Dioxus-Native

All components compile and run on Dioxus (web/WASM, desktop/Wry, mobile). Prefer pure Rust/Dioxus and CSS. Where a behaviour needs the DOM (focus trapping, drag gestures, scroll locking), use `web-sys` on wasm and a small `document::eval` script elsewhere, and escape any Rust string passed into a script with `overlay::js_string`.

### 4. Platform Portability

Components must work across platforms:
- **Web**: WASM via Dioxus web
- **Desktop**: Wry webview
- **Mobile**: iOS/Android native
- **Native** (future): Blitz GPU renderer

---

## Component Design Principles

### Naming
- Public components use plain Ionic-style names (`Button`, `BottomSheet`, `TabLayout`) with no prefix. Do not add prefixed aliases.

### Props
- All props are typed Rust values (no raw string magic).
- Enums for variants, sizes, positions, etc.
- `children: Element` or `Option<Element>` for slot-based composition.
- Optional props are `Option<T>`; defaults are documented on the prop.
- **State ownership**: a component with a primary value (`checked`, `value`) takes an optional
  `Signal<T>` through `state::use_controlled`, so it keeps its own state when none is passed.
  Overlays take a required `open: Signal<bool>`. Pair the signal with an optional
  `onchange`/`on_*` callback that receives the new value, not a DOM event.
- **Names**: `open` for visibility, `value` for selection, `disabled`, `aria_label`, `mode`,
  `class`. Colours use the shared `Color` enum.

### Slots
- Follow Ionic's slot naming: `start`, `end`, `primary`, `secondary` for toolbar components.
- Body content passed as `children` or explicit `Element` props.

### Theming
- CSS variables for all color/sizing tokens. Component CSS must reference `--g3-color-*` tokens (or
  `color-mix()` over them) rather than literal colors, so custom themes take effect everywhere. The
  only sanctioned literals are intentional third-party brand colors (e.g. the Google provider
  button) and neutral elevation/scrim shadows (`rgba(0,0,0,...)`).
- Theme switching is reactive via the prop, not a `Signal<Theme>`: `AppWrapper`/`ThemeProvider`
  render the theme as inline CSS custom properties on the shell element, so passing a new `Theme`
  (e.g. from a signal flipping light/dark) re-themes the whole subtree without a remount. Component
  CSS reads the cascading variables; Rust code that needs a value uses `use_theme()`.
- `color-scheme` is a theme concern, not a mode concern: it defaults to `light` on `:root` and is
  overridden by a dark theme's inline `color-scheme`. Mode selectors (`[data-g3-mode="ios/md"]`)
  must not set `color-scheme`.
- Presets: `Theme::default_light()`, `Theme::default_dark()`, and `Theme::system()`, which pairs
  them with CSS `light-dark()`. All `Theme` fields are public, so consumers build custom themes via
  struct-update syntax, `with_accent()`, or `Theme::adaptive(light, dark)`.
- State is styled from `data-state` and ARIA attributes, not modifier classes. No `!important`;
  the `g3` layer lets any unlayered app rule win.
  (`eq_ui` ships ~26 preset themes; g3-ui has not built out an equivalent preset library yet.)

### ARIA & Accessibility
- Full WAI-ARIA roles, attributes, and keyboard navigation on all interactive components.
- Roving tabindex for composite widgets (tabs, radio groups, trees).
- Decorative elements marked `aria-hidden`.
- Live regions for dynamic content.

---

## What g3-ui Is NOT

- **NOT a clone of eq_ui**: It is a new crate tailored for Greenside Partee.
- **NOT platform-locked**: Works on web, desktop, mobile (Dioxus targets).
- **NOT JS-dependent**: Behaviour lives in Rust and CSS; scripts are small, self-contained, and only used where the DOM requires one.

---

## Quick Checklist for New Components

When adding a new component, verify:

1. [ ] Placed in `src/components/`
2. [ ] Emits only `g3-` classes defined in `assets/g3-ui.css`
3. [ ] Accepts `class: Option<String>`, merged via `merge_classes()`
4. [ ] Has markup tests in `src/tests/`
5. [ ] Documents the component and every prop
6. [ ] Has WAI-ARIA attributes for interactive components
7. [ ] Uses enums/structs for props (no magic strings)
8. [ ] Matches corresponding Ionic component behavior/design
9. [ ] Registered with `g3_playground!` and the `gallery!` list
10. [ ] Builds for wasm32 and native targets (check feature gates for timers)

---

## Local Playground

Agents may start, stop, and restart `dx serve` themselves when implementation
or visual verification requires the playground. Use `--open false`, bind to
`127.0.0.1`, and check whether port `8080` is already listening before launch.
Use `8080` when it is free; otherwise choose the next available port so multiple
project servers can run at once. Run the process in a hidden/background window,
and keep its development logs under the repository's ignored `.codex/` folder.
