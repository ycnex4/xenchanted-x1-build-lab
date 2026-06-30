# XXXL X1 Testnet Local Runtime Skeleton Phase 22 Guardian Payload Structure Boundary

Status: Documentation and specification boundary only.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-22-guardian-payload-structure-boundary`

Base:

- `b7ca875 Merge XXXL phase 21 Mollusk SBF revalidation`

## Purpose

Phase 22 defines the future guardian-signed payload structure before guardian
signature parsing, guardian quorum validation, or live route execution is added.

This phase separates:

- fields that guardians are expected to sign
- fields currently carried by the Phase 21 runtime instruction
- fields currently enforced through runtime account binding
- fields not yet represented by the runtime instruction/account model
- fields that remain future blockers before live route activation

Phase 22 prevents design drift before Phase 23.

Phase 22 is documentation/specification only.

Phase 22 does not change runtime source.

Phase 22 does not change tests.

Phase 22 does not change Cargo files.

Phase 22 does not change deploy scripts.

Phase 22 does not run or require SBF build.

Phase 22 does not touch `target/deploy`.

Phase 22 does not touch keypair files.

Phase 22 does not add signature verification.

Phase 22 does not add guardian quorum validation.

Phase 22 does not add guardian account layout changes.

Phase 22 does not add `message_nonce` replay semantics.

Phase 22 does not enable live route execution.

Phase 22 does not enable SPL CPI execution.

Phase 22 does not enable `invoke_signed`.

Phase 22 does not enable SPL Token `mint_to`.

Phase 22 does not derive `canonical_event_key` from `source_chain_id`.

Phase 22 does not claim production readiness.

Phase 22 does not claim final immutability while upgrade authority exists.

## Phase 21 Baseline

Phase 21 is the baseline for this boundary.

Already closed by Phase 21:

- instruction layout version is `2`
- instruction length remains `208`
- `source_chain_id` is parsed from Rust slice `194..202`
- reserved zero bytes are Rust slice `202..208`
- version `1` rejects with `InvalidVersion`
- nonzero reserved bytes reject with `InvalidInstructionReserved`
- instruction `source_chain_id` binds to existing
  `GatewayConfig.source_chain_id()`
- GatewayConfig layout is unchanged
- Mollusk v2 was revalidated against a refreshed local ignored SBF artifact

Phase 22 builds on this without changing runtime code.

## Future Guardian Signed Payload Field Set

Phase 22 freezes the semantic field set and canonical field order for the future
guardian-signed payload.

Future signed payload semantic order:

1. `message_type`
2. `schema_version`
3. `instruction_layout_version`
4. `route_id`
5. `source_chain_id`
6. `source_token`
7. `source_sender`
8. `source_burn_tx_hash`
9. `source_burn_event_index`
10. `source_block_number`
11. `source_block_hash`
12. `source_finality_block`
13. `canonical_event_key`
14. `x1_recipient`
15. `burned_amount`
16. `source_chain_weight_bps`
17. `xxxl_mint_amount`
18. `target_mint`
19. `guardian_set_id`
20. `message_nonce`
21. `expiration_slot_or_unix_ts`

This phase freezes the semantic field set and canonical order.

This phase does not yet define byte-level encoding.

Before guardian signature verification can be enabled, a later boundary must
define:

- exact byte-level encoding
- integer widths and endian rules
- fixed-width versus length-prefixed field representation
- hash domain / domain separator
- payload hash preimage
- test vectors
- invalid vectors
- cross-language encoding compatibility, if applicable

No guardian signature validation may be implemented from an implicit or
unversioned payload shape.

## Current Phase 21 Runtime Mapping

Current Phase 21 runtime instruction fields map to the future signed payload as
follows:

| Current runtime instruction field | Future signed payload field |
| --- | --- |
| `route_id` | `route_id` |
| `guardian_set_id` | `guardian_set_id` |
| `mint_id` | `target_mint` or canonical mint identifier |
| `canonical_event_key` | `canonical_event_key` |
| `recipient` | `x1_recipient` |
| `amount` | `xxxl_mint_amount` |
| `source_chain_weight_bps` | `source_chain_weight_bps` |
| `source_chain_id` | `source_chain_id` |
| instruction layout version | `instruction_layout_version` |

The current runtime instruction does not yet carry the full future guardian
payload.

The current runtime instruction remains a compact local runtime instruction.

The future guardian-signed payload may be larger and more source-chain aware than
the current runtime instruction.

## Current Account / Runtime Binding

Phase 21 already enforces the following account/runtime bindings:

- `GatewayConfig.route_id` must match instruction `route_id`
- `GatewayConfig.source_chain_id` must match instruction `source_chain_id`
- `GatewayConfig.source_chain_weight_bps` must match instruction
  `source_chain_weight_bps`
- `GatewayConfig.target_mint` must match the instruction mint / target mint path
- `GatewayConfig.guardian_set_id` must match instruction `guardian_set_id`
- `GuardianSet.guardian_set_id` must match instruction `guardian_set_id`
- `MintState.mint_id` must match instruction `mint_id`
- `ProcessedEvent.canonical_event_key` must match instruction
  `canonical_event_key`
- `ProcessedEvent.recipient` must match instruction `recipient`
- `RecipientBalance` owner / mint must match the instruction recipient / mint
  path
- recipient token account owner / mint must match the instruction recipient /
  mint path
- SPL mint authority must match the expected mint authority PDA
- SPL token program id must match the expected SPL Token program id

These bindings remain account/runtime validation.

These bindings are not a substitute for future guardian signature verification.

## Fields Not Yet Represented By Current Runtime Instruction / Account Validation

The following future signed payload fields are not yet represented in the current
runtime instruction/account validation path:

- `source_token`
- `source_sender`
- `source_burn_tx_hash`
- `source_burn_event_index`
- `source_block_number`
- `source_block_hash`
- `source_finality_block`
- `burned_amount`, if distinct from `xxxl_mint_amount`
- `message_nonce` replay semantics
- `expiration_slot_or_unix_ts`

These remain blockers before any live route activation.

These fields may be introduced through a future payload account, expanded
instruction layout, guardian proof account, proof log account, or other reviewed
runtime boundary.

Phase 22 does not choose that storage or transport mechanism.

Phase 22 only freezes the semantic field set and order for the future
guardian-signed payload.

## Source-chain Weight Decision

Phase 21 preserved `source_chain_weight_bps` as a dual-source field:

- instruction field
- `GatewayConfig` binding field

Phase 22 records that `source_chain_weight_bps` remains part of the future
guardian-signed payload unless a later reviewed boundary explicitly moves it to
config-only.

Rationale:

- if it affects the deterministic mint amount, guardians should sign the value
  they approve
- account binding can still ensure the signed/instruction value matches the
  active route configuration
- moving this field to config-only would change trust assumptions and must not
  happen implicitly

No change is made in Phase 22.

## Canonical Event Key Decision

Phase 22 does not derive `canonical_event_key` from `source_chain_id`.

Phase 22 does not change `canonical_event_key` semantics.

Phase 22 does not change replay protection semantics.

Future exact `canonical_event_key` derivation must be specified before live route
activation and before relying on it as the final replay-safety primitive.

The future derivation must define enough source identity to avoid collisions
across:

- source chain
- source token
- source transaction
- event index / log index
- route domain
- message type / schema, if required

No such derivation is implemented in Phase 22.

## Message Nonce / Replay Boundary

`message_nonce` is included in the future guardian-signed payload semantic field
set.

Phase 22 does not implement `message_nonce` storage.

Phase 22 does not implement `message_nonce` replay rejection.

Phase 22 does not decide whether replay protection is keyed by:

- `canonical_event_key`
- `message_nonce`
- both `canonical_event_key` and `message_nonce`
- a guardian proof hash
- a future proof log account

This must be assigned explicitly before live route activation.

## Expiration Boundary

`expiration_slot_or_unix_ts` is included in the future guardian-signed payload
semantic field set.

Phase 22 does not implement expiration checks.

Phase 22 does not decide whether the runtime uses:

- slot
- unix timestamp
- source finality block
- guardian-signed deadline
- a combination of those fields

This must be specified before signature validation is relied on for live route
execution.

## Phase 23 Handoff

Phase 23 may only implement guardian signature/quorum validation after the
following are specified:

- exact byte-level payload encoding
- payload hash domain / domain separator
- payload hash preimage
- signature algorithm and public key representation
- guardian set versioning
- guardian approval ordering and duplicate handling
- quorum threshold rule
- expired payload rejection rule
- replay boundary assignment
- test vectors
- invalid vectors

Phase 23 must not infer payload encoding from this document alone.

Phase 23 must not silently omit source-chain burn evidence fields.

Phase 23 must not enable live route execution.

Phase 23 must not enable SPL CPI execution.

## Explicitly Not Implemented

Phase 22 does not implement:

- guardian signature parsing
- guardian signature verification
- guardian quorum validation
- guardian proof account layout
- guardian approval account layout
- message nonce replay protection
- expiration checks
- source block finality checks
- proof log storage
- source-chain log parsing
- Ethereum burn proof validation
- watcher integration
- relayer integration
- live SPL mint success path
- rollback after live SPL CPI failure

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

## Correct Statement

Correct:

- Phase 22 freezes the semantic guardian payload field set and canonical order
- Phase 22 separates signed payload fields from current account/runtime bindings
- Phase 22 records current Phase 21 runtime mapping
- Phase 22 keeps `source_chain_weight_bps` signed unless later moved by reviewed
  boundary
- Phase 22 does not change runtime code
- Phase 22 does not claim byte-level encoding is implemented
- Phase 22 does not claim signature verification is implemented

Incorrect:

- Phase 22 implements guardian signatures
- Phase 22 implements guardian quorum
- Phase 22 implements replay protection
- Phase 22 enables live route execution
- Phase 22 enables SPL CPI
- Phase 22 changes GatewayConfig layout
- Phase 22 derives `canonical_event_key` from `source_chain_id`
- Phase 22 proves production readiness
- Phase 22 proves final immutability

## Validation

Docs-only validation:

- `git diff --check`: required before commit
- `git status --short --untracked-files=all`: required before commit

No SBF build is required.

No deploy command is allowed.

No upgrade command is allowed.

No transaction submission is allowed.

No SOL spend is allowed.

No `target/deploy` access is required.

## Recommended Next Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-23-guardian-signature-quorum-boundary`

Only begin Phase 23 after byte-level payload encoding and test vectors are
defined.
