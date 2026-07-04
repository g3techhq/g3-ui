# g3-ui

Mobile-first Dioxus components inspired by Ionic.

`g3-ui` provides app-shell, card, button, field, select, sheet, modal, segment, toggle, spinner, and floating-action-button components. The Rust crate name is `g3_ui`.

## Install

```toml
[dependencies]
dioxus = { version = "0.7.9", features = ["router"] }
g3-ui = "0.1"
```

Enable the optional route-transition integration when your app also uses `dx-route-transitions`:

```toml
[dependencies]
g3-ui = { version = "0.1", features = ["transitions"] }
dx-route-transitions = "0.1"
```

The local checkout includes `.cargo/config.toml` to patch `dx-route-transitions` to the sibling `../dx_route_transitions` repo while developing both crates together.

## Basic Use

`G3AppWrapper` loads the stylesheet, provides the current mode, and applies theme tokens to the root shell.

```rust,ignore
use dioxus::prelude::*;
use g3_ui::{G3AppWrapper, G3Body, G3Button, G3Card, G3Header, Theme};

#[component]
fn App() -> Element {
    rsx! {
        G3AppWrapper { theme: Theme::default_light(),
            G3Header { title: "Games" }
            G3Body {
                G3Card { title: "Pending game",
                    "Invite players and choose a format."
                }
                G3Button { onclick: |_| {}, "Create game" }
            }
        }
    }
}
```

For broad imports, use the prelude:

```rust,ignore
use g3_ui::prelude::*;
```

## Route Transition Integration

With the `transitions` feature enabled, `G3AppWrapper` loads the `dx-route-transitions` stylesheet provider and marks the shell with the cover snapshot class. `G3Body` marks its scrollable content with the segment snapshot class for push transitions.

Without the feature, `g3-ui` does not depend on `dx-route-transitions` and does not emit route-transition marker classes.

## Modes and Themes

Components support Ionic-style `ios` and `md` modes. The mode is chosen when a component initializes and is provided by `G3AppWrapper` or `G3ThemeProvider`.

```rust,ignore
use g3_ui::{ComponentMode, G3AppWrapper};

rsx! {
    G3AppWrapper { mode: ComponentMode::Ios, "..." }
}
```

Theme colors are CSS custom properties generated from `Theme`:

```rust,ignore
let theme = Theme::default_light().with_focused("#22c55e");
```

## Component Names

The crate exports both concise names (`Button`, `Card`, `Sheet`) and prefixed aliases (`G3Button`, `G3Card`, `G3Sheet`). The prefixed names are recommended for downstream apps because they avoid collisions with local components.

## Playground

The local playground remains inside this repository:

```powershell
cargo check --manifest-path playground/Cargo.toml
```