//! Seed-based theme generation: color parsing, contrast, and readability.
use crate::Theme;
use crate::palette::*;

fn generate(accent: &str, background: &str) -> Result<GeneratedTheme, InvalidSeed> {
    ThemeSeeds::new(accent, background).generate()
}

#[test]
fn parses_hex_and_rgb_and_rejects_other_css() {
    assert_eq!(Rgba::parse("#fff").unwrap().hex(), "#ffffff");
    assert_eq!(Rgba::parse("#2563eb").unwrap().hex(), "#2563eb");
    assert_eq!(Rgba::parse("#00000080").unwrap().a, 128.0 / 255.0);
    assert_eq!(Rgba::parse("rgba(37, 99, 235, 0.2)").unwrap().a, 0.2);
    assert!(Rgba::parse("light-dark(white, black)").is_none());
    assert!(Rgba::parse("#éa").is_none());
}

#[test]
fn oklch_round_trips() {
    for hex in [
        "#2563eb", "#34d399", "#141b2d", "#f4efe6", "#000000", "#ffffff",
    ] {
        let color = Rgba::parse(hex).unwrap();
        assert_eq!(color.oklch().rgba().hex(), hex);
    }
}

#[test]
fn contrast_matches_wcag() {
    let white = Rgba::parse("#ffffff").unwrap();
    let black = Rgba::parse("#000000").unwrap();
    assert!((contrast(white, black) - 21.0).abs() < 1e-9);
    assert!((contrast(white, white) - 1.0).abs() < 1e-9);
}

#[test]
fn background_lightness_picks_the_scheme() {
    let light = generate("#2563eb", "#f4f6fb").unwrap().theme;
    let dark = generate("#2563eb", "#141b2d").unwrap().theme;
    assert_eq!(light.color_scheme, "light");
    assert_eq!(dark.color_scheme, "dark");
}

#[test]
fn a_seed_that_is_not_a_color_is_named_in_the_error() {
    let error = generate("tomato", "#ffffff").unwrap_err();
    assert_eq!(error.seed, Seed::Accent);
    assert_eq!(error.value, "tomato");
    let error = ThemeSeeds::new("#2563eb", "#ffffff")
        .with_accent_secondary("var(--brand)")
        .generate()
        .unwrap_err();
    assert_eq!(error.seed, Seed::AccentSecondary);
    assert!(Theme::from_seeds(&ThemeSeeds::new("#2563eb", "white")).is_err());
}

#[test]
fn generated_themes_keep_text_readable_on_every_surface() {
    for (accent, background) in [
        ("#2563eb", "#ffffff"),
        ("#facc15", "#ffffff"),
        ("#1d4ed8", "#000000"),
        ("#34d399", "#13201a"),
        ("#f472b6", "#1a1020"),
        ("#c2410c", "#f4efe6"),
        ("#7c3aed", "#808080"),
    ] {
        let theme = generate(accent, background).unwrap().theme;
        let color = |value: &str| Rgba::parse(value).unwrap();
        for surface in [&theme.bg, &theme.bg_secondary, &theme.card, &theme.surface] {
            for text in [
                &theme.text,
                &theme.text_secondary,
                &theme.text_tertiary,
                &theme.accent,
                &theme.danger,
            ] {
                let ratio = contrast(color(text), color(surface));
                assert!(
                    ratio >= TEXT_CONTRAST - 0.01,
                    "{text} on {surface} is {ratio:.2} for seeds {accent} / {background}"
                );
            }
        }
        let on_accent = contrast(color(&theme.on_accent), color(&theme.accent));
        assert!(on_accent >= TEXT_CONTRAST, "on accent {on_accent:.2}");
    }
}

#[test]
fn a_readable_accent_is_kept_and_a_moved_one_is_reported() {
    let kept = generate("#1d4ed8", "#ffffff").unwrap();
    assert_eq!(kept.theme.accent, "#1d4ed8");
    assert!(kept.adjustments.is_empty());
    let moved = generate("#facc15", "#ffffff").unwrap();
    assert_ne!(moved.theme.accent, "#facc15");
    assert_eq!(moved.adjustments.len(), 1);
    let note = &moved.adjustments[0];
    assert_eq!(note.seed, Seed::Accent);
    assert_eq!(
        (note.from.as_str(), note.to.as_str()),
        ("#facc15", moved.theme.accent.as_str())
    );
    assert!(!note.lightened);
    assert!(
        note.to_string()
            .starts_with("Accent #facc15 was darkened to #")
    );
}

#[test]
fn a_dark_page_lightens_a_dim_accent() {
    let moved = generate("#1d4ed8", "#000000").unwrap();
    assert!(moved.adjustments[0].lightened);
}

#[test]
fn from_seeds_is_the_generated_theme() {
    let seeds = ThemeSeeds::new("#c2410c", "#f4efe6");
    assert_eq!(
        Theme::from_seeds(&seeds).unwrap(),
        seeds.generate().unwrap().theme
    );
}

#[test]
fn a_second_accent_seed_is_used_as_given() {
    let theme = ThemeSeeds::new("#f472b6", "#1a1020")
        .with_accent_secondary("#fb923c")
        .generate()
        .unwrap()
        .theme;
    assert_eq!(theme.accent_secondary, "#fb923c");
}
