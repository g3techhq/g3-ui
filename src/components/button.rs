//! Buttons.
use super::Color;
use super::pressable::Destination;
use crate::components::pressable::{ButtonType, Pressable, Target};
use crate::components::{Spinner, SpinnerSize};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;

/// How a [`Button`] is filled.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ButtonFill {
    /// Filled with its color. The primary action on a screen.
    #[default]
    Solid,
    /// A colored border and label, for a secondary action.
    Outline,
    /// Label only, for low-emphasis and toolbar actions.
    Clear,
}

/// [`Button`] size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    /// Compact, for toolbars and dense rows.
    Sm,
    /// Standard.
    #[default]
    Md,
    /// Prominent, for a main call to action.
    Lg,
}

/// How a [`Button`] fills the width of its container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonExpand {
    /// Full width, keeping its rounded corners.
    Block,
    /// Full width with square corners and no side borders, edge to edge.
    Full,
}

/// A button, or a link that looks like one. Like Ionic's `ion-button`.
///
/// Set `to` for a router destination or `href` for a plain link; otherwise it
/// renders a `<button>`. An icon-only button needs an `aria_label`.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # fn save() {}
/// # #[component] fn Settings() -> Element { rsx! {} }
/// # let saving = use_signal(|| false);
/// rsx! {
///     Button { onclick: move |_| save(), "Save" }
///     Button { fill: ButtonFill::Outline, color: Color::Danger, "Delete" }
///     Button { fill: ButtonFill::Clear, aria_label: "Settings", Settings {} }
///     Button { button_type: ButtonType::Submit, loading: saving(), "Sign in" }
/// }
/// # }
/// ```
#[component]
pub fn Button(
    /// Fill. Defaults to [`ButtonFill::Solid`].
    fill: Option<ButtonFill>,
    /// Color. Defaults to [`Color::Accent`].
    color: Option<Color>,
    /// Size. Defaults to [`ButtonSize::Md`].
    size: Option<ButtonSize>,
    /// Stretch to the container's width.
    expand: Option<ButtonExpand>,
    /// Disable the button.
    disabled: Option<bool>,
    /// Show a spinner and ignore presses, for an action in progress.
    loading: Option<bool>,
    /// The `<button>` type. Defaults to [`ButtonType::Button`], so a button in
    /// a form does not submit it unless asked to.
    button_type: Option<ButtonType>,
    /// Router destination. Renders a link.
    #[props(default, into)]
    to: Destination,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Open the link in a new tab.
    new_tab: Option<bool>,
    /// Content before the label, usually an icon.
    start: Option<Element>,
    /// Content after the label, usually an icon.
    end: Option<Element>,
    /// Accessible name. Required when the button has no visible text.
    aria_label: Option<String>,
    /// Called when pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the button.
    class: Option<String>,
    /// Any other HTML attribute, such as `aria_haspopup` or `data-*`.
    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let loading = loading.unwrap_or(false);
    let fill_cls = match fill.unwrap_or_default() {
        ButtonFill::Solid => "g3-btn-solid",
        ButtonFill::Outline => "g3-btn-outline",
        ButtonFill::Clear => "g3-btn-clear",
    };
    // Accent is the base color, so it needs no class of its own.
    let color_cls = match color.unwrap_or_default() {
        Color::Accent => String::new(),
        color => format!("g3-btn-{}", color.as_str()),
    };
    let size_cls = match size.unwrap_or_default() {
        ButtonSize::Sm => "g3-btn-sm",
        ButtonSize::Md => "g3-btn-md-size",
        ButtonSize::Lg => "g3-btn-lg",
    };
    let expand_cls = match expand {
        Some(ButtonExpand::Block) => "g3-btn-expand-block",
        Some(ButtonExpand::Full) => "g3-btn-expand-full",
        None => "",
    };
    let cls = classes([
        "g3-btn",
        mode.pick("g3-btn-ios", "g3-btn-md"),
        fill_cls,
        &color_cls,
        size_cls,
        expand_cls,
        if loading { "g3-btn-loading" } else { "" },
    ]);
    let mut attributes = attributes;
    if let Some(label) = aria_label {
        attributes.push(Attribute::new("aria-label", label, None, false));
    }
    if loading {
        attributes.push(Attribute::new("aria-busy", "true", None, false));
    }
    // Loading keeps the button focusable, so a pressed button does not drop
    // focus to the page while its work runs.
    let start = if loading {
        Some(rsx! {
            Spinner { size: SpinnerSize::Sm, decorative: true }
        })
    } else {
        start
    };
    rsx! {
        Pressable {
            class: merge_classes(cls, class.as_deref()),
            target: Target::from_props(href, to, new_tab.unwrap_or(false)),
            disabled: disabled.unwrap_or(false),
            busy: loading,
            button_type: button_type.unwrap_or_default(),
            onclick,
            attributes,
            span { class: "g3-btn-content",
                if let Some(start) = start {
                    span { class: "g3-btn-start", {start} }
                }
                span { class: "g3-btn-label", {children} }
                if let Some(end) = end {
                    span { class: "g3-btn-end", {end} }
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ButtonPlaygroundDemo() -> Element {
    let fill = use_signal(|| ButtonFill::Solid);
    let color = use_signal(|| Color::Accent);
    let size = use_signal(|| ButtonSize::Md);
    let disabled = use_signal(|| false);
    let loading = use_signal(|| false);
    let expand = use_signal(|| false);
    let label = use_signal(|| "Create".to_string());
    rsx! {
        crate::PlaygroundDemoFrame {
            controls: rsx! {
                crate::Input { label: "Label", value: label }
                crate::SegmentGroup { value: fill, aria_label: "Fill",
                    crate::SegmentButton { value: ButtonFill::Solid, "Solid" }
                    crate::SegmentButton { value: ButtonFill::Outline, "Outline" }
                    crate::SegmentButton { value: ButtonFill::Clear, "Clear" }
                }
                crate::Select {
                    label: "Color",
                    value: color,
                    options: vec![
                        crate::SelectOption::new(Color::Accent, "Accent"),
                        crate::SelectOption::new(Color::Neutral, "Neutral"),
                        crate::SelectOption::new(Color::Success, "Success"),
                        crate::SelectOption::new(Color::Warning, "Warning"),
                        crate::SelectOption::new(Color::Danger, "Danger"),
                    ],
                }
                crate::SegmentGroup { value: size, aria_label: "Size",
                    crate::SegmentButton { value: ButtonSize::Sm, "Sm" }
                    crate::SegmentButton { value: ButtonSize::Md, "Md" }
                    crate::SegmentButton { value: ButtonSize::Lg, "Lg" }
                }
                crate::Checkbox { checked: disabled, label: "Disabled" }
                crate::Checkbox { checked: loading, label: "Loading" }
                crate::Checkbox { checked: expand, label: "Expand" }
            },
            Button {
                fill: fill(),
                color: color(),
                size: size(),
                disabled: disabled(),
                loading: loading(),
                expand: expand().then_some(ButtonExpand::Block),
                "{label}"
            }
        }
    }
}

crate::g3_playground! {
    name: "Button",
    description: "Buttons and button-styled links in solid, outline, and clear fills.",
    demo: ButtonPlaygroundDemo,
    source: "src/components/button.rs",
}
