# XXXL X1 Testnet Local Runtime Skeleton Phase 14 Disabled CPI Reachability Evidence

Status: Docs-only static reachability evidence checkpoint - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-14-disabled-cpi-reachability-evidence`

## Purpose

Phase 14 records source-level evidence that the currently enabled
`consume_gateway_mint` path does not reach:

- SPL CPI execution
- `invoke_signed`
- SPL Token `mint_to`

This is evidence for the current disabled runtime skeleton only.

It is not live-route readiness.

It is not production readiness.

It is not final immutability.

## Scope

Phase 14 is docs-only.

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

Current enabled entrypoint path:

1. `entrypoint!(process_instruction)`
2. `process_instruction`
3. `process_consume_gateway_mint`
4. `build_runtime_consume_gateway_mint_execution_plan_boundary`
5. log disabled execution-plan message
6. return `Ok(())`

Current enabled `process_consume_gateway_mint` does not call:

- `build_runtime_consume_gateway_mint_planning_composition_boundary`
- `build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary`
- `build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary`
- `guarded_mint_to_cpi_execution_gate_boundary`
- `mint_to_cpi_boundary`
- `invoke_signed`
- SPL Token `mint_to`

## Execution Plan Gate Evidence

Current execution-plan boundary rejects if either of these flags is true:

- `live_route_activation_enabled`
- `mint_to_invocation_from_process_instruction_enabled`

Current execution-plan construction sets both flags to false:

- `live_route_activation_enabled: false`
- `mint_to_invocation_from_process_instruction_enabled: false`

Therefore the enabled path can build a disabled execution plan, but it does not
activate live mint execution.

## Dormant CPI Helper Evidence

Dormant CPI code exists.

The codebase contains:

- SPL Token `mint_to` instruction construction
- `invoke_signed`
- `mint_to_cpi_boundary`

Those helpers are intentionally not reached from the currently enabled
`process_consume_gateway_mint` path.

The guarded CPI boundary also contains an explicit execution gate:

- `spl_mint_to_cpi_execution_enabled()` returns `false`
- if CPI execution is disabled, the guarded boundary returns
  `CpiBoundaryNotReady`
- the guarded boundary returns before calling `mint_to_cpi_boundary`

## What Phase 14 Proves

Phase 14 proves, at current source-reachability level only:

- enabled entrypoint path reaches validation and disabled execution-plan
  construction
- enabled entrypoint path returns `Ok(())` after the disabled plan is built
- enabled entrypoint path does not call SPL CPI execution
- enabled entrypoint path does not call `invoke_signed`
- enabled entrypoint path does not execute SPL Token `mint_to`
- dormant CPI helpers remain gated and unreachable from
  `process_consume_gateway_mint`

## What Phase 14 Does Not Prove

Phase 14 does not prove:

- live-route readiness
- SPL CPI readiness
- production readiness
- deploy readiness
- external review completion
- live atomicity
- rollback behavior after SPL CPI failure
- rollback behavior after local state mutation failure
- persistent Stage 1 processed-burn storage
- source-chain ID binding
- `messageNonce` runtime replay semantics
- invalid-vector e2e coverage
- removal of the 10 ignored Mollusk tests

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

Phase 14 preserves these boundaries:

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

Phase 14 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers remain unreachable from the enabled path
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

Phase 14 made no runtime code changes.

Phase 14 made no test code changes.

Phase 14 did not deploy.

Phase 14 did not upgrade.

Phase 14 did not submit transactions.

Phase 14 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-15-disabled-cpi-reachability-tests`

That future stage may add tests around the guarded disabled CPI boundary without
enabling live route execution.


## Phase 17 Audit Follow-up: Step 4 Call Graph Clarification

The documented enabled path remains correct, but Phase 17 audit noted that
step 4 can be expanded for precision.

Current enabled path summary:

1. `entrypoint!(process_instruction)`
2. `process_instruction`
3. `process_consume_gateway_mint`
4. `build_runtime_consume_gateway_mint_execution_plan_boundary`
5. disabled execution-plan log
6. `Ok(())`

Expanded step 4 call graph:

- `build_runtime_consume_gateway_mint_execution_plan_boundary`
  - calls `prepare_consume_gateway_mint_cpi_boundary`
  - calls `build_atomic_consume_gateway_mint_execution_plan`
  - rejects if live-route or mint-to invocation flags are enabled
  - returns the disabled execution plan

This does not change the safety claim.

The enabled path still does not call:

- `build_runtime_consume_gateway_mint_planning_composition_boundary`
- `build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary`
- `build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary`
- `guarded_mint_to_cpi_execution_gate_boundary`
- `mint_to_cpi_boundary`
- `invoke_signed`
- SPL Token `mint_to`
