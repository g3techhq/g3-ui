//! Stylesheet contracts.
use super::STYLESHEET;
use crate::Theme;

/// The body of the first rule whose selector is exactly `selector`.
fn rule_body(selector: &str) -> &'static str {
    let needle = format!("\n{selector} {{");
    let start = STYLESHEET
        .find(&needle)
        .unwrap_or_else(|| panic!("no `{selector}` rule"))
        + needle.len();
    let end = start + STYLESHEET[start..].find("\n}").expect("rule end");
    &STYLESHEET[start..end]
}

/// The stylesheet without comments.
fn code() -> String {
    STYLESHEET
        .split("/*")
        .map(|chunk| chunk.split_once("*/").map_or(chunk, |(_, rest)| rest))
        .collect()
}

#[test]
fn root_tokens_match_the_default_light_theme() {
    let root = rule_body(":root");
    for (name, value) in Theme::default_light().tokens() {
        let declaration = format!("{name}: {value};");
        assert!(
            root.contains(&declaration),
            "`:root` should declare `{declaration}`"
        );
    }
}

#[test]
fn theme_style_attr_declares_every_token() {
    let style = Theme::default_dark().to_style_attr();
    for (name, value) in Theme::default_dark().tokens() {
        assert!(style.contains(&format!("{name}: {value};")));
    }
    assert!(style.ends_with("color-scheme: dark;"));
}

#[test]
fn adaptive_themes_use_light_dark_only_where_presets_differ() {
    let theme = Theme::system();
    assert_eq!(theme.warning, "light-dark(#f5b400, #ffd60a)");
    assert_eq!(theme.on_warning, "#1f1a00");
    assert_eq!(theme.accent, "light-dark(#0066d6, #4ea3ff)");
}

#[test]
fn component_rules_live_in_the_g3_layer() {
    let layer = STYLESHEET.find("@layer g3 {").expect("g3 layer");
    let first_rule = STYLESHEET.find(":root {").expect(":root rule");
    assert!(layer < first_rule, "rules must start inside the layer");
    // Inside a layer `!important` would beat an app's own overrides.
    assert!(!code().contains("!important"));
}

#[test]
fn the_stylesheet_does_not_style_the_host_page() {
    for global in [
        "\n* {",
        "\n*::-webkit-scrollbar",
        "\nhtml {",
        "\nbody {",
        "@font-face",
    ] {
        assert!(!STYLESHEET.contains(global), "unscoped `{global}` rule");
    }
}

#[test]
fn custom_properties_are_namespaced() {
    let code = code();
    for chunk in code.split("--").skip(1) {
        let name: String = chunk
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        assert!(
            name.starts_with("g3-") || name.starts_with("route-transition-") || name.is_empty(),
            "custom property `--{name}` is not namespaced"
        );
    }
}

#[test]
fn colors_come_from_theme_tokens() {
    let root_end = STYLESHEET.find(":root {").unwrap() + rule_body(":root").len();
    let rest = &STYLESHEET[root_end..];
    for line in rest.lines() {
        let line = line.trim();
        if line.starts_with("/*") || line.starts_with('*') {
            continue;
        }
        assert!(!line.contains('#'), "literal color: {line}");
        assert!(!line.contains(" white"), "literal color: {line}");
        assert!(
            !line.contains("rgba(") || line.contains("rgba(0, 0, 0"),
            "only neutral shadows may use rgba: {line}"
        );
    }
}

#[test]
fn only_g3_classes_are_styled() {
    let code = code();
    for chunk in code.split('.').skip(1) {
        let name: String = chunk
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect();
        if name.is_empty() || name.chars().next().unwrap().is_ascii_digit() {
            continue;
        }
        assert!(
            name.starts_with("g3-") || name.starts_with("route-transition-"),
            "selector uses non-g3 class `.{name}`"
        );
    }
}

#[test]
fn overlays_use_the_z_index_scale() {
    for token in ["fab", "sheet-backdrop", "sheet", "modal", "toast"] {
        assert!(STYLESHEET.contains(&format!("z-index: var(--g3-z-{token})")));
    }
}

#[test]
fn motion_respects_reduced_motion() {
    assert!(STYLESHEET.contains("@media (prefers-reduced-motion: reduce)"));
}
