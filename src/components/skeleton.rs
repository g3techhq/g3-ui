//! Loading placeholders.
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// The shape of a [`Skeleton`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SkeletonShape {
    /// One line of text.
    #[default]
    Text,
    /// A block standing in for an image or card.
    Block,
    /// A circle standing in for an avatar.
    Avatar,
    /// The height of a list row.
    Row,
}

/// A pulsing placeholder shown while content loads. Like Ionic's
/// `ion-skeleton-text`. Hidden from screen readers; announce loading with a
/// [`Spinner`](crate::Spinner) or text instead.
#[component]
pub fn Skeleton(
    /// Shape. Defaults to [`SkeletonShape::Text`].
    shape: Option<SkeletonShape>,
    /// Width as a CSS length, such as `"60%"`.
    width: Option<String>,
    /// Extra classes for the placeholder.
    class: Option<String>,
) -> Element {
    let shape_cls = match shape.unwrap_or_default() {
        SkeletonShape::Text => "g3-skeleton-text",
        SkeletonShape::Block => "g3-skeleton-block",
        SkeletonShape::Avatar => "g3-skeleton-avatar",
        SkeletonShape::Row => "g3-skeleton-row",
    };
    rsx! {
        span {
            class: merge_classes(format!("g3-skeleton {shape_cls}"), class.as_deref()),
            style: width.map(|width| format!("width: {width};")),
            aria_hidden: "true",
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn SkeletonPlaygroundDemo() -> Element {
    rsx! {
        crate::PlaygroundDemoFrame {
            div { class: "playground-stack",
                div { class: "playground-row",
                    Skeleton { shape: SkeletonShape::Avatar }
                    div { class: "playground-stack",
                        Skeleton { width: "70%" }
                        Skeleton { width: "40%" }
                    }
                }
                Skeleton { shape: SkeletonShape::Block }
                Skeleton { shape: SkeletonShape::Row }
            }
        }
    }
}

crate::g3_playground! {
    name: "Skeleton",
    description: "Pulsing placeholders for loading content.",
    demo: SkeletonPlaygroundDemo,
    source: "src/components/skeleton.rs",
}
