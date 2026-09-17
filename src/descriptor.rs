//! Component metadata for playgrounds and documentation.
#[cfg(feature = "playground")]
use dioxus::prelude::*;
/// Name and one-line summary for a component, used by the playground and
/// by generated documentation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentDescriptor {
    /// The gallery entry's name, e.g. `"Button"`.
    pub name: &'static str,
    /// One sentence describing what the component is for.
    pub description: &'static str,
}
/// A runnable playground entry: a descriptor plus the demo that renders it
/// and the source shown alongside.
#[cfg(feature = "playground")]
#[derive(Clone, Copy)]
pub struct ComponentPlaygroundDemo {
    /// Which component this demo is for.
    pub descriptor: ComponentDescriptor,
    /// Renders the live demo.
    pub render: fn() -> dioxus::prelude::Element,
    /// The source file holding the demo, displayed next to it so the example
    /// can be copied.
    pub source: &'static str,
    /// The demo function's name, to find it in `source`.
    pub demo_name: &'static str,
    /// The public components the demo shows, whose own source the playground
    /// lists beside the demo's. See [`component_source`].
    pub components: &'static [&'static str],
}

/// Every file that defines a public component.
#[cfg(feature = "playground")]
const COMPONENT_FILES: &[&str] = &[
    include_str!("components/accordion.rs"),
    include_str!("components/action_sheet.rs"),
    include_str!("components/alert.rs"),
    include_str!("components/app_wrapper.rs"),
    include_str!("components/avatar.rs"),
    include_str!("components/back_button.rs"),
    include_str!("components/badge.rs"),
    include_str!("components/bottom_sheet.rs"),
    include_str!("components/button.rs"),
    include_str!("components/calendar.rs"),
    include_str!("components/card.rs"),
    include_str!("components/checkbox.rs"),
    include_str!("components/chip.rs"),
    include_str!("components/confirm_modal.rs"),
    include_str!("components/content.rs"),
    include_str!("components/date_picker.rs"),
    include_str!("components/divider.rs"),
    include_str!("components/fab.rs"),
    include_str!("components/field.rs"),
    include_str!("components/header.rs"),
    include_str!("components/infinite_scroll.rs"),
    include_str!("components/info_button.rs"),
    include_str!("components/layout.rs"),
    include_str!("components/list.rs"),
    include_str!("components/media.rs"),
    include_str!("components/modal.rs"),
    include_str!("components/nav.rs"),
    include_str!("components/navigation_drawer.rs"),
    include_str!("components/overlay_host.rs"),
    include_str!("components/popover.rs"),
    include_str!("components/progress.rs"),
    include_str!("components/radio.rs"),
    include_str!("components/range.rs"),
    include_str!("components/refresher.rs"),
    include_str!("components/searchbar.rs"),
    include_str!("components/segment.rs"),
    include_str!("components/select.rs"),
    include_str!("components/side_sheet.rs"),
    include_str!("components/skeleton.rs"),
    include_str!("components/spinner.rs"),
    include_str!("components/swipe.rs"),
    include_str!("components/tab_layout.rs"),
    include_str!("components/tabs.rs"),
    include_str!("components/text.rs"),
    include_str!("components/time_picker.rs"),
    include_str!("components/toast.rs"),
    include_str!("components/toggle.rs"),
];

/// The definition of the public component `name`: its doc comment,
/// attributes, signature, and body. `None` if no component has that name.
#[cfg(feature = "playground")]
pub fn component_source(name: &str) -> Option<&'static str> {
    COMPONENT_FILES
        .iter()
        .find_map(|file| function_source(file, &format!("pub fn {name}")))
}

/// The function whose declaration starts with `declaration`, from its doc
/// comment through its closing brace.
#[cfg(feature = "playground")]
pub fn function_source(file: &'static str, declaration: &str) -> Option<&'static str> {
    let at = [format!("{declaration}("), format!("{declaration}<")]
        .iter()
        .filter_map(|pattern| file.find(pattern.as_str()))
        .min()?;
    // Back up over the doc comment and attributes above the function.
    let mut start = file[..at].rfind('\n').map_or(0, |i| i + 1);
    while start > 0 {
        let previous = file[..start - 1].rfind('\n').map_or(0, |i| i + 1);
        let line = file[previous..start - 1].trim_start();
        if line.starts_with("///") || line.starts_with("#[") {
            start = previous;
        } else {
            break;
        }
    }
    let open = at + file[at..].find('{')?;
    let mut depth = 0_usize;
    for (offset, ch) in file[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&file[start..=open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}
#[cfg(feature = "playground")]
impl PartialEq for ComponentPlaygroundDemo {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}
/// Every component in the library, with its description.
pub fn component_descriptors() -> Vec<ComponentDescriptor> {
    crate::components::component_descriptors()
}
/// Every playground demo in the library. Requires the `playground` feature.
#[cfg(feature = "playground")]
pub fn component_playground_demos() -> Vec<ComponentPlaygroundDemo> {
    crate::components::component_playground_demos()
}
/// Frames a playground demo: its controls above a device-sized preview.
#[cfg(feature = "playground")]
#[component]
pub fn PlaygroundDemoFrame(
    children: Element,
    controls: Option<Element>,
    app: Option<bool>,
    center: Option<bool>,
    /// Pad the preview's content. Defaults to `true`.
    padding: Option<bool>,
    class: Option<String>,
) -> Element {
    crate::components::overlay::disable_body_scroll_lock_for_subtree();
    let preview_cls = crate::theme::merge_classes("g3-playground-preview", class.as_deref());
    let mode = crate::theme::use_component_mode(None);
    let body_cls = if center.unwrap_or(true) {
        "g3-playground-body-center"
    } else {
        "g3-playground-body-flow"
    };
    rsx! {
        div { class: "g3-playground-demo-stack",
            if let Some(controls) = controls {
                div { class: "playground-controls-pane", {controls} }
            }
            div { class: preview_cls,
                if app.unwrap_or(true) {
                    crate::AppWrapper { mode, class: "g3-playground-device-app",
                        crate::Content {
                            class: body_cls,
                            padding: padding.unwrap_or(true),
                            footer_space: false,
                            {children}
                        }
                    }
                } else {
                    div { class: "g3-playground-raw-surface", {children} }
                }
            }
        }
    }
}
/// Declare a component's playground entry: its `DESCRIPTOR` constant and the
/// demo wiring the playground collects.
///
/// Expands to items in the calling module, so invoke it once per component.
/// Internal: the playground collects only this crate's own entries, so an
/// entry declared elsewhere has nothing to appear in.
#[doc(hidden)]
#[macro_export]
macro_rules! g3_playground {
    (
        name: $name:literal,
        description: $description:literal,
        demo: $demo:ident,
        source: $source:literal $(,)?
    ) => {
        $crate::g3_playground! {
            name: $name,
            description: $description,
            components: [$name],
            demo: $demo,
            source: $source,
        }
    };
    (
        name: $name:literal,
        description: $description:literal,
        components: [$($component:expr),* $(,)?],
        demo: $demo:ident,
        source: $source:literal $(,)?
    ) => {
        pub const DESCRIPTOR: $crate::ComponentDescriptor = $crate::ComponentDescriptor {
            name: $name,
            description: $description,
        };
        #[cfg(feature = "playground")]
        pub const PLAYGROUND: $crate::ComponentPlaygroundDemo = $crate::ComponentPlaygroundDemo {
            descriptor: DESCRIPTOR,
            render: __g3_playground_render,
            source: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $source)),
            demo_name: stringify!($demo),
            components: &[$($component),*],
        };
        #[cfg(feature = "playground")]
        fn __g3_playground_render() -> dioxus::prelude::Element {
            rsx! { $demo {} }
        }
    };
}
