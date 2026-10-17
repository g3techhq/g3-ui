//! The playground's theme presets: the header picker's choices, and the
//! starting points the theme generator loads.
use g3_ui::{Look, Theme as G3Theme};

/// The primary-button fill of the gradient themes.
const GRADIENT: &str =
    "linear-gradient(135deg, var(--g3-color-accent), var(--g3-color-accent-secondary))";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaygroundTheme {
    Light,
    Dark,
    Blue,
    Green,
    Sand,
    Ocean,
    Night,
    Forest,
    Graphite,
    Glass,
    Ember,
    Aurora,
    Custom,
}

impl PlaygroundTheme {
    pub fn label(self) -> &'static str {
        match self {
            Self::Light => "Light",
            Self::Dark => "Dark",
            Self::Blue => "Blue",
            Self::Green => "Green",
            Self::Sand => "Sand",
            Self::Ocean => "Ocean",
            Self::Night => "Night",
            Self::Forest => "Forest",
            Self::Graphite => "Graphite",
            Self::Glass => "Glass",
            Self::Ember => "Ember",
            Self::Aurora => "Aurora",
            Self::Custom => "Custom",
        }
    }

    pub fn all() -> [Self; 13] {
        [
            Self::Light,
            Self::Dark,
            Self::Blue,
            Self::Green,
            Self::Sand,
            Self::Ocean,
            Self::Night,
            Self::Forest,
            Self::Graphite,
            Self::Glass,
            Self::Ember,
            Self::Aurora,
            Self::Custom,
        ]
    }

    /// Every theme with fixed values: all of them but `Custom`.
    pub fn presets() -> impl Iterator<Item = Self> {
        Self::all()
            .into_iter()
            .filter(|theme| *theme != Self::Custom)
    }

    /// Real `g3_ui::Theme` for this brand - fed straight into `AppWrapper`'s
    /// `theme` prop, the same way any consumer of the library themes its app.
    ///
    /// The tinted presets were filled out by the theme generator from an
    /// accent and a page background (see `g3_ui::ThemeSeeds`), so each one's
    /// surfaces step evenly and its text reads at 4.5:1 on all of them.
    pub fn to_theme(self, custom: &G3Theme) -> G3Theme {
        match self {
            Self::Custom => custom.clone(),
            Self::Light => G3Theme::default_light(),
            Self::Dark => G3Theme::default_dark(),
            Self::Blue => G3Theme {
                accent: "#1f5ce4".into(),
                accent_secondary: "#1f5ce4".into(),
                on_accent: "#fafcff".into(),
                text: "#171b22".into(),
                text_secondary: "#4b5059".into(),
                text_tertiary: "#5e636d".into(),
                bg: "#eef4ff".into(),
                bg_secondary: "#e4eaf5".into(),
                card: "#fdfdff".into(),
                surface: "#fafcff".into(),
                control: "#e1e7f2".into(),
                border: "#c0c6d1".into(),
                shadow: "rgba(30, 64, 175, 0.16)".into(),
                ..G3Theme::default_light()
            },
            Self::Green => G3Theme {
                accent: "#1b774a".into(),
                accent_secondary: "#1b774a".into(),
                on_accent: "#f7fef9".into(),
                text: "#161c18".into(),
                text_secondary: "#4a524d".into(),
                text_tertiary: "#5e6560".into(),
                bg: "#edf6f0".into(),
                bg_secondary: "#e3ece6".into(),
                card: "#fafffb".into(),
                surface: "#f5fef8".into(),
                control: "#e0e9e3".into(),
                border: "#bfc8c2".into(),
                shadow: "rgba(24, 74, 47, 0.16)".into(),
                ..G3Theme::default_light()
            },
            // Warm paper with a terracotta accent.
            Self::Sand => G3Theme {
                accent: "#b43b09".into(),
                accent_secondary: "#b43b09".into(),
                on_accent: "#fffbf9".into(),
                text: "#1e1a13".into(),
                text_secondary: "#544f47".into(),
                text_tertiary: "#68635a".into(),
                bg: "#f3ede3".into(),
                bg_secondary: "#e9e3d9".into(),
                card: "#fff9ef".into(),
                surface: "#fbf5eb".into(),
                control: "#e6e0d6".into(),
                border: "#c5bfb6".into(),
                shadow: "rgba(80, 56, 30, 0.16)".into(),
                success: "#107246".into(),
                danger: "#c71108".into(),
                ..G3Theme::default_light()
            },
            // Cool sea glass with a deep teal accent.
            Self::Ocean => G3Theme {
                accent: "#06718d".into(),
                accent_secondary: "#06718d".into(),
                on_accent: "#f7fdff".into(),
                text: "#141c1e".into(),
                text_secondary: "#485254".into(),
                text_tertiary: "#5b6567".into(),
                bg: "#e8f4f6".into(),
                bg_secondary: "#deeaec".into(),
                card: "#f9feff".into(),
                surface: "#f0fcfe".into(),
                control: "#dbe7e9".into(),
                border: "#bbc6c8".into(),
                shadow: "rgba(14, 76, 92, 0.16)".into(),
                danger: "#cb180e".into(),
                ..G3Theme::default_light()
            },
            // Midnight navy. Every elevation steps lighter, so cards, sheets
            // and fields separate from the page instead of sinking into it.
            Self::Night => G3Theme {
                accent: "#7cc4ff".into(),
                accent_secondary: "#7cc4ff".into(),
                on_accent: "#001324".into(),
                text: "#f2f5fe".into(),
                text_secondary: "#c2cadf".into(),
                text_tertiary: "#9ba4bb".into(),
                bg: "#131a2e".into(),
                bg_secondary: "#0c1326".into(),
                card: "#20273c".into(),
                surface: "#272f44".into(),
                control: "#31394f".into(),
                border: "#3f485f".into(),
                shadow: "rgba(0, 0, 0, 0.45)".into(),
                ..G3Theme::default_dark()
            },
            // Pine green, kept off olive so it reads as color, not mud.
            Self::Forest => G3Theme {
                accent: "#4ade80".into(),
                accent_secondary: "#4ade80".into(),
                on_accent: "#001707".into(),
                text: "#eef8f3".into(),
                text_secondary: "#bad1c5".into(),
                text_tertiary: "#91ac9f".into(),
                bg: "#0c2a1e".into(),
                bg_secondary: "#052317".into(),
                card: "#1b382b".into(),
                surface: "#224033".into(),
                control: "#2d4b3d".into(),
                border: "#3c5a4c".into(),
                shadow: "rgba(0, 0, 0, 0.45)".into(),
                danger: "#fe7f74".into(),
                ..G3Theme::default_dark()
            },
            // Neutral charcoal with a violet accent.
            Self::Graphite => G3Theme {
                accent: "#a78bfa".into(),
                accent_secondary: "#a78bfa".into(),
                on_accent: "#130c25".into(),
                text: "#f5f5f9".into(),
                text_secondary: "#cacace".into(),
                text_tertiary: "#a4a4a8".into(),
                bg: "#18181b".into(),
                bg_secondary: "#111114".into(),
                card: "#252528".into(),
                surface: "#2c2c30".into(),
                control: "#36363a".into(),
                border: "#454548".into(),
                shadow: "rgba(0, 0, 0, 0.45)".into(),
                ..G3Theme::default_dark()
            },
            // The three `Look` themes: a page gradient, rounder edged cards
            // and a gradient primary button, over translucent (Glass,
            // Aurora) or lit (Ember) surfaces.
            Self::Glass => G3Theme {
                accent: "#6d28d9".into(),
                accent_secondary: "#c026d3".into(),
                text: "#1b1740".into(),
                text_secondary: "#4a4575".into(),
                text_tertiary: "#5a5680".into(),
                border: "rgba(124, 58, 237, 0.18)".into(),
                bg: "#eef0ff".into(),
                bg_secondary: "#e3e7ff".into(),
                card: "rgba(255, 255, 255, 0.62)".into(),
                surface: "#f7f8ff".into(),
                control: "#e4e6fa".into(),
                shadow: "rgba(79, 70, 229, 0.22)".into(),
                look: Look {
                    background: Some(
                        "radial-gradient(60% 40% at 90% 0%, #f9a8d4aa, transparent 70%), \
                         radial-gradient(70% 45% at 0% 30%, #a5b4fcbb, transparent 70%), \
                         radial-gradient(70% 40% at 80% 100%, #fcd34d66, transparent 70%)"
                            .into(),
                    ),
                    card_backdrop_filter: Some("blur(18px) saturate(1.4)".into()),
                    ..rounded_look()
                },
                ..G3Theme::default_light()
            },
            Self::Ember => G3Theme {
                accent: "#f472b6".into(),
                accent_secondary: "#fb923c".into(),
                on_accent: "#2a0a1c".into(),
                text: "#fbf3fc".into(),
                text_secondary: "#e3cfe6".into(),
                text_tertiary: "#b9a3c2".into(),
                bg: "#1a0f24".into(),
                bg_secondary: "#140a1d".into(),
                card: "#2a1b3c".into(),
                surface: "#332247".into(),
                control: "rgba(244, 114, 182, 0.14)".into(),
                border: "rgba(244, 114, 182, 0.22)".into(),
                shadow: "rgba(236, 72, 153, 0.3)".into(),
                look: Look {
                    background: Some(
                        "radial-gradient(60% 40% at 90% 0%, #f472b64d, transparent 70%), \
                         radial-gradient(70% 45% at 0% 30%, #8b5cf659, transparent 70%), \
                         radial-gradient(70% 40% at 80% 100%, #fb923c2e, transparent 70%)"
                            .into(),
                    ),
                    card_sheen: Some(
                        "linear-gradient(160deg, \
                         color-mix(in srgb, var(--g3-color-text) 7%, transparent), \
                         color-mix(in srgb, var(--g3-color-text) 1.5%, transparent))"
                            .into(),
                    ),
                    ..rounded_look()
                },
                ..G3Theme::default_dark()
            },
            // Deep blue night sky with teal and violet light behind frosted
            // cards.
            Self::Aurora => G3Theme {
                accent: "#2dd4bf".into(),
                accent_secondary: "#a78bfa".into(),
                on_accent: "#001613".into(),
                text: "#eff6fd".into(),
                text_secondary: "#bccdde".into(),
                text_tertiary: "#95a7b8".into(),
                bg: "#0f1d2a".into(),
                bg_secondary: "#081623".into(),
                card: "rgba(28, 42, 56, 0.72)".into(),
                surface: "#233240".into(),
                control: "#2d3c4b".into(),
                border: "rgba(160, 220, 235, 0.16)".into(),
                shadow: "rgba(0, 0, 0, 0.45)".into(),
                look: Look {
                    background: Some(
                        "radial-gradient(60% 40% at 85% 0%, #2dd4bf47, transparent 70%), \
                         radial-gradient(70% 45% at 0% 35%, #8b5cf652, transparent 70%), \
                         radial-gradient(70% 40% at 80% 100%, #22c55e2e, transparent 70%)"
                            .into(),
                    ),
                    card_backdrop_filter: Some("blur(18px) saturate(1.3)".into()),
                    ..rounded_look()
                },
                ..G3Theme::default_dark()
            },
        }
    }
}

/// Rounder cards with a hairline and a soft drop shadow, and a gradient
/// primary button: what the `Look` themes share.
fn rounded_look() -> Look {
    Look {
        accent_fill: Some(GRADIENT.into()),
        card_radius: Some("1.125rem".into()),
        card_border: Some("1px solid var(--g3-color-border)".into()),
        card_shadow: Some("0 0.625rem 1.875rem -0.75rem var(--g3-color-shadow)".into()),
        ..Look::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use g3_ui::palette::{Rgba, TEXT_CONTRAST, contrast};

    #[test]
    fn every_preset_keeps_text_readable_on_every_surface() {
        let custom = G3Theme::default_light();
        for preset in PlaygroundTheme::presets() {
            let theme = preset.to_theme(&custom);
            let bg = Rgba::parse(&theme.bg).unwrap();
            // A translucent card is judged over the plain page.
            let solid = |value: &str| Rgba::parse(value).unwrap().over(bg);
            let surfaces = [&theme.bg, &theme.card, &theme.surface].map(|s| solid(s));
            for (name, text) in [
                ("text", &theme.text),
                ("text_secondary", &theme.text_secondary),
                ("text_tertiary", &theme.text_tertiary),
                ("accent", &theme.accent),
            ] {
                for surface in surfaces {
                    let ratio = contrast(solid(text), surface);
                    assert!(
                        ratio >= TEXT_CONTRAST,
                        "{}: {name} {text} on {} is {ratio:.2}",
                        preset.label(),
                        surface.hex()
                    );
                }
            }
            for fill in [&theme.accent, &theme.accent_secondary] {
                let ratio = contrast(solid(&theme.on_accent), solid(fill));
                assert!(
                    ratio >= TEXT_CONTRAST,
                    "{}: on_accent on {fill} is {ratio:.2}",
                    preset.label()
                );
            }
        }
    }

    #[test]
    fn dark_presets_step_lighter_with_each_elevation() {
        let custom = G3Theme::default_light();
        for preset in PlaygroundTheme::presets() {
            let theme = preset.to_theme(&custom);
            if theme.color_scheme != "dark" {
                continue;
            }
            let bg = Rgba::parse(&theme.bg).unwrap();
            let luminance = |value: &str| Rgba::parse(value).unwrap().over(bg).luminance();
            assert!(
                luminance(&theme.bg) > 0.005,
                "{}: the page is too close to black",
                preset.label()
            );
            assert!(
                luminance(&theme.card) > luminance(&theme.bg)
                    && luminance(&theme.surface) >= luminance(&theme.card),
                "{}: cards and raised surfaces must be lighter than the page",
                preset.label()
            );
        }
    }

    #[test]
    fn every_theme_has_a_unique_label() {
        let labels: std::collections::HashSet<_> =
            PlaygroundTheme::all().iter().map(|t| t.label()).collect();
        assert_eq!(labels.len(), PlaygroundTheme::all().len());
        assert!(PlaygroundTheme::presets().all(|t| t != PlaygroundTheme::Custom));
    }
}
