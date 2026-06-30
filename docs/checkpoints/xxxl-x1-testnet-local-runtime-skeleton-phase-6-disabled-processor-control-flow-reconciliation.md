# XXXL X1 Testnet Local Runtime Skeleton Phase 6 Disabled Processor Control Flow Reconciliation

Status: Docs-only reconciliation complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-6-disabled-processor-control-flow-reconciliation`

This checkpoint reconciles the current enabled processor control flow with the
local runtime skeleton implementation plan's Phase 6 expectation. It documents
the current disabled-plan behavior and the unresolved decision gate around
`Ok(())` versus an explicit disabled-route error.

It does not implement runtime code, change tests, deploy, upgrade, submit
transactions, spend SOL, or enable any live mint path.

## Scope

Phase 6 focuses on the currently enabled executable entrypoint path:

- `process_instruction`
- `process_consume_gateway_mint`
- local instruction/account validation
- disabled execution-plan construction
- return behavior
- no-mutation boundary
- CPI / `invoke_signed` / SPL Token `mint_to` reachability

The purpose is to document exactly where the processor stops today and what
must remain true before any future live-route or SPL-CPI enablement stage.

## Non-goals

Phase 6 does not:

- change runtime source code
- change tests
- add a disabled-route error
- change `Ok(())` behavior
- enable live gateway execution
- enable local state mutation from the enabled entrypoint
- mark Processed Event consumed
- credit Recipient Balance
- update Mint State or supply accounting
- enable SPL CPI
- enable `invoke_signed`
- enable SPL Token `mint_to`
- interpret bytes `194..208`
- resolve the `u128` amount design gap
- resolve the ignored Mollusk evidence gap
- resolve production guardian configuration
- claim production readiness
- claim final immutability

## Inputs Reviewed

Documents:

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-4-validation-error-model-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-5-stage-1-authorization-consumer-modeling.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/gateway/stage-1-5-runtime-mapping-notes.md`

Source and tests inspected without edits:

- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

## Current Enabled Processor Control Flow

The current enabled entrypoint path is validation and plan construction only.

Observed flow:

1. `process_instruction` receives the SVM instruction.
2. The instruction decoder checks strict instruction shape.
3. `CONSUME_GATEWAY_MINT` dispatches to `process_consume_gateway_mint`.
4. `process_consume_gateway_mint` obtains runtime sysvars needed for validation.
5. It calls the runtime execution-plan boundary.
6. The execution-plan boundary calls the validation / preparation boundary.
7. Account count, account flags, account owners, rent, local layouts, PDA
   relationships, route relationships, mint relationships, recipient
   relationships, amount range, and source-chain-weight relationships are
   checked.
8. A disabled execution plan is built.
9. `process_consume_gateway_mint` reaches the constructed disabled plan but
   does not execute it as a live mint operation.
10. The enabled path returns `Ok(())`.

The current enabled path does not execute the plan as a live mint operation.

## Validation Point

Validation happens before disabled-plan construction completes.

The validation boundary includes:

- exact account count and expected account contract
- no unexpected external signer
- expected writable / readonly account flags
- program-owned local account checks
- SPL Token-owned account checks
- rent-exemption checks
- local account discriminator / version / length checks
- token program ID check
- gateway mint authority PDA and bump checks
- SPL mint authority relationship check
- route relationship checks
- guardian-set relationship checks
- processed-event relationship and unconsumed-state checks
- recipient-balance relationship checks
- recipient token account owner/mint checks
- amount nonzero check
- amount `u64` SPL Token range check
- source-chain-weight relationship check

The source-chain-weight relationship check is a Phase 6 naming clarification of
the existing Gateway Config / decoded source-chain-weight account-level
relationship recorded in prior checkpoints. It is not a new runtime behavior
and does not change code or tests.

This is account-level validation, not Stage 1 message verification.

## Disabled Execution-plan Behavior

The current execution plan is a disabled plan.

It records intent-like planning information after validation, but the currently
enabled entrypoint path does not apply the plan as state mutation and does not
submit SPL CPI.

The plan must be understood as scaffold evidence that validation produced a
bounded, non-live plan.

It must not be described as:

- a successful gateway mint
- a successful XXXL token mint
- a processed event write
- a recipient balance credit
- a supply update
- SPL CPI readiness
- production authorization readiness

## Current Return Semantics

The current enabled path returns `Ok(())`.

For Phase 6 documentation purposes, that `Ok(())` means:

- the instruction decoded successfully
- the local account-level validation passed
- a disabled execution plan was constructed
- the processor returned without local mutation
- the processor returned without SPL CPI
- the processor returned without `invoke_signed`
- the processor returned without SPL Token `mint_to`

It does not mean:

- live gateway execution succeeded
- XXXL was minted
- Processed Event was marked consumed
- Recipient Balance was credited
- Mint State / supply accounting was updated
- Stage 1 authorization was verified inside SVM
- production route readiness was achieved

## No-mutation Boundary

The current enabled entrypoint path performs no account mutation.

Specifically, it does not:

- mark Processed Event consumed
- write consumed slot
- credit Recipient Balance
- update Mint State
- update supply accounting
- update local accounting as if mint succeeded
- mutate SPL mint state
- mutate recipient SPL token account

The no-mutation boundary must hold for both:

- validation failure paths
- current validation-success disabled-plan `Ok(())` paths

If a future Stage 1 authorization result is valid at model level but runtime
account-level validation fails, the runtime must not mark the Processed Event,
must not credit Recipient Balance, must not update supply accounting, and must
not execute SPL CPI.

## Replay / Processed-event No-write Boundary

Phase 5 established that runtime replay identity currently maps to
`canonicalEventKey` and Processed Event state.

Phase 6 preserves the current disabled behavior:

- Processed Event unconsumed-state validation exists
- Processed Event key / route / recipient relationships are validated
- the enabled path does not mark the Processed Event consumed
- the enabled path does not perform the paired mint / mark atomic transition

Future live behavior must preserve the atomicity invariant:

- no mint without processed mark
- no processed mark without mint
- no recipient accounting update without processed mark
- no supply accounting update without processed mark
- no state change if validation fails

## CPI / invoke_signed / mint_to Reachability

Dormant CPI helper code exists as future-boundary code.

Phase 6 preserves the distinction between:

- whole-codebase source-level existence of dormant CPI helpers
- reachability from the currently enabled executable entrypoint path

Current enabled path:

- does not call SPL CPI
- does not call `invoke_signed`
- does not call SPL Token `mint_to`

Dormant helper existence must not be interpreted as live route readiness.

Any future change that makes SPL CPI, `invoke_signed`, or SPL Token `mint_to`
reachable from the enabled entrypoint path requires a separate reviewed stage,
tests, blocker transition, and explicit safety evidence.

## Ok(()) versus Explicit Disabled-route Error Decision Gate

The implementation plan expected a disabled processor control-flow boundary
where the processor may decode and validate, but must stop before any live mint
path.

The current code stops through validation + disabled-plan construction +
`Ok(())`.

This is acceptable only as a locked scaffold behavior because:

- no local mutation occurs
- no Processed Event is marked consumed
- no Recipient Balance is credited
- no Mint State / supply accounting is updated
- no SPL CPI executes
- no `invoke_signed` executes
- no SPL Token `mint_to` executes
- all live-route and SPL-CPI blockers remain active

However, Phase 6 does not decide that `Ok(())` is the final disabled-route
behavior.

Before any implementation stage changes disabled processor control flow, the
project must explicitly choose one of these designs:

1. keep disabled-plan `Ok(())` as a scaffold-only no-op success, with tests and
   docs preventing it from being interpreted as live mint success; or
2. replace it with an explicit disabled-route error, making disabled behavior
   clearer to clients and tests.

This is a decision gate, not a resolved production rule.

The invariant is the same in both designs:

- disabled route must stop before mutation
- disabled route must stop before SPL CPI
- disabled route must stop before `invoke_signed`
- disabled route must stop before SPL Token `mint_to`
- disabled route must not remove any blocker
- disabled route must not imply live gateway success

## Comparison with Implementation Plan Phase 6 Expectation

Implementation plan Phase 6 expected:

- processor may decode and validate
- processor must stop before live mint path
- no account writes before disabled-route stop
- no SPL CPI before disabled-route stop
- no `invoke_signed` before disabled-route stop
- no SPL Token `mint_to` before disabled-route stop
- disabled route or equivalent safety behavior must be explicit

Current behavior matches the safety substance:

- decode happens
- validation happens
- disabled plan is built
- no mutation happens
- no SPL CPI happens
- no `invoke_signed` happens
- no SPL Token `mint_to` happens

Current behavior differs in return semantics:

- the enabled path currently returns `Ok(())`
- it does not currently return a distinct disabled-route error

That difference is acceptable for the current locked scaffold only if documented
as no-op disabled-plan return, not live route success.

## Stage 1 Authorization Consumer Continuity

Phase 6 does not change the Phase 5 Stage 1 boundary.

Stage 1 remains the deterministic authorization model.

Runtime remains a consumer / mapping layer for:

- account-level validation
- local replay-state relationship checks
- future account-write atomicity boundaries
- future CPI-boundary enforcement

Runtime still does not verify:

- Stage 1 message hash
- Stage 1 domain separator
- Ed25519 approvals
- guardian quorum
- full source-chain burn evidence
- `x1RecipientHash`
- `burnedAmount` as a separate runtime field
- `deadlineOrFinalityBlock`
- `messageNonce`

A relayer-submitted SVM instruction alone is not proof of Stage 1 authorization.

## sourceChainId Decision Path

Phase 5 recorded that `GatewayConfigAccountView::source_chain_id()` exists, but
the current processor does not compare it to a decoded instruction
`sourceChainId` field because no such named runtime field exists today.

Phase 6 preserves this as an explicit decision path.

Current disabled scaffold decision:

- absence of a named runtime `sourceChainId` instruction field is acceptable
  only because the route is non-live and the processor performs no mint
  mutation / SPL CPI.

Before any live-route or SPL-CPI enablement stage, the project must choose a
reviewed source-chain binding mechanism, for example:

- source chain remains a Gateway Config plus Stage 1 authorization-result
  property; or
- a future reviewed runtime authorization-result envelope adds a named
  `sourceChainId`; or
- a future instruction layout revision adds a named field with explicit
  canonical encoding and tests.

This must not be resolved silently through bytes `194..208`.

## messageNonce Runtime Semantics

`messageNonce` remains a Stage 1 field.

Current runtime replay identity remains:

- `canonicalEventKey`

The current runtime does not assign independent replay semantics to
`messageNonce`.

Any future runtime semantics for `messageNonce` require a separate reviewed
boundary and must not be silently mixed with Processed Event replay state.

## Future Test Obligations

Future Phase 6/7 or code-stage tests should prove:

- valid disabled scaffold input returns the chosen disabled behavior
- disabled `Ok(())`, if retained, performs no mutation
- explicit disabled-route error, if introduced later, performs no mutation
- validation failure performs no mutation
- Processed Event is not marked consumed in disabled path
- Recipient Balance is not credited in disabled path
- Mint State / supply accounting is not updated in disabled path
- SPL CPI is not reached in disabled path
- `invoke_signed` is not reached in disabled path
- SPL Token `mint_to` is not reached in disabled path
- dormant CPI helpers remain unreachable from enabled entrypoint while disabled
- Stage 1 rejection cases never reach runtime mint path
- source-chain ID binding decision receives explicit coverage before live route
- `messageNonce` is not treated as runtime replay state unless a future boundary
  explicitly changes that

The 27 Stage 1 invalid vectors remain Stage 1 rejection cases. They do not
prove SVM runtime behavior today, but future coverage should record that if
Stage 1 rejects a message, the runtime mint path is never reached.

## Deferred Gaps

Phase 6 preserves these gaps:

- `Ok(())` versus explicit disabled-route error remains a decision gate
- bytes `194..208` remain reserved, unparsed, and not zero-validated
- `u128` amount layout with `u64` SPL Token range remains a design gap
- source-chain ID runtime binding remains unresolved before live route
- `messageNonce` has no current runtime replay semantics
- dormant CPI helpers exist but must remain unreachable from enabled path
- 10 ignored Mollusk tests remain an evidence gap
- complete Mollusk/SVM coverage criteria remain undefined before upgrade or
  live readiness
- live processed-event mutation remains deferred
- live recipient-balance mutation remains deferred
- live supply update remains deferred
- SPL CPI execution remains deferred

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

Phase 6 made no runtime code changes.

Phase 6 changed no tests.

Phase 6 did not deploy.

Phase 6 did not upgrade.

Phase 6 did not submit transactions.

Phase 6 did not spend SOL.

Phase 6 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 6 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The existing X1 testnet scaffold remains locked, non-live, and unable to mint
through the currently enabled executable entrypoint path.

## Acceptance Criteria for Phase 6

Phase 6 is acceptable as a docs-only reconciliation stage if:

- current `Ok(())` behavior is described as disabled-plan no-op return, not live
  gateway mint success
- no mutation is attributed to the current enabled path
- no SPL CPI, `invoke_signed`, or SPL Token `mint_to` is attributed to the
  current enabled path
- dormant CPI helpers are distinguished from enabled-path reachability
- `Ok(())` versus explicit disabled-route error remains a decision gate
- Phase 4 and Phase 5 gates remain open
- all blockers remain active
- no production readiness or final immutability is claimed

## Audit Minor Notes Resolution

Phase 6 post-audit clarifications:

- `source-chain-weight relationship check` is a naming clarification for the
  already documented Gateway Config / decoded source-chain-weight
  account-level validation relationship. Phase 6 does not introduce new runtime
  behavior, code, or tests.
- `current-design-checkpoint.md` is a rolling aggregate / reference summary.
  The authoritative Phase 6 artifact is this standalone checkpoint.
- the `sourceChainId` decision path remains open. Phase 6 does not choose a
  preferred resolution. All listed options require a future reviewed boundary
  before live-route or SPL-CPI enablement.
- the enabled processor flow wording avoids relying on an unverified Solana
  Program Log claim. The important documented fact is that the disabled plan is
  constructed/reached and not executed as a live mint operation.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-7-replay-processed-event-local-model-reconciliation`

That stage should reconcile the replay / processed-event local model and future
atomic mark-with-result behavior. It must not enable live route execution, SPL
CPI, `invoke_signed`, or SPL Token `mint_to`.
