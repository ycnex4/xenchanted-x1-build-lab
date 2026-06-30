# XXXL X1 Testnet Local Runtime Skeleton Phase 9 Local Tests Implementation Boundary

Status: Docs-only implementation-boundary checkpoint complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-9-local-tests-implementation-boundary`

This checkpoint defines the first narrow boundary for a future local test
implementation stage after the Phase 8 coverage checkpoint.

It does not implement tests, change runtime code, deploy, upgrade, submit
transactions, spend SOL, or enable any live mint path.

## Scope

Phase 9 defines:

- which files a future narrow test implementation stage may edit
- which files must remain read-only
- which Phase 8 obligations should be selected first
- which Phase 8 obligations remain deferred
- which commands should be used to verify a future test-only stage
- which safety gates must remain active before, during, and after test work

Phase 9 is a boundary document. It is not the test implementation itself.

## Non-goals

Phase 9 does not:

- edit Rust source files
- edit Rust tests
- add tests
- un-ignore ignored tests
- build SBF artifacts
- touch `target/deploy/**`
- touch `.so` artifacts
- run deploy or upgrade
- submit transactions
- spend SOL
- enable live gateway execution
- enable Processed Event mutation
- enable Recipient Balance mutation
- enable Mint State / supply accounting mutation
- enable SPL CPI
- enable `invoke_signed`
- enable SPL Token `mint_to`
- resolve bytes `194..208`
- resolve the `u128` amount design gap
- resolve source-chain ID binding
- assign runtime replay semantics to `messageNonce`
- make Processed Event state proof of Stage 1 authorization
- claim production readiness
- claim final immutability

## Inputs Reviewed

Documents:

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-4-validation-error-model-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-5-stage-1-authorization-consumer-modeling.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-6-disabled-processor-control-flow-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-7-replay-processed-event-local-model-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-8-local-tests-coverage-checkpoint.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/gateway/stage-1-5-runtime-mapping-notes.md`
- `docs/gateway/generated/stage-1-gateway-vectors.json`

Source and tests considered inspect-only for this checkpoint:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/**`

`current-design-checkpoint.md` is a rolling aggregate / reference summary. The
authoritative Phase 9 artifact is this standalone checkpoint.

## Boundary Decision

The first future local test implementation stage must be narrow.

It must not attempt to cover every Phase 8 obligation at once.

It should choose a small subset of current disabled-skeleton obligations and
prove those without changing runtime source.

The recommended first implementation scope is:

- disabled `Ok(())` no-mutation coverage
- selected validation-failure no-mutation coverage
- no SPL CPI / `invoke_signed` / SPL Token `mint_to` reachability evidence from
  the currently enabled path

The recommended first implementation scope must remain limited to the disabled
local runtime skeleton.

It must not include live-route behavior.

It must not include SPL CPI execution.

It must not include Processed Event mutation.

It must not include Recipient Balance mutation.

It must not include Mint State / supply accounting mutation.

## Future Test Implementation Allowed Files

A future test implementation stage may edit only:

- `programs/xxxl-svm/tests/**`
- docs checkpoint files that document that future stage

A future test implementation stage may add a new test helper file only if it is
under:

- `programs/xxxl-svm/tests/**`

A future test implementation stage may update:

- `docs/checkpoints/current-design-checkpoint.md`
- a new standalone checkpoint for that future stage

## Future Test Implementation Read-only Files

A future narrow test implementation stage must keep these read-only:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/Cargo.toml`, unless a separate reviewed boundary explicitly
  allows dependency or feature changes
- workspace-level `Cargo.toml`, unless a separate reviewed boundary explicitly
  allows dependency or feature changes
- deployment scripts
- upgrade scripts
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- `.local-keys/**`
- keypair JSON files
- `.env`
- `target/deploy/**`
- `.so` artifacts

A future narrow test implementation stage must not make dormant CPI helpers
reachable from the enabled `process_instruction` path.

## Candidate Phase 10 Scope

Recommended next implementation stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-10-disabled-no-mutation-tests`

Candidate Phase 10 should target only:

1. valid disabled scaffold input leaves mutable local accounts unchanged;
2. selected validation failures leave mutable local accounts unchanged;
3. current enabled path remains unable to reach SPL CPI, `invoke_signed`, or SPL
   Token `mint_to`.

Candidate Phase 10 should not target:

- all Phase 8 gaps
- live processed-event marking
- live recipient accounting
- live supply accounting
- SPL CPI success
- SPL CPI failure rollback
- double-mint prevention under live mutation
- guardian set rotation replay
- coefficient / source-chain-weight version replay
- pause / unpause replay
- upgrade replay preservation
- source fork / reorg / finality simulation

Those remain future or deferred obligations.

## Candidate Phase 10 Test Ideas

Candidate Phase 10 may add or refine tests for:

- valid disabled `CONSUME_GATEWAY_MINT` returns the documented disabled no-op
  result and leaves Processed Event account data unchanged
- valid disabled `CONSUME_GATEWAY_MINT` leaves Recipient Balance account data
  unchanged
- valid disabled `CONSUME_GATEWAY_MINT` leaves Mint State account data unchanged
- valid disabled `CONSUME_GATEWAY_MINT` leaves SPL mint account data unchanged
- valid disabled `CONSUME_GATEWAY_MINT` leaves recipient SPL token account data
  unchanged
- canonical event key mismatch leaves all mutable local accounts unchanged
- recipient mismatch leaves all mutable local accounts unchanged
- route mismatch leaves all mutable local accounts unchanged
- zero amount or amount-range failure leaves all mutable local accounts unchanged
- SPL CPI helper code remains unreachable from the enabled entrypoint path

Candidate Phase 10 should prefer explicit before/after account-data snapshots
over assumptions based only on return value.

## Candidate Phase 10 Non-goals

Candidate Phase 10 must not:

- modify runtime source
- modify account layouts
- modify instruction encoding
- modify validation behavior
- change error codes
- enable disabled route execution
- mark Processed Event consumed
- credit Recipient Balance
- update Mint State / supply accounting
- execute SPL CPI
- call `invoke_signed`
- call SPL Token `mint_to`
- un-ignore tests that require `cargo build-sbf` and `target/deploy/xxxl_svm.so`
  unless a separate SBF harness boundary is created first
- generate or commit SBF artifacts
- touch deployment files
- touch upgrade files
- make production readiness claims

## Out-of-scope Phase 8 Obligations for First Test Stage

The following Phase 8 obligations should remain deferred after the first narrow
test implementation stage unless a later boundary explicitly includes them:

- exact wrong route account index coverage
- exact wrong guardian set account index coverage
- exact wrong mint state account index Mollusk coverage
- complete full account substitution matrix
- complete full instruction negative matrix
- Stage 1 invalid vector to runtime no-mint-path e2e coverage
- `messageNonce` independence coverage beyond current documentation
- source-chain ID binding coverage
- guardian set rotation replay
- coefficient / source-chain-weight version replay
- pause / unpause replay
- upgrade replay preservation
- source fork / reorg / finality simulation
- live processed mark + mint atomicity
- future SPL CPI failure rollback
- future processed mark failure rollback
- future recipient accounting failure behavior
- future supply accounting failure behavior
- production guardian-set readiness
- production proof-log readiness
- external live-readiness review

## Current Disabled Semantics to Preserve

The future first test stage must preserve these current semantics:

- current `Ok(())` means validation + disabled-plan no-op return only
- current `Ok(())` is not live gateway success
- current `Ok(())` is not XXXL mint success
- current `Ok(())` is not Processed Event consumption
- current `Ok(())` is not Recipient Balance credit
- current `Ok(())` is not supply update
- current enabled path performs no local mutation
- current enabled path performs no SPL CPI
- current enabled path does not call `invoke_signed`
- current enabled path does not call SPL Token `mint_to`

## Stage 1 / Runtime Boundary to Preserve

The future first test stage must not blur these boundaries:

- Stage 1 remains the deterministic authorization model
- runtime remains a consumer / mapping layer for account-level checks and future
  atomicity boundaries
- relayer-submitted SVM instruction data alone is not Stage 1 authorization
- Processed Event state is not Stage 1 authorization
- Processed Event unconsumed state is not Stage 1 authorization
- Processed Event relationship match is not Stage 1 authorization
- runtime replay identity remains `canonicalEventKey`
- `messageNonce` has no current runtime replay semantics
- persistent Stage 1 processed-burn tracking belongs to the off-chain watcher /
  orchestrator / authorization-service boundary
- `burnedAmount == xxxlMintAmount` remains a Stage 1 responsibility because the
  current SVM instruction does not carry `burnedAmount` separately

## Future Verification Commands

A future test implementation stage should record exact commands used.

Recommended baseline checks should be recorded as plain command output:

    git status --short --untracked-files=all
    git diff --check
    git status --short --untracked-files=all -- programs/xxxl-svm/src
    git status --short --untracked-files=all -- programs/xxxl-svm/tests
    cargo test -p xxxl-svm --test mollusk_consume_gateway_mint

If the workspace uses a different exact package/test command at that stage, the
stage must record the exact successful command and output summary.

A future test implementation stage must also record whether ignored Mollusk
tests remain ignored and why.

## Required Future Evidence

A future test implementation stage should provide:

- `git diff --check` output
- source guard proving `programs/xxxl-svm/src/**` was not modified
- test guard showing only intended `programs/xxxl-svm/tests/**` files changed
- exact test command output
- count of passing tests
- count of ignored tests
- explicit statement that no live route was enabled
- explicit statement that no SPL CPI was enabled
- explicit statement that `invoke_signed` and SPL Token `mint_to` remain
  unreachable from the enabled path
- explicit statement that no deploy/upgrade/transaction/SOL-spend action
  occurred

## Gate Preservation

Phase 9 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- no code may read, interpret, or validate bytes `194..208` as named production
  fields until a separate boundary decision
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers contain `mint_to` / `invoke_signed` source-level code, but
  the enabled `process_instruction` path remains documented as not reaching SPL
  CPI, `invoke_signed`, or `mint_to`
- 10 ignored Mollusk tests remain an evidence gap
- complete formal Mollusk/SVM go/no-go coverage criteria remain undefined
  before any on-chain upgrade or live readiness
- Stage 1 remains the deterministic authorization model
- runtime remains only a consumer / mapping layer for account-level checks and
  future atomicity boundaries
- `sourceChainId` runtime handling remains an explicit unresolved decision path
  before live-route or SPL-CPI enablement
- source-chain ID must not be resolved silently through bytes `194..208`
- `messageNonce` has no current runtime replay semantics
- runtime replay identity remains `canonicalEventKey`
- current `Ok(())` behavior remains validation + disabled-plan no-op return
- current `Ok(())` is not live gateway success
- current `Ok(())` is not XXXL mint success
- current `Ok(())` is not Processed Event consumption
- current `Ok(())` is not Recipient Balance credit
- current `Ok(())` is not supply update
- `Ok(())` versus explicit disabled-route error remains a future decision gate
- future live atomicity remains unimplemented

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

Phase 9 made no runtime code changes.

Phase 9 changed no tests.

Phase 9 did not deploy.

Phase 9 did not upgrade.

Phase 9 did not submit transactions.

Phase 9 did not spend SOL.

Phase 9 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 9 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The existing X1 testnet scaffold remains locked, non-live, and unable to mint
through the currently enabled executable entrypoint path.

## Acceptance Criteria for Phase 9

Phase 9 is acceptable as a docs-only implementation-boundary checkpoint if:

- it does not implement tests
- it does not edit runtime code
- it explicitly limits the future first test implementation stage to
  `programs/xxxl-svm/tests/**`
- it keeps `programs/xxxl-svm/src/**` read-only for that future stage
- it chooses a narrow first implementation subset
- it does not attempt to cover all Phase 8 gaps at once
- it preserves current disabled `Ok(())` no-op semantics
- it preserves no SPL CPI / `invoke_signed` / `mint_to` reachability
- it preserves Stage 1 / runtime boundaries
- it preserves Processed Event non-authorization boundaries
- it keeps live atomicity unimplemented
- all blockers remain active
- no production readiness or final immutability is claimed

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-10-disabled-no-mutation-tests`

That stage may implement the first narrow test subset under
`programs/xxxl-svm/tests/**`, but must not modify runtime source, deployment
files, upgrade files, live route execution, SPL CPI, `invoke_signed`, or SPL
Token `mint_to`.
