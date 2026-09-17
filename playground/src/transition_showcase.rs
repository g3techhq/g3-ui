//! Route-transition showcase composed entirely from public g3-ui components.
use dioxus::prelude::*;
use dioxus_icons::lucide::{BookOpen, CircleUserRound, House, Star};
use g3_route_transitions::{RouteTransitionPage, animated_navigate};
use g3_ui::{
    AdaptiveNav, AdaptiveNavCompact, AppWrapper, Badge, Button, ButtonExpand, ButtonFill,
    ButtonSize, Card, Color, Content, Header, Item, ItemDetail, List, ListLines, NavItem,
    NavItemGroup, PlaygroundDemoFrame, SegmentButton, SegmentGroup, TabLayout, ThemeProvider,
};

use super::{PlaygroundViewport, Route, active_playground_settings};

pub const DESCRIPTION: &str = "Real g3-ui app screens that demonstrate route-owned stack, sheet, cross-fade, segmented, and drill-down transitions.";
pub const SOURCE: &str = r#"#[derive(Clone, Debug, PartialEq, Routable, RouteTransitions)]
enum Route {
    // Tab roots cross-fade between each other.
    #[transition(layer = stack_root)]
    #[route("/transitions")]
    Home {},

    // Home -> Detail slides forward; Back slides backward.
    #[transition(layer = stack_page)]
    #[route("/transitions/detail")]
    Detail {},

    // Rises over the current page; Back drops it away.
    #[transition(layer = sheet)]
    #[route("/transitions/queue")]
    Queue {},

    // Segments slide by tab order without adding history entries.
    #[transition(history = replace, peers(group = ratings, order = tab))]
    #[route("/transitions/ratings/:tab")]
    Ratings { tab: u8 },

    // A base page that drills into a stack page.
    #[transition(forward_to = ArticleDetail)]
    #[route("/transitions/article")]
    Article {},

    #[transition(layer = stack_page)]
    #[route("/transitions/article/read")]
    ArticleDetail {},
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
                Header { title: "Fairway" }
                Content { Card { title: "Today's round", "Ready to play" } }
            }
            // Persistent chrome: a desktop rail stays still in every transition.
            AdaptiveNav { /* NavItem { to: Route::Home {}, .. } */ }
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

#[component]
fn TransitionScreen(screen: ShowcaseScreen) -> Element {
    match screen {
        ShowcaseScreen::Queue => rsx! { QueueScreen {} },
        ShowcaseScreen::Ratings(tab) => rsx! { RatingsScreen { tab } },
        _ => rsx! {
            TabLayout {
                RouteTransitionPage { class: "transition-showcase-page".to_string(),
                    Header {
                        title: screen_title(screen),
                        start: matches!(screen, ShowcaseScreen::Detail | ShowcaseScreen::ArticleDetail)
                            .then(|| rsx! {
                                g3_ui::BackButton {
                                    onclick: move |_| {
                                        let destination = if screen == ShowcaseScreen::ArticleDetail {
                                            Route::TransitionArticle {}
                                        } else {
                                            Route::TransitionHome {}
                                        };
                                        spawn(go(destination));
                                    },
                                }
                            }),
                    }
                    Content { footer_space: false,
                        match screen {
                            ShowcaseScreen::Home => rsx! { HomeContent {} },
                            ShowcaseScreen::Detail => rsx! { DetailContent {} },
                            ShowcaseScreen::Profile => rsx! { ProfileContent {} },
                            ShowcaseScreen::Article => rsx! { ArticleContent {} },
                            ShowcaseScreen::ArticleDetail => rsx! { ArticleDetailContent {} },
                            ShowcaseScreen::Queue | ShowcaseScreen::Ratings(_) => rsx! {},
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
        ShowcaseScreen::Detail => "Round details",
        ShowcaseScreen::Profile => "Profile",
        ShowcaseScreen::Article | ShowcaseScreen::ArticleDetail => "Club stories",
        ShowcaseScreen::Home | ShowcaseScreen::Queue | ShowcaseScreen::Ratings(_) => "Fairway",
    }
}

#[component]
fn HomeContent() -> Element {
    rsx! {
        Card {
            title: "Today at Pebble Creek",
            end: rsx! { "10:40 AM" },
            p { "Four players · Match play · White tees" }
            div { class: "transition-showcase-actions",
                Button {
                    size: ButtonSize::Sm,
                    onclick: move |_| async move { go(Route::TransitionDetail {}).await },
                    "View round"
                }
                Button {
                    size: ButtonSize::Sm,
                    fill: ButtonFill::Outline,
                    onclick: move |_| async move { go(Route::TransitionQueue {}).await },
                    "Add players"
                }
            }
        }
        List { inset: true, lines: ListLines::Inset,
            Item {
                label: "Jordan Diaz",
                description: "Ready to play",
                end: rsx! { Badge { color: Color::Success, "Ready" } },
            }
            Item { label: "Sam Meyer", description: "12.4 handicap", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn DetailContent() -> Element {
    rsx! {
        Card { title: "Match play", end: rsx! { "Thru 12" },
            p { "Jordan leads 2 up with six holes remaining." }
        }
        List { inset: true, lines: ListLines::Full,
            Item { label: "Front nine", metadata: "+1" }
            Item { label: "Back nine", metadata: "2 up" }
            Item { label: "Course handicap", metadata: "12" }
        }
    }
}

#[component]
fn ProfileContent() -> Element {
    rsx! {
        Card { title: "Matthew W.",
            p { "42 rounds · 12.1 handicap" }
            Badge { color: Color::Accent, "Following 18" }
        }
        List { inset: true,
            Item { label: "Achievements", detail: ItemDetail::Show }
            Item { label: "Round history", metadata: "42", detail: ItemDetail::Show }
            Item { label: "Settings", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn ArticleContent() -> Element {
    rsx! {
        Card {
            title: "Where the light stays",
            end: rsx! { "5 min" },
            onclick: move |_| async move { go(Route::TransitionArticleDetail {}).await },
            p { "A quiet walk through the closing holes as the course settles into evening." }
        }
        List { inset: true,
            Item { label: "The architecture of risk", description: "Course design", detail: ItemDetail::Show }
            Item { label: "Playing into the wind", description: "Field notes", detail: ItemDetail::Show }
        }
    }
}

#[component]
fn ArticleDetailContent() -> Element {
    rsx! {
        Card { title: "Where the light stays",
            p { class: "transition-showcase-article",
                "By the time the final group reaches seventeen, the long shadows have crossed the fairway. The course feels quieter, and every decision becomes wonderfully simple."
            }
        }
    }
}

#[component]
fn RatingsScreen(tab: u8) -> Element {
    let mut selected = use_signal(|| tab);
    use_effect(use_reactive!(|tab| selected.set(tab)));
    rsx! {
        TabLayout {
            Header {
                title: "Ratings",
                toolbar: rsx! {
                    SegmentGroup {
                        value: selected,
                        aria_label: "Ratings",
                        defer_selection: true,
                        onchange: move |next: u8| {
                            spawn(go(Route::TransitionRatings { tab: next }));
                        },
                        SegmentButton { value: 0_u8, "Courses" }
                        SegmentButton { value: 1_u8, "Players" }
                        SegmentButton { value: 2_u8, "Rounds" }
                    }
                },
            }
            Content { footer_space: false,
                Card {
                    title: match tab { 1 => "Top players", 2 => "Best rounds", _ => "Top courses" },
                    end: rsx! { {match tab { 1 => "9.2", 2 => "-4", _ => "8.9" }} },
                    p {
                        match tab {
                            1 => "Players you follow, ranked by recent form.",
                            2 => "Standout rounds from your golfing circle.",
                            _ => "Courses ranked from your recent ratings.",
                        }
                    }
                }
                List { inset: true, lines: ListLines::Inset,
                    for (name, value) in rating_rows(tab) {
                        Item { key: "{name}", label: name, metadata: value }
                    }
                }
            }
            ShowcaseNav { selected: ShowcaseScreen::Ratings(tab) }
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
fn QueueScreen() -> Element {
    rsx! {
        TabLayout { route_transition_base: false,
            Header {
                title: "Add players",
                start: rsx! {
                    g3_ui::BackButton {
                        label: "Fairway",
                        onclick: move |_| {
                            spawn(go(Route::TransitionHome {}));
                        },
                    }
                },
            }
            Content { footer_space: false,
                Card { title: "Available for 10:40 AM",
                    p { "Choose players to add to the Pebble Creek round." }
                }
                List { inset: true, lines: ListLines::Inset,
                    Item { label: "Morgan Lee", description: "Available at 10:40" }
                    Item { label: "Taylor Kim", description: "Usually walks" }
                }
                Button {
                    expand: ButtonExpand::Block,
                    onclick: move |_| async move { go(Route::TransitionHome {}).await },
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
                label: "Home",
                selected: matches!(selected, ShowcaseScreen::Home | ShowcaseScreen::Queue | ShowcaseScreen::Detail),
                icon: rsx! { House { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionHome {}));
                },
            }
            NavItem {
                label: "Ratings",
                selected: matches!(selected, ShowcaseScreen::Ratings(_)),
                icon: rsx! { Star { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionRatings { tab: 0 }));
                },
            }
            NavItem {
                label: "Stories",
                selected: matches!(selected, ShowcaseScreen::Article | ShowcaseScreen::ArticleDetail),
                icon: rsx! { BookOpen { size: 20 } },
                onclick: move |_| {
                    spawn(go(Route::TransitionArticle {}));
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
