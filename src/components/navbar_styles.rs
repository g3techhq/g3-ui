//! Style constants for Navbar component.
#![allow(dead_code)]

pub const NAVBAR_BASE: &str = "g3-navbar flex min-h-0 flex-1 flex-col overflow-hidden bg-white";
pub const NAVBAR_IOS: &str = "g3-navbar-ios";
pub const NAVBAR_MD: &str = "g3-navbar-md";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("NAVBAR_BASE", NAVBAR_BASE),
        ("NAVBAR_IOS", NAVBAR_IOS),
        ("NAVBAR_MD", NAVBAR_MD),
    ]
}
