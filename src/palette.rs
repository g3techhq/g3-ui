//! A whole [`Theme`] built at runtime from a few seed colors, and the color
//! math behind it: CSS color parsing and WCAG contrast.
//!
//! Give [`ThemeSeeds`] an accent and a page background, and optionally a
//! second accent. The background's lightness picks light or dark and its hue
//! tints every neutral; the text tokens are stepped until they read at 4.5:1
//! on every surface; and the accent is moved only as far as it must be to do
//! the same, which [`GeneratedTheme::adjustments`] reports.
//!
//! ```
//! use g3_ui::palette::{Seed, ThemeSeeds};
//!
//! // A color the user picked, on a warm off-white page.
//! let generated = ThemeSeeds::new("#facc15", "#f4efe6").generate()?;
//! assert_eq!(generated.theme.color_scheme, "light");
//! // Yellow is too light to read as text on that page, so it was darkened.
//! assert_eq!(generated.adjustments[0].seed, Seed::Accent);
//! # Ok::<(), g3_ui::palette::InvalidSeed>(())
//! ```
//!
//! Lightness and chroma are worked in OKLCH, where equal steps look equal, so
//! a surface ladder built from any background keeps the same rhythm.
use crate::Theme;
use std::fmt;

/// The WCAG AA ratio for body text, which every generated text token clears
/// on every surface.
pub const TEXT_CONTRAST: f64 = 4.5;

/// An sRGB color with alpha, channels in `0.0..=1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    /// Red, `0.0..=1.0`.
    pub r: f64,
    /// Green, `0.0..=1.0`.
    pub g: f64,
    /// Blue, `0.0..=1.0`.
    pub b: f64,
    /// Alpha, `0.0` (transparent) to `1.0` (opaque).
    pub a: f64,
}

/// A color in OKLCH: lightness `0..=1`, chroma, hue in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Oklch {
    l: f64,
    c: f64,
    h: f64,
}

impl Rgba {
    /// Parse `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, or `rgb()` and `rgba()`
    /// with comma separated channels. Anything else (`light-dark()`, color
    /// names, `var()`) is `None`.
    ///
    /// ```
    /// use g3_ui::palette::Rgba;
    ///
    /// assert_eq!(Rgba::parse("#2563eb").unwrap().hex(), "#2563eb");
    /// assert_eq!(Rgba::parse("rgba(0, 0, 0, 0.5)").unwrap().a, 0.5);
    /// assert!(Rgba::parse("tomato").is_none());
    /// ```
    pub fn parse(value: &str) -> Option<Self> {
        let value = value.trim();
        if let Some(hex) = value.strip_prefix('#') {
            if !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                return None;
            }
            let digits: Vec<u8> = match hex.len() {
                3 | 4 => hex.bytes().flat_map(|b| [b, b]).collect(),
                6 | 8 => hex.bytes().collect(),
                _ => return None,
            };
            let channel = |i: usize| {
                let pair = std::str::from_utf8(&digits[i * 2..i * 2 + 2]).ok()?;
                u8::from_str_radix(pair, 16)
                    .ok()
                    .map(|v| f64::from(v) / 255.0)
            };
            return Some(Self {
                r: channel(0)?,
                g: channel(1)?,
                b: channel(2)?,
                a: if digits.len() == 8 { channel(3)? } else { 1.0 },
            });
        }
        let inner = value
            .strip_prefix("rgba(")
            .or_else(|| value.strip_prefix("rgb("))?
            .strip_suffix(')')?;
        let parts: Vec<f64> = inner
            .split(',')
            .map(|part| part.trim().parse::<f64>().ok())
            .collect::<Option<_>>()?;
        match parts[..] {
            [r, g, b] => Some(Self::rgb(r / 255.0, g / 255.0, b / 255.0)),
            [r, g, b, a] => Some(Self {
                a,
                ..Self::rgb(r / 255.0, g / 255.0, b / 255.0)
            }),
            _ => None,
        }
    }

    /// An opaque color from channels in `0.0..=1.0`.
    pub const fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Six-digit hex, alpha dropped.
    pub fn hex(self) -> String {
        let byte = |c: f64| (c.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!(
            "#{:02x}{:02x}{:02x}",
            byte(self.r),
            byte(self.g),
            byte(self.b)
        )
    }

    /// The opaque color this one makes painted over an opaque `under`.
    pub fn over(self, under: Rgba) -> Rgba {
        let mix = |top: f64, bottom: f64| top * self.a + bottom * (1.0 - self.a);
        Self::rgb(
            mix(self.r, under.r),
            mix(self.g, under.g),
            mix(self.b, under.b),
        )
    }

    /// WCAG relative luminance.
    pub fn luminance(self) -> f64 {
        let [r, g, b] = self.linear();
        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    fn linear(self) -> [f64; 3] {
        let decode = |c: f64| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        [decode(self.r), decode(self.g), decode(self.b)]
    }

    fn from_linear([r, g, b]: [f64; 3]) -> Self {
        let encode = |c: f64| {
            if c <= 0.003_130_8 {
                12.92 * c
            } else {
                1.055 * c.powf(1.0 / 2.4) - 0.055
            }
        };
        Self::rgb(encode(r), encode(g), encode(b))
    }

    pub(crate) fn oklch(self) -> Oklch {
        let [r, g, b] = self.linear();
        let l = (0.412_221_470_8 * r + 0.536_332_536_3 * g + 0.051_445_992_9 * b).cbrt();
        let m = (0.211_903_498_2 * r + 0.680_699_545_1 * g + 0.107_396_956_6 * b).cbrt();
        let s = (0.088_302_461_9 * r + 0.281_718_837_6 * g + 0.629_978_700_5 * b).cbrt();
        let lightness = 0.210_454_255_3 * l + 0.793_617_785 * m - 0.004_072_046_8 * s;
        let a = 1.977_998_495_1 * l - 2.428_592_205 * m + 0.450_593_709_9 * s;
        let b = 0.025_904_037_1 * l + 0.782_771_766_2 * m - 0.808_675_766 * s;
        Oklch {
            l: lightness,
            c: a.hypot(b),
            h: b.atan2(a).to_degrees().rem_euclid(360.0),
        }
    }
}

/// The WCAG contrast ratio between two opaque colors, from `1.0` to `21.0`.
///
/// Composite a translucent color with [`Rgba::over`] first.
///
/// ```
/// use g3_ui::palette::{Rgba, TEXT_CONTRAST, contrast};
///
/// let text = Rgba::parse("#555c68").unwrap();
/// let card = Rgba::parse("#f7f8fa").unwrap();
/// assert!(contrast(text, card) >= TEXT_CONTRAST);
/// ```
pub fn contrast(a: Rgba, b: Rgba) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

impl Oklch {
    /// Back to sRGB, lowering chroma until the color is in gamut.
    pub(crate) fn rgba(self) -> Rgba {
        let mut c = self.c;
        loop {
            let (a, b) = (c * self.h.to_radians().cos(), c * self.h.to_radians().sin());
            let l = (self.l + 0.396_337_777_4 * a + 0.215_803_757_3 * b).powi(3);
            let m = (self.l - 0.105_561_345_8 * a - 0.063_854_172_8 * b).powi(3);
            let s = (self.l - 0.089_484_177_5 * a - 1.291_485_548 * b).powi(3);
            let linear = [
                4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s,
                -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s,
                -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701 * s,
            ];
            if c <= 0.0005 || linear.iter().all(|v| (-0.0001..=1.0001).contains(v)) {
                return Rgba::from_linear(linear.map(|v| v.clamp(0.0, 1.0)));
            }
            c *= 0.95;
        }
    }

    fn with(self, l: f64, c: f64) -> Self {
        Self {
            l: l.clamp(0.0, 1.0),
            c,
            h: self.h,
        }
    }
}

/// One of the colors a theme is generated from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Seed {
    /// [`ThemeSeeds::accent`].
    Accent,
    /// [`ThemeSeeds::background`].
    Background,
    /// [`ThemeSeeds::accent_secondary`].
    AccentSecondary,
}

impl fmt::Display for Seed {
    /// The seed's name in lower case: `accent`, `page background`, or
    /// `second accent`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Accent => "accent",
            Self::Background => "page background",
            Self::AccentSecondary => "second accent",
        })
    }
}

/// The colors a theme is generated from, each a hex or `rgb()` CSS color.
///
/// ```
/// use g3_ui::palette::ThemeSeeds;
///
/// let seeds = ThemeSeeds::new("#f472b6", "#1a1020").with_accent_secondary("#fb923c");
/// let theme = seeds.generate()?.theme;
/// assert_eq!(theme.color_scheme, "dark");
/// assert_eq!(theme.accent_secondary, "#fb923c");
/// # Ok::<(), g3_ui::palette::InvalidSeed>(())
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ThemeSeeds {
    /// The brand color. Moved lighter or darker, keeping its hue, when it
    /// would not read at 4.5:1 as text on the generated surfaces.
    pub accent: String,
    /// The page background, used as given. Its lightness picks light or dark,
    /// and its hue tints every neutral. A translucent one is judged over
    /// white.
    pub background: String,
    /// A second brand color for gradients (see [`Theme::accent_secondary`]),
    /// used as given. Defaults to `None`: the generated accent.
    pub accent_secondary: Option<String>,
}

impl ThemeSeeds {
    /// Seeds from an accent and a page background, with no second accent.
    pub fn new(accent: impl Into<String>, background: impl Into<String>) -> Self {
        Self {
            accent: accent.into(),
            background: background.into(),
            accent_secondary: None,
        }
    }

    /// These seeds with a second accent for gradients.
    pub fn with_accent_secondary(mut self, accent_secondary: impl Into<String>) -> Self {
        self.accent_secondary = Some(accent_secondary.into());
        self
    }

    /// Fill out all 17 colors of a [`Theme`] from these seeds, with a note
    /// for each seed that was moved to stay readable. See the
    /// [module docs](self). [`Theme::look`] is the platform look; set it on
    /// the result to keep a look of your own.
    ///
    /// # Errors
    ///
    /// [`InvalidSeed`] when a seed is not a hex or `rgb()` color.
    pub fn generate(&self) -> Result<GeneratedTheme, InvalidSeed> {
        generate(self)
    }
}

/// A seed that is not a hex or `rgb()` color, returned by
/// [`ThemeSeeds::generate`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InvalidSeed {
    /// Which seed it was.
    pub seed: Seed,
    /// The value as given.
    pub value: String,
}

impl fmt::Display for InvalidSeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the {} seed `{}` is not a hex or rgb() color",
            self.seed, self.value
        )
    }
}

impl std::error::Error for InvalidSeed {}

/// A seed color that was moved to keep the theme readable.
///
/// Its [`Display`](fmt::Display) is an English sentence; an app that shows
/// the note in another language builds its own from the fields.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Adjustment {
    /// The seed that was moved. Only [`Seed::Accent`] is moved today.
    pub seed: Seed,
    /// The seed as given, as six-digit hex.
    pub from: String,
    /// The color the theme uses instead, as six-digit hex.
    pub to: String,
    /// `true` when it was lightened (on a dark page), `false` when darkened.
    pub lightened: bool,
}

impl fmt::Display for Adjustment {
    /// For example: `Accent #facc15 was darkened to #806300 to read at 4.5:1
    /// on the surfaces.`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self.seed {
            Seed::Accent => "Accent",
            Seed::Background => "Page background",
            Seed::AccentSecondary => "Second accent",
        };
        write!(
            f,
            "{name} {} was {} to {} to read at {TEXT_CONTRAST}:1 on the surfaces.",
            self.from,
            if self.lightened {
                "lightened"
            } else {
                "darkened"
            },
            self.to
        )
    }
}

/// A theme filled out from [`ThemeSeeds`], and what was moved to keep it
/// readable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GeneratedTheme {
    /// The theme, every color token set.
    pub theme: Theme,
    /// One entry per seed that was moved; empty when every seed was used as
    /// given.
    pub adjustments: Vec<Adjustment>,
}

/// Nudge `color`'s lightness, `step` at a time, until it reaches `ratio`
/// against every color in `on` (or runs out of room).
fn readable(color: Oklch, on: &[Rgba], ratio: f64, step: f64) -> Oklch {
    let mut color = color;
    for _ in 0..100 {
        let rgba = color.rgba();
        if on.iter().all(|&bg| contrast(rgba, bg) >= ratio) {
            break;
        }
        let next = color.with(color.l + step, color.c);
        if next.l == color.l {
            break;
        }
        color = next;
    }
    color
}

/// Parse a seed, or say which one was not a color.
fn parse_seed(seed: Seed, value: &str) -> Result<Rgba, InvalidSeed> {
    Rgba::parse(value).ok_or_else(|| InvalidSeed {
        seed,
        value: value.to_string(),
    })
}

/// Neutrals take the background's hue, the text tokens are stepped until they
/// read at 4.5:1 on every surface, and the accent and status colors are moved
/// only as far as they must be to do the same.
fn generate(seeds: &ThemeSeeds) -> Result<GeneratedTheme, InvalidSeed> {
    let bg = parse_seed(Seed::Background, &seeds.background)?.over(Rgba::rgb(1.0, 1.0, 1.0));
    let accent_seed = parse_seed(Seed::Accent, &seeds.accent)?.over(bg);
    let secondary_seed = match &seeds.accent_secondary {
        Some(value) => Some(parse_seed(Seed::AccentSecondary, value)?.over(bg)),
        None => None,
    };
    let dark = bg.luminance() < 0.18;
    let base = bg.oklch();
    // A neutral is tinted, never saturated: a strongly colored page still
    // gets cards and text that read as greys of its hue.
    let neutral = |l: f64, max_chroma: f64| base.with(l, base.c.min(max_chroma)).rgba();
    let (bg_secondary, card, surface, control, border);
    let (text, text_secondary, tertiary_start);
    if dark {
        // Each elevation steps lighter, as Material's dark surfaces do.
        bg_secondary = neutral(base.l - 0.03, 0.05);
        card = neutral(base.l + 0.055, 0.05);
        surface = neutral(base.l + 0.085, 0.05);
        control = neutral(base.l + 0.125, 0.05);
        border = neutral(base.l + 0.18, 0.05);
        text = base.with(0.97, base.c.min(0.012));
        text_secondary = base.with(0.84, base.c.min(0.03));
        tertiary_start = base.with(0.72, base.c.min(0.035));
    } else {
        bg_secondary = neutral(base.l - 0.03, 0.04);
        card = neutral((base.l + 0.035).min(0.995), 0.03);
        surface = neutral((base.l + 0.025).min(0.99), 0.03);
        control = neutral(base.l - 0.04, 0.04);
        border = neutral(base.l - 0.14, 0.05);
        text = base.with(0.22, base.c.min(0.03));
        text_secondary = base.with(0.43, base.c.min(0.04));
        tertiary_start = base.with(0.5, base.c.min(0.04));
    }
    let surfaces = [bg, bg_secondary, card, surface];
    let step = if dark { 0.01 } else { -0.01 };
    let text = readable(text, &surfaces, TEXT_CONTRAST, step);
    let text_secondary = readable(text_secondary, &surfaces, TEXT_CONTRAST, step);
    let text_tertiary = readable(tertiary_start, &surfaces, TEXT_CONTRAST, step);

    let mut adjustments = Vec::new();
    // The accent is link and selected text on cards as well as a fill.
    let accent = readable(accent_seed.oklch(), &surfaces, TEXT_CONTRAST, step).rgba();
    if accent.hex() != accent_seed.hex() {
        adjustments.push(Adjustment {
            seed: Seed::Accent,
            from: accent_seed.hex(),
            to: accent.hex(),
            lightened: dark,
        });
    }
    let accent_secondary = secondary_seed.unwrap_or(accent);
    // Text on the accent fill: whichever of a near-white or a near-black of
    // the accent's own hue reads better.
    // Pure white or black when the tinted pair falls short on a mid-tone.
    let on_light = accent.oklch().with(0.99, 0.01).rgba();
    let on_dark = accent.oklch().with(0.18, 0.05).rgba();
    let lighter = contrast(on_light, accent) >= contrast(on_dark, accent);
    let on_accent = match (
        lighter,
        contrast(on_light, accent).max(contrast(on_dark, accent)),
    ) {
        (true, ratio) if ratio >= TEXT_CONTRAST => on_light,
        (false, ratio) if ratio >= TEXT_CONTRAST => on_dark,
        (true, _) => Rgba::rgb(1.0, 1.0, 1.0),
        (false, _) => Rgba::rgb(0.0, 0.0, 0.0),
    };

    let defaults = if dark {
        Theme::default_dark()
    } else {
        Theme::default_light()
    };
    let status = |value: &str| {
        let seed = Rgba::parse(value).unwrap_or(accent);
        readable(seed.oklch(), &surfaces, TEXT_CONTRAST, step)
            .rgba()
            .hex()
    };
    let shadow = if dark {
        "rgba(0, 0, 0, 0.45)".to_string()
    } else {
        let tint = base.with(0.3, base.c.min(0.08)).rgba();
        format!(
            "rgba({}, {}, {}, 0.16)",
            (tint.r * 255.0).round(),
            (tint.g * 255.0).round(),
            (tint.b * 255.0).round()
        )
    };
    let theme = Theme {
        accent: accent.hex(),
        accent_secondary: accent_secondary.hex(),
        on_accent: on_accent.hex(),
        text: text.rgba().hex(),
        text_secondary: text_secondary.rgba().hex(),
        text_tertiary: text_tertiary.rgba().hex(),
        bg: bg.hex(),
        bg_secondary: bg_secondary.hex(),
        card: card.hex(),
        surface: surface.hex(),
        control: control.hex(),
        border: border.hex(),
        shadow,
        success: status(&defaults.success),
        warning: defaults.warning.clone(),
        on_warning: defaults.on_warning.clone(),
        danger: status(&defaults.danger),
        color_scheme: if dark { "dark" } else { "light" }.into(),
        ..defaults
    };
    Ok(GeneratedTheme { theme, adjustments })
}
