//! An editable theme, shared with every playground route, and its Rust export.
use super::*;

#[derive(Clone, Copy, PartialEq)]
enum GeneratorView {
    Colors,
    Preview,
    Code,
}

impl GeneratorView {
    fn as_str(self) -> &'static str {
        match self {
            Self::Colors => "colors",
            Self::Preview => "preview",
            Self::Code => "code",
        }
    }
}

macro_rules! color_fields {
    ($($field:ident => $label:literal),+ $(,)?) => {
        #[derive(Clone, Copy)]
        struct ColorField {
            key: &'static str,
            label: &'static str,
            get: fn(&G3Theme) -> &str,
            set: fn(&mut G3Theme, String),
        }
        impl PartialEq for ColorField {
            fn eq(&self, other: &Self) -> bool { self.key == other.key }
        }
        const COLORS: &[ColorField] = &[$(ColorField {
            key: stringify!($field), label: $label,
            get: |theme| &theme.$field,
            set: |theme, value| theme.$field = value,
        }),+];
    };
}

color_fields! {
    accent => "Accent", accent_secondary => "Secondary accent", on_accent => "On accent",
    text => "Text", text_secondary => "Secondary text", text_tertiary => "Tertiary text",
    bg => "Page background", bg_secondary => "Recessed background", card => "Card",
    surface => "Raised surface", control => "Control", border => "Border", shadow => "Shadow",
    success => "Success", warning => "Warning", on_warning => "On warning", danger => "Danger",
}

macro_rules! look_fields {
    ($($field:ident => $label:literal),+ $(,)?) => {
        #[derive(Clone, Copy)]
        struct LookField {
            key: &'static str,
            label: &'static str,
            get: fn(&Look) -> &Option<String>,
            set: fn(&mut Look, Option<String>),
        }
        impl PartialEq for LookField {
            fn eq(&self, other: &Self) -> bool { self.key == other.key }
        }
        const LOOKS: &[LookField] = &[$(LookField {
            key: stringify!($field), label: $label,
            get: |look| &look.$field,
            set: |look, value| look.$field = value,
        }),+];
    };
}

look_fields! {
    background => "Page gradient", accent_fill => "Accent fill", card_radius => "Card radius",
    card_border => "Card border", card_shadow => "Card shadow", card_sheen => "Card sheen",
    card_backdrop_filter => "Card backdrop filter",
}

fn rust_source(theme: &G3Theme) -> String {
    let mut source = "use g3_ui::{Look, Theme};\n\nlet theme = Theme {\n".to_string();
    for field in COLORS {
        source.push_str(&format!(
            "    {}: {:?}.into(),\n",
            field.key,
            (field.get)(theme)
        ));
    }
    source.push_str(&format!(
        "    color_scheme: {:?}.into(),\n    look: Look {{\n",
        theme.color_scheme
    ));
    for field in LOOKS {
        let value = match (field.get)(&theme.look) {
            Some(value) => format!("Some({value:?}.into())"),
            None => "None".into(),
        };
        source.push_str(&format!("        {}: {value},\n", field.key));
    }
    source.push_str(
        "    },\n    look_dark: None,\n};\n\n// Pass theme to AppWrapper { theme, ... }\n",
    );
    source
}

// Native color inputs require six-digit hex. The adjacent text field preserves
// arbitrary CSS colors and alpha; the picker replaces the value with solid RGB.
fn picker_color(value: &str) -> String {
    if value.len() == 7
        && value.starts_with('#')
        && value[1..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return value.to_string();
    }
    if value.len() == 4
        && value.starts_with('#')
        && value[1..].bytes().all(|b| b.is_ascii_hexdigit())
    {
        return format!(
            "#{}",
            value[1..].chars().flat_map(|c| [c, c]).collect::<String>()
        );
    }
    if let Some(rgb) = value
        .strip_prefix("rgba(")
        .or_else(|| value.strip_prefix("rgb("))
    {
        let channels = rgb
            .trim_end_matches(')')
            .split(',')
            .take(3)
            .map(|s| s.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>();
        if let Ok(rgb) = channels
            && rgb.len() == 3
        {
            return format!("#{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
        }
    }
    "#000000".into()
}

fn update_color(settings: PlaygroundSettings, field: ColorField, value: String) {
    let mut custom = settings.custom_theme;
    (field.set)(&mut custom.write(), value);
}

/// The native color picker, as an `Input`'s end slot: g3-ui has no color
/// field of its own.
#[component]
fn ColorSwatch(value: String, label: String, onpick: EventHandler<String>) -> Element {
    rsx! {
        input { class: "theme-color-swatch", r#type: "color", value: picker_color(&value),
            aria_label: label,
            oninput: move |event| onpick.call(event.value()),
        }
    }
}

/// Text for a field that edits part of the custom theme: `read` picks that
/// part out, and the field follows it when it changes elsewhere, such as a
/// preset being loaded. The effect reads the theme signal itself, so it
/// reruns on every change to it.
fn use_followed_text(read: impl Fn() -> String + 'static) -> Signal<String> {
    let mut text = use_signal(&read);
    use_effect(move || {
        let value = read();
        if *text.peek() != value {
            text.set(value);
        }
    });
    text
}

#[component]
fn ColorEditor(field: ColorField) -> Element {
    let settings = use_context::<PlaygroundSettings>();
    let value = (field.get)(&(settings.custom_theme)()).to_string();
    let text = use_followed_text(move || (field.get)(&(settings.custom_theme)()).to_string());
    rsx! {
        g3_ui::Input { label: field.label, value: text, aria_label: format!("{} CSS color", field.label),
            oninput: move |value| update_color(settings, field, value),
            end: rsx! {
                ColorSwatch { value, label: format!("Pick {} color", field.label),
                    onpick: move |value| update_color(settings, field, value),
                }
            },
        }
    }
}

#[component]
fn LookEditor(field: LookField) -> Element {
    let settings = use_context::<PlaygroundSettings>();
    let text = use_followed_text(move || {
        (field.get)(&(settings.custom_theme)().look)
            .clone()
            .unwrap_or_default()
    });
    rsx! {
        g3_ui::Input { label: field.label, value: text, placeholder: "Platform default",
            oninput: move |value: String| {
                let mut custom = settings.custom_theme;
                (field.set)(&mut custom.write().look, (!value.trim().is_empty()).then_some(value));
            },
        }
    }
}

/// The presets as swatches; picking one loads its values into the editor.
#[component]
fn PresetPicker() -> Element {
    let settings = use_context::<PlaygroundSettings>();
    let custom = (settings.custom_theme)();
    rsx! {
        section { class: "theme-generator-section", aria_labelledby: "theme-presets-heading",
            h2 { id: "theme-presets-heading", "Presets" }
            p { "Load a preset into the editor to use or copy its values." }
            div { class: "theme-preset-grid",
                for preset in PlaygroundTheme::presets() {
                    PresetSwatch { key: "{preset.label()}", preset, loaded: preset.to_theme(&custom) == custom }
                }
            }
        }
    }
}

#[component]
fn PresetSwatch(preset: PlaygroundTheme, loaded: bool) -> Element {
    let mut settings = use_context::<PlaygroundSettings>();
    let theme = preset.to_theme(&G3Theme::default_light());
    // The swatch is drawn in the preset's own colors, so it previews it.
    let page = format!(
        "background: {}; background-color: {};",
        theme.look.background.as_deref().unwrap_or("none"),
        theme.bg
    );
    let card = format!(
        "background: {}; border-color: {};",
        theme.card, theme.border
    );
    let text = format!("background: {};", theme.text);
    let text_secondary = format!("background: {};", theme.text_secondary);
    let chip = format!(
        "--g3-color-accent: {}; --g3-color-accent-secondary: {}; background: {};",
        theme.accent,
        theme.accent_secondary,
        theme
            .look
            .accent_fill
            .as_deref()
            .unwrap_or("var(--g3-color-accent)")
    );
    let label = preset.label();
    rsx! {
        button { class: "theme-preset", r#type: "button",
            aria_label: format!("Load the {label} preset"),
            aria_pressed: loaded,
            onclick: move |_| settings.custom_theme.set(preset.to_theme(&G3Theme::default_light())),
            span { class: "theme-preset-swatch", aria_hidden: "true", style: page,
                span { class: "theme-preset-card", style: card,
                    span { class: "theme-preset-line", style: text }
                    span { class: "theme-preset-line theme-preset-line-short", style: text_secondary }
                    span { class: "theme-preset-chip", style: chip }
                }
            }
            span { class: "theme-preset-name", "{label}" }
        }
    }
}

#[component]
fn SeedField(label: &'static str, value: Signal<String>) -> Element {
    let mut value = value;
    rsx! {
        g3_ui::Input { label, value, aria_label: format!("{label} seed, CSS color"),
            end: rsx! {
                ColorSwatch { value: value(), label: format!("Pick {label} seed color"),
                    onpick: move |picked| value.set(picked),
                }
            },
        }
    }
}

/// A page background, an accent and an optional second accent, filled out
/// into a whole theme by `g3_ui::ThemeSeeds`.
#[component]
fn SeedGenerator() -> Element {
    let mut settings = use_context::<PlaygroundSettings>();
    let accent = use_signal(|| picker_color(&(settings.custom_theme).peek().accent));
    let background = use_signal(|| picker_color(&(settings.custom_theme).peek().bg));
    let secondary = use_signal(|| picker_color(&(settings.custom_theme).peek().accent_secondary));
    let use_secondary = use_signal(|| false);
    let mut status = use_signal(String::new);
    rsx! {
        section { class: "theme-generator-section", aria_labelledby: "theme-seeds-heading",
            h2 { id: "theme-seeds-heading", "Generate from your colors" }
            p { "Pick a page background, then an accent. The background's lightness chooses light or dark, its hue tints the surfaces, and every text color is set to read at 4.5:1. Generating replaces the colors below and keeps the surface styles." }
            SeedField { label: "Page background", value: background }
            SeedField { label: "Accent", value: accent }
            g3_ui::Checkbox { checked: use_secondary, label: "Second accent for gradients" }
            if use_secondary() {
                SeedField { label: "Second accent", value: secondary }
            }
            div { class: "theme-generator-actions",
                Button { size: ButtonSize::Sm,
                    onclick: move |_| {
                        let mut seeds = g3_ui::ThemeSeeds::new(accent(), background());
                        if use_secondary() {
                            seeds = seeds.with_accent_secondary(secondary.cloned());
                        }
                        match seeds.generate() {
                            Ok(generated) => {
                                let look = settings.custom_theme.peek().look.clone();
                                settings.custom_theme.set(G3Theme { look, ..generated.theme });
                                status.set(if generated.adjustments.is_empty() {
                                    "Generated a theme from your colors.".into()
                                } else {
                                    let notes: Vec<String> = generated.adjustments.iter().map(ToString::to_string).collect();
                                    format!("Generated a theme. {}", notes.join(" "))
                                });
                            }
                            Err(error) => status.set(format!("Enter the {} as hex, such as #2563eb, or rgb().", error.seed)),
                        }
                    },
                    "Generate theme"
                }
            }
            p { role: "status", aria_live: "polite", "{status}" }
        }
    }
}

#[component]
pub fn ThemeGenerator() -> Element {
    let mut settings = use_context::<PlaygroundSettings>();
    let (_, _, viewport) = active_playground_settings();
    let custom = (settings.custom_theme)();
    let mut copy_status = use_signal(String::new);
    let view = use_signal(|| GeneratorView::Colors);
    use_effect(move || {
        let _ = (settings.custom_theme)();
        copy_status.set(String::new());
    });
    let color_scheme = use_followed_text(move || (settings.custom_theme)().color_scheme);
    let source = rust_source(&custom);
    let copy_source = source.clone();
    rsx! {
        article { class: "theme-generator", "data-view": view().as_str(),
            p { "Start from a preset or generate a theme from two or three colors, then edit any of the 17 colors and the optional surface styles. Changes appear in the live preview immediately. Use your theme throughout the playground to try every component." }
            div { class: "theme-generator-actions",
                Button { size: ButtonSize::Sm, onclick: move |_| settings.theme.set(PlaygroundTheme::Custom), "Use throughout playground" }
            }
            div { class: "theme-generator-view-tabs",
                SegmentGroup { value: view, aria_label: "Theme generator view",
                    SegmentButton { value: GeneratorView::Colors, "Colors" }
                    SegmentButton { value: GeneratorView::Preview, "Preview" }
                    SegmentButton { value: GeneratorView::Code, "Code" }
                }
            }
            div { class: "theme-generator-layout",
                g3_ui::Card { class: "theme-generator-editor",
                    PresetPicker {}
                    SeedGenerator {}
                    h2 { "Colors" }
                    p { "Pick a solid color, or enter a CSS color for transparency, light-dark(), and other values." }
                    for field in COLORS {
                        ColorEditor { key: "{field.key}", field: *field }
                    }
                    g3_ui::Select { label: "Color scheme", value: color_scheme,
                        options: vec![
                            SelectOption::new("light".to_string(), "Light"),
                            SelectOption::new("dark".to_string(), "Dark"),
                            SelectOption::new("light dark".to_string(), "Follow system"),
                        ],
                        onchange: move |scheme: String| settings.custom_theme.write().color_scheme = scheme,
                    }
                    g3_ui::AccordionGroup::<&'static str> {
                        g3_ui::AccordionItem { value: "surface", label: "Surface styles (optional)",
                            p { "Enter CSS values. Clear a field to restore the platform default." }
                            for field in LOOKS {
                                LookEditor { key: "{field.key}", field: *field }
                            }
                        }
                    }
                }
                div { class: "theme-generator-result",
                    section { class: "theme-generator-live",
                    h2 { "Live preview" }
                    div { class: "theme-preview-stage",
                        section { class: "playground-viewport-card playground-viewport-{viewport.as_str()}", aria_label: "Custom theme preview",
                            div { class: "playground-viewport-demo",
                                ThemeProvider { theme: custom.clone(), mode: (settings.mode)(), ThemePreview {} }
                            }
                        }
                    }
                    }
                    div { class: "theme-generator-export",
                        h2 { "Use in your app" }
                        Button { fill: ButtonFill::Outline,
                            onclick: move |_| {
                                let source = copy_source.clone();
                                spawn(async move {
                                    let mut eval = document::eval("const text = await dioxus.recv(); try { await navigator.clipboard.writeText(text); dioxus.send(true); } catch { dioxus.send(false); }");
                                    let copied = eval.send(source).is_ok() && eval.recv::<bool>().await.unwrap_or(false);
                                    copy_status.set(if copied { "Copied Rust theme." } else { "Copy unavailable. Select the code below to copy it." }.into());
                                });
                            }, "Copy Rust theme"
                        }
                        p { role: "status", aria_live: "polite", "{copy_status}" }
                        CodeBlock { src: source }
                    }
                }
            }
        }
    }
}

#[component]
fn ThemePreview() -> Element {
    let notifications = use_signal(|| true);
    let checked = use_signal(|| true);
    let name = use_signal(|| "Alex Morgan".to_string());
    let volume = use_signal(|| 65.0);
    let segment = use_signal(|| 0_u8);
    let language = use_signal(|| "English".to_string());
    let mut message = use_signal(String::new);
    rsx! {
        g3_ui::PlaygroundDemoFrame { app: false,
            AppWrapper { class: "g3-playground-device-app",
                Header { title: "Your workspace", toolbar: rsx! {
                    SegmentGroup { value: segment, aria_label: "Preview section",
                        SegmentButton { value: 0_u8, "Overview" }
                        SegmentButton { value: 1_u8, "Settings" }
                    }
                } }
                g3_ui::Content { footer_space: false,
                    g3_ui::Stack { gap: g3_ui::Space::Lg,
                        g3_ui::Card { title: "Profile", subtitle: "Theme preview",
                            g3_ui::Stack { gap: g3_ui::Space::Md,
                                g3_ui::Input { label: "Display name", value: name }
                                Select { label: "Language", value: language,
                                    options: vec![SelectOption::new("English".to_string(), "English"), SelectOption::new("Spanish".to_string(), "Spanish")],
                                }
                                g3_ui::Toggle { label: "Notifications", checked: notifications }
                                g3_ui::Checkbox { label: "Keep me updated", checked }
                                g3_ui::Range { label: "Volume", value: volume, show_value: true }
                                div { class: "theme-generator-actions",
                                    Button { onclick: move |_| message.set("Changes saved.".into()), "Save changes" }
                                    Button { fill: ButtonFill::Outline, "Secondary" }
                                    Button { disabled: true, "Disabled" }
                                }
                                p { role: "status", "{message}" }
                            }
                        }
                        g3_ui::Card { title: "Status colors",
                            div { class: "theme-generator-actions",
                                g3_ui::Badge { color: g3_ui::Color::Success, "Success" }
                                g3_ui::Badge { color: g3_ui::Color::Warning, "Warning" }
                                g3_ui::Badge { color: g3_ui::Color::Danger, "Danger" }
                            }
                            g3_ui::Progress { value: 65.0, label: "Upload progress" }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor_covers_every_public_color_token_and_exports_edited_values() {
        let mut theme = G3Theme::default_light();
        assert_eq!(COLORS.len(), theme.tokens().len());
        for (index, field) in COLORS.iter().enumerate() {
            let token = format!("--g3-color-{}", field.key.replace('_', "-"));
            assert!(theme.tokens().iter().any(|(key, _)| *key == token));
            (field.set)(&mut theme, format!("#{index:06x}"));
        }
        let exported = rust_source(&theme);
        for (index, field) in COLORS.iter().enumerate() {
            assert_eq!((field.get)(&theme), format!("#{index:06x}"));
            assert!(exported.contains(&format!("{}: \"#{index:06x}\".into()", field.key)));
        }
    }

    #[test]
    fn custom_theme_selection_preserves_every_edited_value() {
        let mut theme = G3Theme::default_dark();
        theme.accent = "#aabbcc".into();
        theme.look.card_radius = Some("2rem".into());
        assert_eq!(PlaygroundTheme::Custom.to_theme(&theme), theme);
        assert_ne!(PlaygroundTheme::Blue.to_theme(&theme), theme);
    }

    #[test]
    fn export_preserves_css_and_escapes_rust_string_literals() {
        let mut theme = G3Theme::default_dark();
        theme.border = "rgba(1, 2, 3, 0.25)".into();
        theme.look.background = Some("url(\"example\")\n, linear-gradient(red, blue)".into());
        let exported = rust_source(&theme);
        assert!(exported.contains("border: \"rgba(1, 2, 3, 0.25)\".into()"));
        assert!(
            exported
                .contains("Some(\"url(\\\"example\\\")\\n, linear-gradient(red, blue)\".into())")
        );
        assert!(exported.contains("color_scheme: \"dark\".into()"));
    }

    #[test]
    fn surface_editors_cover_all_look_properties_and_allow_reset() {
        let mut look = Look::default();
        for field in LOOKS {
            (field.set)(&mut look, Some("test".into()));
        }
        assert_eq!(look.tokens().len(), LOOKS.len());
        for field in LOOKS {
            (field.set)(&mut look, None);
        }
        assert_eq!(look, Look::default());
    }

    #[test]
    fn color_picker_accepts_preset_hex_and_alpha_colors_without_losing_text() {
        assert_eq!(picker_color("#aBcDeF"), "#aBcDeF");
        assert_eq!(picker_color("#abc"), "#aabbcc");
        assert_eq!(picker_color("rgba(37, 99, 235, 0.2)"), "#2563eb");
        assert_eq!(picker_color("light-dark(white, black)"), "#000000");
        assert_eq!(picker_color("#éa"), "#000000");
    }
}
