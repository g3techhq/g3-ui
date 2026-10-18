//! Each guide owns its sections and reading order.
use super::*;

#[component]
pub fn GettingStarted() -> Element {
    rsx! {
        section {
            h2 { "1. Add the dependencies" }
            p { "Use Rust 1.88 or newer and Dioxus 0.7.9. Add these entries to your app's Cargo.toml:" }
            CodeBlock { language: Language::Toml, src: "[dependencies]\ndioxus = { version = \"0.7.9\", features = [\"web\"] }\ng3-ui = \"0.6\"".to_string() }
        }
        section {
            h2 { "2. Add a screen" }
            ul {
                li { code { "AppWrapper" } " loads the stylesheet and provides the theme, mode, and overlay hosts." }
                li { code { "Header" } " holds the screen title and toolbar actions." }
                li { code { "Content" } " provides the scrollable body below the header." }
            }
            CodeBlock { src: Example::GettingStarted.code() }
        }
        section {
            h2 { "3. Launch the app" }
            CodeBlock { src: "fn main() {\n    g3_ui::init_auto_mode();\n    dioxus::launch(GettingStartedExample);\n}".to_string() }
            p { "Run " code { "dx serve" } " from your app's folder. No Tailwind setup or separate JavaScript bundle is required." }
        }
        section {
            p { "Use the header controls to try the same screen at different sizes and in iOS or Material mode." }
            LiveExample { example: Example::GettingStarted, label: "Getting started" }
        }
    }
}

#[component]
pub fn ResponsiveLayouts() -> Element {
    rsx! {
        section {
            h2 { "Layout components" }
            dl { class: "playground-doc-choices",
                dt { code { "GridColumns::Fit(18.0)" } }
                dd { "Fits as many 18rem columns as the available space allows. Cards never shrink below that width unless the container is narrower." }
                dt { code { "PageLayout" } }
                dd { "Stacks main content and supporting panels, or arranges them in columns. All slots stay mounted." }
                dt { code { "AdaptiveLayout" } }
                dd { "Mounts one compact, medium, or expanded slot. Use it when the controls or component tree change with width." }
            }
        }
        section {
            h2 { "Content width" }
            p { "Wrap the page body in " code { "LayoutScope" } " when a sidebar or padding leaves less room than the full app. Nested layouts then respond to that content width." }
            p { "Compact is below 48rem, Medium is 48rem to below 64rem, and Expanded starts at 64rem. A tablet can still have compact content if navigation takes up enough space." }
        }
        section {
            p { "Change the layout size." }
            LiveExample { example: Example::ResponsiveLayouts, label: "Responsive layouts" }
        }
        section {
            h2 { "Keep shared state above AdaptiveLayout" }
            p { "Switching slots unmounts the previous slot. Put form values, resources, and playback state in the parent when they must survive a size change. Missing slots fall back to the nearest smaller slot." }
            CodeBlock { src: Example::ResponsiveLayouts.code() }
        }
    }
}

#[component]
pub fn Platforms() -> Element {
    rsx! {
        section {
            h2 { "Build targets" }
            p { "g3-ui renders in browsers and Dioxus native web views. Select the platform on your app's Dioxus dependency." }
            h3 { "Browser" }
            p { "Enable Dioxus's web feature, as shown in Getting started." }
            h3 { "Android, iOS, or desktop" }
            p { "Enable Dioxus's mobile feature for Android and iOS, or desktop for desktop web views. The g3-ui dependency stays the same." }
            CodeBlock { language: Language::Toml, src: "[dependencies]\ng3-ui = \"0.6\"\ndioxus = { version = \"0.7.9\", features = [\"mobile\"] }".to_string() }
            p { "Build with the Dioxus CLI's Android, iOS, or desktop target. Add g3-ui's optional transitions feature when integrating g3-route-transitions." }
        }
        section {
            h2 { "App setup" }
            p { "AppWrapper loads the CSS and supplies viewport metadata for safe areas. Set mode explicitly when you want consistent iOS or Material styling across platforms." }
            LiveExample { example: Example::Platforms, label: "Web and native web views" }
            CodeBlock { src: Example::Platforms.code() }
        }
        section {
            h2 { "Device testing" }
            ul {
                li { "Open the keyboard and check that focused fields remain reachable." }
                li { "Rotate the device and check safe areas and short landscape screens." }
                li { "Try touch gestures and Android Back with an overlay open." }
                li { "Check scrolling and performance on the devices your users have." }
            }
            p { "The browser viewer helps with layout and interaction. It does not simulate an Android keyboard, system Back, or device performance." }
        }
    }
}

#[component]
pub fn LayoutHooks() -> Element {
    rsx! {
        section {
            h2 { "Content width and app width" }
            p { "Both hooks report width. Resizing a desktop window can give it the same layout size as a phone." }
            g3_ui::Grid { columns: g3_ui::GridColumns::Fit(18.0), class: "playground-doc-comparison",
                g3_ui::Card {
                    h3 { code { "use_layout_size()" } }
                    p { "Measures the nearest LayoutScope, or AppWrapper when no local scope exists." }
                    p { strong { "Returns: " } "Compact, Medium, or Expanded." }
                }
                g3_ui::Card {
                    h3 { code { "use_shell_size()" } }
                    p { "Measures the enclosing AppWrapper. A smaller LayoutScope does not change its result." }
                    p { strong { "Returns: " } "Compact or Wide." }
                }
            }
        }
        section {
            h2 { "A wide app can contain narrow content" }
            p { "In a 1200px app with a 300px sidebar, the app is Wide and the remaining 900px content area is Medium. A LayoutScope around the content measures it separately." }
            p { "A nested LayoutScope around a 500px panel reports Compact to its children. use_shell_size() still reports Wide." }
        }
        section {
            p { "At Desktop size, limiting the content to 32rem changes its LayoutSize to Compact. The app's ShellSize stays Wide." }
            LiveExample { example: Example::LayoutHooks, label: "Layout hooks" }
        }
        section {
            h2 { "Width ranges" }
            p { "These ranges apply to the area each hook measures. Pixel examples assume the usual 16px root font size." }
            g3_ui::Table { caption: "Layout and shell width ranges", sticky_first_column: true,
                thead { tr { th { scope: "col", "Measured width" } th { scope: "col", "LayoutSize" } th { scope: "col", "ShellSize" } } }
                tbody {
                    tr { th { scope: "row", "Below 48rem (768px)" } td { "Compact" } td { "Compact" } }
                    tr { th { scope: "row", "48rem to below 64rem (768–1023px)" } td { "Medium" } td { "Wide" } }
                    tr { th { scope: "row", "64rem and above (1024px+)" } td { "Expanded" } td { "Wide" } }
                }
            }
        }
        section {
            h2 { "Usage" }
            ul {
                li { code { "use_layout_size()" } " selects behavior for the available content width, including space left beside navigation." }
                li { code { "use_shell_size()" } " selects behavior for the whole app using two width ranges." }
                li { "For visual rearrangement alone, use Grid, PageLayout, or CSS container queries. A hook is useful when behavior or the mounted controls must change." }
            }
            p { "Call the hooks in a child of the provider being measured. They update when a width boundary is crossed and return Compact before the first measurement, keeping server and initial client rendering consistent." }
            CodeBlock { src: Example::LayoutHooks.code() }
        }
    }
}

#[component]
pub fn OverlayHooks() -> Element {
    rsx! {
        section {
            h2 { "Overlay hooks" }
            p { "Call these hooks in a component inside AppWrapper. Its overlay hosts are already set up." }
            dl { class: "playground-doc-choices",
                dt { code { "use_toast()" } }
                dd { "Queue a short message, such as Saved." }
                dt { code { "use_alert()" } }
                dd { "Ask for confirmation and await a true/false answer." }
                dt { code { "use_action_sheet()" } }
                dd { "Offer actions and await the selected index, or None on cancel." }
                dt { code { "use_overlay_controller()" } }
                dd { "Count registered overlays or dismiss the top one before navigating back." }
            }
        }
        section {
            LiveExample { example: Example::OverlayHooks, label: "Overlay hooks" }
        }
        section {
            h2 { "Event handlers" }
            p { "Await alerts and action sheets in a handler or spawned task. Do not hold a signal read or write borrow across await. Call the hooks before any conditional return." }
            CodeBlock { src: Example::OverlayHooks.code() }
        }
    }
}

#[component]
pub fn ThemeHooks() -> Element {
    rsx! {
        section {
            h2 { "Shared state" }
            p { "AppWrapper owns the theme, mode, and component labels. These hooks return writable signals shared by its children." }
            ul {
                li { code { "use_theme()" } " returns Signal<Theme> for colors and surfaces." }
                li { code { "use_mode()" } " returns Signal<ComponentMode> for iOS or Material styling." }
                li { code { "use_strings()" } " returns Signal<Strings> for component labels, such as Cancel and Confirm." }
            }
            p { "Read a signal with () or read(). Declare it mut to call write() or set() in an event handler. Writes update the provider and its readers. Theme and mode are independent: dark colors can use either platform style." }
        }
        section {
            h2 { "Change colors" }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["ThemeColorControls"], true) }
        }
        section {
            LiveExample { example: Example::ThemeHooks, label: "Theme and mode hooks" }
        }
        section {
            h2 { "Initial values" }
            p { "Provider props are optional starting values. A later prop change replaces that value; unrelated renders preserve hook edits. Child components obtain the signals from the hooks." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["ThemeHooksExample"], true) }
        }
        section {
            h2 { "Change mode and labels" }
            p { "Call use_mode() and use_strings() in the component that changes them." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["AppearanceControls"], false) }
        }
        section {
            h2 { "Read values" }
            h3 { "Colors" }
            p { "For CSS alone, var(--g3-color-accent) follows the theme without a hook. Use theme.read() when Rust needs a color field, or theme() for a snapshot of the whole Theme." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["ThemedAccent"], false) }
            h3 { "Component labels" }
            p { "Component labels come from use_strings(). Application copy is translated separately." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["LocalizedActions"], false) }
        }
        section {
            h2 { "Component mode overrides" }
            p { "A reusable component can accept an optional mode prop. None inherits; an explicit value wins. Resolve the hook before choosing a variant." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["ModeAwareButton"], false) }
        }
        section {
            h2 { "Scoped overrides" }
            p { "ThemeProvider owns separate state for each value supplied as a prop. Omitted values share the parent's signal. This card overrides theme and mode, so edits to either stay within the card; strings remain shared with the app." }
            p { "Call hooks in a child of the provider. Outside a provider, each hook owns local default state." }
            CodeBlock { src: function_code(THEME_HOOKS_SOURCE, &["ScopedThemeExample", "LocalThemeEditor"], false) }
        }
    }
}
