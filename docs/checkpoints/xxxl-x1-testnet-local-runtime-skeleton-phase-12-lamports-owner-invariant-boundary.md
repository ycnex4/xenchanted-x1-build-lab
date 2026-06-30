# XXXL X1 Testnet Local Runtime Skeleton Phase 12 Lamports / Owner Invariant Boundary

Status: Docs-only boundary checkpoint complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-12-lamports-owner-invariant-boundary`

Phase 12 does not change runtime source, tests, helper code, account layouts, or
instruction behavior.

## Purpose

Phase 12 defines the boundary for future stronger no-mutation invariant
coverage after Phase 11 documented the current helper scope.

Current helper:

- `result_and_unchanged_mutable_account_checks`

Current helper checks:

- mutable account data bytes only

Current helper does not check:

- lamports
- owner
- executable flag
- rent epoch

## Phase 12 Decision

Phase 12 keeps the current helper unchanged.

Lamports and owner invariants are worth adding before live-route readiness, but
not inside this docs-only phase.

Recommended future implementation phase:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-13-lamports-owner-invariant-tests`

Candidate Phase 13 target:

- extend or add a test helper that checks mutable account data bytes
- also check lamports
- also check owner
- keep the existing validation expectations unchanged
- keep runtime source read-only

## Explicit Non-goals

Phase 12 does not:

- modify `programs/xxxl-svm/src/**`
- modify `programs/xxxl-svm/tests/**`
- modify `programs/xxxl-svm/Cargo.toml`
- add dependencies
- run deploy or upgrade logic
- submit transactions
- spend SOL
- touch `.local-keys/**`
- touch keypair JSON files
- touch `.env`
- touch `target/deploy/**`
- touch `.so` artifacts

## Deferred Invariants

Lamports invariant:

- deferred to Phase 13 or later
- should verify no lamport movement on disabled scaffold and selected rejection
  paths

Owner invariant:

- deferred to Phase 13 or later
- should verify no account owner changes on disabled scaffold and selected
  rejection paths

Executable flag:

- not part of the immediate Phase 13 target
- may be considered later if there is a specific runtime reason to include it

Rent epoch:

- not part of the immediate Phase 13 target
- may be considered later if there is a specific runtime reason to include it

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

Phase 12 preserves these boundaries:

- Stage 1 remains the deterministic authorization model
- runtime remains a consumer / mapping layer
- relayer-submitted SVM instruction data alone is not Stage 1 authorization
- Processed Event state is not Stage 1 authorization
- runtime replay identity remains `canonicalEventKey`
- `messageNonce` has no current runtime replay semantics
- persistent Stage 1 processed-burn tracking belongs to off-chain watcher /
  orchestrator / authorization-service boundary
- `burnedAmount == xxxlMintAmount` remains a Stage 1 responsibility because the current SVM instruction does not carry `burnedAmount` separately

## Gate Preservation

Phase 12 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers remain unreachable from the enabled path
- 10 ignored Mollusk tests remain an evidence gap
- current `Ok(())` remains disabled-plan no-op return
- live atomicity remains unimplemented
- `sourceChainId` runtime handling remains unresolved before live-route or
  SPL-CPI enablement

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

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.
