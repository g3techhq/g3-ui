//! Platform mode, colour theme, and UI strings.
//!
//! - [`ComponentMode`] picks the platform look (iOS or Material Design).
//! - [`Theme`] holds the colour tokens, written to the page as `--g3-color-*`
//!   custom properties.
//! - [`Strings`] holds every piece of text a component renders on its own, so
//!   an app can translate them.
//!
//! [`AppWrapper`](crate::AppWrapper) and [`ThemeProvider`] provide all three to
//! the components inside them.
use crate::state::use_synced_signal;
use cfg_if::cfg_if;
use dioxus::prelude::*;
use std::cell::Cell;

/// Platform styling mode, like Ionic's `mode` attribute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum ComponentMode {
    /// Material Design: square corners, elevation shadows, uppercase labels.
    #[default]
    Md,
    /// iOS: rounded corners, hairline borders, translucent surfaces.
    Ios,
}

impl ComponentMode {
    /// The `data-g3-mode` attribute value the stylesheet keys on.
    pub fn as_str(self) -> &'static str {
        match self {
            ComponentMode::Md => "md",
            ComponentMode::Ios => "ios",
        }
    }

    /// Pick one of two values by mode. Used for mode-specific class names.
    pub(crate) fn pick<T>(self, ios: T, md: T) -> T {
        match self {
            ComponentMode::Ios => ios,
            ComponentMode::Md => md,
        }
    }
}

/// Colour tokens for g3-ui components.
///
/// Each field is any CSS colour value and becomes a `--g3-color-*` custom
/// property: `accent` becomes `--g3-color-accent`, `text_secondary` becomes
/// `--g3-color-text-secondary`, and so on. Start from a preset and change the
/// fields you need:
///
/// ```
/// use g3_ui::Theme;
///
/// let brand = Theme::default_light().with_accent("#1f7a4d");
/// let night = Theme { bg: "#0b1020".into(), ..Theme::default_dark() };
/// # let _ = (brand, night);
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Theme {
    /// Brand colour: primary buttons, selected controls, focus rings, links.
    /// Most tints in the stylesheet are mixed from it.
    pub accent: String,
    /// Text and icons drawn on an `accent` fill.
    pub on_accent: String,
    /// Body text.
    pub text: String,
    /// De-emphasised text: descriptions, captions, timestamps.
    pub text_secondary: String,
    /// Least emphasis: placeholders, field labels, chevrons, inactive tabs.
    pub text_tertiary: String,
    /// The page background behind everything else.
    pub bg: String,
    /// A recessed background, such as list section headers.
    pub bg_secondary: String,
    /// Cards, list rows, headers, and tab bars.
    pub card: String,
    /// Raised surfaces that float above the page: dialogs and popovers.
    pub surface: String,
    /// Interactive fills: chips, avatars, unselected controls.
    pub control: String,
    /// Hairline borders and separators.
    pub border: String,
    /// Shadow colour, including its alpha.
    pub shadow: String,
    /// Success states and confirmations.
    pub success: String,
    /// States that need attention.
    pub warning: String,
    /// Text and icons drawn on a `warning` fill. Warning is a light colour,
    /// so this is usually dark.
    pub on_warning: String,
    /// Errors and destructive actions.
    pub danger: String,
    /// The CSS `color-scheme`: `"light"`, `"dark"`, or `"light dark"`. Native
    /// form controls and scrollbars follow it.
    pub color_scheme: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self::default_light()
    }
}

impl Theme {
    /// The built-in light theme. Page, cards, and controls are distinct greys
    /// rather than all white, so each elevation reads as its own surface.
    pub fn default_light() -> Self {
        Self {
            accent: "#0066d6".into(),
            on_accent: "#ffffff".into(),
            text: "#111827".into(),
            text_secondary: "#555c68".into(),
            text_tertiary: "#5f6673".into(),
            bg: "#e9ebef".into(),
            bg_secondary: "#dfe2e8".into(),
            card: "#f7f8fa".into(),
            surface: "#f2f4f7".into(),
            control: "#e3e6eb".into(),
            border: "#c5cad3".into(),
            shadow: "rgba(15, 23, 42, 0.14)".into(),
            success: "#067647".into(),
            warning: "#f5b400".into(),
            on_warning: "#1f1a00".into(),
            danger: "#d92d20".into(),
            color_scheme: "light".into(),
        }
    }

    /// The built-in dark theme, tuned separately from the light one rather
    /// than inverted.
    pub fn default_dark() -> Self {
        Self {
            // Light enough for 4.5:1 as text on cards, so fills carry dark text.
            accent: "#4ea3ff".into(),
            on_accent: "#04162b".into(),
            text: "#ffffff".into(),
            text_secondary: "#d1d5db".into(),
            text_tertiary: "#a1a1aa".into(),
            bg: "#111827".into(),
            bg_secondary: "#0f172a".into(),
            card: "#1f2937".into(),
            surface: "#1f2937".into(),
            control: "rgba(255, 255, 255, 0.12)".into(),
            border: "rgba(255, 255, 255, 0.14)".into(),
            shadow: "rgba(0, 0, 0, 0.35)".into(),
            success: "#30d158".into(),
            warning: "#ffd60a".into(),
            on_warning: "#1f1a00".into(),
            danger: "#ff6b61".into(),
            color_scheme: "dark".into(),
        }
    }

    /// A theme that follows the system light/dark setting, using CSS
    /// `light-dark()`. Every token switches with no re-render.
    ///
    /// ```
    /// use g3_ui::Theme;
    ///
    /// let theme = Theme::adaptive(Theme::default_light(), Theme::default_dark());
    /// assert_eq!(theme.color_scheme, "light dark");
    /// ```
    pub fn adaptive(light: Theme, dark: Theme) -> Self {
        let pair = |l: String, d: String| {
            if l == d {
                l
            } else {
                format!("light-dark({l}, {d})")
            }
        };
        Self {
            accent: pair(light.accent, dark.accent),
            on_accent: pair(light.on_accent, dark.on_accent),
            text: pair(light.text, dark.text),
            text_secondary: pair(light.text_secondary, dark.text_secondary),
            text_tertiary: pair(light.text_tertiary, dark.text_tertiary),
            bg: pair(light.bg, dark.bg),
            bg_secondary: pair(light.bg_secondary, dark.bg_secondary),
            card: pair(light.card, dark.card),
            surface: pair(light.surface, dark.surface),
            control: pair(light.control, dark.control),
            border: pair(light.border, dark.border),
            shadow: pair(light.shadow, dark.shadow),
            success: pair(light.success, dark.success),
            warning: pair(light.warning, dark.warning),
            on_warning: pair(light.on_warning, dark.on_warning),
            danger: pair(light.danger, dark.danger),
            color_scheme: "light dark".into(),
        }
    }

    /// The built-in presets, following the system light/dark setting.
    pub fn system() -> Self {
        Self::adaptive(Self::default_light(), Self::default_dark())
    }

    /// Return this theme with a different accent colour.
    pub fn with_accent(mut self, accent: impl Into<String>) -> Self {
        self.accent = accent.into();
        self
    }

    /// Every token as `(custom property, value)` pairs, in declaration order.
    pub fn tokens(&self) -> [(&'static str, &str); 16] {
        [
            ("--g3-color-accent", &self.accent),
            ("--g3-color-on-accent", &self.on_accent),
            ("--g3-color-text", &self.text),
            ("--g3-color-text-secondary", &self.text_secondary),
            ("--g3-color-text-tertiary", &self.text_tertiary),
            ("--g3-color-bg", &self.bg),
            ("--g3-color-bg-secondary", &self.bg_secondary),
            ("--g3-color-card", &self.card),
            ("--g3-color-surface", &self.surface),
            ("--g3-color-control", &self.control),
            ("--g3-color-border", &self.border),
            ("--g3-color-shadow", &self.shadow),
            ("--g3-color-success", &self.success),
            ("--g3-color-warning", &self.warning),
            ("--g3-color-on-warning", &self.on_warning),
            ("--g3-color-danger", &self.danger),
        ]
    }

    /// Every token as CSS declarations for an inline `style` attribute.
    pub fn to_style_attr(&self) -> String {
        let mut style = String::new();
        for (name, value) in self.tokens() {
            style.push_str(name);
            style.push_str(": ");
            style.push_str(value);
            style.push_str("; ");
        }
        style.push_str("color-scheme: ");
        style.push_str(&self.color_scheme);
        style.push(';');
        style
    }
}

/// Text that components render on their own: accessible names, button labels,
/// status messages. English by default; provide a translated set through
/// [`AppWrapper`](crate::AppWrapper) or [`ThemeProvider`].
///
/// ```
/// use g3_ui::Strings;
///
/// let french = Strings {
///     cancel: "Annuler".into(),
///     confirm: "Confirmer".into(),
///     ..Strings::default()
/// };
/// # let _ = french;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Strings {
    /// Accessible name of the controls that dismiss a sheet or dialog.
    pub close: String,
    /// Accessible name of a bottom sheet's drag handle.
    pub sheet_handle: String,
    /// Accessible name of a sheet that has no `aria_label`.
    pub sheet: String,
    /// Accessible name of a navigation drawer that has no `aria_label`.
    pub navigation_drawer: String,
    /// Accessible name of the primary navigation bar or rail.
    pub primary_navigation: String,
    /// Announced while something loads.
    pub loading: String,
    /// Shown by `Content` when a child fails to render.
    pub load_error: String,
    /// Pull-to-refresh prompt before the threshold.
    pub pull_to_refresh: String,
    /// Pull-to-refresh prompt past the threshold.
    pub release_to_refresh: String,
    /// Pull-to-refresh status while refreshing.
    pub refreshing: String,
    /// Cancel buttons.
    pub cancel: String,
    /// Confirm buttons.
    pub confirm: String,
    /// Accessible name of a toast's close button.
    pub dismiss: String,
    /// Accessible name of `InfoButton`.
    pub more_information: String,
    /// `Select` trigger text when nothing is selected.
    pub select_placeholder: String,
    /// Accessible name of clear buttons in inputs and search bars.
    pub clear: String,
    /// `BackButton` label.
    pub back: String,
    /// `Searchbar` placeholder.
    pub search: String,
    /// Accessible name of a swipe row's keyboard "show actions" control.
    pub show_actions: String,
    /// Accessible name of a reorder handle that was not given one.
    pub reorder: String,
    /// Read after a link that opens in a new tab.
    pub opens_in_new_tab: String,
    /// Month names, January first.
    pub months: Vec<String>,
    /// Abbreviated month names, January first.
    pub months_short: Vec<String>,
    /// Weekday names, Sunday first.
    pub weekdays: Vec<String>,
    /// One- or two-letter weekday headings for a calendar, Sunday first.
    pub weekdays_narrow: Vec<String>,
    /// The day a calendar week starts on: 0 for Sunday, 1 for Monday.
    pub first_weekday: u8,
    /// The morning half of a 12-hour clock.
    pub am: String,
    /// The afternoon half of a 12-hour clock.
    pub pm: String,
    /// Accessible name of the AM/PM control.
    pub day_period: String,
    /// Button that accepts a picker's choice.
    pub done: String,
    /// Accessible name of the calendar's previous-month button.
    pub previous_month: String,
    /// Accessible name of the calendar's next-month button.
    pub next_month: String,
    /// Title of the date picker dialog, and its placeholder.
    pub choose_date: String,
    /// Title of the time picker dialog, and its placeholder.
    pub choose_time: String,
    /// Accessible name of an hour column or dial.
    pub hour: String,
    /// Accessible name of a minute column or dial.
    pub minute: String,
    /// Accessible name of a day column.
    pub day: String,
    /// Accessible name of a month column.
    pub month: String,
    /// Accessible name of a year column, and of the calendar's year list.
    pub year: String,
    /// Accessible name of the button that switches a time picker to typing.
    pub type_time: String,
    /// Accessible name of the button that switches a time picker to its dial.
    pub pick_time_on_dial: String,
}

impl Default for Strings {
    fn default() -> Self {
        Self {
            close: "Close".into(),
            sheet_handle: "Drag to close".into(),
            sheet: "Sheet".into(),
            navigation_drawer: "Menu".into(),
            primary_navigation: "Primary navigation".into(),
            loading: "Loading".into(),
            load_error: "Something went wrong. Please try again.".into(),
            pull_to_refresh: "Pull to refresh".into(),
            release_to_refresh: "Release to refresh".into(),
            refreshing: "Refreshing".into(),
            cancel: "Cancel".into(),
            confirm: "Confirm".into(),
            dismiss: "Dismiss".into(),
            more_information: "More information".into(),
            select_placeholder: "Select".into(),
            clear: "Clear".into(),
            back: "Back".into(),
            search: "Search".into(),
            show_actions: "Show actions".into(),
            reorder: "Reorder".into(),
            opens_in_new_tab: "(opens in a new tab)".into(),
            months: [
                "January",
                "February",
                "March",
                "April",
                "May",
                "June",
                "July",
                "August",
                "September",
                "October",
                "November",
                "December",
            ]
            .map(String::from)
            .to_vec(),
            months_short: [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]
            .map(String::from)
            .to_vec(),
            weekdays: [
                "Sunday",
                "Monday",
                "Tuesday",
                "Wednesday",
                "Thursday",
                "Friday",
                "Saturday",
            ]
            .map(String::from)
            .to_vec(),
            weekdays_narrow: ["S", "M", "T", "W", "T", "F", "S"]
                .map(String::from)
                .to_vec(),
            first_weekday: 0,
            am: "AM".into(),
            pm: "PM".into(),
            day_period: "AM or PM".into(),
            done: "Done".into(),
            previous_month: "Previous month".into(),
            next_month: "Next month".into(),
            choose_date: "Select date".into(),
            choose_time: "Select time".into(),
            hour: "Hour".into(),
            minute: "Minute".into(),
            day: "Day".into(),
            month: "Month".into(),
            year: "Year".into(),
            type_time: "Type a time".into(),
            pick_time_on_dial: "Pick a time on the clock".into(),
        }
    }
}

/// Mode, theme, and strings as reactive context. Signals rather than plain
/// values, because reading context is not a subscription: a component whose
/// props did not change would otherwise never see a new mode or theme.
#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Ambient {
    pub mode: Signal<ComponentMode>,
    pub theme: Signal<Theme>,
    pub strings: Signal<Strings>,
}

/// Resolve and publish what a provider hands down: its own props first, then
/// an enclosing provider's values, then the defaults.
///
/// The enclosing provider is looked up once, in `use_hook`:
/// `try_consume_context` searches the current scope first, so on later renders
/// a provider would find its own value instead.
pub(crate) fn use_provide_ambient(
    mode: Option<ComponentMode>,
    theme: Option<Theme>,
    strings: Option<Strings>,
) -> (ComponentMode, Theme) {
    let parent = use_hook(try_consume_context::<Ambient>);
    let mode = mode
        .or_else(|| parent.map(|p| (p.mode)()))
        .unwrap_or_else(get_mode);
    let theme = theme
        .or_else(|| parent.map(|p| (p.theme)()))
        .unwrap_or_default();
    let strings = strings
        .or_else(|| parent.map(|p| (p.strings)()))
        .unwrap_or_default();
    let ambient = Ambient {
        mode: use_synced_signal(mode),
        theme: use_synced_signal(theme.clone()),
        strings: use_synced_signal(strings),
    };
    use_hook(|| provide_context(ambient));
    (mode, theme)
}

fn ambient() -> Option<Ambient> {
    try_consume_context::<Ambient>()
}

/// The theme in effect here, or [`Theme::default`] outside any provider.
/// Subscribes the calling component to theme changes.
pub fn use_theme() -> Theme {
    ambient().map(|a| (a.theme)()).unwrap_or_default()
}

/// The strings in effect here, or [`Strings::default`] outside any provider.
/// Subscribes the calling component to changes.
pub fn use_strings() -> Strings {
    ambient().map(|a| (a.strings)()).unwrap_or_default()
}

/// Resolve a component's mode: its own `mode` prop, then the nearest provider,
/// then the global mode. Subscribes the calling component to mode changes.
pub fn use_component_mode(mode: Option<ComponentMode>) -> ComponentMode {
    mode.or_else(|| ambient().map(|a| (a.mode)()))
        .unwrap_or_else(get_mode)
}

thread_local! {
    static GLOBAL_MODE: Cell<Option<ComponentMode>> = const { Cell::new(None) };
}

/// Set the mode used by components with no `mode` prop and no provider.
pub fn set_mode(mode: ComponentMode) {
    GLOBAL_MODE.with(|m| m.set(Some(mode)));
}

/// The global mode: whatever [`set_mode`] set, or else [`detect_platform_mode`].
pub fn get_mode() -> ComponentMode {
    GLOBAL_MODE
        .with(|m| m.get())
        .unwrap_or_else(detect_platform_mode)
}

/// Set the global mode to [`detect_platform_mode`].
pub fn init_auto_mode() {
    set_mode(detect_platform_mode());
}

/// The platform's native mode: iOS on iPhone and iPad (including their web
/// browsers), Material Design everywhere else.
pub fn detect_platform_mode() -> ComponentMode {
    cfg_if! {
        if #[cfg(target_os = "ios")] {
            ComponentMode::Ios
        } else if #[cfg(target_arch = "wasm32")] {
            detect_web_mode()
        } else {
            ComponentMode::Md
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn detect_web_mode() -> ComponentMode {
    let Some(window) = web_sys::window() else {
        return ComponentMode::Md;
    };
    let navigator = window.navigator();
    let user_agent = navigator.user_agent().unwrap_or_default().to_lowercase();
    let platform = navigator.platform().unwrap_or_default().to_lowercase();
    let is_iphone = user_agent.contains("iphone") || user_agent.contains("ipod");
    // iPadOS reports itself as a Mac; touch support gives it away.
    let is_ipad = user_agent.contains("ipad")
        || (platform.contains("mac")
            && navigator.max_touch_points() > 1
            && user_agent.contains("safari"));
    if is_iphone || is_ipad {
        ComponentMode::Ios
    } else {
        ComponentMode::Md
    }
}

/// Join a component's own classes with a caller's `class` prop.
pub fn merge_classes(base: impl AsRef<str>, class: Option<&str>) -> String {
    let base = base.as_ref().trim();
    let class = class.unwrap_or_default().trim();
    match (base.is_empty(), class.is_empty()) {
        (true, true) => String::new(),
        (true, false) => class.to_string(),
        (false, true) => base.to_string(),
        (false, false) => format!("{base} {class}"),
    }
}

/// Build a class list from parts, skipping empty ones.
pub(crate) fn classes<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::new();
    for part in parts.into_iter().map(str::trim).filter(|p| !p.is_empty()) {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(part);
    }
    out
}

/// Apply a mode, theme, and strings to everything inside, without the app
/// layout that [`AppWrapper`](crate::AppWrapper) adds.
///
/// Renders a `display: contents` element that carries the theme's custom
/// properties, so it adds no box of its own. Props left unset are inherited
/// from an enclosing provider.
///
/// ```
/// # use dioxus::prelude::*;
/// # use g3_ui::prelude::*;
/// # fn demo() -> Element {
/// rsx! {
///     ThemeProvider { theme: Theme::default_dark(),
///         Button { "Dark button" }
///     }
/// }
/// # }
/// ```
#[component]
pub fn ThemeProvider(
    /// Platform look for this subtree.
    mode: Option<ComponentMode>,
    /// Colour tokens for this subtree.
    theme: Option<Theme>,
    /// Component text for this subtree.
    strings: Option<Strings>,
    children: Element,
) -> Element {
    let (mode, theme) = use_provide_ambient(mode, theme, strings);
    rsx! {
        div {
            class: "g3-theme-provider",
            style: theme.to_style_attr(),
            "data-g3-mode": mode.as_str(),
            {children}
        }
    }
}
