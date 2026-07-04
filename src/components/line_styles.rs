//! Style constants for Line component.
#![allow(dead_code)]

pub const LINE: &str = "g3-line";
pub const LINE_H: &str = "g3-line-h";
pub const LINE_V: &str = "g3-line-v";

pub fn catalog() -> Vec<(&'static str, &'static str)> {
    vec![("LINE", LINE), ("LINE_H", LINE_H), ("LINE_V", LINE_V)]
}
