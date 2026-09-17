//! A kitchen-sink app that composes the whole library, for the playground.
#![cfg(feature = "playground")]
use crate::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Activity, Bell, CalendarDays, CircleUserRound, Flag, House, LogOut, MapPin, Menu as MenuIcon,
    Plus, Search, SlidersHorizontal, UserPlus,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Screen {
    Rounds,
    Discover,
    Profile,
    Activity,
}

#[component]
fn DemoAppPlaygroundDemo() -> Element {
    let mode = use_component_mode(None);
    let mut screen = use_signal(|| Screen::Rounds);
    let mut drawer_open = use_signal(|| false);
    let mut filters_open = use_signal(|| false);
    let title = match screen() {
        Screen::Rounds => "Fairway",
        Screen::Discover => "Discover",
        Screen::Profile => "Profile",
        Screen::Activity => "Activity",
    };
    let nav_item =
        move |target: Screen, label: &'static str, icon: Element, group: NavItemGroup| {
            rsx! {
                NavItem {
                    label,
                    icon,
                    group,
                    selected: screen() == target,
                    onclick: move |_| screen.set(target),
                }
            }
        };
    rsx! {
        PlaygroundDemoFrame { app: false,
            AppWrapper { mode, class: "g3-playground-device-app",
                SideSheet { open: drawer_open, title: "Fairway",
                    List { lines: ListLines::None,
                        Item {
                            label: "Rounds",
                            start: rsx! { House { size: 20 } },
                            onclick: move |_| {
                                screen.set(Screen::Rounds);
                                drawer_open.set(false);
                            },
                        }
                        Item {
                            label: "Discover",
                            start: rsx! { Search { size: 20 } },
                            onclick: move |_| {
                                screen.set(Screen::Discover);
                                drawer_open.set(false);
                            },
                        }
                        Item {
                            label: "Sign out",
                            start: rsx! { LogOut { size: 20 } },
                            onclick: move |_| drawer_open.set(false),
                        }
                    }
                }
                TabLayout {
                    Header {
                        title,
                        start: rsx! {
                            Button {
                                fill: ButtonFill::Clear,
                                size: ButtonSize::Sm,
                                aria_label: "Open menu",
                                onclick: move |_| drawer_open.set(true),
                                MenuIcon { size: 22 }
                            }
                        },
                        end: rsx! {
                            Button {
                                fill: ButtonFill::Clear,
                                size: ButtonSize::Sm,
                                aria_label: "Filters",
                                onclick: move |_| filters_open.set(true),
                                SlidersHorizontal { size: 20 }
                            }
                        },
                    }
                    match screen() {
                        Screen::Rounds => rsx! { RoundsScreen {} },
                        Screen::Discover => rsx! { DiscoverScreen {} },
                        Screen::Profile => rsx! { ProfileScreen {} },
                        Screen::Activity => rsx! { ActivityScreen {} },
                    }
                    AdaptiveNav {
                        {nav_item(Screen::Rounds, "Rounds", rsx! { House { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Discover, "Discover", rsx! { Search { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Activity, "Activity", rsx! { Activity { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Profile, "Profile", rsx! { CircleUserRound { size: 20 } }, NavItemGroup::Secondary)}
                    }
                }
                FiltersSheet { open: filters_open }
            }
        }
    }
}

#[component]
fn RoundsScreen() -> Element {
    let view = use_signal(|| 0_usize);
    let mut refreshing = use_signal(|| false);
    let toaster = use_toast();
    let actions = use_action_sheet();
    rsx! {
        Content {
            fab: rsx! {
                FabMenu {
                    aria_label: "Create",
                    icon: rsx! { Plus { size: 24 } },
                    FabButton {
                        size: FabSize::Small,
                        aria_label: "New round",
                        onclick: move |_| {
                            toaster.success("Round created");
                        },
                        Flag { size: 18 }
                    }
                    FabButton { size: FabSize::Small, aria_label: "Invite player",
                        UserPlus { size: 18 }
                    }
                }
            },
            Refresher {
                refreshing: refreshing(),
                on_refresh: move |_| {
                    refreshing.set(true);
                    spawn(async move {
                        dioxus_sdk_time::sleep(std::time::Duration::from_millis(900)).await;
                        refreshing.set(false);
                    });
                },
                Stack {
                    SegmentGroup { value: view, aria_label: "Rounds",
                        SegmentButton { value: 0_usize, "Live" }
                        SegmentButton { value: 1_usize, "Upcoming" }
                    }
                    Card {
                        title: "Saturday four-ball",
                        subtitle: "Pebble Creek · hole 7",
                        end: rsx! { Badge { color: Color::Success, "-2" } },
                        onclick: move |_| {
                            spawn(async move {
                                let choice = actions
                                    .show(ActionSheetOptions {
                                        title: Some("Saturday four-ball".into()),
                                        message: None,
                                        buttons: vec![
                                            ActionSheetButton::new("Share scorecard"),
                                            ActionSheetButton::destructive("Leave round"),
                                        ],
                                    })
                                    .await;
                                if choice == Some(0) {
                                    toaster.show("Scorecard link copied");
                                }
                            });
                        },
                        Progress { value: 7.0, max: 18.0, label: "Holes played", value_text: "7 of 18" }
                    }
                    List { variant: crate::ListVariant::Raised,
                        ListHeader { "Leaderboard" }
                        for (name, score) in [("Alex Morgan", "-3"), ("Grace Park", "E"), ("Sam Ortiz", "+1")] {
                            Item {
                                key: "{name}",
                                label: name,
                                start: rsx! { Avatar { name } },
                                metadata: score,
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DiscoverScreen() -> Element {
    let query = use_signal(String::new);
    let course = use_signal(|| "pebble".to_string());
    let format = use_signal(|| Some("stroke"));
    let walking = use_signal(|| true);
    let players = use_signal(|| 4_i64);
    let date = use_signal(String::new);
    rsx! {
        Content {
            Stack { gap: Space::Lg,
                Searchbar { value: query, placeholder: "Search courses" }
                Stack { horizontal: true, wrap: true, gap: Space::Sm,
                    Chip { start: rsx! { MapPin { size: 14 } }, "Nearby" }
                    Chip { start: rsx! { CalendarDays { size: 14 } }, "This weekend" }
                }
                Card { title: "New round",
                    Stack {
                        Select {
                            label: "Course",
                            value: course,
                            options: vec![
                                SelectOption::new("pebble".to_string(), "Pebble Creek"),
                                SelectOption::new("oak".to_string(), "Oak Hollow").description("Closed Mondays"),
                            ],
                        }
                        Input { label: "Tee date", value: date, input_type: InputType::Date }
                        Stepper { label: "Players", value: players, min: 1, max: 8 }
                        RadioGroup { value: format, label: "Format",
                            Radio { value: "stroke", label: "Stroke play" }
                            Radio { value: "match", label: "Match play" }
                        }
                        Toggle { checked: walking, label: "Walking" }
                    }
                }
                AccordionGroup::<&'static str> {
                    AccordionItem { value: "rules", label: "Local rules", "Lift, clean, and place in the fairway." }
                    AccordionItem { value: "pace", label: "Pace of play", "Four hours fifteen for eighteen holes." }
                }
            }
        }
    }
}

#[component]
fn ProfileScreen() -> Element {
    let alerts = use_alert();
    let toaster = use_toast();
    let notifications = use_signal(|| true);
    rsx! {
        Content {
            Stack { gap: Space::Lg,
                Stack { horizontal: true, align: StackAlign::Center,
                    Avatar { name: "Matthew W.", size: AvatarSize::Lg }
                    Stack { gap: Space::Xs,
                        Text { variant: TextVariant::Heading, "Matthew W." }
                        Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Handicap 12.4" }
                    }
                }
                List { variant: crate::ListVariant::Raised,
                    Item {
                        label: "Notifications",
                        end: rsx! { Toggle { checked: notifications, aria_label: "Notifications" } },
                    }
                    Item { label: "Home course", metadata: "Pebble Creek", onclick: |_| {} }
                    Item { label: "Version", metadata: "0.4.0" }
                }
                Button {
                    fill: ButtonFill::Outline,
                    color: Color::Danger,
                    expand: ButtonExpand::Block,
                    onclick: move |_| async move {
                        if alerts.confirm("Sign out?", "You can sign back in any time.").await {
                            toaster.show("Signed out");
                        }
                    },
                    "Sign out"
                }
            }
        }
    }
}

#[component]
fn ActivityScreen() -> Element {
    let mut rows = use_signal(|| vec!["Birdie on 7", "Invite from Grace", "Round saved"]);
    rsx! {
        Content {
            List {
                for row in rows() {
                    SwipeItem {
                        key: "{row}",
                        end_behavior: SwipeBehavior::Dismiss,
                        end_actions: rsx! {
                            SwipeAction { color: Color::Danger, onclick: move |_| rows.write().retain(|r| *r != row), "Delete" }
                        },
                        on_dismiss: move |_| rows.write().retain(|r| *r != row),
                        Item { label: row, description: "Swipe left to delete", start: rsx! { Bell { size: 20 } } }
                    }
                }
            }
            if rows().is_empty() {
                Text { tone: TextTone::Secondary, "All caught up." }
            }
        }
    }
}

#[component]
fn FiltersSheet(open: Signal<bool>) -> Element {
    let nearby = use_signal(|| true);
    let friends = use_signal(|| false);
    let active = nearby() as u8 + friends() as u8;
    rsx! {
        BottomSheet { open, title: "Filters",
            Stack {
                Toggle { checked: nearby, label: "Nearby courses", helper: "Within 25 miles" }
                Toggle { checked: friends, label: "Friends only" }
                Button { expand: ButtonExpand::Block, onclick: move |_| open.set(false), "Apply ({active})" }
            }
        }
    }
}

crate::g3_playground! {
    name: "Demo App",
    description: "A small app composed from the whole library.",
    components: [],
    demo: DemoAppPlaygroundDemo,
    source: "src/components/demo_app.rs",
}
