# XXXL X1 Testnet Local Runtime Skeleton Phase 5 Stage 1 Authorization Consumer Modeling

Status: Docs-only modeling complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-5-stage-1-authorization-consumer-modeling`

This checkpoint models how the local X1 runtime skeleton consumes the Stage 1
gateway authorization result. It does not implement runtime code, change tests,
deploy, upgrade, submit transactions, spend SOL, or enable any live mint path.

## Scope

Phase 5 reconciles the Stage 1 deterministic gateway authorization model with
the current disabled XXXL SVM runtime skeleton.

Required framing:

Stage 1 establishes the deterministic authorization model. Runtime maps that
result to account-level validation, replay protection, and atomicity
boundaries.

Stage 1 proves gateway authorization properties at model level; the runtime
must preserve those properties at account-write and CPI-boundary level.

## Non-goals

Phase 5 does not:

- implement a Stage 1 proof verifier in the SVM runtime
- add source-chain message decoding to the SVM instruction
- add guardian signature verification to the SVM runtime
- change account layouts or instruction layouts
- interpret instruction bytes `194..208` as named production fields
- mutate processed-event, recipient-balance, or mint-state accounts from the
  enabled entrypoint
- enable live gateway execution
- enable SPL CPI, `invoke_signed`, or SPL Token `mint_to`
- remove or weaken any deployment blocker
- claim production authorization readiness

## Inputs Reviewed

Documents:

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-4-validation-error-model-reconciliation.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `docs/xxxl/xxxl-stage-1-gateway-authorization-consumer.md`
- `docs/gateway/stage-1-5-runtime-mapping-notes.md`
- `docs/gateway/generated/stage-1-gateway-vectors.json`

Source and tests inspected without edits:

- `programs/xxxl-svm/src/account_contract.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/deployment_status.rs`
- `programs/xxxl-svm/src/entrypoint.rs`
- `programs/xxxl-svm/src/error.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/lib.rs`
- `programs/xxxl-svm/src/pda.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/program_id_status.rs`
- `programs/xxxl-svm/src/safety_invariants.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/validation.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

## Stage 1 Authorization Boundary

The Stage 1 authorization result consumed by XXXL is the successful output of
the deterministic gateway model. The explicit Stage 1 consumer contract records
the minimal model-level values:

- `authorizationOk`
- `authorized`
- `markedProcessed`
- `canonicalEventKey`
- `amount`

The XXXL consumer must reject that contract unless:

- `authorizationOk` is true
- `authorized` is true
- `markedProcessed` is true
- `amount` is greater than zero
- `canonicalEventKey` has not already been consumed locally

Stage 1 owns:

- canonical gateway message validation
- canonical field order and fixed-width encoding
- domain separator binding
- message hash binding
- route binding
- source chain binding
- source token binding
- source sender / burn evidence binding
- source block / finality evidence binding at model level
- canonical event key calculation
- X1 recipient hash calculation
- burned amount and XXXL mint amount binding
- source-chain weight rule for the Stage 1 route
- mint token binding
- Ed25519 approval verification
- guardian quorum verification
- processed burn replay protection in the Stage 1 model

The runtime consumer must not duplicate Stage 1's full message-verification
role in an ad hoc way. It must consume an approved Stage 1 result and preserve
that result through account-level validation, replay-state validation, and
future atomic account writes.

## Runtime Consumer Boundary

The current SVM skeleton consumes only a subset of the Stage 1 result as named
instruction fields:

- `route_id`
- `guardian_set_id`
- `mint_id`
- `canonical_event_key`
- `recipient`
- `amount`
- `source_chain_weight_bps`

The current SVM instruction does not carry these Stage 1 fields as named
runtime fields:

- `sourceChainId`
- `sourceToken`
- `sourceSender`
- `sourceBurnTxHash`
- `sourceBurnEventIndex`
- `sourceBlockNumber`
- `sourceBlockHash`
- `sourceNonce`
- `burnedAmount`
- `deadlineOrFinalityBlock`
- `messageNonce`

Bytes `194..208` remain reserved, unparsed, and not zero-validated. They must
not be treated as hidden Stage 1 fields without a separate boundary.

The runtime currently maps the consumed subset into account-level checks:

- route ID against Gateway Config and Processed Event
- guardian set ID against Gateway Config and Guardian Set
- mint ID against Mint State, Gateway Config, Recipient Balance, SPL mint, and
  recipient token account
- canonical event key against Processed Event
- recipient against Processed Event, Recipient Balance, and recipient token
  account owner
- amount against nonzero and SPL Token `u64` range checks
- source-chain weight against Gateway Config

## Field Responsibility Matrix

| Stage 1 field | Stage 1 responsibility | Current runtime representation | Current runtime responsibility |
| --- | --- | --- | --- |
| `messageType` | validates the expected Stage 1 gateway message type | not a named instruction field | not currently re-checked by runtime; belongs to the consumed Stage 1 authorization result |
| `schemaVersion` | validates the expected Stage 1 message schema version | not a named instruction field; SVM instruction version is separate | runtime checks only its own SVM instruction version, not Stage 1 `schemaVersion` |
| `routeId` | validates exact route in signed message | `args.route_id` | re-checks Gateway Config route and Processed Event route |
| `sourceChainId` | validates accepted source chain | not a named instruction field; Gateway Config has a stored source-chain ID view | not currently re-checked against instruction data |
| `sourceToken` | validates accepted source token | not a named instruction field | not currently re-checked by runtime |
| `sourceSender` | binds source burn sender evidence | not a named instruction field | not currently re-checked by runtime |
| `sourceBurnTxHash` | binds burn transaction evidence | not a named instruction field | only indirectly represented through canonical event key |
| `sourceBurnEventIndex` | binds burn event index evidence | not a named instruction field | only indirectly represented through canonical event key |
| `sourceBlockNumber` | binds finality evidence | not a named instruction field | not currently re-checked by runtime |
| `sourceBlockHash` | binds finality evidence | not a named instruction field | not currently re-checked by runtime |
| `sourceNonce` | binds source-side message uniqueness | not a named instruction field | not currently re-checked by runtime |
| `canonicalEventKey` | derives event identity from source evidence and rejects source replay in Stage 1 model | `args.canonical_event_key` and Processed Event account field | re-checks Processed Event key and unconsumed state |
| `x1RecipientHash` | hashes and validates 32-byte X1 recipient in signed message | `args.recipient` carries raw recipient bytes, not the hash | re-checks recipient account relationships, not the hash derivation |
| `burnedAmount` | validates burn amount greater than zero and Stage 1 amount rule | not a named instruction field | not independently re-checked by runtime |
| `sourceChainWeightBps` | validates Stage 1 full-weight route rule | `args.source_chain_weight_bps` | re-checks Gateway Config source-chain weight |
| `xxxlMintAmount` | binds mint amount to burned amount in Stage 1 | `args.amount` | re-checks nonzero and `u64` SPL Token range |
| `mintToken` | validates Stage 1 mint token identity | `args.mint_id` / SPL mint Pubkey | re-checks Mint State, Gateway Config, Recipient Balance, SPL mint, and token account mint relationships |
| `deadlineOrFinalityBlock` | included in signed payload; baseline model does not enforce expiry | not a named instruction field | not currently re-checked by runtime |
| `messageNonce` | included in signed payload as a mandatory field | not a named instruction field | not currently re-checked by runtime |

## Runtime Re-check Matrix

The runtime must not blindly trust instruction fields, account metas, account
data, or relayer-provided accounts. The current skeleton re-checks:

| Boundary | Current re-check | Current status |
| --- | --- | --- |
| account count | exactly 9 accounts | implemented |
| account flags | expected writable/readonly and no external signer | implemented |
| local account owners | program-owned local accounts owned by current program | implemented |
| SPL-owned accounts | SPL mint and recipient token account owned by SPL Token program | implemented |
| rent | program-owned and SPL accounts rent-exempt | implemented |
| local layout | discriminator, version, exact length | implemented |
| token program | account 8 key equals `spl_token::id()` | implemented |
| mint authority PDA | Mint State PDA field, PDA derivation, bump, and SPL mint authority | implemented |
| route | Gateway Config and Processed Event route match decoded route | implemented |
| guardian set | Gateway Config and Guardian Set IDs match decoded guardian set ID | implemented |
| processed event replay | Processed Event not already consumed | implemented as validation; enabled entrypoint does not mark consumed |
| processed event identity | canonical event key, route, and recipient match decoded fields | implemented |
| recipient balance | owner and mint match decoded recipient and mint | implemented |
| recipient token account | initialized token account with expected owner and mint | implemented |
| amount | nonzero and not larger than `u64::MAX` | implemented |
| source-chain weight | Gateway Config weight matches decoded weight | implemented |

The runtime currently does not re-check:

- Stage 1 message hash
- Stage 1 domain separator
- Ed25519 signatures
- guardian quorum
- source token
- source sender
- source burn transaction hash
- source burn event index
- source block number
- source block hash
- source nonce
- burned amount as a separate field
- `xxxlMintAmount == burnedAmount`
- recipient hash derivation
- message expiry / TTL
- message nonce

Those properties remain Stage 1 model responsibilities until a separate,
reviewed runtime authorization-result representation is introduced.

## What Runtime Must Not Reinterpret

The runtime must not reinterpret:

- a relayer-submitted instruction as proof of Stage 1 authorization
- guardian set account presence as guardian quorum verification
- `canonicalEventKey` as an arbitrary relayer-chosen replay key
- `args.amount` as independent operator-chosen mint amount
- `args.recipient` as an unchecked replacement for the signed recipient hash
- Gateway Config source-chain weight as permission to change Stage 1 monetary
  policy silently
- bytes `194..208` as source-chain fields, deadline fields, nonce fields, or
  other production fields
- dormant CPI helper existence as live mint readiness

The runtime must preserve the Stage 1 result rather than recompute a different
authorization model from partial runtime fields.

## What Runtime Must Not Trust Blindly

Even after Stage 1 authorization succeeds, the runtime must not blindly trust:

- account order
- account writability
- signer flags
- account owners
- rent-exemption status
- local account discriminators, versions, or lengths
- SPL mint authority
- recipient token account owner or mint
- mint authority PDA account and bump
- processed-event account identity
- recipient-balance owner or mint
- decoded amount range
- decoded route, mint, guardian set, canonical event key, recipient, or
  source-chain weight when accounts disagree

The runtime account layer is responsible for rejecting account substitution and
relationship mismatches before any future mutation or CPI.

## Failure Behavior Matrix

| Failure case | Stage 1 responsibility | Current runtime behavior / gap |
| --- | --- | --- |
| invalid message | rejected by Stage 1 message validation | no runtime verifier; malformed runtime instruction still rejected by current instruction decoder if it violates SVM layout |
| quorum failure | rejected by Stage 1 guardian quorum verification | no runtime quorum verifier; runtime only checks guardian set ID relationships |
| invalid approval | rejected by Stage 1 approval verification | no runtime approval verifier |
| unknown guardian | rejected by Stage 1 guardian verification | runtime does not inspect guardian keys |
| replayed `canonicalEventKey` | Stage 1 rejects source replay; runtime must reject local replay | current runtime rejects an already-consumed Processed Event account, but enabled entrypoint does not mark events consumed |
| wrong route | Stage 1 rejects signed route mismatch | current runtime rejects Gateway Config or Processed Event route mismatch |
| wrong mint | Stage 1 rejects wrong `mintToken` | current runtime rejects Mint State, Gateway Config, Recipient Balance, SPL mint, or recipient token account mint mismatch |
| wrong guardian set | Stage 1 result must bind to the intended guardian set model | current runtime rejects Gateway Config or Guardian Set ID mismatch |
| stale / expired message | Stage 1 includes `deadlineOrFinalityBlock`; baseline does not enforce expiry | current runtime has no TTL/finality-block field or expiry check |
| mismatched recipient | Stage 1 rejects wrong `x1RecipientHash` | current runtime rejects Processed Event recipient, Recipient Balance owner, or recipient token owner mismatch |
| mismatched amount | Stage 1 rejects `xxxlMintAmount != burnedAmount` | current runtime checks decoded amount is nonzero and within `u64`; it does not see `burnedAmount` separately |
| mismatched source-chain weight | Stage 1 rejects non-10000 Stage 1 route weight | current runtime rejects Gateway Config source-chain weight mismatch |

Every rejected runtime path must remain no-mutation. If a Stage 1
authorization result is valid at model level but runtime account-level
validation fails, the runtime must not mark the Processed Event consumed, must
not credit Recipient Balance, must not update mint/supply accounting, and must
not execute SPL CPI. Current enabled behavior remains validation-only and
therefore performs no mutation in either success or failure cases.

Current Mollusk and lower-level coverage include selected no-mutation checks,
but complete Mollusk/SVM coverage criteria remain undefined.

## Replay and Processed-event Mapping

Stage 1 proves processed burn replay protection at model level. The runtime
maps `canonicalEventKey` to a local Processed Event account:

- the Processed Event must not already be consumed
- its `canonical_event_key` must match the decoded `args.canonical_event_key`
- its route must match `args.route_id`
- its recipient must match `args.recipient`

Current enabled behavior is validation-only:

- `process_instruction -> process_consume_gateway_mint` validates and builds a
  disabled plan
- it does not mark the Processed Event consumed
- it does not credit Recipient Balance
- it does not update Mint State / supply
- it does not perform SPL CPI

Future live behavior must preserve atomicity:

- no mint without processed mark
- no processed mark without mint
- no recipient accounting update without processed mark
- no supply accounting update without processed mark
- failure at any step leaves all required state unchanged

Replay variants for coefficient version, guardian set version, pause/unpause,
upgrade, and source fork remain deferred.

## Guardian Set and Quorum Mapping

Stage 1 proves guardian quorum at model level for the deterministic guardian
configuration and approval set.

The current runtime skeleton does not verify Ed25519 signatures or count
guardian quorum. It only checks:

- decoded `guardian_set_id`
- Gateway Config guardian set ID
- Guardian Set account guardian set ID

This means guardian quorum output is currently represented as an assumed
property of the consumed Stage 1 authorization result, not as an independently
verified SVM runtime computation.

`PRODUCTION_GUARDIAN_SET_UNSET` remains active. No production guardian
configuration is introduced by Phase 5.

## Amount and Mint Mapping

Stage 1 validates:

- `burnedAmount > 0`
- `xxxlMintAmount = burnedAmount`
- `sourceChainWeightBps = 10000` for the Stage 1 full-weight route
- `mintToken` equals the configured XXXL mint token identity

The current runtime skeleton validates:

- decoded `amount` is greater than zero
- decoded `amount <= u64::MAX`
- Gateway Config `source_chain_weight_bps()` equals decoded
  `source_chain_weight_bps`
- Mint State mint, Gateway Config target mint, Recipient Balance mint, SPL mint
  account, and recipient token account mint relationships align
- SPL mint authority is the gateway mint authority PDA

Open design gap preserved from Phase 4:

- the instruction decodes amount as `u128`, but the enabled SPL-compatible
  boundary accepts only values within the `u64` SPL Token amount range; final
  rationale remains unresolved.

The runtime must not mint a different amount than the Stage 1 approved amount.
Today there is no separate runtime field for `burnedAmount`, so
`xxxlMintAmount == burnedAmount` remains a Stage 1 model property rather than a
runtime re-check.

## Recipient Mapping

Stage 1 validates a nonzero 32-byte X1 recipient and binds
`x1RecipientHash` to the signed message.

The current runtime skeleton carries raw recipient bytes in `args.recipient`
and re-checks:

- Processed Event recipient
- Recipient Balance owner
- recipient token account owner

It does not recompute or verify `x1RecipientHash`. Runtime hash verification
would require a separate reviewed representation of the Stage 1 authorization
payload or result.

## Deadline, Finality, and Staleness Mapping

Stage 1 generated vectors include `deadlineOrFinalityBlock` and `messageNonce`
as mandatory 32-byte fields. Stage 1.5 records that the baseline does not
enforce message expiry and that production runtime must choose a validity
model.

The current SVM instruction has no named deadline, finality block, or message
nonce field. It therefore cannot enforce stale-message or TTL rejection today.

Any future expiry, finality, or nonce enforcement must be introduced through a
separate reviewed boundary. It must not use bytes `194..208` silently.

## Current Local Skeleton Behavior

Implemented today:

- strict 208-byte `CONSUME_GATEWAY_MINT` decode
- instruction discriminator and version checks
- encoded account meta count and local index checks
- 9-account contract validation
- local account owner, rent, discriminator, version, and length checks
- route / guardian / mint / processed-event / recipient relationship checks
- SPL mint and recipient token account validation
- gateway mint authority PDA and bump validation
- nonzero and `u64` amount checks
- disabled execution-plan construction
- no local mutation from enabled entrypoint
- no SPL CPI from enabled entrypoint

Deferred:

- runtime representation of the full Stage 1 authorization result
- runtime verification of Stage 1 message hash / domain separator
- runtime Ed25519 approval and guardian quorum verification
- runtime source-chain field re-checks not represented in the current
  instruction
- runtime recipient hash verification
- message expiry / TTL enforcement
- live processed-event mutation
- live recipient-balance mutation
- live supply update
- SPL CPI execution
- complete Mollusk/SVM coverage checkpoint criteria

## Deferred Implementation Gaps

The following Phase 4 gates remain preserved:

- bytes `194..208` are reserved, unparsed, and not zero-validated; no code may
  read, interpret, or validate them as named production fields until a separate
  boundary.
- the `u128` amount layout with `u64` SPL range remains a design gap unless
  classified more precisely in a later stage.
- the current enabled runtime path validates and builds a disabled execution
  plan, then returns `Ok(())` without mutation or SPL CPI; there is no distinct
  live-route-disabled error from the enabled path today.
- dormant CPI helpers contain `mint_to` / `invoke_signed` source-level code,
  but the enabled `process_instruction` path does not reach SPL CPI,
  `invoke_signed`, or `mint_to`.
- 10 ignored Mollusk tests remain an evidence gap with shared reason
  `requires cargo build-sbf and target/deploy/xxxl_svm.so`.
- complete Mollusk/SVM coverage criteria remain undefined before upgrade or
  live readiness.

Additional Phase 5 gaps:

- current runtime instruction layout does not carry most Stage 1 source fields
  as named runtime fields.
- `GatewayConfigAccountView::source_chain_id()` exists, but current processor
  validation does not compare it to a decoded instruction source-chain field.
  Phase 6 must keep this as an explicit decision path for disabled processor
  control-flow reconciliation: either source chain remains a Gateway Config /
  Stage 1 authorization-result property, or a future reviewed boundary adds a
  named runtime representation. This must not be resolved silently through
  bytes `194..208`, and it must be closed before any live-route or SPL-CPI
  enablement stage.
- guardian quorum remains a Stage 1 model property, not a runtime computation.
- recipient hash derivation remains a Stage 1 model property, not a runtime
  computation.
- stale / expired message behavior remains undecided.

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

Phase 5 made no runtime code changes.

Phase 5 changed no tests.

Phase 5 did not deploy.

Phase 5 did not upgrade.

Phase 5 did not submit transactions.

Phase 5 did not spend SOL.

Phase 5 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 5 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

Phase 5 made no new deployment. The existing X1 testnet scaffold remains
locked, non-live, and unable to mint through the currently enabled executable
entrypoint path.

Dormant CPI helpers still have source-level and unit-test call sites, but
remain unreachable from the currently enabled executable entrypoint path.

## Acceptance Criteria for Phase 5

Phase 5 is acceptable as a docs-only modeling stage if:

- the Stage 1 authorization result is described as the consumed model-level
  boundary, not as runtime proof of production readiness
- the runtime account-level re-checks are distinguished from Stage 1 message
  verification
- all Stage 1 fields listed in generated vectors are classified as either
  already represented in the current runtime or deferred
- account-write and CPI atomicity requirements are preserved as future runtime
  obligations
- Phase 4 gates remain open and are not silently resolved
- all deployment, live-route, SPL-CPI, guardian, proof-log, and external-review
  blockers remain active

## Audit Minor Notes Resolution

Phase 5 post-audit clarifications:

- the Stage 1 generated vector `fieldOrder` contains 19 fields; the Field
  Responsibility Matrix now explicitly lists `messageType` and `schemaVersion`
  in addition to the previously listed route/source/recipient/amount/replay
  fields.
- `current-design-checkpoint.md` is a rolling aggregate / summary artifact. The
  authoritative Phase 5 artifact is this standalone checkpoint. External audit
  can validate Phase 5 from the Phase 1-5 checkpoint files, the implementation
  boundary, the Stage 1 consumer document, Stage 1.5 mapping notes, vectors,
  and validation output without relying on the full aggregate file.
- the 27 Stage 1 invalid vectors remain Stage 1 rejection cases. They do not
  imply SVM runtime verification today. Future Phase 6/7 coverage should record
  that if Stage 1 rejects a message, the runtime mint path is never reached; a
  relayer-submitted SVM instruction alone is not proof of Stage 1 authorization.
- `messageNonce` has no current runtime replay semantics. Runtime replay
  identity remains `canonicalEventKey`. Any future runtime semantics for
  `messageNonce` require a separate reviewed boundary and must not be mixed
  silently with Processed Event replay state.
- source-chain ID remains an explicit Phase 6 decision path before any live
  route or SPL-CPI enablement stage.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-6-disabled-processor-control-flow-reconciliation`

That stage should reconcile the current `Ok(())` disabled-plan behavior with
the implementation plan's Phase 6 expectation for disabled processor control
flow. It must not enable live route execution, SPL CPI, `invoke_signed`, or SPL
Token `mint_to`.
