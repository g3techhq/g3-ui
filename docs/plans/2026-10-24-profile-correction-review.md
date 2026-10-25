# Proposed recovery of corrected profiles

The maintainer explicitly approved the exact fields and existing ledger below
on 2026-10-24. This records source implementation scope; it is not a deployment
or a change to the disabled-by-default recovery configuration.

## Exact fields

| App | Correctable profile values in a recovery event |
| --- | --- |
| Swift Circuits | `display_name` |
| Greenside Partee | `display_name`, `handicap` |
| Media Mancer | `display_name`, `handle` (existing implementation) |
| Donation Engine | `display_name`, `handle` |

All other correction keys are rejected. Sign-in email, passwords, provider
identities/grants, billing/purchase credentials, donation/payment history and
camera/device data are excluded. Confirmed financial history needs its own
adjustment or reversal workflow, not this profile endpoint.

## Destination and custody

Use the existing internal service at `http://privacy-ledger:8081/events` as
documented in g3-platform's `docs/privacy-operations.md`. There is no public
listener. Participating app backends use separate app-scoped credentials.
The service encrypts recovery events before uploading to independently held B2
storage and acknowledges only verified external storage. Recovery readers use
separate credentials. This proposal adds no vendor, tracking or marketing use.
The values remain personal information even with an opaque subject identifier.

The event contains only the reviewed fields plus schema/app/request identifiers,
the opaque subject digest, a monotonic subject version, and action `correct`.
Names and handles can identify a person; the ledger would retain an encrypted
copy specifically so an old backup cannot undo the correction. Retained recovery
copies keep the existing separately managed custody and retention obligations.

## Proposed application behavior

The app verifies the current signed-in actor and the reviewed account/session.
It validates normalized field values, allocates one stable request/version, and
commits a private pending request before ledger communication. An unavailable
ledger leaves the current profile unchanged and shows pending status. A matching
acknowledgment permits a guarded transaction to apply and verify the correction,
then remove the temporary payload. Required recovery stays opt-in through the
existing deployment configuration, disabled by default. No host setting changes
or transmission of real account data are proposed during implementation tests.

Deletion and correction share ordering. Pending deletion blocks new corrections;
late workers cannot recreate a deleted account. Quarantined backup replay accepts
only reviewed field shapes, preserves later corrections/deletions, and rejects
conflicting evidence. Synthetic local fixtures test failures and replay without
contacting an external ledger or real provider.

## Approval record

Automatic approval review initially required specific approval of the corrected
values and destination. The maintainer answered: "Approve these fields and the
existing ledger." Implementation may proceed under this scope. No new optional
processing purpose, live host activation or transmission of real test data was
authorized by that answer.
