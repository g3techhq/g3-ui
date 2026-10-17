# Responsive layouts and documentation implementation

Implemented 2026-10-16, following the documentation and responsive-layout
investigation. The earlier audit fixes in the working tree are preserved.

## Library API

- LayoutSize and use_layout_size expose Compact below 48rem, Medium from
  48rem, and Expanded from 64rem. Existing ShellSize and wide_columns keep
  their binary 48rem behavior.
- LayoutScope establishes an explicit nearer g3-app-shell container after
  navigation and page padding. The shared observer reports only class changes,
  works without animation frames, and disconnects on unmount.
- AdaptiveLayout takes compact, medium, and expanded Element slots. Only one
  branch mounts. Missing slots fall back downward. A keyed branch guarantees
  that selecting a different supplied variant unmounts the old one, while
  reusing a fallback preserves its mounted state. Shared form state and players
  should live above the switch.
- PageLayout takes start/end asides and main children. Its slots stay mounted
  as CSS changes between one, two, and three columns. On medium widths the two
  asides share a vertical stack; expanded layouts keep most width for the main
  column. PageLayoutOrder chooses main-first or asides-first DOM reading order.
- Grid adds medium_columns and expanded_columns. Fit remains the preferred
  policy for repeated cards because it responds to the grid's actual width.
  Nested grids reset absent overrides instead of inheriting a parent's policy.
- Wide content padding now uses container units, rather than viewport units.

## Consuming applications

Tawny's VideoGrid and skeleton use Fit(18rem) for standard videos and Fit(10rem)
for shorts, replacing abrupt fixed column-count jumps. The grid needs no new
measurement observer. Its persistent player stays in the existing shell. Two
behavior-preserving existing lint findings were also cleaned up: a nested
timestamp condition and a redundant seek callback closure.

Media Mancer Home and MediaDetail use PageLayout instead of long breakpoint
utility strings. Home retains main-first reading order. MediaDetail uses hero,
rating, then main content; WhereToWatch now sits in the main flow. This is an
intentional reading-order change, without duplicate card trees or resources.
Hero sizing uses its own available-width container and bounds cover size.
Shelf covers are fluid with their card width, scaling from 7rem to 10rem.
Poster grids use larger minimums clamped to available space.

Both source Tailwind CSS and the existing generated stylesheet were updated;
pre-existing edits in the generated stylesheet are preserved. No permanent
dependency path override was added. App changes using PageLayout require this
unreleased library checkout until its API is published by the maintainer.

## Playground

The home page opens Getting started. Shared page metadata drives guide, pattern,
and hook navigation and the search index. Ctrl+K/Cmd+K opens an accessible search
dialog; normal Tab/Enter navigation opens results. Exported component names
and hook names are searchable. Guides cover installation, web/native features,
responsive composition, layout hooks, overlay hooks, and theme hooks. GitHub,
API-reference, and per-demo source links provide the path back to the code.

Mobile, Tablet, Landscape, Desktop, and Resize previews are available. Resize
uses a keyboard- and touch-accessible slider. Width, mode, and theme changes
preserve the mounted demo. Wide previews scroll from a reachable start edge
when necessary; phone frames fit the available pane, including its scrollbar.

## Verification

- Library pre-push passed: 139 default tests, 140 with transitions, 79 passing
  doctests with one ignored, five playground tests, 14 production-script tests,
  build checks, strict lint, formatting, and spelling.
- Library WASM and Android ARM64 cross-compilation passed.
- Tawny checks passed for web, server, mobile, and desktop. Its 66 web Rust
  tests and 138 server tests passed; one live-network server test was skipped.
  Player JavaScript and Python unit tests also passed. Strict lint passed on all
  four feature sets after the two small cleanups. Biome still reports 50
  non-failing warnings in existing app styles. Timestamp-link tests were rerun
  after that cleanup and all three passed.
- Media Mancer checks and strict lint passed for web, server, and mobile.
  Its 95 web tests and 210 server tests passed on the single-worker retry.
  Native checks used the installed Visual Studio environment and
  AWS_LC_SYS_NO_ASM=1 because NASM is unavailable. This is a test-process
  setting, not a product configuration change.
- The playground was built and inspected through the browser. Compact,
  medium, and expanded branches and one/two/three-column placement were
  observed. A typed draft survived preview width, mode, and theme changes.
  Ctrl+K search found use_toast and AdaptiveLayout and opened their pages.
  Keyboard Home/End on the width slider crossed compact/expanded boundaries.
  Phone markup had no document-level horizontal overflow. Screenshots are in
  target/responsive-task, outside the package contents.
- Browser automation source now includes responsive placement, state retention,
  keyboard resizing, and search regression coverage. The standalone Playwright
  command was not run in this session; browser interaction used the connected
  browser controls.

The first parallel native test builds exhausted Windows memory; both app test
retries passed with one worker. Temporary lockfile changes in both apps and the
playground were restored byte-for-byte to their pre-task contents, including
any pre-existing edits. The pre-existing parent Cargo path overrides were left
unchanged. Consuming-app screens were not run in a browser in this session,
and no Android device/emulator runtime or performance check was performed.

## Header and theme generator follow-up

Guide pages now keep platform and reading-width controls visible. Reading widths
are separate from component preview preferences. iOS toolbar segments use the
library's 28px button height and an 8px bottom inset; MD's underline remains
flush. A specificity conflict in preview aspect ratios was corrected: browser
measurements confirm an 844 × 390 landscape frame and an 832 × 624 tablet frame.

The theme generator edits all 17 color fields and all seven optional Look fields,
previews real controls, copies a Rust Theme value, and retains the custom theme
across playground routes. On compact shells, Colors/Preview/Code views keep the
editor usable. Generated output preserves CSS alpha values and escapes Rust
strings. Copy feedback clears after edits so it cannot describe an older theme.

The follow-up pre-push checks passed, including 10 playground tests. Browser
checks covered live color/radius changes, global custom theme selection on the
Card page, clipboard copy, keyboard platform selection, phone tabs, dark mode,
phone document width, and both platform toolbar geometries at phone and wide
widths. Browser regression source includes these behaviors and preset ratios;
its syntax check passed, while browser interaction used the connected browser
controls rather than the standalone Playwright command. Screenshots remain in
target/responsive-task.

After further review, static guides again omit the platform/size toolbar and
preview-width labels. Only component demos, the transition showcase, and the
theme generator expose those controls. Guides use a fluid reading column.
Segment keyboard outlines now draw inward, avoiding clipping by the group's
paint containment while keeping keyboard focus visible. Both phone toolbar
rows remain independent radio groups, and the Header grows to fit them.

## Running documentation examples and shared theme scroll

All six guide, pattern, and hook pages now render compiled examples in the
same device viewer used by components. Their platform and viewport controls
are restored because they now affect running previews. Displayed code is
extracted from those compiled functions; a regression test checks every entry.
The theme-hook editor passes writable signals as props rather than adding an
application context. The existing library hooks still return reactive values;
no public signal API was changed.

The theme generator uses a single page scroll for the editor and results.
The result column is no longer sticky or height limited, and exported code has
no vertical height cap. Browser measurements confirmed both columns move by the
same amount when scrolling over the results. Phone Colors/Preview/Code tabs
remain available and retain their mounted preview state.

Validation: just pre-push passed, including 13 playground tests and 14 script
tests. Connected-browser checks exercised all six examples: getting-started
button feedback, saved input, responsive shared draft/filter state, Compact /
Medium / Expanded hook values, toast, confirmation, action-sheet results,
controlled-sheet dismissal, theme and mode changes, localized labels, and
localized action feedback. Visual checks covered phone and wide layouts in
iOS and MD, light and dark colors. No browser runtime errors were recorded in
this fresh browser session. The standalone browser-test command was not run;
these flows used the connected browser. Screenshots are in
target/responsive-task. No consuming-app or Android runtime changes were
involved in this documentation-only follow-up.

## Guide structure and hook explanations

Removed DocPage and its generic prose/preview/code template. Guides now have
individual components with semantic sections, headings, setup steps, and
topic-specific code placement. Shared metadata is limited to navigation and
search. Getting started retains the phone/tablet/desktop and Ionic design
intent of the maintainer's introduction. Dependency snippets are actual,
copyable TOML with the code display's TOML grammar enabled.

The theme guide distinguishes a local mutable Theme value from writable
provider state, explains nearest-provider inheritance, and retains the running
mutation example. No hook API or provider ownership behavior changed.

The layout guide compares local content width with enclosing AppWrapper width.
The demo can constrain its LayoutScope to 32rem: in Desktop view the local
result changes to Compact while the shell remains Wide. The guide includes
the exact width ranges, a sidebar example, and guidance for choosing each hook.
Comparisons stack on phones rather than splitting hook names across table cells.

Validation for this pass: just pre-push passed (140 default and 141 transitions
tests, 79 doc tests with one ignored, 12 playground tests, and 14 script tests).
Connected-browser review covered all six pages, TOML highlighting, light/MD
and dark/iOS presentation at phone and wide widths, the constrained-layout
interaction, and retained theme controls. The phone comparison has no page or
card overflow; the fresh browser session recorded no runtime errors. Screenshots
include layout-explanation-wide.png and layout-explanation-phone.png under
target/responsive-task. The running playground was reused and manually rebuilt;
no new serve process was started. Compared with the pre-pass lockfile, only the
TOML grammar package and its dependency reference were added.

## Writable theme state, overview, and search

use_theme now returns Signal<Theme>. Reads subscribe in the usual Dioxus way;
set replaces the theme and write edits fields. Providers own the theme state,
and CSS and readers use that same state. An explicit ThemeProvider override is
isolated, while a provider without a theme override shares its parent's handle.
The context itself stays live when an override is added or removed. Changed
theme props replace local state; unrelated renders preserve edits. The prop
effect remembers its last applied input independently of its callback closure,
and does not overwrite a hook write made before the initial effect runs.

Five mounted tests cover CSS and reader updates, whole-theme replacement,
retained writes, changed props, shared inheritance, scoped isolation, changing
provider handles, local fallback, and a default AppWrapper with no app-owned
theme signal. Existing consumers have no use_theme call sites to migrate.
README, rustdoc, playground examples, and CHANGELOG document the new return
type and migration. Struct field edits still need write/with_mut; mut alone
does not dereference a Signal into its stored struct.

The root route now presents the library's phone/tablet/desktop proposition and
an interactive workspace card. Overview is available from navigation and
search. Getting started now begins with dependency setup and keeps only setup,
launch, and first-screen instructions.

Search uses a 48rem desktop dialog, fixed input/count/footer, native result
buttons, and one scrolling results area. The scrollbar is a subtle rounded
thumb without arrow buttons. Query changes reset the results scroll. Phone
layout keeps the input and Close search action visible; the shorter placeholder
fits narrow screens. The landing hero's typography is scoped to its heading,
so it does not override titles inside the live preview.

Validation: just pre-push passed with 145 default-feature tests, 146 transitions
tests, 80 doc tests (one ignored), 12 playground tests, and 14 script tests.
The no-default-features library check also passed. Tawny passed web, server,
mobile, and desktop checks; Media Mancer passed web, server, and mobile checks.
Both used this checkout through the existing parent Cargo path override. Tawny's
lockfile was temporarily resolved to g3-ui 0.5.1 so Cargo actually used the
override. Media Mancer's Windows TLS dependency required its supported
AWS_LC_SYS_PREBUILT_NASM=1 build option because NASM was not installed; no app
configuration was changed. Both consumer lockfiles were restored byte-for-byte.

Connected-browser checks covered the landing preview's saved state, setup-only
Getting started, search filtering, keyboard result activation, empty results,
Escape focus return, phone sizing, fixed input/footer while results scroll, and
theme mutations updating both CSS and readers while a local override remains
isolated. Visual review included phone and wide layouts, iOS and MD, and light
and dark colors. Screenshots are in target/responsive-task. Consumer apps were
type-checked; their live UI and an actual Android runtime were not run because
only the playground server was running. The standalone browser harness was
syntax-checked; interaction used the connected browser.

## Running documentation examples and shared theme scroll

All six guide, pattern, and hook pages now render compiled examples in the
same device viewer used by components. Their platform and viewport controls
are restored because they now affect running previews. Displayed code is
extracted from those compiled functions; a regression test checks every entry.
The theme-hook editor passes writable signals as props rather than adding an
application context. The existing library hooks still return reactive values;
no public signal API was changed.

The theme generator uses a single page scroll for the editor and results.
The result column is no longer sticky or height limited, and exported code has
no vertical height cap. Browser measurements confirmed both columns move by the
same amount when scrolling over the results. Phone Colors/Preview/Code tabs
remain available and retain their mounted preview state.

Validation: just pre-push passed, including 13 playground tests and 14 script
tests. Connected-browser checks exercised all six examples: getting-started
button feedback, saved input, responsive shared draft/filter state, Compact /
Medium / Expanded hook values, toast, confirmation, action-sheet results,
controlled-sheet dismissal, theme and mode changes, localized labels, and
localized action feedback. Visual checks covered phone and wide layouts in
iOS and MD, light and dark colors. No browser runtime errors were recorded in
this fresh browser session. The standalone browser-test command was not run;
these flows used the connected browser. Screenshots are in
target/responsive-task. No consuming-app or Android runtime changes were
involved in this documentation-only follow-up.

## Plain guide copy and writable mode/strings

Removed tutorial-style preview headings and redundant interaction prompts from
all six guides. Getting started retains the useful header-control instruction;
the responsive example now says only to change the layout size. Native setup
uses the same g3-ui dependency as the browser: the crate has no default features
to disable, and the app selects its Dioxus renderer.

Added use_mode() -> Signal<ComponentMode> and changed use_strings() to return
Signal<Strings>. Provider state follows the same ownership rules as use_theme:
explicit overrides are isolated, omitted values inherit the same handle,
changed props replace current values, and unrelated renders preserve edits.
The mode resolver still honors explicit component props. Hook examples now
obtain writable state directly in children instead of accepting parent signals.
Built-in label consumers read the signals; hooks formerly inside optional-label
fallbacks now run unconditionally. README, rustdoc, exports and migration notes
are updated. Nine mounted provider-state tests cover writes, CSS, labels,
prop changes, inheritance, override changes, isolation and local defaults.

Reproduced the layout example bug: removing max-width from the style string
left the previous 32rem constraint in the DOM. Both branches now set max-width
explicitly. Connected-browser measurements confirmed 512px constrained and
1191.9375px expanded, with Compact/Expanded values and a persistent Wide shell.
The round trip worked in both modes; phone previews remained Compact.

Validation: just pre-push passed with 149 default tests, 150 transitions tests,
82 doc tests (one ignored), 12 playground tests and 14 script tests. Tawny's
web/server/mobile/desktop checks and Media Mancer's web/server/mobile checks
passed against this checkout through the existing parent Cargo path override.
Both consumer lockfiles were restored byte-for-byte. Browser interaction also
confirmed mode writes change styling, strings writes update Cancel/Confirm,
and shared changes survive a status update while local theme edits stay scoped.
Visual review covered phone and desktop widths, iOS and MD, light and dark.
Only the playground UI was run; consumer live UIs and an Android runtime were
not launched. Screenshots and check logs are under target/.

## Button navigation and example grouping

Replaced landing-page navigation links and the missing-guide fallback with
public g3-ui Button links. Removed the custom landing CTA styling and broad
anchor-color overrides so the buttons use their normal platform appearance.
Removed the redundant Real components copy from the preview, landing footer,
and theme-generator descriptions. Demo app and Route transitions now appear
under Examples in both navigation and search, with their existing URLs kept.
Search accepts Demo app, DemoApp, Route transitions, RouteTransitions, and
Examples. Navigation accessibility labels now describe the full menu.

The route transitions page has an outlined crates.io button beside its tour
action, linking to https://crates.io/crates/g3-route-transitions in a new tab.
The action group wraps and centers each row at narrow widths.

Validation: just pre-push passed (149 default tests, 150 transitions tests,
82 doc tests with one ignored, 12 playground tests, and 14 script tests).
Connected-browser checks covered landing buttons in MD and iOS at phone and
wide widths, light/dark colors, the Examples navigation section, search for
Examples returning both pages, Demo app navigation, and the new package link's
URL, target and rel attributes. Button navigation to Responsive layouts worked.
The phone action group centers wrapped rows and has no horizontal overflow.
No errors were recorded in the fresh browser session. Screenshots are in
target/responsive-task/navigation-buttons-* and route-transitions-package-*.
No library or consumer-app behavior changed; native app runtimes were not run.

## Capability-focused overview copy

Rewrote the overview for Rust/Dioxus developers. The hero names the component
styles, supported platforms and component categories. Feature sections describe
mode overrides, available-width layouts, runtime theme changes and overlay hook
results. The installation section names Cargo.toml and AppWrapper. Removed
slogans, the Project settings subtitle, introductory preview instructions and
promotional counters; removed their unused styles. Updated overview search copy.

Validation: just pre-push passed. Visual checks covered MD and iOS at 390px and
1440px, including light and dark colors. The save action still displays the
selected project name. No browser errors were recorded. Screenshots are in
target/responsive-task/capability-copy-*. No library or consuming-app code changed.
