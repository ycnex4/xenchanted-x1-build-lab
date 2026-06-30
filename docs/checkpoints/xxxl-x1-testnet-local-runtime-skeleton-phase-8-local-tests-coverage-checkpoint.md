# XXXL X1 Testnet Local Runtime Skeleton Phase 8 Local Tests Coverage Checkpoint

Status: Docs-only coverage checkpoint complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-8-local-tests-coverage-checkpoint`

This checkpoint reconciles local test obligations and coverage criteria for the
disabled XXXL SVM runtime skeleton before any future code-stage, live-route, or
SPL-CPI enablement work.

It does not implement tests, change runtime code, deploy, upgrade, submit
transactions, spend SOL, or enable any live mint path.

## Scope

Phase 8 classifies:

- current test / evidence coverage recorded by earlier phases
- disabled runtime skeleton invariants that require coverage
- gaps that must remain visible before code-stage work
- future live-only tests that must not be confused with current disabled tests
- minimum coverage obligations before any live-route or SPL-CPI enablement

Phase 8 is a checkpoint, not an implementation stage.

## Non-goals

Phase 8 does not:

- edit Rust source files
- edit Rust tests
- add Mollusk tests
- un-ignore existing ignored tests
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
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/gateway/stage-1-5-runtime-mapping-notes.md`
- `docs/gateway/generated/stage-1-gateway-vectors.json`

Source and tests considered inspect-only for this checkpoint:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

`current-design-checkpoint.md` is a rolling aggregate / reference summary. The
authoritative Phase 8 artifact is this standalone checkpoint.

## Prior Coverage Baseline

Prior phases recorded these coverage facts:

- the local runtime skeleton has a Mollusk integration test file for
  `CONSUME_GATEWAY_MINT`
- Phase 2 recorded 54 Mollusk tests total
- Phase 2 recorded 44 non-ignored tests and 10 ignored tests
- Phase 2 recorded a successful non-ignored Mollusk run with 44 passing and 10
  ignored
- Phase 3 recorded instruction decode coverage for discriminator, version,
  length, extra bytes, meta count, selected encoded indices, zero amount, route
  mismatch, canonical event key mismatch, and recipient mismatch
- Phase 4 recorded validation / error model coverage and classified remaining
  gaps
- Phase 5 recorded Stage 1 authorization consumer boundaries and future coverage
  obligations
- Phase 6 recorded disabled processor control-flow coverage obligations
- Phase 7 recorded replay / Processed Event atomicity coverage obligations

Phase 8 does not re-run or expand tests. It reconciles coverage obligations
before any later implementation stage.

## Current Disabled Runtime Invariants Requiring Coverage

The current disabled runtime skeleton must preserve these invariants:

1. Valid disabled scaffold input may decode and validate.
2. Valid disabled scaffold input may build a disabled execution plan.
3. Current `Ok(())` means validation + disabled-plan no-op return only.
4. Current `Ok(())` is not live gateway success.
5. Current `Ok(())` is not XXXL mint success.
6. Current `Ok(())` is not Processed Event consumption.
7. Current `Ok(())` is not Recipient Balance credit.
8. Current `Ok(())` is not Mint State / supply update.
9. Current enabled path performs no local mutation.
10. Current enabled path performs no SPL CPI.
11. Current enabled path does not call `invoke_signed`.
12. Current enabled path does not call SPL Token `mint_to`.
13. Dormant CPI helper existence is not enabled-path reachability.
14. Processed Event presence alone is not authorization.
15. Processed Event unconsumed state alone is not authorization.
16. Relayer-submitted SVM instruction data alone is not Stage 1 authorization.
17. Runtime replay identity remains `canonicalEventKey`.
18. `messageNonce` has no current runtime replay semantics.
19. Bytes `194..208` remain reserved / unparsed / not zero-validated.
20. Source-chain ID binding remains unresolved before live route.

## Coverage Classification

Phase 8 uses four labels:

- **covered-current**: prior phases recorded current coverage or evidence
- **partial-current**: some coverage exists, but exact scenario coverage is
  incomplete
- **deferred-gap**: explicitly open coverage/design gap before later stages
- **future-live-only**: cannot be fully tested until a future live mutation /
  SPL-CPI design exists

## Instruction Decode Coverage

| Coverage item | Classification | Notes |
| --- | --- | --- |
| wrong discriminator | covered-current | Prior phases recorded non-ignored Mollusk and lower-level coverage |
| wrong version | covered-current | Prior phases recorded non-ignored Mollusk and lower-level coverage |
| wrong instruction length / truncated instruction | covered-current | Prior phases recorded non-ignored Mollusk and lower-level coverage |
| extra bytes beyond exact instruction length | covered-current | Prior phases recorded non-ignored Mollusk and lower-level coverage |
| wrong encoded account meta count | covered-current | Prior phases recorded non-ignored Mollusk and lower-level coverage |
| wrong encoded processed event account index | covered-current | Prior phases recorded non-ignored Mollusk coverage |
| wrong encoded recipient balance account index | covered-current | Prior phases recorded non-ignored Mollusk coverage |
| zero amount | covered-current | Prior phases recorded non-ignored Mollusk and lower-level handler coverage |
| wrong encoded route account index | deferred-gap | Exact non-ignored Mollusk and exact route-index unit coverage remain open |
| wrong encoded guardian set account index | deferred-gap | Exact non-ignored Mollusk and exact guardian-index unit coverage remain open |
| wrong encoded mint state account index | partial-current | Lower-level unit coverage recorded, but exact non-ignored Mollusk coverage remains open |
| amount overflow / `u64` boundary | partial-current | Lower-level handler coverage recorded, but exact non-ignored Mollusk coverage remains open |
| bytes `194..208` semantics | deferred-gap | Reserved / unparsed / not zero-validated; no production semantics decided |

## Account / Validation Coverage

| Coverage item | Classification | Notes |
| --- | --- | --- |
| account count | covered-current | Current account contract expects 9 accounts |
| account writable / readonly flags | covered-current | Prior phases recorded account-contract validation |
| no unexpected external signer | covered-current | Prior phases recorded account-contract validation |
| local account owner checks | covered-current | Prior phases recorded owner validation |
| SPL Token program key check | covered-current | Prior phases recorded token program validation |
| rent exemption checks | covered-current | Prior phases recorded rent validation |
| local discriminator / version / exact length | covered-current | Prior phases recorded layout validation |
| gateway mint authority PDA / bump | covered-current | Prior phases recorded PDA validation |
| SPL mint authority relationship | covered-current | Prior phases recorded relationship validation |
| route relationship | covered-current | Gateway Config / Processed Event route relationships recorded |
| guardian set relationship | partial-current | Lower-level coverage recorded; exact non-ignored Mollusk mismatch remains open |
| mint relationship | partial-current | Several mint relationships recorded; explicit mint-state mint mismatch coverage remains open |
| processed event canonical key relationship | covered-current | Prior phases recorded key mismatch coverage |
| processed event route relationship | covered-current | Prior phases recorded route mismatch coverage |
| processed event recipient relationship | covered-current | Prior phases recorded recipient mismatch coverage |
| recipient balance owner / mint relationship | covered-current | Prior phases recorded recipient relationship validation |
| recipient token account owner / mint | covered-current | Prior phases recorded recipient token-account validation |
| source-chain weight relationship | partial-current | Existing Gateway Config / decoded source-chain-weight relationship recorded; exact Mollusk mismatch coverage remains open |
| source-chain ID binding | deferred-gap | No named instruction field today; decision path remains open |
| `messageNonce` runtime semantics | deferred-gap | No runtime semantics today; replay identity remains `canonicalEventKey` |

## Disabled Processor Control-flow Coverage

| Coverage item | Classification | Notes |
| --- | --- | --- |
| valid scaffold decode + validate + disabled-plan construction | covered-current | Prior phases recorded current enabled control flow |
| `Ok(())` documented as disabled-plan no-op return | covered-current | Phase 6 recorded return semantics |
| `Ok(())` not live gateway success | covered-current | Phase 6/7 recorded boundary |
| no Processed Event mark in enabled path | covered-current | Phase 6/7 recorded no-write boundary |
| no Recipient Balance credit in enabled path | covered-current | Phase 6/7 recorded no-write boundary |
| no Mint State / supply update in enabled path | covered-current | Phase 6/7 recorded no-write boundary |
| no SPL CPI in enabled path | covered-current | Phase 1/2/4/6/7 recorded enabled-path boundary |
| no `invoke_signed` in enabled path | covered-current | Prior phases recorded enabled-path boundary |
| no SPL Token `mint_to` in enabled path | covered-current | Prior phases recorded enabled-path boundary |
| static / unit proof that dormant CPI helpers remain unreachable | deferred-gap | Future coverage should make enabled-path reachability explicit |
| explicit disabled-route error behavior | deferred-gap | `Ok(())` versus explicit disabled-route error remains a future decision gate |

## Replay / Processed Event Coverage

| Coverage item | Classification | Notes |
| --- | --- | --- |
| Processed Event consumed-state validation exists | covered-current | Prior phases recorded current validation |
| consumed Processed Event rejection | partial-current | Validation exists; future live double-mint behavior remains unimplemented |
| canonical event key mismatch | covered-current | Prior phases recorded coverage |
| route mismatch | covered-current | Prior phases recorded partial route mismatch coverage |
| recipient mismatch | covered-current | Prior phases recorded coverage |
| Processed Event presence is not authorization | covered-current | Phase 7 recorded boundary |
| Processed Event unconsumed state is not authorization | covered-current | Phase 7 recorded boundary |
| Processed Event relationship match is not authorization | covered-current | Phase 7 recorded boundary |
| same `canonicalEventKey` cannot mint twice | future-live-only | Requires future live mutation / mark-with-result model |
| same source burn cannot mint different recipient | future-live-only | Requires Stage 1 + runtime end-to-end live model |
| same source burn cannot mint different amount | future-live-only | Stage 1 amount binding external; runtime sees decoded amount only |
| same source burn cannot mint different route | future-live-only | Requires future live end-to-end model |
| `messageNonce` independence | deferred-gap | Needs explicit future coverage that runtime replay identity remains `canonicalEventKey` |
| source fork / reorg replay | deferred-gap | Finality / incident model unresolved |
| guardian set version replay | deferred-gap | Rotation semantics unresolved |
| coefficient / source-chain-weight version replay | deferred-gap | Version semantics unresolved |
| pause / unpause replay | deferred-gap | Future governance/safety model unresolved |
| upgrade replay preservation | deferred-gap | Future upgrade model must preserve replay state |

## Stage 1 Authorization Boundary Coverage

Stage 1 remains the deterministic authorization model.

Runtime remains a consumer / mapping layer for account-level checks and future
atomicity boundaries.

Coverage obligations:

| Coverage item | Classification | Notes |
| --- | --- | --- |
| relayer-submitted SVM instruction alone is not authorization | covered-current | Phase 5/6/7 recorded boundary |
| Stage 1 rejection never reaches runtime mint path | deferred-gap | Future end-to-end coverage should connect Stage 1 invalid vectors to runtime no-mint path |
| 27 Stage 1 invalid vectors | deferred-gap | They prove Stage 1 rejection behavior, not SVM runtime behavior today |
| Stage 1 message hash / domain separator | external to runtime | Stage 1 responsibility |
| Ed25519 approval / guardian quorum | external to runtime | Stage 1 responsibility |
| `burnedAmount == xxxlMintAmount` | external to runtime | Stage 1 responsibility; current runtime does not carry `burnedAmount` separately |
| `x1RecipientHash` derivation | external to runtime | Stage 1 responsibility; current runtime carries raw recipient bytes |
| persistent Stage 1 processed-burn tracking | external to runtime | Off-chain watcher / orchestrator / authorization-service boundary |

## Live Atomicity Coverage

Future live atomicity is not implemented today.

Required future invariant:

- no mint without processed mark
- no processed mark without mint
- no recipient accounting update without processed mark
- no supply accounting update without processed mark
- no state change if validation fails

Coverage classification:

| Coverage item | Classification | Notes |
| --- | --- | --- |
| validation failure leaves state unchanged | partial-current | Selected no-mutation checks recorded; complete matrix remains open |
| disabled `Ok(())` leaves state unchanged | partial-current | Boundary recorded; future tests should assert all mutable accounts unchanged |
| future SPL CPI failure cannot leave processed mark | future-live-only | Requires SPL-CPI implementation design |
| future processed mark failure cannot leave mint | future-live-only | Requires live mutation design |
| future recipient accounting failure atomicity | future-live-only | Atomic-result membership must be decided before live implementation |
| future supply accounting failure atomicity | future-live-only | Atomic-result membership must be decided before live implementation |
| single-transaction Solana / SVM rollback assumption | deferred-gap | Expected platform rollback; future design must stay within or explicitly review alternatives |

## Ignored Mollusk Tests

Prior phases recorded:

- 10 ignored Mollusk tests
- shared visible reason: `requires cargo build-sbf and target/deploy/xxxl_svm.so`

Phase 8 keeps this as an evidence gap.

Before any on-chain upgrade or live readiness claim, the project must choose one
of these paths:

1. create a reviewed SBF artifact / harness stage;
2. convert ignored tests to a non-ignored Mollusk-compatible harness;
3. replace them with equivalent coverage and record the rationale.

No ignored-test resolution is implemented by Phase 8.

## Minimum Future Test Obligations Before Code-stage

Before any source/test implementation stage is accepted, the project should
define which of these are targeted by that stage:

- exact wrong route account index coverage
- exact wrong guardian set account index coverage
- exact wrong mint state account index Mollusk coverage
- amount overflow / `u64` boundary Mollusk coverage
- guardian set ID mismatch Mollusk coverage
- source-chain weight mismatch Mollusk coverage
- explicit mint-state mint mismatch coverage
- disabled `Ok(())` no-mutation coverage across all mutable accounts
- validation failure no-mutation coverage across all mutable accounts
- static or unit reachability proof for no SPL CPI / `invoke_signed` /
  SPL Token `mint_to` from enabled path
- Processed Event presence / unconsumed state / relationship match not being
  treated as authorization
- `messageNonce` not being runtime replay identity
- `sourceChainId` unresolved binding not being silently encoded into bytes
  `194..208`
- Stage 1 invalid vectors mapped to future no-runtime-mint-path e2e coverage
  obligations

## Minimum Future Test Obligations Before Live-route or SPL-CPI Enablement

Before any live-route or SPL-CPI enablement stage, coverage must include:

- full account substitution matrix
- full instruction decode negative matrix
- full no-partial-write validation failure matrix
- future processed mark + mint atomicity tests
- future SPL CPI failure rollback tests
- future processed mark failure rollback tests
- future recipient accounting failure behavior
- future supply accounting failure behavior
- double-submit `canonicalEventKey` replay tests
- same source burn / different recipient tests
- same source burn / different amount tests
- same source burn / different route tests
- guardian set rotation replay tests
- coefficient/source-chain-weight version replay tests
- pause/unpause replay tests
- upgrade replay preservation tests
- source fork / reorg / finality assumption tests or documented simulation
- explicit production guardian-set readiness evidence
- explicit production proof-log readiness evidence
- external review checkpoint

## Gate Preservation

Phase 8 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- no code may read, interpret, or validate bytes `194..208` as named production
  fields until a separate boundary decision
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers contain `mint_to` / `invoke_signed` source-level code, but
  the enabled `process_instruction` path remains documented as not reaching SPL
  CPI, `invoke_signed`, or `mint_to`
- 10 ignored Mollusk tests remain an evidence gap
- complete Mollusk/SVM coverage criteria remain undefined before any on-chain
  upgrade or live readiness
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

Phase 8 made no runtime code changes.

Phase 8 changed no tests.

Phase 8 did not deploy.

Phase 8 did not upgrade.

Phase 8 did not submit transactions.

Phase 8 did not spend SOL.

Phase 8 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 8 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The existing X1 testnet scaffold remains locked, non-live, and unable to mint
through the currently enabled executable entrypoint path.

## Acceptance Criteria for Phase 8

Phase 8 is acceptable as a docs-only coverage checkpoint if:

- it does not implement tests
- it does not edit runtime code
- it classifies current evidence versus gaps clearly
- it keeps ignored Mollusk tests visible as an evidence gap
- it does not turn future live tests into current claims
- it preserves Stage 1 / runtime boundaries
- it preserves Processed Event non-authorization boundaries
- it preserves current disabled `Ok(())` no-op semantics
- it preserves no SPL CPI / `invoke_signed` / `mint_to` reachability
- it keeps live atomicity unimplemented
- all blockers remain active
- no production readiness or final immutability is claimed

## Audit Minor Notes Resolution

Phase 8 post-audit clarifications:

- `/ state-transition model` was removed from Phase 8 Stage 1 wording. Stage 1
  remains the deterministic authorization model.
- persistent processed-burn tracking remains external to runtime and belongs to
  the off-chain watcher / orchestrator / authorization-service boundary.
- Phase 9 should explicitly scope any allowed test edits to
  `programs/xxxl-svm/tests/**`; `programs/xxxl-svm/src/**` should remain
  read-only unless a later separate implementation boundary explicitly changes
  that.
- Phase 9 should choose a narrow subset of Phase 8 test obligations rather than
  attempting to cover every listed gap in one stage.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-9-local-tests-implementation-boundary`

That stage should define the first narrow test-implementation boundary for the
disabled local runtime skeleton. It may allow test edits only if explicitly
scoped, but must not enable live route execution, SPL CPI, `invoke_signed`, or
SPL Token `mint_to`.
