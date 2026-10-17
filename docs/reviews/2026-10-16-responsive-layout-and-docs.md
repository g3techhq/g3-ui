# Responsive layouts and documentation investigation

Date: 2026-10-16. Scope: g3-ui, Tawny video lists, Media Mancer home and
media-detail pages, and Libero's documentation site. This is an investigation
and proposed design, not an implementation or a runtime certification of the
two consuming apps.

Implementation follow-up: see
[responsive-implementation](2026-10-16-responsive-implementation.md) for the
implemented API, consuming-app changes, and verification results.

## Findings from source

| Location | Current behavior | Consequence |
| --- | --- | --- |
| g3-ui `src/components/app_wrapper.rs` | `ShellSize` is Compact below 48rem and Wide above it. | Rust consumers cannot distinguish medium from large shells. |
| g3-ui `src/components/layout.rs` | Grid has compact and wide column/gap props; wide begins at shell width 48rem. | A fixed desktop column count can apply while the content area is still narrow. |
| g3-ui `assets/g3-ui.css` | AdaptiveNav becomes a 4rem icon rail at 48rem; its expanded 15rem rail is enabled at 64rem. | There already is a middle navigation stage, but it is not represented by the public size API. |
| Tawny `src/components/video_card.rs:77` | Standard videos jump from one to four columns; shorts jump from two to six at 48rem. Skeletons jump from one to four. | Tablet, landscape-phone, and split-window cards can become much too narrow. |
| Media Mancer `src/components/home/home.rs:104` | A stack capped at 60rem becomes three columns at page width 76rem, with two fixed 24rem sidebars and two 2rem gaps. | At the threshold the center receives only 24rem, after previously being allowed 60rem. More available page space can abruptly shrink the main content. |
| Media Mancer `src/components/media/media_detail.rs:82` | At page width 72rem, two fixed 20rem sidebars and two 2rem gaps leave 28rem for the center. Mobile order is interleaved with many order utilities. | The two layouts require substantial repeated layout rules, and lack an intermediate arrangement. |
| Media Mancer `assets/tailwind.css:723` | The hero changes at *shell* width 72rem, whereas its enclosing page changes at *page* width 72rem. | Navigation and padding can make the hero change before its intended sidebar layout exists. |
| Media Mancer `src/components/shared/media_scroll_card.rs:24` | Shelf cards and their covers are fixed to 104px width. | Adding a medium breakpoint alone will not resize these cards. |
| g3-ui playground `src/main.rs` | Preview choices are Mobile and Desktop. | Intermediate and landscape layouts are easy to miss during review. |

Tawny's exact visible card width also depends on padding and gaps. Before
those deductions, four columns beside the 4rem rail at a 48rem shell have at
most 11rem each. Six shorts have at most about 7.3rem each. The source confirms
the sizing policy; these are bounds, not measured screenshots from Tawny.

## Recommended library design

### Three explicit stages, with clear scope

Use Compact, Medium, and Expanded names. `md` already refers to Material mode
in this library. Existing 48rem and 64rem shell transitions are a sensible
starting point to validate, rather than adding a third unrelated set of
device breakpoints.

Distinguish shell width from available content width. Shell width governs
navigation and global chrome. A layout inside Content needs to respond to
the room remaining after navigation and padding; reusable cards sometimes
need their own local container. Container queries should remain the default
for spatial layout. Do not replace them with viewport media queries.

Keep `ShellSize::Wide` and its existing 48rem meaning compatible. Adding a
variant to this public enum breaks exhaustive matches, and redefining Wide
would silently change behavior in consuming apps. An additive three-stage
API and a documented migration are preferable. A new local layout scope
should be explicit and documented as an extension to the repository's current
shell-only container convention.

### AdaptiveLayout for different trees

The user's suggested API is useful when the actual controls differ. Proposed
usage, not an existing exported component:

```rust,ignore
AdaptiveLayout {
    compact: rsx! { CompactControls { state } },
    medium: rsx! { TabletControls { state } },
    expanded: rsx! { DesktopControls { state } },
}
```

Contract to settle before implementation:

- Render only the selected branch. Optional medium/expanded slots fall back
  to the nearest smaller supplied slot; compact is required.
- Read the enclosing layout scope, with the app shell as the default scope.
- Keep all hooks unconditional. SSR and the first client render select the
  same fallback, then measurement can select the appropriate branch.
- Report width-class changes to Rust, not every resize event. Dispose the
  observer when its scope unmounts, on both web and native web views.
- Document that switching branches unmounts their local state. Shared data,
  resources, form signals, and players belong above the switch. Passing an
  Element slot does not itself preserve mounted component identity.

For Tawny specifically, PersistentPlayer must remain outside any such switch.
Keeping all variants mounted and hiding them is not a good default: it keeps
effects and requests running and can duplicate ids and accessible controls.

### Layout components for the same content

Media Mancer already largely renders each card once. Prefer preserving that
tree and expressing the arrangement with CSS, rather than duplicating entire
pages in AdaptiveLayout.

A PageLayout with main/start/end slots would suit Home's broad structure:
stacked on compact widths, main plus one aside when space permits, and both
asides only when the main column retains its minimum usable width. Sidebars
need bounded flexible widths, rather than two fixed 24rem columns. Empty slots
must not reserve tracks.

Media detail needs more precise ordering than three fixed slots: its hero,
rating, description, and review cards interleave on compact layouts. Prototype
that page before finalizing the API. A typed placement/item API or documented
grid-area pattern may be needed; do not claim a three-slot wrapper alone
solves this ordering. Check keyboard and reading order against visual order.

For repeated cards, prefer GridColumns::Fit with a content-appropriate minimum
width. g3-ui already supports this, including clamping to the available width
on small screens. It can naturally produce one, two, three, or four columns.
Video cards and poster cards need different minimums. Add intermediate sizing
props only where a fixed count is intentional, with explicit scope and
backward compatibility for existing wide props.

Landscape phones also have limited height. Width classes do not replace safe
areas, scrolling, height limits, or touch-friendly controls. Width must not be
used as a proxy for mouse input.

## Documentation site direction

Libero's site presents web, desktop, Android, and iOS support; its desktop-like
documentation layout does not mean the library is desktop-only. Its useful
patterns for g3-ui are visible at <https://libero-ui.dev/> and
<https://libero-ui.dev/about/getting-started>:

- A persistent header with documentation search, Ctrl+K, GitHub, and theme
  controls.
- Navigation grouped into guides, component categories, and hooks.
- A getting-started page with installation, root provider, stylesheet setup,
  platform commands, and feature flags.
- Live examples next to source, with navigable documentation pages.

For g3-ui, add Getting started, Components, Hooks, Layout patterns, and
Platforms. Explain the actual exported public hooks; link to other g3 crates
for functionality owned elsewhere. Generate search entries and source links
from shared page metadata to avoid a second manually maintained catalog.
Search should cover guides, component names, hook names, and useful aliases.

Keep iOS/Material, light/dark, and responsive preview controls prominent.
Add tablet, landscape, and a resizable preview; resizing the documentation
window alone does not test a demo held at a fixed phone width. Keep the docs
shell independently responsive, with a usable mobile menu and search.

Libero currently documents a different Dioxus version. Reuse the information
architecture, not its dependency versions or provider instructions.

## Implementation and verification order

1. Prototype the three-stage scope, AdaptiveLayout, and CSS layout patterns
   against both consuming pages before freezing the public API.
2. Fix Tawny's grid policy and matching skeletons using available card width.
3. Refactor Media Mancer's page arrangements, align hero/page query scope, and
   give shelf cards an intentional responsive size policy.
4. Build the documentation navigation, getting-started and hook pages, shared
   search index, source links, and additional preview sizes.

Verify around each breakpoint, not only at its nominal width: portrait phone,
landscape phone, tablet, half-window desktop, and large desktop; navigation
expanded/collapsed; larger text; iOS/Material; light/dark; keyboard and touch.
Check that rotation preserves shared form data and playback, does not trigger
duplicate requests, and does not leave hidden controls focusable. Include
SSR/first-client agreement and native/web feature builds for any new component.

The two consuming apps were inspected in source for this investigation. Their
responsive changes have not been implemented or visually verified here.
