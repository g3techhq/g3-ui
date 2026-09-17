//! Static: a listening app whose screens each demonstrate one route
//! transition. Built only from public g3-ui components.
use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleUserRound, Headphones, Library, Radio};
use g3_route_transitions::{RouteTransitionPage, animated_navigate};
use g3_ui::{
    AdaptiveNav, AdaptiveNavCompact, AppWrapper, Badge, Button, ButtonExpand, ButtonFill,
    ButtonSize, Card, Color, Content, Header, Item, ItemDetail, List, ListLines, NavItem,
    NavItemGroup, PlaygroundDemoFrame, SegmentButton, SegmentGroup, TabLayout, Text, TextTone,
    TextVariant, ThemeProvider,
};

use super::{PlaygroundViewport, Route, active_playground_settings};

pub const DESCRIPTION: &str = "Static, a listening app whose screens each demonstrate one route-owned transition: cross-fading tab roots, a pushed page, a rising sheet, sliding segments, and a drill-down.";
pub const SOURCE: &str = r#"#[derive(Clone, Debug, PartialEq, Routable, RouteTransitions)]
enum Route {
    // Tab roots cross-fade between each other.
    #[transition(layer = stack_root)]
    #[route("/transitions")]
    Listen {},

    // Listen -> Album slides forward; Back slides backward.
    #[transition(layer = stack_page)]
    #[route("/transitions/album")]
    Album {},

    // Rises over the current page; Back drops it away.
    #[transition(layer = sheet)]
    #[route("/transitions/queue")]
    Queue {},

    // Segments slide by tab order without adding history entries.
    #[transition(history = replace, peers(group = library, order = tab))]
    #[route("/transitions/library/:tab")]
    Library { tab: u8 },

    // A base page that drills into a stack page.
    #[transition(forward_to = Episode)]
    #[route("/transitions/radio")]
    Radio {},

    #[transition(layer = stack_page)]
    #[route("/transitions/radio/episode")]
    Episode {},
}

rsx! {
    // Overlay region: rises and falls for sheets.
    AppWrapper {
        // Base region: stays put, dims under sheets.
        // Sheet routes pass `route_transition_base: false` and hide their
        // navigation on phones with `compact: Hidden`, so a desktop rail
        // stays beside them.
        TabLayout {
            // Page region: slides for Forward/Backward.
            RouteTransitionPage {
                Header { title: "Listen" }
                Content { Card { title: "Now playing", "Blue Static" } }
            }
            // Persistent chrome: a desktop rail stays still in every transition.
            AdaptiveNav { /* NavItem { to: Route::Listen {}, .. } */ }
        }
    }
}"#;

#[derive(Clone, Copy, PartialEq)]
enum ShowcaseScreen {
    Listen,
    Album,
    Queue,
    Library(u8),
    Radio,
    Episode,
    Profile,
}

#[component]
pub fn TransitionListen() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Listen } }
}

#[component]
pub fn TransitionAlbum() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Album } }
}

#[component]
pub fn TransitionQueue() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Queue } }
}

#[component]
pub fn TransitionLibrary(tab: u8) -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Library(tab.min(2)) } }
}

#[component]
pub fn TransitionProfile() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Profile } }
}

#[component]
pub fn TransitionRadio() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Radio } }
}

#[component]
pub fn TransitionEpisode() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Episode } }
}

#[component]
fn TransitionShowcase(screen: ShowcaseScreen) -> Element {
    let (mode, theme, viewport) = active_playground_settings();
    let label = match viewport {
        PlaygroundViewport::Desktop => "Compact desktop",
        PlaygroundViewport::Mobile => "Mobile",
    };
    rsx! {
        section {
            class: "playground-viewport-card playground-viewport-{viewport.as_str()} transition-showcase-card",
            aria_label: format!("{label} route transition preview"),
            div { class: "playground-viewport-demo",
                ThemeProvider { mode, theme: theme.to_theme(),
                    PlaygroundDemoFrame {
                        app: false,
                        center: false,
                        class: "transition-showcase-preview",
                        div { class: "transition-showcase-device-frame", aria_hidden: "true" }
                        AppWrapper { mode, class: "g3-playground-device-app transition-showcase-app",
                            TransitionScreen { screen }
                        }
                    }
                }
            }
        }
    }
}

async fn go(route: Route) {
    animated_navigate(route).await;
}

/// The line under each title saying which transition brought this screen in.
#[component]
fn TransitionNote(children: Element) -> Element {
    rsx! {
        Text { variant: TextVariant::Caption, tone: TextTone::Secondary, {children} }
    }
}

#[component]
fn TransitionScreen(screen: ShowcaseScreen) -> Element {
    match screen {
        ShowcaseScreen::Queue => rsx! { QueueScreen {} },
        ShowcaseScreen::Library(tab) => rsx! { LibraryScreen { tab } },
        _ => rsx! {
            TabLayout {
                RouteTransitionPage { class: "transition-showcase-page".to_string(),
                    Header {
                        title: screen_title(screen),
                        start: matches!(screen, ShowcaseScreen::Album | ShowcaseScreen::Episode)
                            .then(|| rsx! {
                                g3_ui::BackButton {
                                    onclick: move |_| {
                                        let destination = if screen == ShowcaseScreen::Episode {
                                            Route::TransitionRadio {}
                                        } else {
                                            Route::TransitionListen {}
                                        };
                                        spawn(go(destination));
                                    },
                                }
                            }),
                    }
                    Content { footer_space: false,
                        match screen {
                            ShowcaseScreen::Listen => rsx! { ListenContent {} },
                            ShowcaseScreen::Album => rsx! { AlbumContent {} },
                            ShowcaseScreen::Profile => rsx! { ProfileContent {} },
                            ShowcaseScreen::Radio => rsx! { RadioContent {} },
                            ShowcaseScreen::Episode => rsx! { EpisodeContent {} },
                            ShowcaseScreen::Queue | ShowcaseScreen::Library(_) => rsx! {},
                        }
                    }
                }
                ShowcaseNav { selected: screen }
            }
        },
    }
}

fn screen_title(screen: ShowcaseScreen) -> &'static str {
    match screen {
        ShowcaseScreen::Album => "Blue Static",
        ShowcaseScreen::Profile => "Profile",
        ShowcaseScreen::Radio => "Radio",
        ShowcaseScreen::Episode => "Side B, revisited",
        ShowcaseScreen::Listen | ShowcaseScreen::Queue | ShowcaseScreen::Library(_) => "Listen",
    }
}

#[component]
fn ListenContent() -> Element {
    rsx! {
        TransitionNote { "A tab root. Moving between Listen, Library, Radio, and Profile cross-fades." }
        Card {
            title: "Blue Static",
            end: rsx! { "3:48" },
            p { "Afterglow · Track 3 of 9" }
            div { class: "transition-showcase-actions",
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| async move { go(Route::TransitionAlbum {}).await },
                    "Open album"
                }
                Button {
                    size: ButtonSize::Sm,
                    fill: ButtonFill::Outline,
                    onclick: move |_| async move { go(Route::TransitionQueue {}).await },
                    "Open queue"
                }
            }
        }
        List { variant: g3_ui::ListVariant::Raised, lines: ListLines::Inset,
            Item {
                label: "Paper Moon",
                description: "Afterglow",
                end: rsx! { Badge { color: Color::Success, "Next" } },
            }
            Item { label: "Night Ferry", description: "Marine Drive", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn AlbumContent() -> Element {
    rsx! {
        TransitionNote { "A stack page. It slid in over Listen; Back slides it away." }
        Card { title: "Blue Static", end: rsx! { "2024" },
            p { "Afterglow · nine tracks · 38 minutes" }
        }
        List { variant: g3_ui::ListVariant::Raised, lines: ListLines::Full,
            Item { label: "Low Tide", metadata: "4:12" }
            Item { label: "Paper Moon", metadata: "3:05" }
            Item { label: "Blue Static", metadata: "3:48" }
        }
    }
}

#[component]
fn ProfileContent() -> Element {
    rsx! {
        TransitionNote { "Another tab root, so arriving here cross-fades rather than slides." }
        Card { title: "Matthew W.",
            p { "412 hours listened · 38 albums saved" }
            Badge { color: Color::Accent, "Following 18" }
        }
        List { variant: g3_ui::ListVariant::Raised,
            Item { label: "Saved albums", metadata: "38", detail: ItemDetail::Show }
            Item { label: "Listening history", detail: ItemDetail::Show }
            Item { label: "Settings", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn RadioContent() -> Element {
    rsx! {
        TransitionNote { "A base page that declares where it drills to, so the push is ready before the tap." }
        Card {
            title: "The Long Player",
            end: rsx! { "52 min" },
            onclick: move |_| async move { go(Route::TransitionEpisode {}).await },
            p { "This week: a record that only makes sense on the second side." }
        }
        List { variant: g3_ui::ListVariant::Raised,
            Item { label: "Field Recordings", description: "Weekly · 40 min", detail: ItemDetail::Show }
            Item { label: "Night Shift", description: "Daily · 20 min", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn EpisodeContent() -> Element {
    rsx! {
        TransitionNote { "The drill-down target, pushed as a stack page." }
        Card { title: "Side B, revisited",
            p { class: "transition-showcase-article",
                "The first side asks the question and the second one answers it, which is why the running order matters more than any single track on the record."
            }
        }
    }
}

#[component]
fn LibraryScreen(tab: u8) -> Element {
    let mut selected = use_signal(|| tab);
    use_effect(use_reactive!(|tab| selected.set(tab)));
    rsx! {
        TabLayout {
            Header {
                title: "Library",
                toolbar: rsx! {
                    SegmentGroup {
                        value: selected,
                        aria_label: "Library",
                        defer_selection: true,
                        onchange: move |next: u8| {
                            spawn(go(Route::TransitionLibrary { tab: next }));
                        },
                        SegmentButton { value: 0_u8, "Albums" }
                        SegmentButton { value: 1_u8, "Artists" }
                        SegmentButton { value: 2_u8, "Playlists" }
                    }
                },
            }
            Content { footer_space: false,
                TransitionNote { "Each segment is its own route. They slide by tab order and replace history, so Back leaves the library rather than walking the segments." }
                List { variant: g3_ui::ListVariant::Raised, lines: ListLines::Inset,
                    for (name, value) in library_rows(tab) {
                        Item { key: "{name}", label: name, metadata: value }
                    }
                }
            }
            ShowcaseNav { selected: ShowcaseScreen::Library(tab) }
        }
    }
}

fn library_rows(tab: u8) -> [(&'static str, &'static str); 3] {
    match tab {
        1 => [
            ("Afterglow", "9 albums"),
            ("Marine Drive", "4 albums"),
            ("Otto Lang", "2 albums"),
        ],
        2 => [
            ("Late shift", "31 tracks"),
            ("Long drive", "64 tracks"),
            ("Rain on glass", "18 tracks"),
        ],
        _ => [
            ("Blue Static", "Afterglow"),
            ("Harbour Lights", "Marine Drive"),
            ("Tape Hiss", "Otto Lang"),
        ],
    }
}

#[component]
fn QueueScreen() -> Element {
    rsx! {
        TabLayout { route_transition_base: false,
            Header {
                title: "Queue",
                start: rsx! {
                    g3_ui::BackButton {
                        label: "Listen",
                        onclick: move |_| {
                            spawn(go(Route::TransitionListen {}));
                        },
                    }
                },
            }
            Content { footer_space: false,
                TransitionNote { "A sheet. It rose over Listen, which stayed put and dimmed behind it." }
                Card { title: "Up next",
                    p { "Three tracks left in this queue." }
                }
                List { variant: g3_ui::ListVariant::Raised, lines: ListLines::Inset,
                    Item { label: "Paper Moon", description: "Afterglow", metadata: "3:05" }
                    Item { label: "Night Ferry", description: "Marine Drive", metadata: "5:22" }
                }
                Button {
                    expand: ButtonExpand::Block,
                    onclick: move |_| async move { go(Route::TransitionListen {}).await },
                    "Done"
                }
            }
            // The sheet covers the bottom tabs on a phone, but a desktop rail
            // stays beside it.
            ShowcaseNav { selected: ShowcaseScreen::Queue, compact: AdaptiveNavCompact::Hidden }
        }
    }
}

#[component]
fn ShowcaseNav(selected: ShowcaseScreen, compact: Option<AdaptiveNavCompact>) -> Element {
    rsx! {
        AdaptiveNav { aria_label: "Transition demo navigation", compact,
            NavItem {
                label: "Listen",
                selected: matches!(selected, ShowcaseScreen::Listen | ShowcaseScreen::Queue | ShowcaseScreen::Album),
                icon: rsx! { Headphones { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionListen {}));
                },
            }
            NavItem {
                label: "Library",
                selected: matches!(selected, ShowcaseScreen::Library(_)),
                icon: rsx! { Library { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionLibrary { tab: 0 }));
                },
            }
            NavItem {
                label: "Radio",
                selected: matches!(selected, ShowcaseScreen::Radio | ShowcaseScreen::Episode),
                icon: rsx! { Radio { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionRadio {}));
                },
            }
            NavItem {
                label: "Profile",
                selected: selected == ShowcaseScreen::Profile,
                group: NavItemGroup::Secondary,
                icon: rsx! { CircleUserRound { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionProfile {}));
                },
            }
        }
    }
}
