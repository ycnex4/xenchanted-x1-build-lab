# XXXL X1 Testnet Local Runtime Skeleton Phase 3 Instruction Decode Reconciliation

Status: Docs-only reconciliation complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation`

Base:

- `0273835 Add X1 testnet local runtime skeleton phase 2 account layout reconciliation`

This checkpoint reconciles the current `CONSUME_GATEWAY_MINT` instruction
decoder with the Phase 2 account layout reconciliation, processor boundary,
and current Mollusk coverage. It does not implement runtime code.

## Inputs Reviewed

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/account_contract.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

## Summary

The current decoder is strict about the instruction discriminator, layout
version, exact 208-byte instruction length, encoded account meta count, and
five encoded local account indices. It then extracts route, guardian set, mint,
canonical event key, recipient, amount, and source-chain weight fields at fixed
offsets.

The decoded fields do not execute a live mint. They flow into
`process_instruction -> process_consume_gateway_mint`, which builds a disabled
execution plan and returns without state mutation, SPL CPI, `invoke_signed`, or
SPL Token `mint_to` from the currently enabled executable entrypoint path.

## Current Instruction Layout

`CONSUME_GATEWAY_MINT_INSTRUCTION_LEN` is exactly `208` bytes.

Current byte layout:

| Bytes | Field | Current rule |
| --- | --- | --- |
| `0..8` | discriminator | must equal `CONSUME_GATEWAY_MINT_DISCRIMINATOR` |
| `8..10` | layout version | little-endian `u16`, must equal `INSTRUCTION_LAYOUT_VERSION = 1` |
| `10` | encoded account meta count | must equal `9` |
| `11` | route account index | must equal `1` |
| `12` | guardian set account index | must equal `2` |
| `13` | mint state account index | must equal `0` |
| `14` | processed event account index | must equal `3` |
| `15` | recipient balance account index | must equal `4` |
| `16..48` | route ID | copied into `args.route_id` |
| `48..80` | guardian set ID | copied into `args.guardian_set_id` |
| `80..112` | mint ID | copied into `args.mint_id` |
| `112..144` | canonical event key | copied into `args.canonical_event_key` |
| `144..176` | recipient | copied into `args.recipient` |
| `176..192` | amount | little-endian `u128`, copied into `args.amount` |
| `192..194` | source-chain weight | little-endian `u16`, copied into `args.source_chain_weight_bps` |
| `194..208` | trailing reserved/unparsed bytes | preserved in `args.raw`; not currently named or zero-checked |

## Decode Checks

Discriminator check:

- `XxxlInstruction::unpack` rejects any non-matching first 8 bytes with
  `InvalidDiscriminator`.

Instruction version check:

- bytes `8..10` are decoded as little-endian `u16`.
- any value other than `1` returns `InvalidVersion`.

Exact byte-length check:

- input length must equal `208`.
- shorter or longer input returns `InvalidInstruction`.

Encoded account meta count check:

- byte `10` must equal `CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT = 9`.
- mismatch returns `InvalidInstruction`.

Encoded account index checks:

- byte `11` must equal route account index `1`.
- byte `12` must equal guardian set account index `2`.
- byte `13` must equal mint state account index `0`.
- byte `14` must equal processed event account index `3`.
- byte `15` must equal recipient balance account index `4`.
- any mismatch returns `InvalidInstruction`.

Field extraction:

- all route, guardian set, mint, event, recipient, amount, and source-chain
  weight fields are extracted only after the length, discriminator, version,
  account-meta-count, and encoded-index checks pass.
- bytes `194..208` remain unparsed beyond being retained in `args.raw`.

## Account Index Reconciliation

Route account index:

- decoder field: byte `11`, expected `1`
- account contract: index `1`, `gateway_config`
- processor use: `account_at(accounts, args.route_account_index as usize)`
- Phase 2 layout consumed: `GatewayConfigAccountView`

Guardian set account index:

- decoder field: byte `12`, expected `2`
- account contract: index `2`, `guardian_set`
- processor use: `account_at(accounts, args.guardian_set_account_index as usize)`
- Phase 2 layout consumed: `GuardianSetAccountView`

Mint state account index:

- decoder field: byte `13`, expected `0`
- account contract: index `0`, `mint_state`
- processor use: `account_at(accounts, args.mint_state_account_index as usize)`
- Phase 2 layout consumed: `MintStateAccountView`

Processed event account index:

- decoder field: byte `14`, expected `3`
- account contract: index `3`, `processed_event`
- processor use:
  `account_at(accounts, args.processed_event_account_index as usize)`
- Phase 2 layout consumed: `ProcessedEventAccountView`

Recipient balance account index:

- decoder field: byte `15`, expected `4`
- account contract: index `4`, `recipient_balance`
- processor use:
  `account_at(accounts, args.recipient_balance_account_index as usize)`
- Phase 2 layout consumed: `RecipientBalanceAccountView`

The remaining processor account indices are not encoded in instruction data:

- index `5`: `spl_token_mint`
- index `6`: `recipient_token_account`
- index `7`: `mint_authority_pda`
- index `8`: `token_program`

Those indices come from `account_contract.rs` / processor constants rather than
the decoded instruction payload.

## Field Reconciliation With Phase 2 Account Layouts

Route ID:

- decoded from bytes `16..48`
- must match `GatewayConfigAccountView::route_id()`
- must match `ProcessedEventAccountView::route_id()`
- copied into the disabled execution plan as `route_id`

Guardian set ID:

- decoded from bytes `48..80`
- must match `GatewayConfigAccountView::guardian_set_id()`
- must match `GuardianSetAccountView::guardian_set_id()`

Mint ID:

- decoded from bytes `80..112`
- must match `MintStateAccountView::mint_pubkey()`
- must match `GatewayConfigAccountView::target_mint()`
- must match `RecipientBalanceAccountView::mint()`
- is converted to `Pubkey` for recipient SPL token account validation
- copied into the disabled execution plan as `mint`

Canonical event key:

- decoded from bytes `112..144`
- must match `ProcessedEventAccountView::canonical_event_key()`
- copied into the disabled execution plan as `canonical_event_key`

Recipient:

- decoded from bytes `144..176`
- must match `ProcessedEventAccountView::recipient()`
- must match `RecipientBalanceAccountView::owner()`
- is converted to `Pubkey` for recipient SPL token account validation
- copied into the disabled execution plan as `recipient`

Amount:

- decoded from bytes `176..192` as little-endian `u128`
- processor boundary rejects zero amount
- processor boundary rejects values larger than `u64::MAX`
- prepared CPI boundary stores amount as `u64`
- disabled execution plan stores amount as `u64`
- enabled entrypoint does not mint this amount

Source-chain weight:

- decoded from bytes `192..194` as little-endian `u16`
- must match `GatewayConfigAccountView::source_chain_weight_bps()`
- copied into the disabled execution plan as `source_chain_weight_bps`

Recipient Balance continuity from Phase 2:

- Recipient Balance is local model-level accounting only.
- Recipient Balance is not an SPL Token account.
- Recipient Balance is not the recipient ATA.
- actual recipient token balance remains managed by the recipient SPL token
  account / ATA, outside this Phase 3 checkpoint.

## Processor Validation Relationship

The decoded instruction fields are consumed by
`prepare_consume_gateway_mint_cpi_boundary` after `process_instruction`
dispatches `ConsumeGatewayMint`.

Current processor relationship checks include:

- account count and encoded account meta count
- account contract writable/signer flags
- SPL Token program ID equals `spl_token::id()`
- program-owned local accounts are owned by the current `program_id`
- rent exemption for local program-owned and SPL token accounts
- local account discriminator, version, and exact length checks through Phase 2
  account views
- mint state `mint_pubkey` and gateway mint authority PDA relationship
- gateway config route, guardian set, target mint, and source-chain weight
  relationships
- guardian set ID relationship
- processed event unconsumed status and canonical event key, route, recipient
  relationships
- recipient balance owner and mint relationships
- initialized SPL mint with expected mint authority
- initialized recipient token account with expected owner and mint
- nonzero amount and amount within the SPL Token `u64` range

After validation, the enabled processor builds an
`AtomicConsumeGatewayMintExecutionPlan` with disabled live-route and disabled
mint-to flags, then returns without executing local mutation or SPL CPI.

## Test Coverage Reconciliation

Coverage categories:

- Mollusk integration coverage means a non-ignored test in
  `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`.
- Lower-level coverage means a unit or handler-style test in
  `programs/xxxl-svm/src/**`.
- Deferred means no exact current test was found for that specific variant.

| Requirement | Current coverage |
| --- | --- |
| wrong discriminator | Mollusk integration: `mollusk_rejects_wrong_instruction_discriminator_without_live_route`; lower-level unit: `consume_gateway_mint_rejects_wrong_instruction_discriminator` |
| wrong version | Mollusk integration: `mollusk_rejects_wrong_instruction_version_without_live_route`; lower-level unit: `consume_gateway_mint_rejects_wrong_instruction_version` |
| wrong instruction length / truncated instruction | Mollusk integration: `mollusk_harness_rejects_malformed_instruction_without_live_route`; lower-level unit: `consume_gateway_mint_rejects_wrong_instruction_length` |
| extra bytes | Mollusk integration: `mollusk_rejects_extra_instruction_bytes_without_live_route`; lower-level unit: `consume_gateway_mint_rejects_wrong_instruction_length` |
| wrong encoded account meta count | Mollusk integration: `mollusk_rejects_wrong_encoded_account_meta_count_without_live_route`; lower-level unit: `consume_gateway_mint_rejects_wrong_account_meta_count` |
| wrong encoded route account index | Deferred as an exact byte-specific variant; decoder branch is structurally shared with the lower-level `consume_gateway_mint_rejects_wrong_account_index_boundary` test, but that test mutates the mint-state index byte |
| wrong encoded guardian set account index | Deferred as an exact byte-specific variant; decoder branch is structurally shared with the lower-level `consume_gateway_mint_rejects_wrong_account_index_boundary` test, but that test mutates the mint-state index byte |
| wrong encoded mint state account index | Lower-level unit: `consume_gateway_mint_rejects_wrong_account_index_boundary`; no non-ignored Mollusk exact variant found |
| wrong encoded processed event account index | Mollusk integration: `mollusk_rejects_wrong_encoded_processed_event_account_index_without_live_route`; shared lower-level decoder branch also exists |
| wrong encoded recipient balance account index | Mollusk integration: `mollusk_rejects_wrong_encoded_recipient_balance_account_index_without_live_route`; shared lower-level decoder branch also exists |
| zero amount | Mollusk integration: `mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged`; lower-level handler: `handler_integration_rejects_zero_amount` |
| amount overflow / `u64` boundary | Lower-level handler: `handler_integration_rejects_amount_larger_than_spl_token_u64_range`; no non-ignored Mollusk exact variant found |
| route ID mismatch | Mollusk integration covers processed-event route mismatch: `mollusk_rejects_wrong_processed_event_route_id_without_live_route`; lower-level handler covers gateway-config route mismatch: `handler_integration_rejects_gateway_route_mismatch` |
| guardian set ID mismatch | Lower-level handler: `handler_integration_rejects_gateway_config_guardian_set_id_mismatch` and `handler_integration_rejects_wrong_guardian_set_id`; no non-ignored Mollusk exact variant found |
| mint ID mismatch | Mollusk integration covers recipient-balance and recipient token mint mismatches: `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`, `mollusk_rejects_wrong_recipient_token_mint_without_live_route`; lower-level handler covers gateway-config target mint mismatch: `handler_integration_rejects_gateway_config_target_mint_mismatch`; no explicit mint-state mint mismatch test was found |
| canonical event key mismatch | Mollusk integration: `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`; lower-level handler: `handler_integration_rejects_wrong_processed_event_canonical_event_key` |
| recipient mismatch | Mollusk integration: `mollusk_rejects_wrong_processed_event_recipient_without_live_route`, `mollusk_wrong_processed_event_recipient_rejection_leaves_mutable_accounts_unchanged`, and `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`; lower-level handler coverage also exists |
| source-chain weight mismatch | Lower-level handler: `handler_integration_rejects_gateway_config_source_chain_weight_mismatch`; no non-ignored Mollusk exact variant found |

Ignored Mollusk tests remain present in the file and are not counted as
non-ignored coverage for this reconciliation.

## Gaps And Deferrals

Current exact coverage gaps:

- wrong encoded route account index has no exact non-ignored Mollusk test and
  no exact route-index unit variant
- wrong encoded guardian set account index has no exact non-ignored Mollusk
  test and no exact guardian-index unit variant
- wrong encoded mint state account index has lower-level unit coverage but no
  exact non-ignored Mollusk test
- amount overflow / `u64` boundary has lower-level handler coverage but no
  exact non-ignored Mollusk test
- guardian set ID mismatch has lower-level handler coverage but no exact
  non-ignored Mollusk test
- source-chain weight mismatch has lower-level handler coverage but no exact
  non-ignored Mollusk test
- mint-state `mint_pubkey` mismatch did not appear as an explicit test variant
- bytes `194..208` are part of the exact 208-byte instruction and preserved in
  `args.raw`, but are not currently named, interpreted, or zero-validated

Phase 2 replay deferrals remain open:

- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay rejection

No complete replay readiness is claimed.

## CPI And Live Route Continuity

Phase 2 found that dormant CPI helpers have source-level and unit-test call
sites. This remains true.

The whole codebase contains dormant CPI helper code and unit-test/source call
sites for planning, disabled guard checks, instruction building, and one direct
negative `mint_to_cpi_boundary` path.

The enabled `process_instruction -> process_consume_gateway_mint` path still
does not reach SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

Any claim about `invoke_signed` or SPL Token `mint_to` in this checkpoint is
scoped to the currently enabled executable entrypoint path, not to the whole
codebase.

Phase 3 does not make dormant CPI helpers reachable.

Phase 3 does not make `CONSUME_GATEWAY_MINT` executable as a live mint.

The enabled entrypoint path remains unable to mint.

## Safety Confirmation

Phase 3 made no runtime code changes.

Phase 3 changed no tests.

Phase 3 did not change `instruction.rs`.

Phase 3 did not change `processor.rs`.

Phase 3 did not edit `programs/xxxl-svm/src/**`.

Phase 3 did not edit `programs/xxxl-svm/tests/**`.

Phase 3 did not deploy or upgrade.

Phase 3 did not submit transactions.

Phase 3 did not spend SOL.

Phase 3 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 3 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The runtime remains non-deployed, non-live, and unable to mint through the
currently enabled executable entrypoint path.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

The implementation-plan blockers remain active:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

The source-level deployment report remains unchanged and still reports active
blockers, including:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Recommended Next Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-4-validation-error-model-reconciliation`

Before any runtime implementation stage, separately decide whether the
instruction trailing bytes `194..208` should remain reserved/unconstrained or
become an explicitly zero-checked reserved field, and whether to add exact
Mollusk coverage for the deferred decode and relationship variants.

## External Audit Minor Notes Addressed

External audit result:

- `ACCEPT WITH MINOR NOTES`
- no blocking issues found
- `bytes 194..208` are not a blocker for this docs-only stage
- all runtime blockers remain active

### Authoritative record marker

This Phase 3 checkpoint is the authoritative Phase 3 record.

The Phase 3 section appended to `docs/checkpoints/current-design-checkpoint.md`
is a condensed summary only.

### `args.raw` propagation clarification

Bytes `194..208` are retained in the decoded instruction args as trailing
reserved / unparsed bytes.

This Phase 3 checkpoint does not claim that these bytes are safe for production
semantics.

Before any future implementation stage can rely on live execution, Phase 4 or a
dedicated instruction-bytes stage must explicitly verify and record whether
`args.raw` propagates beyond the instruction args struct.

Required future evidence:

- confirm whether `args.raw` is absent from `AtomicConsumeGatewayMintExecutionPlan`
- confirm whether `args.raw` is absent from CPI boundary structures
- confirm whether `args.raw` is absent from account mutation paths
- confirm whether `args.raw` is absent from logs or telemetry that could later be
  treated as protocol evidence

Current design decision remains open:

- either bytes `194..208` remain explicitly reserved / unconstrained, with a
  documented rationale
- or bytes `194..208` become explicitly zero-checked reserved bytes, with tests

No live route may be enabled before this decision is made and reviewed.

### Mint-state mint relationship evidence gap

This checkpoint records a specific evidence gap:

- it was not confirmed in this Phase 3 docs-only reconciliation whether the
  current code actually checks `mint_state.mint_pubkey()` against `args.mint_id`
- if that check exists, the missing explicit test is a coverage gap
- if that check does not exist, the missing relationship check is a code gap

Before Phase 4 closure, this must be resolved by code inspection and recorded as
one of:

- `check exists; add explicit test coverage`
- `check absent; add implementation gap before any live route work`

No production readiness or complete validation readiness is claimed until this
is resolved.

### Encoded account index independent coverage gaps

The encoded account index gaps are distinct byte-specific gaps, not a single
generic gap.

Current independent coverage status:

- byte `11`, route account index: no independent targeted test recorded
- byte `12`, guardian set account index: no independent targeted test recorded
- byte `13`, mint state account index: lower-level unit coverage exists, but no
  exact non-ignored Mollusk integration test recorded
- byte `14`, processed event account index: non-ignored Mollusk integration
  coverage recorded
- byte `15`, recipient balance account index: non-ignored Mollusk integration
  coverage recorded

Before any later disabled processor control-flow closure can claim full decode
coverage, each of bytes `11..15` should have independent targeted coverage.

### `u128` amount field rationale remains open

The instruction currently encodes amount as little-endian `u128` in bytes
`176..192`.

The processor rejects zero and values larger than `u64::MAX`.

This means valid SPL Token mint amounts are effectively constrained to the
`u64` range even though the encoded field is 16 bytes.

The rationale for the 16-byte field is not finalized in this checkpoint.

Possible rationales include cross-chain schema alignment, future-proofing, or
canonical message width compatibility, but Phase 3 does not choose among them.

Before live execution or instruction format closure, the project must record the
actual rationale and keep the validation rule explicit:

- any amount equal to zero is invalid
- any amount greater than `u64::MAX` is invalid
- only the validated `u64` amount may reach any future SPL Token mint boundary

### Pending `mint_to_cpi_boundary` / `invoke_signed` question

The Phase 2 question remains explicitly open:

- does `mint_to_cpi_boundary` contain `invoke_signed` internally?
- does the direct `cpi.rs` unit-test negative path execute that internal
  `invoke_signed` call?

This does not affect the current enabled entrypoint-path safety claim.

The enabled `process_instruction -> process_consume_gateway_mint` path still
does not reach SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

However, the whole-codebase CPI presence map is not considered complete until
this question is answered and recorded.

### Ignored Mollusk test explanation gap

The current checkpoint records the existence and count of ignored Mollusk tests,
but does not fully explain each ignored test's reason and future disposition.

This remains a documentation / evidence gap inherited from Phase 2.

It does not block this docs-only Phase 3 merge, but it must be resolved before
any complete Mollusk coverage claim.

### Replay and trailing-byte safety clarification

For the current disabled-route state, trailing bytes `194..208` do not weaken
replay protection because replay protection is checked through the processed
event account and canonical event key relationship.

However, if these trailing bytes are later assigned semantics such as
coefficient version, guardian set version, fork identifier, or any other
replay-sensitive field, the replay model must be updated before live route
activation.

No complete replay readiness is claimed by Phase 3.
