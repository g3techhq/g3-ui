//! Style constants for Fab (floating action button) component.
#![allow(dead_code)]

pub const FAB: &str = "g3-fab";
pub const FAB_IOS: &str = "g3-fab-ios";
pub const FAB_MD: &str = "g3-fab-md";
pub const FAB_SMALL: &str = "g3-fab-small";
pub const FAB_TRANSLUCENT: &str = "g3-fab-translucent";
pub const FAB_LIST_BASE: &str = "g3-fab-list";
pub const FAB_LIST_IOS: &str = "g3-fab-list-ios";
pub const FAB_LIST_MD: &str = "g3-fab-list-md";
pub const FAB_LIST_TOP: &str = "g3-fab-list-top";
pub const FAB_LIST_BOTTOM: &str = "g3-fab-list-bottom";
pub const FAB_LIST_START: &str = "g3-fab-list-start";
pub const FAB_LIST_END: &str = "g3-fab-list-end";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("FAB", FAB),
        ("FAB_IOS", FAB_IOS),
        ("FAB_MD", FAB_MD),
        ("FAB_SMALL", FAB_SMALL),
        ("FAB_TRANSLUCENT", FAB_TRANSLUCENT),
        ("FAB_LIST_IOS", FAB_LIST_IOS),
        ("FAB_LIST_MD", FAB_LIST_MD),
        ("FAB_LIST_TOP", FAB_LIST_TOP),
        ("FAB_LIST_BOTTOM", FAB_LIST_BOTTOM),
        ("FAB_LIST_START", FAB_LIST_START),
        ("FAB_LIST_END", FAB_LIST_END),
    ]
}
