//! The head of a desktop or mobile app's first page.

use crate::{Theme, UI_CSS};
use manganis::Asset;
use std::fmt;

/// The head of a desktop or mobile app's first page, for Dioxus's
/// `Config::with_custom_head`.
///
/// A WebView paints its first page before the app's first render reaches it.
/// On the web the build links the stylesheets in that page's head
/// (`with_static_head`), but desktop and mobile builds serve their own page and
/// leave static-head assets out of it, so every stylesheet arrived with the
/// first render and the app showed unstyled until they loaded, longest on a
/// slow phone. This head links them from the start. Stylesheets linked in the
/// head block first paint, and Dioxus asks for the first render only once the
/// page has loaded them.
///
/// It links [`UI_CSS`], then (with the `transitions` feature) the
/// route-transition stylesheet, then the app's own stylesheets and styles in
/// the order they were added. Given a [`Theme`], it paints that theme's page
/// background and text color until the [`AppWrapper`](crate::AppWrapper)
/// mounts, so the first page is not white in a dark app; pass the theme the
/// app renders first.
///
/// Keep linking every stylesheet at runtime too: desktop and mobile bundles
/// only collect assets that something links at runtime. `AppWrapper` already
/// links its own.
///
/// Styles and theme values are written into the page without escaping, like
/// [`Theme`]'s (see its notes): use only the app's own.
///
/// ```
/// # use dioxus::prelude::*;
/// # fn app() -> Element { rsx! {} }
/// const APP_CSS: Asset = asset!("/assets/g3-ui.css");
///
/// let head = g3_ui::NativeHead::new()
///     .theme(&g3_ui::Theme::system())
///     .stylesheet(APP_CSS)
///     .to_string();
/// assert!(head.contains(r#"<link rel="stylesheet""#));
/// // In `main`, on desktop and mobile:
/// // dioxus::LaunchBuilder::new()
/// //     .with_cfg(dioxus::mobile::Config::new().with_custom_head(head))
/// //     .launch(app);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NativeHead {
    stylesheets: Vec<String>,
    styles: Vec<String>,
    theme: Option<Theme>,
}

impl NativeHead {
    /// A head with g3-ui's own stylesheets.
    pub fn new() -> Self {
        Self::default()
    }

    /// Links one of the app's stylesheets, after the ones added before it.
    pub fn stylesheet(mut self, stylesheet: Asset) -> Self {
        self.stylesheets.push(stylesheet.to_string());
        self
    }

    /// Adds CSS of the app's own inline, for an app that keeps its styles in
    /// Rust strings rather than stylesheet assets. It applies after every
    /// linked stylesheet.
    pub fn style(mut self, css: impl Into<String>) -> Self {
        self.styles.push(css.into());
        self
    }

    /// Paints `theme`'s page background and text color until the
    /// `AppWrapper` mounts. An adaptive theme ([`Theme::adaptive`]) follows
    /// the page's `prefers-color-scheme`. An Android WebView reports "light"
    /// there whatever the phone is set to (g3-native-plugins' `appearance`
    /// asks the system, but only once the app is running), so on Android an
    /// adaptive theme's first page is the light one.
    pub fn theme(mut self, theme: &Theme) -> Self {
        self.theme = Some(theme.clone());
        self
    }
}

impl fmt::Display for NativeHead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let library = [
            Some(UI_CSS.to_string()),
            #[cfg(feature = "transitions")]
            Some(g3_route_transitions::ROUTE_TRANSITIONS_CSS.to_string()),
        ];
        for href in library.iter().flatten().chain(&self.stylesheets) {
            write!(f, r#"<link rel="stylesheet" href="{href}">"#)?;
        }
        if self.theme.is_none() && self.styles.is_empty() {
            return Ok(());
        }
        f.write_str("<style>")?;
        if let Some(theme) = &self.theme {
            // Only until the shell mounts: the shell then paints the page
            // itself, in whatever theme is current by then, and a rule left
            // on the root would keep the first theme behind it.
            write!(
                f,
                "html:not(:has(.g3-app)) {{ {} background: var(--g3-page-background, var(--g3-color-bg)); color: var(--g3-color-text); }}",
                theme.to_style_attr(),
            )?;
        }
        for css in &self.styles {
            f.write_str(css)?;
        }
        f.write_str("</style>")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_the_library_stylesheets_first() {
        let head = NativeHead::new().to_string();
        let ui = format!(r#"<link rel="stylesheet" href="{UI_CSS}">"#);
        assert!(head.starts_with(&ui), "{head}");
        #[cfg(feature = "transitions")]
        assert!(
            head.contains(&format!(
                r#"<link rel="stylesheet" href="{}">"#,
                g3_route_transitions::ROUTE_TRANSITIONS_CSS
            )),
            "{head}"
        );
        assert!(!head.contains("<style>"), "{head}");
    }

    #[test]
    fn app_stylesheets_and_styles_follow_in_order() {
        let head = NativeHead::new()
            .stylesheet(UI_CSS)
            .style(".a {}")
            .style(".b {}")
            .to_string();
        let library_end = head.find('>').unwrap() + 1;
        assert!(head[library_end..].contains("<link"), "{head}");
        let a = head.find(".a {}").unwrap();
        let b = head.find(".b {}").unwrap();
        assert!(head.rfind("<link").unwrap() < a && a < b, "{head}");
        assert!(head.ends_with("</style>"), "{head}");
    }

    #[test]
    fn a_theme_paints_the_page_only_until_the_shell_mounts() {
        let head = NativeHead::new().theme(&Theme::default_dark()).to_string();
        assert!(head.contains("html:not(:has(.g3-app)) {"), "{head}");
        assert!(head.contains("--g3-color-bg: #111827;"), "{head}");
        assert!(head.contains("color-scheme: dark;"), "{head}");
        assert!(
            head.contains("background: var(--g3-page-background, var(--g3-color-bg));"),
            "{head}"
        );
    }

    #[test]
    fn an_adaptive_theme_follows_the_system() {
        let head = NativeHead::new().theme(&Theme::system()).to_string();
        assert!(head.contains("light-dark(#e9ebef, #111827)"), "{head}");
        assert!(head.contains("color-scheme: light dark;"), "{head}");
    }
}
