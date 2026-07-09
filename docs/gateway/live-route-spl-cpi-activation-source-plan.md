# Live Route / SPL CPI Activation Source Plan

Status: COMPLETE
Package: live-route-spl-cpi-activation-source-plan
Approval: APPROVE_LIVE_ROUTE_SPL_CPI_ACTIVATION_SOURCE_PLAN_NO_ACTIVATION

This package is a planning and source-readiness package only.

It does not activate the live route.
It does not enable SPL CPI execution.
It does not deploy, upgrade, mutate RPC state, instantiate guardians, instantiate proof logs, emit local-only fixtures, or remove active blockers.

## Current Preserved State

- ExternalReviewIncomplete: REMOVED.
- PlaceholderProgramId: REMOVED.
- Program ID: BOUND.
- Program-ID-dependent PDA evidence: COMPLETE.
- LiveRouteDisabled: ACTIVE.
- SplCpiExecutionDisabled: ACTIVE.
- ProductionGuardianSetUnset: ACTIVE.
- ProductionProofLogUnset: ACTIVE.
- runtime_deployable: false.
- predeploy_gate: blocked.

## Future Boundary: LiveRouteDisabled Removal

`LiveRouteDisabled` may be removed only in a later separately approved source-change package.

That future package must explicitly prove:

1. The route transition is still non-deploying unless paired with a separate deployment package.
2. All source-level route guards are reviewed.
3. No hidden default enables the live route.
4. The live route path has explicit test coverage.
5. The runtime remains blocked if SPL CPI, guardian set, proof log, deployment readiness, or exact activation GO are missing.
6. The package name and approval text explicitly authorize `LiveRouteDisabled` removal.

This current package does not remove `LiveRouteDisabled`.

## Future Boundary: SplCpiExecutionDisabled Removal

`SplCpiExecutionDisabled` may be removed only in a later separately approved source-change package.

That future package must explicitly prove:

1. SPL CPI execution path is reviewed end-to-end.
2. Token program ID assumptions are fixed and reviewed.
3. Mint authority PDA signer seeds and bump are verified against the bound Program ID.
4. CPI account ordering, ownership, signer, and writable constraints are documented.
5. Failure modes are tested.
6. The runtime remains blocked if live route, guardian set, proof log, deployment readiness, or exact activation GO are missing.
7. The package name and approval text explicitly authorize `SplCpiExecutionDisabled` removal.

This current package does not remove `SplCpiExecutionDisabled`.

## Guardian Set Preconditions

Before `ProductionGuardianSetUnset` can be removed, a future package must define and review:

1. Production guardian public keys.
2. Guardian threshold.
3. Rotation policy.
4. Key custody policy.
5. Recovery policy.
6. Evidence that no private key material is committed or shared.
7. Tests or review evidence showing that quorum behavior matches the declared threshold.
8. Explicit approval for guardian set instantiation or source binding.

This current package does not instantiate a guardian set.

## Proof Log Preconditions

Before `ProductionProofLogUnset` can be removed, a future package must define and review:

1. Production proof log account model.
2. Proof log PDA or account derivation model.
3. Replay and duplicate-proof behavior.
4. Storage boundary and serialization model.
5. Failure behavior for missing, malformed, or already-consumed proof logs.
6. Evidence that proof-log state cannot be silently bypassed.
7. Explicit approval for proof log instantiation or source binding.

This current package does not instantiate a proof log.

## Final Deployment-Readiness Review Gates

Before any deployment or activation path, a final deployment-readiness package must re-gate the complete post-plan state.

Minimum required gates:

1. Program ID remains bound.
2. Program-ID-dependent PDA evidence remains complete.
3. Live route source state is explicitly reviewed.
4. SPL CPI execution source state is explicitly reviewed.
5. Production guardian set state is explicitly reviewed.
6. Production proof log state is explicitly reviewed.
7. Runtime deployment blockers are intentionally cleared only by approved packages.
8. Predeploy gate allows deploy only after all blockers are cleared.
9. No private key or keypair material is committed or shared.
10. No RPC mutation is performed without a separate exact authorization.
11. No deployment or upgrade is performed without a separate exact authorization.
12. No activation is performed without a separate exact activation GO.

## Non-Goals

This package does not:

- activate the runtime;
- deploy the program;
- upgrade the program;
- mutate RPC state;
- enable the live route;
- enable SPL CPI execution;
- instantiate guardians;
- instantiate proof logs;
- emit local-only fixtures;
- remove active blockers;
- perform cleanup;
- commit or share private key material.

## Next Required Decision

Request approval for:

activation-package-closure

That future package remains non-executing unless separately approved. Exact activation still requires a separate GO.
