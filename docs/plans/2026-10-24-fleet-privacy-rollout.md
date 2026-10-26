# Fleet privacy implementation

Scope confirmed by the maintainer on 2026-10-24: Swift Circuits, Ad Sentinel
(repository `ad-sentinel`), Media Mancer, Greenside Partee and Donation Engine.
Tawny is excluded from new work; preserve its existing implementation.
No new optional processing purposes are authorized. Do not invent consent grants
or withdrawal switches for analytics/marketing that these apps do not perform.
Unsupported purpose-specific grants stay refused. Implement only applicable
permission withdrawal and product controls already supported by the app.

## Acceptance boundaries

- Review the real personal data, identity, shared content and provider custody
  per app. Correction must not rewrite login credentials or confirmed financial
  history without the appropriate verified workflow.
- Bind private actions and delayed responses to the reviewed account/session.
- Include private records in bounded exports and verified primary erasure.
- Preserve deletion/correction ordering and independently recorded recovery when
  enabled. Restores remain quarantined when evidence is incomplete.
- Keep essential help, sign-out, export, correction, deletion and status available
  through legal-update gates. Acknowledgment/terms are not processing consent.
- Test failures, retries, cross-account isolation, late writers and actual backup
  replay. Verify browser/native builds and document remaining device/provider
  evidence honestly. Keep production changes and publication separate.
- Preserve unrelated working-tree changes and use main for milestone commits.
  Do not start Dioxus development servers. Do not change actual host settings.

## Initial source audit

| App | Existing foundation | Work to finish |
| --- | --- | --- |
| Swift Circuits | Public brand/notices, private legal receipts, bounded export, device deletion receipts, guarded primary deletion, purchase/Apple custody and quarantined replay | Profile correction and ordered correction replay; remaining platform verification |
| Media Mancer | Brand/legal, projected export, deletion proof/status, durable display-name/handle correction and replay already exist | Audit current account/session, recovery, provider and legal parity; close actual gaps without duplicating existing features |
| Greenside Partee | Brand/legal, projected export, guarded primary/session removal, durable deletion and conservative shared-scorecard replay | Durable display-name/handicap correction and replay; audit status/device/provider parity |
| Donation Engine | Basic template account deletion and financial app tables | Most shared brand/privacy integration; protect provider-confirmed donation history and project-specific records |
| Ad Sentinel | No accounts/cloud server; local TV profiles, encrypted pairing credentials, camera sessions, remote host and unpair controls | Applicable local export/correction/removal and permission controls; shared public brand/help/notices; no account backend to invent |

## Preserved local edits

Swift Circuits has existing gameplay/content edits, including four account-page
copy edits and untracked lesson boards. Greenside has appearance/styling edits.
Donation Engine has an existing untracked Cargo.lock. Ad Sentinel and Media Mancer
were clean at audit (Ad Sentinel Git inspection needs unsandboxed execution).
Further Greenside game-feed/scorecard, private-write and documentation edits
were present on 2026-10-26. They were preserved; the session/export follow-up
below made no new Greenside source changes.

No completion is claimed by this initial inventory. Record implementation and
verification milestones below as each app is handled.

## Native storage checkpoint

Published source commit `62c6d0d5bee5aa4adc981c1402b666494a977471` in
g3-native-plugins adds `remove_durable`. Android waits for encrypted preferences
commit, including a reported failure; iOS delegates to the existing synchronous
Keychain removal. Web/macOS refuse the committed encrypted operation.
The ordinary asynchronous API remains compatible.

The full native library pre-push checks pass. Its packaged Android test bed
passed write/read/list/durable removal on a disposable API 35 x86_64 emulator,
then passed again after force-stop/relaunch while checking the prior probe was
still absent. Apple and Android packaged CI both passed in run `40133239257`;
no Apple device run is claimed.
This is source publication only, with no crate release or app deployment.

## Ad Sentinel local milestone

Local main commit `ab0200d` adds bounded allowlisted local export, confirmed
restartable encrypted removal, pending-write fences, remote generation guards,
and independent screen-off restoration evidence. Existing TV edits and permission
controls remain the applicable correction/withdrawal flows. Shared public help,
read-only legal documents, installed metadata and update notices now appear in
Settings; there is no invented account backend or authenticated receipt service.

The frozen portable source passes 86 core/device/protocol tests and 12 app tests,
plus strict desktop clippy and actual Android packaging. A fresh disposable
emulator verified corrected-name export, cancellation, confirmed removal and
absence after forced restart. Material/light phone and iOS/dark wide UI were
inspected; unavailable catalog access did not block local controls. Live catalog
publication, required-prompt interaction, Apple runtime and real TV restoration
remain distinct verification limits. Source and custody details are recorded in
Ad Sentinel's `docs/privacy-data.md`.

Concurrent TV-support edits appeared during this work. They were preserved;
verification used a frozen privacy source copy, and only that reviewed snapshot
was committed. No Ad Sentinel app package was published or deployed.

## Media Mancer export milestone

Local main commit `76ae331` closes two observed parity gaps: current-account
responses now receive `private, no-store`, and each allowlisted export projection
is bounded inside the database before transport, including oversized profiles.
No optional processing purpose was added; the existing correction/replay and
deletion workers remain in place.

All three feature checks and strict clippy pass, along with 237 ordinary server
tests, 105 web tests, seven disposable-database integration tests and web lint/
format checks. The real database fixture covers oversized rows, isolation,
pending corrections, legal receipts, deletion and recovery. This records this
backend milestone; remaining session/device parity review stays in scope.

Local main commit `e5e47a2` adds authentication generations to profile/setup
responses, preferences, account and ratings downloads, deletion confirmations
and receipt checks. It rejects old A responses even after A-to-B-to-A sign-in,
clears mounted drafts/confirmations, and prevents a late initial account answer
from replacing a newer session. Legal review refetches per authentication
generation and requires a fresh completed response before acknowledgment.
Sixteen session-neutral response paths strip cookies on success and refusal,
while authentication keeps its normal cookie behavior.

The follow-up passes 239 ordinary server tests, 106 web tests, strict clippy on
all three builds and Rust/web formatting. A real packaged browser held A's
successful appearance response, switched to guest B, then delivered the old
response. B's distinct profile and Auto theme survived delivery and full reload.
Phone/wide screenshots and detailed limits are in Media Mancer's
`docs/verification/2026-10-26/`; all owned fixtures were removed and no console
errors/warnings were captured. The seven unchanged database scenarios were not
repeated in this follow-up. Native device runtime, published-catalog legal
prompts, competing login requests and unrelated API-cookie parity remain
separate checks; no deployment or provider operation is claimed.

## Swift profile correction milestone

Local main commit `8b6466e` adds normalized display-name correction, a private
restartable outbox, exact acknowledgment reuse, deletion/correction ordering,
pending-value export/erasure and quarantined correction replay. The exact fields
and existing internal ledger were explicitly approved by the maintainer; see
`2026-10-24-profile-correction-review.md`. No optional processing was added.

Verification passes 99 web-workspace tests, 88 ordinary server tests, both scoped
database/reimport drills, strict web/server clippy and actual aarch64 Android
compilation. The local image
`sha256:41bb48b9a1f40b52cadc4fb05baf0fe0efcc15c62aa401a00237ed8115a45324`
passes encrypted old-backup correction/deletion replay, fresh/restored startup
with scoped credentials as UID 10001/read-only, and actual HTTP profile argument,
rejected-owner and no-store checks. Browser verification at 390x844/1440x900 covers
pending state, sign-out, B isolated while A completes, A's corrected value on
return, and keyboard focus. The original screenshots were in Swift's build
directory, which is no longer present; the observed results remain recorded here.

The shared platform accepts only this reviewed correction shape; all 56 platform
tests pass. Recovery stays disabled by default; no app image was published and no
actual host setting was changed. Swift's existing gameplay/content/copy edits
remain uncommitted and separate from this milestone.


## Greenside profile correction checkpoint

Local main commit `c08195a` adds the approved name/handicap correction queue,
live private pending status, authentication generations, pending-value export
and erasure, guarded profile writes and quarantined replay. Its initial tests
preserved stored scorecard records. A subsequent calculated-results audit found
that Lone Wolf still used the current profile handicap; the scoring follow-up
below addresses that separate result-changing defect. Recovery stays opt-in; a
stored required policy refuses silent rollback to disabled, and enabling it
refuses unfinished direct jobs.

The milestone passes 384 ordinary server tests, 194 web tests, the separate real
WebSocket/database-editor drill, all three feature checks and strict clippy,
actual aarch64 Android compilation, 11 Node checks and 12 CI/helper tests. The
shared platform commit `890d9e9` admits only normalized names and integer
handicaps under the existing quarantine boundary; all 58 platform tests pass.

Commit `c5fc7d5` also requires an exact acknowledgment and verified primary
removal before a later deletion supersedes an old correction. Its 43 privacy
tests and strict server lint pass. The local image
`sha256:10d8fcb3f20855dd68dbd51824d68569c37bb9da204d9ed3a6c7b1e78bdae95d`
passed encrypted old-backup replay and scoped read-only startup. Browser checks
covered normalization, pending state, sign-out, separate A/B corrections,
refresh/reload and keyboard use at phone/wide sizes. Durable report/screenshots
are committed under Greenside's `docs/verification/2026-10-25/`.

Local main commit `1d262ce` records handicaps per started round and uses them in all
player projections, while displaying the corrected current name. Legacy rounds
capture the value currently used before the next profile handicap change;
values already overwritten before this feature cannot be reconstructed. A
regression compares actual Lone Wolf points/winnings before and after correction
and proves the unfrozen value would change the result. The updated synthetic
server suite passes 385 tests, alongside 194 web tests and strict checks on all
three feature builds. The separate database-editor drill also verifies live
player notifications. The refreshed local image
`sha256:0c3df86f85e4690cfc523c29fa0e352191047e7a68834a2b20ecffaf9952f692`
passes encrypted replay from a backup predating snapshots and scoped read-only
startup. Actual form correction changed a synthetic profile from handicap 20 to
17 while its completed round kept 20. Phone/wide checks cover slow/failed review,
retry, keyboard focus, sign-out during review and a second account staying at its
own Guest/20 values after delayed responses finish. No browser errors/warnings
were captured. The durable `scoring-preservation.json` and screenshots record
these results; all owned fixtures were removed. Profile fields wait for a fresh
successful review before becoming editable. No current native device runtime or
new Android target build is claimed for this follow-up.
Existing Greenside styling and appearance edits remain outside these commits
and frozen build contexts.

Donation Engine's bounded, account-bound JSON/CSV exports are committed locally
as `e066300`.
Its 55 server tests, 34 web tests and a disposable database contract pass,
including oversized rows, other-account isolation and credential canaries. All
three feature builds pass strict clippy; formatting and Node/fixture-helper
checks also pass. The server-rendered profile now initializes once and cannot
restore a signed-out or switched account. A real browser JSON download contains
only the synthetic owner's record and excludes credential/other-account
canaries, with a private/no-store response. Packaged CSV checks also pass. The
script now returns an explicit completion value, avoiding the false failure
previously shown after a successful download. A controlled delayed response
exposed a second issue: session cookies on old export replies could overwrite a
newer login. Both download endpoints now strip those cookies, including on
refusals; authentication retains its normal cookie changes. The final package
delivered a held successful A export after guest B signed in: no new export file
appeared and B's own activity refresh still succeeded. Phone/wide screenshots and
a durable report are under Donation's `docs/verification/2026-10-26/`; all owned
fixtures were removed. Its pre-existing untracked Cargo.lock remains unchanged.
Durable correction/deletion/legal controls remain outstanding;
the existing Every.org flow and financial records are unchanged.

Donation Engine local main commit `de61ed6` also binds deletion confirmations to
the reviewed account and authentication generation, rejects anonymous/mismatched
server calls before provider work, and checks primary account/owned-row removal
inside its transaction. A synthetic deletion event recreating an owned child
forces a full rollback; normal erasure preserves the other account's records.
The 55 server/34 web suites and strict checks on all three builds pass. The
updated packaged phone dialog was opened and cancelled, with database read-back
confirming that the account remained. Actual erasure/rollback was exercised in
the disposable database contract; no browser-confirmed purge or native device
run is claimed. The durable report and screenshot are in the same verification
folder. The original untracked lock remains unchanged.

Donation local main commit `6db15cc` adds a shared transactional write/retirement
fence for user and all seven owned tables. Late writes and reused account IDs
refuse after primary deletion; owned rows cannot move between accounts. The
retired fence clears its account reference and cannot be reset or deleted by
ordinary application writes. Startup verifies its function, fields and all nine
events before serving. Database-scoped runtime authentication is now supported;
no deployment identity or actual host setting was changed.

The 55 ordinary server/34 web suites, strict checks on all three builds and both
disposable database contracts pass. Scoped editor tests cover every owned table,
missing schema, identity reuse and five explicit old transaction snapshots held
open across another connection's successful deletion. The same writer succeeds
without deletion. The packaged debug server starts with pre-applied schemas and
scoped editor credentials; actual HTTP readiness/current-user reads and
signed-out export refusal pass. Interrupted temporary artifact copies were
restored from intact matching build output before the final startup check.
The detailed `write-retirement-verification.json` records limits; owned fixtures
were removed and the original untracked lock remains unchanged. No current
native runtime or release-container run is claimed for this follow-up.

Donation still needs its durable correction/deletion outboxes, shared legal
controls, session-store retirement, provider cleanup evidence, replay and
platform wrapper. Wider response-cookie and device parity review remains in
scope for the fleet; these bounded milestones do not complete that audit.
No new optional processing, actual deployment or real provider operation is
included. Tawny remains excluded from new implementation work.


Donation local main commit `8008474` adds an opt-in deletion outbox using the
existing g3-brand recovery event and transport. A stable event must receive an
exact digest acknowledgment before primary erasure. Leased workers checkpoint
primary removal, tracked session retirement and Apple cleanup; failures remain
pending. Pending deletion freezes personal writes and invalidates older
sessions. Completed requests clear account references and temporary grants.
A separate OS-random 256-bit receipt secret permits minimal signed-out status;
the browser holds it in memory until explicitly saved. Request metadata exports
exclude credentials, proof hashes and internal worker fields. Current-user,
delete and status responses join exports in refusing session-cookie changes.

The 59 ordinary server/37 web suites, strict server/web/mobile host checks,
formatting, three Node and three cleanup-helper tests pass. All three isolated
database contracts pass with scoped Editor runtime credentials, including
outages, wrong acknowledgments, stale workers, interrupted primary cleanup,
provider retry, missing-grant review and immutable recovery identity. The
packaged debug app passes scoped startup and signed-out receipt checks at phone
and wide sizes, keyboard submission, wrong-proof refusal, an exact saved-file
read-back, full-reload clearing and fresh guest navigation. No browser warnings
were captured. A synthetic status row supplies the browser preview; actual
purge/retry/session/provider behavior is covered by the database/HTTP fixture.
No browser-confirmed deletion, automatic receipt handoff, required-mode packaged
worker startup, current native runtime or release-container run is claimed.
Durable reports/screenshots are under Donation's verification folder. All owned
fixtures were removed, and the pre-existing untracked lock remains unchanged.

Recovery remains disabled by default and has not been deployed. Donation still
needs correction/replay, its platform restore wrapper, shared legal controls and
an evidence-retention/expiry procedure. The wider fleet cookie/device review
remains outstanding. No new optional processing or Tawny work is included.
