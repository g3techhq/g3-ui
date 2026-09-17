//! Images and tooltips.
use super::overlay::js_string;
use crate::state::use_element_id;
use crate::theme::{classes, merge_classes};
use dioxus::prelude::*;

/// How an [`Img`] fills its box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ImgFit {
    /// Fill the box, cropping the image.
    #[default]
    Cover,
    /// Fit inside the box, letterboxing the image.
    Contain,
}

/// An image that loads lazily, holds its space with a placeholder while it
/// loads, and shows `fallback` if it fails. Like Ionic's `ion-img`.
///
/// ```rust,ignore
/// rsx! { Img { src: course.photo, alt: course.name, aspect_ratio: "16 / 9" } }
/// ```
#[component]
pub fn Img(
    /// Image URL.
    src: String,
    /// Text alternative. Use `""` for a decorative image.
    alt: String,
    /// Box proportions, as a CSS `aspect-ratio` such as `"4 / 3"`.
    aspect_ratio: Option<String>,
    /// How the image fills its box. Defaults to [`ImgFit::Cover`].
    fit: Option<ImgFit>,
    /// Shown in place of an image that fails to load.
    fallback: Option<Element>,
    /// Load immediately rather than when near the viewport.
    eager: Option<bool>,
    /// Extra classes for the wrapper.
    class: Option<String>,
) -> Element {
    let mut state = use_signal(|| "loading");
    let mut last_src = use_signal(|| src.clone());
    if *last_src.peek() != src {
        last_src.set(src.clone());
        state.set("loading");
    }
    let fit_cls = match fit.unwrap_or_default() {
        ImgFit::Cover => "",
        ImgFit::Contain => "g3-img-contain",
    };
    rsx! {
        span {
            class: merge_classes(classes(["g3-img", fit_cls]), class.as_deref()),
            "data-state": state(),
            style: aspect_ratio.map(|ratio| format!("aspect-ratio: {ratio};")),
            if state() == "error" && fallback.is_some() {
                {fallback}
            } else {
                img {
                    src,
                    alt,
                    loading: if eager.unwrap_or(false) { "eager" } else { "lazy" },
                    decoding: "async",
                    onload: move |_| state.set("loaded"),
                    onerror: move |_| state.set("error"),
                }
            }
        }
    }
}

/// Where a [`Tooltip`] appears.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum TooltipPlacement {
    /// Above the trigger.
    #[default]
    Top,
    /// Below the trigger.
    Bottom,
}

/// Points the trigger at its tooltip, unless the caller already did.
const DESCRIBE_SCRIPT: &str = r#"
const tip = document.getElementById(__ID__);
const anchor = tip && tip.parentElement;
const control = anchor && anchor.querySelector("a[href], button, input, select, textarea, [tabindex]");
if (control && control !== tip && !control.hasAttribute("aria-describedby")) {
    control.setAttribute("aria-describedby", tip.id);
}
"#;

/// A short label shown while the trigger is hovered or focused.
///
/// A tooltip only supplements a control that is already named. To have it
/// read out, give the trigger `aria_describedby` with the tooltip's `id`.
///
/// ```rust,ignore
/// rsx! {
///     Tooltip { id: "export-tip", label: "Download as CSV",
///         Button { aria_label: "Export", aria_describedby: "export-tip", Download {} }
///     }
/// }
/// ```
#[component]
pub fn Tooltip(
    /// The tooltip text.
    label: String,
    /// Where it appears. Defaults to [`TooltipPlacement::Top`].
    placement: Option<TooltipPlacement>,
    /// Element id of the tooltip. Generated when not given. The first
    /// focusable element in `children` is described by it automatically.
    id: Option<String>,
    /// Extra classes for the tooltip.
    class: Option<String>,
    /// The trigger.
    children: Element,
) -> Element {
    let id = use_element_id("tooltip", id);
    // Escape hides the tip until the pointer or focus leaves, as WCAG 1.4.13
    // asks of content that appears on hover or focus.
    let mut dismissed = use_signal(|| false);
    {
        let id = id.clone();
        use_effect(move || {
            document::eval(&DESCRIBE_SCRIPT.replace("__ID__", &js_string(&id)));
        });
    }
    let placement = match placement.unwrap_or_default() {
        TooltipPlacement::Top => "top",
        TooltipPlacement::Bottom => "bottom",
    };
    rsx! {
        span {
            class: "g3-tooltip-anchor",
            "data-dismissed": dismissed().then_some("true"),
            onkeydown: move |event| {
                if event.key() == Key::Escape && !dismissed() {
                    dismissed.set(true);
                }
            },
            onpointerleave: move |_| dismissed.set(false),
            onfocusout: move |_| dismissed.set(false),
            {children}
            span {
                id,
                class: merge_classes("g3-tooltip", class.as_deref()),
                role: "tooltip",
                "data-placement": placement,
                "{label}"
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn MediaPlaygroundDemo() -> Element {
    rsx! {
        crate::PlaygroundDemoFrame {
            crate::Stack {
                Img {
                    src: "https://picsum.photos/seed/g3-fairway/640/360",
                    alt: "A fairway at sunrise",
                    aspect_ratio: "16 / 9",
                }
                Img {
                    src: "https://invalid.example/missing.jpg",
                    alt: "Missing image",
                    aspect_ratio: "16 / 9",
                    fallback: rsx! { span { class: "playground-img-fallback", "Image unavailable" } },
                }
                Tooltip { id: "export-tip", label: "Download as CSV",
                    crate::Button { fill: crate::ButtonFill::Outline, aria_describedby: "export-tip", "Export" }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Img",
    description: "Lazy images with placeholders and fallbacks, and tooltips.",
    components: ["Img", "Tooltip"],
    demo: MediaPlaygroundDemo,
    source: "src/components/media.rs",
}
