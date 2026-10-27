# Fleet privacy completion record

The approved source implementation and local verification are complete as of
2026-10-27. Scope is Swift Circuits, Ad Sentinel, Media Mancer, Greenside Partee,
Donation Engine and their shared platform/native support. Tawny received no new
privacy implementation. This record supersedes pending items in the earlier
[chronological plan](2026-10-24-fleet-privacy-rollout.md).

## Delivered behavior

Account apps provide bounded own-data exports, reviewed profile correction,
account deletion, private legal review and minimal deletion progress where
deletion continues asynchronously. The approved fields are Swift display name;
Greenside display name and handicap; Media/Donation display name and handle.
Correction never changes login credentials or confirmed financial history.
Greenside preserves the recorded scoring handicap in started rounds and retains
linkable shared scorecard seats as documented, without claiming anonymity.

Private actions and delayed results are bound to the reviewed account/session.
Ordinary API responses cannot replace login cookies, including errors. Essential
privacy controls remain usable through the optional legal gate. A notice receipt
or terms acceptance is separate from processing consent. No new optional
processing purpose was introduced; unsupported grants/withdrawals remain refused.

Ad Sentinel implements the applicable local-device flow: bounded export, TV
editing, restartable encrypted erasure and device permission/help controls. It
does not invent cloud accounts, financial workflows or consent purposes.

When independently recorded recovery is enabled, leased private outboxes require
an exact ledger acknowledgment before applying the reviewed change. Deletion
supersedes earlier correction; interruptions remain retryable. Quarantined replay
reapplies corrections/deletions after an old backup is restored, refusing conflicts
or incomplete provider evidence. Recovery remains disabled by default. The apps
own their personal-data rules; g3-platform owns the shared encrypted ledger,
backup/replay isolation, app-scoped configuration and deployment examples.

Saved status capabilities require a random request ID and a separate OS-random
256-bit secret. Replies expose only progress/review/provider flags, use
private/no-store and cannot request deletion. Native purchase ownership remains
with the original app account; transfers require support review.

## Source checkpoints

All listed commits are local main checkpoints unless noted. Unrelated working
edits were preserved; Donation's pre-existing untracked Cargo.lock is unchanged.

| Repository | Implementation checkpoints |
| --- | --- |
| Swift Circuits | `8b6466e` correction/replay; `8a56462` ordinary response-cookie protection |
| Ad Sentinel | `ab0200d` applicable local privacy/export/correction/erasure flows |
| Media Mancer | `76ae331`, `e5e47a2` export and account-bound actions; `f2db29f` response protection and 256-bit proofs; `ac45321` current inventory |
| Greenside Partee | `c08195a`, `c5fc7d5`, `1d262ce` correction/scoring; `e3c72e1` response protection; `d51df05` saved receipts; `0904e33` safe embedded-schema bootstrap/guard rollout and final evidence |
| Donation Engine | `28f3f2a` profile/legal/privacy/replay foundation; `246fdf6` response protection; `9501829` legal middleware/reset-token fixes and runtime evidence; `237d623` final verification record |
| g3-platform | `b4d5be0`, `890d9e9`, `cf6745f`, `0415e3a` strict wrappers, shared configuration and Donation overlay; `a1e2f13` final operations record |
| g3-native-plugins | `62c6d0d5bee5aa4adc981c1402b666494a977471` durable encrypted removal; this source push was specifically authorized and completed, with Apple/Android CI passing |

## Verification

Tests ran against frozen reviewed source outside the sibling-development patch
environment, preserving unrelated current work. Strict server/web/mobile-host
checks pass for the four account apps. These are feature/build checks, not an
assertion that every current native device package contains every later fix.

| App/support | Verified evidence |
| --- | --- |
| Swift | 90 game-crate server tests, 56 game-crate web tests in final response follow-up; earlier workspace web, actual Android target/browser, isolated DB and encrypted replay evidence recorded in the app |
| Media | 242 server, 106 web, seven real database scenarios, strict three-feature checks; cross-account held-response/reload browser checks; earlier packaged image/replay checkpoint retained honestly |
| Greenside | 391 server, 195 web; real embedded schema synchronization and failed/repeated guard upgrades; packaged browser phone/wide/keyboard; actual Android APK/AAB and receipt sharing/status; final encrypted image replay and scoped startup |
| Donation | 72 ordinary server tests, 42 web, eight isolated database scenarios, strict three-feature checks; actual browser correction/export/legal/account switching and Android pending/restart/apply; required legal/outage/reset-owner HTTP checks; final encrypted image replay |
| Ad Sentinel | 98 tests, strict desktop, Android package/emulator/relaunch evidence from its implementation checkpoint |
| Shared support | 65 platform tests and compose validation; full g3-ui `just pre-push`; native SDK Android restart/removal tests and Apple/Android CI |

The final Greenside local image is
`sha256:1eea3593c1deadfed3e7dded71bb0fb15880742c91e0f67d9b2dbb23aa1793da`.
The final Donation local image is
`sha256:00bf67b3f874646410516b8afc62d5b06e1fa17fb220530e676841a3aa00be94`.
Both pass actual encrypted synthetic old-backup import and ordered replay,
preserving another account and the applicable historic records. These are local
Docker image IDs, not published registry manifest digests.

Detailed current evidence:

- [Greenside receipt/build report](../../../greenside-partee/docs/verification/2026-10-27/receipt-verification.json), [HTTP/database read-back](../../../greenside-partee/docs/verification/2026-10-27/receipt-web-readback.json) and [upgrade/recovery instructions](../../../greenside-partee/docs/privacy-recovery.md).
- [Donation fleet report](../../../donation-engine/docs/verification/2026-10-26/fleet-privacy-verification.json), [HTTP report](../../../donation-engine/docs/verification/2026-10-26/fleet-http-verification.json) and [Android report](../../../donation-engine/docs/verification/2026-10-26/fleet-native-verification.json).
- [Platform operations](../../../g3-platform/docs/privacy-operations.md), including app-specific restore wrappers and deployment settings.

Greenside's Android receipt package predates the final input-edit clearing effect
and server-only bootstrap fixes; the final browser/image and host checks cover
those changes. Donation's native follow-up checks correction/legal behavior,
not native permanent deletion/share confirmation. No Apple device runtime,
live provider success, real B2 custody/expiry or universal current-device parity
is claimed. Synthetic catalogs were never published as real legal documents.

## Deployment boundaries and cleanup

No shared host setting was changed, no app/image/crate was published and no real
Apple, Stripe, Every.org or offsite operation was performed. The specifically
authorized native SDK source push is the only publication in this effort.

Production rollout uses reviewed published image digests, app-scoped credentials,
the documented schema/guard transitions, actual offsite custody/expiry evidence
and the existing controlled deployment procedure. Legal enforcement requires
approved published documents. Preserve retirement/request evidence through the
reviewed backup/late-writer horizon. These remain explicit operator rollout
requirements, not silently enabled features or claims of legal compliance.

The owned browser tabs were closed and viewport override reset. The owned
read-only Android emulator was stopped without saving its source AVD. Exact
fixture container/network IDs were checked and removed; ports 18086, 18087, 18089–18094 and
18098–18099 used by the fixtures were verified closed. No development server was
started and no unrelated service was stopped.
