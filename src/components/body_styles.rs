//! Style constants for Body component.
#![allow(dead_code)]

pub const BODY_BASE: &str = "g3-body flex-1 h-full overflow-hidden flex flex-col relative";
pub const BODY_IOS: &str = "g3-body-ios";
pub const BODY_MD: &str = "g3-body-md";
pub const BODY_CONTENT: &str =
    "g3-body-content overflow-y-auto overflow-x-hidden px-6 md:px-40 lg:px-80 pt-6 relative";
pub const FOOTER_SPACER: &str = "min-h-6";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BODY_BASE", BODY_BASE),
        ("BODY_IOS", BODY_IOS),
        ("BODY_MD", BODY_MD),
    ]
}
