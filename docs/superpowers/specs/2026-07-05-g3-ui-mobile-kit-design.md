# g3_ui Mobile Kit Design

Date: 2026-07-05

## Context

`g3_ui` is a mobile-first Dioxus component library inspired by Ionic. It already covers the Greenside Partee needs, but it is missing several primitives expected in a general mobile app component library. The approved direction is a cohesive mobile kit: ship the requested components plus a small set of simple mobile essentials, with an Ionic-style list/item family and gesture-capable swipe rows. Full virtualization/recycling is intentionally out of scope for this pass.

## Goals

- Add the requested components: Checkbox, List/Item, Refresher, Radio, Toast, and Accordion.
- Add small mobile essentials that fit the same surface: Badge, Avatar, Chip, Searchbar, Skeleton, and Progress.
- Make `G3List` / `G3Item` strong enough for the user's next project, including long press, start/end swipe actions, elastic partial swipes, and full-swipe callbacks.
- Follow the existing `g3_ui` structure: flat files in `src/components/`, co-located `*_styles.rs`, public `G3*` aliases, descriptors, playground demos, smoke/API tests, and shared CSS in `assets/g3_ui.css`.
- Keep the implementation Dioxus-native and portable across web, desktop, and mobile. No `document::eval()` patterns.

## Non-Goals

- Do not build a virtualized recycler or windowed list in this pass.
- Do not add app-specific Greenside branding or product-specific theme tokens.
- Do not introduce JavaScript gesture libraries.
- Do not rewrite existing `SettingsGroup`; it can remain as a convenience settings surface while `List`/`Item` becomes the general list primitive.

## Component Set

### Forms

- `Checkbox`: controlled checked signal, optional indeterminate state, disabled state, helper/error text, label placement, ARIA checkbox semantics, and iOS/MD visuals.
- `RadioGroup`: controlled selected value, optional allow-empty selection, disabled state, roving keyboard behavior where practical in Dioxus, and ARIA radiogroup semantics.
- `Radio`: typed value, disabled state, label placement, checked state derived from `RadioGroup`, and native-feeling iOS/MD marks.

### Lists

- `List`: container with `inset`, `lines`, `class`, `mode`, and `role="list"` defaults.
- `Item`: row primitive with slots for `start`, `end`, `overline`, `label`, `description`, `metadata`, and `children`; supports button/link/static modes, disabled state, detail chevron, selected state, lines, and ARIA roles.
- `ItemDivider`: section/header divider for grouped lists.
- `SwipeItem`: wrapper around `Item` content with optional start/end action panes, long-press callback, drag progress callback, full-swipe callback, disabled state, and CSS custom properties for swipe offset/progress.
- `SwipeAction`: declarative action content for start/end panes. Supports text, icon slot, destructive/accent variants, and callbacks.

The list family should preserve an upgrade path to virtualization later by keeping row state local and by exposing item identity through ordinary Dioxus keys rather than hidden list internals.

### Feedback

- `Toast`: controlled open signal, message/header, optional icon/start slot, actions, position (`top`, `middle`, `bottom`), duration, dismiss callback, swipe-to-dismiss where feasible, and polite live-region semantics.
- `Refresher`: pull-to-refresh surface for scroll containers. It exposes `state`, `pull_min`, `pull_max`, `pull_factor`, `on_refresh`, and `complete`/state reset wiring through controlled state rather than browser-only APIs.
- `Progress`: determinate/indeterminate progress bar with value/max, buffer value if useful, color variants through CSS variables, and ARIA progressbar semantics.
- `Skeleton`: text/block/avatar skeleton primitives plus a list-row skeleton preset for loading mobile lists.

### Disclosure

- `AccordionGroup`: single or multiple expansion, disabled/read-only state, controlled value, and keyboard focus movement for headers.
- `Accordion`: value, header slot, content, disabled/read-only state, animated open/close, and ARIA button/region wiring.

### Mobile Essentials

- `Badge`: compact count/status label with neutral/success/warning/danger/accent variants.
- `Avatar`: image/fallback initials surface sized for list rows and headers.
- `Chip`: compact selectable/removable pill with optional start/end slots and disabled state.
- `Searchbar`: mobile search input with clear button, optional cancel action, debounce-friendly `oninput`, and iOS/MD styling.

## Swipe Item Behavior

`SwipeItem` is the highest-risk part of this work and should be designed before polish:

- The row has start and end reveal panes. Start actions appear when dragging right; end actions appear when dragging left.
- Partial swipes reveal actions and settle open when the drag crosses the midpoint or has enough velocity.
- Dragging beyond the action width applies elastic resistance rather than moving linearly. The initial constant should mirror Ionic loosely: an elastic factor around `0.55`.
- Full swipe fires when the drag exceeds the pane width plus a threshold, initially around `30px`. The callback reports side, offset, and ratio.
- Long press fires after a configurable delay, initially around `500ms`, and cancels on meaningful movement.
- The component should expose enough state for callers to remove an item after a full swipe, but removal stays caller-owned.
- Visual content behind a full swipe is supplied by the action pane slots, so callers can show an image, icon, or colored action.

## API Shape

All public components use the existing `g3_ui` style:

- Public names are prefixed aliases such as `G3List`, while internal component names can remain concise (`List`).
- Props are typed Rust values, with enums for variants and placement.
- Components accept `class: Option<String>` and merge with base classes through `merge_classes()`.
- Components accept `mode: Option<ComponentMode>` where platform styling matters.
- Controlled state uses `Signal<T>` / `ReadSignal<T>` patterns already used by `Toggle`, `SegmentGroup`, and `Select`.
- Playground demos live next to each component and are registered through `crate::g3_playground!`.

Expected shared enums include:

- `ListLines`: `Full`, `Inset`, `None`
- `ItemKind`: `Static`, `Button`, `Link`
- `ItemDetail`: `Auto`, `Show`, `Hide`
- `SwipeSide`: `Start`, `End`
- `ToastPosition`: `Top`, `Middle`, `Bottom`
- `AccordionMode`: `Single`, `Multiple`
- `ControlLabelPlacement`: `Start`, `End`, `Fixed`, `Stacked`
- `StatusColor`: `Neutral`, `Accent`, `Success`, `Warning`, `Danger`

## Styling

Each component gets a co-located style file with class constants, while visual rules land in `assets/g3_ui.css`.

The CSS should:

- Use existing theme variables first: `--color-focused`, `--color-card`, `--color-card-border`, `--color-bg`, `--color-control`, `--color-text`, `--color-text-secondary`, `--color-success`, `--color-warning`, and `--color-danger`.
- Keep mode-specific styling under `[data-g3-mode="ios"]` and `[data-g3-mode="md"]` or explicit component mode classes.
- Use CSS custom properties for dynamic list/swipe behavior, especially `--g3-swipe-offset`, `--g3-swipe-progress`, and `--g3-swipe-action-width`.
- Keep mobile row heights stable. Rows should not resize on hover, selected state, swipe reveal, or loading state.

## Accessibility

- Interactive controls use native button/input semantics where practical.
- Checkbox and radio include `aria-checked`, disabled wiring, and associated label text.
- Radio group uses `role="radiogroup"` and radio items use `role="radio"`.
- Accordion headers use buttons with `aria-expanded` and `aria-controls`; content uses a labelled region.
- Toast content uses a polite live region and keeps action buttons reachable.
- List rows expose appropriate role defaults and do not hide nested interactive children behind a full-row click target when multiple controls are present.

## Testing

Use test-driven implementation for behavior and API contracts.

Minimum verification:

- Component smoke tests render every new component under `G3ThemeProvider`.
- Source/API tests verify new files are registered in `src/components/mod.rs`, `src/lib.rs`, `src/prelude.rs`, descriptors, and playground demos.
- CSS contract tests verify key class names and custom properties exist.
- Swipe math tests cover elastic resistance, full-swipe threshold, side detection, long-press cancellation threshold, and settle-open/close decisions. These should be pure helper tests rather than browser-only tests.
- Playground builds with `cargo check --manifest-path playground/Cargo.toml`.
- Library tests pass with `cargo test -p g3-ui --lib` or the current crate-equivalent command.
- Run `cargo fmt --all -- --check` and `git diff --check`.

## Implementation Order

1. Add shared list/swipe math helpers and tests.
2. Add small visual primitives: Badge, Avatar, Chip, Progress, Skeleton.
3. Add Checkbox and Radio/RadioGroup.
4. Add List, Item, ItemDivider, SwipeAction, and SwipeItem.
5. Add Toast and Refresher.
6. Add AccordionGroup and Accordion.
7. Register all public exports, descriptors, prelude exports, and playground demos.
8. Run focused tests, full library checks, and browser playground validation for the list/swipe demos.

## Open Decisions Resolved

- Scope: use the cohesive mobile kit approach.
- List behavior: include swipe/long-press gesture rows in the first pass.
- Recycler behavior: defer virtualization/recycling until a later pass.
- Extra essentials: include Badge, Avatar, Chip, Searchbar, Skeleton, and Progress.
