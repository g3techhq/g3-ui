//! Responsive composition without tying a layout to the browser viewport.
use crate::state::use_element_id;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// Available width of a shell or the nearest [`LayoutScope`].
///
/// These stages match navigation: bottom bar below 48rem, icon rail from
/// 48rem, and room for labeled navigation from 64rem. They describe space,
/// not a device or its input capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LayoutSize {
    /// Less than 48rem. Also the server and initial client fallback.
    #[default]
    Compact,
    /// At least 48rem and less than 64rem.
    Medium,
    /// At least 64rem.
    Expanded,
}

impl LayoutSize {
    pub(crate) fn from_report(value: u8) -> Self {
        match value {
            2 => Self::Expanded,
            1 => Self::Medium,
            _ => Self::Compact,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Medium => "medium",
            Self::Expanded => "expanded",
        }
    }
}

/// The available width class, from the nearest [`LayoutScope`] or AppWrapper.
/// Defaults to Compact outside a provider and before the first measurement.
/// Prefer CSS for rearranging the same content; branching unmounts local state.
pub fn use_layout_size() -> LayoutSize {
    use_hook(try_consume_context::<Signal<LayoutSize>>)
        .map(|size| size())
        .unwrap_or_default()
}

pub(crate) const SIZE_SCRIPT: &str = include_str!("responsive.js");

/// Establish an available-width container inside a page, after navigation and
/// padding. Nested layout queries use this container instead of the outer shell.
/// It intentionally uses the same container name as AppWrapper; keep global
/// navigation outside this scope. Children keep their identity during resizing.
#[component]
pub fn LayoutScope(
    /// Extra classes for the full-width scope.
    class: Option<String>,
    /// Content whose responsive behavior uses this available width.
    children: Element,
) -> Element {
    let id = use_element_id("layout", None);
    let mut size = use_signal(LayoutSize::default);
    use_context_provider(|| size);
    super::script::use_dom_script(
        Some(SIZE_SCRIPT.replace("__ID__", &super::overlay::js_string(&id))),
        move |report: u8| size.set(LayoutSize::from_report(report)),
    );
    rsx! {
        div { id, class: merge_classes("g3-layout-scope", class.as_deref()), {children} }
    }
}

/// Mount just one layout for the available width.
///
/// Missing slots fall back to the nearest smaller supplied slot. Switching
/// slots unmounts the old tree: keep shared form signals, resources, and players
/// above this component. Use [`PageLayout`] to rearrange content without
/// remounting it. Server and first client render both select `compact`.
#[component]
pub fn AdaptiveLayout(
    /// Required narrow layout and initial fallback.
    compact: Element,
    /// Medium layout. Defaults to `compact`.
    medium: Option<Element>,
    /// Expanded layout. Defaults to `medium`, then `compact`.
    expanded: Option<Element>,
    /// Extra classes for the active layout wrapper.
    class: Option<String>,
) -> Element {
    let size = use_layout_size();
    let (active, content) = match (size, expanded, medium) {
        (LayoutSize::Expanded, Some(content), _) => (LayoutSize::Expanded, content),
        (LayoutSize::Medium | LayoutSize::Expanded, _, Some(content)) => {
            (LayoutSize::Medium, content)
        }
        _ => (LayoutSize::Compact, compact),
    };
    rsx! {
        div {
            class: merge_classes("g3-adaptive-layout", class.as_deref()),
            "data-size": active.as_str(),
            for (key, branch) in std::iter::once((active.as_str(), content)) {
                div { key: "{key}", {branch} }
            }
        }
    }
}

/// Reading order of a [`PageLayout`] on narrow widths. This also sets DOM and
/// keyboard order at all widths; CSS only changes the column placement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageLayoutOrder {
    /// Main content, start aside, end aside.
    #[default]
    MainStartEnd,
    /// Start aside, end aside, main content.
    StartEndMain,
}

/// A page that stacks, gains an aside column at 48rem of available width,
/// then splits the asides to either side at 80rem. Each aside is at least
/// 18rem, enough for a list row's avatar, name and action; the split waits
/// until the main column still has about 42rem. Empty slots reserve no
/// tracks. All slots stay mounted when the width changes. Measures its own
/// space after navigation.
#[component]
pub fn PageLayout(
    /// Main page content.
    children: Element,
    /// Start aside. Omitted by default.
    start: Option<Element>,
    /// End aside. Omitted by default.
    end: Option<Element>,
    /// DOM and compact reading order. Defaults to MainStartEnd.
    order: Option<PageLayoutOrder>,
    /// Extra classes for the layout grid.
    class: Option<String>,
) -> Element {
    let has_start = start.is_some();
    let has_end = end.is_some();
    let main = rsx! { div { class: "g3-page-main", {children} } };
    let start = start.map(|content| rsx! { div { class: "g3-page-start", {content} } });
    let end = end.map(|content| rsx! { div { class: "g3-page-end", {content} } });
    let asides = rsx! { div { class: "g3-page-asides", {start} {end} } };
    let content = match order.unwrap_or_default() {
        PageLayoutOrder::MainStartEnd => rsx! { {main} {asides} },
        PageLayoutOrder::StartEndMain => rsx! { {asides} {main} },
    };
    rsx! {
        LayoutScope {
            div {
                class: merge_classes("g3-page-layout", class.as_deref()),
                "data-start": has_start.to_string(),
                "data-end": has_end.to_string(),
                {content}
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ResponsivePlaygroundDemo() -> Element {
    use crate::{Card, Input, Space, Stack};
    let draft = use_signal(String::new);
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
        LayoutScope {
            Stack { gap: Space::Lg,
                AdaptiveLayout {
                    compact: rsx! { Card { title: "Compact controls", "Less than 48rem available." } },
                    medium: rsx! { Card { title: "Medium controls", "48rem to 64rem available." } },
                    expanded: rsx! { Card { title: "Expanded controls", "At least 64rem available." } },
                }
                PageLayout {
                    start: rsx! { Card { title: "Start aside", "Navigation or supporting information." } },
                    end: rsx! { Card { title: "End aside", "Related information." } },
                    Card { title: "Content stays mounted",
                        Input { label: "Draft survives resizing", value: draft }
                    }
                }
            }
        }
        }
    }
}

crate::g3_playground! {
    name: "Responsive",
    description: "Available-width scopes, adaptive variants, and stable page layouts.",
    components: ["LayoutScope", "AdaptiveLayout", "PageLayout"],
    demo: ResponsivePlaygroundDemo,
    source: "src/components/responsive.rs",
}
