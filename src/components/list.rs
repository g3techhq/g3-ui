//! Lists and list rows.
use super::pressable::Destination;
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

/// How a [`List`] sits on the page. Apart from `EdgeToEdge`, these match
/// [`CardVariant`](crate::CardVariant): the rows sit in a rounded group,
/// like an iOS settings screen, drawn the way a card with that variant is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ListVariant {
    /// Rows span the full width of the screen, with no group around them,
    /// like a phone inbox or a settings page on Android. Put the list in
    /// [`Content`](crate::Content) with `padding: false` so nothing insets
    /// it; a sheet or drawer takes the same `padding` prop.
    #[default]
    EdgeToEdge,
    /// A rounded group lifted off the page with a shadow.
    Raised,
    /// A rounded group outlined by a border.
    Flat,
    /// A rounded group on a tinted surface.
    Filled,
}

/// Marks the children of a [`List`], so rows know to be list items.
#[derive(Clone, Copy)]
pub(crate) struct InList;

/// Marks the content of a [`SwipeItem`](crate::SwipeItem): the swipe row is
/// the list item, so the row inside it is not another one.
#[derive(Clone, Copy)]
pub(crate) struct InSwipeRow;

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
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # #[derive(Routable, Clone, Debug, PartialEq)]
/// # enum Route {
/// #     #[route("/")]
/// #     Profile {},
/// # }
/// # #[component] fn Profile() -> Element { rsx! {} }
/// # let notify = use_signal(|| false);
/// rsx! {
///     List { variant: ListVariant::Raised,
///         ListHeader { "Account" }
///         Item { label: "Profile", to: Route::Profile {} }
///         Item { label: "Notifications", end: rsx! { Toggle { checked: notify, aria_label: "Notifications" } } }
///     }
/// }
/// # }
/// ```
#[component]
pub fn List(
    /// Edge to edge, or a rounded group drawn like a card. Defaults to
    /// [`ListVariant::EdgeToEdge`].
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
    use_context_provider(|| InList);
    let cls = classes([
        "g3-list",
        mode.pick("g3-list-ios", "g3-list-md"),
        match variant.unwrap_or_default() {
            ListVariant::EdgeToEdge => "",
            ListVariant::Raised => "g3-list-grouped",
            ListVariant::Flat => "g3-list-grouped g3-list-flat",
            ListVariant::Filled => "g3-list-grouped g3-list-filled",
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
    /// Heading level of the header, 1 to 6. Defaults to 2; set it so the page's
    /// headings do not skip a level.
    heading_level: Option<u8>,
    /// Extra classes for the header row.
    class: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: merge_classes("g3-list-header-row", class.as_deref()),
            role: "listitem",
            super::text::Heading { level: heading_level.unwrap_or(2), class: "g3-list-header", {children} }
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
    #[props(default, into)]
    to: Destination,
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
    /// Let the label and description wrap onto more lines instead of being
    /// cut short, for text the reader needs in full. Like Ionic's
    /// `ion-text-wrap`.
    wrap: Option<bool>,
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
    let list_item = use_hook(|| {
        try_consume_context::<InList>().is_some() && try_consume_context::<InSwipeRow>().is_none()
    });
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
            if wrap.unwrap_or(false) {
                "g3-item-wrap"
            } else {
                ""
            },
        ]),
        class.as_deref(),
    );
    // Controls in `end`, such as a Follow button, cannot sit inside the row's
    // own button: a button in a button is invalid, and neither can then be
    // reached properly by keyboard or a screen reader. Such a row makes only
    // its text the action, stretched over the row, and keeps the controls
    // beside it, as a Card does.
    let split = interactive && checked.is_none() && end.is_some();
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
    let text = rsx! {
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
    };
    let start = start.map(|start| {
        rsx! {
            span { class: "g3-item-start", {start} }
        }
    });
    let trailing = rsx! {
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
    // A name belongs on the control when there is one, otherwise on the list
    // item; a plain `div` may not carry one.
    let (control_label, row_label) = if interactive {
        (aria_label, None)
    } else {
        (None, aria_label)
    };
    if let Some(label) = control_label {
        attributes.push(Attribute::new("aria-label", label, None, false));
    }
    rsx! {
        div {
            class: "g3-item-row",
            role: list_item.then_some("listitem"),
            aria_label: row_label.filter(|_| list_item),
            if split {
                div { class: merge_classes(cls, Some("g3-item-split")),
                    {start}
                    Pressable {
                        class: "g3-item-main g3-item-action",
                        target,
                        disabled,
                        onclick,
                        attributes,
                        {text}
                    }
                    {trailing}
                }
            } else if interactive {
                Pressable { class: cls, target, disabled, onclick, attributes,
                    {start}
                    span { class: "g3-item-main", {text} }
                    {trailing}
                }
            } else {
                div { class: cls,
                    {start}
                    span { class: "g3-item-main", {text} }
                    {trailing}
                }
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ListPlaygroundDemo() -> Element {
    use crate::{Color, SwipeAction, SwipeBehavior, SwipeItem, use_toast};
    let variant = use_signal(|| ListVariant::Raised);
    let lines = use_signal(|| ListLines::Inset);
    let start_behavior = use_signal(|| SwipeBehavior::Activate);
    let end_behavior = use_signal(|| SwipeBehavior::Reveal);
    let mut notify = use_signal(|| true);
    const ROWS: [&str; 4] = ["Round 12", "Round 11", "Round 10", "Round 9"];
    let mut rows = use_signal(|| ROWS.to_vec());
    let toast = use_toast();
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            // An edge-to-edge list is meant to touch the screen edges.
            padding: variant() != ListVariant::EdgeToEdge,
            controls: rsx! {
                crate::SegmentGroup { value: variant, aria_label: "Variant",
                    crate::SegmentButton { value: ListVariant::EdgeToEdge, "Edge to edge" }
                    crate::SegmentButton { value: ListVariant::Raised, "Raised" }
                    crate::SegmentButton { value: ListVariant::Flat, "Flat" }
                    crate::SegmentButton { value: ListVariant::Filled, "Filled" }
                }
                crate::SegmentGroup { value: lines, aria_label: "Lines",
                    crate::SegmentButton { value: ListLines::Full, "Full lines" }
                    crate::SegmentButton { value: ListLines::Inset, "Inset lines" }
                    crate::SegmentButton { value: ListLines::None, "No lines" }
                }
                crate::Button {
                    fill: crate::ButtonFill::Outline,
                    color: crate::Color::Neutral,
                    disabled: rows.read().len() == ROWS.len(),
                    onclick: move |_| rows.set(ROWS.to_vec()),
                    "Restore deleted rows"
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
            crate::Stack {
                gap: if variant() == ListVariant::EdgeToEdge { crate::Space::None } else { crate::Space::Lg },
                List { variant: variant(), lines: lines(),
                    ListHeader { "Settings" }
                    Item { label: "Profile", description: "Name and handicap", onclick: |_| {} }
                    Item { label: "Notifications", checked: notify(), onclick: move |_| notify.toggle() }
                    Item { label: "Version", metadata: "0.4.0" }
                    Item {
                        label: "Handicap",
                        description: "Wraps rather than cutting short: the average of your best eight differentials from your last twenty rounds.",
                        wrap: true,
                    }
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
}

crate::g3_playground! {
    name: "List",
    description: "Lists, rows, section headers, and swipeable rows.",
    components: ["List", "ListHeader", "Item", "SwipeItem", "SwipeAction"],
    demo: ListPlaygroundDemo,
    source: "src/components/list.rs",
}
