# XXXL X1 Testnet Local Runtime Skeleton Phase 7 Replay Processed-event Local Model Reconciliation

Status: Docs-only reconciliation complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-7-replay-processed-event-local-model-reconciliation`

This checkpoint reconciles the replay / processed-event local model for the
current disabled XXXL SVM runtime skeleton and the future live atomicity model.

It documents how `canonicalEventKey`, Processed Event account state, local
replay checks, future processed mark, recipient accounting, mint/supply
accounting, and SPL CPI must relate.

It does not implement runtime code, change tests, deploy, upgrade, submit
transactions, spend SOL, or enable any live mint path.

## Scope

Phase 7 focuses on:

- Stage 1 replay boundary
- runtime Processed Event account boundary
- current disabled no-write replay behavior
- future live mark-with-result atomicity
- replay scenario classification
- `messageNonce` non-semantics at runtime
- source fork / finality / reorg considerations
- guardian set / coefficient / source-chain-weight replay considerations
- future test obligations

The current enabled path remains disabled and no-op:

- validates instruction and accounts
- builds a disabled execution plan
- returns `Ok(())`
- performs no local mutation
- does not mark Processed Event consumed
- does not credit Recipient Balance
- does not update Mint State / supply accounting
- does not execute SPL CPI
- does not call `invoke_signed`
- does not call SPL Token `mint_to`

## Non-goals

Phase 7 does not:

- change runtime source code
- change tests
- implement processed-event mutation
- implement recipient-balance mutation
- implement mint-state / supply accounting mutation
- implement SPL CPI
- implement `invoke_signed`
- implement SPL Token `mint_to`
- add Stage 1 verification inside SVM
- make Processed Event account presence proof of Stage 1 authorization
- make Processed Event unconsumed state proof of Stage 1 authorization
- make relayer-submitted instruction data proof of Stage 1 authorization
- assign runtime replay semantics to `messageNonce`
- resolve bytes `194..208`
- resolve the `u128` amount design gap
- resolve source-chain ID binding
- resolve production guardian configuration
- resolve complete Mollusk/SVM coverage criteria
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
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/gateway/stage-1-5-runtime-mapping-notes.md`
- `docs/gateway/generated/stage-1-gateway-vectors.json`

Source and tests inspected without edits:

- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/validation.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

`current-design-checkpoint.md` is a rolling aggregate / reference summary. The
authoritative Phase 7 artifact is this standalone checkpoint.

## Stage 1 Replay Boundary

Stage 1 owns source replay protection at deterministic authorization /
state-transition model level. In production, any persistent processed-burn
tracking belongs to the off-chain watcher / orchestrator / authorization
service boundary, not to the SVM runtime Processed Event account.

At Stage 1 model level, `canonicalEventKey` represents the canonical identity
of the source burn event accepted for the Ethereum-to-X1 gateway route. It is
derived from the source-side burn evidence and is used to prevent the same
source event from producing more than one authorized mint result.

Stage 1 is responsible for:

- validating canonical gateway message structure
- validating canonical field order and fixed-width encoding
- validating route binding
- validating source chain binding
- validating source token binding
- validating source sender / burn evidence binding
- validating source burn transaction hash
- validating source burn event index
- validating source block number / block hash evidence at model level
- validating source nonce
- calculating / validating `canonicalEventKey`
- calculating / validating `x1RecipientHash`
- validating burned amount and XXXL mint amount binding
- validating source-chain weight rule
- validating mint token binding
- validating guardian approvals
- validating guardian quorum
- checking Stage 1 processed burn replay state
- rejecting re-authorization of an already-authorized `canonicalEventKey`;
  any persistent Stage 1 processed-burn tracking belongs to the off-chain
  orchestrator / authorization-service boundary

Stage 1 rejection means the runtime mint path must not be reached. A
relayer-submitted SVM instruction alone is not proof of Stage 1 authorization.

## Runtime Processed Event Boundary

Runtime owns local Processed Event account relationship checks and future local
consumed-state atomicity.

At runtime level, `canonicalEventKey` is represented as the local replay
identity bound into the Processed Event account. It is not recomputed from full
source-chain evidence by the current SVM skeleton.

The current runtime Processed Event boundary checks:

- the Processed Event account has the expected local layout
- the Processed Event account is program-owned
- the Processed Event account has expected discriminator / version / length
- the Processed Event account is not already consumed
- the Processed Event `canonical_event_key` matches decoded
  `args.canonical_event_key`
- the Processed Event route matches decoded `args.route_id`
- the Processed Event recipient matches decoded `args.recipient`

The current runtime Processed Event boundary does not check:

- Stage 1 message hash
- Stage 1 domain separator
- Ed25519 approvals
- guardian quorum
- source burn transaction hash
- source burn event index
- source block number
- source block hash
- source nonce
- `burnedAmount`
- `x1RecipientHash`
- `deadlineOrFinalityBlock`
- `messageNonce`

Those remain Stage 1 model responsibilities or future reviewed boundary
decisions.

## Current Processed Event Account Relationship Model

The Processed Event account currently acts as a local relationship anchor for
the disabled runtime skeleton.

It binds:

- consumed / unconsumed state
- canonical event key
- route
- recipient

It participates in validation together with:

- decoded instruction fields
- Gateway Config route
- Recipient Balance owner
- recipient token account owner
- Mint State / Gateway Config / token-account mint relationships

Processed Event account presence alone is not authorization.

Processed Event unconsumed state alone is not authorization.

Processed Event relationship match alone is not authorization.

Future live mint must require both:

- valid Stage 1 authorization result
- runtime account-level validation

## Current Enabled No-write Replay Behavior

The current enabled path is disabled and no-op.

Current behavior:

- validates Processed Event account layout and relationships
- checks Processed Event is not already consumed
- checks Processed Event `canonical_event_key`
- checks Processed Event route
- checks Processed Event recipient
- builds a disabled execution plan
- returns `Ok(())`

Current behavior does not:

- mark Processed Event consumed
- write consumed slot
- credit Recipient Balance
- update Mint State / supply accounting
- execute SPL CPI
- execute `invoke_signed`
- execute SPL Token `mint_to`

Therefore current Processed Event validation is replay-shape validation only,
not live replay-state transition.

## Future Live Atomicity Model

Future live behavior must preserve mark-with-result atomicity.

Required invariant:

- no mint without processed mark
- no processed mark without mint
- no recipient accounting update without processed mark
- no supply accounting update without processed mark
- no state change if validation fails

Meaning of each invariant:

1. No mint without processed mark:
   A future live mint must not update SPL token state or local supply state
   unless the corresponding Processed Event is marked consumed in the same
   atomic runtime transition.

2. No processed mark without mint:
   The runtime must not mark an event consumed if the corresponding mint /
   recipient accounting / supply accounting result does not happen.

3. No recipient accounting update without processed mark:
   Recipient Balance must not be credited unless the Processed Event is also
   marked consumed in the same atomic transition.

4. No supply accounting update without processed mark:
   Mint State / local supply accounting must not increase unless the Processed
   Event is also marked consumed in the same atomic transition.

5. No state change if validation fails:
   If instruction decoding, account validation, Stage 1 authorization
   consumption, local replay validation, recipient validation, amount validation,
   mint validation, or CPI preparation fails, the runtime must leave all
   relevant state unchanged.

## Future Failure Ordering

If validation fails before future mutation:

- Processed Event must remain unmodified
- Recipient Balance must remain unmodified
- Mint State / supply accounting must remain unmodified
- SPL mint must remain unmodified
- recipient token account must remain unmodified

If future SPL CPI / mint execution fails after validation:

- Processed Event must not remain marked consumed without mint
- Recipient Balance must not remain credited without mint
- Mint State / supply accounting must not remain increased without mint
- on Solana / SVM, transaction-level atomicity must ensure the paired local
  writes do not remain if the instruction fails; an equivalent strategy is only
  relevant if a future reviewed architecture leaves the single-transaction
  model

If future processed mark would fail:

- mint must not occur
- recipient accounting must not occur
- supply accounting must not occur
- no partial success may be observable

The future implementation must avoid designs where processed mark and mint are
split across separate independently successful transactions.

## Replay Scenario Matrix

| Scenario | Stage 1 model responsibility | Runtime local responsibility | Current status |
| --- | --- | --- | --- |
| same `canonicalEventKey` submitted twice | reject source replay after first successful Stage 1 authorization | reject already-consumed Processed Event in future live path | current validation can reject consumed Processed Event, but enabled path does not mark consumed |
| same source burn with different recipient | canonical event key / recipient hash binding must prevent alternate recipient authorization | Processed Event recipient must match decoded recipient / recipient accounts | current relationship validation exists; no live mark |
| same source burn with different amount | Stage 1 amount binding must reject unauthorized amount and preserve `burnedAmount == xxxlMintAmount` for the route | runtime checks decoded amount is nonzero and within the `u64` SPL range, but does not see `burnedAmount` separately | current amount range validation exists; Stage 1 amount binding remains external |
| same source burn with different route | Stage 1 route binding must reject wrong route | runtime checks route against Gateway Config and Processed Event | current relationship validation exists |
| same `messageNonce`, different `canonicalEventKey` | Stage 1 defines nonce meaning; nonce is not runtime replay identity today | runtime must not use `messageNonce` for replay today | no current runtime nonce semantics |
| different `messageNonce`, same `canonicalEventKey` | Stage 1 replay protection still keyed by canonical event identity | runtime replay identity remains `canonicalEventKey` | no current runtime nonce semantics |
| guardian set rotation | Stage 1 must define guardian set/version validity for signed authorization | runtime checks guardian set ID relationship only | guardian set version replay remains deferred |
| coefficient/source-chain-weight version change | Stage 1 must define source-chain-weight / conversion policy | runtime checks current Gateway Config source-chain weight relationship | coefficient/version replay remains deferred |
| pause/unpause | future governance/safety model must define replay effect | runtime must not allow pause/unpause to bypass replay or mint rules | deferred |
| upgrade | future upgrade model must preserve replay state and consumed events | runtime must not allow upgrade to clear or bypass replay | deferred |
| source fork / reorg / finality correction | Stage 1/finality model must decide accepted finality evidence | runtime sees only accepted canonical event key and local accounts | deferred |

## messageNonce Non-semantics

`messageNonce` is one of the 19 Stage 1 vector fields.

Current runtime does not expose `messageNonce` as a named instruction field.

Current runtime does not use `messageNonce` as replay identity.

Current runtime replay identity remains:

- `canonicalEventKey`

`messageNonce` must not be silently mixed with Processed Event replay state.

Any future runtime semantics for `messageNonce` require a separate reviewed
boundary, explicit encoding, tests, and replay analysis.

## source fork / finality / reorg considerations

Source fork, reorg, and finality correction handling remain outside the current
runtime skeleton.

Stage 1 / watcher / guardian / finality policy must define when a source burn
event is final enough to authorize.

Runtime currently does not verify:

- source block number
- source block hash
- finality depth
- fork choice
- reorg correction
- deadline/finality expiry

Runtime currently consumes only the local representation of an already accepted
authorization result. Future runtime live behavior must not compensate for weak
source finality by weakening local replay rules.

If a source event was accepted and later considered invalid due to finality
correction, the recovery / incident model must be a separate reviewed design
topic. It must not silently clear local Processed Event state or rewrite supply
without an explicit protocol-level decision.

## Guardian Set / Coefficient / Source-chain-weight Replay Considerations

Guardian set, coefficient, and source-chain-weight changes can create replay
ambiguity if not bound explicitly.

Current status:

- runtime checks decoded guardian set ID against Gateway Config and Guardian
  Set account
- runtime checks decoded source-chain weight against Gateway Config
- runtime does not verify guardian signatures
- runtime does not verify guardian set version semantics
- runtime does not verify conversion coefficient version semantics
- runtime does not carry source-chain ID as a named instruction field

Deferred replay questions:

- whether guardian set version must be part of future runtime authorization
  result envelope
- whether source-chain-weight or conversion coefficient version must be part of
  future replay identity
- whether a message signed under an old guardian set can be consumed after
  rotation
- whether a message signed under an old weight/coefficient can be consumed
  after policy change
- how pause/unpause affects already signed but unconsumed messages
- how upgrades preserve replay state and prevent replay bypass

These questions must be resolved before live-route or SPL-CPI enablement.

## Failure Behavior Matrix

| Failure case | Required behavior |
| --- | --- |
| invalid instruction decode | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| invalid account owner/layout/rent | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| wrong route | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| wrong mint | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| wrong recipient | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| amount zero / overflow | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| source-chain weight mismatch | no Processed Event mark, no recipient credit, no supply update, no SPL CPI |
| Processed Event already consumed | no second mint, no second recipient credit, no supply update, no SPL CPI |
| Stage 1 authorization absent | runtime mint path must not be reached in future live design |
| Stage 1 authorization invalid | runtime mint path must not be reached in future live design |
| future SPL CPI failure | no processed mark or local accounting may remain without mint |
| future processed mark failure | no mint or local accounting may occur |
| future recipient accounting failure | no processed mark or mint may remain without recipient accounting if recipient accounting is part of the atomic result |
| future supply accounting failure | no processed mark or mint may remain without supply accounting if supply accounting is part of the atomic result |

Whether Recipient Balance and Mint State / supply accounting are part of the
same atomic live result must be decided before any live-route / SPL-CPI
implementation stage. Phase 8 should track the coverage obligation, but the
implementation boundary must make the final atomic result definition explicit
before code enables live mutation.

## Current Coverage and Deferred Replay Gaps

Current evidence from prior phases includes:

- Processed Event consumed-state validation exists
- Processed Event canonical event key relationship validation exists
- Processed Event route relationship validation exists
- Processed Event recipient relationship validation exists
- selected no-mutation validation failure checks exist
- enabled path remains disabled and no-op
- enabled path does not reach SPL CPI, `invoke_signed`, or SPL Token `mint_to`

Deferred gaps remain:

- enabled path does not mark Processed Event consumed
- live processed-event mutation not implemented
- live recipient accounting not implemented
- live supply accounting not implemented
- live SPL CPI not implemented
- complete atomic mark-with-result tests not defined
- coefficient/version replay unresolved
- guardian set version replay unresolved
- pause/unpause replay unresolved
- upgrade replay unresolved
- source fork / reorg replay unresolved
- `messageNonce` runtime semantics unresolved by design and currently absent
- source-chain ID runtime binding unresolved
- 10 ignored Mollusk tests remain an evidence gap
- complete Mollusk/SVM coverage criteria remain undefined before upgrade or
  live readiness

## Future Test Obligations

Future Phase 7/code-stage tests should prove:

- disabled `Ok(())` performs no Processed Event mutation
- disabled `Ok(())` performs no Recipient Balance mutation
- disabled `Ok(())` performs no Mint State / supply mutation
- disabled `Ok(())` performs no SPL mint or recipient token account mutation
- validation failure performs no mutation across all local accounts
- consumed Processed Event prevents replay in future live path
- canonical event key mismatch is rejected
- route mismatch is rejected
- recipient mismatch is rejected
- same canonical event key cannot mint twice
- same source burn cannot mint to a different recipient through runtime replay
  bypass
- same source burn cannot mint a different amount through runtime replay bypass
- `messageNonce` is not treated as runtime replay identity unless future
  boundary changes it
- source-chain ID binding decision receives explicit tests before live route
- guardian set rotation replay rules receive explicit tests before live route
- coefficient/source-chain-weight version replay rules receive explicit tests
  before live route
- pause/unpause replay rules receive explicit tests before live route
- upgrade replay preservation receives explicit tests before live route
- source fork/finality correction assumptions are documented before live route
- SPL CPI failure cannot leave processed mark / local accounting without mint
- processed mark failure cannot leave mint / local accounting without mark
- recipient accounting failure cannot leave partial successful result if it is
  part of the atomic live transition
- supply accounting failure cannot leave partial successful result if it is part
  of the atomic live transition
- Stage 1 invalid vectors remain Stage 1 rejection cases and never reach a
  runtime mint path in future end-to-end tests

## Phase 4 / Phase 5 / Phase 6 Gate Preservation

Phase 7 preserves these gates:

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

Phase 7 made no runtime code changes.

Phase 7 changed no tests.

Phase 7 did not deploy.

Phase 7 did not upgrade.

Phase 7 did not submit transactions.

Phase 7 did not spend SOL.

Phase 7 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 7 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The existing X1 testnet scaffold remains locked, non-live, and unable to mint
through the currently enabled executable entrypoint path.

## Acceptance Criteria for Phase 7

Phase 7 is acceptable as a docs-only reconciliation stage if:

- Stage 1 source replay responsibility remains separate from runtime local
  Processed Event responsibility
- Processed Event account presence is not treated as authorization
- Processed Event unconsumed state is not treated as authorization
- relayer-submitted SVM instruction data is not treated as authorization
- runtime replay identity remains `canonicalEventKey`
- `messageNonce` is not assigned runtime replay semantics
- current enabled path remains no-write
- future live atomicity requires mark-with-result behavior
- all replay variants are classified as current, future, or deferred
- Phase 4, Phase 5, and Phase 6 gates remain open
- all blockers remain active
- no production readiness or final immutability is claimed

## Audit Minor Notes Resolution

Phase 7 post-audit clarifications:

- Stage 1 replay wording now distinguishes deterministic authorization /
  state-transition modeling from persistent production tracking. Persistent
  processed-burn tracking belongs to the off-chain watcher / orchestrator /
  authorization-service boundary, not to the SVM runtime Processed Event
  account.
- Stage 1 should be read as rejecting re-authorization of an already-authorized
  `canonicalEventKey`; Processed Event state remains runtime-local replay state
  and is not Stage 1 authorization.
- Solana / SVM transaction-level atomicity is the expected rollback mechanism
  for failed instructions. Any "equivalent strategy" is only relevant if a
  future reviewed architecture leaves the single-transaction model.
- Recipient Balance and Mint State / supply accounting atomic-result membership
  must be decided before any live-route / SPL-CPI implementation stage. Phase 8
  should track coverage obligations, but should not silently decide the live
  mutation model.
- Runtime currently checks decoded `amount` for nonzero / `u64` SPL range only.
  `burnedAmount == xxxlMintAmount` remains a Stage 1 responsibility because the
  current SVM instruction does not carry `burnedAmount` as a separate runtime
  field.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-8-local-tests-coverage-checkpoint`

That stage should reconcile local test obligations and coverage criteria for the
disabled runtime skeleton before any code-stage or live-route/SPL-CPI enablement
work. It must not enable live route execution, SPL CPI, `invoke_signed`, or SPL
Token `mint_to`.
