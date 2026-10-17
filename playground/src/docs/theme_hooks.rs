use dioxus::prelude::*;
use g3_ui::prelude::*;

#[component]
pub fn ThemeHooksExample() -> Element {
    let mut status = use_signal(String::new);

    rsx! {
        AppWrapper { theme: Theme::default_light().with_accent("#7c3aed"), mode: ComponentMode::Md, strings: Strings::default(),
            Content { footer_space: false,
                Stack { gap: Space::Md,
                    AppearanceControls {}
                    ThemedAccent {}
                    LocalizedActions {
                        oncancel: move |_| status.set("Cancelled".into()),
                        onconfirm: move |_| status.set("Saved".into()),
                    }
                    ModeAwareButton {}
                    // An explicit prop overrides the ambient platform mode.
                    ModeAwareButton { mode: ComponentMode::Md }
                    p { role: "status", "{status}" }
                    ScopedThemeExample {}
                }
            }
        }
    }
}

#[component]
fn ThemeColorControls() -> Element {
    let mut theme = use_theme();
    rsx! {
        Stack { gap: Space::Sm,
            // set() replaces the provider's entire theme value.
            Button { onclick: move |_| theme.set(Theme::default_dark()),
                "Dark colors"
            }
            Button { onclick: move |_| theme.set(Theme::default_light()),
                "Light colors"
            }
            // write() changes a field while preserving the other colors.
            Button { onclick: move |_| theme.write().accent = "#16a34a".into(),
                "Green accent"
            }
        }
    }
}

#[component]
fn AppearanceControls() -> Element {
    let mut mode = use_mode();
    let mut strings = use_strings();
    rsx! {
        Stack { gap: Space::Sm,
            ThemeColorControls {}
            // Mode is independent of the current light/dark colors.
            Button { onclick: move |_| *mode.write() = ComponentMode::Ios, "Use iOS" }
            Button { onclick: move |_| mode.set(ComponentMode::Md), "Use Material" }
            Button {
                onclick: move |_| {
                    let mut labels = strings.write();
                    labels.cancel = "Cancelar".into();
                    labels.confirm = "Confirmar".into();
                },
                "Spanish labels"
            }
            Button { onclick: move |_| strings.set(Strings::default()),
                "English labels"
            }
        }
    }
}

#[component]
fn ThemedAccent() -> Element {
    let theme = use_theme();
    // Reading subscribes this component to shared theme changes.
    let accent = theme.read().accent.clone();

    rsx! {
        span { style: "color: {accent};",
            "Current accent: {accent}"
        }
    }
}

#[component]
fn LocalizedActions(
    oncancel: EventHandler<MouseEvent>,
    onconfirm: EventHandler<MouseEvent>,
) -> Element {
    let strings = use_strings();
    let labels = strings.read();

    rsx! {
        Button { fill: ButtonFill::Clear, onclick: oncancel, "{labels.cancel}" }
        Button { onclick: onconfirm, "{labels.confirm}" }
    }
}

#[component]
fn ModeAwareButton(mode: Option<ComponentMode>) -> Element {
    // None follows the nearest provider. Some(mode) takes precedence.
    let resolved_mode = use_component_mode(mode);
    let fill = match resolved_mode {
        ComponentMode::Ios => ButtonFill::Clear,
        ComponentMode::Md => ButtonFill::Outline,
    };

    rsx! {
        Button { mode: resolved_mode, fill, "Continue" }
    }
}

#[component]
fn ScopedThemeExample() -> Element {
    rsx! {
        ThemeProvider {
            theme: Theme::default_dark().with_accent("#facc15"),
            mode: ComponentMode::Ios,
            Card { title: "Local override",
                ThemedAccent {}
                LocalThemeEditor {}
                ModeAwareButton {}
            }
        }
    }
}

#[component]
fn LocalThemeEditor() -> Element {
    let mut theme = use_theme();
    rsx! {
        Button { onclick: move |_| theme.write().accent = "#f97316".into(),
            "Change local accent"
        }
    }
}
