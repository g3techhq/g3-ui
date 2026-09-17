//! Fairway: a round-of-golf app built from the library, for the playground.
//!
//! Each tab is one kind of screen a real app has, so the components land where
//! they belong rather than being piled together: a feed that refreshes, a form
//! that books something, an edge-to-edge inbox, and a settings page.
#![cfg(feature = "playground")]
use crate::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::{
    Bell, BookOpen, CalendarDays, CircleUserRound, Flag, House, LogOut, MapPin, Menu as MenuIcon,
    Plus, RotateCcw, Search, SlidersHorizontal, Star, Trophy, UserPlus,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Screen {
    Today,
    Book,
    Activity,
    Profile,
}

impl Screen {
    fn title(self) -> &'static str {
        match self {
            Screen::Today => "Today",
            Screen::Book => "Book a tee time",
            Screen::Activity => "Activity",
            Screen::Profile => "Profile",
        }
    }
}

#[component]
fn DemoAppPlaygroundDemo() -> Element {
    let mode = use_component_mode(None);
    let mut screen = use_signal(|| Screen::Today);
    let mut drawer_open = use_signal(|| false);
    let mut filters_open = use_signal(|| false);
    // Owned here because they live in the header's toolbar, above the page.
    let when = use_signal(|| 0_usize);
    let query = use_signal(String::new);
    let nearby = use_signal(|| true);
    let friends = use_signal(|| false);
    let within = use_signal(|| 25.0_f64);
    let filters = nearby() as u8 + friends() as u8;
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
                // Everything the tab bar has no room for.
                SideSheet { open: drawer_open, title: "Fairway",
                    List { lines: ListLines::None,
                        Item {
                            label: "Matthew W.",
                            description: "Handicap 12.4",
                            start: rsx! { Avatar { name: "Matthew W." } },
                        }
                        Divider {}
                        Item {
                            label: "Saved courses",
                            metadata: "6",
                            start: rsx! { Star { size: 20 } },
                            onclick: move |_| drawer_open.set(false),
                        }
                        Item {
                            label: "Handicap history",
                            start: rsx! { Trophy { size: 20 } },
                            onclick: move |_| drawer_open.set(false),
                        }
                        Item {
                            label: "Rules of golf",
                            start: rsx! { BookOpen { size: 20 } },
                            onclick: move |_| drawer_open.set(false),
                        }
                        Divider {}
                        Item {
                            label: "Sign out",
                            start: rsx! { LogOut { size: 20 } },
                            onclick: move |_| drawer_open.set(false),
                        }
                    }
                }
                TabLayout {
                    Header {
                        title: screen().title(),
                        start: rsx! {
                            Button {
                                fill: ButtonFill::Clear,
                                size: ButtonSize::Sm,
                                aria_label: "Open menu",
                                onclick: move |_| drawer_open.set(true),
                                MenuIcon { size: 22 }
                            }
                        },
                        // Only the booking screen has anything to filter.
                        end: (screen() == Screen::Book).then(|| rsx! {
                            Button {
                                fill: ButtonFill::Clear,
                                size: ButtonSize::Sm,
                                aria_label: "Filters",
                                onclick: move |_| filters_open.set(true),
                                SlidersHorizontal { size: 20 }
                                if filters > 0 {
                                    Badge { color: Color::Accent, "{filters}" }
                                }
                            }
                        }),
                        // The second row belongs to the screen below it: the
                        // feed switches between two lists, the booking screen
                        // searches courses.
                        toolbar: match screen() {
                            Screen::Today => rsx! {
                                SegmentGroup { value: when, aria_label: "Rounds",
                                    SegmentButton { value: 0_usize, "Live" }
                                    SegmentButton { value: 1_usize, "Upcoming" }
                                }
                            },
                            Screen::Book => rsx! {
                                Searchbar { value: query, placeholder: "Search courses" }
                            },
                            Screen::Activity | Screen::Profile => rsx! {},
                        },
                    }
                    match screen() {
                        Screen::Today => rsx! { TodayScreen { when } },
                        Screen::Book => rsx! { BookScreen { within } },
                        Screen::Activity => rsx! { ActivityScreen {} },
                        Screen::Profile => rsx! { ProfileScreen {} },
                    }
                    AdaptiveNav {
                        {nav_item(Screen::Today, "Today", rsx! { House { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Book, "Book", rsx! { Search { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Activity, "Activity", rsx! { Bell { size: 20 } }, NavItemGroup::Primary)}
                        {nav_item(Screen::Profile, "Profile", rsx! { CircleUserRound { size: 20 } }, NavItemGroup::Secondary)}
                    }
                }
                FiltersSheet { open: filters_open, nearby, friends, within }
            }
        }
    }
}

/// The feed: what is happening now, or what is booked next. Pulls to refresh
/// and carries the app's one create action.
#[component]
fn TodayScreen(when: Signal<usize>) -> Element {
    let mut refreshing = use_signal(|| false);
    let toaster = use_toast();
    let actions = use_action_sheet();
    rsx! {
        Content {
            refreshing: refreshing(),
            on_refresh: move |_| {
                refreshing.set(true);
                spawn(async move {
                    dioxus_sdk_time::sleep(std::time::Duration::from_millis(900)).await;
                    refreshing.set(false);
                    toaster.show("Scores up to date");
                });
            },
            fab: rsx! {
                FabMenu {
                    aria_label: "Create",
                    icon: rsx! { Plus { size: 24 } },
                    FabButton {
                        size: FabSize::Small,
                        aria_label: "Start a round",
                        onclick: move |_| {
                            toaster.success("Round started");
                        },
                        Flag { size: 18 }
                    }
                    FabButton { size: FabSize::Small, aria_label: "Invite a player",
                        UserPlus { size: 18 }
                    }
                }
            },
            if when() == 0 {
                Stack { gap: Space::Lg,
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
                    List { variant: ListVariant::Raised,
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
            } else {
                List { variant: ListVariant::Raised,
                    ListHeader { "Booked" }
                    for (course, day, time) in [
                        ("Pebble Creek", "Saturday", "10:40 AM"),
                        ("Oak Hollow", "Sunday", "7:20 AM"),
                        ("Mill Ridge", "Next Thursday", "4:05 PM"),
                    ] {
                        Item {
                            key: "{course}{day}",
                            label: course,
                            description: day,
                            metadata: time,
                            start: rsx! { CalendarDays { size: 20 } },
                            detail: ItemDetail::Show,
                            onclick: move |_| {},
                        }
                    }
                }
            }
        }
    }
}

/// The form screen: choose a course and a time, then ask for it.
#[component]
fn BookScreen(within: Signal<f64>) -> Element {
    let alerts = use_alert();
    let toaster = use_toast();
    let course = use_signal(|| "pebble".to_string());
    let format = use_signal(|| Some("stroke"));
    let walking = use_signal(|| true);
    let players = use_signal(|| 4_i64);
    let date = use_signal(|| None::<CalendarDate>);
    let time = use_signal(|| None::<TimeOfDay>);
    let mut weekend = use_signal(|| true);
    let mut twilight = use_signal(|| false);
    rsx! {
        Content {
            Stack { gap: Space::Lg,
                Stack { horizontal: true, wrap: true, gap: Space::Sm,
                    Chip { start: rsx! { MapPin { size: 14 } }, "Within {within() as i64} mi" }
                    Chip {
                        selected: weekend(),
                        onclick: move |_| weekend.toggle(),
                        "This weekend"
                    }
                    Chip {
                        selected: twilight(),
                        onclick: move |_| twilight.toggle(),
                        "Twilight rate"
                    }
                }
                Card { title: "Tee time",
                    Stack {
                        Select {
                            label: "Course",
                            value: course,
                            options: vec![
                                SelectOption::new("pebble".to_string(), "Pebble Creek"),
                                SelectOption::new("oak".to_string(), "Oak Hollow").description("Closed Mondays"),
                                SelectOption::new("mill".to_string(), "Mill Ridge"),
                            ],
                        }
                        DatePicker { label: "Day", value: date, min: CalendarDate::today() }
                        TimePicker { label: "Time", value: time, minute_step: 10 }
                        Stepper { label: "Players", value: players, min: 1, max: 4 }
                        RadioGroup { value: format, label: "Format",
                            Radio { value: "stroke", label: "Stroke play" }
                            Radio { value: "match", label: "Match play" }
                        }
                        Toggle { checked: walking, label: "Walking", helper: "No cart needed" }
                    }
                }
                Button {
                    expand: ButtonExpand::Block,
                    onclick: move |_| async move {
                        let confirmed = alerts
                            .confirm("Request this tee time?", "The pro shop confirms by email.")
                            .await;
                        if confirmed {
                            toaster.success("Tee time requested");
                        }
                    },
                    "Request tee time"
                }
                AccordionGroup::<&'static str> {
                    AccordionItem { value: "rules", label: "Local rules", "Lift, clean, and place in the fairway." }
                    AccordionItem { value: "pace", label: "Pace of play", "Four hours fifteen for eighteen holes." }
                    AccordionItem { value: "dress", label: "Dress code", "Collared shirts. Soft spikes only." }
                }
            }
        }
    }
}

/// The inbox: rows that run to both edges, because nothing insets them.
#[component]
fn ActivityScreen() -> Element {
    let initial = || {
        vec![
            ("Birdie on 7", "Grace Park saw your card"),
            ("Invite from Grace", "Sunday at Oak Hollow"),
            ("Round saved", "Pebble Creek · 82"),
        ]
    };
    let mut rows = use_signal(initial);
    let mut dismissed = use_signal(Vec::<(&'static str, &'static str)>::new);
    rsx! {
        // `padding: false` is what lets an edge-to-edge list reach the edges.
        Content { padding: false,
            List {
                ListHeader { "This week" }
                for (label, description) in rows() {
                    SwipeItem {
                        key: "{label}",
                        end_behavior: SwipeBehavior::Dismiss,
                        end_actions: rsx! {
                            SwipeAction {
                                color: Color::Danger,
                                onclick: move |_| dismiss(&mut rows, &mut dismissed, label),
                                "Delete"
                            }
                        },
                        on_dismiss: move |_| dismiss(&mut rows, &mut dismissed, label),
                        Item {
                            label,
                            description,
                            start: rsx! { Bell { size: 20 } },
                        }
                    }
                }
                if rows().is_empty() {
                    Item { label: "All caught up", description: "Nothing new this week" }
                }
            }
            if !dismissed().is_empty() {
                List { lines: ListLines::None,
                    Item {
                        label: "Undo delete",
                        metadata: "{dismissed().len()}",
                        start: rsx! { RotateCcw { size: 20 } },
                        onclick: move |_| {
                            let mut restored = dismissed.take();
                            restored.append(&mut rows.write());
                            rows.set(restored);
                        },
                    }
                }
            }
        }
    }
}

fn dismiss(
    rows: &mut Signal<Vec<(&'static str, &'static str)>>,
    dismissed: &mut Signal<Vec<(&'static str, &'static str)>>,
    label: &'static str,
) {
    let Some(at) = rows.peek().iter().position(|(row, _)| *row == label) else {
        return;
    };
    let row = rows.write().remove(at);
    dismissed.write().push(row);
}

/// The settings screen: who you are, and the switches that follow you around.
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
                        Text { variant: TextVariant::Caption, tone: TextTone::Secondary, "Handicap 12.4 · 42 rounds" }
                    }
                }
                List { variant: ListVariant::Raised,
                    Item {
                        label: "Notifications",
                        end: rsx! { Toggle { checked: notifications, aria_label: "Notifications" } },
                    }
                    Item { label: "Home course", metadata: "Pebble Creek", detail: ItemDetail::Show, onclick: |_| {} }
                    Item { label: "Handicap index", metadata: "12.4", detail: ItemDetail::Show, onclick: |_| {} }
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

/// The booking screen's filters, in a sheet because they modify the page
/// behind them rather than replacing it.
#[component]
fn FiltersSheet(
    open: Signal<bool>,
    nearby: Signal<bool>,
    friends: Signal<bool>,
    within: Signal<f64>,
) -> Element {
    rsx! {
        BottomSheet { open, title: "Filters",
            Stack {
                Toggle { checked: nearby, label: "Nearby courses" }
                Range {
                    label: "Distance",
                    value: within,
                    min: 5.0,
                    max: 50.0,
                    step: 5.0,
                    disabled: !nearby(),
                    show_value: true,
                    helper: format!("Within {} miles", within() as i64),
                }
                Toggle { checked: friends, label: "Friends playing" }
                Button { expand: ButtonExpand::Block, onclick: move |_| open.set(false), "Show courses" }
            }
        }
    }
}

crate::g3_playground! {
    name: "DemoApp",
    description: "One app, four screen types: a feed, a form, an inbox, and settings.",
    components: [],
    demo: DemoAppPlaygroundDemo,
    source: "src/components/demo_app.rs",
}
