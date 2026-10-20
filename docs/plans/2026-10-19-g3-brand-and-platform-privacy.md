# g3-brand and shared platform implementation plan

Date: 2026-10-19
Status: Shared foundation implemented; application integration and rollout pending
Scope: g3-brand, g3-platform, g3-native-plugins, g3-stack, and consuming apps

This plan captures the agreed direction for shared g3tech branding, app discovery,
store-update notices, legal documents, and privacy operations. It also covers
private dependency access during GitHub Actions builds. It authorizes no production
deployment or legal-policy publication by itself.

The implementation now lives in the private
[g3-brand repository](https://github.com/g3techhq/g3-brand). Its
[implementation status](https://github.com/g3techhq/g3-brand/blob/main/docs/implementation-status.md)
records verified work and remaining prerequisites. Platform operations are in
g3-platform's docs/privacy-operations.md. This copy retains the agreed design;
g3-ui remains a general component library.

## 1. Outcomes

- Every participating app has a recognizable g3tech presence and a way to discover
  other g3tech apps.
- App listings, links, release notices, announcements, and legal documents can
  change centrally without shipping another app build.
- Shared UI inherits each app's theme, dark/light setting, and iOS/Material mode.
- iOS and Android installations can discover optional and required store updates.
- Users can read applicable policies, understand changes, and make explicit choices
  when acceptance or specific consent is required.
- Export, correction, deletion, and consent withdrawal have real backend workflows.
- Backups, logs, traces, and recovery procedures follow documented retention and
  deletion rules. A restore cannot silently undo a user's completed request.
- Private source is accessible to authorized developers and CI without embedding
  credentials in application images.

## 2. Architecture and ownership

```text
Web / iOS / Android apps
  ├─ g3-brand UI and client
  │    └─ public, cacheable HTTPS content API
  │         └─ g3-brand service
  │              └─ dedicated catalog/legal database
  └─ authenticated app backend
       ├─ app database, files, sessions, and caches
       ├─ acceptance and consent records
       └─ privacy requests and durable recovery records

g3-platform on DigitalOcean
  ├─ Caddy and app/service containers
  ├─ shared SurrealDB instance with restricted app credentials
  ├─ encrypted logical backups and expiry jobs
  └─ Alloy collection → configured log/trace storage
```

### g3-brand

Use one private repository for the client library and central service, with a
coordinated release process. Prefer a small library plus a service binary; split
additional crates only when dependency or build boundaries require it. Gate
server dependencies so web and mobile clients never compile the database/admin
implementation.

The library owns typed content models, fetching, cache integration, presentation,
update evaluation, and host-app callbacks. It uses public g3-ui components.

The service owns public catalog/legal content and authenticated administration.
The initial catalog does not require accounts, cross-app identity, or user-level
analytics. Public content requests must not contain user identifiers or forward
app session cookies.

### Each consuming app

The app provides its stable identity, environment, release channel, installed
native version/build, and hooks into its own authenticated backend. That backend
owns acceptance receipts, consent enforcement, data exports, corrections, and
deletion. Installing g3-brand does not make those operations automatic.

### g3-platform

The platform owns service deployment, database credentials, backup encryption and
expiry, restore procedures, telemetry configuration, and operator access.

### Existing shared libraries

- g3-ui: generic components, theme context, accessibility, and layout.
- g3-cache: caching where its existing behavior fits; check public catalog lifetime
  against its account-scoped cache clearing. Private receipts remain account-scoped.
- g3-native-plugins: native app identity/version/build and opening store URLs.
  Keep store-version checking distinct from its existing content-bundle updater.
- g3-stack: document the integration pattern. Keep the public template usable
  without access to a private g3-brand repository.

## 3. Current platform baseline

Reviewed local g3-platform configuration; live cloud settings were not inspected.

| Area | Existing implementation | Gap |
| --- | --- | --- |
| Hosting | One Droplet, Caddy, one SurrealDB, per-app containers; images pulled by digest | Add central service and verify live regions/access controls |
| Database isolation | Namespaces per app, shared root login | Restrict runtime identities; separate schema/admin credentials |
| Backups | Hourly logical exports and exports before deployments | Encryption, maximum-age expiry, recovery workflow |
| Local backup retention | Last 48 files per app | Count cap does not establish an age limit |
| Offsite copies | Optional rclone upload | Destination, encryption, lifecycle, historical-version expiry not enforced here |
| Docker logs | json-file rotation, approximately 50 MB per container | No age guarantee or source-level data policy |
| Central logs | Optional Alloy forwarding to Loki | Retention, filtering, deletion handling, access and region verification |
| Traces | No trace pipeline configured | Instrumentation, collector/export path, sampling and retention |
| Recovery | README recommends periodic restore drills | No implemented deletion/correction replay procedure |

Files to change later include g3-platform's compose.yml, compose.logging.yml,
alloy-config.alloy, Caddyfile, cloud-init.yaml, scripts/backup.sh, deploy scripts,
systemd units, and operational documentation.

## 4. Client configuration and UI

Configure the integration once near the app root:

- Stable app ID; platform, channel, and environment.
- Installed native version and build identifier, supplied by trusted app metadata.
- Central API base URL and public web links.
- Host handlers for privacy requests, legal receipts, and native link opening.
- Optional product copy/localization overrides.

Render inside the existing AppWrapper. Do not introduce another theme provider.
Read ambient theme/mode and use g3-ui components and color tokens. Keep the app
and library on one compatible g3-ui/Dioxus dependency line.

Expose both individual pieces and a ready-made About/g3tech page:

- Made by g3tech / More from g3tech entry.
- App catalog, excluding the current app by stable ID.
- About details with installed version/build and support/feedback links.
- Legal document viewer, revision history, and change notices.
- Update notice and required-update screen.
- Privacy controls and request status using host-provided backend actions.
- Release notes and targeted announcements in a subsequent phase.

Use accessible headings, named links, keyboard navigation, focus restoration,
and appropriate announcement semantics. Support compact/wide layouts and both
platform modes without hardcoded brand colors overriding the host theme.

### Dependency maintenance

Catalog and policy content changes require no library release. Compatible g3-ui
updates should resolve within the supported dependency range. Breaking g3-ui or
Dioxus updates require a compatibility check and usually a g3-brand release.
Document that commitment; do not claim the crate is independent of UI upgrades.

## 5. Central content service and data

Use a dedicated namespace/database on the existing SurrealDB instance, with
separate production and nonproduction data. Its runtime reader cannot read app
databases. Keep migrations and administration outside ordinary public reads.

Initial record types:

| Record | Key fields |
| --- | --- |
| App | Stable ID, name, description, icon URL, published status, sort order |
| App links | Website, iOS store, Android store, support, feedback |
| Platform release | App, platform, channel, latest version/build, minimum supported release, store URL, message, enabled/effective dates |
| Legal revision | Document ID, immutable revision, locale, content, content digest, publication/effective dates, change summary |
| Legal applicability | App and platform where relevant; references to shared document and app supplement revisions |
| Release notes | App/platform/channel/release, localized content |
| Announcement | Stable ID/revision, applicable apps/platforms/version ranges, severity/style, start/end dates, dismissal rule |

Use typed values for fixed behaviors. Public responses contain published content
only. Do not put acceptance records, user identities, database IDs, or arbitrary
executable markup into this API.

Provide a versioned HTTPS API with bounded payloads, ETags, cache headers,
request timeouts, and a documented backwards-compatibility policy. Old app builds
must continue working as optional fields and content are added. Define behavior
for unknown enum values and unsupported response versions.

Choose direct client reads or an app-backend proxy after checking browser CORS,
native networking, and existing g3-cache integration. Keep either route public
and independent of account cookies. Do not couple catalog availability to login.

Store public icons and document assets at stable public URLs. Select the hostname,
storage destination, and cache invalidation strategy before deployment.

Start administration with a validated, authenticated CLI/script that creates
drafts and publishes revisions. Avoid unaudited direct production edits. Add an
admin UI later. Validate links, applicability, dates, version ordering, and content
before publication; allow disabling notices quickly and repointing the current
document revision without modifying immutable history.

## 6. Caching and offline behavior

- Show cached catalog content immediately and refresh in the background.
- Refresh on opening the section and on app foreground, with a bounded interval
  and backoff rather than a request on every navigation.
- Retain the last valid response if a refresh fails. A catalog outage must not
  block the app.
- Cache published legal revisions for reading, including the exact revision
  displayed during an acceptance flow.
- Remember notice dismissals by app, notice revision, and appropriate account or
  installation scope. Snooze optional updates so foreground checks do not nag.
- Never infer acceptance or consent from a cached fetch, dismissal, timeout, or
  network failure.
- A backend operation requiring a particular accepted revision or consent checks
  its authoritative record. Only the affected operation is restricted when that
  requirement cannot be established.

## 7. Store-update behavior

Maintain separate iOS and Android policies, including release channel. Evaluate
actual native version/build metadata, not just the Rust package version. Define
platform-specific ordering and test it; never compare version strings lexically.

- Below latest but at/above minimum: optional Update / Later prompt.
- Below minimum: explicit update-required experience, reason, store link, retry,
  and support route. Retain access to legal documents and privacy-request routes.
- Initial implementation opens the appropriate store listing. Android's native
  in-app update flow is an optional later enhancement.
- Activate announcements only when the intended users can actually obtain the
  release. Account for store approval, staged rollout, country/channel
  availability, and minimum OS support before raising the minimum.
- A minimum version that protects an incompatible backend operation is enforced
  by that backend as well as the UI. Client version headers alone are not an
  authorization mechanism.
- Define policy expiry and offline behavior explicitly. A failed content check
  must not invent an update requirement; a previously confirmed requirement must
  not become an irreversible offline trap.

## 8. Shared legal documents and notices

Use shared g3tech terms/privacy content with explicit app-specific supplements
where practices differ. Legal review must confirm the operating entity, scope,
applicability, jurisdictions, retention, processors, and actual data practices.
Shared branding does not imply shared accounts or permission to combine app data.

Resolve each app's applicable shared and supplement revisions into an immutable
document set. An acceptance receipt references the exact set and content digests,
so later shared edits cannot change what the person accepted. Only notify an app
when its applicable document set changes.

Support three distinct actions:

1. Notice shown/acknowledged: an informational policy change.
2. Terms accepted: affirmative agreement where the reviewed terms flow requires it.
3. Specific consent granted/withdrawn: a separate purpose-specific choice.

Do not present every privacy revision as mandatory consent, combine optional
processing with required terms, or treat acknowledgment as permission to collect
new data. OS permissions and store disclosures remain separate obligations.

Each revision needs a publication date, effective date, change summary, locale,
applicability, and reviewed notice/acceptance behavior. Preserve revision history
and permanent public URLs. Public policy pages must work without signing in.

Host-app backend receipts include account/app scope, exact document revisions,
action, and server timestamp, with only necessary evidence. Authenticate requests
and handle stale submissions by requiring the person to review the correct
revision. Do not claim acceptance succeeded before the backend commits it.

## 9. Privacy rights and data inventory

Before drafting final promises, inventory each app's data flows:

- Account/profile records and application content, including shared content and
  records referencing other users.
- Uploaded objects, generated assets, search indexes, queues and background jobs.
- Authentication sessions/tokens, push registrations, device caches and CDN data.
- Logs, traces, security records, support tickets, and crash reports.
- Backups, historical object versions, Droplet/volume snapshots, and restore copies.
- Payment/accounting records and legal/consent/request evidence with justified
  exceptions and retention periods.
- External processors, their regions, contracts, subprocessors, and deletion/export
  capabilities; include DigitalOcean, Cloudflare, and the configured telemetry and
  offsite-storage providers.

For every category, record purpose, applicable legal basis, owner, location,
retention trigger/maximum age, deletion method, recipients, and exceptions.
Determine GDPR territorial scope and CCPA/CPRA applicability; California business
thresholds mean its rules do not automatically apply to every small app. Review
other applicable laws rather than treating these two as an exhaustive list.

Implement verified request workflows for access/export, correction, deletion, and
consent withdrawal, with opt-out/limitation controls where applicable. Use a safe
request channel for people who cannot sign in. Set jurisdiction-appropriate
deadlines, escalation, request evidence, and completion wording after review.

Exports must be securely delivered and expire. Deletion must invalidate sessions,
stop jobs that could recreate data, clear reachable caches, and propagate to
processors. Any justified retained records must be minimal, purpose-restricted,
and explained. An opaque or hashed identifier can still be personal data.

## 10. Backups, erasure, and recovery

### Proposed engineering defaults

These values are proposals, not legal safe harbors or existing guarantees.

| Category | Proposed maximum age | Notes |
| --- | --- | --- |
| Local logical exports | 48 hours | Also cap space/count; expire even when new backups stop |
| Encrypted offsite exports | 30 days | Apply to relevant historical versions and all replicas |
| Routine application logs | 14 days | Include local copies and collector buffers |
| Detailed traces | 7 days | Use sampling and strict field selection |
| Security/legal/request records | Separate reviewed schedule | Do not hide these in routine logs |

The system's actual longest recoverable copy determines deletion timing. Account
for expiry job delays and provider behavior before promising an exact deadline.
CCPA-covered request/response records have a minimum 24-month retention
requirement under section 7101; this does not justify retaining the deleted account
or unrelated logs for that period.

### Backup deployment

- Keep database-aware logical exports per app/database, including the central
  catalog/legal service. Review how exported schema/configuration handles users
  and secrets; restore through controlled administrative tooling.
- Compress and encrypt before writing backup payloads to disk or sending them
  offsite. Restrict file permissions and avoid plaintext temporary exports.
- Keep recovery keys outside the Droplet with documented recovery access and
  rotation. An encrypted backup without recoverable keys is unusable.
- Configure destination expiry, version expiry, and multipart cleanup as supported;
  verify the actual provider's capabilities. Give upload credentials minimal access
  and separate lifecycle administration from the production host.
- Schedule age-based local cleanup independently of backup success. Retain a
  separate space/count limit and alert on failed or unexpectedly empty exports.
- Verify encrypted integrity/decryption and periodically import into a scratch
  instance. gzip integrity alone does not prove the database can be restored.
- Define recovery point/time objectives and backup success/freshness alerts.
- Inventory DigitalOcean backups/snapshots separately. Snapshots can persist
  indefinitely and Droplet backups do not cover attached block-storage volumes.
- Document a justified backup-use restriction and expiration schedule; erasure
  handling must follow the applicable jurisdiction's rules, not a blanket claim
  that backups are exempt.

### Durable recovery records

Maintain a restricted, independently recoverable record of completed deletions and
other changes that must survive rollback. Include app scope, stable non-reused
subject ID, action/version, and processing state; minimize personal content.

Design the request/outbox/ledger sequence so an app cannot report a deletion
complete and then lose the only record during a crash or restore. If an external
ledger is used, completion waits for its durable acknowledgment. The ledger must
remain available when the shared SurrealDB server itself is lost or rolled back.

Keep recovery records at least through the last backup/snapshot that could
reintroduce the affected data, plus the documented processing margin, subject to
applicable record-keeping obligations. Corrections and withdrawals need their own
reviewed minimal representation, not copies of entire account histories.

### Restore procedure

1. Restore into an isolated environment with outbound jobs, notifications, and
   production access disabled.
2. Obtain the current independent recovery records, not just the restored copy.
3. Reapply relevant deletions, corrections, withdrawals, and session invalidation
   using tested idempotent app-specific operations.
4. Reconcile file/object stores, indexes, caches, and processors as needed.
5. Verify representative deleted users remain absent and revoked processing
   remains disabled. Confirm required schema/version compatibility.
6. Record the recovery watermark/evidence and only then permit production traffic.
7. Expire scratch copies and temporary credentials after the exercise.

Automate the procedure and run an end-to-end drill before production rollout,
then at least quarterly and after recovery-path changes.

## 11. Logs and tracing

Standardize app logging with structured tracing output and an allowlist of fields:
service, environment, app release, route template, status, duration, random
request/trace correlation, and safe error classification.

Exclude raw request/response bodies, email addresses, tokens, cookies,
Authorization headers, raw query strings, and user content by default. Review
error formatting, panic output, SDK logging, OAuth URLs, DB errors, and HTTP span
attributes so indirect paths do not leak the same data.

Redact at the application before Docker stores stdout/stderr. Collector-side
filtering is an additional control; it cannot erase an earlier local copy.

Extend the existing Alloy deployment to collect logs and receive/export OTLP
traces to a selected, configured destination. Confirm whether the existing Loki
destination is managed or self-hosted before choosing trace storage. Do not assume
the current log-only configuration already collects traces.

- Label by bounded values such as service and environment. Fix the current
  comment proposing user/request IDs as Loki labels. Correlation identifiers,
  when needed, belong in structured fields/metadata rather than index labels.
- Choose bounded sampling and production verbosity; temporary debug logging gets
  an automatic expiry.
- Enforce time-based expiry locally and remotely; rotation by bytes alone is
  insufficient. Verify collector WAL/buffer and provider deletion delays.
- Audit host journals, authentication/fail2ban logs, reverse-proxy errors, and
  Cloudflare/provider logs in addition to app logs. IP addresses can be personal data.
- If necessary personal information remains in telemetry, document its purpose,
  access, retention, and a tested deletion method or applicable retention exception.
  Short retention is not by itself a substitute for handling a valid request.
- Restrict dashboards/collector ingress, use authenticated encrypted export, and
  review operator access. Keep retained compliance records separate from diagnostics.
- Alert on backup failure/freshness, disk pressure, expiry failures, collector
  backlog, and privacy-request processing failures without including user payloads.

## 12. Shared deployment changes

- Add the central service container, image digest variable, hostname, readiness
  check, deploy target, and backup inventory entry.
- Provision restricted database identities for each app/service. Prefer
  database-scoped identities where possible and separate schema migration users.
  Remove shared root credentials from app containers after connection changes
  are verified. Keep administrative recovery access restricted.
- Keep SurrealDB private to the host/container network; expose only intended
  HTTPS routes through Caddy. Reverify Cloudflare-origin firewall assumptions.
- Install backup/encryption/expiry jobs, telemetry configuration, and monitoring
  with documented credentials and recovery access.
- Verify provider regions, data-processing agreements and transfer mechanisms
  against the data inventory before publishing the final privacy policy.
- Keep staging separate, with synthetic data. Do not casually restore production
  user data into developer environments.
- Record effective configuration and a repeatable deployment/rollback procedure.
  Application rollback must preserve legal receipts and durable privacy actions.

## 13. Private source distribution and GitHub Actions

Start with a private g3techhq/g3-brand repository and Git dependencies pinned to a
release tag or full commit. Commit each application's Cargo.lock. Do not create
or publish anything until the repository and automation are explicitly set up.

Set package.publish = false in unpublished packages to prevent accidental registry
publication. A private Cargo registry is a later option if version-range publishing
becomes worth operating; it is not needed for this initial distribution model.

Use HTTPS dependency URLs for GitHub App token authentication. Developers use
their authorized Git credentials; Cargo can use the Git CLI/credential helper.

### GitHub App setup

1. Register an organization GitHub App with Contents: read-only, installed on the
   private dependency repository only.
2. Save its identifier as an organization variable and private key as an
   organization Actions secret, shared only with approved consuming repositories.
3. Generate an installation token in each authorized workflow, explicitly scoped
   to g3-brand. Use the GitHub-owned token action, pinned to a reviewed commit.
4. Pass the generated token to Docker BuildKit as a secret. Use an ephemeral Git
   credential helper that reads the secret; never embed it in dependency URLs,
   build arguments, image layers, command output, or persistent Git configuration.
5. Keep GHCR publishing on the workflow's existing GITHUB_TOKEN. That token cannot
   generally read a different private source repository.

### Docker and other builds

- Add authenticated fetching to every stage that may need the dependency. In the
  current cargo-chef pipeline, account for both cargo chef cook and dx bundle.
- Prefer fetching all required locked dependencies early, while the token is valid,
  and reusing the source cache for later compilation. Test native and wasm paths
  and long cold builds against the installation token's expiry.
- Do not rely on runner checkout credentials being inherited inside Docker.
- Keep source-bearing build caches private; do not export private crate sources
  through public cache artifacts or runtime layers.
- Apply equivalent authorization to native iOS/Android builds and non-Docker Rust
  checks. A Docker-only fix does not cover those workflows.
- Run credentialed builds only for trusted code. Define fork-PR behavior without
  granting untrusted changes the GitHub App key or dependency token.
- Document key rotation/revocation and consumer onboarding. Consider checking out
  or vendoring the exact locked source before compilation if token lifetime or
  Cargo authentication makes direct builder fetching unreliable.

The DigitalOcean host continues pulling finished images by digest. It needs image
registry access, not private Rust-source credentials. Private distribution controls
source access; it does not make client-delivered code or embedded values secret.

## 14. Implementation phases

| Phase | Deliverables | Exit criteria |
| --- | --- | --- |
| 0. Confirm scope | Per-app data inventory, applicability review, hostnames, repository/package layout, provider/region inventory, retention and recovery objectives | Written decisions and owners; legal/privacy requirements reviewed |
| 1. Platform privacy foundation | Restricted DB users, encrypted local/offsite backups, enforced expiry, durable recovery records, restore tooling, logging rules | Recovery drill proves deletion survives restore; expiry and access checks pass |
| 2. Private build foundation | Private repo, GitHub App, authorized Docker and mobile builds | Cold build fetches dependency; tokens absent from images/logs; public template still builds |
| 3. Catalog service and UI | Versioned public API, validated publication tooling, cached catalog, About/support page | One pilot app works in both modes/themes and offline; public data only |
| 4. Legal and privacy integration | Versioned document sets, notices, exact receipts, consent choices, host request workflows | Backend enforcement and cross-device behavior verified; reviewed policy matches operations |
| 5. Store updates | Native version/build helper, per-platform release policies, optional/required prompts | Real store navigation and availability checks pass on iOS and Android |
| 6. Fleet rollout | Pilot follow-up, remaining app integrations, optional release notes/announcements, operator guide | Each app passes its data/UI/recovery checks; live configuration verified |

Phases 1 and 2 can progress independently after scope decisions. Public catalog
work can begin before privacy operations are finished, but do not publish deletion
promises or enable new data collection on the strength of UI alone.

Use one pilot app selected for active maintainability and a representative mobile
build. Roll out to Media Mancer, Greenside Partee, Swift Circuits, and other apps
after confirming each app's actual readiness; the platform's current hosted list
does not imply every app already has identical auth or deletion capabilities.

## 15. Validation and acceptance

### Client/service

- Markup/accessibility tests for new UI and contract tests for content, version
  compatibility, date/app targeting, and malformed/unknown inputs.
- Validate optional/required updates across version/build/channel/OS scenarios.
- Verify exact document-set acceptance, publication races, account switching,
  cross-device receipts, withdrawals, dismissals, stale caches, and network failure.
- Inspect compact/wide layouts in iOS/Material and light/dark; exercise keyboard
  and native devices/emulators, external browser/store links, and actual native
  installed-version metadata.
- For g3-ui changes, follow its markup/playground, changelog, consumer verification,
  and just pre-push requirements. Reuse the running playground; never start
  dx serve without the user's instruction.
- Confirm the service's public endpoints cannot return drafts or private receipts
  and the admin surface requires appropriate authorization.

### Privacy/platform

- Create a synthetic user with records, files, sessions and related processor data;
  back up, delete, restore, replay, and prove the user does not return.
- Repeat for correction and consent withdrawal, including jobs that could restart
  processing. Simulate ledger unavailability and a crash during request processing.
- Demonstrate expiry for local/offsite backups, historical versions, snapshots,
  routine logs, traces, collector buffers, and scratch restores as applicable.
- Confirm app credentials cannot read another app or the central database.
- Inject synthetic canary secrets/personal values through requests, errors and
  spans; verify they do not appear in local logs or remote telemetry.
- Exercise secure exports, request verification, processor deletion, legal
  exceptions, request deadlines, and minimal retained completion evidence.
- Verify backup decryption and recovery with the production host unavailable.

### CI/distribution

- Cold and warm Docker builds plus native iOS/Android and web builds succeed with
  the private dependency and locked revisions.
- Removing authorization produces a clear dependency-fetch failure.
- Image history, layers, logs and exported caches contain no access credentials;
  source-bearing caches have private access controls.
- Unauthorized repositories and untrusted fork workflows cannot acquire the key.
- Production deployment requires only the finished image, not the private source.

Completion requires evidence of the actual live retention and restore behavior,
not just configuration files. Track any blocked checks explicitly before rollout.

## 16. Decisions still to resolve

1. Final crate/service names, package layout, public hostname, and pilot app.
2. Whether g3-platform is already deployed and which live settings differ from
   this repository.
3. Offsite backup destination, encryption/key recovery, and longest allowed
   recoverable copy including snapshots and replicas.
4. Existing Loki hosting/region, trace destination, deletion capabilities, and
   achievable retention for collector/provider copies.
5. Actual controller/legal entity, GDPR/CCPA applicability, processors/transfers,
   required notice/consent behavior, and approved retention exceptions.
6. Independent durability mechanism and ownership for privacy recovery records.
7. Native version/build API and required-update behavior for unsupported OS or
   users outside a release's store availability.
8. Public catalog caching route, document rendering format/locales, and initial
   publication workflow.

## 17. Reference sources

Sources checked on 2026-10-19. Recheck applicable law, store policies, and provider
capabilities during implementation. This is an engineering plan, not certification
of GDPR/CCPA compliance or a substitute for reviewing the actual legal documents.

- [GDPR: Regulation (EU) 2016/679, including Articles 5, 12, 17, 19 and 32](https://eur-lex.europa.eu/legal-content/ENG/ALL/?uri=celex%3A32016R0679)
- [UK ICO: right to erasure and backup handling](https://ico.org.uk/for-organisations/uk-gdpr-guidance-and-resources/individual-rights/individual-rights/right-to-erasure/)
- [California current laws and regulations](https://www.cppa.ca.gov/regulations/index.html)
- [CCPA regulations effective 2026-01-01: sections 7022 and 7101](https://www.cppa.ca.gov/regulations/pdf/ccpa_statute_eff_20260101.pdf)
- [CPPA applicability FAQ](https://www.cppa.ca.gov/faq.html)
- [Apple App Review Guidelines: privacy](https://developer.apple.com/app-store/review/guidelines/#privacy)
- [Google Play User Data policy](https://support.google.com/googleplay/android-developer/answer/10144311?hl=en)
- [Android in-app updates](https://developer.android.com/guide/playcore/in-app-updates)
- [DigitalOcean backup limitations](https://docs.digitalocean.com/products/backups/details/limits/)
- [DigitalOcean snapshot retention](https://docs.digitalocean.com/products/snapshots/details/features/)
- [Grafana Loki labels](https://grafana.com/docs/loki/latest/get-started/labels/)
- [Grafana Loki log deletion](https://grafana.com/docs/loki/latest/operations/storage/logs-deletion/)
- [Cargo Git dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [Cargo Git authentication](https://doc.rust-lang.org/cargo/appendix/git-authentication.html)
- [Cargo registries and publication restrictions](https://doc.rust-lang.org/cargo/reference/registries.html)
- [GitHub App authentication in Actions](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/making-authenticated-api-requests-with-a-github-app-in-a-github-actions-workflow)
- [GitHub Actions token scope](https://docs.github.com/en/actions/concepts/security/github_token)
- [Docker build secrets in Actions](https://docs.docker.com/build/ci/github-actions/secrets/)
