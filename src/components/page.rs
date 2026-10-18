//! The root element of a page beside a drawer or side sheet.
use crate::theme::merge_classes;
use dioxus::prelude::*;

/// One page of the app: its [`Header`](crate::Header) and
/// [`Content`](crate::Content), filling the shell. Like Ionic's `ion-page`.
///
/// A [`NavigationDrawer`](crate::NavigationDrawer) or a pushing
/// [`SideSheet`](crate::SideSheet) needs exactly one page element beside it
/// in the [`AppWrapper`](crate::AppWrapper), which it narrows or moves; this
/// is that element. It stacks the header over the content, gives the content
/// the rest of the height to scroll in, and paints the page background.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// let drawer_open = use_signal(|| true);
/// rsx! {
///     AppWrapper {
///         NavigationDrawer { open: drawer_open, padding: false,
///             List { Item { label: "Rounds", onclick: |_| {} } }
///         }
///         Page {
///             Header { title: "Rounds" }
///             Content { Card { title: "Saturday four-ball" } }
///         }
///     }
/// }
/// # }
/// ```
#[component]
pub fn Page(
    /// Extra classes for the page element.
    class: Option<String>,
    /// The page: usually a `Header` and a `Content`.
    children: Element,
) -> Element {
    rsx! {
        div { class: merge_classes("g3-page", class.as_deref()), {children} }
    }
}

#[cfg(feature = "playground")]
#[component]
fn PagePlaygroundDemo() -> Element {
    use dioxus_icons::lucide::Menu;
    let mut open = use_signal(|| true);
    rsx! {
        crate::PlaygroundDemoFrame {
            app: false,
            controls: rsx! {
                crate::Checkbox { checked: open, label: "Drawer open" }
            },
            crate::AppWrapper { class: "g3-playground-device-app",
                crate::NavigationDrawer { open, padding: false,
                    crate::List { lines: crate::ListLines::None,
                        crate::Item { label: "Rounds", selected: true, onclick: |_| {} }
                        crate::Item { label: "Players", onclick: |_| {} }
                    }
                }
                Page {
                    crate::Header {
                        title: "Rounds",
                        start: rsx! {
                            crate::Button {
                                fill: crate::ButtonFill::Clear,
                                aria_label: "Toggle navigation",
                                onclick: move |_| open.toggle(),
                                Menu { size: 22 }
                            }
                        },
                    }
                    crate::Content { footer_space: false,
                        crate::Card { title: "One page", "The header stays put while the content scrolls, and the page narrows beside the drawer." }
                    }
                }
            }
        }
    }
}

crate::g3_playground! {
    name: "Page",
    description: "The page beside a drawer: header over scrolling content.",
    demo: PagePlaygroundDemo,
    source: "src/components/page.rs",
}
