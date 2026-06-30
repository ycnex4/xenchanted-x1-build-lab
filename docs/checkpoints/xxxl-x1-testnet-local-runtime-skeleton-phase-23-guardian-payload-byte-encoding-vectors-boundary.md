# XXXL X1 Testnet Local Runtime Skeleton Phase 23 Guardian Payload Byte Encoding Vectors Boundary

Status: TypeScript-only encoding and vector boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-23-guardian-payload-byte-encoding-vectors-boundary`

Base:

- `8e0b956 Merge XXXL phase 22 guardian payload structure`

## Purpose

Phase 23 turns the Phase 22 semantic guardian payload structure into an exact
byte-level encoding and deterministic vector surface.

Phase 23 implements:

- canonical guardian payload byte encoding v1
- payload hash domain label and domain separator
- payload hash preimage construction
- payload hash generation
- valid vector fixture
- deterministic invalid-vector manifest
- TypeScript tests for encoding, hashing, validation, and explicit non-goals

Phase 23 implements byte-level guardian payload encoding and vectors only.

Phase 23 does not implement guardian signature verification.

Phase 23 does not implement guardian quorum validation.

Phase 23 does not import or call the Stage 1 Ed25519 verifier.

Phase 23 does not add guardian public key account handling.

Phase 23 does not add runtime account mutations.

## Files Added Or Changed

Added:

- `src/xxxl/guardian-payload-encoding.ts`
- `tests/xxxl/guardian-payload-encoding.test.ts`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-23-guardian-payload-byte-encoding-vectors-boundary.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

## Canonical Field Order

Phase 23 preserves the exact Phase 22 guardian-signed payload semantic field
order:

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

The exported field order is:

- `XXXL_GUARDIAN_PAYLOAD_FIELD_ORDER`

## Encoding

Encoding name:

- `XXXL_GUARDIAN_PAYLOAD_CANONICAL_BINARY_V1`

Message type:

- `XXXL_GATEWAY_MINT`

Schema version:

- `1`

Current instruction layout version:

- `2`

Canonical binary v1 encoding:

- `message_type`: u16 little-endian byte length plus UTF-8 bytes
- `schema_version`: u16 little-endian
- `instruction_layout_version`: u16 little-endian
- `route_id`: bytes32
- `source_chain_id`: u64 little-endian
- `source_token`: u16 little-endian byte length plus raw bytes
- `source_sender`: u16 little-endian byte length plus raw bytes
- `source_burn_tx_hash`: u16 little-endian byte length plus raw bytes
- `source_burn_event_index`: u64 little-endian
- `source_block_number`: u64 little-endian
- `source_block_hash`: u16 little-endian byte length plus raw bytes
- `source_finality_block`: u64 little-endian
- `canonical_event_key`: bytes32
- `x1_recipient`: bytes32
- `burned_amount`: u128 little-endian
- `source_chain_weight_bps`: u16 little-endian
- `xxxl_mint_amount`: u128 little-endian
- `target_mint`: bytes32
- `guardian_set_id`: bytes32
- `message_nonce`: bytes32
- `expiration_slot_or_unix_ts`: u64 little-endian

The encoder is pure TypeScript and does not call SVM runtime code.

## Hashing

Hash domain label:

- `XXXL_GUARDIAN_PAYLOAD_HASH_V1`

Hash domain separator:

- `keccakUtf8Label("XXXL_GUARDIAN_PAYLOAD_HASH_V1")`

Payload bytes:

- encoded guardian payload only

Hash preimage:

- `keccakUtf8Label(domain label) || encoded guardian payload`

Payload hash:

- `keccak256(hash preimage)`

The valid vector id is:

- `xxxl-guardian-payload-canonical-binary-v1-valid-001`

The valid vector payload hash is:

- `0xab0ee59a1268f3eebf4a9d42725640ce68226e642a61dabd5f904e7680f08015`

The valid vector domain separator is:

- `0xf1958bbf04d45ddbc5a9f93f200f5005ee47b05cf61a90faf4d93cd6e3eccd66`

## Validation Rules

The encoder rejects:

- `route_id` that is not exactly 32 bytes
- `canonical_event_key` that is not exactly 32 bytes
- `x1_recipient` that is not exactly 32 bytes
- `target_mint` that is not exactly 32 bytes
- `guardian_set_id` that is not exactly 32 bytes
- `message_nonce` that is not exactly 32 bytes
- empty `source_token`
- empty `source_sender`
- empty `source_burn_tx_hash`
- empty `source_block_hash`
- length-prefixed byte fields longer than 65535 bytes
- `source_chain_weight_bps` outside `0..10000`
- zero `burned_amount`
- zero `xxxl_mint_amount`
- u64 fields outside `0..2^64-1`
- u128 fields outside `0..2^128-1`
- wrong `message_type`
- wrong `schema_version`
- `instruction_layout_version` other than `2`

The vector validator catches:

- wrong encoding name
- wrong field order
- wrong hash domain label
- tampered `encodedPayloadHex`
- tampered `hashPreimageHex`
- tampered `payloadHash`
- tampered `hashDomainSeparatorHex`
- invalid field values

## Explicit Non-Goals

Phase 23 does not change `programs/xxxl-svm/src`.

Phase 23 does not change `programs/xxxl-svm/tests`.

Phase 23 does not change Cargo files.

Phase 23 does not run `cargo build-sbf`.

Phase 23 does not touch `target/deploy`.

Phase 23 does not inspect or touch `.local-keys` contents.

Phase 23 does not inspect or touch keypair files.

Phase 23 does not read `.env`.

Phase 23 does not add deploy commands.

Phase 23 does not add upgrade commands.

Phase 23 does not submit transactions.

Phase 23 does not spend SOL.

Phase 23 does not enable live route execution.

Phase 23 does not enable SPL CPI execution.

Phase 23 does not enable `invoke_signed`.

Phase 23 does not enable SPL Token `mint_to`.

Phase 23 does not derive `canonical_event_key` from `source_chain_id`.

Phase 23 does not claim production readiness.

Phase 23 does not claim final immutability while upgrade authority exists.

## Safety Status

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

`source_chain_weight_bps` remains signed and dual-source:

- guardian payload field
- runtime instruction field
- `GatewayConfig` binding field

`canonical_event_key` is not derived from `source_chain_id`.

GatewayConfig layout is unchanged.

Live route remains disabled.

SPL CPI remains disabled.

Enabled `process_instruction` remains a disabled-plan no-op for live atomicity.

## Phase 24 Handoff

Phase 24 may implement guardian signature and quorum validation only after this
Phase 23 byte encoding and vector surface is audited.

Phase 24 must not reinterpret this payload shape implicitly.

Phase 24 must preserve the Phase 23 hash domain unless a reviewed boundary
explicitly changes it.

Phase 24 must still separately specify:

- signature algorithm and public key representation
- guardian set versioning
- guardian approval ordering
- duplicate guardian handling
- quorum threshold rule
- expired payload rejection rule
- replay boundary assignment
- runtime storage or proof account shape

Phase 24 must not enable live route execution or SPL CPI unless a separate
reviewed boundary explicitly allows it.

## Validation

Commands run:

- `npm test -- --run`: passed, 97 test files passed, 862 tests passed
- `npm run build`: passed

Required final workspace checks:

- `git diff --check`
- `git status --short --untracked-files=all`

No Cargo validation was run.

No SBF build was run.

No Solana command was run.
