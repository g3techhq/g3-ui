# Proposed production providers

Checked 2026-10-20. Backblaze B2 is selected by the user and its transport is
implemented in g3-platform. Grafana remains the proposed managed destination.
Accounts, spending, DNS, buckets, remote retention and production deployment have
not been configured. See the implementation handoff for verification and remaining
app integrations.

| Purpose | Recommendation | Reason |
| --- | --- | --- |
| Catalog and public policy service | catalog.g3tech.net | Matches the user's domain and makes the service's role clear |
| Encrypted offsite backups and recovery records | Backblaze B2, separate private buckets | Independent provider from the DigitalOcean host, usage-based storage, scoped application keys and lifecycle controls |
| Operational logs and traces | Grafana Cloud Free initially | Managed Loki/trace services, compatible with the prepared Alloy collection pipeline, bounded free allowance |

## Cost and retention

Backblaze currently lists $6.95/TB/month, the first 10 GB free and no minimum
storage-duration fee. At the posted storage rate, 100 GB of retained data is about
$0.70/month before free allowances and any transfer charges. This measures all
retained backup copies and versions, not just the live database's size.

Grafana currently lists 50 GB/month of logs and 50 GB/month of traces in its free
plan, both with 14-day retention. The proposed seven-day trace schedule in the
earlier plan is therefore not an available Free-plan setting: propose 14 days
for sanitized logs and traces, subject to privacy review. If seven days is a firm
requirement, select another trace backend. Paid plan retention must be reviewed
again before upgrading; its documented minimum is currently 30 days.

Keep the existing proposed 48-hour local backup limit. Target roughly 30 days of
offsite backup availability and longer retention for recovery records, initially
45 days, but derive the final horizon from measured lifecycle/version expiry,
all snapshots and every other recoverable copy. Lifecycle execution is asynchronous;
these starting values are not exact deletion-time guarantees.

## Required implementation checks before enabling the providers

- Keep backups and recovery records encrypted before upload. Keep decryption keys
  off the application server and use separate recovery credentials.
- Use separate private buckets and scoped credentials. Keep lifecycle and account
  administration credentials off the host.
- B2 writeFiles permits hiding files as well as uploading new versions. Removing
  deleteFiles is not sufficient to make a bucket append-only. The generic ledger
  and restore tooling needs provider-specific hardening and a version-aware recovery
  drill before it can claim this protection. Review bounded Object Lock and its
  interaction with the approved retention policy; recover protected historical
  versions even when a current object is hidden or replaced.
- Configure B2's actual lifecycle rules, including hidden versions and unfinished
  uploads. The existing S3-shaped lifecycle JSON is an example, not a verified B2
  configuration.
- Confirm storage regions, provider agreements and actual retention behavior before
  publishing privacy promises. A provider choice does not establish compliance.
- Send sanitized operational data only to Grafana. Keep account content, tokens,
  cookies, emails, raw request bodies and private user identifiers out of telemetry.
- Configure external health, backup-freshness and disk alerts. Monitor free-tier
  limits and ingestion failures so telemetry loss is visible.

## Sources

- [Backblaze B2 pricing](https://www.backblaze.com/cloud-storage/pricing)
- [Application key capabilities](https://www.backblaze.com/docs/cloud-storage-application-key-capabilities)
- [B2 lifecycle rules](https://www.backblaze.com/docs/en/cloud-storage-lifecycle-rules)
- [B2 Object Lock](https://www.backblaze.com/docs/cloud-storage-object-lock)
- [Grafana pricing](https://grafana.com/pricing/)
- [Grafana trace retention](https://grafana.com/docs/grafana-cloud/platform/pricing-and-usage/traces/)

Provider-directory lookup was unavailable because its current authentication had
expired. Recommendations were verified against the primary sources above; no
provider account or subscription was created.
