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
                // `padding: false`: the rows are the navigation, so they
                // press from edge to edge.
                SideSheet { open: drawer_open, title: "Fairway", padding: false,
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
                        // `None`, not an empty element: a header with an
                        // empty second row still reserves its height.
                        toolbar: match screen() {
                            Screen::Today => Some(rsx! {
                                SegmentGroup { value: when, aria_label: "Rounds",
                                    SegmentButton { value: 0_usize, "Live" }
                                    SegmentButton { value: 1_usize, "Upcoming" }
                                }
                            }),
                            Screen::Book => Some(rsx! {
                                Searchbar { value: query, placeholder: "Search courses" }
                            }),
                            Screen::Activity | Screen::Profile => None,
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
                        start: rsx! { Flag { size: 22 } },
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
                    // A scorecard is rows of data, so it is a table. The
                    // names stay in view while the holes scroll past.
                    Table { caption: "Leaderboard", sticky_first_column: true,
                        thead {
                            tr {
                                th { scope: "col", "Player" }
                                for hole in 1..=7 {
                                    th { key: "{hole}", scope: "col", "{hole}" }
                                }
                                th { scope: "col", "Total" }
                            }
                        }
                        tbody {
                            tr {
                                th { scope: "row", "Par" }
                                for par in PARS {
                                    td { "data-muted": "true", "{par}" }
                                }
                                td { "data-muted": "true", "28" }
                            }
                            for (name, strokes, total) in LEADERBOARD {
                                tr { key: "{name}",
                                    th { scope: "row", "{name}" }
                                    for (hole, score) in strokes.iter().enumerate() {
                                        td {
                                            key: "{hole}",
                                            "data-color": score_color(*score, PARS[hole]),
                                            "{score}"
                                        }
                                    }
                                    td { "{total}" }
                                }
                            }
                        }
                    }
                    // A sideways strip: more courses than fit across, each
                    // with its rating shown rather than asked for.
                    Shelf { title: "Courses near you", gap: Space::Md,
                        for (course, rating, reviews) in NEARBY {
                            CourseCard { key: "{course}", course, rating, reviews }
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

/// Par for the first seven holes, which the live round has reached.
const PARS: [u8; 7] = [4, 5, 3, 4, 4, 3, 5];

/// Each player's strokes on those holes, and their score against par.
const LEADERBOARD: [(&str, [u8; 7], &str); 3] = [
    ("Alex Morgan", [4, 4, 3, 3, 4, 3, 4], "-3"),
    ("Grace Park", [4, 5, 3, 4, 4, 3, 5], "E"),
    ("Sam Ortiz", [5, 5, 3, 4, 4, 3, 5], "+1"),
];

/// Nearby courses, with their average rating out of five.
const NEARBY: [(&str, f64, u32); 4] = [
    ("Pebble Creek", 4.5, 212),
    ("Oak Hollow", 3.5, 87),
    ("Mill Ridge", 4.0, 140),
    ("Cedar Point", 5.0, 31),
];

/// One course in the nearby strip. Its own component, so its rating has a
/// signal of its own rather than one made on every render.
#[component]
fn CourseCard(course: &'static str, rating: f64, reviews: u32) -> Element {
    let rating = use_signal(|| rating);
    rsx! {
        Card { class: "w-44", title: course, subtitle: "{reviews} reviews",
            Rating { aria_label: "Average for {course}", value: rating, readonly: true, size: 16 }
        }
    }
}

/// A birdie or better is good news, a bogey or worse is not. The cell says
/// the score as well, since color alone does not reach everyone.
fn score_color(strokes: u8, par: u8) -> Option<&'static str> {
    match strokes.cmp(&par) {
        std::cmp::Ordering::Less => Some(Color::Success.as_str()),
        std::cmp::Ordering::Equal => None,
        std::cmp::Ordering::Greater => Some(Color::Warning.as_str()),
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
/// What a row's two edges do. Between the three rows every
/// [`SwipeBehavior`] appears on both sides.
#[derive(Clone, Copy, PartialEq)]
enum Swipes {
    /// Activate one way, dismiss the other.
    ArchiveOrDelete,
    /// Hold actions open on both sides.
    RevealBoth,
    /// Dismiss one way, activate the other.
    DeleteOrRead,
}

impl Swipes {
    fn start(self) -> SwipeBehavior {
        match self {
            Swipes::ArchiveOrDelete => SwipeBehavior::Activate,
            Swipes::RevealBoth => SwipeBehavior::Reveal,
            Swipes::DeleteOrRead => SwipeBehavior::Dismiss,
        }
    }

    fn end(self) -> SwipeBehavior {
        match self {
            Swipes::ArchiveOrDelete => SwipeBehavior::Dismiss,
            Swipes::RevealBoth => SwipeBehavior::Reveal,
            Swipes::DeleteOrRead => SwipeBehavior::Activate,
        }
    }
}

type Row = (&'static str, &'static str, Swipes);

const ACTIVITY_ROWS: [Row; 3] = [
    (
        "Birdie on 7",
        "Swipe right to archive, left to delete",
        Swipes::ArchiveOrDelete,
    ),
    (
        "Invite from Grace",
        "Swipe either way to uncover buttons",
        Swipes::RevealBoth,
    ),
    (
        "Round saved",
        "Swipe right to delete, left to mark read",
        Swipes::DeleteOrRead,
    ),
];

/// The inbox: rows that run to both edges, because nothing insets them, and
/// that swipe differently on each side.
#[component]
fn ActivityScreen() -> Element {
    let toaster = use_toast();
    let mut rows = use_signal(|| ACTIVITY_ROWS.to_vec());
    let mut gone = use_signal(Vec::<Row>::new);
    let mut take_row = move |label: &'static str| {
        let at = rows.peek().iter().position(|(row, ..)| *row == label);
        if let Some(at) = at {
            let row = rows.write().remove(at);
            gone.write().push(row);
        }
    };
    rsx! {
        // `padding: false` is what lets an edge-to-edge list reach the edges.
        Content { padding: false,
            List {
                ListHeader { "This week" }
                for (label, description, swipes) in rows() {
                    SwipeItem {
                        key: "{label}",
                        start_behavior: swipes.start(),
                        end_behavior: swipes.end(),
                        start_actions: match swipes {
                            Swipes::ArchiveOrDelete => rsx! {
                                SwipeAction { color: Color::Success, "Archive" }
                            },
                            Swipes::RevealBoth => rsx! {
                                SwipeAction {
                                    color: Color::Accent,
                                    onclick: move |_| { toaster.success("Invite accepted"); },
                                    "Accept"
                                }
                            },
                            Swipes::DeleteOrRead => rsx! {
                                SwipeAction { color: Color::Danger, "Delete" }
                            },
                        },
                        end_actions: match swipes {
                            Swipes::ArchiveOrDelete | Swipes::DeleteOrRead => rsx! {
                                SwipeAction {
                                    color: if swipes == Swipes::DeleteOrRead { Color::Accent } else { Color::Danger },
                                    onclick: move |_| {
                                        if swipes == Swipes::DeleteOrRead {
                                            toaster.show(ToastOptions::new("Marked read").replace());
                                        } else {
                                            take_row(label);
                                        }
                                    },
                                    if swipes == Swipes::DeleteOrRead { "Mark read" } else { "Delete" }
                                }
                            },
                            Swipes::RevealBoth => rsx! {
                                SwipeAction {
                                    onclick: move |_| { toaster.show("Muted"); },
                                    "Mute"
                                }
                                SwipeAction {
                                    color: Color::Danger,
                                    onclick: move |_| take_row(label),
                                    "Decline"
                                }
                            },
                        },
                        on_activate: move |state: SwipeState| match state.side {
                            SwipeSide::Start => {
                                take_row(label);
                                toaster.success("Archived");
                            }
                            SwipeSide::End => {
                                // Read one, then another: each toast takes the
                                // last one's place rather than queueing.
                                toaster.show(ToastOptions::new("Marked read").replace());
                            }
                        },
                        on_dismiss: move |_| take_row(label),
                        Item {
                            label,
                            description,
                            start: rsx! { Bell { size: 20 } },
                        }
                    }
                }
            }
            if rows().is_empty() {
                EmptyState {
                    title: "All caught up",
                    icon: rsx! { Bell { size: 40 } },
                    "Nothing new this week."
                }
            }
            if !gone().is_empty() {
                List { lines: ListLines::None,
                    Item {
                        label: "Bring them back",
                        metadata: "{gone().len()}",
                        start: rsx! { RotateCcw { size: 20 } },
                        onclick: move |_| {
                            let mut restored = gone.take();
                            restored.append(&mut rows.write());
                            restored.sort_by_key(|row| {
                                ACTIVITY_ROWS.iter().position(|(label, ..)| *label == row.0)
                            });
                            rows.set(restored);
                        },
                    }
                }
            }
        }
    }
}

/// The settings screen: who you are, and the switches that follow you around.
#[component]
fn ProfileScreen() -> Element {
    let alerts = use_alert();
    let toaster = use_toast();
    let notifications = use_signal(|| true);
    let mut favorites = use_signal(|| vec!["Pebble Creek", "Oak Hollow", "Mill Ridge"]);
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
                // Dragged, or moved with the arrow keys from the handle.
                ReorderList {
                    onreorder: move |(from, to): (usize, usize)| {
                        favorites.with_mut(|courses| {
                            let course = courses.remove(from);
                            courses.insert(to, course);
                        });
                    },
                    List { variant: ListVariant::Raised,
                        ListHeader { "Favorite courses" }
                        for (index, course) in favorites().into_iter().enumerate() {
                            ReorderItem { key: "{course}", index,
                                Item {
                                    label: course,
                                    description: (index == 0).then(|| "Shown first when you book".to_string()),
                                    end: rsx! { ReorderHandle { label: format!("Move {course}") } },
                                }
                            }
                        }
                    }
                }
                List { variant: ListVariant::Raised,
                    // Text the reader needs in full wraps rather than being cut.
                    Item {
                        label: "Handicap",
                        description: "Your index is the average of your best eight differentials from your last twenty rounds, updated the day after each round.",
                        wrap: true,
                    }
                }
                Divider { label: "Account", spaced: true }
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
