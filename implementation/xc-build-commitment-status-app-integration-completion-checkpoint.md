# XC Build commitment status app integration completion checkpoint

This document closes the XC Build commitment status app integration design milestone.

This checkpoint is documentation-only.

No runtime code is changed in this checkpoint.

No dependencies are changed in this checkpoint.

No real RPC is executed in this checkpoint.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Completed chain

The XC Build commitment status app integration design milestone completed the full progression:

1. commitment status app integration design
2. commitment status app integration design review
3. merge to main

## Current main status

Latest completed main milestone:

    main -> 5ca42a3 Merge branch 'xc-build-commitment-status-app-integration-design-review'

Final validation after merge:

- npm run typecheck passed
- npm test passed: 41 test files, 323 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

## Completed documents

Design document:

    implementation/xc-build-commitment-status-app-integration-design.md

Review document:

    implementation/xc-build-commitment-status-app-integration-design-review-notes.md

Checkpoint update:

    docs/checkpoints/current-design-checkpoint.md

## Accepted app integration direction

The accepted app/service direction is:

    expose commitmentStatus as optional current XNTD commitment context

not:

    enforce commitmentStatus globally
    reject historical proofs because commitmentStatus is UNCOMMITTED

## Accepted future helper direction

Accepted future helper direction:

    appGetBuildView()

Accepted future view direction:

    AppBuildView {
      build
      commitmentStatus
    }

## Boundary preserved

The design preserves these boundaries:

- appSubmitProof behavior remains unchanged
- watcher behavior remains unchanged
- registrar behavior remains unchanged
- proof payload behavior remains unchanged
- app/service view layer does not call real RPC directly
- current context remains dependency-injected
- external X1 project usage remains optional
- Forge participation remains out of scope for MVP commitment status

## Historical safety

UNCOMMITTED Build must not mean:

- invalid Build
- invalid historical contribution
- erased history
- rejected Core redeem proof
- automatic loss of historyBld
- automatic loss of availableBld

Commitment status means:

    current XNTD commitment signal

not:

    Build validity

## Build actor scope

Build actor is not part of this milestone.

Build actor remains a separate future idea.

This milestone reviews only commitmentStatus app/service exposure.

## Recommended next milestone

Recommended next implementation milestone:

    xc-build-commitment-status-app-view

Purpose:

- add appGetBuildView()
- return BuildState plus commitmentStatus
- keep helper read-only and non-mutating
- avoid appSubmitProof changes
- avoid watcher changes
- avoid registrar changes
- avoid proof payload changes
- avoid real RPC
- avoid Forge requirements
- avoid Build actor scope

## Decision

The XC Build commitment status app integration design milestone is complete.

Next step may be runtime implementation:

    xc-build-commitment-status-app-view
