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

No completion is claimed by this initial inventory. Record implementation and
verification milestones below as each app is handled.
