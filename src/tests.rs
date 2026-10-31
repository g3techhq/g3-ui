//! Rendering tests.
//!
//! Components are rendered to HTML with `dioxus-ssr` and checked for their
//! markup contract: roles, ARIA attributes, ids, and classes. Every render
//! also checks that each `g3-` class it emits is defined in the stylesheet.
use crate::prelude::*;
// Both preludes export an `InputType` (Dioxus's is the `beforeinput` event's);
// the explicit import picks g3-ui's for every test module.
use crate::prelude::InputType;
use dioxus::prelude::*;
use std::collections::BTreeSet;

mod components;

mod display;

mod forms;

mod overlays;

mod palette;

mod regressions;

mod responsive;

mod stylesheet;

mod theme_state;

const STYLESHEET: &str = include_str!("../assets/g3-ui.css");

/// Classes the stylesheet defines.
fn stylesheet_classes() -> BTreeSet<String> {
    let mut classes = BTreeSet::new();
    let bytes = STYLESHEET.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'.' && STYLESHEET[i + 1..].starts_with("g3-") {
            let rest = &STYLESHEET[i + 1..];
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                .unwrap_or(rest.len());
            classes.insert(rest[..end].to_string());
            i += end;
        }
        i += 1;
    }
    classes
}

/// Classes the markup uses.
fn html_classes(html: &str) -> BTreeSet<String> {
    let mut classes = BTreeSet::new();
    for chunk in html.split("class=\"").skip(1) {
        let value = chunk.split('"').next().unwrap_or_default();
        classes.extend(value.split_whitespace().map(str::to_string));
    }
    classes
}

/// Render a component tree to HTML and check its classes exist.
pub(crate) fn render(app: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    let defined = stylesheet_classes();
    let missing: Vec<_> = html_classes(&html)
        .into_iter()
        .filter(|class| class.starts_with("g3-") && !defined.contains(class))
        .collect();
    assert!(
        missing.is_empty(),
        "classes missing from g3-ui.css: {missing:?}"
    );
    html
}

/// The opening tag of the first element with `class` among its classes.
pub(crate) fn element_with_class<'a>(html: &'a str, class: &str) -> &'a str {
    let (open, close) = class_tag(html, class);
    &html[open..=close]
}

/// The text directly inside the first element with `class`, up to its first
/// child tag.
///
/// Assert on this rather than on `class="..">text`: dioxus-ssr writes an
/// element's static attributes before its dynamic ones, whatever order the
/// `rsx!` names them in, so what precedes the `>` is not stable.
pub(crate) fn text_of_class<'a>(html: &'a str, class: &str) -> &'a str {
    let (_, close) = class_tag(html, class);
    let text = &html[close + 1..];
    &text[..text.find('<').unwrap_or(text.len())]
}

/// Where the opening tag of the first element with `class` starts and ends.
fn class_tag(html: &str, class: &str) -> (usize, usize) {
    let mut search = 0;
    while let Some(offset) = html[search..].find("class=\"") {
        let start = search + offset + "class=\"".len();
        let value = html[start..].split('"').next().unwrap_or_default();
        if value.split_whitespace().any(|c| c == class) {
            let open = html[..start].rfind('<').expect("tag start");
            let close = start + html[start..].find('>').expect("tag end");
            return (open, close);
        }
        search = start;
    }
    panic!("no element with class {class} in:\n{html}");
}

/// The opening tag of the first element carrying `attribute` (a whole
/// `name="value"` pair), wherever it sits among the element's attributes.
pub(crate) fn element_with_attribute<'a>(html: &'a str, attribute: &str) -> &'a str {
    let at = html
        .find(attribute)
        .unwrap_or_else(|| panic!("no element with {attribute} in:\n{html}"));
    let open = html[..at].rfind('<').expect("tag start");
    let close = at + html[at..].find('>').expect("tag end");
    &html[open..=close]
}

/// Every id the markup names, in the order it names them.
pub(crate) fn ids_in(html: &str) -> Vec<String> {
    html.split(" id=\"")
        .skip(1)
        .filter_map(|chunk| chunk.split('"').next())
        .map(str::to_string)
        .collect()
}

/// The opening tag of the element with `id`.
pub(crate) fn element_with_id<'a>(html: &'a str, id: &str) -> &'a str {
    let needle = format!(" id=\"{id}\"");
    let at = html
        .find(&needle)
        .unwrap_or_else(|| panic!("no element with id {id}"));
    let open = html[..at].rfind('<').expect("tag start");
    let close = at + html[at..].find('>').expect("tag end");
    &html[open..=close]
}

/// Every `id="..."` value in the markup.
pub(crate) fn ids(html: &str) -> Vec<String> {
    html.split(" id=\"")
        .skip(1)
        .map(|chunk| chunk.split('"').next().unwrap_or_default().to_string())
        .collect()
}
