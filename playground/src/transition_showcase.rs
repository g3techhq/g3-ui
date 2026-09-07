//! Route-transition showcase composed entirely from public g3-ui components.
use dioxus::prelude::*;
use g3_route_transitions::{RouteTransitionPage, animated_navigate};
use g3_ui::{
    AppWrapper, Badge, Body, Button, ButtonSize, ButtonStyle, Card, ComponentMode, Header, Item,
    ItemDetail, List, ListLines, Navbar, NavbarTab, NavbarTabBar, NavbarTabDesktopPlacement,
    PlaygroundDemoFrame, RightSlot, SegmentButton, SegmentGroup, Sheet, SheetPlacement,
    StatusColor,
};

use super::{PlaygroundViewport, Route, active_playground_settings};

pub const DESCRIPTION: &str = "Real g3-ui app screens demonstrating route-owned push, sheet, fade, segmented, and morph transitions.";
pub const SOURCE: &str = r#"#[route_transitions]
#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[transition(root)]
    #[route("/transitions")]
    Home {},

    #[transition(pushed)]
    #[route("/transitions/detail")]
    Detail {},

    #[transition(cover)]
    #[route("/transitions/queue")]
    Queue {},

    #[transition(base, replace, push(group = ratings, order = tab))]
    #[route("/transitions/ratings/:tab")]
    Ratings { tab: u8 },
}

rsx! {
    G3AppWrapper {
        RouteTransitionPage {
            G3Navbar {
                G3Header { title: "Fairway" }
                G3Body { G3Card { title: "Today's round", "Ready to play" } }
                G3NavbarTabBar { /* routed tabs */ }
            }
        }
    }
}"#;

#[derive(Clone, Copy, PartialEq)]
enum ShowcaseScreen {
    Home,
    Detail,
    Queue,
    Ratings(u8),
    Profile,
    Article,
    ArticleDetail,
}

#[component]
pub fn TransitionHome() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Home } }
}

#[component]
pub fn TransitionDetail() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Detail } }
}

#[component]
pub fn TransitionQueue() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Queue } }
}

#[component]
pub fn TransitionRatings(tab: u8) -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Ratings(tab.min(2)) } }
}

#[component]
pub fn TransitionProfile() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Profile } }
}

#[component]
pub fn TransitionArticle() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::Article } }
}

#[component]
pub fn TransitionArticleDetail() -> Element {
    rsx! { TransitionShowcase { screen: ShowcaseScreen::ArticleDetail } }
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
            div {
                class: "playground-viewport-demo",
                "data-g3-mode": mode.as_str(),
                g3_ui::G3ThemeProvider { mode, theme: theme.to_theme(),
                    PlaygroundDemoFrame {
                        app: false,
                        center: false,
                        class: "transition-showcase-preview",
                        div { class: "transition-showcase-device-frame", aria_hidden: "true" }
                        AppWrapper {
                            mode,
                            class: "g3-playground-device-app transition-showcase-app",
                            TransitionScreen { screen, mode }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TransitionScreen(screen: ShowcaseScreen, mode: ComponentMode) -> Element {
    match screen {
        ShowcaseScreen::Queue => rsx! { QueueScreen { mode } },
        ShowcaseScreen::Ratings(tab) => rsx! { RatingsScreen { tab, mode } },
        _ => rsx! {
            RouteTransitionPage { class: "transition-showcase-page".to_string(),
                Navbar { mode,
                    Header {
                        mode,
                        title: screen_title(screen).to_string(),
                        start_button: matches!(screen, ShowcaseScreen::Detail | ShowcaseScreen::ArticleDetail)
                            .then(|| rsx! {
                                Button {
                                    mode,
                                    style: ButtonStyle::Clear,
                                    size: ButtonSize::Sm,
                                    aria_label: "Back".to_string(),
                                    onclick: move |_| async move {
                                        let destination = if screen == ShowcaseScreen::ArticleDetail {
                                            Route::TransitionArticle {}
                                        } else {
                                            Route::TransitionHome {}
                                        };
                                        animated_navigate(destination).await;
                                    },
                                    "Back"
                                }
                            }),
                    }
                    Body { mode, has_footer_space: false,
                        match screen {
                            ShowcaseScreen::Home => rsx! { HomeContent { mode } },
                            ShowcaseScreen::Detail => rsx! { DetailContent { mode } },
                            ShowcaseScreen::Profile => rsx! { ProfileContent { mode } },
                            ShowcaseScreen::Article => rsx! { ArticleContent { mode } },
                            ShowcaseScreen::ArticleDetail => rsx! { ArticleDetailContent { mode } },
                            ShowcaseScreen::Queue | ShowcaseScreen::Ratings(_) => rsx! {},
                        }
                    }
                    ShowcaseTabs { selected: screen }
                }
            }
        },
    }
}

fn screen_title(screen: ShowcaseScreen) -> &'static str {
    match screen {
        ShowcaseScreen::Detail => "Round details",
        ShowcaseScreen::Profile => "Profile",
        ShowcaseScreen::Article | ShowcaseScreen::ArticleDetail => "Club stories",
        ShowcaseScreen::Home | ShowcaseScreen::Queue | ShowcaseScreen::Ratings(_) => "Fairway",
    }
}

#[component]
fn HomeContent(mode: ComponentMode) -> Element {
    rsx! {
        Card {
            mode,
            title: "Today at Pebble Creek".to_string(),
            right_slot: RightSlot::Text("10:40 AM".to_string()),
            p { "Four players · Match play · White tees" }
            div { class: "transition-showcase-actions",
                Button {
                    mode,
                    size: ButtonSize::Sm,
                    onclick: move |_| async move { animated_navigate(Route::TransitionDetail {}).await },
                    "View round"
                }
                Button {
                    mode,
                    size: ButtonSize::Sm,
                    style: ButtonStyle::Outline,
                    onclick: move |_| async move { animated_navigate(Route::TransitionQueue {}).await },
                    "Add players"
                }
            }
        }
        List { mode, inset: true, lines: ListLines::Inset,
            Item {
                mode,
                label: "Jordan Diaz".to_string(),
                description: "Ready to play".to_string(),
                end: rsx! { Badge { color: StatusColor::Success, "Ready" } },
            }
            Item {
                mode,
                label: "Sam Meyer".to_string(),
                description: "12.4 handicap".to_string(),
                detail: ItemDetail::Show,
            }
        }
    }
}

#[component]
fn DetailContent(mode: ComponentMode) -> Element {
    rsx! {
        Card {
            mode,
            title: "Match play".to_string(),
            right_slot: RightSlot::Text("Thru 12".to_string()),
            p { "Jordan leads 2 up with six holes remaining." }
        }
        List { mode, inset: true, lines: ListLines::Full,
            Item { mode, label: "Front nine".to_string(), metadata: "+1".to_string() }
            Item { mode, label: "Back nine".to_string(), metadata: "2 up".to_string() }
            Item { mode, label: "Course handicap".to_string(), metadata: "12".to_string() }
        }
    }
}

#[component]
fn ProfileContent(mode: ComponentMode) -> Element {
    rsx! {
        Card { mode, title: "Matthew W.".to_string(),
            p { "42 rounds · 12.1 handicap" }
            Badge { color: StatusColor::Accent, "Following 18" }
        }
        List { mode, inset: true,
            Item { mode, label: "Achievements".to_string(), detail: ItemDetail::Show }
            Item { mode, label: "Round history".to_string(), metadata: "42".to_string(), detail: ItemDetail::Show }
            Item { mode, label: "Settings".to_string(), detail: ItemDetail::Show }
        }
    }
}

#[component]
fn ArticleContent(mode: ComponentMode) -> Element {
    rsx! {
        Card {
            mode,
            title: "Where the light stays".to_string(),
            right_slot: RightSlot::Text("5 min".to_string()),
            onclick: move |_| async move { animated_navigate(Route::TransitionArticleDetail {}).await },
            p { "A quiet walk through the closing holes as the course settles into evening." }
        }
        List { mode, inset: true,
            Item { mode, label: "The architecture of risk".to_string(), description: "Course design".to_string(), detail: ItemDetail::Show }
            Item { mode, label: "Playing into the wind".to_string(), description: "Field notes".to_string(), detail: ItemDetail::Show }
        }
    }
}

#[component]
fn ArticleDetailContent(mode: ComponentMode) -> Element {
    rsx! {
        Card { mode, title: "Where the light stays".to_string(),
            p { class: "transition-showcase-article",
                "By the time the final group reaches seventeen, the long shadows have crossed the fairway. The course feels quieter, and every decision becomes wonderfully simple."
            }
        }
    }
}

#[component]
fn RatingsScreen(tab: u8, mode: ComponentMode) -> Element {
    let active = use_signal(|| tab as usize);
    rsx! {
        Navbar { mode,
            Header {
                mode,
                title: "Ratings".to_string(),
                toolbar: rsx! {
                    SegmentGroup {
                        mode,
                        active,
                        defer_active: true,
                        on_change: move |next: usize| {
                            spawn(async move {
                                animated_navigate(Route::TransitionRatings { tab: next as u8 }).await;
                            });
                        },
                        SegmentButton { mode, index: 0, "Courses" }
                        SegmentButton { mode, index: 1, "Players" }
                        SegmentButton { mode, index: 2, "Rounds" }
                    }
                },
            }
            Body { mode, has_footer_space: false,
                Card {
                    mode,
                    title: match tab { 1 => "Top players", 2 => "Best rounds", _ => "Top courses" }.to_string(),
                    right_slot: RightSlot::Text(match tab { 1 => "9.2", 2 => "-4", _ => "8.9" }.to_string()),
                    p { match tab {
                        1 => "Players you follow, ranked by recent form.",
                        2 => "Standout rounds from your golfing circle.",
                        _ => "Courses ranked from your recent ratings.",
                    } }
                }
                List { mode, inset: true, lines: ListLines::Inset,
                    for (name, value) in rating_rows(tab) {
                        Item { mode, label: name.to_string(), metadata: value.to_string() }
                    }
                }
            }
            ShowcaseTabs { selected: ShowcaseScreen::Ratings(tab) }
        }
    }
}

fn rating_rows(tab: u8) -> [(&'static str, &'static str); 3] {
    match tab {
        1 => [
            ("Jordan Diaz", "9.2"),
            ("Sam Meyer", "8.8"),
            ("Alex Lin", "8.6"),
        ],
        2 => [
            ("Pebble Creek", "-4"),
            ("Pine Hollow", "-2"),
            ("Ocean Links", "E"),
        ],
        _ => [
            ("The Quiet Course", "8.9"),
            ("Afterglow", "8.7"),
            ("Blue Static", "8.5"),
        ],
    }
}

#[component]
fn QueueScreen(mode: ComponentMode) -> Element {
    let open = use_signal(|| true);
    rsx! {
        Navbar { mode,
            Header { mode, title: "Fairway".to_string() }
            Body { mode, has_footer_space: false, HomeContent { mode } }
            ShowcaseTabs { selected: ShowcaseScreen::Home }
        }
        Sheet { mode, is_open: open, placement: SheetPlacement::Bottom,
            h3 { "Add players" }
            List { mode, lines: ListLines::Inset,
                Item { mode, label: "Morgan Lee".to_string(), description: "Available at 10:40".to_string() }
                Item { mode, label: "Taylor Kim".to_string(), description: "Usually walks".to_string() }
            }
            Button {
                mode,
                expand: true,
                onclick: move |_| async move { animated_navigate(Route::TransitionHome {}).await },
                "Done"
            }
        }
    }
}

#[component]
fn ShowcaseTabs(selected: ShowcaseScreen) -> Element {
    rsx! {
        NavbarTabBar { aria_label: "Transition demo navigation".to_string(),
            NavbarTab {
                label: "Home".to_string(),
                selected: matches!(selected, ShowcaseScreen::Home | ShowcaseScreen::Queue | ShowcaseScreen::Detail),
                onclick: move |_| async move { animated_navigate(Route::TransitionHome {}).await },
            }
            NavbarTab {
                label: "Ratings".to_string(),
                selected: matches!(selected, ShowcaseScreen::Ratings(_)),
                onclick: move |_| async move { animated_navigate(Route::TransitionRatings { tab: 0 }).await },
            }
            NavbarTab {
                label: "Stories".to_string(),
                selected: matches!(selected, ShowcaseScreen::Article | ShowcaseScreen::ArticleDetail),
                onclick: move |_| async move { animated_navigate(Route::TransitionArticle {}).await },
            }
            NavbarTab {
                label: "Profile".to_string(),
                selected: selected == ShowcaseScreen::Profile,
                desktop_placement: NavbarTabDesktopPlacement::Bottom,
                onclick: move |_| async move { animated_navigate(Route::TransitionProfile {}).await },
            }
        }
    }
}
