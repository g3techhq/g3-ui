//! A titled row that scrolls sideways.
use super::hscroll::use_horizontal_scroll;
use super::layout::Space;
use crate::state::use_element_id;
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// A titled row of items that scrolls sideways, like a streaming app's rows
/// of posters or a store's "New this week". Items keep their own width, so
/// the row overflows and is scrolled rather than squeezed.
///
/// Touch pans it natively, and a mouse can drag it or turn the wheel over it;
/// a wheel over a row already at one of its ends scrolls the page instead, so
/// a row never traps the page. The edges fade while there is more past them.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// # let courses = vec!["Pebble Creek", "Oak Hollow", "Mill Ridge"];
/// rsx! {
///     Shelf {
///         title: "Nearby courses",
///         end: rsx! { Button { fill: ButtonFill::Clear, size: ButtonSize::Sm, "See all" } },
///         for course in courses {
///             Card { key: "{course}", title: course, "18 holes" }
///         }
///     }
/// }
/// # }
/// ```
///
/// Give the items a width, since a row has no width of its own to share out:
/// `class: "w-40"`, or a fixed-size [`Img`](crate::Img).
#[component]
pub fn Shelf(
    /// Heading above the row.
    title: Option<String>,
    /// Content at the end of the heading, such as a "See all" link.
    end: Option<Element>,
    /// Accessible name for the row when there is no title.
    aria_label: Option<String>,
    /// Space between items. Defaults to [`Space::Md`].
    gap: Option<Space>,
    /// Settle on an item's leading edge when scrolling stops. Defaults to
    /// `true`.
    snap: Option<bool>,
    /// Heading level of the title, 1 to 6. Defaults to 2.
    heading_level: Option<u8>,
    /// Extra classes for the shelf.
    class: Option<String>,
    children: Element,
) -> Element {
    let id = use_element_id("shelf", None);
    let track_id = format!("{id}-track");
    let title_id = format!("{id}-title");
    use_horizontal_scroll(track_id.clone(), true);
    let labelled_by = title.is_some().then(|| title_id.clone());
    // A visible title names the row; the label only stands in when there is none.
    let aria_label = if title.is_none() { aria_label } else { None };
    rsx! {
        div {
            class: merge_classes("g3-shelf", class.as_deref()),
            if title.is_some() || end.is_some() {
                div { class: "g3-shelf-header",
                    if let Some(title) = title {
                        super::text::Heading {
                            level: heading_level.unwrap_or(2),
                            class: "g3-shelf-title",
                            span { id: title_id, "{title}" }
                        }
                    }
                    if let Some(end) = end {
                        div { class: "g3-shelf-end", {end} }
                    }
                }
            }
            // A group, so the scroll area can carry the title as its name. It
            // is focusable so a keyboard can scroll it even when nothing in it
            // takes focus.
            div {
                id: track_id,
                class: "g3-shelf-track",
                role: "group",
                tabindex: "0",
                aria_labelledby: labelled_by,
                aria_label,
                "data-gap": gap.unwrap_or_default().as_str(),
                "data-snap": snap.unwrap_or(true).to_string(),
                {children}
            }
        }
    }
}

#[cfg(feature = "playground")]
#[component]
fn ShelfPlaygroundDemo() -> Element {
    let snap = use_signal(|| true);
    let courses = [
        ("Pebble Creek", "18 holes · 6,512 yd"),
        ("Oak Hollow", "18 holes · 6,204 yd"),
        ("Mill Ridge", "9 holes · 3,118 yd"),
        ("Cedar Run", "18 holes · 6,880 yd"),
        ("Fox Glen", "18 holes · 6,401 yd"),
        ("Stone Bridge", "27 holes · 9,960 yd"),
    ];
    rsx! {
        crate::PlaygroundDemoFrame { center: false,
            controls: rsx! {
                crate::Checkbox { checked: snap, label: "Snap to items" }
            },
            crate::Stack { gap: Space::Lg,
                Shelf {
                    title: "Nearby courses",
                    snap: snap(),
                    end: rsx! {
                        crate::Button {
                            fill: crate::ButtonFill::Clear,
                            size: crate::ButtonSize::Sm,
                            "See all"
                        }
                    },
                    for (name, detail) in courses {
                        crate::Card { key: "{name}", class: "g3-shelf-demo-card", title: name, "{detail}" }
                    }
                }
                Shelf { title: "Filters", gap: Space::Sm, snap: false,
                    for label in ["Nearby", "Open now", "Walking", "Twilight", "Under $50", "Links", "Parkland"] {
                        crate::Chip { key: "{label}", "{label}" }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Shelf",
    description: "A titled row that scrolls sideways, by touch, drag, or wheel.",
    components: ["Shelf"],
    demo: ShelfPlaygroundDemo,
    source: "src/components/shelf.rs",
}
