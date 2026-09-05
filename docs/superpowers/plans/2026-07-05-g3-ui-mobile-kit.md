# g3-ui Mobile Kit Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the approved mobile component kit to `g3-ui`: forms, list/item with swipe gestures, feedback components, disclosure components, and small mobile essentials.

**Architecture:** Keep the existing flat `src/components/` architecture. Each component family gets a focused Rust file plus a sibling `*_styles.rs`, playground demo, descriptor, smoke/API tests, public exports, prelude exports, and CSS in `assets/g3-ui.css`. The list swipe behavior is split into pure math helpers plus Dioxus event handling so the risky gesture decisions are testable without a browser.

**Tech Stack:** Rust 2024, Dioxus 0.7.9, `dioxus-icons`, `dioxus-sdk-time`, Manganis assets, CSS custom properties, existing `g3-ui` theme/mode utilities.

---

## Scope Check

The design covers several component families, but they belong to one crate release surface and share one export/playground/CSS integration path. This plan keeps them as separate task slices so each slice produces working, testable software without requiring a full recycler implementation.

## File Structure

- Create `src/components/primitives.rs` and `src/components/primitives_styles.rs` for `Badge`, `Avatar`, `Chip`, `Progress`, and `Skeleton`.
- Create `src/components/checkbox.rs` and `src/components/checkbox_styles.rs` for `Checkbox`.
- Create `src/components/radio.rs` and `src/components/radio_styles.rs` for `RadioGroup` and `Radio`.
- Create `src/components/list.rs` and `src/components/list_styles.rs` for `List`, `Item`, `ItemDivider`, `SwipeItem`, `SwipeAction`, swipe enums, and pure swipe math helpers.
- Create `src/components/searchbar.rs` and `src/components/searchbar_styles.rs` for `Searchbar`.
- Create `src/components/toast.rs` and `src/components/toast_styles.rs` for `Toast`.
- Create `src/components/refresher.rs` and `src/components/refresher_styles.rs` for `Refresher`.
- Create `src/components/accordion.rs` and `src/components/accordion_styles.rs` for `AccordionGroup` and `Accordion`.
- Modify `src/components/mod.rs` to register modules, exports, descriptors, and playground demos.
- Modify `src/lib.rs` to export public symbols, aliases, and smoke/source contract tests.
- Modify `src/prelude.rs` to re-export public symbols and aliases.
- Modify `assets/g3-ui.css` to add mode-aware styles for the new components.

## Shared Verification Commands

Run these at the end of each task unless the task gives a narrower command:

```powershell
cargo test --lib
cargo check --manifest-path playground\Cargo.toml
cargo fmt --all -- --check
git diff --check
```

Expected: all commands exit successfully. If a command fails because a prior task intentionally added a failing test, finish that task's implementation before running the shared bundle again.

### Task 1: Add Mobile Primitive Components

**Files:**
- Create: `src/components/primitives.rs`
- Create: `src/components/primitives_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing source contract tests**

Add these tests to the existing `#[cfg(test)] mod tests` in `src/lib.rs`:

```rust
#[test]
fn mobile_primitives_are_public_and_registered() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();

    assert!(crate_root.join("src/components/primitives.rs").exists());
    assert!(crate_root.join("src/components/primitives_styles.rs").exists());
    assert!(components_mod.contains("mod primitives;"));
    assert!(components_mod.contains("mod primitives_styles;"));
    assert!(components_mod.contains("primitives::DESCRIPTOR"));
    for symbol in [
        "Badge", "Avatar", "Chip", "Progress", "Skeleton", "SkeletonShape",
        "StatusColor", "G3Badge", "G3Avatar", "G3Chip", "G3Progress", "G3Skeleton",
    ] {
        assert!(public_source.contains(symbol), "{symbol} missing from lib exports");
        assert!(prelude_source.contains(symbol), "{symbol} missing from prelude");
    }
}

#[test]
fn mobile_primitive_styles_use_shared_theme_tokens() {
    let stylesheet = include_str!("../assets/g3-ui.css");
    for selector in [
        ".g3-badge", ".g3-avatar", ".g3-chip", ".g3-progress", ".g3-skeleton",
    ] {
        assert!(stylesheet.contains(selector), "{selector} style missing");
    }
    assert!(stylesheet.contains("var(--color-focused)"));
    assert!(stylesheet.contains("var(--color-success)"));
    assert!(stylesheet.contains("var(--color-warning)"));
    assert!(stylesheet.contains("var(--color-danger)"));
}
```

- [ ] **Step 2: Run the failing tests**

Run:

```powershell
cargo test --lib mobile_primitives_are_public_and_registered
cargo test --lib mobile_primitive_styles_use_shared_theme_tokens
```

Expected: both fail because `primitives.rs`, exports, descriptors, and CSS do not exist yet.

- [ ] **Step 3: Add style constants**

Create `src/components/primitives_styles.rs`:

```rust
//! Style constants for compact mobile primitives.
#![allow(dead_code)]

pub const BADGE: &str = "g3-badge";
pub const BADGE_NEUTRAL: &str = "g3-badge-neutral";
pub const BADGE_ACCENT: &str = "g3-badge-accent";
pub const BADGE_SUCCESS: &str = "g3-badge-success";
pub const BADGE_WARNING: &str = "g3-badge-warning";
pub const BADGE_DANGER: &str = "g3-badge-danger";

pub const AVATAR: &str = "g3-avatar";
pub const AVATAR_SM: &str = "g3-avatar-sm";
pub const AVATAR_MD: &str = "g3-avatar-md";
pub const AVATAR_LG: &str = "g3-avatar-lg";
pub const AVATAR_FALLBACK: &str = "g3-avatar-fallback";

pub const CHIP: &str = "g3-chip";
pub const CHIP_SELECTED: &str = "g3-chip-selected";
pub const CHIP_DISABLED: &str = "g3-chip-disabled";
pub const CHIP_START: &str = "g3-chip-start";
pub const CHIP_LABEL: &str = "g3-chip-label";
pub const CHIP_END: &str = "g3-chip-end";

pub const PROGRESS: &str = "g3-progress";
pub const PROGRESS_TRACK: &str = "g3-progress-track";
pub const PROGRESS_FILL: &str = "g3-progress-fill";
pub const PROGRESS_INDETERMINATE: &str = "g3-progress-indeterminate";

pub const SKELETON: &str = "g3-skeleton";
pub const SKELETON_TEXT: &str = "g3-skeleton-text";
pub const SKELETON_BLOCK: &str = "g3-skeleton-block";
pub const SKELETON_AVATAR: &str = "g3-skeleton-avatar";
pub const SKELETON_ROW: &str = "g3-skeleton-row";
```

- [ ] **Step 4: Add the component implementation**

Create `src/components/primitives.rs`:

```rust
//! Compact mobile primitives: badge, avatar, chip, progress, and skeleton.

use super::primitives_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum StatusColor {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

impl StatusColor {
    fn badge_class(self) -> &'static str {
        match self {
            Self::Neutral => s::BADGE_NEUTRAL,
            Self::Accent => s::BADGE_ACCENT,
            Self::Success => s::BADGE_SUCCESS,
            Self::Warning => s::BADGE_WARNING,
            Self::Danger => s::BADGE_DANGER,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize {
    Sm,
    #[default]
    Md,
    Lg,
}

impl AvatarSize {
    fn class(self) -> &'static str {
        match self {
            Self::Sm => s::AVATAR_SM,
            Self::Md => s::AVATAR_MD,
            Self::Lg => s::AVATAR_LG,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SkeletonShape {
    #[default]
    Text,
    Block,
    Avatar,
    Row,
}

impl SkeletonShape {
    fn class(self) -> &'static str {
        match self {
            Self::Text => s::SKELETON_TEXT,
            Self::Block => s::SKELETON_BLOCK,
            Self::Avatar => s::SKELETON_AVATAR,
            Self::Row => s::SKELETON_ROW,
        }
    }
}

#[component]
pub fn Badge(color: Option<StatusColor>, class: Option<String>, children: Element) -> Element {
    let color = color.unwrap_or_default();
    rsx! {
        span { class: merge_classes(format!("{} {}", s::BADGE, color.badge_class()), class.as_deref()), {children} }
    }
}

#[component]
pub fn Avatar(src: Option<String>, alt: Option<String>, fallback: Option<String>, size: Option<AvatarSize>, class: Option<String>) -> Element {
    let size = size.unwrap_or_default();
    let label = alt.clone().or_else(|| fallback.clone()).unwrap_or_else(|| "Avatar".to_string());
    rsx! {
        span { class: merge_classes(format!("{} {}", s::AVATAR, size.class()), class.as_deref()), role: "img", aria_label: label,
            if let Some(src) = src {
                img { src, alt: alt.unwrap_or_default() }
            } else {
                span { class: s::AVATAR_FALLBACK, aria_hidden: "true", "{fallback.unwrap_or_default()}" }
            }
        }
    }
}

#[component]
pub fn Chip(
    selected: Option<bool>,
    disabled: Option<bool>,
    start: Option<Element>,
    end: Option<Element>,
    class: Option<String>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Element,
) -> Element {
    let selected = selected.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let state_cls = format!(
        "{} {}",
        if selected { s::CHIP_SELECTED } else { "" },
        if disabled { s::CHIP_DISABLED } else { "" }
    );
    rsx! {
        button {
            class: merge_classes(format!("{} {state_cls}", s::CHIP), class.as_deref()),
            r#type: "button",
            disabled,
            aria_pressed: selected.to_string(),
            onclick: move |event| {
                if let Some(onclick) = onclick {
                    onclick.call(event);
                }
            },
            if let Some(start) = start { span { class: s::CHIP_START, {start} } }
            span { class: s::CHIP_LABEL, {children} }
            if let Some(end) = end { span { class: s::CHIP_END, {end} } }
        }
    }
}

#[component]
pub fn Progress(value: Option<f64>, max: Option<f64>, class: Option<String>) -> Element {
    let max = max.unwrap_or(100.0).max(1.0);
    let is_indeterminate = value.is_none();
    let value = value.unwrap_or(0.0).clamp(0.0, max);
    let percent = value / max * 100.0;
    rsx! {
        div {
            class: merge_classes(
                format!("{} {}", s::PROGRESS, if is_indeterminate { s::PROGRESS_INDETERMINATE } else { "" }),
                class.as_deref(),
            ),
            role: "progressbar",
            aria_valuemin: "0",
            aria_valuemax: max.to_string(),
            aria_valuenow: if is_indeterminate { None } else { Some(value.to_string()) },
            div { class: s::PROGRESS_TRACK,
                div { class: s::PROGRESS_FILL, style: format!("--g3-progress-value: {percent}%;") }
            }
        }
    }
}

#[component]
pub fn Skeleton(shape: Option<SkeletonShape>, class: Option<String>) -> Element {
    let shape = shape.unwrap_or_default();
    rsx! {
        span { class: merge_classes(format!("{} {}", s::SKELETON, shape.class()), class.as_deref()), aria_hidden: "true" }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn PrimitivesPlaygroundDemo() -> Element {
    let mut selected = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: selected(), onchange: move |_| selected.toggle() } span { "Chip selected" } }
            },
            div { class: "flex flex-col gap-4 w-full",
                div { class: "flex items-center gap-2",
                    Badge { color: StatusColor::Accent, "New" }
                    Badge { color: StatusColor::Success, "Ready" }
                    Badge { color: StatusColor::Danger, "3" }
                }
                div { class: "flex items-center gap-3",
                    Avatar { fallback: "MW" }
                    Chip { selected: selected(), onclick: |_| {}, "Filter" }
                }
                Progress { value: 64.0 }
                Skeleton { shape: SkeletonShape::Row }
            }
        }
    }
}

crate::g3_playground! {
    name: "Primitives",
    g3_name: "G3Badge / G3Avatar / G3Chip",
    description: "Compact mobile badges, avatars, chips, progress, and skeletons.",
    demo: PrimitivesPlaygroundDemo,
}
```

- [ ] **Step 5: Register modules and exports**

Modify `src/components/mod.rs`:

```rust
mod primitives;
mod primitives_styles;
pub use primitives::*;
```

Add `primitives::DESCRIPTOR` to `component_descriptors()` and `primitives::PLAYGROUND` to `component_playground_demos()`.

Modify `src/lib.rs` public exports:

```rust
pub use components::{
    Avatar, AvatarSize, Badge, Chip, Progress, Skeleton, SkeletonShape, StatusColor,
};
pub use components::{
    Avatar as G3Avatar, Badge as G3Badge, Chip as G3Chip, Progress as G3Progress,
    Skeleton as G3Skeleton,
};
```

Modify `src/prelude.rs` to export the same symbols and aliases.

- [ ] **Step 6: Add CSS**

Append this section to `assets/g3-ui.css`:

```css
/* -- Mobile primitives -- */

.g3-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 1.25rem;
    min-height: 1.25rem;
    padding: 0.125rem 0.45rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 700;
    line-height: 1;
    white-space: nowrap;
}
.g3-badge-neutral { background: var(--color-control); color: var(--color-text); }
.g3-badge-accent { background: var(--color-focused); color: white; }
.g3-badge-success { background: var(--color-success); color: white; }
.g3-badge-warning { background: var(--color-warning); color: #111827; }
.g3-badge-danger { background: var(--color-danger); color: white; }

.g3-avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    border-radius: 999px;
    background: var(--color-control);
    color: var(--color-text);
    font-weight: 700;
    flex-shrink: 0;
}
.g3-avatar img { width: 100%; height: 100%; object-fit: cover; }
.g3-avatar-sm { width: 28px; height: 28px; font-size: 0.7rem; }
.g3-avatar-md { width: 40px; height: 40px; font-size: 0.875rem; }
.g3-avatar-lg { width: 56px; height: 56px; font-size: 1.125rem; }

.g3-chip {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    min-height: 32px;
    padding: 0 0.75rem;
    border: 1px solid var(--color-card-border);
    border-radius: 999px;
    background: var(--color-control);
    color: var(--color-text);
    cursor: pointer;
}
.g3-chip-selected { border-color: var(--color-focused); color: var(--color-focused); background: color-mix(in srgb, var(--color-focused) 10%, transparent); }
.g3-chip-disabled { opacity: 0.45; cursor: not-allowed; }

.g3-progress-track {
    position: relative;
    width: 100%;
    height: 4px;
    overflow: hidden;
    border-radius: 999px;
    background: color-mix(in srgb, var(--color-text) 16%, transparent);
}
.g3-progress-fill {
    width: var(--g3-progress-value, 0%);
    height: 100%;
    border-radius: inherit;
    background: var(--color-focused);
    transition: width var(--transition-normal) var(--transition-md);
}
.g3-progress-indeterminate .g3-progress-fill {
    width: 38%;
    animation: g3-progress-indeterminate 1.2s ease-in-out infinite;
}
@keyframes g3-progress-indeterminate {
    0% { transform: translateX(-120%); }
    100% { transform: translateX(280%); }
}

.g3-skeleton {
    display: inline-block;
    border-radius: 0.375rem;
    background: linear-gradient(90deg, var(--color-control), color-mix(in srgb, var(--color-text) 10%, transparent), var(--color-control));
    background-size: 220% 100%;
    animation: g3-skeleton-pulse 1.4s ease-in-out infinite;
}
.g3-skeleton-text { width: 100%; height: 0.85rem; }
.g3-skeleton-block { width: 100%; height: 4rem; }
.g3-skeleton-avatar { width: 40px; height: 40px; border-radius: 999px; }
.g3-skeleton-row { width: 100%; height: 58px; border-radius: 12px; }
@keyframes g3-skeleton-pulse {
    0% { background-position: 120% 0; }
    100% { background-position: -120% 0; }
}
```

- [ ] **Step 7: Run tests and commit**

Run:

```powershell
cargo test --lib mobile_primitives_are_public_and_registered
cargo test --lib mobile_primitive_styles_use_shared_theme_tokens
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Expected: all pass. Commit:

```powershell
git add src/components/primitives.rs src/components/primitives_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add mobile primitive components"
```

### Task 2: Add Checkbox

**Files:**
- Create: `src/components/checkbox.rs`
- Create: `src/components/checkbox_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing tests**

Add to `src/lib.rs` tests:

```rust
#[test]
fn checkbox_is_public_registered_and_accessible() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let checkbox_source = std::fs::read_to_string(crate_root.join("src/components/checkbox.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/checkbox.rs").exists());
    assert!(crate_root.join("src/components/checkbox_styles.rs").exists());
    assert!(components_mod.contains("mod checkbox;"));
    assert!(components_mod.contains("checkbox::DESCRIPTOR"));
    assert!(public_source.contains("Checkbox"));
    assert!(public_source.contains("G3Checkbox"));
    assert!(public_source.contains("ControlLabelPlacement"));
    assert!(checkbox_source.contains("role: \"checkbox\""));
    assert!(checkbox_source.contains("aria_checked"));
    assert!(checkbox_source.contains("indeterminate"));
}
```

- [ ] **Step 2: Run the failing test**

Run:

```powershell
cargo test --lib checkbox_is_public_registered_and_accessible
```

Expected: fails because files and exports are missing.

- [ ] **Step 3: Add style constants**

Create `src/components/checkbox_styles.rs`:

```rust
//! Style constants for Checkbox.
#![allow(dead_code)]

pub const CHECKBOX: &str = "g3-checkbox";
pub const CHECKBOX_IOS: &str = "g3-checkbox-ios";
pub const CHECKBOX_MD: &str = "g3-checkbox-md";
pub const CONTROL: &str = "g3-checkbox-control";
pub const MARK: &str = "g3-checkbox-mark";
pub const LABEL: &str = "g3-checkbox-label";
pub const HINT: &str = "g3-checkbox-hint";
pub const ERROR: &str = "g3-checkbox-error";
pub const PLACEMENT_START: &str = "g3-control-label-start";
pub const PLACEMENT_END: &str = "g3-control-label-end";
pub const PLACEMENT_FIXED: &str = "g3-control-label-fixed";
pub const PLACEMENT_STACKED: &str = "g3-control-label-stacked";
```

- [ ] **Step 4: Add component**

Create `src/components/checkbox.rs`:

```rust
//! Checkbox component with Ionic-style label placement.

use super::checkbox_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ControlLabelPlacement {
    #[default]
    Start,
    End,
    Fixed,
    Stacked,
}

impl ControlLabelPlacement {
    pub(crate) fn class(self) -> &'static str {
        match self {
            Self::Start => s::PLACEMENT_START,
            Self::End => s::PLACEMENT_END,
            Self::Fixed => s::PLACEMENT_FIXED,
            Self::Stacked => s::PLACEMENT_STACKED,
        }
    }
}

#[component]
pub fn Checkbox(
    mut checked: Signal<bool>,
    label: Option<String>,
    indeterminate: Option<bool>,
    disabled: Option<bool>,
    helper_text: Option<String>,
    error_text: Option<String>,
    placement: Option<ControlLabelPlacement>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    on_checked_change: Option<Callback<bool>>,
) -> Element {
    let mode = use_component_mode(mode);
    let disabled = disabled.unwrap_or(false);
    let indeterminate = indeterminate.unwrap_or(false);
    let placement = placement.unwrap_or_default();
    let mode_cls = match mode {
        ComponentMode::Ios => s::CHECKBOX_IOS,
        ComponentMode::Md => s::CHECKBOX_MD,
    };
    let aria_checked = if indeterminate { "mixed".to_string() } else { checked().to_string() };

    rsx! {
        button {
            class: merge_classes(format!("{} {mode_cls} {}", s::CHECKBOX, placement.class()), class.as_deref()),
            r#type: "button",
            role: "checkbox",
            disabled,
            aria_checked,
            onclick: move |_| {
                if disabled { return; }
                let next = !checked();
                checked.set(next);
                if let Some(on_checked_change) = on_checked_change {
                    on_checked_change.call(next);
                }
            },
            span { class: s::CONTROL, "data-indeterminate": indeterminate.to_string(),
                span { class: s::MARK, aria_hidden: "true" }
            }
            span { class: s::LABEL,
                if let Some(label) = label { span { "{label}" } }
                if let Some(helper_text) = helper_text { span { class: s::HINT, "{helper_text}" } }
                if let Some(error_text) = error_text { span { class: s::ERROR, role: "alert", "{error_text}" } }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn CheckboxPlaygroundDemo() -> Element {
    let checked = use_signal(|| true);
    let mut indeterminate = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: indeterminate(), onchange: move |_| indeterminate.toggle() } span { "Indeterminate" } }
            },
            Checkbox { checked, label: "Enable notifications", indeterminate: indeterminate(), helper_text: "Shown in mobile lists" }
        }
    }
}

crate::g3_playground! {
    name: "Checkbox",
    g3_name: "G3Checkbox",
    description: "Accessible checkbox with iOS and Material styling.",
    demo: CheckboxPlaygroundDemo,
}
```

- [ ] **Step 5: Register, export, and style**

Register `checkbox` in `src/components/mod.rs`, add `checkbox::DESCRIPTOR` and `checkbox::PLAYGROUND`, export `Checkbox`, `ControlLabelPlacement`, and `G3Checkbox` in `src/lib.rs` and `src/prelude.rs`.

Append CSS:

```css
/* -- Checkbox and shared control label placement -- */

.g3-checkbox {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    min-height: 44px;
    border: 0;
    background: transparent;
    color: var(--color-text);
    text-align: left;
    cursor: pointer;
}
.g3-control-label-end { flex-direction: row-reverse; }
.g3-control-label-fixed .g3-checkbox-label { width: 9rem; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.g3-control-label-stacked { flex-direction: column; align-items: flex-start; }
.g3-checkbox-control {
    position: relative;
    width: 22px;
    height: 22px;
    flex-shrink: 0;
    border: 2px solid var(--color-card-border);
    background: var(--color-card);
}
.g3-checkbox-ios .g3-checkbox-control { border-radius: 6px; }
.g3-checkbox-md .g3-checkbox-control { border-radius: 2px; }
.g3-checkbox[aria-checked="true"] .g3-checkbox-control,
.g3-checkbox[aria-checked="mixed"] .g3-checkbox-control {
    border-color: var(--color-focused);
    background: var(--color-focused);
}
.g3-checkbox-mark {
    position: absolute;
    inset: 4px;
    border-right: 2px solid white;
    border-bottom: 2px solid white;
    transform: rotate(45deg);
    opacity: 0;
}
.g3-checkbox[aria-checked="true"] .g3-checkbox-mark { opacity: 1; }
.g3-checkbox[aria-checked="mixed"] .g3-checkbox-mark {
    inset: 9px 4px auto;
    height: 2px;
    border: 0;
    background: white;
    transform: none;
    opacity: 1;
}
.g3-checkbox-label { display: flex; flex-direction: column; gap: 0.125rem; min-width: 0; }
.g3-checkbox-hint { color: var(--color-text-secondary); font-size: 0.8125rem; }
.g3-checkbox-error { color: var(--color-danger); font-size: 0.8125rem; }
.g3-checkbox:disabled { opacity: 0.45; cursor: not-allowed; }
.g3-checkbox:focus-visible { outline: 2px solid var(--color-focused); outline-offset: 3px; }
```

- [ ] **Step 6: Run tests and commit**

Run:

```powershell
cargo test --lib checkbox_is_public_registered_and_accessible
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/checkbox.rs src/components/checkbox_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add checkbox component"
```

### Task 3: Add Radio Group and Radio

**Files:**
- Create: `src/components/radio.rs`
- Create: `src/components/radio_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing tests**

Add to `src/lib.rs` tests:

```rust
#[test]
fn radio_group_is_public_registered_and_accessible() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let radio_source = std::fs::read_to_string(crate_root.join("src/components/radio.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/radio.rs").exists());
    assert!(crate_root.join("src/components/radio_styles.rs").exists());
    assert!(components_mod.contains("mod radio;"));
    assert!(components_mod.contains("radio::DESCRIPTOR"));
    for symbol in ["RadioGroup", "Radio", "G3RadioGroup", "G3Radio"] {
        assert!(public_source.contains(symbol), "{symbol} missing from lib exports");
    }
    assert!(radio_source.contains("role: \"radiogroup\""));
    assert!(radio_source.contains("role: \"radio\""));
    assert!(radio_source.contains("aria_checked"));
    assert!(radio_source.contains("allow_empty_selection"));
}
```

- [ ] **Step 2: Run failing test**

Run:

```powershell
cargo test --lib radio_group_is_public_registered_and_accessible
```

Expected: fails because radio files and exports are missing.

- [ ] **Step 3: Add style constants**

Create `src/components/radio_styles.rs`:

```rust
//! Style constants for Radio.
#![allow(dead_code)]

pub const GROUP: &str = "g3-radio-group";
pub const RADIO: &str = "g3-radio";
pub const RADIO_IOS: &str = "g3-radio-ios";
pub const RADIO_MD: &str = "g3-radio-md";
pub const CONTROL: &str = "g3-radio-control";
pub const MARK: &str = "g3-radio-mark";
pub const LABEL: &str = "g3-radio-label";
```

- [ ] **Step 4: Add component**

Create `src/components/radio.rs`:

```rust
//! Radio group and radio components.

use super::checkbox::ControlLabelPlacement;
use super::radio_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[derive(Clone)]
struct RadioGroupContext {
    value: Signal<String>,
    disabled: bool,
    allow_empty_selection: bool,
    on_change: Option<Callback<String>>,
}

#[component]
pub fn RadioGroup(
    value: Signal<String>,
    disabled: Option<bool>,
    allow_empty_selection: Option<bool>,
    class: Option<String>,
    on_change: Option<Callback<String>>,
    children: Element,
) -> Element {
    provide_context(RadioGroupContext {
        value,
        disabled: disabled.unwrap_or(false),
        allow_empty_selection: allow_empty_selection.unwrap_or(false),
        on_change,
    });
    rsx! {
        div { class: merge_classes(s::GROUP, class.as_deref()), role: "radiogroup", {children} }
    }
}

#[component]
pub fn Radio(
    value: String,
    label: Option<String>,
    disabled: Option<bool>,
    placement: Option<ControlLabelPlacement>,
    class: Option<String>,
    mode: Option<ComponentMode>,
) -> Element {
    let mode = use_component_mode(mode);
    let context = use_context::<RadioGroupContext>();
    let selected = context.value() == value;
    let disabled = disabled.unwrap_or(false) || context.disabled;
    let placement = placement.unwrap_or_default();
    let mode_cls = match mode {
        ComponentMode::Ios => s::RADIO_IOS,
        ComponentMode::Md => s::RADIO_MD,
    };

    rsx! {
        button {
            class: merge_classes(format!("{} {mode_cls} {}", s::RADIO, placement.class()), class.as_deref()),
            r#type: "button",
            role: "radio",
            disabled,
            aria_checked: selected.to_string(),
            tabindex: if selected { "0" } else { "-1" },
            onclick: move |_| {
                if disabled { return; }
                if selected && context.allow_empty_selection {
                    context.value.set(String::new());
                    if let Some(on_change) = &context.on_change {
                        on_change.call(String::new());
                    }
                    return;
                }
                context.value.set(value.clone());
                if let Some(on_change) = &context.on_change {
                    on_change.call(value.clone());
                }
            },
            span { class: s::CONTROL, span { class: s::MARK, aria_hidden: "true" } }
            if let Some(label) = label {
                span { class: s::LABEL, "{label}" }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn RadioPlaygroundDemo() -> Element {
    let value = use_signal(|| "push".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            RadioGroup { value,
                Radio { value: "push", label: "Push notifications" }
                Radio { value: "email", label: "Email" }
                Radio { value: "none", label: "None" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Radio",
    g3_name: "G3RadioGroup / G3Radio",
    description: "Single-select radio group.",
    demo: RadioPlaygroundDemo,
}
```

- [ ] **Step 5: Register, export, and style**

Register `radio` in `src/components/mod.rs`, add `radio::DESCRIPTOR` and `radio::PLAYGROUND`, export `RadioGroup`, `Radio`, `G3RadioGroup`, and `G3Radio` in `src/lib.rs` and `src/prelude.rs`.

Append CSS:

```css
/* -- Radio -- */

.g3-radio-group { display: flex; flex-direction: column; gap: 0.25rem; }
.g3-radio {
    display: inline-flex;
    align-items: center;
    gap: 0.75rem;
    min-height: 44px;
    border: 0;
    background: transparent;
    color: var(--color-text);
    text-align: left;
    cursor: pointer;
}
.g3-radio-control {
    position: relative;
    width: 22px;
    height: 22px;
    flex-shrink: 0;
    border: 2px solid var(--color-card-border);
    border-radius: 999px;
    background: var(--color-card);
}
.g3-radio-mark {
    position: absolute;
    inset: 5px;
    border-radius: 999px;
    background: var(--color-focused);
    opacity: 0;
    transform: scale(0.4);
    transition: opacity var(--transition-fast), transform var(--transition-fast);
}
.g3-radio[aria-checked="true"] .g3-radio-control { border-color: var(--color-focused); }
.g3-radio[aria-checked="true"] .g3-radio-mark { opacity: 1; transform: scale(1); }
.g3-radio-md .g3-radio-control { width: 20px; height: 20px; }
.g3-radio-label { min-width: 0; }
.g3-radio:disabled { opacity: 0.45; cursor: not-allowed; }
.g3-radio:focus-visible { outline: 2px solid var(--color-focused); outline-offset: 3px; }
```

- [ ] **Step 6: Run tests and commit**

Run:

```powershell
cargo test --lib radio_group_is_public_registered_and_accessible
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/radio.rs src/components/radio_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add radio components"
```

### Task 4: Add List, Item, SwipeItem, and Swipe Math

**Files:**
- Create: `src/components/list.rs`
- Create: `src/components/list_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing tests for public surface and CSS contracts**

Add to `src/lib.rs` tests:

```rust
#[test]
fn list_family_is_public_registered_and_uses_swipe_contracts() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let list_source = std::fs::read_to_string(crate_root.join("src/components/list.rs")).unwrap_or_default();
    let stylesheet = include_str!("../assets/g3-ui.css");

    assert!(crate_root.join("src/components/list.rs").exists());
    assert!(crate_root.join("src/components/list_styles.rs").exists());
    assert!(components_mod.contains("mod list;"));
    assert!(components_mod.contains("list::DESCRIPTOR"));
    for symbol in [
        "List", "Item", "ItemDivider", "SwipeItem", "SwipeAction", "ListLines",
        "ItemKind", "ItemDetail", "SwipeSide", "SwipeState", "G3List", "G3Item",
        "G3SwipeItem", "G3SwipeAction",
    ] {
        assert!(public_source.contains(symbol), "{symbol} missing from lib exports");
    }
    assert!(list_source.contains("elastic_swipe_offset"));
    assert!(list_source.contains("should_full_swipe"));
    assert!(list_source.contains("LONG_PRESS_MS"));
    assert!(stylesheet.contains("--g3-swipe-offset"));
    assert!(stylesheet.contains("--g3-swipe-progress"));
    assert!(stylesheet.contains("--g3-swipe-action-width"));
}
```

- [ ] **Step 2: Write failing pure swipe math tests**

Create `src/components/list.rs` with only the tested helper definitions and tests first:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwipeSide {
    Start,
    End,
}

pub const DEFAULT_SWIPE_ACTION_WIDTH: f64 = 88.0;
pub const FULL_SWIPE_MARGIN: f64 = 30.0;
pub const ELASTIC_FACTOR: f64 = 0.55;
pub const LONG_PRESS_MS: u64 = 500;
pub const LONG_PRESS_CANCEL_DISTANCE: f64 = 8.0;

pub fn elastic_swipe_offset(raw_offset: f64, action_width: f64) -> f64 {
    let limit = action_width.max(1.0);
    if raw_offset > limit {
        limit + (raw_offset - limit) * ELASTIC_FACTOR
    } else if raw_offset < -limit {
        -limit + (raw_offset + limit) * ELASTIC_FACTOR
    } else {
        raw_offset
    }
}

pub fn should_full_swipe(offset: f64, action_width: f64) -> bool {
    offset.abs() >= action_width.max(1.0) + FULL_SWIPE_MARGIN
}

pub fn swipe_side(offset: f64) -> Option<SwipeSide> {
    if offset > 0.0 {
        Some(SwipeSide::Start)
    } else if offset < 0.0 {
        Some(SwipeSide::End)
    } else {
        None
    }
}

pub fn swipe_ratio(offset: f64, action_width: f64) -> f64 {
    offset / action_width.max(1.0)
}

pub fn should_cancel_long_press(delta_x: f64, delta_y: f64) -> bool {
    delta_x.hypot(delta_y) > LONG_PRESS_CANCEL_DISTANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elastic_swipe_offset_slows_after_action_width() {
        assert_eq!(elastic_swipe_offset(44.0, 88.0), 44.0);
        assert_eq!(elastic_swipe_offset(188.0, 88.0), 143.0);
        assert_eq!(elastic_swipe_offset(-188.0, 88.0), -143.0);
    }

    #[test]
    fn full_swipe_requires_action_width_plus_margin() {
        assert!(!should_full_swipe(117.0, 88.0));
        assert!(should_full_swipe(118.0, 88.0));
        assert!(should_full_swipe(-118.0, 88.0));
    }

    #[test]
    fn swipe_side_follows_offset_direction() {
        assert_eq!(swipe_side(12.0), Some(SwipeSide::Start));
        assert_eq!(swipe_side(-12.0), Some(SwipeSide::End));
        assert_eq!(swipe_side(0.0), None);
    }

    #[test]
    fn long_press_cancels_after_movement_threshold() {
        assert!(!should_cancel_long_press(4.0, 4.0));
        assert!(should_cancel_long_press(9.0, 0.0));
    }
}
```

- [ ] **Step 3: Run the failing tests**

Run:

```powershell
cargo test --lib list_family_is_public_registered_and_uses_swipe_contracts
cargo test --lib elastic_swipe_offset_slows_after_action_width
```

Expected: source contract fails because the full component is missing; the helper tests compile once the temporary helper-only `list.rs` exists and should pass. This gives a red test for integration and green proof for the helper formulas before UI work.

- [ ] **Step 4: Add list style constants**

Create `src/components/list_styles.rs`:

```rust
//! Style constants for List and Item components.
#![allow(dead_code)]

pub const LIST: &str = "g3-list";
pub const LIST_IOS: &str = "g3-list-ios";
pub const LIST_MD: &str = "g3-list-md";
pub const LIST_INSET: &str = "g3-list-inset";

pub const ITEM: &str = "g3-item";
pub const ITEM_IOS: &str = "g3-item-ios";
pub const ITEM_MD: &str = "g3-item-md";
pub const ITEM_BUTTON: &str = "g3-item-button";
pub const ITEM_SELECTED: &str = "g3-item-selected";
pub const ITEM_DISABLED: &str = "g3-item-disabled";
pub const ITEM_LINES_FULL: &str = "g3-item-lines-full";
pub const ITEM_LINES_INSET: &str = "g3-item-lines-inset";
pub const ITEM_LINES_NONE: &str = "g3-item-lines-none";
pub const ITEM_START: &str = "g3-item-start";
pub const ITEM_MAIN: &str = "g3-item-main";
pub const ITEM_OVERLINE: &str = "g3-item-overline";
pub const ITEM_LABEL: &str = "g3-item-label";
pub const ITEM_DESCRIPTION: &str = "g3-item-description";
pub const ITEM_METADATA: &str = "g3-item-metadata";
pub const ITEM_END: &str = "g3-item-end";
pub const ITEM_DETAIL: &str = "g3-item-detail";
pub const ITEM_DIVIDER: &str = "g3-item-divider";

pub const SWIPE_ITEM: &str = "g3-swipe-item";
pub const SWIPE_ACTIONS: &str = "g3-swipe-actions";
pub const SWIPE_ACTIONS_START: &str = "g3-swipe-actions-start";
pub const SWIPE_ACTIONS_END: &str = "g3-swipe-actions-end";
pub const SWIPE_CONTENT: &str = "g3-swipe-content";
pub const SWIPE_ACTION: &str = "g3-swipe-action";
pub const SWIPE_ACTION_DESTRUCTIVE: &str = "g3-swipe-action-destructive";
pub const SWIPE_ACTION_ACCENT: &str = "g3-swipe-action-accent";
```

- [ ] **Step 5: Replace helper-only file with full list component**

Expand `src/components/list.rs`. Keep the pure helper functions and tests from Step 2, then add:

```rust
//! General mobile list, item, and swipe row components.

use super::list_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ListLines {
    Full,
    #[default]
    Inset,
    None,
}

impl ListLines {
    fn class(self) -> &'static str {
        match self {
            Self::Full => s::ITEM_LINES_FULL,
            Self::Inset => s::ITEM_LINES_INSET,
            Self::None => s::ITEM_LINES_NONE,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ItemKind {
    Static,
    Button,
    Link(String),
}

impl Default for ItemKind {
    fn default() -> Self { Self::Static }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ItemDetail {
    #[default]
    Auto,
    Show,
    Hide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeState {
    pub side: SwipeSide,
    pub offset: f64,
    pub ratio: f64,
    pub full: bool,
}

#[component]
pub fn List(inset: Option<bool>, lines: Option<ListLines>, class: Option<String>, mode: Option<ComponentMode>, children: Element) -> Element {
    let mode = use_component_mode(mode);
    let mode_cls = match mode { ComponentMode::Ios => s::LIST_IOS, ComponentMode::Md => s::LIST_MD };
    rsx! {
        div {
            class: merge_classes(format!("{} {mode_cls} {}", s::LIST, if inset.unwrap_or(false) { s::LIST_INSET } else { "" }), class.as_deref()),
            role: "list",
            "data-lines": format!("{:?}", lines.unwrap_or_default()),
            {children}
        }
    }
}

#[component]
pub fn Item(
    kind: Option<ItemKind>,
    lines: Option<ListLines>,
    selected: Option<bool>,
    disabled: Option<bool>,
    detail: Option<ItemDetail>,
    start: Option<Element>,
    end: Option<Element>,
    overline: Option<String>,
    label: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Option<Element>,
) -> Element {
    let mode = use_component_mode(mode);
    let kind = kind.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);
    let selected = selected.unwrap_or(false);
    let detail = detail.unwrap_or_default();
    let mode_cls = match mode { ComponentMode::Ios => s::ITEM_IOS, ComponentMode::Md => s::ITEM_MD };
    let interactive = !matches!(kind, ItemKind::Static) || onclick.is_some();
    let show_detail = matches!(detail, ItemDetail::Show) || (matches!(detail, ItemDetail::Auto) && interactive && mode == ComponentMode::Ios);
    let cls = merge_classes(
        format!(
            "{} {mode_cls} {} {} {} {}",
            s::ITEM,
            lines.unwrap_or_default().class(),
            if interactive { s::ITEM_BUTTON } else { "" },
            if selected { s::ITEM_SELECTED } else { "" },
            if disabled { s::ITEM_DISABLED } else { "" },
        ),
        class.as_deref(),
    );

    let content = rsx! {
        if let Some(start) = start { span { class: s::ITEM_START, {start} } }
        span { class: s::ITEM_MAIN,
            if let Some(overline) = overline { span { class: s::ITEM_OVERLINE, "{overline}" } }
            if let Some(label) = label { span { class: s::ITEM_LABEL, "{label}" } }
            if let Some(description) = description { span { class: s::ITEM_DESCRIPTION, "{description}" } }
            if let Some(children) = children { {children} }
        }
        if let Some(metadata) = metadata { span { class: s::ITEM_METADATA, "{metadata}" } }
        if let Some(end) = end { span { class: s::ITEM_END, {end} } }
        if show_detail { ChevronRight { class: s::ITEM_DETAIL, size: 18 } }
    };

    match kind {
        ItemKind::Link(href) => rsx! { a { class: cls, href, role: "listitem", aria_disabled: disabled.to_string(), {content} } },
        ItemKind::Button => rsx! {
            button {
                class: cls,
                r#type: "button",
                role: "listitem",
                disabled,
                onclick: move |event| if let Some(onclick) = onclick { onclick.call(event); },
                {content}
            }
        },
        ItemKind::Static => rsx! { div { class: cls, role: "listitem", {content} } },
    }
}
```

Then add `ItemDivider`, `SwipeAction`, and `SwipeItem`:

```rust
#[component]
pub fn ItemDivider(class: Option<String>, children: Element) -> Element {
    rsx! { div { class: merge_classes(s::ITEM_DIVIDER, class.as_deref()), role: "separator", {children} } }
}

#[component]
pub fn SwipeAction(
    side: SwipeSide,
    destructive: Option<bool>,
    accent: Option<bool>,
    class: Option<String>,
    onclick: Option<Callback<Event<MouseData>>>,
    children: Element,
) -> Element {
    let variant = if destructive.unwrap_or(false) { s::SWIPE_ACTION_DESTRUCTIVE } else if accent.unwrap_or(false) { s::SWIPE_ACTION_ACCENT } else { "" };
    rsx! {
        button {
            class: merge_classes(format!("{} {variant}", s::SWIPE_ACTION), class.as_deref()),
            r#type: "button",
            "data-side": match side { SwipeSide::Start => "start", SwipeSide::End => "end" },
            onclick: move |event| if let Some(onclick) = onclick { onclick.call(event); },
            {children}
        }
    }
}

#[component]
pub fn SwipeItem(
    start_actions: Option<Element>,
    end_actions: Option<Element>,
    action_width: Option<f64>,
    disabled: Option<bool>,
    class: Option<String>,
    on_drag: Option<Callback<SwipeState>>,
    on_full_swipe: Option<Callback<SwipeState>>,
    on_long_press: Option<Callback<()>>,
    children: Element,
) -> Element {
    let action_width = action_width.unwrap_or(DEFAULT_SWIPE_ACTION_WIDTH);
    let disabled = disabled.unwrap_or(false);
    let mut start_x = use_signal(|| 0.0);
    let mut start_y = use_signal(|| 0.0);
    let mut offset = use_signal(|| 0.0);
    let mut dragging = use_signal(|| false);
    let mut long_press_generation = use_signal(|| 0_u64);

    let emit_state = move |next_offset: f64, full: bool| {
        if let Some(side) = swipe_side(next_offset) {
            let state = SwipeState { side, offset: next_offset, ratio: swipe_ratio(next_offset, action_width), full };
            if let Some(on_drag) = on_drag {
                on_drag.call(state);
            }
            if full {
                if let Some(on_full_swipe) = on_full_swipe {
                    on_full_swipe.call(state);
                }
            }
        }
    };

    rsx! {
        div {
            class: merge_classes(s::SWIPE_ITEM, class.as_deref()),
            style: format!(
                "--g3-swipe-offset: {}px; --g3-swipe-progress: {}; --g3-swipe-action-width: {}px;",
                offset(),
                swipe_ratio(offset(), action_width).abs().min(1.4),
                action_width,
            ),
            onpointerdown: move |event| {
                if disabled { return; }
                dragging.set(true);
                start_x.set(event.client_coordinates().x);
                start_y.set(event.client_coordinates().y);
                let generation = long_press_generation.with_mut(|value| { *value += 1; *value });
                if let Some(on_long_press) = on_long_press {
                    spawn(async move {
                        dioxus_sdk_time::sleep(Duration::from_millis(LONG_PRESS_MS)).await;
                        if long_press_generation() == generation && dragging() {
                            on_long_press.call(());
                        }
                    });
                }
            },
            onpointermove: move |event| {
                if !dragging() || disabled { return; }
                let dx = event.client_coordinates().x - start_x();
                let dy = event.client_coordinates().y - start_y();
                if should_cancel_long_press(dx, dy) {
                    long_press_generation.with_mut(|value| *value += 1);
                }
                let next = elastic_swipe_offset(dx, action_width);
                offset.set(next);
                emit_state(next, should_full_swipe(next, action_width));
            },
            onpointerup: move |_| {
                if disabled { return; }
                dragging.set(false);
                long_press_generation.with_mut(|value| *value += 1);
                let current = offset();
                if should_full_swipe(current, action_width) {
                    emit_state(current, true);
                    offset.set(0.0);
                } else if current.abs() > action_width / 2.0 {
                    offset.set(match swipe_side(current) { Some(SwipeSide::Start) => action_width, Some(SwipeSide::End) => -action_width, None => 0.0 });
                } else {
                    offset.set(0.0);
                }
            },
            onpointercancel: move |_| {
                dragging.set(false);
                long_press_generation.with_mut(|value| *value += 1);
                offset.set(0.0);
            },
            if let Some(start_actions) = start_actions {
                div { class: format!("{} {}", s::SWIPE_ACTIONS, s::SWIPE_ACTIONS_START), {start_actions} }
            }
            if let Some(end_actions) = end_actions {
                div { class: format!("{} {}", s::SWIPE_ACTIONS, s::SWIPE_ACTIONS_END), {end_actions} }
            }
            div { class: s::SWIPE_CONTENT, {children} }
        }
    }
}
```

Add a playground demo with a basic list, a link item, a button item, and one `SwipeItem`.

- [ ] **Step 6: Register, export, and style**

Register `list` in `src/components/mod.rs`, add `list::DESCRIPTOR` and `list::PLAYGROUND`, export every list symbol and alias in `src/lib.rs` and `src/prelude.rs`.

Append CSS:

```css
/* -- List and swipe item -- */

.g3-list { width: 100%; color: var(--color-text); }
.g3-list-inset { overflow: hidden; border-radius: 12px; border: 1px solid var(--color-card-border); }
.g3-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    min-height: 56px;
    padding: 0.75rem 1rem;
    border: 0;
    background: var(--color-card);
    color: var(--color-text);
    text-decoration: none;
    box-sizing: border-box;
}
.g3-item-button { cursor: pointer; text-align: left; }
.g3-item-selected { background: color-mix(in srgb, var(--color-focused) 8%, var(--color-card)); }
.g3-item-disabled { opacity: 0.45; pointer-events: none; }
.g3-item-lines-full { border-bottom: 1px solid var(--color-card-border); }
.g3-item-lines-inset::after {
    content: "";
    position: absolute;
    left: 1rem;
    right: 0;
    bottom: 0;
    height: 1px;
    background: var(--color-card-border);
}
.g3-item-lines-none { border-bottom: 0; }
.g3-item-start, .g3-item-end { display: inline-flex; align-items: center; flex-shrink: 0; }
.g3-item-main { display: flex; flex: 1; min-width: 0; flex-direction: column; gap: 0.125rem; }
.g3-item-overline { color: var(--color-text-secondary); font-size: 0.75rem; text-transform: uppercase; }
.g3-item-label { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 600; }
.g3-item-description { color: var(--color-text-secondary); font-size: 0.875rem; }
.g3-item-metadata { color: var(--color-text-secondary); font-size: 0.8125rem; white-space: nowrap; }
.g3-item-detail { color: var(--color-label-secondary); fill: none; flex-shrink: 0; }
.g3-item-divider { padding: 0.5rem 1rem; color: var(--color-text-secondary); font-size: 0.75rem; font-weight: 700; text-transform: uppercase; background: var(--color-bg-secondary); }
[data-g3-mode="ios"] .g3-list-inset { border-radius: 12px; }
[data-g3-mode="md"] .g3-list-inset { border-radius: 4px; }

.g3-swipe-item {
    position: relative;
    overflow: hidden;
    width: 100%;
    touch-action: pan-y;
    background: var(--color-bg-secondary);
}
.g3-swipe-actions {
    position: absolute;
    inset-block: 0;
    display: flex;
    width: var(--g3-swipe-action-width, 88px);
}
.g3-swipe-actions-start { left: 0; justify-content: flex-start; }
.g3-swipe-actions-end { right: 0; justify-content: flex-end; }
.g3-swipe-content {
    position: relative;
    z-index: 1;
    transform: translate3d(var(--g3-swipe-offset, 0px), 0, 0);
    transition: transform var(--transition-normal) var(--transition-md);
}
.g3-swipe-action {
    min-width: var(--g3-swipe-action-width, 88px);
    border: 0;
    background: var(--color-control);
    color: var(--color-text);
    font-weight: 700;
}
.g3-swipe-action-accent { background: var(--color-focused); color: white; }
.g3-swipe-action-destructive { background: var(--color-danger); color: white; }
```

- [ ] **Step 7: Run tests and commit**

Run:

```powershell
cargo test --lib list_family_is_public_registered_and_uses_swipe_contracts
cargo test --lib elastic_swipe_offset_slows_after_action_width
cargo test --lib full_swipe_requires_action_width_plus_margin
cargo test --lib swipe_side_follows_offset_direction
cargo test --lib long_press_cancels_after_movement_threshold
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/list.rs src/components/list_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add list and swipe item components"
```

### Task 5: Add Searchbar

**Files:**
- Create: `src/components/searchbar.rs`
- Create: `src/components/searchbar_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing test**

Add:

```rust
#[test]
fn searchbar_is_public_registered_and_has_clear_action() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let source = std::fs::read_to_string(crate_root.join("src/components/searchbar.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/searchbar.rs").exists());
    assert!(components_mod.contains("mod searchbar;"));
    assert!(components_mod.contains("searchbar::DESCRIPTOR"));
    assert!(public_source.contains("Searchbar"));
    assert!(public_source.contains("G3Searchbar"));
    assert!(source.contains("aria_label: \"Clear search\""));
    assert!(source.contains("on_clear"));
}
```

- [ ] **Step 2: Run failing test**

Run:

```powershell
cargo test --lib searchbar_is_public_registered_and_has_clear_action
```

Expected: fails because searchbar files and exports are missing.

- [ ] **Step 3: Add constants and component**

Create `src/components/searchbar_styles.rs`:

```rust
//! Style constants for Searchbar.
#![allow(dead_code)]

pub const SEARCHBAR: &str = "g3-searchbar";
pub const SEARCHBAR_IOS: &str = "g3-searchbar-ios";
pub const SEARCHBAR_MD: &str = "g3-searchbar-md";
pub const INPUT: &str = "g3-searchbar-input";
pub const CLEAR: &str = "g3-searchbar-clear";
pub const CANCEL: &str = "g3-searchbar-cancel";
```

Create `src/components/searchbar.rs`:

```rust
//! Mobile searchbar component.

use super::searchbar_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;

#[component]
pub fn Searchbar(
    value: Signal<String>,
    placeholder: Option<String>,
    disabled: Option<bool>,
    show_cancel: Option<bool>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    oninput: Option<EventHandler<Event<FormData>>>,
    on_change: Option<Callback<String>>,
    on_clear: Option<Callback<()>>,
    on_cancel: Option<Callback<()>>,
) -> Element {
    let mode = use_component_mode(mode);
    let disabled = disabled.unwrap_or(false);
    let mode_cls = match mode { ComponentMode::Ios => s::SEARCHBAR_IOS, ComponentMode::Md => s::SEARCHBAR_MD };
    rsx! {
        div { class: merge_classes(format!("{} {mode_cls}", s::SEARCHBAR), class.as_deref()),
            input {
                class: s::INPUT,
                r#type: "search",
                value: "{value()}",
                placeholder: placeholder.unwrap_or_else(|| "Search".to_string()),
                disabled,
                oninput: move |event: Event<FormData>| {
                    value.set(event.value());
                    if let Some(oninput) = oninput {
                        oninput.call(event.clone());
                    }
                    if let Some(on_change) = on_change {
                        on_change.call(event.value());
                    }
                },
            }
            if !value().is_empty() {
                button {
                    class: s::CLEAR,
                    r#type: "button",
                    aria_label: "Clear search",
                    onclick: move |_| {
                        value.set(String::new());
                        if let Some(on_clear) = on_clear {
                            on_clear.call(());
                        }
                    },
                    "x"
                }
            }
            if show_cancel.unwrap_or(false) {
                button {
                    class: s::CANCEL,
                    r#type: "button",
                    onclick: move |_| if let Some(on_cancel) = on_cancel { on_cancel.call(()); },
                    "Cancel"
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn SearchbarPlaygroundDemo() -> Element {
    let value = use_signal(String::new);
    rsx! { crate::PlaygroundDemoFrame { Searchbar { value, show_cancel: true } } }
}

crate::g3_playground! {
    name: "Searchbar",
    g3_name: "G3Searchbar",
    description: "Mobile search input with clear and cancel affordances.",
    demo: SearchbarPlaygroundDemo,
}
```

- [ ] **Step 4: Register, export, style, test, commit**

Register and export `Searchbar`/`G3Searchbar`.

Append CSS:

```css
/* -- Searchbar -- */

.g3-searchbar { display: flex; align-items: center; gap: 0.5rem; width: 100%; }
.g3-searchbar-input {
    min-width: 0;
    flex: 1;
    min-height: 40px;
    border: 1px solid var(--color-card-border);
    border-radius: 10px;
    background: var(--color-control);
    color: var(--color-text);
    padding: 0 2.25rem 0 0.875rem;
}
.g3-searchbar-clear {
    margin-left: -2rem;
    width: 1.5rem;
    height: 1.5rem;
    border: 0;
    border-radius: 999px;
    background: var(--color-label-secondary);
    color: white;
}
.g3-searchbar-cancel { border: 0; background: transparent; color: var(--color-focused); font-weight: 600; }
.g3-searchbar-md .g3-searchbar-input { border-radius: 4px; }
```

Run:

```powershell
cargo test --lib searchbar_is_public_registered_and_has_clear_action
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/searchbar.rs src/components/searchbar_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add searchbar component"
```

### Task 6: Add Toast

**Files:**
- Create: `src/components/toast.rs`
- Create: `src/components/toast_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing test**

Add:

```rust
#[test]
fn toast_is_public_registered_and_uses_live_region() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let source = std::fs::read_to_string(crate_root.join("src/components/toast.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/toast.rs").exists());
    assert!(crate_root.join("src/components/toast_styles.rs").exists());
    assert!(components_mod.contains("mod toast;"));
    assert!(components_mod.contains("toast::DESCRIPTOR"));
    assert!(public_source.contains("Toast"));
    assert!(public_source.contains("G3Toast"));
    assert!(public_source.contains("ToastPosition"));
    assert!(source.contains("role: \"status\""));
    assert!(source.contains("aria_live: \"polite\""));
}
```

- [ ] **Step 2: Run failing test**

Run:

```powershell
cargo test --lib toast_is_public_registered_and_uses_live_region
```

Expected: fails because toast files and exports are missing.

- [ ] **Step 3: Add component**

Create `src/components/toast_styles.rs`:

```rust
//! Style constants for Toast.
#![allow(dead_code)]

pub const TOAST: &str = "g3-toast";
pub const TOAST_IOS: &str = "g3-toast-ios";
pub const TOAST_MD: &str = "g3-toast-md";
pub const TOAST_OPEN: &str = "g3-toast-open";
pub const TOAST_CLOSED: &str = "g3-toast-closed";
pub const POSITION_TOP: &str = "g3-toast-top";
pub const POSITION_MIDDLE: &str = "g3-toast-middle";
pub const POSITION_BOTTOM: &str = "g3-toast-bottom";
pub const CONTENT: &str = "g3-toast-content";
pub const HEADER: &str = "g3-toast-header";
pub const MESSAGE: &str = "g3-toast-message";
pub const ACTIONS: &str = "g3-toast-actions";
```

Create `src/components/toast.rs`:

```rust
//! Toast overlay component.

use super::toast_styles as s;
use crate::theme::{ComponentMode, merge_classes, use_component_mode};
use dioxus::prelude::*;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastPosition {
    Top,
    Middle,
    #[default]
    Bottom,
}

impl ToastPosition {
    fn class(self) -> &'static str {
        match self {
            Self::Top => s::POSITION_TOP,
            Self::Middle => s::POSITION_MIDDLE,
            Self::Bottom => s::POSITION_BOTTOM,
        }
    }
}

#[component]
pub fn Toast(
    open: Signal<bool>,
    header: Option<String>,
    message: String,
    position: Option<ToastPosition>,
    duration_ms: Option<u64>,
    action: Option<Element>,
    class: Option<String>,
    mode: Option<ComponentMode>,
    on_dismiss: Option<Callback<()>>,
) -> Element {
    let mode = use_component_mode(mode);
    let position = position.unwrap_or_default();
    let mode_cls = match mode { ComponentMode::Ios => s::TOAST_IOS, ComponentMode::Md => s::TOAST_MD };
    use_effect(move || {
        if open() {
            if let Some(duration_ms) = duration_ms {
                spawn(async move {
                    dioxus_sdk_time::sleep(Duration::from_millis(duration_ms)).await;
                    open.set(false);
                    if let Some(on_dismiss) = on_dismiss {
                        on_dismiss.call(());
                    }
                });
            }
        }
    });
    rsx! {
        div {
            class: merge_classes(format!("{} {mode_cls} {} {}", s::TOAST, position.class(), if open() { s::TOAST_OPEN } else { s::TOAST_CLOSED }), class.as_deref()),
            role: "status",
            aria_live: "polite",
            aria_hidden: (!open()).to_string(),
            div { class: s::CONTENT,
                if let Some(header) = header { div { class: s::HEADER, "{header}" } }
                div { class: s::MESSAGE, "{message}" }
            }
            if let Some(action) = action { div { class: s::ACTIONS, {action} } }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn ToastPlaygroundDemo() -> Element {
    let open = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Button { onclick: move |_| open.set(true), "Show toast" }
            Toast { open, message: "Saved", duration_ms: 2500 }
        }
    }
}

crate::g3_playground! {
    name: "Toast",
    g3_name: "G3Toast",
    description: "Transient status message.",
    demo: ToastPlaygroundDemo,
}
```

- [ ] **Step 4: Register, export, style, test, commit**

Register and export `Toast`, `ToastPosition`, `G3Toast`, and `G3ToastPosition`.

Append CSS:

```css
/* -- Toast -- */

.g3-toast {
    position: fixed;
    z-index: 120;
    left: 1rem;
    right: 1rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.875rem 1rem;
    border-radius: 10px;
    background: #1f2937;
    color: white;
    box-shadow: 0 8px 24px rgba(0,0,0,0.22);
    transition: opacity var(--transition-normal) var(--transition-md), transform var(--transition-normal) var(--transition-md);
}
.g3-toast-top { top: calc(1rem + env(safe-area-inset-top)); }
.g3-toast-middle { top: 50%; transform: translateY(-50%); }
.g3-toast-bottom { bottom: calc(1rem + env(safe-area-inset-bottom)); }
.g3-toast-closed { opacity: 0; pointer-events: none; transform: translateY(0.75rem); }
.g3-toast-open { opacity: 1; pointer-events: auto; }
.g3-toast-header { font-weight: 700; margin-bottom: 0.125rem; }
.g3-toast-message { font-size: 0.875rem; line-height: 1.35; }
.g3-toast-actions { flex-shrink: 0; }
.g3-toast-ios { border-radius: 14px; backdrop-filter: blur(18px); }
.g3-toast-md { border-radius: 4px; }
```

Run:

```powershell
cargo test --lib toast_is_public_registered_and_uses_live_region
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/toast.rs src/components/toast_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add toast component"
```

### Task 7: Add Refresher

**Files:**
- Create: `src/components/refresher.rs`
- Create: `src/components/refresher_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing tests**

Add:

```rust
#[test]
fn refresher_is_public_registered_and_state_driven() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let source = std::fs::read_to_string(crate_root.join("src/components/refresher.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/refresher.rs").exists());
    assert!(crate_root.join("src/components/refresher_styles.rs").exists());
    assert!(components_mod.contains("mod refresher;"));
    assert!(components_mod.contains("refresher::DESCRIPTOR"));
    assert!(public_source.contains("Refresher"));
    assert!(public_source.contains("RefresherState"));
    assert!(public_source.contains("G3Refresher"));
    assert!(source.contains("pull_min"));
    assert!(source.contains("pull_max"));
    assert!(source.contains("on_refresh"));
}
```

- [ ] **Step 2: Run failing test**

Run:

```powershell
cargo test --lib refresher_is_public_registered_and_state_driven
```

Expected: fails because refresher files and exports are missing.

- [ ] **Step 3: Add component**

Create `src/components/refresher_styles.rs`:

```rust
//! Style constants for Refresher.
#![allow(dead_code)]

pub const REFRESHER: &str = "g3-refresher";
pub const CONTENT: &str = "g3-refresher-content";
pub const PULLING: &str = "g3-refresher-pulling";
pub const READY: &str = "g3-refresher-ready";
pub const REFRESHING: &str = "g3-refresher-refreshing";
```

Create `src/components/refresher.rs`:

```rust
//! Pull-to-refresh surface.

use super::refresher_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum RefresherState {
    #[default]
    Inactive,
    Pulling,
    Ready,
    Refreshing,
    Completing,
    Cancelling,
}

impl RefresherState {
    fn class(self) -> &'static str {
        match self {
            Self::Pulling => s::PULLING,
            Self::Ready => s::READY,
            Self::Refreshing => s::REFRESHING,
            _ => "",
        }
    }
}

#[component]
pub fn Refresher(
    state: Signal<RefresherState>,
    pull_min: Option<f64>,
    pull_max: Option<f64>,
    pull_factor: Option<f64>,
    disabled: Option<bool>,
    class: Option<String>,
    on_refresh: Callback<()>,
    children: Option<Element>,
) -> Element {
    let mut start_y = use_signal(|| 0.0);
    let pull_min = pull_min.unwrap_or(60.0);
    let pull_max = pull_max.unwrap_or(120.0);
    let pull_factor = pull_factor.unwrap_or(1.0).max(0.0);
    let disabled = disabled.unwrap_or(false);
    let current_state = state();
    rsx! {
        div {
            class: merge_classes(format!("{} {}", s::REFRESHER, current_state.class()), class.as_deref()),
            "data-state": format!("{:?}", current_state),
            onpointerdown: move |event| if !disabled { start_y.set(event.client_coordinates().y); },
            onpointermove: move |event| {
                if disabled || matches!(state(), RefresherState::Refreshing) { return; }
                let delta = ((event.client_coordinates().y - start_y()) * pull_factor).max(0.0);
                if delta >= pull_max {
                    state.set(RefresherState::Refreshing);
                    on_refresh.call(());
                } else if delta >= pull_min {
                    state.set(RefresherState::Ready);
                } else if delta > 0.0 {
                    state.set(RefresherState::Pulling);
                }
            },
            onpointerup: move |_| {
                if disabled { return; }
                match state() {
                    RefresherState::Ready => {
                        state.set(RefresherState::Refreshing);
                        on_refresh.call(());
                    }
                    RefresherState::Pulling => state.set(RefresherState::Cancelling),
                    _ => {}
                }
            },
            div { class: s::CONTENT,
                if let Some(children) = children { {children} } else { crate::Spinner {} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn RefresherPlaygroundDemo() -> Element {
    let state = use_signal(RefresherState::default);
    rsx! {
        crate::PlaygroundDemoFrame {
            Refresher { state, on_refresh: move |_| state.set(RefresherState::Completing) }
            crate::List {
                crate::Item { label: "Pull down in a scroll surface", description: "State-driven refresher" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Refresher",
    g3_name: "G3Refresher",
    description: "State-driven pull-to-refresh surface.",
    demo: RefresherPlaygroundDemo,
}
```

- [ ] **Step 4: Register, export, style, test, commit**

Register and export `Refresher`, `RefresherState`, `G3Refresher`, and `G3RefresherState`.

Append CSS:

```css
/* -- Refresher -- */

.g3-refresher {
    display: flex;
    justify-content: center;
    min-height: 48px;
    color: var(--color-focused);
    transition: transform var(--transition-normal) var(--transition-md), opacity var(--transition-normal) var(--transition-md);
}
.g3-refresher-content { display: flex; align-items: center; justify-content: center; min-height: 48px; }
.g3-refresher-pulling { opacity: 0.7; }
.g3-refresher-ready { opacity: 1; }
.g3-refresher-refreshing { opacity: 1; }
```

Run:

```powershell
cargo test --lib refresher_is_public_registered_and_state_driven
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/refresher.rs src/components/refresher_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add refresher component"
```

### Task 8: Add Accordion

**Files:**
- Create: `src/components/accordion.rs`
- Create: `src/components/accordion_styles.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `assets/g3-ui.css`

- [ ] **Step 1: Write failing test**

Add:

```rust
#[test]
fn accordion_is_public_registered_and_aria_wired() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let components_mod = std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
    let public_source = include_str!("lib.rs")
        .split("#[cfg(test)]")
        .next()
        .expect("library source should have a public section");
    let source = std::fs::read_to_string(crate_root.join("src/components/accordion.rs")).unwrap_or_default();

    assert!(crate_root.join("src/components/accordion.rs").exists());
    assert!(crate_root.join("src/components/accordion_styles.rs").exists());
    assert!(components_mod.contains("mod accordion;"));
    assert!(components_mod.contains("accordion::DESCRIPTOR"));
    for symbol in ["AccordionGroup", "Accordion", "AccordionMode", "G3AccordionGroup", "G3Accordion"] {
        assert!(public_source.contains(symbol), "{symbol} missing from lib exports");
    }
    assert!(source.contains("aria_expanded"));
    assert!(source.contains("aria_controls"));
    assert!(source.contains("role: \"region\""));
}
```

- [ ] **Step 2: Run failing test**

Run:

```powershell
cargo test --lib accordion_is_public_registered_and_aria_wired
```

Expected: fails because accordion files and exports are missing.

- [ ] **Step 3: Add component**

Create `src/components/accordion_styles.rs`:

```rust
//! Style constants for Accordion.
#![allow(dead_code)]

pub const GROUP: &str = "g3-accordion-group";
pub const GROUP_INSET: &str = "g3-accordion-group-inset";
pub const ACCORDION: &str = "g3-accordion";
pub const HEADER: &str = "g3-accordion-header";
pub const CONTENT: &str = "g3-accordion-content";
pub const CONTENT_OPEN: &str = "g3-accordion-content-open";
pub const CONTENT_CLOSED: &str = "g3-accordion-content-closed";
```

Create `src/components/accordion.rs`:

```rust
//! Accordion disclosure components.

use super::accordion_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AccordionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone)]
struct AccordionGroupContext {
    value: Signal<Vec<String>>,
    mode: AccordionMode,
    disabled: bool,
    readonly: bool,
}

#[component]
pub fn AccordionGroup(
    value: Signal<Vec<String>>,
    mode: Option<AccordionMode>,
    disabled: Option<bool>,
    readonly: Option<bool>,
    inset: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    provide_context(AccordionGroupContext {
        value,
        mode: mode.unwrap_or_default(),
        disabled: disabled.unwrap_or(false),
        readonly: readonly.unwrap_or(false),
    });
    rsx! {
        div { class: merge_classes(format!("{} {}", s::GROUP, if inset.unwrap_or(false) { s::GROUP_INSET } else { "" }), class.as_deref()), {children} }
    }
}

#[component]
pub fn Accordion(
    value: String,
    header: Element,
    disabled: Option<bool>,
    readonly: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let context = use_context::<AccordionGroupContext>();
    let is_open = context.value().contains(&value);
    let disabled = disabled.unwrap_or(false) || context.disabled;
    let readonly = readonly.unwrap_or(false) || context.readonly;
    let content_id = format!("g3-accordion-content-{value}");
    rsx! {
        div { class: merge_classes(s::ACCORDION, class.as_deref()),
            button {
                class: s::HEADER,
                r#type: "button",
                disabled,
                aria_expanded: is_open.to_string(),
                aria_controls: content_id.clone(),
                onclick: move |_| {
                    if disabled || readonly { return; }
                    let mut next = context.value();
                    if is_open {
                        next.retain(|item| item != &value);
                    } else if context.mode == AccordionMode::Multiple {
                        next.push(value.clone());
                    } else {
                        next = vec![value.clone()];
                    }
                    context.value.set(next);
                },
                {header}
            }
            div {
                id: content_id,
                class: format!("{} {}", s::CONTENT, if is_open { s::CONTENT_OPEN } else { s::CONTENT_CLOSED }),
                role: "region",
                hidden: !is_open,
                {children}
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn AccordionPlaygroundDemo() -> Element {
    let value = use_signal(|| vec!["one".to_string()]);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            AccordionGroup { value, inset: true,
                Accordion { value: "one", header: rsx! { span { "Details" } }, "Accordion content" }
                Accordion { value: "two", header: rsx! { span { "More" } }, "More content" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Accordion",
    g3_name: "G3AccordionGroup / G3Accordion",
    description: "Expandable disclosure sections.",
    demo: AccordionPlaygroundDemo,
}
```

- [ ] **Step 4: Register, export, style, test, commit**

Register and export `AccordionGroup`, `Accordion`, `AccordionMode`, `G3AccordionGroup`, `G3Accordion`, and `G3AccordionMode`.

Append CSS:

```css
/* -- Accordion -- */

.g3-accordion-group { width: 100%; }
.g3-accordion-group-inset { overflow: hidden; border: 1px solid var(--color-card-border); border-radius: 12px; }
.g3-accordion { border-bottom: 1px solid var(--color-card-border); background: var(--color-card); }
.g3-accordion-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    min-height: 48px;
    padding: 0.75rem 1rem;
    border: 0;
    background: transparent;
    color: var(--color-text);
    font-weight: 700;
    text-align: left;
}
.g3-accordion-header:focus-visible { outline: 2px solid var(--color-focused); outline-offset: -2px; }
.g3-accordion-content { padding: 0 1rem 1rem; color: var(--color-text-secondary); }
.g3-accordion-content-closed { display: none; }
.g3-accordion-content-open { display: block; }
```

Run:

```powershell
cargo test --lib accordion_is_public_registered_and_aria_wired
cargo test --lib
cargo fmt --all -- --check
git diff --check
```

Commit:

```powershell
git add src/components/accordion.rs src/components/accordion_styles.rs src/components/mod.rs src/lib.rs src/prelude.rs assets/g3-ui.css
git commit -m "Add accordion components"
```

### Task 9: Final Integration and Playground QA

**Files:**
- Modify: `src/lib.rs`
- Modify: `src/prelude.rs`
- Modify: `src/components/mod.rs`
- Modify: `assets/g3-ui.css`
- Read: `playground/src/main.rs`

- [ ] **Step 1: Add full surface smoke test**

Extend `CompositeSmokeApp` in `src/lib.rs` with a small instance of every new component:

```rust
#[component]
fn NewMobileKitSmokeApp() -> Element {
    let checked = use_signal(|| true);
    let radio = use_signal(|| "one".to_string());
    let search = use_signal(String::new);
    let toast_open = use_signal(|| true);
    let refresher_state = use_signal(RefresherState::default);
    let accordion = use_signal(|| vec!["first".to_string()]);

    rsx! {
        G3ThemeProvider {
            G3Badge { "New" }
            G3Avatar { fallback: "MW" }
            G3Chip { onclick: |_| {}, "Chip" }
            G3Progress { value: 50.0 }
            G3Skeleton { shape: SkeletonShape::Row }
            G3Checkbox { checked, label: "Check" }
            G3RadioGroup { value: radio,
                G3Radio { value: "one", label: "One" }
            }
            G3Searchbar { value: search }
            G3List {
                G3Item { label: "Item", description: "Description" }
                G3SwipeItem {
                    start_actions: rsx! { G3SwipeAction { side: SwipeSide::Start, accent: true, "Pin" } },
                    end_actions: rsx! { G3SwipeAction { side: SwipeSide::End, destructive: true, "Delete" } },
                    G3Item { label: "Swipe", description: "Drag row" }
                }
            }
            G3Toast { open: toast_open, message: "Saved" }
            G3Refresher { state: refresher_state, on_refresh: |_| {} }
            G3AccordionGroup { value: accordion,
                G3Accordion { value: "first", header: rsx! { span { "First" } }, "Content" }
            }
        }
    }
}

#[test]
fn new_mobile_kit_renders() {
    render(NewMobileKitSmokeApp);
}
```

- [ ] **Step 2: Add descriptor coverage assertions**

Extend `descriptors_cover_public_surface`:

```rust
for g3_name in [
    "G3Badge / G3Avatar / G3Chip",
    "G3Checkbox",
    "G3RadioGroup / G3Radio",
    "G3List / G3Item",
    "G3Searchbar",
    "G3Toast",
    "G3Refresher",
    "G3AccordionGroup / G3Accordion",
] {
    assert!(
        component_descriptors()
            .iter()
            .any(|descriptor| descriptor.g3_name == g3_name),
        "{g3_name} descriptor missing"
    );
}
```

- [ ] **Step 3: Run full verification**

Run:

```powershell
cargo test --lib
cargo check --manifest-path playground\Cargo.toml
cargo fmt --all -- --check
git diff --check
```

Expected: all pass.

- [ ] **Step 4: Start playground for browser validation**

Run:

```powershell
dx serve --open false --port 8082 --addr 127.0.0.1
```

If `dx` is not on PATH, run:

```powershell
C:\Users\Matthew\.cargo\bin\dx.exe serve --open false --port 8082 --addr 127.0.0.1
```

Expected: playground serves at `http://127.0.0.1:8082/`.

- [ ] **Step 5: Browser-verify mobile demos**

Use the Browser plugin at a compact viewport such as `390x844`. Verify:

- The playground loads.
- The component menu includes the new demos.
- `G3List / G3Item` demo shows stable row heights and swipe action panes.
- `G3Checkbox` and `G3RadioGroup / G3Radio` demos show iOS and MD modes without overlap.
- `G3Toast` appears above the app body and dismisses when duration is configured.
- `G3AccordionGroup / G3Accordion` expands and collapses without layout overlap.

- [ ] **Step 6: Commit integration polish when Step 5 changed files**

If browser QA in Step 5 required edits, run the shared verification commands again, then commit the exact files changed by that QA pass:

```powershell
git add src/lib.rs src/prelude.rs src/components/mod.rs assets/g3-ui.css
git commit -m "Verify mobile kit integration"
```

If Step 5 required no changes, do not create an empty commit.

## Completion Criteria

- Every component in the design spec has a public non-prefixed name and `G3*` alias.
- Every component family has a descriptor and playground demo.
- `cargo test --lib` passes.
- `cargo check --manifest-path playground\Cargo.toml` passes.
- `cargo fmt --all -- --check` passes.
- `git diff --check` passes.
- Browser playground QA covers the new list/swipe surface and at least one iOS/MD mode switch.
