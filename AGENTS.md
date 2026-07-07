# g3_ui - Goals & Philosophies

> Reference document for AI agents working on g3_ui.
> Read this before making any changes to the crate.

---

## Identity

g3_ui is the Dioxus component library for the Greenside Partee project. It uses a flat component taxonomy and borrows its design philosophy and component behavior from **Ionic Framework**.

---

## Foundational Influences

### 1. eq_ui - Code Structure & Architecture

g3_ui adopts the following structural conventions from eq_ui:

- **Flat Component Module**: Components live together in `src/components/`; metadata groups them as Components or Utilities.
- **Co-located `_styles.rs`**: Each component file has a sibling `*_styles.rs` holding its Tailwind class constants. Shared tokens live in `theme.rs`.
- **CSS Custom Properties**: Colors and theming via CSS variables defined in theme CSS files.
- **Theme Context**: `EqTheme`-style runtime theme switching with a context provider/consumer pattern.
- **Playground Registration**: Components expose a `descriptor()` function for interactive playground demos.
- **Smoke Tests**: Each component ships with a `#[cfg(test)]` smoke test rendering the component.
- **`class` Prop**: Every component accepts a `class: String` prop for per-instance style overrides via `merge_classes()`.
- **WAI-ARIA**: Full ARIA roles/attributes on all interactive components.

### 2. Ionic Framework - Design & Behavior

g3_ui matches Ionic's component behaviors and design language:

- **Adaptive Styling**: Components have `ios` and `md` (Material Design) modes, selected once at init. The `mode` property is virtual (not reactive after init) - it sets which platform aesthetic to use and does not update at runtime. This is the clearest indicator of Ionic inspiration.
- **CSS Variables Theming**: Ionic uses CSS custom properties as its entire theming system. g3_ui inherits this approach through eq_ui's CSS variable system.
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
- Style constants in co-located `_styles.rs` files
- `theme.rs` for shared tokens + `merge_classes()` utility
- Each component gets a smoke test

### 3. Dioxus-Native

All components compile and run on Dioxus (web/WASM, desktop/Wry, mobile). Avoid `document::eval()` patterns; prefer pure Rust/Dioxus approaches. Where browser APIs are needed, use `dioxus-primitives` or feature-gated `gloo-timers` / `tokio` based on target arch.

### 4. Platform Portability

Components must work across platforms:
- **Web**: WASM via Dioxus web
- **Desktop**: Wry webview
- **Mobile**: iOS/Android native
- **Native** (future): Blitz GPU renderer

---

## Component Design Principles

### Naming
- Prefix all public components with `G3` (e.g., `G3Button`, `G3Toolbar`, `G3MobileAppShell`).

### Props
- All props are typed Rust values (no raw string magic).
- Enums for variants, sizes, positions, etc.
- `children: Element` or `Option<Element>` for slot-based composition.
- `class: String` for override (with `#[props(into, default)]`).
- **State ownership**: stateful components take an owned `Signal<T>` for their primary value
  (`checked`, `value`, `open`, `active`) and read/write it directly, rather than taking a
  `ReadSignal<T>` + required `onchange`. Pair it with an *optional* `onchange`/`on_*` callback for
  side effects the caller wants to observe. This is the convention `Field` and `Select` were
  migrated to — do not reintroduce the `ReadSignal` + required-callback pattern for new components.

### Slots
- Follow Ionic's slot naming: `start`, `end`, `primary`, `secondary` for toolbar components.
- Body content passed as `children` or explicit `Element` props.

### Theming
- CSS variables for all color/sizing tokens.
- Theme is resolved once at provider mount via context (like `mode`), not a reactive `Signal`. To
  switch themes at runtime, remount `G3ThemeProvider`/`G3AppWrapper` with a new `Theme`.
- Two built-in presets today: `Theme::default_light()` and `Theme::default_dark()`. All `Theme`
  fields are public, so consumers build custom themes via struct-update syntax or `with_focused()`.
  (`eq_ui` ships ~26 preset themes; g3_ui has not built out an equivalent preset library yet.)

### ARIA & Accessibility
- Full WAI-ARIA roles, attributes, and keyboard navigation on all interactive components.
- Roving tabindex for composite widgets (tabs, radio groups, trees).
- Decorative elements marked `aria-hidden`.
- Live regions for dynamic content.

---

## What g3_ui Is NOT

- **NOT a clone of eq_ui**: It is a new crate tailored for Greenside Partee.
- **NOT platform-locked**: Works on web, desktop, mobile (Dioxus targets).
- **NOT JS-dependent**: No `document::eval()` - pure Dioxus + CSS.

---

## Quick Checklist for New Components

When adding a new component, verify:

1. [ ] Placed in `src/components/`
2. [ ] Has co-located `_styles.rs` with Tailwind constants
3. [ ] Accepts `class: String` for override via `merge_classes()`
4. [ ] Has `#[cfg(test)]` smoke test
5. [ ] Has WAI-ARIA attributes for interactive components
6. [ ] Uses enums/structs for props (no magic strings)
7. [ ] Matches corresponding Ionic component behavior/design
8. [ ] Registered with `descriptor()` if playground support is needed
9. [ ] Works on WASM and wasm32 targets (check feature gates for timers)