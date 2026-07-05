//! SettingsGroup component - grouped list of labeled rows (like iOS Settings).

use super::settings_group_styles as s;
use crate::theme::merge_classes;
use dioxus::prelude::*;
use dioxus_icons::lucide::ChevronRight;

/// A single labeled row within a SettingsGroup (no internal separator).
#[component]
fn SettingRow(
    label: String,
    value: Option<String>,
    right_slot: Option<Element>,
    class: Option<String>,
) -> Element {
    rsx! {
        div { class: merge_classes("px-4 py-3", class.as_deref()),
            div { class: "flex justify-between items-center",
                div { class: "flex flex-col min-w-0 flex-1",
                    div { class: "{s::LABEL} truncate", "{label}" }
                    if let Some(v) = value {
                        div { class: s::VALUE, "{v}" }
                    }
                }
                if let Some(right) = right_slot {
                    div { class: "shrink-0", {right} }
                }
            }
        }
    }
}

/// A container for grouped settings rows with consistent label/value styling.
/// Separators (gray lines) are placed between rows at full container width.
#[component]
pub fn SettingsGroup(
    top_margin: Option<bool>,
    inset: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let margin_class = top_margin.unwrap_or(false).then_some("mt-3").unwrap_or("");
    let inset_class = inset.unwrap_or(false).then_some(s::INSET).unwrap_or("");

    rsx! {
        div {
            class: merge_classes(
                format!("{} {margin_class} {inset_class}", s::CONTAINER),
                class.as_deref(),
            ),
            {children}
        }
    }
}

/// Convenience component: a setting row with a label, optional text value, and optional right slot.
/// The `value` prop is optional - when `right_slot` is provided and `value` is not set,
/// only the right slot is shown (e.g., a Select dropdown without duplicate text).
#[component]
pub fn Setting(
    label: String,
    value: Option<String>,
    right_slot: Option<Element>,
    class: Option<String>,
) -> Element {
    rsx! {
        SettingRow { label, value, right_slot, class }
    }
}

/// A full-width settings row that performs an in-app action.
#[component]
pub fn SettingAction(
    label: String,
    value: Option<String>,
    destructive: Option<bool>,
    disabled: Option<bool>,
    class: Option<String>,
    onclick: Callback<Event<MouseData>>,
) -> Element {
    let is_destructive = destructive.unwrap_or(false);
    let is_disabled = disabled.unwrap_or(false);
    let label_class = if is_destructive {
        format!("{} text-red-600", s::LABEL)
    } else {
        s::LABEL.to_string()
    };

    rsx! {
        button {
            class: merge_classes(
                "w-full px-4 py-3 text-left bg-transparent border-0 disabled:opacity-50 disabled:cursor-not-allowed",
                class.as_deref(),
            ),
            r#type: "button",
            disabled: is_disabled,
            onclick,
            div { class: "flex justify-between items-center gap-3",
                div { class: "flex flex-col min-w-0 flex-1",
                    div { class: "{label_class} truncate", "{label}" }
                    if let Some(value) = value {
                        div { class: s::VALUE, "{value}" }
                    }
                }
                if !is_destructive {
                    ChevronRight { class: "shrink-0 text-gray-400 fill-none", size: 18 }
                }
            }
        }
    }
}

/// A full-width settings row that navigates through native anchor semantics.
#[component]
pub fn SettingLink(
    label: String,
    value: Option<String>,
    href: String,
    class: Option<String>,
) -> Element {
    rsx! {
        a {
            class: merge_classes(
                "block w-full px-4 py-3 text-left no-underline",
                class.as_deref(),
            ),
            href: href,
            div { class: "flex justify-between items-center gap-3",
                div { class: "flex flex-col min-w-0 flex-1",
                    div { class: "{s::LABEL} truncate", "{label}" }
                    if let Some(value) = value {
                        div { class: s::VALUE, "{value}" }
                    }
                }
                ChevronRight { class: "shrink-0 text-gray-400 fill-none", size: 18 }
            }
        }
    }
}

/// Full-width gray separator to place between Setting rows.
#[component]
pub fn Separator(class: Option<String>) -> Element {
    rsx! {
        hr {
            class: merge_classes("border-t border-gray-200", class.as_deref()),
            role: "separator",
        }
    }
}

#[cfg(feature = "playground")]
#[component]
pub fn SettingsGroupPlaygroundDemo() -> Element {
    let mut inset = use_signal(|| true);
    let mut destructive = use_signal(|| false);
    rsx! {
        crate::PlaygroundDemoFrame {
            center: false,
            controls: rsx! {
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: inset(), onchange: move |_| inset.toggle() } span { "Inset" } }
                label { class: "g3-playground-check", input { r#type: "checkbox", checked: destructive(), onchange: move |_| destructive.toggle() } span { "Destructive action" } }
            },
            crate::Card { title: "Settings",
                SettingsGroup { inset: inset(),
                    Setting { label: "Game type", value: "Traditional" }
                    Separator {}
                    Setting { label: "Scoring", value: "Match play" }
                    Separator {}
                    SettingAction { label: "Leave game", destructive: destructive(), onclick: |_| {} }
                }
            }
        }
    }
}
crate::g3_playground! {
    name: "SettingsGroup",
    g3_name: "G3SettingsGroup",
    description: "Grouped settings rows.",
    demo: SettingsGroupPlaygroundDemo,
    source: "src/components/settings_group.rs",
}

#[cfg(test)]
mod tests {
    #[test]
    fn setting_action_uses_native_button_semantics() {
        let source = include_str!("settings_group.rs");
        let action = source
            .split("pub fn SettingAction")
            .nth(1)
            .expect("missing SettingAction");

        assert!(action.contains("button {"));
        assert!(action.contains("r#type: \"button\""));
        assert!(action.contains("disabled: is_disabled"));
    }

    #[test]
    fn destructive_setting_action_is_opt_in() {
        let source = include_str!("settings_group.rs");
        assert!(source.contains("destructive.unwrap_or(false)"));
        assert!(source.contains("text-red-600"));
    }

    #[test]
    fn setting_link_uses_anchor_semantics() {
        let source = include_str!("settings_group.rs");
        let link = source
            .split("pub fn SettingLink")
            .nth(1)
            .expect("missing SettingLink");

        assert!(link.contains("a {"));
        assert!(link.contains("href: href"));
    }

    #[test]
    fn setting_rows_truncate_labels_before_compressing_right_slots() {
        let source = include_str!("settings_group.rs");
        let setting_row_source = source
            .split("fn SettingRow")
            .nth(1)
            .and_then(|source| source.split("pub fn SettingsGroup").next())
            .expect("missing SettingRow source");

        assert!(setting_row_source.contains(r#"class: "flex flex-col min-w-0 flex-1""#));
        assert!(setting_row_source.contains(r#"class: "{s::LABEL} truncate""#));
        assert!(setting_row_source.contains(r#"class: "shrink-0""#));
    }
}
