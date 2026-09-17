//! Lists and list rows.
use crate::components::pressable::{Pressable, Target};
use crate::theme::{ComponentMode, classes, merge_classes, use_component_mode};
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;

/// How separators are drawn between list rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ListLines {
    /// Separators span the full width.
    Full,
    /// Separators start at the text, clear of leading icons.
    #[default]
    Inset,
    /// No separators.
    None,
}

impl ListLines {
    fn as_str(self) -> &'static str {
        match self {
            ListLines::Full => "full",
            ListLines::Inset => "inset",
            ListLines::None => "none",
        }
    }
}

/// How a [`List`] sits on the page.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ListVariant {
    /// Rows run edge to edge on the page background.
    #[default]
    Plain,
    /// Rows sit in a rounded group with margins, like iOS settings screens.
    Grouped,
}

/// Whether an [`Item`] shows a trailing chevron.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ItemDetail {
    /// Show it for tappable rows on iOS, as iOS does.
    #[default]
    Auto,
    /// Always show it.
    Show,
    /// Never show it.
    Hide,
}

/// A vertical list of [`Item`]s. Like Ionic's `ion-list`.
///
/// ```rust,ignore
/// rsx! {
///     List { variant: ListVariant::Grouped,
///         ListHeader { "Account" }
///         Item { label: "Profile", to: Route::Profile {} }
///         Item { label: "Notifications", end: rsx! { Toggle { checked: notify, aria_label: "Notifications" } } }
///     }
/// }
/// ```
#[component]
pub fn List(
    /// Plain or grouped. Defaults to [`ListVariant::Plain`].
    variant: Option<ListVariant>,
    /// Separators between rows. Defaults to [`ListLines::Inset`].
    lines: Option<ListLines>,
    /// Accessible name of the list.
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the list.
    class: Option<String>,
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let cls = classes([
        "g3-list",
        mode.pick("g3-list-ios", "g3-list-md"),
        match variant.unwrap_or_default() {
            ListVariant::Plain => "",
            ListVariant::Grouped => "g3-list-grouped",
        },
    ]);
    rsx! {
        div {
            class: merge_classes(cls, class.as_deref()),
            role: "list",
            aria_label,
            "data-lines": lines.unwrap_or_default().as_str(),
            {children}
        }
    }
}

/// A section heading inside a [`List`].
#[component]
pub fn ListHeader(
    /// Extra classes for the header row.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: merge_classes("g3-list-header-row", class.as_deref()),
            role: "listitem",
            h3 { class: "g3-list-header", {children} }
        }
    }
}

/// A list row with leading content, text lines, metadata, and trailing
/// content. Like Ionic's `ion-item`.
///
/// It is tappable when given `onclick`, `to`, or `href`: a button, a router
/// link, or a plain link. With `checked` it is a checkbox row and `onclick`
/// should flip the value.
#[component]
pub fn Item(
    /// Main text.
    label: Option<String>,
    /// Small text above the label.
    overline: Option<String>,
    /// Secondary text under the label.
    description: Option<String>,
    /// Short trailing text, such as a value or date.
    metadata: Option<String>,
    /// Leading content: an icon, avatar, or thumbnail.
    start: Option<Element>,
    /// Trailing content: a badge, toggle, or button.
    end: Option<Element>,
    /// Called when the row is pressed.
    onclick: Option<EventHandler<MouseEvent>>,
    /// Router destination.
    #[props(into)]
    to: Option<NavigationTarget>,
    /// Plain link destination, used when `to` is not set.
    href: Option<String>,
    /// Open the link in a new tab.
    new_tab: Option<bool>,
    /// Make the row a checkbox with this state, drawn in place of `end`.
    checked: Option<bool>,
    /// Highlight the row as the current one.
    selected: Option<bool>,
    /// Disable the row.
    disabled: Option<bool>,
    /// Trailing chevron. Defaults to [`ItemDetail::Auto`].
    detail: Option<ItemDetail>,
    /// Separator under this row, overriding the list's.
    lines: Option<ListLines>,
    /// Accessible name, when the visible text is not enough.
    aria_label: Option<String>,
    /// Platform look. Defaults to the ambient mode.
    mode: Option<ComponentMode>,
    /// Extra classes for the row.
    class: Option<String>,
    /// Extra content under the text lines.
    children: Element,
) -> Element {
    let mode = use_component_mode(mode);
    let target = Target::from_props(href, to, new_tab.unwrap_or(false));
    let interactive = onclick.is_some() || target.is_link() || checked.is_some();
    let selected = selected.unwrap_or(false);
    let disabled = disabled.unwrap_or(false);
    let show_detail = checked.is_none()
        && match detail.unwrap_or_default() {
            ItemDetail::Show => true,
            ItemDetail::Hide => false,
            ItemDetail::Auto => interactive && mode == ComponentMode::Ios,
        };
    let lines_cls = match lines {
        Some(ListLines::Full) => "g3-item-lines-full",
        Some(ListLines::Inset) => "g3-item-lines-inset",
        Some(ListLines::None) => "g3-item-lines-none",
        None => "",
    };
    let cls = merge_classes(
        classes([
            "g3-item",
            mode.pick("g3-item-ios", "g3-item-md"),
            lines_cls,
            if interactive { "g3-item-button" } else { "" },
            if selected { "g3-item-selected" } else { "" },
            if disabled { "g3-item-disabled" } else { "" },
        ]),
        class.as_deref(),
    );
    let end = match checked {
        Some(_) => Some(rsx! {
            span { class: "g3-item-check", aria_hidden: "true",
                span { class: "g3-item-check-mark" }
            }
        }),
        None => end.map(|end| {
            rsx! {
                span { class: "g3-item-end", {end} }
            }
        }),
    };
    let content = rsx! {
        if let Some(start) = start {
            span { class: "g3-item-start", {start} }
        }
        span { class: "g3-item-main",
            if let Some(overline) = overline {
                span { class: "g3-item-overline", "{overline}" }
            }
            if let Some(label) = label {
                span { class: "g3-item-label", "{label}" }
            }
            if let Some(description) = description {
                span { class: "g3-item-description", "{description}" }
            }
            {children}
        }
        if let Some(metadata) = metadata {
            span { class: "g3-item-metadata", "{metadata}" }
        }
        {end}
        if show_detail {
            span { class: "g3-item-detail", aria_hidden: "true",
                ChevronRight { size: 18 }
            }
        }
    };
    let mut attributes = Vec::new();
    if let Some(checked) = checked {
        attributes.push(Attribute::new("role", "checkbox", None, false));
        attributes.push(Attribute::new(
            "aria-checked",
            checked.to_string(),
            None,
            false,
        ));
    }
    if selected && interactive {
        attributes.push(Attribute::new("aria-current", "true", None, false));
    }
    if let Some(label) = aria_label.clone() {
        attributes.push(Attribute::new("aria-label", label, None, false));
    }
    rsx! {
        div { class: "g3-item-row", role: "listitem",
            if interactive {
                Pressable { class: cls, target, disabled, onclick, attributes, {content} }
            } else {
                div { class: cls, aria_label, {content} }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ListPlaygroundDemo() -> Element {
    use crate::{Color, SwipeAction, SwipeBehavior, SwipeItem, use_toast};
    let variant = use_signal(|| ListVariant::Grouped);
    let lines = use_signal(|| ListLines::Inset);
    let start_behavior = use_signal(|| SwipeBehavior::Activate);
    let end_behavior = use_signal(|| SwipeBehavior::Reveal);
    let mut notify = use_signal(|| true);
    let mut rows = use_signal(|| vec!["Round 12", "Round 11", "Round 10", "Round 9"]);
    let toast = use_toast();
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                crate::SegmentGroup { value: variant, aria_label: "Variant",
                    crate::SegmentButton { value: ListVariant::Plain, "Plain" }
                    crate::SegmentButton { value: ListVariant::Grouped, "Grouped" }
                }
                crate::SegmentGroup { value: lines, aria_label: "Lines",
                    crate::SegmentButton { value: ListLines::Full, "Full lines" }
                    crate::SegmentButton { value: ListLines::Inset, "Inset lines" }
                    crate::SegmentButton { value: ListLines::None, "No lines" }
                }
                crate::Select {
                    label: "Swipe right",
                    value: start_behavior,
                    options: vec![
                        crate::SelectOption::new(SwipeBehavior::Reveal, "Reveal buttons"),
                        crate::SelectOption::new(SwipeBehavior::Activate, "Activate (archive)"),
                        crate::SelectOption::new(SwipeBehavior::Dismiss, "Dismiss"),
                    ],
                }
                crate::Select {
                    label: "Swipe left",
                    value: end_behavior,
                    options: vec![
                        crate::SelectOption::new(SwipeBehavior::Reveal, "Reveal buttons"),
                        crate::SelectOption::new(SwipeBehavior::Activate, "Activate (delete)"),
                        crate::SelectOption::new(SwipeBehavior::Dismiss, "Dismiss"),
                    ],
                }
            },
            List { variant: variant(), lines: lines(),
                ListHeader { "Settings" }
                Item { label: "Profile", description: "Name and handicap", onclick: |_| {} }
                Item { label: "Notifications", checked: notify(), onclick: move |_| notify.toggle() }
                Item { label: "Version", metadata: "0.4.0" }
            }
            List { variant: variant(), lines: lines(),
                ListHeader { "Rounds — swipe either way" }
                for row in rows() {
                    SwipeItem {
                        key: "{row}",
                        start_behavior: start_behavior(),
                        end_behavior: end_behavior(),
                        start_actions: rsx! {
                            SwipeAction {
                                color: Color::Success,
                                onclick: move |_| { toast.success(format!("{row} archived")); },
                                "Archive"
                            }
                        },
                        end_actions: rsx! {
                            if end_behavior() == SwipeBehavior::Reveal {
                                SwipeAction {
                                    color: Color::Accent,
                                    onclick: move |_| { toast.show(format!("{row} pinned")); },
                                    "Pin"
                                }
                            }
                            SwipeAction {
                                color: Color::Danger,
                                onclick: move |_| rows.write().retain(|r| *r != row),
                                "Delete"
                            }
                        },
                        on_activate: move |state: crate::SwipeState| match state.side {
                            crate::SwipeSide::Start => {
                                toast.success(format!("{row} archived"));
                            }
                            crate::SwipeSide::End => rows.write().retain(|r| *r != row),
                        },
                        on_dismiss: move |_| rows.write().retain(|r| *r != row),
                        Item { label: row, description: "Pebble Creek" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "List",
    description: "Lists, rows, section headers, and swipeable rows.",
    demo: ListPlaygroundDemo,
    source: "src/components/list.rs",
}
