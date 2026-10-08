//! Stylesheet contracts.
use super::STYLESHEET;
use crate::{Look, Theme};

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
fn an_unset_look_writes_nothing_and_a_set_one_writes_its_tokens() {
    assert!(Theme::default_light().look.tokens().is_empty());
    assert!(!Theme::default_light().to_style_attr().contains("--g3-card"));

    let theme = Theme {
        look: Look {
            background: Some("linear-gradient(red, blue)".into()),
            card_radius: Some("18px".into()),
            ..Look::default()
        },
        ..Theme::default_light()
    };
    let style = theme.to_style_attr();
    // The page color closes the background, so the gradient is not see-through.
    assert!(
        style.contains("--g3-page-background: linear-gradient(red, blue), var(--g3-color-bg);")
    );
    assert!(style.contains("--g3-card-radius: 18px;"));
    assert!(theme.look_css("x").is_none());
}

#[test]
fn adaptive_themes_keep_both_looks_in_a_scheme_rule_not_inline() {
    let with = |radius: &str, base: Theme| Theme {
        look: Look {
            card_radius: Some(radius.into()),
            ..Look::default()
        },
        ..base
    };
    let theme = Theme::adaptive(
        with("4px", Theme::default_light()),
        with("20px", Theme::default_dark()),
    );
    // Inline declarations would beat the media rule, so none are written.
    assert!(!theme.to_style_attr().contains("--g3-card-radius"));
    let css = theme.look_css("shell").expect("a scheme rule");
    assert!(css.contains("#shell { --g3-card-radius: 4px; }"));
    assert!(css.contains("prefers-color-scheme: dark"));
    assert!(css.contains("--g3-card-radius: 20px;"));

    let same = Theme::adaptive(
        with("4px", Theme::default_light()),
        with("4px", Theme::default_dark()),
    );
    assert!(same.look_dark.is_none());
    assert!(same.to_style_attr().contains("--g3-card-radius: 4px;"));
}

#[test]
fn cards_and_page_layers_read_the_look_tokens() {
    let css = code();
    for token in [
        "--g3-page-background",
        "--g3-card-radius",
        "--g3-card-border",
        "--g3-card-shadow",
        "--g3-card-sheen",
        "--g3-card-backdrop-filter",
        "--g3-accent-fill",
    ] {
        assert!(css.contains(&format!("var({token}")), "{token} is unused");
    }
    // The route-transition layers paint `--route-transition-bg`.
    assert!(css.contains("--route-transition-bg: var(--g3-page-background"));
}

#[test]
fn a_nested_theme_clears_the_look_it_does_not_set() {
    let theme = Theme {
        look: Look {
            card_radius: Some("18px".into()),
            ..Look::default()
        },
        ..Theme::default_light()
    };
    let nested = theme.style_attr(true);
    assert!(nested.contains("--g3-card-radius: 18px;"));
    // `initial` is the guaranteed-invalid value, so `var(--x, fallback)` falls back.
    assert!(nested.contains("--g3-page-background: initial;"));
    assert!(nested.contains("--g3-accent-fill: initial;"));
    assert!(!nested.contains("--g3-card-radius: initial;"));
    assert!(!theme.to_style_attr().contains("initial"));
}

#[test]
fn a_plain_list_in_a_card_paints_no_row_fill_of_its_own() {
    let css = code();
    assert!(css.contains("background: var(--g3-list-row-surface, var(--g3-list-surface));"));
    assert!(
        rule_body(".g3-card .g3-list:not(.g3-list-grouped)")
            .contains("--g3-list-row-surface: transparent;")
    );
    // A swipe row slides over its actions, so it keeps an opaque fill.
    assert!(css.contains("background: var(--g3-list-surface, var(--g3-color-card));"));
}

#[test]
fn the_accent_fill_reaches_only_the_accent_button() {
    let css = code();
    assert!(css.contains("--g3-btn-fill: var(--g3-accent-fill, var(--g3-btn-color));"));
    for color in ["neutral", "success", "warning", "danger"] {
        let body = rule_body(&format!(".g3-btn-{color}"));
        assert!(
            body.contains("--g3-btn-fill: var(--g3-btn-color);"),
            ".g3-btn-{color} keeps its own fill"
        );
    }
    assert!(rule_body(".g3-btn-solid").contains("background: var(--g3-btn-fill);"));
}

#[test]
fn the_second_accent_follows_the_accent_until_set() {
    let theme = Theme::default_light().with_accent("#1f7a4d");
    assert_eq!(theme.accent_secondary, "#1f7a4d");
    let split = Theme {
        accent_secondary: "#ff8800".into(),
        ..Theme::default_light()
    }
    .with_accent("#1f7a4d");
    assert_eq!(split.accent_secondary, "#ff8800");
}

#[test]
fn component_rules_live_in_the_g3_layer() {
    let layer = STYLESHEET.find("@layer g3 {").expect("g3 layer");
    let first_rule = STYLESHEET.find(":root {").expect(":root rule");
    assert!(layer < first_rule, "rules must start inside the layer");
    // Inside a layer `!important` would beat an app's own overrides.
    assert!(!code().contains("!important"));
}

/// The sheet names the layer order before anything else can. Tailwind's
/// preflight lives in `base`; were `g3` ranked below it, the reset would
/// strip every component's padding, margins and borders.
#[test]
fn the_g3_layer_ranks_above_a_css_reset_and_below_utilities() {
    let order = STYLESHEET
        .find("@layer theme, base, g3, components, utilities;")
        .expect("layer order statement");
    let layer = STYLESHEET.find("@layer g3 {").expect("g3 layer");
    assert!(order < layer, "the order is named before the layer opens");
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

#[test]
fn overlays_enter_on_a_keyframe_rather_than_a_transition() {
    // An overlay is rendered only once its top-layer wrapper opens, and by
    // then it already carries its open state: a transition has nothing left
    // to animate, so the panel appears at rest. Every opening surface needs a
    // keyframe entrance, which starts when the panel first renders.
    for animation in [
        "g3-popover-enter",
        "g3-popover-sheet-enter",
        "g3-sheet-enter-bottom",
        "g3-sheet-enter-side",
    ] {
        assert!(
            STYLESHEET.contains(&format!("@keyframes {animation}")),
            "stylesheet should define @keyframes {animation}"
        );
        assert!(
            STYLESHEET.contains(&format!("animation: {animation}")),
            "stylesheet should run {animation} on an opening overlay"
        );
    }
}

#[test]
fn a_dragged_reorder_item_follows_the_pointer_without_a_transition() {
    // The list gives every item a transition while dragging, so the dragged
    // item's own rule has to outrank it or it trails behind the pointer.
    assert!(
        rule_body(".g3-reorder[data-dragging=\"true\"] .g3-reorder-item").contains("transition")
    );
    assert!(
        rule_body(".g3-reorder .g3-reorder-item[data-dragging=\"true\"]")
            .contains("transition: none;")
    );
}
