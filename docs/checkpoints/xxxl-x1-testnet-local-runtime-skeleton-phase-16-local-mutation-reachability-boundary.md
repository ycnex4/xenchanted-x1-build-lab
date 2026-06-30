# XXXL X1 Testnet Local Runtime Skeleton Phase 16 Local Mutation Reachability Boundary

Status: Docs-only reachability boundary checkpoint - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-16-local-mutation-reachability-boundary`

## Purpose

Phase 16 records a strict boundary correction:

The local mutation boundary is not described as globally disabled.

The local mutation boundary exists and can mutate local state if it is called
directly.

The current enabled `process_instruction` path does not call that local mutation
boundary.

This checkpoint prevents an unsafe or misleading statement such as:

- "local mutation boundary is disabled"

The correct statement is:

- "local mutation boundary is not reached from the currently enabled
  `process_instruction` path"

## Scope

Phase 16 is docs-only.

Allowed changes:

- checkpoint documentation only

Not modified:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/**`
- `programs/xxxl-svm/Cargo.toml`
- workspace-level Cargo files
- deployment scripts
- upgrade scripts
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- `.local-keys/**`
- keypair JSON files
- `.env`
- `target/deploy/**`
- `.so` artifacts

## Enabled Entrypoint Path

Current enabled entrypoint path remains:

1. `entrypoint!(process_instruction)`
2. `process_instruction`
3. `process_consume_gateway_mint`
4. `build_runtime_consume_gateway_mint_execution_plan_boundary`
5. disabled execution-plan log
6. `Ok(())`

The enabled `process_consume_gateway_mint` path builds the runtime execution
plan boundary and returns `Ok(())`.

It does not call:

- `build_runtime_consume_gateway_mint_planning_composition_boundary`
- `build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary`
- `apply_atomic_state_mutation_composition_boundary`
- `apply_processed_event_mutation_boundary`
- `apply_recipient_balance_mutation_boundary`
- `mark_processed_event_consumed`
- `credit_recipient_balance`

## Local Mutation Boundary Exists

The runtime source contains a public local mutation composition boundary:

- `build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary`

That boundary:

- builds planning composition
- checks disabled live-route / invoke-signed flags
- borrows Processed Event account data mutably
- borrows Recipient Balance account data mutably
- calls `apply_atomic_state_mutation_composition_boundary`

Therefore, if called directly with valid accounts and args, it is a local-state
mutation boundary.

It must not be described as disabled.

## Local Mutation Composition Mutates State

The local mutation composition can mutate:

- Processed Event account data
- Recipient Balance account data

The mutation composition calls:

- `apply_processed_event_mutation_boundary`
- `apply_recipient_balance_mutation_boundary`

Those paths ultimately perform:

- processed-event consumed marking
- recipient balance crediting

Therefore, direct-call local mutation behavior is not the same as enabled
`process_instruction` behavior.

## Correct Boundary Statement

Correct:

- local mutation boundary exists
- local mutation boundary is reachable by direct public function call
- local mutation boundary can mutate local state if directly called
- local mutation boundary is not reached from the currently enabled
  `process_instruction` path

Incorrect:

- local mutation boundary is disabled
- local mutation boundary cannot mutate
- local mutation boundary proves live-route atomicity
- local mutation boundary proves production readiness

## Relationship To Phase 13

Phase 13 strengthened enabled-path no-mutation checks in Mollusk.

Those checks cover the current enabled `process_instruction` path.

They do not prove that direct-call local mutation boundary cannot mutate.

That is expected.

The boundary is:

- enabled path no-mutation evidence
- not global no-mutation of all public helper functions

## Relationship To Phase 14 And Phase 15

Phase 14 recorded source-level CPI reachability evidence.

Phase 15 added tests around the guarded disabled CPI boundary.

Neither phase claimed that the direct-call local mutation boundary is disabled.

Phase 16 preserves that distinction explicitly.

## What Phase 16 Proves

Phase 16 proves at documentation / source-boundary level only:

- the local mutation boundary exists
- the local mutation boundary can mutate local state if directly called
- the local mutation boundary is not part of the currently enabled
  `process_instruction` path
- current enabled path remains validation plus disabled execution-plan
  construction only
- Phase 13 no-mutation evidence should be interpreted as enabled-path evidence,
  not as a global claim about all public helper functions

## What Phase 16 Does Not Prove

Phase 16 does not prove:

- live-route readiness
- local mutation safety under production execution
- live atomicity
- rollback behavior after SPL CPI failure
- rollback behavior after local state mutation failure
- persistent Stage 1 processed-burn storage
- source-chain ID binding
- `messageNonce` runtime replay semantics
- invalid-vector e2e coverage
- removal of the 10 ignored Mollusk tests
- external review completion
- production readiness
- deploy readiness
- final immutability

## Current Disabled Semantics Preserved

Current `Ok(())` still means:

- validation succeeded
- disabled execution plan was built
- no live gateway success
- no XXXL mint success
- no Processed Event consumption
- no Recipient Balance credit
- no Mint State / supply accounting mutation
- no SPL CPI
- no `invoke_signed`
- no SPL Token `mint_to`

## Stage 1 / Runtime Boundary Preserved

Phase 16 preserves these boundaries:

- Stage 1 remains the deterministic authorization model
- runtime remains a consumer / mapping layer
- relayer-submitted SVM instruction data alone is not Stage 1 authorization
- Processed Event state is not Stage 1 authorization
- runtime replay identity remains `canonicalEventKey`
- `messageNonce` has no current runtime replay semantics
- persistent Stage 1 processed-burn tracking belongs to off-chain watcher /
  orchestrator / authorization-service boundary
- `burnedAmount == xxxlMintAmount` remains a Stage 1 responsibility because the
  current SVM instruction does not carry `burnedAmount` separately

## Gate Preservation

Phase 16 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers remain gated
- direct-call local mutation boundary remains distinct from enabled path
- 10 ignored Mollusk tests remain an evidence gap
- current `Ok(())` remains disabled-plan no-op return
- live atomicity remains unimplemented
- `sourceChainId` runtime handling remains unresolved before live-route or
  SPL-CPI enablement
- source-chain ID must not be resolved silently through bytes `194..208`

## Safety Blocker Preservation

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Phase 16 made no runtime code changes.

Phase 16 made no test code changes.

Phase 16 did not deploy.

Phase 16 did not upgrade.

Phase 16 did not submit transactions.

Phase 16 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-17-source-chain-id-runtime-boundary`

That future stage should clarify the unresolved `sourceChainId` runtime boundary
before any live-route or SPL-CPI enablement.
