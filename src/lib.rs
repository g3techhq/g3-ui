#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use manganis::{Asset, AssetOptions, asset};

/// The component stylesheet.
///
/// On the web it is linked into the document head at build time, so the page
/// never paints unstyled. [`AppWrapper`] also links it at runtime, because
/// desktop and mobile bundles only collect assets that something links at
/// runtime. On the web both resolve to the same URL.
pub static UI_CSS: Asset = asset!(
    "/assets/g3-ui.css",
    AssetOptions::css().with_static_head(true)
);

mod components;
mod descriptor;
mod state;
mod theme;

pub mod prelude;

pub use components::*;
pub use descriptor::{ComponentDescriptor, component_descriptors};
#[cfg(feature = "playground")]
#[doc(hidden)]
pub use descriptor::{
    ComponentPlaygroundDemo, PlaygroundDemoFrame, component_playground_demos, component_source,
    function_source,
};
pub use theme::{
    ComponentMode, Strings, Theme, ThemeProvider, detect_platform_mode, get_mode, init_auto_mode,
    merge_classes, set_mode, use_component_mode, use_strings, use_theme,
};

#[cfg(test)]
mod tests;
