# XXXL X1 Testnet Local Runtime Skeleton Phase 18 u128/u64 SPL Amount Boundary

Status: Docs-only amount boundary checkpoint - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-18-u128-u64-spl-amount-boundary`

## Purpose

Phase 18 records the current runtime amount boundary between:

- instruction-level `amount: u128`
- SPL Token CPI-level `amount: u64`

The current instruction layout parses `amount` as `u128`.

The current SPL Token mint path can only mint a `u64` amount.

Therefore, for the current SPL Token backend, runtime rejects any amount that
cannot be represented as `u64`.

This is not a live-route enablement.

This is not SPL-CPI enablement.

This is a boundary checkpoint only.

## Scope

Phase 18 is docs-only.

Allowed changes:

- checkpoint documentation only
- current checkpoint summary only

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

## Current Source Boundary

Current instruction args:

- `amount` is parsed as `u128`

Current preparation boundary:

- rejects `args.amount == 0`
- rejects `args.amount > u64::MAX as u128`
- only after those checks casts `args.amount as u64`

Current CPI boundary:

- `MintToCpiBoundary.amount` is `u64`

Current SPL Token instruction boundary:

- SPL Token `mint_to` uses `u64` amount

Therefore, current runtime behavior is:

- zero amount: invalid
- amount within `1..=u64::MAX`: accepted by amount range boundary
- amount greater than `u64::MAX`: invalid
- no truncation
- no wrapping
- no saturation
- no partial mint semantics

## Correct Statement

Correct:

- current instruction layout carries `amount` as `u128`
- current SPL Token CPI boundary carries `amount` as `u64`
- current runtime rejects `amount > u64::MAX`
- current runtime rejects `amount == 0`
- current runtime casts to `u64` only after range validation
- current SPL route cannot mint values greater than `u64::MAX`

Incorrect:

- runtime silently truncates `u128` to `u64`
- runtime wraps `u128` into `u64`
- runtime saturates large amounts to `u64::MAX`
- runtime can currently mint a SPL amount greater than `u64::MAX`
- `u128` instruction amount means current SPL backend supports full `u128`
  mint amounts
- Phase 18 enables SPL CPI
- Phase 18 enables live route

## Design Decision For Current SPL Backend

For the current SPL Token backend, the `u64` limit is treated as the active
runtime amount boundary.

Amounts greater than `u64::MAX` are invalid for the current route.

This is the correct current behavior because SPL Token `mint_to` cannot receive
a `u128` amount.

The current runtime must fail before CPI planning/execution rather than silently
altering the amount.

## Why u128 Still Exists In The Instruction Layout

The instruction layout currently uses `u128` because the gateway/protocol amount
model may need wider accounting than SPL Token CPI can execute directly.

However, a wider instruction field does not automatically make the current SPL
backend capable of minting wider amounts.

If a future backend or layout supports wider-than-u64 mint semantics, it must be
introduced explicitly.

That future design must not reuse the current SPL Token CPI boundary as if it
already supports `u128`.

## Relationship To Stage 1

Stage 1 remains responsible for deterministic authorization and amount
derivation.

Runtime remains responsible for refusing to execute an amount that cannot be
represented by the current target mint backend.

For the current SPL Token route:

- Stage 1 may model amount as a wider protocol value
- runtime still rejects values outside the SPL Token `u64` execution range
- no runtime truncation is allowed

## Existing Test Evidence

Existing unit coverage includes rejection of amount larger than the SPL Token
`u64` range.

The relevant behavior is already tested before Phase 18.

Phase 18 does not add or modify tests because it is docs-only.

## What Phase 18 Proves

Phase 18 proves documentation/source-boundary clarity only:

- `u128` instruction amount is wider than the current SPL Token CPI boundary
- current SPL Token route is bounded to `u64`
- current runtime rejects values outside the `u64` range
- current runtime does not claim full `u128` SPL mint support
- live-route enablement remains blocked

## What Phase 18 Does Not Prove

Phase 18 does not prove:

- live-route readiness
- SPL CPI readiness
- production readiness
- deploy readiness
- final immutability
- live atomicity
- rollback after local mutation failure
- rollback after SPL CPI failure
- sourceChainId final binding
- persistent Stage 1 processed-burn storage
- `messageNonce` runtime replay semantics
- invalid-vector e2e coverage
- removal of ignored Mollusk evidence gaps
- support for minting more than `u64::MAX` through SPL Token

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

## Gate Preservation

Phase 18 preserves these gates:

- live route remains disabled
- SPL CPI remains disabled
- direct-call local mutation boundary remains distinct from enabled path
- bytes `194..208` remain reserved, unparsed, and not zero-validated
- `sourceChainId` runtime handling remains unresolved
- current `Ok(())` remains disabled-plan no-op return
- live atomicity remains unimplemented
- amount greater than `u64::MAX` remains invalid for current SPL route
- zero amount remains invalid

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

Phase 18 made no runtime code changes.

Phase 18 made no test code changes.

Phase 18 did not deploy.

Phase 18 did not upgrade.

Phase 18 did not submit transactions.

Phase 18 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-19-live-atomicity-boundary`

That future stage should document the remaining live atomicity boundary before
any live-route or SPL-CPI enablement.
