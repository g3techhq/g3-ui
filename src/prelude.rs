//! Every component, enum, and hook in one import.
//!
//! ```
//! use dioxus::prelude::*;
//! use g3_ui::prelude::*;
//! ```
//!
//! This does not re-export Dioxus. Import `dioxus::prelude::*` yourself so
//! your app controls its own Dioxus version and names.
pub use crate::components::*;
pub use crate::theme::{
    ComponentMode, Strings, Theme, ThemeProvider, detect_platform_mode, get_mode, init_auto_mode,
    merge_classes, set_mode, use_component_mode, use_strings, use_theme,
};
