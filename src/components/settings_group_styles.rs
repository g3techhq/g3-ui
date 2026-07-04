//! Style constants for SettingsGroup component.
#![allow(dead_code)]

pub const CONTAINER: &str = "bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden";
pub const INSET: &str = "g3-settings-group-inset";
pub const LABEL: &str = "text-sm text-gray-600 font-medium uppercase tracking-wide";
pub const VALUE: &str = "text-base text-gray-900";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![
        ("CONTAINER", CONTAINER),
        ("INSET", INSET),
        ("LABEL", LABEL),
        ("VALUE", VALUE),
    ]
}
