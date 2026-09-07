//! InfoButton style constants.
#![allow(dead_code)]
/// Base class for info button.
pub const BASE: &str = "g3-info-btn";
/// iOS variant - subtle circular background with spring transition.
pub const IOS: &str = "g3-info-btn-ios";
/// Android variant - flat, no background.
pub const MD: &str = "g3-info-btn-md";
/// Catalog of all style constants.
pub const ICON: &str = "g3-info-btn-icon";
pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![("base", BASE), ("ios", IOS), ("md", MD)]
}
