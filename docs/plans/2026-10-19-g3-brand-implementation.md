# g3-brand implementation handoff

Updated 2026-10-21. The shared foundation, B2 transport, Media Mancer catalog and
legal review, scoped account download, durable opt-in deletion/correction queues
and replay are implemented. Reviewed consent purposes, remaining app
integrations and production verification remain pending.

## Saved implementation

- Private repository: https://github.com/g3techhq/g3-brand
- Current verified library commit: `07d3e99391fcafabfcc6b5e0a044ac9ab4f54d7d`.
- The deployed-image candidate below was built from `97d9f22`; it is separate
  from the newer account-status helpers used by the pilot app.
- Successful GitHub checks and image publication:
  https://github.com/g3techhq/g3-brand/actions/runs/39347353835
- Exact image:
  `ghcr.io/g3techhq/g3-brand@sha256:2bccd4d7ca6e1f796be96df682a02a83e53bf198eb42f40cba8df46147f5f6ba`.
- Platform example configuration pins this image. Production configuration and
  services have not been changed.

| Project | Implemented changes |
| --- | --- |
| g3-brand | Public catalog service and publication tooling; immutable legal revisions; per-app receipt/consent helpers; native update evaluation; themed discovery/legal/privacy components; cache/storage contracts; private-build templates and CI |
| g3-platform | Database isolation; encrypted B2 backups with checksum/retention verification; separate recovery writer/reader keys; history-aware ledger replay; encrypted backup retrieval; quarantined restore tools; journal/trace collection; deployment overlays and operator guides |
| g3-native-plugins | Optional app-info with installed iOS/Android version/build metadata, bridges, tests and testbed card; draft PR 23, unreleased |
| g3-stack | Database-scoped authentication and private integration guidance; template retains public dependencies |
| g3-auth | Opt-in account-indexed sessions, legacy session indexing and transactional retirement fences; existing default adapter stays compatible |
| Media Mancer | Public About/catalog/native store notices; account-bound legal review; scoped account download; tracked session retirement and DB write guards; durable opt-in deletion/processor and profile-correction retries, private status receipts and isolated replay; private-build wiring and portable lockfile |
| Greenside Partee, Swift Circuits | Database-scoped authentication support, preserving existing local root configuration as a compatibility option |

g3-brand is committed and pushed. Native app-info is committed as
`1896034867ae692db0503c38136cfbcfc804448a` and local main now includes it, with draft review
https://github.com/g3techhq/g3-native-plugins/pull/23. Verified foundation changes
are committed on main: platform `8095e2c`, template `228ead0`, Greenside `812a910`,
Swift `8089cde`, Media Mancer catalog `1134eab` and legal review `280d9fc`.
Platform legal rollout instructions are in `a9f6a89`.

Additional committed milestones on main:

- g3-auth `9f68e70` (pushed): tracked sessions and retirement fences. Media Mancer
  pins this public Git revision until a maintainer-approved release.
- Media Mancer `953dec9`, `25f9f1a`, `d3b7d38`, `84f37ef`, `973134f`:
  private response/log protections, bounded projected exports, session schema
  checks and transactional write guards against deleted-data resurrection.
- Media Mancer `605ddf2`: recovery-first deletion queue, leased retries, durable
  Apple cleanup and minimal read-only status receipts. A queued request does not
  mean completed cleanup. Copied moderation reports keep the request open.
- Media Mancer `4fd134d` and g3-platform `09ac046`: deletion replay and its
  quarantined, digest-pinned host wrapper. An actual old logical snapshot is
  exported/imported in the isolated DB fixture, then replay removes the restored
  account and sessions while preserving another account. Missing provider evidence
  or copied-report decisions prevents successful release verification.
- Shared API response protections: template `8e37d78`, Greenside `24098f7`,
  Donation Engine `227cd4e` and Swift `02213cc`. Auth user caches are disabled in
  all four consumers so other devices recheck deleted accounts.
- Request telemetry: template `c85038f`, Media Mancer `0db9297`, Greenside
  `cce87ee`, Donation Engine `22b6a49` and Swift `d9d2800`. Only the dedicated
  request target and source-defined route pattern/status/duration are logged;
  dependency events and ambient spans are excluded regardless of `RUST_LOG`.
- Account response minimization: Greenside `95df434` and Donation Engine
  `af28907`. Provider identities stay private; Donation Engine receives a link
  status hint while protected server operations derive it again from DB rows.
- Media Mancer `95658fc`: CI creates a fresh synthetic database, runs all isolated
  recovery fixtures and removes only its captured container ID.
- Media Mancer `fa169f7`, platform `8ff3d11`: durable normalized/moderated profile
  corrections, handle reservations, retry leases, deletion coordination and
  quarantined correction replay. A real old-snapshot import exercises correction
  and deletion while preserving another account. Historical handle conflicts keep
  quarantine closed for reviewed reconciliation.
- Media Mancer `2376799`: Docker contexts exclude local catalog indexes,
  environment files and test reports. An actual context image check passed.
- Media Mancer `2912b0b`, `cf17fdf`: actual Linux application image, encrypted
  old-snapshot correction/deletion recovery drill, and CI publication gate. The
  local drill passed with the exact CI crypto helper; no image was pushed and
  the changed publication workflow has not run on GitHub.
- Greenside `4f8a513`, `c0a5616`, `551750e`: fixed billing failure log labels,
  bounded owner-projected account download and transactionally captured Apple
  grants with durable revocation retries. Completed shared score subjects remain
  intact. Broader deletion/recovery and processor handling remain work.
- Greenside `015e6cc`: tracked sessions, transactionally retired identities,
  durable legacy-payload cleanup and bounded startup backfill. Portable public
  dependency pins pass 310 server/182 web tests and strict checks, including
  WebAssembly and Android. Real HTTP tests verify two warmed devices lose access
  while another account remains signed in.
- Greenside `95d5063`, `f200628`, `5d6eca5`, `03064b6`, `78f955b`, `49b184b`
  and `b06e95e`: retired-profile/reference guards; actor/game transactions for
  scores, lobby actions, creation and pending cleanup; game revisions serialize
  player edits with status/seat changes. Completed shared history stays intact.
- Greenside `6d39167`: deletion confirmation describes shared scores, separate
  operational/provider copies and App Store subscription cancellation accurately.
- Greenside `fb7909e`: fresh transactional joins and organizer-only random owner
  capabilities. Links expire after 24 hours, rotate on replacement and are
  consumed with handoff; public-ID-derived legacy codes are rejected. Issuance,
  link pages and sign-in redirects receive cache/referrer protections.
- Greenside `c81f2d3`: primary account removal preserves bets still used by
  surviving games and serializes with edits to all affected games. The expanded
  real database-editor fixture verifies repeated removal, shared completed
  scores/seats/bets and durable Apple grant capture. Independent restore replay
  and broader processor handling remain work.

The correction milestone passed 234 ordinary plus seven isolated server tests,
104 web tests, strict server/web/WebAssembly checks and a mobile build check.
Platform tests now total 32. Live queued-form interaction remains unverified;
no dx serve was started. The local Linux image and encrypted synthetic-backup
drill passed; these do not verify actual B2 retention or production.

The deletion milestone passed the 237-test server suite (including all isolated
DB fixtures), 104 web tests and strict server/web/WebAssembly lint. The subsequent
two replay tests and strict server lint also pass. Platform tests now total 31.
Receipt previews were checked in iOS/Material, light/dark, at phone and wide sizes.
These are static actual-route previews; no dx serve or live interactive/device
test was started. The later local Linux drill passes; encrypted offsite restore, provider
retention, moderation/evidence procedures and production configuration remain
unverified. Recovery and legal enforcement are disabled by default.
Existing unrelated work is preserved. Media Mancer uses an
intentional pinned native Git patch until a release is approved; verification did
not leave local path dependencies in its portable lockfile.

## Verification

- g3-brand: 21 contract/markup tests, real isolated SurrealDB integration, strict
  lint, WebAssembly UI/client check, Docker build and service smoke test. GitHub
  CI passes for the saved code commit.
- Catalog UI: phone/wide layouts in iOS/Material and light/dark, including keyboard
  focus; static fixtures do not validate native browser/store navigation.
- Platform: 32 Python tests pass, combined Compose configuration and Caddy
  parsing, actual age/SurrealDB export/import/replay drill with synthetic data, and
  Alloy 1.7.5 startup with an empty journal fixture. B2 fixtures cover scoped
  authorization, retention failures, pagination, expired tokens, hidden history,
  corrupt replacements and multipart checksum verification. The B2 recovery drill
  proves a newer corrupt version/hide marker cannot resurrect a deleted fixture
  user. No live B2 account has been configured or verified.
- g3-native-plugins: pre-push suite and Rust Android/iOS app-info cross-checks.
  Swift/Kotlin execution on a device remains pending.
- g3-ui: full pre-push suite passes. No library components were changed.
- g3-stack: web/server/mobile checks, tests and strict lint pass. Its separate
  template consistency check reports an existing stale Cargo.toml.liquid.
- All four changed authentication modules pass server checks. Swift Circuits
  required a temporary local g3-native-plugins 0.4.5 override for its existing
  safe-area dependency; its committed lockfile still selects 0.4.4.
- Media Mancer: web/server/mobile checks and strict lint, real WebAssembly check,
  Android aarch64 cross-check with the installed NDK (including native metadata and
  external-link call sites), 99 web and 218 server tests, actual About-route markup
  rendering, and workflow/Docker
  syntax checks. Static actual-route previews inspected in all four appearances
  at 390×844 and 1440×900. Cold Docker fetching of pinned private/public dependencies
  passes with an ephemeral BuildKit secret; no secret mount or Git credential file
  remains in the test image. A later full Linux web/server image built successfully:
  `media-mancer:privacy-local`, image ID
  `sha256:ca8cba0065b42c4f467efc9fdce44ef99dbbbd2a61f6c718da137021c9571407`.
  Its actual server CLI passed encrypted backup/import/correction/deletion replay
  using synthetic data on a temporary internal Docker network, preserving another
  account. Seven Python helper checks and workflow syntax validation pass. This
  does not verify offsite storage, a mobile bundle or GitHub App installation.
  Interactive browser tests are added but not run:
  no application server is listening on 8080, and no dx serve was started.
  Fork formatting was also tested offline with an empty Cargo cache; it does not
  require private dependency access. Temporary preview/recovery servers are stopped.
- Greenside Partee: 342 ordinary server and 183 web tests, strict server/web/mobile and
  WebAssembly checks pass. Account download markup was inspected in both modes,
  light/dark, at phone and wide widths, including keyboard focus and loading.
  Android aarch64 compilation/lint passed after checking the real sharing branch.
  Invitation and deletion-confirmation fixtures were also inspected in both
  modes/themes at phone and wide widths, with keyboard/loading states. Actual DB
  fixtures cover projected downloads, cache/referrer headers, shared-scorecard
  removal, atomic joins/ownership claims and durable Apple cleanup. They include
  paused writer/state/removal races, last-seat/owner claim races and rollback.
  The expanded isolated database-editor WebSocket drill exercises actual primary
  removal, repeat removal, preserved shared bets/scores and Apple capture.
  Live hydrated invitation creation/save/share and Apple provider
  calls remain unverified. Its local HTTP tests required execution outside the
  network sandbox. Windows verification uses AWS-LC's bundled assembler objects.

Earlier foundation checks used workspace g3 path patches. The Media Mancer pilot
was additionally verified from outside that config scope with Rust 1.95.0, its
published dependencies and exact Git revisions. Its lockfile contains one native
plugin instance. Cold consumer GitHub Actions still requires installing/configuring
the scoped GitHub App. No organization credentials were created or changed.

## Remaining integration and rollout

1. Install the read-only GitHub App and configure approved consumer repositories
   using the private crate's CI templates. Store its key in GitHub secrets, not in
   chat or source. Verify cold private-dependency Docker and mobile builds.
2. Use the selected hostname catalog.g3tech.net. Supply real app/store links and
   reviewed content. No production policy text or app catalog has been published.
3. Media Mancer's account-bound legal status, exact-set acknowledgments, verified
   receipt writes and change notices are committed. Its opt-in terms middleware
   respects effective dates and keeps essential privacy controls available;
   no processing consent is inferred. Legal integration passed 100 web tests,
   221 server tests plus the isolated receipt test, and strict checks for all
   three builds. Public policy fallback previews were inspected at phone and wide
   widths; live interaction and device tests remain pending. Finish reviewed
   consent purposes. Deletion/correction recovery and their restore hook
   are implemented but require the production checks above. Required store
   notices are advisory UI, not backend authorization. Run the added interactive
   browser tests and actual mobile store-navigation checks. Then integrate other
   apps after their readiness checks.
4. Complete each app's data inventory, secure export, verified deletion/correction,
   consent withdrawal, processor handling, deadlines and idempotent recovery hooks.
   Shared contracts/UI do not implement those app-specific operations.
5. Release app-info with maintainer approval and test actual iOS/Android builds.
   Swift Circuits needs a compatible g3-ui migration before consuming brand UI.
6. B2 is selected and implemented; configure actual private buckets, their region,
   key custody, Object Lock/lifecycle/version expiry, snapshots,
   log/trace destinations and retention, access controls and monitoring. Verify
   actual provider behavior and perform a real offsite recovery exercise.
7. Review legal applicability, controller identity, processors/transfers, policy
   wording, consent purposes and retention exceptions before enabling promises.

Operational procedures are in g3-platform/docs/privacy-operations.md. The private
crate's docs/implementation-status.md describes library boundaries and deployment
prerequisites. This implementation does not establish regulatory compliance.
