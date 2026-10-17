# g3-ui

[![CI](https://github.com/g3techhq/g3-ui/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/g3techhq/g3-ui/actions/workflows/ci.yml)
[![Playground image](https://github.com/g3techhq/g3-ui/actions/workflows/publish-playground-image.yml/badge.svg?branch=main)](https://github.com/g3techhq/g3-ui/actions/workflows/publish-playground-image.yml)
[![Crates.io](https://img.shields.io/crates/v/g3-ui.svg)](https://crates.io/crates/g3-ui)
[![docs.rs](https://docs.rs/g3-ui/badge.svg)](https://docs.rs/g3-ui)
[![License](https://img.shields.io/crates/l/g3-ui.svg)](#license)

Mobile-first [Dioxus](https://dioxuslabs.com/) components inspired by [Ionic](https://ionicframework.com/).

<p align="center">
  <img
    src="docs/media/social-preview.png"
    alt="Four mobile g3-ui screens showing a feed, a booking form, a sheet, and a profile"
    width="1280"
  >
</p>

`g3-ui` is a component library for Dioxus web and desktop/android/ios using web view. It may be expanded to work with Dioxus Native (blitz) in the future. Components try to be compatible with desktop and mobile sizing as much as possible. Every component has iOS and Material Design (MD) styling,
CSS-variable theming, and WAI-ARIA semantics. The Rust crate name is `g3_ui`.

## Install

```toml
[dependencies]
dioxus = { version = "0.7.9", features = ["router"] }
g3-ui = "0.5"
```

Enable the optional route-transition integration when your app also uses `g3-route-transitions`:

```toml
[dependencies]
g3-ui = { version = "0.5", features = ["transitions"] }
g3-route-transitions = "0.4"
```

The minimum supported Rust version is 1.88.

## Quick Start

`AppWrapper` loads the stylesheet, resolves the platform mode, applies the theme, and hosts the
toasts, alerts, and action sheets opened from code. Everything else nests inside it.

```
use dioxus::prelude::*;
use g3_ui::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        AppWrapper { theme: Theme::system(),
            Header { title: "Games" }
            Content {
                Card { title: "Pending game",
                    "Invite players and choose a format."
                }
                Button { onclick: |_| {}, "Create game" }
            }
        }
    }
}
```

`g3_ui::prelude` exports the components and theme items only. Import `dioxus::prelude` yourself.
Component names are unprefixed. If one collides with a local name, import it under an alias
(`use g3_ui::Button as UiButton;`) or write the path (`g3_ui::Button { .. }`).

## Components

| Category | Components |
| --- | --- |
| App shell | `AppWrapper`, `Header`, `BackButton`, `Content`, `TabLayout`, `ThemeProvider` |
| Navigation | `AdaptiveNav`, `NavBar`, `NavRail`, `NavItem`, `NavigationDrawer`, `Tabs`, `TabList`, `Tab`, `TabPanel`, `SegmentGroup`, `SegmentButton` |
| Actions | `Button`, `Fab`, `FabButton`, `FabList`, `FabMenu`, `InfoButton`, `Popover`, `Menu`, `MenuItem` |
| Forms | `Input`, `TextArea`, `Select`, `Checkbox`, `Toggle`, `RadioGroup`, `Radio`, `Range`, `Stepper`, `Searchbar`, `DatePicker`, `TimePicker`, `Calendar` |
| Content | `Card`, `List`, `ListHeader`, `Item`, `SwipeItem`, `SwipeAction`, `Text`, `Img`, `Tooltip`, `Avatar`, `Badge`, `Chip`, `Divider` |
| Layout | `Stack`, `Grid`, `LayoutScope`, `AdaptiveLayout`, `PageLayout` |
| Disclosure | `AccordionGroup`, `AccordionItem` |
| Overlays | `BottomSheet`, `SideSheet`, `Modal`, `Alert`, `ConfirmModal`, `ActionSheet`, `Toast` |
| Feedback | `Progress`, `Skeleton`, `Spinner`, `Refresher`, `InfiniteScroll` |

[docs.rs/g3-ui](https://docs.rs/g3-ui) has the full prop reference. The [playground](#playground)
shows every component live.

## Responsive layouts

For responsive composition, use `LayoutScope` inside page content so layouts
measure the space remaining after navigation. `use_layout_size()` returns
`LayoutSize::Compact` below 48rem, `Medium` from 48rem, and `Expanded` from 64rem.
The existing binary `use_shell_size()` API keeps its 48rem behavior.

`GridColumns::Fit(18.0)` chooses as many readable 18rem cards as fit, including
intermediate column counts. Fixed-count layouts can use `medium_columns` and
`expanded_columns` in addition to the compatible `wide_columns` prop.

`PageLayout { start, end, children }` rearranges persistent content into one,
two, or three columns. `AdaptiveLayout { compact, medium, expanded }` mounts
one variant; omitted slots fall back to the nearest smaller supplied variant.
Keep shared signals, resources, and players above an adaptive switch, because
switching variants unmounts the old branch. `LayoutScope` intentionally
establishes a nearer `g3-app-shell` container for its children; keep navigation
outside it.

The playground includes `/docs/getting-started`, `/docs/responsive-layouts`,
platform and hook guides, documentation search, and additional preview widths.

## Component State

A component with a value takes an optional `Signal<T>` and reads and writes it directly. Leave
the signal out and the component keeps its own state. An optional `onchange` or `on_*` callback
reports changes when you need a side effect as well:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
# #[derive(Clone, Copy, PartialEq)] enum Format { Stroke, Match }
# fn save(_name: String) {}
let checked = use_signal(|| false);
let name = use_signal(String::new);
let format = use_signal(|| None::<Format>);

rsx! {
    Toggle { checked, label: "Notifications" }
    Input { label: "Name", value: name, onchange: move |name| save(name) }
    RadioGroup { value: format, label: "Format",
        Radio { value: Format::Stroke, label: "Stroke play" }
        Radio { value: Format::Match, label: "Match play" }
    }
}
# }
```

`Select`, `RadioGroup`, `SegmentGroup`, `Tabs`, and `AccordionGroup` are generic over the value
type, so options can be your own enums instead of strings or indexes.

## Forms and Events

Forms are signal-driven. Read the controls' signals on submit and validate them in the app.
`Input` and `TextArea` forward native attributes such as `name`, `required`, and `pattern`.
Custom selections (`Select`, date/time pickers, checkbox/toggle and generic radio values) do
not provide a complete native `FormData` or browser constraint-validation contract. Their
`required` indicator does not enforce submission. For native serialization, supply hidden
inputs from the same signals and validate before sending.

| Control | Notification |
| --- | --- |
| `Input`, `TextArea` | `oninput` for typing; `onchange` on native commit, or after `debounce_ms` when set. Clearing notifies immediately and cancels a pending debounce. |
| `Searchbar` | Uses the debounced text contract; clearing is immediate. |
| `Range` | `oninput` while moving; `onchange` on native commit. |
| Selection controls, tabs, calendar, stepper | `onchange` after a user selection, including keyboard selection. |
| `DatePicker`, `TimePicker` | `onchange` on confirmation (or Clear); cancelling preserves the committed signal. |
| `Modal`, sheets, popovers | `on_dismiss` for component-initiated dismissal, including `dismiss_top()`. Setting `open` yourself does not call it. |
| `Toast` | `on_dismiss` for every close, including timer and owner-initiated closes. |

Changing a controlled signal updates the UI without synthesizing a user notification. Plain
configuration props follow rerenders; a changed toast duration restarts its timer, and changed
picker values or bounds reset the open draft. Use `defer_selection` where offered when navigation
must complete before the owning signal changes.

## Dates and Times

`DatePicker` and `TimePicker` are form fields that open a picker. They are the way in:
`Input` has no date or time kind, because the browser's own picker ignores the mode and
the theme. Their values are `CalendarDate` and `TimeOfDay`, which parse from and print as
ISO 8601 (`2026-09-19`, `14:05`), so nothing has to agree on a string format:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
let tee_day = use_signal(|| None::<CalendarDate>);
let tee_time = use_signal(|| None::<TimeOfDay>);

rsx! {
    DatePicker { label: "Tee day", value: tee_day, min: CalendarDate::today() }
    TimePicker { label: "Tee time", value: tee_time, minute_step: 10 }
}
# }
```

`style` decides how one opens. By default it follows the mode:

| `PickerStyle` | Date | Time |
|---|---|---|
| `Auto` | wheels on iOS, dialog on MD | wheels on iOS, dialog on MD |
| `Wheels` | month, day, and year columns in a bottom sheet | hour, minute, and AM/PM columns |
| `Dialog` | a calendar with Cancel and OK | a clock face, with a typing mode |
| `Popover` | a calendar anchored to the field | wheels anchored to the field |

A `Popover` picker becomes a bottom sheet on a phone. `Calendar` is also available on its
own, for a month grid inside a page.

Wheel time pickers round minutes down to the nearest `minute_step` within the hour, including
immediate confirmation: 14:59 with a step of five displays and commits 14:55. Dialog typing keeps
minute precision. Date bounds and `is_date_disabled` apply to every picker style and confirmation.

`CalendarDate::today()` and `TimeOfDay::now()` use the device's local clock. For server rendering,
pass the same explicit `reference_date` to `Calendar`/`DatePicker` on server and client; a server
cannot infer the visitor's time zone.

Both are usable without a pointer. The calendar's arrow keys move a day at a time, Home and
End reach the ends of the week, and Page Up and Page Down change month or year. A wheel column
is a spin button, and the clock face is a radio group; Material's dialog can also be typed into.

## Pull to Refresh

Give `Content` an `on_refresh` handler and it refreshes when pulled down from the top. Keep
`refreshing` true while the work runs:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
# #[component] fn RoundList() -> Element { rsx! {} }
# async fn reload() {}
let mut refreshing = use_signal(|| false);

rsx! {
    Content {
        refreshing: refreshing(),
        on_refresh: move |_| async move {
            refreshing.set(true);
            reload().await;
            refreshing.set(false);
        },
        RoundList {}
    }
}
# }
```

## Overlays From Code

Toasts, alerts, and action sheets can be opened from an event handler without declaring them in
markup. `AppWrapper` renders them one at a time. Alerts and action sheets return a future with
the user's answer:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() {
# fn remove_round() {}
let toast = use_toast();
let alerts = use_alert();

let delete = move |_: MouseEvent| async move {
    if alerts.confirm("Delete round?", "This cannot be undone.").await {
        remove_round();
        toast.success("Round deleted");
    }
};
# }
```

`use_action_sheet()` works the same way and resolves to the chosen button's index. Each overlay is
also a component (`Toast`, `Alert`, `ActionSheet`) that takes an `open` signal, for cases where the
markup should own it.

## Modes and Themes

Components have Ionic-style `Ios` and `Md` modes. A component takes the first mode it finds: its
own `mode` prop, the nearest `AppWrapper` or `ThemeProvider`, then the global default set by
`set_mode` or `init_auto_mode`. Changing the provider's `mode` prop updates the subtree.

```no_run
# use dioxus::prelude::*;
# #[component] fn App() -> Element { rsx! {} }
fn main() {
    g3_ui::init_auto_mode(); // iOS look on Apple platforms, MD elsewhere
    dioxus::launch(App);
}
```

In an app rendered on a server (Dioxus fullstack), pass `mode` to `AppWrapper`, for example from
a saved setting. The server cannot see the device, so a mode detected in the browser would not
match the markup the server sent, and hydration keeps the server's.

Colors are CSS custom properties (`--g3-color-*`) generated from a `Theme`. The presets are
`Theme::default_light()`, `Theme::default_dark()`, and `Theme::system()`, which follows the
operating system through CSS `light-dark()`. Every field is public:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() {
# let light_brand = Theme::default_light();
# let dark_brand = Theme::default_dark();
let brand = Theme::system().with_accent("#1f7a4d");

let custom = Theme {
    bg: "#0b1020".into(),
    card: "#151b2e".into(),
    ..Theme::default_dark()
};

let paired = Theme::adaptive(light_brand, dark_brand);
# }
```

To build a theme at runtime, say from a color the user picks, give `ThemeSeeds` an accent and a
page background. The rest of the colors are filled out to read at 4.5:1, and an accent that
would not is moved only as far as it must be (`ThemeSeeds::generate` says what moved):

```
# use g3_ui::prelude::*;
let theme = Theme::from_seeds(&ThemeSeeds::new("#e8590c", "#fff8f0"))?;
# Ok::<(), g3_ui::palette::InvalidSeed>(())
```

Beyond color, `Theme::look` shapes the surfaces: a page gradient and how cards are cut, edged and
lit. Every field is optional and unset ones keep the platform look:

```
# use g3_ui::prelude::*;
let theme = Theme {
    look: Look {
        background: Some("radial-gradient(60% 40% at 90% 0%, #f9a8d4aa, transparent 70%)".into()),
        card_radius: Some("1.125rem".into()),
        card_border: Some("1px solid var(--g3-color-border)".into()),
        card_shadow: Some("0 0.625rem 1.875rem -0.75rem var(--g3-color-shadow)".into()),
        ..Look::default()
    },
    ..Theme::default_light()
};
```

`Theme::accent_secondary` is a second brand color, and `Look::accent_fill` can use it: the primary
`Button` then paints `accent_fill` (a gradient from `var(--g3-color-accent)` to
`var(--g3-color-accent-secondary)`, say) while success, warning, danger and neutral buttons keep
their own fills. A `ThemeProvider` replaces its parent's look rather than adding to it.

A gradient cannot be written as `light-dark()`, so `Theme::adaptive` keeps both themes' looks and
applies the dark one under `prefers-color-scheme: dark`. A translucent `Theme::card` pairs with
`card_backdrop_filter: Some("blur(18px)".into())`.

The theme is written as inline custom properties on the wrapper, so passing a different `Theme`
re-themes the tree in place. `use_theme()` returns the nearest provider's writable
`Signal<Theme>`. Read with `theme()` or `theme.read()`; change it in an event handler with
`theme.set(Theme::default_dark())` or `theme.write().accent = "#16a34a".into()`.
The wrapper's CSS and all readers follow those writes. No app-owned theme signal or extra
context is required. Explicit theme overrides own separate state; providers without a theme
override share the parent's handle. A changed theme prop replaces the current state.

Migration: code that previously used `use_theme().accent` should read through the signal,
for example `use_theme().read().accent.clone()`, or use `use_theme()()` for a Theme snapshot.

Built-in component text such as "Close" and "Cancel" comes from `Strings`. Pass translated
strings as an initial value in `AppWrapper { strings, .. }`, or change them from a child
with `let mut strings = use_strings(); strings.write().cancel = "Cancelar".into();`.
`use_strings()` returns `Signal<Strings>`; read labels with `strings.read().cancel` or
`strings()` for a snapshot. `use_mode()` returns `Signal<ComponentMode>` and changes the
provider's styling with `mode.set(ComponentMode::Ios)` or `*mode.write() = ComponentMode::Ios`.
Both hooks share the same inheritance, override, and prop synchronization behavior as
`use_theme()`. `use_component_mode(mode)` remains the resolver for a component's optional
mode prop: an explicit override wins over ambient mode changes.

Migration: replace reads such as `use_strings().cancel` with
`use_strings().read().cancel.clone()`. Call the hook unconditionally, before branching.
No parent signal props or extra context are needed to change theme, mode, or labels.

Rating speech and date ordering also come from
`Strings`: `rating_unrated`, `rating_value` (`{value}`, `{max}`), `date_long`, `date_short`,
and `date_month_year` (`{weekday}`, `{month}`, `{day}`, `{year}` as appropriate).

To plug in a translation system (a map, a Fluent bundle such as dioxus-i18n, or a server's
catalog), build the strings with `Strings::from_lookup`. It asks for each piece of text by a
message id, the field name (`"cancel"`, or `"months_1"` for January), and keeps English for
every id the lookup returns `None` for:

```
# use g3_ui::prelude::*;
let strings = Strings::from_lookup(|id| match id {
    "cancel" => Some("Annuler".into()),
    "confirm" => Some("Confirmer".into()),
    _ => None,
});
# let _ = strings;
```

## Responsive App Shell

`AppWrapper` measures its own width with a CSS container query rather than the browser viewport,
so an app embedded in a narrow frame keeps its phone layout. At `48rem` and wider:

- `AdaptiveNav` inside a `TabLayout` moves from the bottom edge to a rail beside the page.
  `NavBar` and `NavRail` are the fixed forms, for apps that decide the layout themselves.
- Bottom sheets become floating panels, `Select` and `Popover` open as anchored menus instead of
  sheets, and modals widen.
- Toasts become a centred snackbar.

At `64rem` the `Header` toolbar moves inline with the title.

`Grid` takes `wide_columns` and `wide_gap` for the same breakpoint, and
`Content { width: ContentWidth::Readable }` keeps text at a comfortable width on a wide shell
while its scrollbar stays at the page edge.

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
# #[derive(Routable, Clone, Debug, PartialEq)]
# enum Route {
#     #[route("/")]
#     Rounds {},
#     #[route("/profile")]
#     Profile {},
# }
# #[component] fn Rounds() -> Element { rsx! {} }
# #[component] fn Profile() -> Element { rsx! {} }
# #[component] fn Flag() -> Element { rsx! {} }
# #[component] fn User() -> Element { rsx! {} }
rsx! {
    TabLayout {
        Header { title: "Rounds" }
        Content { /* page */ }
        AdaptiveNav {
            NavItem { label: "Rounds", icon: rsx! { Flag {} }, to: Route::Rounds {}, selected: true }
            NavItem { label: "Profile", icon: rsx! { User {} }, to: Route::Profile {},
                group: NavItemGroup::Secondary }
        }
    }
}
# }
```

`NavItemGroup::Secondary` moves an item to the bottom of the rail; the phone tab bar keeps the
declared order. `AdaptiveNav { compact: AdaptiveNavCompact::Hidden }` shows only the rail and
hides the phone tab bar, for full-screen routes. Set `--g3-nav-rail-width` to widen the rail.

## Sheets and Drawers

`BottomSheet` rises from the bottom edge. It can rest at several heights, given as fractions of
the app's height. With `backdrop_detent`, the page stays usable while the sheet is low:

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
# let results_open = use_signal(|| false);
# rsx! {
BottomSheet { open: results_open, detents: vec![0.2, 0.5, 1.0], backdrop_detent: 2,
    /* content */
}
# }
# }
```

`SideSheet` slides in from the start or end edge. `SideSheetBehavior::Overlay` covers the page,
`Push` moves the page aside, and `Reveal` moves the page to uncover a sheet that stays still.
Both edges are logical, so `SheetEdge::Start` is on the right in a right-to-left document.

`NavigationDrawer` is persistent navigation beside the page, like Ionic's split pane. It is not a
dialog: the page narrows to make room, and nothing is dimmed or trapped. For `Push`, `Reveal`, and
`NavigationDrawer`, render the sheet and one page element as direct children of `AppWrapper`.

A sheet normally opens over a scrim that closes it when tapped. `backdrop: SheetBackdrop::None`
leaves the page behind visible and interactive, for something like comments beside a video. The
sheet is then no longer modal, and a bottom sheet closes only when dragged down.

Android Back should dismiss the most recently presented surface before navigating. Obtain
`use_overlay_controller()` once in your app, and call `dismiss_top()` in your Back callback.
It covers modals, alerts, action sheets, popovers and dismissible sheets, including sheets
without a scrim. It returns `true` when Back was consumed. A nondismissible modal consumes Back
without exposing the surface underneath. Toasts and permanent navigation drawers are excluded.
`count()` is reactive. `open_sheet_count()` remains available for sheet-only indicators.

```
# use dioxus::prelude::*;
# use g3_ui::prelude::*;
# fn demo() -> Element {
let overlays = use_overlay_controller();
rsx! { Button { onclick: move |_| { overlays.dismiss_top(); }, "Dismiss top overlay" } }
# }
```

## Route Transition Integration

With the `transitions` feature enabled, g3-ui components provide the
[`g3-route-transitions`](https://github.com/g3techhq/g3-route-transitions)
snapshot regions for you:

| Component | Region | Effect |
|---|---|---|
| `AppWrapper` | overlay (plus the stylesheet) | Rises and falls for routed sheets |
| `TabLayout` | base | Stays put during navigation and dims under a sheet |
| `Content` | segment | Slides for ordered peer routes such as segmented tabs |
| `AdaptiveNav` or `NavRail` as a rail | persistent | Stays in place above a rising sheet (the phone tab bar stays part of the base) |

Add `RouteTransitionPage` yourself, around each page's header and content, to get stack push and
pop motion:

```rust,ignore
use g3_route_transitions::RouteTransitionPage;

rsx! {
    TabLayout {
        RouteTransitionPage {
            Header { title: "Rounds" }
            Content { /* page content */ }
        }
        AdaptiveNav { /* persistent tabs stay still */ }
    }
}
```

Layout rules:

- **Sheet routes:** a route declared with `layer = sheet` must render
  `TabLayout { route_transition_base: false, .. }` (or no tab layout) and no
  `RouteTransitionPage`. Otherwise its content is captured outside the rising
  overlay. To keep the desktop rail beside the sheet, render the same tabs with
  `AdaptiveNav { compact: AdaptiveNavCompact::Hidden, .. }`. Phones hide them,
  so the sheet still covers the bottom tabs.
- **Segmented screens:** leave `RouteTransitionPage` out of screens whose
  `Content` should slide by itself. Inside a page, the whole page moves instead.
- **Nested wrappers:** `AppWrapper` is the overlay region by default. If a
  documentation shell or other non-navigating wrapper contains a second app
  wrapper, set `route_transition_overlay: false` on the outer one. A document
  may only have one overlay region.

See the `g3-route-transitions` README for the route metadata that decides which transition runs.

Without the feature, `g3-ui` does not depend on `g3-route-transitions` and does not emit
route-transition marker classes.

## Styling

All rules live in the `g3` cascade layer, so any unlayered rule in your stylesheet overrides them
without extra specificity or `!important`. Components accept a `class` prop. `Button`,
`FabButton`, and `Input` also pass any other HTML attribute through, such as `aria_haspopup` or
`data-*`. Component state is exposed as `data-state` and
ARIA attributes (`[data-state="open"]`, `[aria-checked="true"]`) rather than modifier classes.

## Upgrading From 0.3

0.4 renames most of the API. [CHANGELOG.md](CHANGELOG.md) lists every change; the common ones are:

| 0.3 | 0.4 |
| --- | --- |
| `G3`-prefixed aliases | unprefixed names only |
| `G3Body` | `Content` |
| `G3Navbar`, `G3NavbarTabBar`, `G3NavbarTab` | `TabLayout`, `AdaptiveNav`, `NavItem` |
| `G3Sheet` with `SheetPlacement` | `BottomSheet`, `SideSheet`, `NavigationDrawer` |
| `G3Field` | `Input`, `TextArea` |
| `G3Line`, `G3ItemDivider` | `Divider`, `ListHeader` |
| `G3SheetButton` | `InfoButton { sheet, .. }` |
| `G3FabContainer` | `FabMenu` |
| `ButtonStyle` / `style:` | `ButtonFill` / `fill:` |
| `Card { inset }` | `Card { variant: CardVariant::Filled }` |
| `List { inset }` | `List { variant: ListVariant::Raised }` |
| `SwipeItem { behavior }` | `start_behavior`, `end_behavior` |
| `is_open`, `active` props | `open`, `value` |
| `--color-*` variables | `--g3-color-*` |
| `Theme::with_focused`, `focused` | `Theme::with_accent`, `accent` |

## Playground

The [deployed interactive component gallery](https://g3ui.g3tech.net/) is built from `playground/`
inside this repository. It renders every component with live controls. Switch between MD and iOS,
and between Mobile and Desktop frames, to see the same tree in a compact and a wide shell. Every
demo has a stable URL such as `/components/button`.

The deployed [`/transitions` showcase](https://g3ui.g3tech.net/transitions) composes the real app
shell, header, content, navigation, cards, lists, buttons, and segmented controls with
`g3-route-transitions`, including a routed sheet.

When developing `g3-ui` and `g3-route-transitions` side by side, uncomment the adjacent
`[patch.crates-io]` block in `.cargo/config.toml`. Cargo then redirects every
`g3-route-transitions` dependency in the library and playground to the sibling checkout. Comment
the block again before committing; normal builds and published packages continue using the version
from crates.io.

```powershell
cd playground
dx serve
```

To type-check the playground without launching a dev server:

```powershell
cargo check --manifest-path playground/Cargo.toml
```

To regenerate the transition showcase media while the playground is running on port 8080, install
the optional capture dependency and run the recording recipe. The tour pauses for more than a
second between animations so each transition is readable.

```powershell
npm ci --prefix playground
just record-transitions
```

Production images are published to the GitHub Container Registry. See
[`deploy/README.md`](deploy/README.md) for Portainer and Docker Compose instructions.

## Browser and WebView Requirements

The supported capability baseline is modern evergreen browsers/WebViews with CSS container
queries, `:has()`, `color-mix()`, `light-dark()` colors, `inert`, Pointer Events, ResizeObserver,
and the Popover API. Use Chromium/Android System WebView 123+, Safari/WKWebView 17.5+, or Firefox
125+ as the feature baseline; this is not a claim of runtime testing every version. Android OS
version alone does not establish WebView compatibility. See the
[light-dark compatibility reference](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/color_value/light-dark)
and [Popover reference](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/showPopover).
An older engine without Popover falls back to ordinary positioning, which can be clipped by
transformed ancestors and cannot guarantee the same overlay stacking. Zoom remains enabled.

Continuous gestures stay in JavaScript to avoid per-move Android bridge calls. `SwipeItem::on_swipe`
is an opt-in exception; prefer completion callbacks. `InfiniteScroll` paginates but does not
virtualize rows: bound or window large lists in the app. Profile release builds on target devices
before changing rendering, blur, or shared theme storage based on performance assumptions.

Run `just pre-push` for Rust, markup, script behavior/disposal, and spelling checks. With the
playground already running, `npm ci --prefix playground` and `just browser-test` run the real-browser
regressions (install Playwright browsers first with `npx --prefix playground playwright install chromium`).
See [the runtime test guide](docs/testing.md) for the matrix and Android checks.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
