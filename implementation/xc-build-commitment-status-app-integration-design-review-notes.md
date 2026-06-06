# XC Build commitment status app integration design review

This document reviews the XC Build commitment status app integration design milestone.

Reviewed branch:

    xc-build-commitment-status-app-integration-design-review

Reviewed design milestone:

    xc-build-commitment-status-app-integration-design

Reviewed files:

- implementation/xc-build-commitment-status-app-integration-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build commitment status app integration design is accepted.

The design correctly exposes `commitmentStatus` as optional current XNTD commitment context.

The design correctly avoids turning commitment status into global enforcement.

The design correctly states that UNCOMMITTED Build must not cause historical proof rejection.

The design correctly preserves appSubmitProof behavior.

The design correctly preserves watcher behavior.

The design correctly preserves registrar behavior.

The design correctly preserves proof payload behavior.

The design correctly avoids direct real RPC access in the app/service view layer.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-commitment-status-app-integration-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Commitment status review

The design correctly uses the new terminology:

    commitmentStatus

The design does not use `activeStatus` as the app/service direction.

The design correctly frames commitment status as:

    current XNTD commitment context

not:

    Build validity
    historical contribution validity
    global eligibility
    punishment

## App view review

The recommended helper direction is accepted:

    appGetBuildView()

The recommended future view shape is accepted:

    AppBuildView {
      build
      commitmentStatus
    }

This is the right level for MVP because it exposes commitment status next to Build state without mutating Build state or enforcing policy.

## Enforcement boundary review

The design correctly rejects hidden enforcement.

The app/service layer should not automatically reject:

- Core redeem proof
- historical contribution
- history_bld reads
- available_bld reads
- Build state inspection

only because commitmentStatus is UNCOMMITTED.

This keeps commitmentStatus as a signal, not a global rule.

## appSubmitProof boundary review

The design correctly states that the first app integration should not change appSubmitProof behavior.

Proof submission should continue to process valid proofs according to existing registrar rules.

Commitment status should not become a hidden proof rejection rule.

## Registrar / watcher / proof boundary review

The design correctly keeps the following unchanged:

- watcher candidate shapes
- proof payload shapes
- registrar handlers
- registrar mutation rules

Commitment status is derived from existing Build state and optional current context.

It should not be added to proof payloads in this milestone.

## RPC boundary review

The design correctly states that the app/service helper must not:

- read process.env
- create a public client
- create a wallet client
- call real RPC directly
- import viem or ethers directly
- execute transactions

Current context should remain dependency-injected.

## External project policy review

The design correctly allows external X1 projects to use or ignore commitmentStatus.

External projects may display commitmentStatus, give COMMITTED Builds a bonus, or require COMMITTED status for their own feature.

The app/service layer should expose the signal cleanly, but not force external policy.

## Historical contribution review

The design correctly preserves the distinction:

    Build history is historical.
    Commitment status is current XNTD commitment context.

UNCOMMITTED status must not be confused with erased history.

This preserves historyBld, availableBld, originBld, Core redeem history, and Build history.

## Scope review

This review intentionally does not add future Build actor checks.

Build actor remains a separate future idea and is not part of this milestone.

The current milestone reviews only commitmentStatus app/service exposure.

## Boundary review

The design does not add:

- runtime code
- tests
- dependencies
- real RPC execution
- appSubmitProof behavior changes
- watcher behavior changes
- registrar behavior changes
- proof payload behavior changes
- commitment status enforcement
- external project policy
- UNCOMMITTED Build history erasure
- Forge requirements
- unlock mechanics
- BLD transfer/sale rule changes
- CLI commands

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 41 test files, 323 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Review decision

The XC Build commitment status app integration design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-commitment-status-app-integration-completion-checkpoint
