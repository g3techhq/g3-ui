# g3-brand implementation handoff

Updated 2026-10-23. The shared foundation, B2 transport, Media Mancer catalog and
legal review, scoped account download, durable opt-in deletion/correction queues
and replay are implemented. Reviewed consent purposes, remaining app
integrations and production verification remain pending.

## Continuation evidence, 2026-10-22

This section supersedes older verification gaps below where explicitly noted.
Greenside's full locked Linux web/server build from `26e276f`, with Dockerfile
and build-context fixes, passes its encrypted synthetic old-backup drill. Local
image ID:
`sha256:5a2e543d9572fa644f2907781a1d7f9312ae0e864f42285688720d0c4ec4ffe2`.
The drill verifies actual age encryption/decryption and snapshot import, queued
intent reconciliation and old-guard migration, repeated independent deletion
replay, other-account/shared-score preservation, late-write refusal and packaged
non-root/read-only startup using database-scoped credentials. All owned fixture
containers/networks were removed. The consumer publishing gate tests the same
candidate it subsequently pushes; no image has been published by these checks.
Twelve Python helper tests, 11 Node checks, formatting and workflow lint pass.
Unrelated appearance/styling edits remain outside this milestone.
Greenside `63c3932` saves the gate, and platform `bc70315` records the evidence.

Tawny's queue (`0bd8b09`), replay (`503aef8`), report-file protection (`1aaa8c1`)
and packaged recovery gate (`271f2dd`) are also committed. Its encrypted local
image drill passes, including scoped read-only startup after packaging the
SurrealKit metadata setup file. Platform `ee0ea25` supplies the optional shared
host overlay, and `109f140` records the packaged check. The overlay is disabled
by default. Actual GitHub App setup, offsite B2/version-expiry verification,
reviewed processor workflows, correction/withdrawal coverage across the fleet,
and hydrated/native checks remain open. No production settings were changed.

Tawny `d38f65b` adds disabled-by-default backend effective-terms checks. Real
sessions/receipts verify exact account/set acceptance, changed-set and outage
holds, essential account access and retirement after deletion. The bounded
catalog test verifies expiry, failure cooldown and recovery without stale
authorization. Checks pass 210 ordinary server/87 web tests, strict checks for
all four host builds, actual WebAssembly, Node/extractor/helper suites and both
Compose configurations. Platform `3c6650c` documents the disabled rollout option.
The fresh full Linux image is
`sha256:7b1370d550871696c8f54c9e24425051dd9d242facacb7bd36bb8298b21e4b5a`.
Its encrypted old-backup/repeated replay drill passes. The packaged Dioxus routes
also refuse ordinary activity with required terms and an unavailable catalog,
while guest/session bootstrap, account lookup, private export and deletion work.
Non-root/read-only scoped startup and session revocation pass; owned fixtures
were removed. This is a local image ID, not a published registry digest.
Tawny `90f940f` saves the packaged endpoint gate; platform `0cb5fd3` records it.

Swift `f52e30d` records the source-verified account/device/provider inventory.
`2832713` keeps purchase references server-only and pins the reviewed public
native source. `2a76b6b` migrates to g3-ui 0.6/route transitions 0.5 and tracked
sessions: startup indexes recognized legacy sessions, holds unknown payloads,
and account deletion transactionally retires owned sessions and late writers.
An isolated scoped database drill verifies rollback, repeated migration,
other-account isolation and old-writer refusal. Checks pass 69 web workspace
and 80 ordinary server workspace tests, strict host/mobile/WebAssembly checks,
workflow lint and three fixture cleanup tests. Static shared-auth previews were
inspected in both modes/themes at phone/wide widths with keyboard focus.
Later Swift milestones below supersede the catalog/write-guard gaps. Existing
gameplay/lesson edits are preserved and excluded from these commits.

### Swift integration, 2026-10-23

- `d6066a9`: public About/catalog, installation-only validated cache, installed
  native update notices and scoped private-dependency CI/BuildKit access. No
  private source/target cache uploads; fork PRs format without private access.
  Saved/offline/busy previews were inspected in both modes and Circuit/Bauhaus
  themes at phone/wide widths, with keyboard focus and account access. Fixed
  shared-button foreground overrides and Circuit accent contrast; all ten game
  themes pass a contrast test. No Dioxus development server was started.
- `893fe8c`: private progress/identity/entitlement writes touch their live account
  row and conflict with primary deletion. Ownership changes and retired owners
  are refused; fulfillment checks even no-op paths. Startup holds missing guards
  and unexpected private orphans for review. An owned scoped SurrealDB 3.2.4 drill
  tests real sync/fulfillment helpers, late identity writes, final rollback,
  repeated removal, overlapping transactions and other-account preservation.
- `2f2266b`: reviewed-owner checks on sign-out, primary deletion, sync and native
  proof verification; keyed account scopes and request generations reject stale
  completions. Entitlement responses add an owner without changing legacy
  snapshot fields; new clients refuse foreign/unowned cached copies. Local
  account activation no longer merges another account's gameplay. Real in-memory
  store tests preserve independent profiles and still allow anonymous merges.
- `e59daf3`: explicit authenticated JSON account downloads, fixed safe projections,
  validated RON/portable gameplay and row/byte/time/concurrency limits. The real
  scoped drill excludes foreign-account and secret canaries, refuses malformed or
  oversized data and deleted/wrong actors, and returns no partial file. Browser
  Blob delivery treats contents as data and revokes temporary resources; native
  sharing is bounded separately and awaits actual device checks.
- `b4db892`: local account retirement after primary acknowledgment, two-slot
  removal, durable retry queue and permanent delayed-save refusal. Anonymous and
  other profiles survive. Failure injection covers queue/marker/slot/active/final
  checkpoint writes, restart and repeated retry. A failed active-marker write
  preserves anonymous gameplay. Pending retry controls were inspected in the
  themed account surface at phone/wide widths with keyboard focus. Unavailable
  storage and responses lost before local recording remain explicit limits.
- `8879eca`: web checkout requires a reviewed live session actor and returns
  that owner. Client account/generation checks refuse stale or ownerless redirects
  and only open the reviewed hosted HTTPS domain. The real scoped authorization
  drill makes no Stripe call. Existing remote checkout cancellation stays separate.
- `8db4bc0`: primary deletion atomically captures bounded Apple grant evidence in
  a private leased retry queue. Failure keeps credentials for retry/assistance;
  success scrubs raw subject/client/grant. Tests cover final rollback, fixed
  errors, backoff, competing/expired leases, restart, missing/oversized evidence,
  other-account preservation and schema holds. The actual grant-persistence
  helper also fixes a protected SurrealDB parameter name used by Apple sign-in.
- `d33a452`: original-account purchase custody survives entitlement replacement
  and primary deletion. Startup backfills current links and holds conflicting
  legacy evidence; recorded native/Stripe purchases cannot transfer automatically.
  The scoped drill covers six original claims, retirement, reassignment refusal,
  preserved unrelated purchases, repeated migration and export exclusion.
- Native bridge `983f4b2`: opaque account UUID launch options and attributed
  callbacks for StoreKit/Play, keeping legacy APIs compatible. Proofs and account
  metadata are redacted from Debug output. Pre-push, all 49 feature tests, actual
  Android target and packaged Kotlin testbed pass locally. Source was pushed to
  main with user approval after the user removed its branch restriction. GitHub
  quality plus facade and packaged iOS Simulator/Android checks pass in run
  `40004306324`. Licensed store/device flows remain unverified.
  No crate or app release was requested or performed.
- `92ccdfd`: provider-bound native purchase attribution. A reviewed live account
  gets a stable opaque UUID before launch; verified provider account metadata
  must match the live actor inside the grant transaction. Apple original IDs and
  Google purchase tokens retain legacy ownership aliases atomically. Unknown
  legacy, foreign and retired purchases require support review. A transaction
  proof cannot authorize finishing a different store transaction. Required
  account-correlation schema is checked at startup. The scoped drill covers
  wrong-account first callbacks, stable tokens, immutable correlation, alias
  migration/conflicts, repeat restores and post-erasure refusal. Native rollout
  needs an older-client assistance/refund plan before deployment.
- Native `efd9b4a` adds committed encrypted checkpoints for the next pending-proof
  milestone; `2f57a30` records platform evidence. The source is published. Its
  pre-push and packaged Android checks pass locally; CI run `40006888391` passes
  quality and both packaged iOS Simulator/Android builds.
- `0849364`: bounded native encrypted receipt checkpoints before verification,
  with original-owner metadata committed before finish. Asynchronous finish does
  not clear evidence: store acknowledgment plus fresh current-account verification
  must precede committed removal. Unknown schema and full/unreadable checkpoints
  hold purchases; failed writes keep the inbox for retry. Seven stateful tests
  cover restart, failures, bounds, acknowledgment and suspended A-to-B-to-A results.
  Foreground retries are implemented; actual device/key/disk behavior, deadlines,
  deleted-account assistance/refunds and retention remain review boundaries.
- `3e67fd2`: one root-owned live account check, memory-only private reads,
  synchronous auth/request boundaries and best-effort legacy cache clearing with
  a bounded wait. Same-origin notifications contain only a random marker and
  cause an independent server recheck. Dioxus tests prove cleanup survives Account
  panel removal, old checks are cancelled and stalled cache cannot block account
  controls. Six browser lifecycle tests cover hidden/disposed pages and blocked
  storage. Physical cache erasure and actual hydrated cookie races remain open.
- `a2f60b1`: root-owned saved deletion intent, per-device private receipts and
  leased server primary checkpoints. Intent is persisted before HTTP and the
  intended device account is retired even when cloud submission is delayed;
  another active account is preserved. Native receipt writes use encrypted
  durable storage, while browser receipts use separate origin-readable keys.
  The user approved signed-out status reads requiring both a random receipt ID
  and separate 256-bit secret; only minimal progress/review/provider flags are
  returned, and submission still requires the reviewed authenticated actor.
  Required recording mode demands the exact independent ledger digest before
  the atomic primary checkpoint/removal. It fences pending writes, refuses the
  old direct removal path and cannot be silently downgraded. Limits, unknown
  evidence, failures and expired leases retain intent for retry/assistance.
  Apple grant custody and original purchase retirement remain separate from
  claims that providers or old backups have been erased. Required mode stays off.
  Platform `d4e9127` records the migration and disabled example settings; all
  48 platform tests pass. No actual host environment was changed.
- `83fed16`: checkout return notices keep payment/unlock status unconfirmed until
  server verification; cancellation URLs no longer claim that no charge occurred.
- `7713ec8`: quarantined deletion replay exits before normal server/provider startup,
  verifies exact source schema against a temporary definition-only reference,
  indexes supported legacy sessions and reconciles canonical ledger identity.
  It reuses leased atomic primary erasure, original purchase retirement and Apple
  grant capture. Missing identity, unexpected definitions/session bytes and
  conflicting request/revision/digest evidence preserve quarantine. Pending Apple
  work fails final verification without a provider call. Successful primary replay
  cannot reconstruct newer purchase history/grants missing from an older backup;
  independent custody review remains necessary before manual cutover.
  Platform `51b31e1` adds Swift's digest-pinned isolated wrapper and example hook;
  all 53 platform tests pass. The real synthetic old logical export/import drill
  passes repeated removal, other-account preservation and late-write refusal.
  The publishing workflow now tests a candidate before publishing that same image.
- `9fa61cc`: explicitly marks successful/private/signed-out API responses
  `private, no-store`, beyond the shared cache guard's revalidation default.
  Actual local HTTP tests cover success, auth refusal, fault redaction and an
  accidental public cache header. The full locked private Linux web/server build
  passes with an ephemeral BuildKit secret. Final local image:
  `sha256:a238a746e5923f9bfb40f8478343ee216e00931bcf1f141a6ea1642ab26fce54`.
  It passes actual encrypted old-backup import, repeated primary replay,
  unrelated-account/session preservation and late-write refusal. Both restored
  required-mode and empty-database direct-mode startup pass with scoped EDITOR
  credentials, UID/GID 10001 and a read-only root filesystem. Packaged API checks
  prove minimal signed-out status, wrong-secret refusal, authenticated-only
  submission and no-store responses. All owned fixtures are removed. The image
  includes preserved uncommitted gameplay edits, excluded from privacy commits.
  It is not a reviewed registry digest. No GitHub workflow, image publication or
  deployment was triggered. Platform `ee8ab83` rejects duplicate/ambiguous JSON at
  ledger storage and authenticated recovery; all 55 platform tests pass.
- `71afede`: the packaged browser exercise exposed that retiring a local save
  could hide a still-active server session. Account identity now follows the live
  session; sign-out and export remain available while sync is disabled without a
  local account save. Persisted retirement is checked before profile activation.
  Account changes replace credential form state through a real keyed fragment,
  and authentication/sign-out clear fields explicitly. Owner/generation checks
  still reject delayed account actions. Updated packaged verification is pending.
- Latest checks: 97 web-workspace tests, 82 ordinary server tests, four save-library
  tests, strict
  server/web/WebAssembly and actual aarch64 Android compilation with the installed
  NDK. Twelve Python helpers, the JavaScript foreground lifecycle, formatting and
  workflow lint pass. Nine Node tests cover foreground/session/download lifecycles
  and private receipt storage. The scoped database drill covers receipt limits,
  wrong-proof/foreign refusal, ledger failure, lease loss, late rollback, missing
  evidence review, restart and verification after primary removal. Rendered
  receipt controls pass both modes/themes and enabled/busy states at 390px/1280px,
  with keyboard access. A disposable hydrated browser exercise checks receipt
  persistence after navigation/reload, deletion completion while a different
  account stays signed in and signed-out status lookup. Updated pending-session
  controls and remaining cookie/storage races still need browser/device checks. Two old Android-only unnecessary returns were corrected.
  Whole-repository spelling still flags old short direction-variable identifiers
  in the generator; changed integration files pass. Tests include existing
  uncommitted gameplay edits; those edits remain excluded from these commits.
  The g3-ui plan update passes full `just pre-push`: 180/181 nextest tests,
  93 documentation tests (two ignored), 19 playground tests and 16 script tests,
  plus all build/lint/format/spelling checks. The run uses an owned temporary
  directory and two documentation-test threads; its log is under ignored
  `g3-ui/target/privacy-checks/pre-push.log`.
- Platform `d2a0570` derives Swift's backend origin from its shared Caddy hostname
  and adds the rollout guide; 48 platform tests pass. `637ec48` records mandatory
  live-owner schema; `0a6e1ba6c37afb75de88a9c3dcdda99eb6fdc629` documents reviewed-owner client migration. Old
  clients omitting the actor are refused, so retain the current web account page
  during native rollout. Catalog/channel configuration belongs to image builds.
- Platform `8fa959e` records the download/device-cleanup rollout and custody
  boundaries. Shared Compose origin and compiled catalog settings remain separate.
- The user confirmed native restoration policy: preserve the original app-account
  ownership and require support review for transfers. Implementations must not
  infer a new owner from whichever account is signed in at callback delivery.
- Remaining: actual offsite/custody recovery, native device/deadline workflows,
  persisted-store failure and hydrated cookie/browser/device tests, reviewed
  registry digest and actual GitHub App access. No provider calls, production settings, releases or
  app image publication were performed by these continuation milestones.

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
| Tawny | Public catalog/native notices, legal review and optional backend terms checks; scoped download; write/session guards; safe diagnostics; scoped DB access; durable opt-in deletion and quarantined replay; optional shared-host deployment |
| Greenside Partee | Public catalog/native notices and legal receipts; optional backend terms checks; scoped export; tracked sessions and transactional write guards; durable opt-in deletion and quarantined replay; packaged encrypted recovery gate |
| Swift Circuits | Public catalog/native notices and scoped private builds; reviewed-owner actions and live session/write guards; bounded export, durable device deletion receipts, retained provider/purchase custody and quarantined primary replay; legal receipts, correction/withdrawal and real device/offsite recovery remain pending |

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
  scores/seats/bets and durable Apple grant capture. The later `cbdea58` milestone
  adds deletion-only primary replay; live ledger/processor handling remains work.

- Greenside `10f18a3`: public About/catalog and installed-metadata update notices
  under the host theme, validated public installation cache, foreground refresh
  and policy-specific snoozes. Help/legal routes remain public. Shared legal
  receipts/change notices followed in `1779b00`.
  Private Git access is wired into Docker and mobile/CI with scoped GitHub App
  tokens; source-bearing CI cache uploads are removed and forks receive no access.
  Eight Python protocol/cleanup tests, four workflow checks and Dockerfile checks
  pass. A real BuildKit-secret probe fetched the exact private source pin, refused
  a missing secret, and left no credential in image metadata/history. Its image
  was removed. This is not a full cold consumer image/GitHub App installation test.

- Greenside `1779b00`: account-bound shared legal status, exact-set immutable
  receipts, change notices and published policy pages with readable host layouts.
  Private receipts are included only in bounded owner-projected exports and
  removed with the primary account. Retired writers cannot recreate them.
  The DB-editor drill verifies idempotent receipts, current-set changes and late
  write rejection. No processing-consent purposes or backend terms enforcement
  are enabled. Static legal fixtures were inspected in both modes/themes at
  phone/wide sizes; hydrated and actual device interactions remain unverified.

- Media Mancer `8832253` and Greenside `bfe13b7`: downloads, profile/setup drafts,
  deletion confirmations and legal actions are bound to the account displayed
  when opened. The server still derives the actor from the session and rejects
  missing/mismatched bindings before data, ledger or provider work. Older clients
  must refresh/update. Late responses cannot alter another account; legal
  actions also reject older generations. Private legal/account responses are
  no-store. Media Mancer passes 237 ordinary server/105 web tests; Greenside
  passes 355 ordinary server/192 web tests, with strict checks for all builds.
  Live account-switch interaction is still unverified.

- Greenside `cbdea58` and g3-platform `f9ab859`: deletion-only primary replay
  inside restore quarantine. The app command bundles release schemas, indexes
  legacy sessions, keeps private preparation evidence and checks actual erasure,
  ownership transfer, preserved completed scorecards and irreversible retirement.
  Unknown fields, missing identity evidence, restored processor fields and any
  pending Apple job keep quarantine closed. The wrapper requires a reviewed
  digest-pinned app image and one verified internal scratch network; it forwards
  no provider/ledger credentials and never enables traffic. An actual old logical
  export/import drill proves private profile/session data is removed again while
  another account and shared scores survive. Interruption, retry, conflicting
  events and newly appearing provider evidence are tested. Greenside now passes
  359 ordinary server/192 web tests, strict server/web/mobile checks, its isolated
  snapshot drill, eight Python consumer checks and 11 Node checks. Platform tests
  total 37. The live Greenside deletion endpoint still lacks independent ledger
  recording; processor resolution, corrections/withdrawals and an encrypted
  offsite/final-image drill remain required work. Retained signup date, handicap
  and stable seat ID remain linked personal data and need retention review.

- Greenside `493f865`: optional effective-terms enforcement is implemented and
  disabled by default. Protected account activity, including guest accounts and
  authenticated invite joins, checks exact current receipts after the effective
  date. Sign-in, legal review, privacy controls and subscription management stay
  available. Real local sessions/DB receipts prove that another account or an
  older set cannot authorize activity, and catalog failure closes protected work.
  No processing consent or production policy/enforcement setting is inferred.

- Greenside `ae80843` and `dc387af`: primary deletion clears appearance choices
  and replaces original signup dates with a fixed epoch placeholder. The v2
  retired-user guard permits only that erasure value; schema rollout installs it
  before removing v1, and startup rejects partial migration. Bounded startup
  cleanup also handles older deleted rows without changing active profiles or
  shared scorecard handicaps/IDs. The old-snapshot drill includes guard migration.
  Current checks pass 365 ordinary server/192 web tests, strict host checks for
  all three builds, and the isolated database-editor/snapshot drill. No live
  deployment or final encrypted Linux/offsite image drill was performed.

- Greenside `3f8f88a`: optional live recovery-first deletion intent and leased
  processing. One stable request/revision is saved from the authenticated actor;
  exact independent-ledger acknowledgment precedes billing cancellation and
  primary removal. Atomic primary/checkpoint writes, retry leases, cancellation
  checkpoints and Apple/session verification prevent false progress after an
  outage or interruption. Legacy purge refuses queued intent; required mode
  reports saved/pending rather than overall completion. Default disabled mode
  preserves the earlier null success response. Unknown payload fields and
  unresolved Google/Stripe/App Store evidence hold the request for review.
  The final private state is only `primary_verified`, not expiry of every copy.
  Owner exports project minimal request state and exclude processor/future fields.
  Quarantined replay reconciles the exact queued intent in an old snapshot.
  Checks pass 377 ordinary server/193 web tests, strict checks for all host builds,
  eight Python/11 Node checks and the isolated old-schema/queued-intent snapshot
  drill. Actual scoped HTTP ledger transport is covered with synthetic receipts;
  no real processor, B2 object or deployment setting was changed. Native clients
  must understand queued outcomes before enabling required mode. Public status
  proofs, deadline monitoring, broader processors, correction/withdrawal and the
  encrypted final-image/offsite drill remain work. Pending intent does not freeze
  every write; existing guards retire writes atomically with primary removal.

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
- Platform: 37 Python tests pass, combined Compose configuration and Caddy
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
- Greenside Partee: 377 ordinary server and 193 web tests, strict server/web/mobile and
  WebAssembly checks pass. Account download markup was inspected in both modes,
  light/dark, at phone and wide widths, including keyboard focus and loading.
  Android aarch64 compilation/lint passed after checking the real sharing branch.
  Invitation and deletion-confirmation fixtures were also inspected in both
  modes/themes at phone and wide widths, with keyboard/loading states. Actual DB
  fixtures cover projected downloads, cache/referrer headers, shared-scorecard
  removal, atomic joins/ownership claims and durable Apple cleanup. They include
  paused writer/state/removal races, last-seat/owner claim races and rollback.
  The expanded isolated database-editor WebSocket drill exercises actual primary
  removal, repeat removal, preserved shared bets/scores and Apple capture. It now
  also exports/imports an old logical snapshot, migrates an older retired-user
  guard and replays the independently held deletion event twice. Full encrypted image/offsite recovery remains unverified.
  Live hydrated invitation creation/save/share and Apple provider
  calls remain unverified. Its local HTTP tests required execution outside the
  network sandbox. Windows verification uses AWS-LC's bundled assembler objects.

Earlier foundation checks used workspace g3 path patches. The Media Mancer pilot
was additionally verified from outside that config scope with Rust 1.95.0, its
published dependencies and exact Git revisions. Its lockfile contains one native
plugin instance. Cold consumer GitHub Actions still requires installing/configuring
the scoped GitHub App. No organization credentials were created or changed.

## Tawny rollout (2026-10-22)

- `b7bc05b`: public catalog/About, installation cache, installed native update
  notices, help during backend failure and scoped private build access. Static
  phone/wide previews cover inherited modes/themes and offline states.
- `a98e319`: reviewed-owner registration/deletion, isolated drafts and stale
  response guards. `f3202b1` adds exact-set private legal receipts; `c322a42`
  adds the bounded actor-projected account download.
- `e60b14b`: transactional owner/relation write fences. `4b44906` pins the
  reviewed tracked-session adapter, indexes known legacy payloads and retires
  stored sessions with primary deletion. Unknown legacy formats hold startup.
- `4c0a84f`: sanitized server/extractor logging, dependency/panic suppression and
  disabled retained unreviewed database/provider output in standalone production.
- `32ee730`: scoped database credentials without root fallback and an actual
  disposable database-editor WebSocket drill, including empty/legacy startup,
  private erasure, late-write refusal and foreign-data isolation.
- `0bd8b09`: optional durable recovery-first deletion. Stable intent/revision,
  exact ledger acknowledgment, leases/retries and atomic primary checkpoints
  keep failures pending. Unknown fields and changing identity hold removal.
  Current clients distinguish saved requests from immediate deletion. Export
  reveals only the caller's request ID/state/timestamps. Required mode stays off.
- `503aef8`: deletion-only quarantined replay/verification, using the shared
  primary erase transaction and source-defined guards. Independent exact events
  reconcile saved checkpoints without replacing conflicts. Missing identity,
  unrecognized sessions, unknown schema/evidence or unattributable private
  orphans keep quarantine. A real pre-request logical export/import passes
  repeated replay while preserving another account, shared facts and demo rows.
- Platform `85a2709` adds Tawny's restricted replay wrapper/example hook;
  `ee0ea25` adds optional scoped deployment, unexposed extraction sidecars,
  Caddy routing, the remote request-event allowlist and a separate app-data
  inventory. Unreviewed shared DB/provider output is not retained. The overlay
  remains opt-in; actual host credentials/images/domains are not configured.
- `1aaa8c1`: disables RustyPipe's independent raw request/response file reporter.
  Extractor cache/report/secret custody is explicitly inventoried; old files are
  not silently erased or assumed anonymous.
- `271f2dd`: full private Linux build, encrypted synthetic old-backup replay and
  packaged scoped-server gate before publishing the same tested candidate.
  The startup drill found a missing SurrealKit metadata setup file; the runtime
  now includes it. The local repaired image
  `sha256:3344bb9ffeba436a9ace99d3546ee961a85ac213d6181d30aaf50d00266809aa`
  preserves the full build's server hash/config and passes non-root/read-only
  startup, encrypted import, repeat replay, late-write/conflict refusal and other
  account preservation. This local ID is not a published registry digest.
  Platform `109f140` records the evidence. Fixtures are removed; no image pushed.
- Latest checks: 205 ordinary server/87 web tests, strict all-four-host checks,
  nine Node, ten extractor Python, twelve CI helper tests and the separate
  wire/snapshot drill pass.
  Actual WASM/Android checks passed at the queue milestone. Saved/pending notices
  were inspected statically in all modes/themes at phone/wide sizes. Platform
  tests pass 47 cases; combined Caddy routing adapts and Alloy starts its graph
  against an empty synthetic journal without network access. Fixtures are removed.
- Remaining: actual encrypted offsite/B2 lifecycle recovery, reviewed published
  image and GitHub App installation/build verification, extractor/provider
  runtime compatibility, backend legal enforcement, public status proofs,
  reviewed consent purposes, deadline/processor procedures, correction/withdrawal
  replay and actual hydrated/native device flows. No policy publication, provider
  erasure, B2/DO configuration, release or production deployment was performed.

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
   browser tests and actual mobile store-navigation checks. Tawny now has the catalog/update and legal-receipt host integrations described
   above. Continue remaining apps after their readiness checks. Greenside now has the catalog/update host,
   shared legal revisions/receipts and optional live independent recovery
   recording. Finish processor handling, status/deadline workflows and the
   remaining correction/withdrawal recovery hooks. Its first
   deletion-only primary replay is implemented with conservative quarantine holds.
   Optional effective-terms enforcement is implemented and remains disabled.
4. Complete each app's data inventory, secure export, verified deletion/correction,
   consent withdrawal, processor handling, deadlines and idempotent recovery hooks.
   Shared contracts/UI do not implement those app-specific operations.
5. Release app-info with maintainer approval and test actual iOS/Android builds.
   Swift Circuits now pins reviewed app-info and has migrated its shared UI;
   public brand integration and actual Android compilation pass; actual device
   interaction and maintainer-approved releases remain pending.
6. B2 is selected and implemented; configure actual private buckets, their region,
   key custody, Object Lock/lifecycle/version expiry, snapshots,
   log/trace destinations and retention, access controls and monitoring. Verify
   actual provider behavior and perform a real offsite recovery exercise.
7. Review legal applicability, controller identity, processors/transfers, policy
   wording, consent purposes and retention exceptions before enabling promises.

Operational procedures are in g3-platform/docs/privacy-operations.md. The private
crate's docs/implementation-status.md describes library boundaries and deployment
prerequisites. This implementation does not establish regulatory compliance.
