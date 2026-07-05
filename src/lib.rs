//! g3_ui - reusable UI component library.

use manganis::Asset;
use manganis::asset;

pub static UI_CSS: Asset = asset!("/assets/g3_ui.css");

mod components;
mod descriptor;
mod theme;

pub use components::{
    Avatar, AvatarSize, Badge, Button, ButtonSize, ButtonStyle, Checkbox, Chip,
    ControlLabelPlacement, Field, InfoButton, Line, LineOrientation, Progress, Radio, RadioGroup,
    SegmentButton, SegmentGroup, Skeleton, SkeletonShape, Spinner, StatusColor, Toggle, ToggleSize,
};

pub use components::{
    Avatar as G3Avatar, Badge as G3Badge, Button as G3Button, Checkbox as G3Checkbox,
    Chip as G3Chip, ControlLabelPlacement as G3ControlLabelPlacement, Field as G3Field,
    InfoButton as G3InfoButton, Line as G3Line, Progress as G3Progress, Radio as G3Radio,
    RadioGroup as G3RadioGroup, SegmentButton as G3SegmentButton, SegmentGroup as G3SegmentGroup,
    Skeleton as G3Skeleton, Spinner as G3Spinner, Toggle as G3Toggle, ToggleSize as G3ToggleSize,
};

pub use components::{
    Card, ConfirmModal, Fab, FabButton, FabContainer, FabHorizontal, FabList, FabListSide, FabSize,
    FabVertical, Navbar, RightSlot, Select, SelectOption, Separator, Setting, SettingAction,
    SettingLink, SettingsGroup, Sheet, SheetButton, SheetPlacement,
};

pub use components::{
    Card as G3Card, ConfirmModal as G3ConfirmModal, Fab as G3Fab, FabButton as G3FabButton,
    FabContainer as G3FabContainer, FabList as G3FabList, Modal as G3Modal, Navbar as G3Navbar,
    Select as G3Select, Separator as G3Separator, Setting as G3Setting,
    SettingAction as G3SettingAction, SettingLink as G3SettingLink,
    SettingsGroup as G3SettingsGroup, Sheet as G3Sheet, SheetButton as G3SheetButton,
    SheetPlacement as G3SheetPlacement,
};

pub use components::{AppWrapper, Body, Header};

pub use components::{AppWrapper as G3AppWrapper, Body as G3Body, Header as G3Header};

pub use descriptor::{ComponentDescriptor, component_descriptors};
#[cfg(feature = "playground")]
pub use descriptor::{ComponentPlaygroundDemo, PlaygroundDemoFrame, component_playground_demos};

/// Re-export the prelude for convenience.
pub mod prelude;

/// Re-export theme utilities.
pub use theme::{
    ComponentMode, G3Mode, G3Theme, G3ThemeProvider, Theme, get_mode, init_auto_mode,
    merge_classes, set_mode, use_component_mode,
};

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;

    fn render(app: fn() -> Element) {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
    }

    #[component]
    fn PrimitiveSmokeApp() -> Element {
        let checked = use_signal(|| false);
        let active = use_signal(|| 0_usize);
        let field_value = use_signal(String::new);

        rsx! {
            G3ThemeProvider {
                mode: ComponentMode::Ios,
                G3Button { onclick: |_| {}, "Button" }
                G3Button {
                    style: ButtonStyle::Neutral,
                    start: rsx! { span { "G" } },
                    onclick: |_| {},
                    "Provider"
                }
                G3Toggle { checked }
                G3SegmentGroup { active,
                    G3SegmentButton { index: 0, "One" }
                    G3SegmentButton { index: 1, "Two" }
                }
                G3Field {
                    label: "Name",
                    value: field_value,
                    onchange: |_| {},
                    placeholder: "Name",
                }
                G3Spinner {}
                G3InfoButton {}
                G3Line {}
            }
        }
    }

    #[component]
    fn CompositeSmokeApp() -> Element {
        let sheet_open = use_signal(|| false);
        let modal_open = use_signal(|| false);
        let select_value = use_signal(|| "One".to_string());

        rsx! {
            G3Card { title: "Card", "Body" }
            G3Card {
                G3SettingsGroup { inset: true,
                    G3Setting { label: "Setting", value: "Value" }
                }
            }
            G3Card {
                title: "Inset choice",
                inset: true,
                selected: true,
                onclick: |_| {},
                "Choice body"
            }            G3Sheet { is_open: sheet_open, "Sheet body" }
            G3SheetButton { description: "Sheet button body" }
            G3ConfirmModal {
                open: modal_open,
                title: "Confirm",
                on_confirm: |_| {},
            }
            G3Select {
                value: select_value,
                options: vec![SelectOption::from("One"), SelectOption::from(("Two", "Second"))],
                onchange: |_| {},
            }
            G3SettingsGroup {
                G3Setting { label: "Setting", value: "Value" }
                G3Separator {}
                G3SettingAction {
                    label: "Action",
                    onclick: |_| {},
                }
                G3Separator {}
                G3SettingLink {
                    label: "Link",
                    href: "https://example.com",
                }
            }
            G3Fab {
                G3FabButton { onclick: |_| {}, "Fab" }
                G3FabList {
                    activated: true,
                    G3FabButton { onclick: |_| {}, size: FabSize::Small, "Mini" }
                }
            }
            G3FabContainer {
                main_button: rsx! { "Open" },
                list_buttons: rsx! {
                    G3FabButton { onclick: |_| {}, size: FabSize::Small, "A" }
                },
            }
        }
    }

    #[component]
    fn LayoutSmokeApp() -> Element {
        rsx! {
            G3AppWrapper {
                G3Navbar {
                    G3Header { title: "Header" }
                    G3Body {
                        Spinner { center: true }
                    }
                }
            }
        }
    }
    #[component]
    fn MobilePrimitiveAliasSmokeApp() -> Element {
        rsx! {
            G3Badge { color: StatusColor::Accent, "Live" }
            G3Avatar { fallback: "GP" }
            G3Chip { selected: true, onclick: |_| {}, "Walking" }
            G3Progress { value: 50.0 }
            G3Skeleton { shape: SkeletonShape::Row }
        }
    }

    #[test]
    fn primitives_render() {
        render(PrimitiveSmokeApp);
    }

    #[test]
    fn composites_render() {
        render(CompositeSmokeApp);
    }

    #[test]
    fn layouts_render() {
        render(LayoutSmokeApp);
    }
    #[test]
    fn mobile_primitive_aliases_render() {
        render(MobilePrimitiveAliasSmokeApp);
    }

    #[test]
    fn descriptors_cover_public_surface() {
        assert!(
            component_descriptors()
                .iter()
                .any(|descriptor| descriptor.g3_name == "G3Button")
        );
        assert!(
            component_descriptors()
                .iter()
                .any(|descriptor| descriptor.g3_name == "G3Sheet")
        );
        assert!(
            !component_descriptors()
                .iter()
                .any(|descriptor| descriptor.g3_name == "G3SettingsCard")
        );
        assert!(
            component_descriptors()
                .iter()
                .any(|descriptor| descriptor.g3_name == "G3Navbar")
        );
        assert!(
            component_descriptors()
                .iter()
                .any(|descriptor| descriptor.g3_name
                    == "G3Badge / G3Avatar / G3Chip / G3Progress / G3Skeleton")
        );
    }

    #[test]
    fn playground_source_include_uses_explicit_manifest_relative_paths() {
        let descriptor_source = include_str!("descriptor.rs");

        let primitives_source = include_str!("components/primitives.rs");

        assert!(descriptor_source.contains("source: $source:literal"));
        assert!(
            descriptor_source
                .contains("include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/\", $source))")
        );
        assert!(primitives_source.contains("source: \"src/components/primitives.rs\""));
        assert!(!descriptor_source.contains("include_str!(file!())"));
    }
    #[test]
    fn mobile_primitives_are_public_and_registered() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let components_mod =
            std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();

        assert!(crate_root.join("src/components/primitives.rs").exists());
        assert!(
            crate_root
                .join("src/components/primitives_styles.rs")
                .exists()
        );
        assert!(components_mod.contains("mod primitives;"));
        assert!(components_mod.contains("mod primitives_styles;"));
        assert!(components_mod.contains("primitives::DESCRIPTOR"));
        for symbol in [
            "Badge",
            "Avatar",
            "Chip",
            "Progress",
            "Skeleton",
            "SkeletonShape",
            "StatusColor",
            "G3Badge",
            "G3Avatar",
            "G3Chip",
            "G3Progress",
            "G3Skeleton",
        ] {
            assert!(
                public_source.contains(symbol),
                "{symbol} missing from lib exports"
            );
            assert!(
                prelude_source.contains(symbol),
                "{symbol} missing from prelude"
            );
        }
    }

    #[test]
    fn checkbox_is_public_registered_and_accessible() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let components_mod =
            std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let checkbox_source =
            std::fs::read_to_string(crate_root.join("src/components/checkbox.rs"))
                .unwrap_or_default();

        assert!(crate_root.join("src/components/checkbox.rs").exists());
        assert!(
            crate_root
                .join("src/components/checkbox_styles.rs")
                .exists()
        );
        assert!(components_mod.contains("mod checkbox;"));
        assert!(components_mod.contains("checkbox::DESCRIPTOR"));
        assert!(public_source.contains("Checkbox"));
        assert!(public_source.contains("G3Checkbox"));
        assert!(public_source.contains("ControlLabelPlacement"));
        assert!(checkbox_source.contains("role: \"checkbox\""));
        assert!(checkbox_source.contains("aria_checked"));
        assert!(checkbox_source.contains("indeterminate"));
    }

    #[test]
    fn radio_group_is_public_registered_and_accessible() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let components_mod =
            std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let radio_source =
            std::fs::read_to_string(crate_root.join("src/components/radio.rs")).unwrap_or_default();

        assert!(crate_root.join("src/components/radio.rs").exists());
        assert!(crate_root.join("src/components/radio_styles.rs").exists());
        assert!(components_mod.contains("mod radio;"));
        assert!(components_mod.contains("radio::DESCRIPTOR"));
        for symbol in ["RadioGroup", "Radio", "G3RadioGroup", "G3Radio"] {
            assert!(
                public_source.contains(symbol),
                "{symbol} missing from lib exports"
            );
        }
        assert!(radio_source.contains("role: \"radiogroup\""));
        assert!(radio_source.contains("role: \"radio\""));
        assert!(radio_source.contains("aria_checked"));
        assert!(radio_source.contains("let tab_index = if is_disabled"));
        assert!(!radio_source.contains("group_has_selection"));
        assert!(radio_source.contains("allow_empty_selection"));
    }

    #[test]
    fn mobile_primitive_styles_use_shared_theme_tokens() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        for selector in [
            ".g3-badge",
            ".g3-avatar",
            ".g3-chip",
            ".g3-progress",
            ".g3-skeleton",
        ] {
            assert!(stylesheet.contains(selector), "{selector} style missing");
        }
        assert!(stylesheet.contains("var(--color-focused)"));
        assert!(stylesheet.contains("var(--color-success)"));
        assert!(stylesheet.contains("var(--color-warning)"));
        assert!(stylesheet.contains("var(--color-danger)"));
    }
    #[test]
    fn mobile_primitives_keep_accessible_defaults() {
        let primitives_source = include_str!("components/primitives.rs");

        assert!(primitives_source.contains("unwrap_or(100.0)"));
        assert!(primitives_source.contains("let label = alt"));
        assert!(primitives_source.contains("or_else(|| fallback.clone())"));
        assert!(primitives_source.contains("unwrap_or_else(|| \"Avatar\".to_string())"));
        assert!(primitives_source.contains("aria_label: label"));
    }

    #[test]
    fn mobile_primitive_motion_respects_reduced_motion() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains("@media (prefers-reduced-motion: reduce)"));
        assert!(stylesheet.contains(".g3-progress-indeterminate .g3-progress-fill"));
        assert!(stylesheet.contains(".g3-skeleton"));
        assert!(stylesheet.contains("animation: none"));
    }
    #[test]
    fn button_styles_leave_layout_spacing_to_the_caller() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        for selector in [
            ".g3-btn-outline",
            ".g3-btn-clear",
            ".g3-btn-sm",
            ".g3-btn-md-size",
            ".g3-btn-lg",
        ] {
            let block = stylesheet
                .split(selector)
                .nth(1)
                .and_then(|rest| rest.split('}').next())
                .unwrap_or_else(|| panic!("missing {selector} style block"));
            assert!(
                !block.contains("margin:"),
                "{selector} should not add outside margins"
            );
        }

        assert!(stylesheet.contains(".g3-btn-neutral"));
        assert!(stylesheet.contains(".g3-btn-content-start"));
    }

    #[test]
    fn select_trigger_keeps_placeholder_on_one_line() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let select_block = stylesheet
            .split(".g3-select-btn")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing select button style block");

        assert!(select_block.contains("white-space: nowrap"));
        assert!(select_block.contains("line-height: 1"));
    }
    #[test]
    fn buttons_have_obvious_disabled_state() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        let disabled_block = stylesheet
            .split(".g3-btn:disabled")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing disabled button style block");

        assert!(disabled_block.contains("opacity: 0.45"));
        assert!(disabled_block.contains("cursor: not-allowed"));
        assert!(disabled_block.contains("filter: grayscale"));
        assert!(disabled_block.contains("box-shadow: none"));
        assert!(stylesheet.contains(".g3-btn:disabled:hover"));
        assert!(stylesheet.contains(".g3-btn:disabled:active"));
    }

    #[test]
    fn provider_buttons_match_google_typography_and_alignment() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains("font-family: \"G3 Provider Roboto\""));
        let provider_font = stylesheet
            .split("@font-face")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing provider font face");
        assert!(provider_font.contains("font-weight: 500"));

        let neutral_block = stylesheet
            .split(".g3-btn-neutral")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing neutral button style block");
        assert!(neutral_block.contains("background: white"));
        assert!(neutral_block.contains("color: #1f1f1f"));
        assert!(neutral_block.contains("border: 1px solid #747775"));

        let neutral_button_block = stylesheet
            .split(".g3-btn.g3-btn-neutral")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing neutral button typography block");
        assert!(neutral_button_block.contains("font-weight: 500"));

        let content_block = stylesheet
            .split(".g3-btn-content-start")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing start-content style block");
        assert!(content_block.contains("justify-content: center"));
        assert!(content_block.contains("position: relative"));

        let start_block = stylesheet
            .split(".g3-btn-start {")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing start-icon style block");
        assert!(start_block.contains("left: 0.75rem"));
        assert!(start_block.contains("position: absolute"));
    }

    #[test]
    fn layout_surfaces_have_material_contracts() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains("border-radius: 0.25rem"));
        assert!(stylesheet.contains(".g3-card-control"));
        assert!(stylesheet.contains(".g3-card-inset"));
        assert!(stylesheet.contains(".g3-settings-group-inset"));
    }

    #[test]
    fn component_source_tree_is_flat() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

        assert!(crate_root.join("components").is_dir());
        assert!(!crate_root.join("atoms").exists());
        assert!(!crate_root.join("molecules").exists());
        assert!(!crate_root.join("organisms").exists());
    }

    #[test]
    fn settings_card_is_not_a_public_component() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let components_mod =
            std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
        let lib_source = std::fs::read_to_string(crate_root.join("src/lib.rs")).unwrap();
        let public_source = lib_source
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");

        assert!(!crate_root.join("src/components/settings_card.rs").exists());
        assert!(
            !crate_root
                .join("src/components/settings_card_styles.rs")
                .exists()
        );
        assert!(!components_mod.contains("settings_card"));
        assert!(!public_source.contains("SettingsCard"));
        assert!(!public_source.contains("G3SettingsCard"));
    }

    #[test]
    fn navbar_is_public_layout_component_and_owns_transition_base_marker() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let components_mod =
            std::fs::read_to_string(crate_root.join("src/components/mod.rs")).unwrap();
        let lib_source = std::fs::read_to_string(crate_root.join("src/lib.rs")).unwrap();
        let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();
        let public_source = lib_source
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let navbar_source = std::fs::read_to_string(crate_root.join("src/components/navbar.rs"))
            .unwrap_or_default();

        assert!(crate_root.join("src/components/navbar.rs").exists());
        assert!(crate_root.join("src/components/navbar_styles.rs").exists());
        assert!(components_mod.contains("mod navbar;"));
        assert!(components_mod.contains("pub(crate) mod navbar_styles;"));
        assert!(components_mod.contains("pub use navbar::*;"));
        assert!(public_source.contains("Navbar"));
        assert!(public_source.contains("G3Navbar"));
        assert!(prelude_source.contains("Navbar"));
        assert!(prelude_source.contains("G3Navbar"));
        assert!(navbar_source.contains("#[cfg(feature = \"transitions\")]"));
        assert!(navbar_source.contains("ROUTE_TRANSITION_BASE_CLASS"));
    }
    #[test]
    fn transitions_feature_is_optional_and_drives_shell_and_body_markers() {
        let cargo = include_str!("../Cargo.toml");
        let app_wrapper_source = include_str!("components/app_wrapper.rs");
        let body_source = include_str!("components/body.rs");
        let body_styles = include_str!("components/body_styles.rs");

        assert!(cargo.contains("transitions = [\"dep:dx-route-transitions\"]"));
        assert!(cargo.contains("dx-route-transitions = { version = \"0.1.0\", optional = true }"));
        assert!(!body_styles.contains("route-transition-segment"));
        assert!(app_wrapper_source.contains("RouteTransitionProvider"));
        assert!(app_wrapper_source.contains("ROUTE_TRANSITION_COVER_CLASS"));
        assert!(body_source.contains("ROUTE_TRANSITION_SEGMENT_CLASS"));
    }
    #[test]
    fn app_wrapper_bundles_library_stylesheet() {
        let source = include_str!("components/app_wrapper.rs");

        assert!(source.contains("UI_CSS"));
        assert!(source.contains("document::Link"));
    }

    #[test]
    fn theme_defaults_are_configurable_without_mode() {
        let theme = Theme::default_light().with_focused("#22c55e");

        assert_eq!(theme.focused, "#22c55e");
        assert_eq!(theme.bg, "#f8f8f8");
    }

    #[test]
    fn app_wrapper_accepts_custom_theme_tokens() {
        let source = include_str!("components/app_wrapper.rs");

        assert!(source.contains("theme: Option<Theme>"));
        assert!(source.contains("theme.as_ref().map(Theme::to_style_attr)"));
        assert!(!source.contains("G3Theme { mode }"));
    }
    #[test]
    fn focused_theme_color_drives_tint_styles() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(!stylesheet.contains("rgba(0, 122, 255"));
        assert!(stylesheet.contains("color-mix(in srgb, var(--color-focused) 8%"));
        assert!(stylesheet.contains("color-mix(in srgb, var(--color-focused) 20%"));
    }
    #[test]
    fn segments_follow_ionic_mode_contracts() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        let ios_button_block = stylesheet
            .split(".g3-segment-btn-ios {")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing iOS segment button style block");
        assert!(ios_button_block.contains("min-height: 28px"));
        assert!(ios_button_block.contains("font-size: 13px"));

        let md_button_block = stylesheet
            .split(".g3-segment-btn-md {")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing MD segment button style block");
        assert!(md_button_block.contains("min-height: 48px"));
        assert!(md_button_block.contains("font-size: 14px"));
        assert!(md_button_block.contains("font-weight: 500"));
        assert!(md_button_block.contains("letter-spacing: 0.06em"));
        assert!(md_button_block.contains("text-transform: uppercase"));
        assert!(stylesheet.contains(".g3-segment-toolbar"));
        assert!(stylesheet.contains(".g3-segment-standalone"));
        assert!(stylesheet.contains(".g3-segment-route-wrapper"));
        assert!(stylesheet.contains(".g3-segment-btn-ios:focus"));
        assert!(stylesheet.contains(".g3-segment-btn-ios:focus-visible"));
        assert!(stylesheet.contains(".g3-segment-standalone .g3-segment-btn-md"));
        assert!(stylesheet.contains(".g3-segment-standalone .g3-segment-btn-ios"));
        assert!(stylesheet.contains("min-width: 0"));
        assert!(stylesheet.contains(".g3-segment-standalone.g3-segment-ios .g3-segment-btn-ios"));
        assert!(stylesheet.contains("--g3-segment-ios-background, rgba(60, 60, 67, 0.065)"));
        assert!(stylesheet.contains("transform 320ms cubic-bezier(0.4, 0, 0.2, 1)"));
    }

    #[test]
    fn closed_sheets_do_not_paint_offscreen_shadows() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let sheet_source = include_str!("components/sheet.rs");

        assert!(sheet_source.contains("STATE_CLOSED"));
        assert!(stylesheet.contains(".g3-sheet.g3-sheet-closed"));
        assert!(stylesheet.contains("box-shadow: none"));
    }

    #[test]
    fn header_slots_own_top_bar_edge_spacing() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains(".g3-header-row"));
        assert!(
            stylesheet.contains("grid-template-columns: minmax(44px, 1fr) auto minmax(44px, 1fr)")
        );
        assert!(stylesheet.contains(".g3-header-start-slot"));
        assert!(stylesheet.contains(".g3-header-end-slot"));
        assert!(stylesheet.contains("padding: 0 0.5rem"));
    }

    #[test]
    fn header_toolbar_and_ios_scrollbar_contracts_are_mobile_clean() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        let ios_header_block = stylesheet
            .split(".g3-header-ios")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing iOS header block");
        assert!(ios_header_block.contains("background: var(--color-card)"));

        let md_toolbar_block = stylesheet
            .split(".g3-header-md .g3-header-toolbar")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing MD header toolbar block");
        assert!(md_toolbar_block.contains("padding: 0"));

        assert!(stylesheet.contains(".g3-header-ios .g3-header-slot .g3-btn"));
        assert!(stylesheet.contains(".g3-app-shell .g3-body-content"));
        assert!(stylesheet.contains(".g3-app-shell .g3-body-content::-webkit-scrollbar"));
        assert!(stylesheet.contains("scrollbar-width: none"));
        assert!(stylesheet.contains(".g3-shell-md .g3-body-content"));
        assert!(stylesheet.contains("-ms-overflow-style: none"));
    }

    #[test]
    fn body_loading_uses_the_centered_shared_spinner() {
        let source = include_str!("components/body.rs");

        assert!(source.contains("Spinner { center: true }"));
        assert!(!source.contains("ResourceLoading"));
    }

    #[test]
    fn resource_helpers_and_timeout_are_not_public_g3_ui_api() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();
        let util_source =
            std::fs::read_to_string(crate_root.join("src/util/mod.rs")).unwrap_or_default();

        for symbol in [
            "ResourceLoading",
            "ResourceError",
            "G3ResourceLoading",
            "G3ResourceError",
            "TimeoutError",
            "with_timeout",
        ] {
            assert!(
                !public_source.contains(symbol),
                "{symbol} is still exported"
            );
            assert!(
                !prelude_source.contains(symbol),
                "{symbol} is still in the prelude"
            );
            assert!(
                !util_source.contains(symbol),
                "{symbol} is still in util exports"
            );
        }

        assert!(!crate_root.join("src/util/resource_view.rs").exists());
        assert!(!crate_root.join("src/util/timeout.rs").exists());
    }

    #[test]
    fn spinner_has_centering_option_for_resource_fallbacks() {
        let source = include_str!("components/spinner.rs");
        let styles = include_str!("components/spinner_styles.rs");

        assert!(source.contains("center: Option<bool>"));
        assert!(source.contains("if center.unwrap_or(false)"));
        assert!(styles.contains("CENTERED"));
        assert!(styles.contains("g3-spinner-centered"));
        assert!(include_str!("../assets/g3_ui.css").contains(".g3-spinner-centered"));
    }

    #[test]
    fn sheets_hide_scrollbars_without_skipping_close_animation() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        let closed_block = stylesheet
            .split(".g3-sheet.g3-sheet-closed")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing closed sheet style block");
        assert!(closed_block.contains("visibility: hidden"));
        assert!(stylesheet.contains("visibility 0s linear var(--transition-normal)"));
        assert!(stylesheet.contains(".g3-sheet-open"));
        assert!(stylesheet.contains("transition-delay: 0s"));
        assert!(stylesheet.contains(".g3-sheet-content::-webkit-scrollbar"));
        assert!(stylesheet.contains(".g3-sheet-content"));
        assert!(stylesheet.contains("scrollbar-width: none"));
    }

    #[test]
    fn sheets_support_bottom_and_side_placements_without_changing_default() {
        let sheet_source = include_str!("components/sheet.rs");
        let sheet_styles = include_str!("components/sheet_styles.rs");
        let stylesheet = include_str!("../assets/g3_ui.css");
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");

        assert!(public_source.contains("SheetPlacement"));
        assert!(sheet_source.contains("pub enum SheetPlacement"));
        assert!(sheet_source.contains("placement: Option<SheetPlacement>"));
        assert!(sheet_source.contains("unwrap_or_default()"));
        assert!(sheet_styles.contains("SHEET_BOTTOM"));
        assert!(sheet_styles.contains("SHEET_LEFT"));
        assert!(sheet_styles.contains("SHEET_RIGHT"));
        assert!(stylesheet.contains(".g3-sheet-bottom"));
        assert!(stylesheet.contains(".g3-sheet-left"));
        assert!(stylesheet.contains(".g3-sheet-right"));
        assert!(stylesheet.contains(".g3-sheet-handle-wrap-ios"));
    }

    #[test]
    fn route_sheet_page_is_not_public_component_api() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let public_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();
        let descriptor_source = include_str!("descriptor.rs");

        assert!(!public_source.contains("SheetPage"));
        assert!(!public_source.contains("G3SheetPage"));
        assert!(!public_source.contains("RouteSheetSurface"));
        assert!(!public_source.contains("G3RouteSheetSurface"));
        assert!(!prelude_source.contains("SheetPage"));
        assert!(!prelude_source.contains("G3SheetPage"));
        assert!(!prelude_source.contains("RouteSheetSurface"));
        assert!(!prelude_source.contains("G3RouteSheetSurface"));
        assert!(!descriptor_source.contains("G3SheetPage"));
        assert!(!descriptor_source.contains("RouteSheetSurface"));
        assert!(!crate_root.join("src/components/sheet_page.rs").exists());
        assert!(
            !crate_root
                .join("src/components/sheet_page_styles.rs")
                .exists()
        );
    }

    #[test]
    fn sheet_backdrops_can_cover_the_full_app_shell() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let body_block = stylesheet
            .split(".g3-body {")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing body style block");
        let backdrop_block = stylesheet
            .split(".g3-sheet-backdrop")
            .nth(1)
            .and_then(|rest| rest.split('}').next())
            .expect("missing sheet backdrop style block");

        assert!(!body_block.contains("isolation: isolate"));
        assert!(backdrop_block.contains("position: fixed"));
        assert!(backdrop_block.contains("inset: 0"));
        assert!(backdrop_block.contains("z-index"));
    }

    #[test]
    fn shared_message_text_styles_are_component_owned() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains(".g3-message-text"));
        assert!(stylesheet.contains(".g3-message-text-success"));
        assert!(stylesheet.contains(".g3-message-text-danger"));
        assert!(stylesheet.contains(".g3-message-text-warning"));
    }
    #[test]
    fn playground_uses_component_owned_demos_instead_of_auto_specs() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let playground_root = crate_root.join("playground/src");
        let playground_main = std::fs::read_to_string(playground_root.join("main.rs")).unwrap();
        let components_mod = include_str!("components/mod.rs");
        let button_source = include_str!("components/button.rs");

        assert!(
            !playground_root
                .join("playground_gen/spec_generator.rs")
                .exists()
        );
        assert!(!playground_main.contains("auto_spec"));
        assert!(!playground_main.contains("auto_specs"));
        assert!(playground_main.contains("component_playground_demos"));
        assert!(components_mod.contains("button::PLAYGROUND"));
        let descriptor_source = include_str!("descriptor.rs");
        assert!(button_source.contains("crate::g3_playground!"));
        assert!(button_source.contains("ButtonPlaygroundDemo"));
        assert!(button_source.contains("crate::PlaygroundDemoFrame"));
        assert!(descriptor_source.contains("pub fn PlaygroundDemoFrame"));
        assert!(descriptor_source.contains("rsx! { $demo {} }"));
        assert!(!descriptor_source.contains("$demo()"));
        assert!(!playground_main.contains("PhoneFrame { {rendered_demo} }"));
        assert!(!components_mod.contains("fn render_button_demo"));
        assert!(!components_mod.contains("ComponentCategory"));
        assert!(!playground_main.contains("CategoryNav"));
        assert!(!playground_main.contains("category"));
        assert!(!playground_main.contains("gallery"));
        assert!(!playground_main.contains("Gallery"));
    }
    #[test]
    fn segment_panel_is_removed_from_public_surface() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let segment_source = include_str!("components/segment.rs");
        let lib_source = include_str!("lib.rs")
            .split("#[cfg(test)]")
            .next()
            .expect("library source should have a public section");
        let prelude_source = std::fs::read_to_string(crate_root.join("src/prelude.rs")).unwrap();

        assert!(!segment_source.contains("pub fn SegmentPanel"));
        assert!(!segment_source.contains("render: Callback"));
        assert!(!segment_source.contains("render.call"));
        assert!(!lib_source.contains("SegmentPanel"));
        assert!(!lib_source.contains("G3SegmentPanel"));
        assert!(!prelude_source.contains("SegmentPanel"));
        assert!(!prelude_source.contains("G3SegmentPanel"));
    }

    #[test]
    fn segment_group_is_not_route_or_transition_aware() {
        let segment_source = include_str!("components/segment.rs");

        assert!(!segment_source.contains("SegmentRouteTarget"));
        assert!(!segment_source.contains("HashMap"));
        assert!(!segment_source.contains("animated_update"));
        assert!(!segment_source.contains("NavigationAnimation"));
        assert!(!segment_source.contains("navigator.push"));
        assert!(!segment_source.contains("route: Option"));
    }
    #[test]
    fn segment_group_can_defer_active_for_animated_callers() {
        let segment_source = include_str!("components/segment.rs");

        assert!(segment_source.contains("defer_active: Option<bool>"));
        assert!(segment_source.contains("defer_active: defer_active.unwrap_or(false)"));
        assert!(segment_source.contains("if !context.defer_active"));
        assert!(segment_source.contains("(context.active).set(index)"));
    }

    #[test]
    fn segments_expose_indicator_and_child_view_contracts() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let segment_source = include_str!("components/segment.rs");
        let segment_styles = include_str!("components/segment_styles.rs");

        assert!(segment_source.contains("\"data-active\""));
        assert!(stylesheet.contains(".g3-segment-md::after"));
        assert!(stylesheet.contains(".g3-segment-ios::before"));
        assert!(stylesheet.contains("--g3-segment-count"));
        assert!(stylesheet.contains("--g3-segment-active"));
        assert!(
            stylesheet.contains("transform: translateX(calc(var(--g3-segment-active, 0) * 100%))")
        );
        assert!(!segment_source.contains("s::VIEWPORT"));
        assert!(!segment_styles.contains("VIEWPORT"));
        assert!(!stylesheet.contains(".g3-segment-viewport"));
        assert!(!stylesheet.contains(".g3-segment-view"));
        assert!(!stylesheet.contains("data-g3-segment-animation"));
        assert!(!stylesheet.contains("@keyframes g3-segment-in-left"));
        assert!(!stylesheet.contains("@keyframes g3-segment-in-right"));
        assert!(!stylesheet.contains("g3-segment-slide-left"));
        assert!(!stylesheet.contains("g3-segment-slide-right"));
    }

    #[test]
    fn segment_on_change_observes_previous_active_index() {
        let segment_source = include_str!("components/segment.rs");
        let callback_index = segment_source
            .find("on_change.call(index)")
            .expect("missing segment on_change callback");
        let active_set_index = segment_source
            .find("(context.active).set(index)")
            .expect("missing segment active mutation");

        assert!(
            callback_index < active_set_index,
            "segment on_change must run before active changes so callers can compute slide direction"
        );
    }

    #[test]
    fn segment_group_does_not_own_child_panel_layout() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let segment_source = include_str!("components/segment.rs");

        assert!(!stylesheet.contains(".g3-segment-viewport"));
        assert!(!stylesheet.contains(".g3-segment-view"));
        assert!(!stylesheet.contains(".g3-segment-view-exiting"));
        assert!(!segment_source.contains("{children}\n            }\n        }\n    }\n}"));
    }

    #[test]
    fn modal_card_owns_fixed_centering_and_exit_motion() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let modal_source = include_str!("components/modal.rs");

        let card_block = stylesheet
            .split(".g3-modal-card")
            .nth(1)
            .expect("missing modal card style block")
            .split('}')
            .next()
            .expect("missing modal card declaration block");

        assert!(card_block.contains("position: fixed"));
        assert!(card_block.contains("top: 50%"));
        assert!(card_block.contains("left: 50%"));
        assert!(stylesheet.contains(".g3-modal[data-state=\"closed\"]"));
        assert!(stylesheet.contains("@keyframes g3-modal-out"));
        assert!(!modal_source.contains("top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"));
    }

    #[test]
    fn modal_close_motion_follows_overlay_closed_state() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(stylesheet.contains(".g3-modal-overlay[data-state=\"closed\"] .g3-modal"));
        assert!(stylesheet.contains("g3-modal-out var(--g3-modal-debug-duration)"));
        assert!(stylesheet.contains("--g3-modal-debug-duration: 600ms"));
        assert!(stylesheet.contains("calc(-50% + 2rem)"));
    }

    #[test]
    fn segment_panel_edge_clipping_styles_are_removed() {
        let stylesheet = include_str!("../assets/g3_ui.css");

        assert!(!stylesheet.contains(".g3-body-content > .g3-segment-viewport"));
        assert!(!stylesheet.contains("width: calc(100% + 3rem)"));
        assert!(!stylesheet.contains("width: calc(100% + 20rem)"));
        assert!(!stylesheet.contains("width: calc(100% + 40rem)"));
    }

    #[test]
    fn segments_use_custom_properties_for_animated_any_count_indicators() {
        let stylesheet = include_str!("../assets/g3_ui.css");
        let segment_source = include_str!("components/segment.rs");

        assert!(!segment_source.contains("count: Option<usize>"));
        assert!(segment_source.contains("count_segment_children"));
        assert!(segment_source.contains("count_dynamic_components"));
        assert!(segment_source.contains("DynamicNode::Fragment"));
        assert!(segment_source.contains("--g3-segment-count"));
        assert!(segment_source.contains("--g3-segment-active"));
        assert!(stylesheet.contains("width: calc(100% / var(--g3-segment-count, 3))"));
        assert!(stylesheet.contains("width: calc((100% - 8px) / var(--g3-segment-count, 3))"));

        assert!(
            stylesheet.contains("transform: translateX(calc(var(--g3-segment-active, 0) * 100%))")
        );
        assert!(!stylesheet.contains(":nth-child(5):last-child"));
        assert!(!stylesheet.contains("translateX(400%)"));
    }
    #[test]
    fn timing_uses_dioxus_sdk_time_instead_of_custom_target_split() {
        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let cargo = std::fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
        let field_source = include_str!("components/field.rs");

        assert!(cargo.contains("dioxus-sdk-time"));
        assert!(field_source.contains("dioxus_sdk_time::sleep"));
        assert!(!field_source.contains("gloo_timers"));
        assert!(!field_source.contains("tokio::time"));
    }
}
