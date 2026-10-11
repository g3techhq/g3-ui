# Release readiness audit — October 10, 2026

Scope: g3-ui, g3-auth, g3-cache, g3-native-plugins, g3-route-transitions, g3-stack, Tawny, Greenside Partee, Media Mancer, and Swift Circuits. This is a source and local validation audit of the current checkouts, including existing uncommitted Tawny changes. No fixes, releases, deployments, or development servers were started.

Follow-up: the findings below describe the original audit state. Fixes,
validation and remaining release checks are recorded in `RELEASE-FIXES.md`.
The sibling g3-auth override is for local development only. It now lives in
parent Cargo configuration outside the repositories; CI and Docker use registry dependencies.

Recommendation: address the P1 findings before releasing the affected apps. Passing local builds currently does not establish that a standalone checkout or published dependency graph works.

## Findings

### 1. P1 — Three applications require a sibling g3-auth checkout

**Evidence:** Greenside Partee `Cargo.toml:10`, Media Mancer `Cargo.toml:148`, and Swift Circuits `Cargo.toml:18` commit a `[patch.crates-io]` pointing to `../g3-auth`. Their CI workflows check out only the application. Their image workflows use `context: .`; the sibling repository is outside the Docker build context.

**Impact:** a developer with all repositories present can build successfully, while a standalone CI checkout or image build cannot resolve the patched dependency. Removing the patch alone is insufficient: the applications request the new `apple` and `rate-limit` features, which are absent from the locally cached published g3-auth 0.1.2 manifest.

**Action:** publish a new g3-auth version containing the required features and security fix, update application requirements and lockfiles, remove the temporary patches, and validate each app from a standalone checkout.

### 2. P1 — Android and iOS workflows request a nonexistent Dioxus package name

**Evidence:** g3-stack `.github/workflows/build-mobile.yml:69,124`, Tawny `:69,124`, and Media Mancer `:71,128` use `"dioxus-cli${DIOXUS_CLI_VERSION}"`, where the variable is `0.7.9`.

**Impact:** the installer receives `dioxus-cli0.7.9`, instead of `dioxus-cli@0.7.9`, so these workflows cannot install the intended CLI. Greenside Partee already uses the correct separator.

**Action:** add `@` in all six commands and exercise both workflow jobs. This is confirmed from string expansion; the remote workflows were not dispatched.

### 3. P1 — Greenside Partee streams private account fields to game viewers

**Evidence:** `src/components/games/pending_game/pending_game.rs:129`, `src/components/live_game/live_game.rs:139`, and `src/components/games/completed_game/completed_game.rs:42` select `array::first(<-user.*) AS user`. `src/db/player.rs:19` serializes that value as `Option<User>`. `src/db/user.rs:96` includes email, Google/Apple IDs, Stripe customer/subscription IDs, and App Store transaction metadata.

The live endpoint macro in `src/db/live.rs:61,127` extracts only the database; it does not check the caller's game membership. `src/db/connection.rs` authenticates the database connection as root. Table `PERMISSIONS NONE` therefore does not replace application authorization.

**Impact:** the response includes fields that a player list does not need, and an authenticated caller can supply another game's record ID without a membership check in this path. The source path is confirmed; a two-account HTTP/WebSocket demonstration was not performed.

**Action:** return a dedicated public player profile projection and explicitly authorize the game before sending the initial snapshot or subscribing. Test unrelated accounts as well as participants, and ensure access changes terminate existing subscriptions when required.

### 4. P1 — Tawny has registration but no account-deletion flow

**Evidence:** `src/api/account.rs` implements guest creation, registration, sign-in, sign-out, and current-account reads. `src/components/account.rs:137` exposes registration and sign-out, but no deletion action. A source/documentation search found no account-deletion implementation.

**Impact:** users can create a permanent account but cannot remove it and its library from the app. Treat this as a mobile release prerequisite; g3-stack already provides an example of transactional account deletion and schema coverage tests.

**Action:** add authenticated deletion, confirmation, a transaction covering account-owned data, session invalidation, and client cache cleanup. Verify deletion from a second device and during active playback.

### 5. P1 — Tawny authentication endpoints have no application-level throttling

**Evidence:** `src/api/account.rs:13,33,50` creates accounts and checks passwords. `src/auth/accounts.rs:85,93` performs Argon2 hashing/verification. `src/main.rs:25` installs session/auth/cache layers but no rate limiter; no authentication limiter was found elsewhere in `src/`.

**Impact:** repeated public sign-in requests can consume password-hashing work, and guest creation can create unlimited account rows. An external proxy may mitigate this, but that was not established in this audit.

**Action:** apply separate limits to guest creation, registration, and sign-in. Use a trusted address source for signed-out traffic and test a returned 429. Consider moving password hashing off the asynchronous request executor as a separate performance check.

### 6. P1 — The new g3-auth release still has the already-published version

**Evidence:** `g3-auth/Cargo.toml:3` remains `0.1.2`; `CHANGELOG.md:7` has unreleased Apple auth, rate limiting, shared credential validation, and the Google callback CSRF fix. A cached registry package already exists as g3-auth 0.1.2 and lacks the new features.

**Impact:** the updated source cannot be published under the existing version, and applications cannot obtain the new behavior from the existing release. This also explains the temporary sibling patches.

**Action:** assign a new version, finalize its changelog, publish through the authorized workflow, then migrate the consumers. Do not release the apps against the old callback behavior by merely dropping the patch.

### 7. P2 — Native-plugin strict lint misses browser-only failures

**Reproduction:** from g3-native-plugins, `cargo clippy --all-features --target wasm32-unknown-unknown --no-deps --offline --locked -- -D warnings` fails.

**Evidence:** `src/storage.rs:213` triggers `new_without_default`; `src/external_url.rs:82,87` lacks method documentation. Its host `just pre-push` passes because those implementations are target-gated.

**Action:** fix the three warnings and add an actual WASM strict-lint job. Host checks with browser features are not a substitute for compiling browser-gated code.

### 8. P2 — Android store queries discard error results

**Evidence:** g3-native-plugins `src/android/in_app_purchases/src/main/kotlin/dev/dioxus/g3_native_plugins/in_app_purchases/InAppPurchasesPlugin.kt:188,261` ignores the `BillingResult` argument of product and purchase queries, then publishes the collected list after both callbacks.

**Impact:** a failed query can be reported as a successful empty or incomplete catalogue/entitlement list. The caller cannot distinguish a store outage from owning nothing or a missing product. This was identified in source; no store outage was simulated on a device.

**Action:** inspect both results, report errors through the existing result envelope, and preserve the last verified state on failure. Cover failure of either half of the INAPP/SUBS queries.

### 9. P2 — Client cache storage grows with every distinct read

**Evidence:** g3-cache `src/client/mod.rs:177,212` stores answers in a process-lifetime `BTreeMap`. Entries are cleared on owner changes but have no size limit or eviction. `keep_for` rejects old persistent answers when loading; it does not remove them. Persistent backends provide load/save/whole-store-clear, but no expiry sweep.

**Impact:** browsing many media, video, or paginated keys accumulates memory and disk usage for the same account. This is a design limitation confirmed from source, not a measured memory leak.

**Action:** add configurable entry/byte limits and eviction, plus persistent expiry cleanup. Measure a long browsing session before choosing defaults.

### 10. P2 — Swift Circuits fails the formatting gate

**Reproduction:** `cargo fmt --all -- --check` reports changes in `crates/game/src/app.rs:25` and `crates/game/src/main.rs:158`.

**Action:** format those files and rerun CI. The earlier locked-build error was caused by running under g3-ui's local override; it is not a Swift Circuits lockfile defect.

## Cross-project discrepancies and validation gaps

- **Local versus release dependency graphs:** ignored `.cargo/config.toml` overrides exist in g3-ui, Tawny, Greenside Partee, and Media Mancer. Some apply; others warn that their patch is unused. For example, g3-ui's root check uses local transitions 0.4.4 while its playground retains a registry graph. Media Mancer checks resolve registry g3-ui 0.5.0 even though local overrides reference 0.5.1. Record and test both the local integration graph and the final published graph. Version differences within compatible ranges are not automatically defects.
- **Native coverage:** g3-native-plugins CI checks Rust facades on Android and Apple targets, but those `cargo check` commands do not establish that the Kotlin/Swift implementations compile, package, and work on a device. Add native testbed bundles and device smoke results for auth, purchases, storage, media, and Back before calling native releases verified.
- **Publishing gates:** the five library publish workflows do not explicitly depend on their quality jobs or validate that the selected commit passed CI. The multi-crate dry runs list the parent package rather than compiling its archive. Once the macros version exists, perform a full parent package verification and require a successful quality result for the published commit. Repository environment protections were not inspected.
- **Minimum Rust versions:** g3-ui declares 1.88; g3-stack declares 1.89; Tawny declares 1.90; most libraries omit a minimum. Local checks use 1.95.0. Verify claimed minimum versions separately; this audit did not establish that they are incorrect.
- **Shared-file drift:** the server URL implementations in g3-stack, Media Mancer, and Greenside Partee match in behavior; their tests differ. The shared error-suppression implementation matches. Do not treat different file hashes or app-specific additions alone as bugs.
- **Tawny work in progress:** its player, transport, state, and lockfile had pre-existing uncommitted changes. The audit uses those files. A release must choose a reviewed commit and validate it with registry dependencies.
- **Cache owner races need a focused test:** `set_cache_owner` clears memory, then awaits persistent clear/save before notifying readers. Reads/restores in flight and concurrent owner changes are not tied to an owner generation. Test rapid A→B→signed-out changes with delayed requests and IndexedDB transactions; no cross-account race was reproduced in this audit.

## Local validation

| Project | Results obtained |
| --- | --- |
| g3-ui | Full `just pre-push` passes; WASM all-features check passes. 117 unit/markup tests per feature run; 78 passing doctests, one ignored. |
| g3-auth | Full `just pre-push` passes, including server/social/rate-limit and WASM checks. 62 tests and 11 macro tests pass; documentation examples are ignored. |
| g3-cache | Full `just pre-push` passes, including WASM, desktop/mobile features, and testbed checks. 12 server tests, five client/key tests, and ten macro tests pass. |
| g3-native-plugins | Full host `just pre-push` passes: 23 default tests and 43 all-feature tests. WASM check passes with warnings; WASM strict lint fails as described above. |
| g3-route-transitions | Full `just pre-push` passes: 56 tests and three doctests. Additional WASM all-features check passes. |
| g3-stack | Prescribed web/server/mobile checks and `just test` pass, including template script tests. |
| Tawny | Web/server/mobile/desktop checks and `just test` pass. The server test run passes 127 tests, with one skipped. |
| Greenside Partee | Prescribed build checks and information-site synchronization check pass with the Windows assembly workaround. |
| Media Mancer | Prescribed web/server/mobile checks pass with the Windows assembly workaround. |
| Swift Circuits | Formatting fails; locked workspace client and server checks pass with the Windows assembly workaround. |

The Windows workaround was `AWS_LC_SYS_PREBUILT_NASM=1`, already documented in g3-auth's repository configuration. Initial app checks without it failed because NASM is unavailable here; those failures are environmental, not application code defects.

Not completed: browser/gesture/keyboard review, authenticated two-account runtime tests, Android/iOS builds and device tests, application-wide strict lint, all application tests, cargo-deny/advisory scans, published archive checks, clean remote CI runs, deployment checks, and production configuration verification. No development server was started. Passing results above are not a complete release certification.

## Suggested release order

1. Fix the private player projection/authorization and Tawny account/authentication gaps.
2. Version and publish g3-auth with its security fix; update the dependent applications and remove sibling patches.
3. Fix mobile installer commands, browser strict lint, Android store error handling, and Swift Circuits formatting.
4. Validate standalone clean checkouts using final registry lockfiles; run strict lint, tests, archive checks, and actual web/native bundles for that commit.
5. Run device and two-account smoke tests, then use the existing release workflows after maintainer authorization.
