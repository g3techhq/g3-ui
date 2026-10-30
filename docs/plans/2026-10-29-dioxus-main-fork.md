# Dioxus fork: track main until 0.8.0

Status: proposed 2026-10-29. Nothing below has been started.

Move every g3 crate and app off Dioxus 0.7.9 onto a fork of Dioxus `main` with
the open mobile-release and robustness PRs merged in. Stay on the fork until
Dioxus 0.8.0 is published, then drop it.

## Scope

| Kind | Repositories |
| --- | --- |
| Fork | `Gateway3B/dioxus` (exists; a plain mirror of upstream main, 41 commits behind, no commits of its own) |
| Crates | g3-ui, g3-route-transitions, g3-native-plugins, g3-cache, g3-auth, g3-brand |
| Apps | media-mancer, tawny, greenside-partee, donation-engine, swift-circuits, ad-sentinel |
| Template | g3-stack |

All of them pin `dioxus =0.7.9` today (rill is on 0.7.3 and out of scope).

## Decisions (2026-10-29)

1. **Fork base: upstream `main`, with our commits on top.** The fork's `main`
   is upstream `main` plus our commits; new upstream changes come in by
   rebasing our commits onto upstream. Upstream main is past the
   `dioxus-core` → `dioxus-vdom` rename (#6021, #6024), so code uses the new
   names from the start. The workspace version is still `0.8.0-alpha.3`.
   Consequence: `dioxus-icons 0.2.0-alpha.3` depends on
   `dioxus-core-template`, which the rename removed, so it needs a fork
   with that dependency renamed (see Phase 2). `dioxus-sdk-time` and
   `dioxus-code` depend only on `dioxus` and are patched with it.
2. **g3 crates move `main` to the fork.** Tag the last 0.7-compatible commit
   in each, publish nothing to crates.io until 0.8.0 (a crates.io release that
   only builds with a `[patch]` is broken for anyone else), and pin the crates
   from the apps by git `rev`.
3. **The fork stays at `Gateway3B/dioxus`.**
4. **Wasm splitting is on hold** unless fullstack hydration works on main or a
   fix exists that is worth carrying (see Phase 6). The earlier attempt's
   error was never recorded.

## Phase 0: fixes that do not wait for the fork

Each of these is independent and works on 0.7.9.

- **Client release profiles are not tuned, and one is misnamed.** `dx` builds
  the web client with a profile named `wasm-release` and the server with
  `server-release` (`packages/cli/src/platform.rs`, `profile_name`). Its own
  doc comment says `web-release`, which is wrong. greenside-partee defines
  `web-release` with `opt-level = "s"`, so that setting never applies. The
  other apps define no release profile, so their wasm gets Cargo's defaults:
  `opt-level = 3`, no LTO, 16 codegen units, unwinding panics. Add to every
  app:

  ```toml
  [profile.wasm-release]
  inherits = "release"
  opt-level = "z"        # measure "s" too; see Phase 5
  lto = "fat"
  codegen-units = 1
  panic = "abort"        # client only: the server must keep unwinding
  debug = false

  [profile.android-release]
  inherits = "release"
  opt-level = 3          # phones run the UI here; size matters less than CPU
  lto = "fat"
  codegen-units = 1

  [profile.ios-release]
  inherits = "android-release"
  ```

  Leave `server-release` unwinding. Rename greenside's `web-release` to
  `wasm-release`.
- **Docker dependency caches miss.** media-mancer and donation-engine run
  `cargo chef cook --release` (host target, `release` profile), but `dx bundle`
  then builds `wasm-release` for `wasm32-unknown-unknown` and `server-release`
  for the host. Neither reuses the cooked layer: in media-mancer's 2026-10-18
  staging build, `dx bundle` recompiled all 399 crates after a 5m51s cook and
  took 27 minutes, about 20 of them on the server. greenside cooks the right
  targets but under `web-release`. Cook `--profile wasm-release --target
  wasm32-unknown-unknown --features web` and `--profile server-release
  --features server,web`, as greenside does, in every app's Dockerfile.
- **Turn on pre-compression.** The fullstack server already serves
  `ServeFile::precompressed_br()` and marks hashed assets immutable, but no
  app sets `[web] pre_compress = true` in `Dioxus.toml`, so nothing `.br`
  exists to serve. Today compression depends on the proxy: Cloudflare in front
  of media-mancer, a reverse proxy in front of Tawny. Build-time brotli at
  quality 11 is smaller than on-the-fly compression and does not depend on
  proxy settings.
- **Release logger level.** `dioxus-signals` creates INFO-level spans for every
  `Signal::new` and every memo recompute, and Dioxus's default logger runs at
  INFO in release. On web each span is a `performance.mark` plus `measure`
  left in the browser's buffer. Call `dioxus::logger::init(Level::WARN)` on the
  client in release builds before `launch`. Upstream fixed it in #5930, so
  this is only needed until the fork lands.
- **Play target API.** Every app sets `target_sdk = 35`. Google Play's yearly
  rule usually requires the previous year's API level from late August, which
  would be API 36 for 2026. Confirm in Play Console before the first upload.
  #5906 moves the Android template to AGP 9 and API 37.
- **iOS runner SDK.** `build-mobile.yml` runs on `macos-15`. App Store uploads
  require the current Xcode SDK; check which Xcode the job selects, or move to
  a `macos-26` runner.

## Phase 1: build the fork

### Layout

- The fork's `main` is upstream `main` plus our commits on top. Bring in
  upstream changes with `git rebase upstream/main` and force-push.
- Apply each upstream PR as **one squashed commit** titled
  `g3: <what> (DioxusLabs/dioxus#NNNN)`. Merge commits do not survive a plain
  rebase, and one commit per PR is easy to drop once it lands upstream (the
  rebase then reports it empty, or `git rebase --onto` skips it). Our own
  patches use the same `g3:` prefix without a PR number.
- `G3-PATCHES.md` at the fork root lists every commit: PR number, why we need
  it, upstream status, conflict notes. Review it on each rebase.
- **Tag every state anything consumes**, e.g. `g3-2026.11.02`, and pin the
  tag's commit hash. A rebase rewrites `main`, and Cargo cannot fetch a
  commit that no branch or tag reaches any more, so an untagged pin breaks
  the next clean build.
- Rebase every two weeks or when upstream tags an alpha. When upstream bumps
  past `0.8.0-alpha.3`, the ecosystem crates' exact pins need their new
  alphas in the same step.

### The `dx` CLI

`dx` must be built from the same commit as the libraries. A workflow in the
fork builds `dioxus-cli` for `x86_64-unknown-linux-gnu`,
`aarch64-apple-darwin` and `x86_64-pc-windows-msvc` on every `g3-*` tag and
attaches the binaries to a GitHub release. CI jobs and Dockerfiles download
that release instead of running `cargo binstall dioxus-cli@0.7.9`, which avoids
a roughly ten-minute `cargo install --git` on every run. Locally, run `cargo
install --git <fork> --tag <tag> dioxus-cli --locked`.

### Upstream PRs to merge

"Clean" means it merged onto `7bd964ae9`, five commits before upstream's
current `main`, without conflicts (checked 2026-10-29 with `git merge-tree`).
Re-check against the rename: PRs touching `packages/core` now apply to
`packages/vdom`. Conflicts list the files involved.

**Store release (required before any store upload)**

| PR | What it fixes | Merge |
| --- | --- | --- |
| #5735 | `versionCode` is hard-coded to `1`, so Play rejects every upload after the first; adds `CFBundleVersion` for iOS | clean |
| #5591 | `dx bundle --ios` produces an `.ipa` Apple's validator rejects; adds `--appstore` signing | clean |
| #5864 | Locally built Swift plugin frameworks are unsigned, so device installs fail. g3-native-plugins ships Swift plugins | clean |
| #5901 | iOS apps never get an icon; App Store upload fails its icon check | clean |
| #5693 | Android icon generation | conflict: `Cargo.lock` only |
| #5906 | Android template to AGP 9 and API 37 | clean |
| #5972 | APKs package every ABI instead of the one built | clean |
| #5861 | `[android]` manifest, resources and raw keys are parsed but never applied | conflict: `AndroidManifest.xml.hbs` |
| #5960 | Release builds kill Android's text-selection toolbar (cut/copy/paste) | clean |
| #5893, iOS half | Apps built with the iOS 27 SDK are killed at launch without a `UIApplicationSceneManifest`. The PR targets `v0.7`; main already has tao 0.37, so port only the Info.plist manifest as a `g3:` commit | port |

**Mobile robustness**

| PR | What it fixes | Merge |
| --- | --- | --- |
| #5862 → #5886 → #5863 → #5890 | One stack. A recreated Android activity or an iOS/macOS web-content-process kill leaves a blank page for good; this redraws it with component state kept | conflicts in `desktop/src/mobile.rs`, `desktop/Cargo.toml`, `manganis/src/android/activity.rs`, Android manifest template. Main's tao 0.37 upgrade (#5936) already did part of #5862 (`set_activity`); keep main's version and add the rest |
| #5812 | Edits stop after the edit websocket reconnects (iOS resume); edits now survive until acknowledged | conflict: `desktop/src/edits.rs`, CI workflow |
| #5714 | On some Android devices the entry point resolves to a system library's `main` and crashes at launch | clean |
| #5895 | The interpreter keeps removed DOM subtrees, listeners and observers alive; memory grows with mount/unmount cycles | clean |

**Build correctness**

| PR | What it fixes | Merge |
| --- | --- | --- |
| #6027 | A failing `wasm-opt` is swallowed and the un-optimized wasm ships (#5705, the reason for the `--debug-symbols false` workaround in the Dockerfiles) | clean |
| #5912 | Release builds default to no debug symbols | clean |
| #5900 | Bundled JS is hashed by its esbuild output, so changed JS gets a new URL | clean |

**Wasm splitting (Phase 6 only)**

#5770, #5771, #5772, #5795, #5975, #5990, #5991, #5992 merge clean. #5738 and
#5668 conflict: #5738 in `wasm-split-cli/src/lib.rs`, #5668 in core suspense
(`diff/component.rs`, `runtime.rs`, `suspense/component.rs`), which #5554
rewrote. #5668's splitter half may apply alone; its suspense half needs
re-porting. Merge these only once the Phase 6 spike says splitting works.

**Not merging:** #5587 (background throttling only affects macOS hosts),
#5475 (signal rerun scoping; useful, but a reactivity change with no
measured gain for us). Revisit #5475 if profiling shows extra reruns.

### Verification for the fork itself

- `cargo check --workspace` and `cargo test -p dioxus-desktop -p dioxus-cli`
  on the `g3` branch after each merge.
- A hello-world from g3-stack built with the fork's `dx` for web, Android
  (emulator) and iOS (simulator) before tagging.

## Phase 2: move the g3 crates

Order follows the dependency graph: g3-native-plugins, g3-cache, g3-auth,
g3-route-transitions, g3-brand, then g3-ui.

### Shared steps per crate

1. Tag the current `main` as the last 0.7-compatible release
   (`v<version>-dioxus-0.7`).
2. Change every `dioxus*` and `manganis` requirement to `=0.8.0-alpha.3`.
3. Add the fork patch block (below) so the crate's own CI builds against it.
4. Bump dependents to the new crate commit by git `rev`.
5. Edition 2024 where the crate is still 2021 (main moved to 2024 in #5502;
   not required, but it matches the macros' expansions).
6. Definition of done as each repo already defines it, plus a build of its
   testbed or playground with the fork's `dx`.

### The patch block

Every crate and app gets the same block, generated once and kept in g3-stack
as the source of truth. A `[patch]` is only needed for crates that something
in our graph requests from crates.io; crates the fork depends on internally
resolve inside the fork automatically. Cargo warns about unused entries;
that is expected.

```toml
[patch.crates-io]
# Dioxus fork: upstream main + our commits. See G3-PATCHES.md in the fork.
dioxus = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
dioxus-fullstack = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
dioxus-ssr = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
dioxus-html = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
dioxus-signals = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
dioxus-cli-config = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
manganis = { git = "https://github.com/Gateway3B/dioxus", rev = "<tag commit>" }
# dioxus-icons 0.2.0-alpha.3 depends on dioxus-core-template, renamed to
# dioxus-vdom-template on main. A fork with that one dependency renamed:
dioxus-icons = { git = "https://github.com/Gateway3B/dioxus-icons", rev = "<commit>" }
```

`dioxus-sdk-time` and `dioxus-code` need no entry: they depend only on
`dioxus`. Check with `cargo tree -d -i dioxus`: exactly one `dioxus` must
appear in every app.

### Per-crate changes

- **g3-native-plugins.** On the fork, `ndk_context` holds the Android
  Application, not the Activity (#5862's direction, already half on main).
  `android_bridge.rs` (`AndroidPlugin::new`), `back_button.rs` (lines 85,
  144, 168) and `media.rs` (line 55) cast `android.context()` to an Activity
  and construct Kotlin plugins with it. Switch them to
  `manganis::android::with_activity`, and stop caching a plugin that holds an
  Activity across activity recreation: rebuild it when the Activity changes.
  `getClassLoader` still works on the Application. Bump `manganis` and `jni`
  to what the fork uses. CI installs `dioxus-cli 0.7.9` with
  `cargo install`; point it at the fork's `dx` release.
- **g3-cache.** `src/client/mod.rs:641` renders with
  `dioxus_core::NoOpMutations`. Use `dioxus::core::NoOpMutations`: the
  `dioxus::core` alias exists in 0.7.9, at the fork base and after the
  rename (where it points at `dioxus-vdom`), so the rename needs no change.
  `client/store.rs` uses `ndk_context` only for `getCacheDir`, which works on
  the Application. The two `use_reactive!` calls should become `ReadSignal`
  props (AGENTS.md already asks for this).
- **g3-auth.** Depends on `dioxus-cookie 0.2`, which requires `dioxus ^0.7`
  and has no 0.8 release. Fork it or vendor its small surface into g3-auth.
- **g3-route-transitions.** No API changes expected. Re-run its transition
  tests: the edit protocol now sends nothing for renders that change nothing,
  and markerless placeholders change which DOM nodes exist around
  conditional content.
- **g3-brand.** Version bump only.
- **g3-ui.** `dioxus-icons 0.2.0-alpha.3`, `dioxus-sdk-time 0.8.0-alpha.3`,
  `dioxus-code 0.2.0-alpha.3` replace the 0.7 versions; check the icon API.
  `#[props(extends = GlobalAttributes)]` and `extends = input` still work on
  main. Two `use_reactive!` calls to replace. Run the browser suite at 390x844
  and 1440x900: markerless placeholders (#5554) remove the comment nodes that
  used to sit where conditional content and empty lists render.

## Phase 3: move the apps

### Every app

- Replace `=0.7.9` pins with `=0.8.0-alpha.3`, add the fork patch block, and
  pin the g3 crates by `rev`.
- CI (`pr-ci.yml`, `build-mobile.yml`, `publish-staging.yml`,
  `prod-deploy.yml`) and the Dockerfile: download the fork's `dx` release
  instead of `cargo binstall dioxus-cli@${DIOXUS_CLI_VERSION}`.
- Remove the client logger workaround from Phase 0 (fixed upstream by #5930).
- Keep the Dockerfile's `--debug-symbols false` until a release build is
  confirmed to run `wasm-opt` (see Phase 5); then drop it (#5912, #6027).
- Update `AGENTS.md`: the version table, the "pin `dioxus*` exactly
  (`=0.7.9`)" rule (it becomes "pin the fork rev"), and refresh
  `docs/dioxus/` from the fork.
- Read the `dx serve` output at the first run: hot-patching is on by default
  on main (#5506). Earlier notes say it failed on fullstack web
  ("Missing server jump table"); main fixed several fullstack hot-patch bugs
  (#5790, #5843). If it still fails, use `dx serve --hotpatch false`.
- Definition of done per `AGENTS.md`, plus an Android emulator and iOS
  simulator launch.

### Per app

| App | Extra work |
| --- | --- |
| media-mancer | `dioxus::dioxus_core::spawn_forever` in `profile.rs` (300, 371) and `media_feed.rs` (364): use `dioxus::core::spawn_forever`, which keeps its name through the later rename. `connectors/store.rs` uses `ndk_context` for a directory only; fine. |
| tawny | Remove the `set_server_url` "first value wins" workaround (`server_url.rs`, `main.rs:115`, `components/account.rs:31`); #5704 lets it be set again. `local_streams.rs:159` casts the context to an Activity: move to `with_activity`. |
| greenside-partee | `dioxus-time` (git, dioxus-std `0.7` branch) → `dioxus-sdk-time 0.8.0-alpha.3`. `dioxus-primitives` comes from `DioxusLabs/components`, still on 0.7.8: fork and bump it, or replace what greenside uses. Rename `web-release` → `wasm-release` (Phase 0). |
| donation-engine | Uses every g3 crate; no Dioxus-specific code found. Docker cook fix from Phase 0. |
| swift-circuits | `dioxus-sdk-time =0.7.0` → `0.8.0-alpha.3`. Web assets live in `web/`; check its `Dioxus.toml` paths still resolve. |
| ad-sentinel | `dioxus-sdk-time` as above. Only app with `min_sdk = 28` and `[permissions]`; recheck after #5861 starts applying `[android]` keys. |
| g3-stack | The template's `.liquid` twins rely on the `cargo-generate` embedded in `dx` (0.23.3). Main moved to 0.25 (#5944): run `dx new` from the fork against the template before tagging. CI pins `cargo-generate@0.23.3`. |

## Phase 4: store-ready mobile releases

Today `build-mobile.yml` produces an unsigned APK/AAB and an unsigned IPA.
With the Phase 1 PRs:

- **Icons.** iOS (#5901) takes the largest raster image from `[ios] icon`,
  else `[bundle] icon`, and rejects transparency. greenside, donation-engine,
  ad-sentinel and g3-stack point `[bundle] icon` at an SVG only;
  media-mancer and tawny have none. Each app needs a 1024x1024 opaque PNG for
  iOS and the Android sources #5693 expects. g3-brand is the natural place to
  generate them.
- **Version numbers.** With #5735, set
  `DX_ANDROID_VERSION_CODE=${{ github.run_number }}` and
  `DX_BUNDLE_VERSION=${{ github.run_number }}` in the release jobs, so every
  upload has a higher build number than the last.
- **Android signing.** `dx` already supports a keystore under
  `[android.signing]` (`jks_file`, `jks_password`, `key_alias`,
  `key_password`; it was `[bundle.android]` in 0.7). The passwords are plain
  values in `Dioxus.toml`, so the committed file must not have the section:
  the release job appends it from GitHub secrets and writes the keystore to
  the runner. Use Play App Signing with an upload key. The generated Gradle
  file also contains the passwords, so never upload the build directory as an
  artifact.
- **iOS signing.** `--appstore` (#5591) with an Apple Distribution
  certificate and an App Store provisioning profile imported into a
  temporary keychain on the macOS runner. Swift plugin frameworks are signed
  first (#5864). Upload with an App Store Connect API key.
- **SDK levels.** `target_sdk` per Play's current rule (Phase 0) and the
  iOS scene manifest (Phase 1) before building with the iOS 27 SDK.
- **Device checks before each first submission:** rotate and toggle dark mode
  (Android activity recreation, #5862 stack); leave the app in the background
  for several minutes, then resume and interact (#5812, #5890); select text
  in a release build (#5960); install the IPA on a physical iPhone (#5864).

## Phase 5: bundle size

### Where media-mancer stands (measured 2026-10-29)

The web client (`--no-default-features --features web`, wasm32), built with
the Phase 0 profile (`opt-level = "z"`, fat LTO, one codegen unit,
`panic = "abort"`), then wasm-bindgen 0.2.126 and Binaryen 123 `wasm-opt -Oz`:

| Stage | Raw | gzip -9 | brotli -q 11 |
| --- | ---: | ---: | ---: |
| after wasm-bindgen | 7,423 KB | 2,276 KB | 1,541 KB |
| after `wasm-opt -Oz` | 5,115 KB | 1,934 KB | 1,335 KB |
| after `wasm-opt -O3` | 5,696 KB | 1,934 KB | 1,321 KB |

For comparison, Cargo's default release settings (what `wasm-release`
inherits today) through the same tools:

| Stage | Raw | gzip -9 | brotli -q 11 |
| --- | ---: | ---: | ---: |
| after wasm-bindgen | 11,912 KB | 2,931 KB | 1,808 KB |
| after `wasm-opt -Oz` | 8,025 KB | 2,923 KB | 1,756 KB |

The Phase 0 profile cuts the download by 24% (1.76 → 1.33 MB brotli) and
the wasm the browser has to compile by 36% (8.0 → 5.1 MB).

So the transfer size is about 1.3 MB with brotli. The Dioxus guide's 234 KB
figure is for a TodoMVC app; ours carries g3-ui, g3-cache, g3-auth,
fullstack server-function clients, reqwest and the SurrealDB types.
`-O3` transfers about the same as `-Oz` and may run faster; measure both on a
phone before choosing.

Where the bytes go (twiggy, before `wasm-opt`): the app's own code is the
largest crate at 1.27 MB, spread over screens (profile 148 KB, media 140 KB,
lists 113 KB, shared 158 KB). Then `subsecond` 495 KB, `dioxus-hooks`
355 KB, `dioxus-core` 292 KB, `g3-ui` 245 KB, `dioxus-signals` 242 KB.

### Steps, in order of return

1. **Phase 0 profiles and `pre_compress`.** Required for any of the numbers
   above to reach users.
2. **A client-side `RecordId`.** Measured 2026-10-29; see "SurrealDB types
   in the client" below. Saves 948 KB of wasm and 227 KB of brotli download
   in media-mancer without changing how code uses database types.
3. **Check `subsecond` in release.** 495 KB is attributed to the hot-patch
   runtime in a release build. Most of it is probably component bodies
   inlined into its call wrappers rather than dead weight. Build once with
   Dioxus's `devtools` feature off (`default-features = false` plus the
   needed features; #6028 made web's devtools optional on main) and compare.
4. **A size budget in CI.** Record the brotli size of `*_bg.wasm` on every
   release build and fail above a budget, so regressions show up in review.
5. **Wasm splitting** (Phase 6) for anything beyond that.

**Under 1 MB:** steps 1 and 2 are measured at 1,108 KB brotli (from 1,756 KB
today). The `subsecond` check and smaller trims are the next candidates;
below 1 MB beyond that would need splitting, which is on hold (Phase 6).
Mobile is unaffected: Android and iOS ship native code.

### SurrealDB types in the client

Status (2026-10-30): implemented, uncommitted. g3-auth has a `record-id`
feature (`src/record_id.rs`, six wire tests) and media-mancer imports
`g3_auth::RecordId` behind a temporary path patch until g3-auth is pushed.
The other apps switch the same way.

**Cause.** `surrealdb-types` has no optional features: regex, geo,
flatbuffers, rust_decimal, papaya, ulid and the rest are always compiled in.
Most of that only reaches the wasm through one path. Row types hold
`RecordId`s, and a `RecordId` key may be an array or object of any SurrealDB
`Value`. So deserializing a `RecordId` includes `Value`'s full deserializer,
which compiles regex patterns (`Value::Regex`) and parses geometry.
`RecordId`'s `PartialEq`, `Hash`, `Ord` and `Clone` bring in `Value`'s versions
the same way. twiggy confirms the chain: `AppStateProvider` → `User`
deserialize → `RecordId` deserialize → `Object` → `Value` → `Regex::from_str`
→ `RegexBuilder::build`. The app never creates such keys: every key is a
string.

**Fix.** On the server, `RecordId` stays SurrealDB's. On the client it is a
copy with the same serde shape that only models string, number and UUID
keys, and implements `SurrealValue` by delegating to SurrealDB's type, so
row types keep `derive(SurrealValue)` unchanged:

```rust
#[cfg(feature = "server")]
pub use surrealdb_types::{RecordId, RecordIdKey};
#[cfg(not(feature = "server"))]
pub use client::{RecordId, RecordIdKey, Table};
```

`surrealdb-types` stays a client dependency for `Datetime`, `SurrealValue`
and the derive macro; code that is not reachable costs nothing in the wasm.

**Measured** in a copy of media-mancer (the working tree including
uncommitted changes, same local g3 crates), with the Phase 0 profile and
`wasm-opt -Oz`:

| | Raw | gzip -9 | brotli -q 11 |
| --- | ---: | ---: | ---: |
| Before | 5,115 KB | 1,934 KB | 1,335 KB |
| Client `RecordId` | 4,167 KB | 1,581 KB | 1,108 KB |

regex, geo, rust_decimal and flatbuffers no longer appear in the binary;
`surrealdb_types` code is 0 KB and the data section fell from 971 KB to
628 KB. The change was 43 import rewrites plus one 170-line module; client
and server both compile.

**Wire check.** A round-trip test against `surrealdb-types 3.2.4` produced
byte-identical JSON (server functions) and CBOR (hydration data) in both
directions for string, number, UUID and escaped-character keys; `to_sql()`
matched; an array key failed to deserialize instead of being misread.

**Where it lives.** Every app has `surrealdb-types` as an unconditional
dependency and uses `RecordId` in shared code (greenside 54 files,
donation-engine 25, swift-circuits 12, media-mancer 46). Put the type in
**g3-auth**: every app depends on it, it already pins `surrealdb-types` for
its server side, and `session_user.record_id()` already returns a
`RecordId`. Apps then import `g3_auth::RecordId` instead of
`surrealdb_types::RecordId`. Add the round-trip test to g3-auth's tests, and a
test that fails if `surrealdb-types` gains a key variant the copy does not
know about, so a version bump cannot silently change the wire format.

## Phase 6: wasm splitting (spike first)

Splitting only helps the web build; Android and iOS ship native code.
`#[wasm_split("/path")]` on a route (Dioxus 0.7 release notes) moves that
route's code into a chunk fetched on first visit; `dx` needs `--wasm-split`.

### Why it probably failed before

Open upstream issues that hit our setup directly:

- **#4631: fullstack + wasm-split hydration fails.** Every g3 app is
  fullstack. No fix PR exists, and the only report since (2026-08-05) predates
  #5554. The JS function that crashed (`hydrate_node`) no longer exists on
  main: hydration was rewritten in Rust. But upstream's only wasm-split test
  (`playwright-tests/wasm-split-harness`) is client-rendered, so nothing
  checks fullstack + split today.
- **#4769: `--wasm-split` panics with `codegen-units = 1`.** That is the
  Phase 0 size profile. Split builds may need more codegen units.
- **#5993: a failed chunk request never settles.** The loader hangs instead
  of reaching an error boundary. No fix PR; we would write one.
- **#5668: crashes on every split route in client-rendered apps**, from
  splitter symbol handling and core suspense; the splitter fixes in Phase 1
  cover the rest of the known cases.

### Status (2026-10-29): on hold

Not worth carrying fixes for. Once the fork exists, a one-hour check on
g3-stack (steps 1–2 below) tells us whether main's hydration rewrite fixed
#4631. If it did, continue; if not, drop splitting until upstream fixes it.

### Check, then spike (on g3-stack)

1. g3-stack on the fork with the wasm-split PRs merged. Split two routes.
2. Check: first load of a split route (server-rendered, then hydrated);
   client navigation to a split route; a chunk returning 404 or aborted;
   two routes sharing a chunk; `codegen-units = 1` versus the default.
3. Check g3-route-transitions: a split route suspends until its chunk
   arrives, so a push transition would wait on the network. It needs to keep
   the old page until the chunk is ready, and the router should prefetch a
   route's chunk when a link to it becomes visible or on `pointerdown`.
4. Go if hydration and failure handling work. Then split media-mancer's
   rarely-first routes (profile, settings, privacy and data export, social,
   list detail) and measure the initial download.

## Exit: when Dioxus 0.8.0 is published

- Drop the patch blocks; depend on `0.8.0` from crates.io.
- Re-check `G3-PATCHES.md`: anything not upstream that we still need becomes
  an upstream PR or stays in a smaller fork with a reason.
- Publish the g3 crates against 0.8.0 (normal release procedure), then move
  the apps to the published versions.
- Drop the `dioxus-icons` fork once a release depends on the renamed crates.

## Risks

- **Unreviewed PRs.** Most PRs merged into `g3` have no maintainer review.
  Each needs our own test, on a device for the mobile ones.
- **Upstream churn.** Main is mid-restructure (crate rename today, trait-based
  HTML elements in #5554). Rebasing past the rename forces the ecosystem
  crates to move at the same time. Rebase on a schedule, not continuously.
- **`dx` and library drift.** A `dx` from a different commit than the
  libraries produces hydration errors. The tagged-release workflow exists to
  prevent that.
- **Exact-pinned ecosystem alphas.** `=0.8.0-alpha.3` pins mean that when
  upstream bumps its version, `dioxus-icons`, `dioxus-sdk-time` and
  `dioxus-code` must move to matching alphas in the same rebase.
- **Two Dioxus versions in one tree.** Any dependency still requiring
  `dioxus 0.7` (`dioxus-cookie`, `dioxus-primitives`, `dioxus-time`) pulls a
  second copy and fails to compile or misbehaves at runtime. `cargo tree -d
  -i dioxus` must show one version in every app.

## Reference: other upstream changes since 0.7.9

From the 2026-10-29 review of the 130 commits on main.

- **Webview events** (webview → Rust) are still a synchronous XHR per Rust
  handler. Keep gestures in JS (g3-ui `gesture.rs`).
- **Webview edits** (Rust → webview): 0.7.9 sends an edit batch after every
  re-render, even an empty one, and blocks the VirtualDom until the webview
  acknowledges on its next animation frame. Main sends only when a render
  wrote something (`send_touched_edits`).
- **Edit protocol rewrite** (#5554): no placeholder comment nodes; templates
  live in core. Not shown to diff faster than 0.7; it introduced a slowdown
  later fixed by #5802.
- **tao 0.37 / wry 0.57** (#5936): Android drains its event queue every turn,
  fixing freezes after lifecycle events.
- **Store-prop leak** (#5711): mapped stores passed as props leaked per
  render. We pass none today.
- **Portals and multiwindow** (#5554): portals only target desktop windows,
  so they cannot replace g3-ui's overlay layer yet.
- **New events:** `beforeinput`, selection events, clipboard data on paste.
- **Logging:** signal spans moved to TRACE (#5930); web logs use real console
  levels (#5921).
- **Dev loop:** hot-patching on by default (#5506); `rust-lld` on Windows
  fixed (#5242).
