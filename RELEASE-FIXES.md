# Release fixes — October 10, 2026

Scope: all ten workspace projects. The ten audit findings now have source,
configuration or release-preparation fixes. No crate was published, workflow
dispatched, deployment performed, or `dx serve` started. Existing Tawny player
and transport changes were retained.

## Changes

| Audit finding | Fix |
| --- | --- |
| Sibling g3-auth dependency | Move sibling patches into the parent `.cargo/config.toml`, outside every repository. Install staged-lockfile commit guards in all ten repositories. CI and Docker use versioned crates.io dependencies; no sibling checkout or named context is required. |
| Mobile CLI installation | Correct all six missing `@` separators in g3-stack, Tawny and Media Mancer. |
| Greenside player privacy/access | Replace full `User` payloads with `PlayerProfile` containing only id, display name and handicap, including the join and scoring queries. Authorize game owners, participants and enclosing-tournament viewers; refuse unrelated or deleted accounts. Check initial snapshots, every live message and idle subscriptions every five seconds. Stop sending when the socket closes. |
| Tawny account deletion | Add a guarded endpoint and destructive confirmation for guests and permanent accounts. Transactionally remove owned data and playlist relations, recompute shared channel polling, and delete the account. Disable auth-user caching so another device loses access. Discard pending playback progress, stop playback and clear the device cache. |
| Tawny auth throttling | Separate limits for guests, registration and sign-in, plus a shared ceiling against cookie rotation. Configure trusted proxy addresses explicitly. Run password hashing/verification on blocking workers. |
| g3-auth release version | Prepare 0.1.3 and update the consumers using its new features. Publishing remains a separate release action. |
| Browser native-plugin lint | Add browser storage `Default`, missing external-URL docs, WASM checks and strict lint to the quality gate. |
| Android billing query failures | Check both INAPP/SUBS query results; return the existing error envelope for either failure. Preserve the last successful catalogue. Also report consumption and acknowledgement failures. |
| Cache growth and owner races | Prepare g3-cache 0.2.1. Default each store to 256 answers and 32 MiB of JSON payload; expose configurable limits. Evict memory by recent reads, disk by saved age, and remove expired/malformed disk answers. Reject previous-owner results and immediately hide previous-owner mounted answers. |
| Swift formatting | Format the two failing Rust files and verify the formatting gate. |

Additional release fixes:

- All five library publish workflows depend on quality checks for their selected
  commit. Multi-crate dry runs verify the parent archive and fail explicitly
  when its new macros version has not reached the registry.
- Native-plugin publishing also requires Android and iOS testbed builds, which
  compile the Kotlin/Swift implementations. Prepare g3-native-plugins 0.4.4.
- Verify g3-ui's declared Rust 1.88 minimum locally and add it to CI/publishing.
- Raise Tawny's Docker Rust image from 1.89 to the workspace's 1.95 toolchain.
- Earlier locked checks used sibling auth and registry versions of other
  libraries. They do not verify the final registry-only auth graph, which
  requires publication and lockfile updates. Development overrides now live
  only in parent Cargo configuration outside the repositories. Each affected application has
  `docs/release-dependencies.md` explaining development and release inputs.

## Validation

| Project | Evidence obtained |
| --- | --- |
| g3-ui | Rust 1.88 all-features library check passes. New CI/publish workflow passes actionlint. Existing component behavior is unchanged; the original audit's full pre-push result still applies to that code. |
| g3-auth | Full pre-push passes: client, server/social/rate-limit, native-client features, WASM, strict lint, 63 library tests and 11 macro tests. The 0.1.3 archive compiles with server, Google, Apple and rate-limit features against registry dependencies. |
| g3-cache | Full pre-push passes, including all backends, WASM lint and both testbed builds. 13 server, eight client/key and ten macro tests pass. Tests cover capacity, persistent expiry/eviction and a mounted owner switch. The 0.2.1 archive compiles. |
| g3-native-plugins | Full pre-push passes, including new WASM strict lint; 23 default and 43 all-feature tests pass. Billing contract assertions pass. The 0.4.4 archive compiles. Platform build workflows pass actionlint. |
| g3-route-transitions | Publish workflow passes actionlint. Runtime code is unchanged; the original audit's pre-push result applies. |
| g3-stack | Mobile workflow passes actionlint. Runtime code is unchanged; original web/server/mobile checks and tests apply. |
| Tawny | Web/server/mobile/desktop checks pass with the local libraries. Full server suite: 129 pass, one existing skip. Final account suite: 11 pass, including HTTP 429/Retry-After limits under cookie rotation, owned-table deletion, playlist relations, another account, other-device authentication and shared channel polling. Server/client strict lint passes. Locked client/server checks pass with only the committed auth override. Compose configurations validate with placeholder deployment values. |
| Greenside Partee | Prescribed checks and information-site check pass. Full server suite: 290 tests pass, including profile serialization and game authorization/revocation. Server strict lint passes. Locked CI client graph passes. |
| Media Mancer | Prescribed web/server/mobile checks pass; locked CI client graph passes with only the committed auth override. |
| Swift Circuits | Formatting and locked workspace client/server checks pass. |

Changed workflows pass actionlint (without shellcheck/pyflakes). Whitespace
checks pass. Rust commands used the repository's documented Windows NASM
workaround where needed. Local packaging locks were moved off a yanked
yoke-derive version; those library locks are intentionally ignored by Git.

## Remaining release actions and validation

1. Review the prepared library changes. Sibling auth is now strictly local development configuration; the earlier CI/Docker sibling support was removed after the user clarified its purpose.
2. Publish g3-auth 0.1.3, g3-cache 0.2.1 and g3-native-plugins 0.4.4 through their gated workflows after the maintainer's release decision. Then update **all five applications' registry lockfiles** with local patches disabled and rerun locked release checks. The existing auth path lock entries were produced by development builds and must be resolved against the actual published crate; no registry checksums were fabricated. Keep sibling patches in the parent Cargo configuration for local development only. Commit guards reject staged local shared-library lock entries.
3. Run the remote platform builds and device smoke tests. No Android device or
   emulator is connected here; iOS requires macOS. Kotlin billing changes have
   contract coverage but have not been exercised against the Play store.
4. Review Tawny's deletion confirmation in a running browser/native app and
   exercise active playback during deletion. Port 8080 had no running server;
   the instruction not to start `dx serve` was honored.
5. Verify the remaining claimed application minimum Rust versions, advisory
   scans, signed native bundles and deployment configuration for the selected
   release commit. These were validation gaps, not confirmed source defects.

This records implemented fixes and observed checks, not a certification that
every native platform or production deployment has been exercised.
