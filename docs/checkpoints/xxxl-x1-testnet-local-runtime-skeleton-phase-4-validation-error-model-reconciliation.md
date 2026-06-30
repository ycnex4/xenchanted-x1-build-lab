# XXXL X1 Testnet Local Runtime Skeleton Phase 4 Validation Error Model Reconciliation

Status: Docs-only reconciliation complete - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-4-validation-error-model-reconciliation`

Base:

- `771d085 Add X1 testnet local runtime skeleton phase 3 instruction decode reconciliation`

This checkpoint reconciles the current validation pipeline and error model with
the local runtime skeleton implementation plan. It does not implement runtime
code and does not change tests.

## Inputs Reviewed

Docs read first:

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`

Source and tests inspected without edits:

- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/validation.rs`
- `programs/xxxl-svm/src/error.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/account_contract.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/deployment_status.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

Additional evidence checked for open questions:

- `docs/xxxl/xxxl-production-runtime-byte-layout.md`
- `docs/xxxl/xxxl-runtime-candidate-account-instruction-schema.md`
- `docs/xxxl/xxxl-runtime-instruction-serialization-vectors.md`

## Validation Pipeline Map

Current enabled entrypoint pipeline:

1. `process_instruction` dispatches by calling `XxxlInstruction::unpack`
   before matching on the decoded instruction.
2. `XxxlInstruction::unpack` performs instruction decode checks:
   - exact length `CONSUME_GATEWAY_MINT_INSTRUCTION_LEN = 208`
   - instruction discriminator
   - `INSTRUCTION_LAYOUT_VERSION = 1`
   - encoded account meta count
   - encoded local account index bytes `11..15`
   - fixed-field extraction into `ConsumeGatewayMintArgs`
3. `process_instruction` dispatches
   `XxxlInstruction::ConsumeGatewayMint(args)` to
   `process_consume_gateway_mint`.
4. `process_consume_gateway_mint` obtains `Rent::get()` and `Clock::get()`,
   then calls `build_runtime_consume_gateway_mint_execution_plan_boundary`.
5. `build_runtime_consume_gateway_mint_execution_plan_boundary` calls
   `prepare_consume_gateway_mint_cpi_boundary`.
6. `prepare_consume_gateway_mint_cpi_boundary` checks:
   - account count equals `CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS`
   - decoded `args.account_meta_count` equals
     `CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT`
7. `prepare_consume_gateway_mint_cpi_boundary` calls
   `assert_consume_gateway_mint_account_contract`.
8. `assert_consume_gateway_mint_account_contract` checks:
   - exact account slice length
   - required writable / readonly flags
   - no unexpected signer accounts
9. `prepare_consume_gateway_mint_cpi_boundary` resolves accounts with
   `account_at`:
   - decoded indices for `mint_state`, `gateway_config`, `guardian_set`,
     `processed_event`, and `recipient_balance`
   - fixed processor constants for `spl_token_mint`,
     `recipient_token_account`, `mint_authority_pda`, and `token_program`
10. `prepare_consume_gateway_mint_cpi_boundary` checks token program account
    key equals `spl_token::id()`.
11. `prepare_consume_gateway_mint_cpi_boundary` checks program-owned local
    accounts through:
    - `assert_account_owner`
    - `assert_rent_exempt`
12. `prepare_consume_gateway_mint_cpi_boundary` checks rent exemption for:
    - `spl_token_mint`
    - `recipient_token_account`
13. `prepare_consume_gateway_mint_cpi_boundary` borrows local account data and
    constructs:
    - `MintStateAccountView`
    - `GatewayConfigAccountView`
    - `GuardianSetAccountView`
    - `ProcessedEventAccountView`
    - `RecipientBalanceAccountView`
14. Each local account view calls `assert_account_layout`, which checks:
    - exact account data length
    - local account discriminator
    - `RUNTIME_LAYOUT_VERSION`
15. `prepare_consume_gateway_mint_cpi_boundary` checks Mint State
    relationships:
    - `mint_state.mint_pubkey() == args.mint_id`
    - `mint_state.gateway_mint_authority_pda() == mint_authority_pda.key`
16. `prepare_consume_gateway_mint_cpi_boundary` calls
    `assert_gateway_mint_authority_pda` using the current program ID and the
    bump from `MintStateAccountView`.
17. `prepare_consume_gateway_mint_cpi_boundary` checks Gateway Config
    relationships:
    - route ID
    - guardian set ID
    - target mint
    - source-chain weight bps
18. `prepare_consume_gateway_mint_cpi_boundary` checks Guardian Set
    `guardian_set_id`.
19. `prepare_consume_gateway_mint_cpi_boundary` checks Processed Event:
    - not already consumed
    - canonical event key
    - route ID
    - recipient
20. `prepare_consume_gateway_mint_cpi_boundary` checks Recipient Balance:
    - owner
    - mint
21. `prepare_consume_gateway_mint_cpi_boundary` converts decoded mint and
    recipient bytes into `Pubkey` values.
22. `prepare_consume_gateway_mint_cpi_boundary` calls
    `assert_initialized_mint_account` for the SPL mint.
23. `prepare_consume_gateway_mint_cpi_boundary` calls
    `assert_recipient_ata_boundary` for the recipient SPL token account.
24. `prepare_consume_gateway_mint_cpi_boundary` rejects amount `0` and amounts
    larger than `u64::MAX`.
25. `prepare_consume_gateway_mint_cpi_boundary` returns a prepared
    `MintToCpiBoundary` with amount downcast to `u64`, mint decimals, and
    source-chain weight.
26. `build_runtime_consume_gateway_mint_execution_plan_boundary` calls
    `build_atomic_consume_gateway_mint_execution_plan`.
27. `build_atomic_consume_gateway_mint_execution_plan` rechecks:
    - amount is nonzero
    - amount is within `u64::MAX`
    - prepared boundary amount equals decoded amount
    - prepared source-chain weight equals decoded source-chain weight
28. `build_atomic_consume_gateway_mint_execution_plan` returns an
    `AtomicConsumeGatewayMintExecutionPlan` with:
    - `live_route_activation_enabled = false`
    - `mint_to_invocation_from_process_instruction_enabled = false`
29. `build_runtime_consume_gateway_mint_execution_plan_boundary` rejects only
    if those disabled flags unexpectedly become true.
30. `process_consume_gateway_mint` logs that the execution plan was built and
    returns `Ok(())`.

Current disabled behavior:

- the enabled `process_instruction -> process_consume_gateway_mint` path
  validates and builds a disabled execution plan, then returns successfully
  without local mutation or SPL CPI.
- the enabled entrypoint path does not return a distinct route-disabled error.
- SPL CPI disabled behavior is represented by separate planning/gate
  boundaries, especially `guarded_mint_to_cpi_execution_gate_boundary`, which
  returns `CpiBoundaryNotReady` while `spl_mint_to_cpi_execution_enabled()` is
  false.
- `build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary` is not
  called by the currently enabled `process_instruction` path.

No function name or ordering in the map above is inferred beyond the inspected
source. The exact policy reason for enabled-route success after disabled-plan
construction is a documented scaffold behavior, not a Phase 4 code change.

## Error Model Map

Current error enum:

- `InvalidInstruction = 1`
- `InvalidAccountOwner = 2`
- `InvalidRentExemption = 3`
- `InvalidRecipientAta = 4`
- `InvalidPda = 5`
- `InvalidDiscriminator = 6`
- `InvalidVersion = 7`
- `CpiBoundaryNotReady = 8`

| Validation class | Current error | Evidence | Evidence classification |
| --- | --- | --- | --- |
| wrong instruction discriminator | `InvalidDiscriminator` | `XxxlInstruction::unpack`; `mollusk_rejects_wrong_instruction_discriminator_without_live_route`; `consume_gateway_mint_rejects_wrong_instruction_discriminator` | code-confirmed; Mollusk-covered; lower-level covered |
| wrong instruction version | `InvalidVersion` | `XxxlInstruction::unpack`; `mollusk_rejects_wrong_instruction_version_without_live_route`; `consume_gateway_mint_rejects_wrong_instruction_version` | code-confirmed; Mollusk-covered; lower-level covered |
| wrong instruction length / truncated instruction | `InvalidInstruction` | `XxxlInstruction::unpack`; `mollusk_harness_rejects_malformed_instruction_without_live_route`; `consume_gateway_mint_rejects_wrong_instruction_length` | code-confirmed; Mollusk-covered; lower-level covered |
| extra instruction bytes | `InvalidInstruction` | exact length check in `XxxlInstruction::unpack`; `mollusk_rejects_extra_instruction_bytes_without_live_route`; `consume_gateway_mint_rejects_wrong_instruction_length` | code-confirmed; Mollusk-covered; lower-level covered |
| wrong encoded account meta count | `InvalidInstruction` | `XxxlInstruction::unpack`; `prepare_consume_gateway_mint_cpi_boundary`; `mollusk_rejects_wrong_encoded_account_meta_count_without_live_route` | code-confirmed; Mollusk-covered; lower-level covered |
| wrong encoded account index byte | `InvalidInstruction` | `XxxlInstruction::unpack`; exact Mollusk coverage for processed-event and recipient-balance index bytes only; lower-level unit coverage mutates mint-state index byte | code-confirmed; partial Mollusk coverage; partial lower-level coverage |
| wrong account count | `InvalidInstruction` | `prepare_consume_gateway_mint_cpi_boundary`; `assert_consume_gateway_mint_account_contract`; `mollusk_rejects_wrong_account_count_without_live_route`; `handler_integration_rejects_wrong_account_count` | code-confirmed; Mollusk-covered; lower-level covered |
| wrong signer / unexpected signer | `InvalidInstruction` | `assert_consume_gateway_mint_account_contract`; `mollusk_rejects_unexpected_signer_without_live_route` | code-confirmed; Mollusk-covered |
| wrong writable / readonly flags | `InvalidInstruction` | `assert_consume_gateway_mint_account_contract`; `mollusk_rejects_writable_readonly_mismatch_without_live_route` | code-confirmed; Mollusk-covered |
| wrong program owner | `InvalidAccountOwner` | `assert_account_owner`; `mollusk_rejects_wrong_mint_state_owner_without_live_route`; gateway/guardian owner Mollusk tests; handler integration owner test | code-confirmed; Mollusk-covered; lower-level covered |
| wrong local account discriminator | `InvalidDiscriminator` | `assert_account_layout`; `mollusk_rejects_wrong_mint_state_discriminator_without_live_route`; state account view discriminator tests | code-confirmed; partial Mollusk coverage; lower-level covered |
| wrong local account layout version | `InvalidVersion` | `assert_account_layout`; `account_view_rejects_wrong_version` | code-confirmed; lower-level covered; absent exact Mollusk coverage |
| wrong local account length | `InvalidInstruction` | `assert_account_layout`; `mollusk_rejects_truncated_gateway_config_without_live_route`; state account view truncated-data tests | code-confirmed; partial Mollusk coverage; lower-level covered |
| rent exemption failure | `InvalidRentExemption` | `assert_rent_exempt`; seven low-rent Mollusk tests for local and SPL accounts; validation helper unit tests | code-confirmed; Mollusk-covered; lower-level covered |
| wrong SPL Token program | `InvalidAccountOwner` | `prepare_consume_gateway_mint_cpi_boundary`; `handler_integration_rejects_wrong_spl_token_program_id`; CPI planning token-program test | code-confirmed; lower-level covered; absent exact Mollusk coverage |
| uninitialized SPL mint | `InvalidInstruction` | `assert_initialized_mint_account`; `mollusk_rejects_uninitialized_spl_mint_without_live_route`; validation helper unit test | code-confirmed; Mollusk-covered; lower-level covered |
| wrong mint authority | `InvalidPda` | `assert_initialized_mint_account`; `assert_gateway_mint_authority_pda`; `mollusk_rejects_wrong_spl_mint_authority_without_live_route`; PDA/bump Mollusk tests | code-confirmed; Mollusk-covered; lower-level covered |
| wrong recipient token account owner | `InvalidRecipientAta` | `assert_recipient_ata_boundary`; `mollusk_rejects_wrong_recipient_token_owner_without_live_route`; validation helper unit test | code-confirmed; Mollusk-covered; lower-level covered |
| wrong recipient token account mint | `InvalidRecipientAta` | `assert_recipient_ata_boundary`; `mollusk_rejects_wrong_recipient_token_mint_without_live_route`; validation helper unit test | code-confirmed; Mollusk-covered; lower-level covered |
| zero amount | `InvalidInstruction` | `prepare_consume_gateway_mint_cpi_boundary`; `build_atomic_consume_gateway_mint_execution_plan`; `mollusk_zero_amount_rejection_leaves_mutable_accounts_unchanged`; handler/planning tests | code-confirmed; Mollusk-covered; lower-level covered |
| amount greater than `u64::MAX` | `InvalidInstruction` | `prepare_consume_gateway_mint_cpi_boundary`; `build_atomic_consume_gateway_mint_execution_plan`; `handler_integration_rejects_amount_larger_than_spl_token_u64_range` | code-confirmed; lower-level covered; absent exact Mollusk coverage |
| route ID mismatch | `InvalidInstruction` | Gateway Config route check; Processed Event route check; `mollusk_rejects_wrong_processed_event_route_id_without_live_route`; `handler_integration_rejects_gateway_route_mismatch` | code-confirmed; partial Mollusk coverage; lower-level covered |
| guardian set ID mismatch | `InvalidInstruction` | Gateway Config and Guardian Set checks; handler integration tests for both | code-confirmed; lower-level covered; absent exact Mollusk coverage |
| mint ID mismatch | Mixed: `InvalidInstruction` or `InvalidRecipientAta` depending on failing relationship | Mint State / Gateway Config mismatches return `InvalidInstruction`; Recipient Balance or recipient token account mint mismatches return `InvalidRecipientAta` | code-confirmed; partial Mollusk coverage; partial lower-level coverage |
| processed event already consumed | `InvalidInstruction` | Processed Event check; `mollusk_rejects_consumed_processed_event_replay_without_live_route`; handler and mutation tests | code-confirmed; Mollusk-covered; lower-level covered |
| processed event canonical event key mismatch | `InvalidInstruction` | Processed Event check; `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`; handler test | code-confirmed; Mollusk-covered; lower-level covered |
| processed event route mismatch | `InvalidInstruction` | Processed Event check; `mollusk_rejects_wrong_processed_event_route_id_without_live_route`; handler test | code-confirmed; Mollusk-covered; lower-level covered |
| processed event recipient mismatch | `InvalidInstruction` | Processed Event check; `mollusk_rejects_wrong_processed_event_recipient_without_live_route`; no-mutation Mollusk test; handler test | code-confirmed; Mollusk-covered; lower-level covered |
| recipient balance owner mismatch | `InvalidRecipientAta` | Recipient Balance check; `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`; handler test | code-confirmed; Mollusk-covered; lower-level covered |
| recipient balance mint mismatch | `InvalidRecipientAta` | Recipient Balance check; `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`; handler test | code-confirmed; Mollusk-covered; lower-level covered |
| source-chain weight mismatch | `InvalidInstruction` | Gateway Config source-chain weight check; `handler_integration_rejects_gateway_config_source_chain_weight_mismatch` | code-confirmed; lower-level covered; absent exact Mollusk coverage |
| live route disabled | No distinct error from currently enabled entrypoint; disabled plan returns `Ok(())` after validation | `process_consume_gateway_mint` builds plan and returns success; deployment blocker `LIVE_ROUTE_DISABLED` remains active | code-confirmed; deferred by blocker model |
| SPL CPI disabled | `CpiBoundaryNotReady` in separate disabled CPI gate boundary | `guarded_mint_to_cpi_execution_gate_boundary`; `runtime_disabled_spl_cpi_gate_boundary_rejects_at_gate_without_mutation`; CPI unit gate test | code-confirmed; lower-level covered; not reached by enabled entrypoint |
| CPI boundary not ready | `CpiBoundaryNotReady` | unexpected live/mint flags in processor planning boundaries; disabled CPI gate | code-confirmed; lower-level covered |
| wrong PDA account or bump | `InvalidPda`, except Mint State PDA byte mismatch with account 7 returns `InvalidInstruction` before derivation check | `assert_gateway_mint_authority_pda`; PDA/bump tests; semantic mismatch Mollusk test | code-confirmed; Mollusk-covered; lower-level covered |
| SPL mint unpack failure | `InvalidInstruction` | `assert_initialized_mint_account` maps unpack failure to `InvalidInstruction` | code-confirmed; lower-level helper path present |
| recipient token account unpack failure | `InvalidRecipientAta` | `assert_recipient_ata_boundary` maps unpack failure to `InvalidRecipientAta` | code-confirmed; lower-level helper path present |
| recipient balance arithmetic overflow in local mutation composition | `InvalidInstruction` | `credit_recipient_balance`; local mutation overflow tests | code-confirmed; lower-level covered; not called by enabled entrypoint |
| atomic execution step order mismatch | `InvalidInstruction` | `assert_atomic_consume_gateway_mint_step_order`; execution plan unit tests | code-confirmed; lower-level covered; not called by enabled entrypoint |

## Phase 3 Open Questions Resolved Or Classified

### A. `mint_state.mint_pubkey()` vs `args.mint_id`

Classification:

- check exists; explicit test missing = coverage gap

Evidence:

- `prepare_consume_gateway_mint_cpi_boundary` checks
  `mint_state.mint_pubkey() != args.mint_id` and returns
  `InvalidInstruction` on mismatch.
- exact source location: `programs/xxxl-svm/src/processor.rs:302`.

Coverage status:

- no explicit test named for Mint State `mint_pubkey` mismatch was found.
- related mint mismatch coverage exists for Gateway Config target mint,
  Recipient Balance mint, recipient SPL token account mint, and CPI planning
  mint mapping.

### B. `args.raw` propagation

Classification:

- absent downstream beyond instruction args; no downstream safety consumer found

Evidence:

- `ConsumeGatewayMintArgs` contains `raw`.
- `XxxlInstruction::unpack` copies all 208 input bytes into `raw`.
- `consume_gateway_mint_instruction_parses_known_layout` asserts `args.raw`
  equals the original bytes.
- search for `.raw` under `programs/xxxl-svm/src` and
  `programs/xxxl-svm/tests` found no downstream `args.raw` consumer beyond
  the instruction unit assertion.

Specific propagation checks:

- `AtomicConsumeGatewayMintExecutionPlan`: `args.raw` absent
- CPI boundary structs / functions: `args.raw` absent
- account mutation paths: `args.raw` absent
- logs / telemetry / messages: `args.raw` absent
- downstream consumer: none found during this inspection

Safety implication:

- trailing bytes are preserved only in decoded args, but the current enabled
  processor path uses named decoded fields instead.
- this does not decide future semantics for trailing bytes.

### C. Bytes `194..208`

Classification:

- reserved / unparsed / not zero-validated
- production semantics not decided
- live route must not be enabled before decision

Evidence:

- instruction length is exactly `208`.
- bytes `192..194` decode source-chain weight.
- no named field reads bytes `194..208`.
- bytes `194..208` are included only because `raw.copy_from_slice(input)`
  copies the whole input.

Current error model:

- there is no current error for non-zero trailing bytes inside an otherwise
  valid 208-byte instruction.
- extra bytes beyond 208 return `InvalidInstruction`.

### D. `u128` amount field with `u64` valid range

Classification:

- rationale absent / design gap

Evidence:

- code decodes amount as `u128`.
- code rejects zero and values greater than `u64::MAX`.
- `docs/xxxl/xxxl-production-runtime-byte-layout.md` records a global rule
  that `u128` fields are 16-byte aligned and that the instruction includes an
  amount plus reserved padding.
- account-contract review docs mention SPL Token amount compatibility and
  `u128` to `u64` overflow checks.

Assessment:

- current documents explain a 16-byte `u128` layout convention and the SPL
  Token `u64` compatibility check.
- they do not state a final rationale for why this instruction encodes amount
  as `u128` while accepting only the `u64` range.
- no rationale is invented by Phase 4.

### E. `mint_to_cpi_boundary` / `invoke_signed`

Answers:

- `mint_to_cpi_boundary` contains an internal `invoke_signed` call.
- exact source location: `programs/xxxl-svm/src/cpi.rs:205`.
- one unit test directly calls `mint_to_cpi_boundary`:
  `mint_to_boundary_rejects_wrong_pda_before_invoke_signed`.
- exact test location: `programs/xxxl-svm/src/cpi.rs:817`.
- that test passes a wrong PDA and fails at `assert_gateway_mint_authority_pda`
  before instruction construction and before the internal `invoke_signed`.
- `mint_to_cpi_boundary` is not reachable from the currently enabled
  `process_instruction -> process_consume_gateway_mint` path.

Whole-codebase vs enabled-entrypoint distinction:

- whole codebase: dormant `mint_to_cpi_boundary` source exists and contains
  `invoke_signed`; a unit test calls it on a negative pre-invoke path.
- enabled entrypoint path: does not reach SPL CPI, `mint_to_cpi_boundary`,
  `invoke_signed`, or SPL Token `mint_to`.

### F. Ignored Mollusk Tests

All 10 ignored Mollusk tests have the visible ignore reason:

- `requires cargo build-sbf and target/deploy/xxxl_svm.so`

List:

| Line | Test | Visible reason | Future disposition |
| --- | --- | --- | --- |
| 736 | `invalid_consume_gateway_mint_account_count_rejects_before_live_route` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 757 | `invalid_consume_gateway_mint_readonly_account_passed_writable_rejects_before_validation` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 777 | `invalid_consume_gateway_mint_required_writable_account_passed_readonly_rejects_before_validation` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 798 | `invalid_consume_gateway_mint_unexpected_signer_rejects_before_validation` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 818 | `invalid_consume_gateway_mint_wrong_program_account_owner_rejects_before_live_route` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 1042 | `invalid_consume_gateway_mint_wrong_recipient_token_owner_rejects_before_live_route` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 1065 | `invalid_consume_gateway_mint_zero_amount_rejects_before_live_route` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 1090 | `invalid_consume_gateway_mint_length_rejects_before_scaffold_path` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 1112 | `invalid_consume_gateway_mint_discriminator_rejects_before_scaffold_path` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |
| 1134 | `invalid_consume_gateway_mint_version_rejects_before_scaffold_path` | `requires cargo build-sbf and target/deploy/xxxl_svm.so` | unresolved evidence; future SBF artifact/harness stage |

No additional per-test reason or disposition comment was found next to these
ignored tests beyond the shared ignore attribute.

## Coverage / Gap Classification

| Area | Current status | Evidence | Classification |
| --- | --- | --- | --- |
| `mint_state.mint_pubkey()` vs `args.mint_id` | check exists, explicit test missing | `prepare_consume_gateway_mint_cpi_boundary` line 302 | coverage gap |
| `args.raw` downstream propagation | no downstream consumer found beyond instruction unit assertion | `.raw` search found only `instruction.rs` unit assertion | covered |
| bytes `194..208` non-zero trailing bytes | accepted if total length is 208 and earlier fields are valid | no named reader or zero check found | design gap |
| `u128` amount rationale | `u128` layout convention and `u64` compatibility checks exist, final rationale absent | byte-layout docs and current code | design gap |
| amount overflow Mollusk coverage | lower-level handler coverage only | `handler_integration_rejects_amount_larger_than_spl_token_u64_range` | coverage gap |
| encoded route index independent coverage | no exact targeted variant found | Phase 3 decode coverage review | coverage gap |
| encoded guardian set index independent coverage | no exact targeted variant found | Phase 3 decode coverage review | coverage gap |
| encoded mint state index Mollusk coverage | lower-level unit exists, no exact non-ignored Mollusk variant | `consume_gateway_mint_rejects_wrong_account_index_boundary` | coverage gap |
| guardian set ID mismatch Mollusk coverage | lower-level handler coverage only | `handler_integration_rejects_gateway_config_guardian_set_id_mismatch`; `handler_integration_rejects_wrong_guardian_set_id` | coverage gap |
| source-chain weight mismatch Mollusk coverage | lower-level handler coverage only | `handler_integration_rejects_gateway_config_source_chain_weight_mismatch` | coverage gap |
| ignored Mollusk tests explanation | shared ignore reason visible, future disposition not settled | 10 ignored tests list above | evidence gap |
| `mint_to_cpi_boundary` / `invoke_signed` unit-test behavior | direct test fails before `invoke_signed`; enabled entrypoint unreachable | `cpi.rs:185..215`, `cpi.rs:817..894` | covered |
| live route disabled error path | enabled entrypoint returns `Ok(())` after disabled plan; no distinct route-disabled runtime error | `process_consume_gateway_mint`; deployment blocker active | deferred by blocker model |
| SPL CPI disabled error path | separate disabled SPL CPI gate returns `CpiBoundaryNotReady`; not called by enabled entrypoint | `guarded_mint_to_cpi_execution_gate_boundary`; processor/cpi tests | covered |
| complete replay readiness | processed-event replay covered, version/pause/upgrade/source-fork replay deferred | Phase 2 and Phase 3 deferrals | deferred by blocker model |
| production readiness | not claimed and blockers remain active | deployment status and checkpoint safety boundary | not applicable while disabled |

## Safety Boundary

Phase 4 is docs-only.

Phase 4 made no runtime code changes.

Phase 4 changed no tests.

Phase 4 did not deploy or upgrade.

Phase 4 did not submit transactions.

Phase 4 did not spend SOL.

Phase 4 did not touch `.local-keys/**`.

Phase 4 did not touch keypair JSON files.

Phase 4 did not touch `.env` files.

Phase 4 did not touch `target/deploy/**`.

Phase 4 did not add or commit `.so` artifacts.

Phase 4 did not add deployment scripts.

Phase 4 did not add upgrade scripts.

Phase 4 did not add CI/CD workflows that deploy, upgrade, submit transactions,
or spend SOL.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

`PRODUCTION_PROGRAM_ID_UNSET` remains active.

`PRODUCTION_GUARDIAN_SET_UNSET` remains active.

`PRODUCTION_PROOF_LOG_UNSET` remains active.

`EXTERNAL_REVIEW_INCOMPLETE` remains active.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

The runtime remains non-deployed, non-live, and unable to mint through the
currently enabled executable entrypoint path.

The whole codebase still contains dormant CPI helper code and a unit-test
negative path for `mint_to_cpi_boundary`.

The currently enabled executable entrypoint path still does not reach SPL CPI,
`invoke_signed`, or SPL Token `mint_to`.

## Recommended Next Stage

The implementation plan defines Phase 5 as Stage 1 authorization consumer
modeling. Therefore the recommended next docs-only stage is:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-5-stage-1-authorization-consumer-modeling`

Before any runtime implementation stage, resolve or explicitly defer the Phase
4 design and coverage gaps above, especially trailing bytes `194..208`, the
`u128` amount rationale, the live-route disabled error semantics, and exact
Mollusk coverage for the listed decode / relationship cases.

## External Audit Minor Notes Addressed

External audit result:

- `ACCEPT WITH MINOR NOTES`
- no blocking issues found
- Phase 4 remains safe to merge as a docs-only reconciliation stage
- all runtime blockers remain active

### Live-route-disabled `Ok(())` decision gate

The current enabled entrypoint validates input, builds a disabled execution
plan, and returns `Ok(())` without minting.

This is recorded as current scaffold behavior, not as production behavior.

Before Phase 5 closure or any implementation stage that changes enabled
runtime behavior, the project must explicitly decide and record one of:

- scaffold `Ok(())` is intentional while the live route is disabled and will be
  replaced only when live route activation is intentionally introduced
- a distinct route-disabled error should be returned before live route
  activation work begins

Until that decision is recorded, live-route-disabled error semantics remain
deferred by the blocker model.

### Mint ID mismatch error uniformity design question

Current mint ID mismatch errors are mixed by relationship source:

- Mint State / Gateway Config mint relationship failures return
  `InvalidInstruction`
- Recipient Balance / recipient SPL token account mint relationship failures
  return `InvalidRecipientAta`

This may be intentional because recipient account failures are classified under
recipient ATA validation, while local route/config/mint-state relationship
failures are classified as invalid instruction relationships.

However, Phase 4 does not decide whether this mixed mapping is final API
design or a design inconsistency.

Classification:

- `mint ID mismatch error uniformity`: design gap

Before production error semantics are finalized, the project must decide whether
mint mismatch should keep relationship-specific error codes or use a more
uniform error mapping.

### CPI and live-route continuity

Whole-codebase statement:

- dormant CPI helper code exists
- `mint_to_cpi_boundary` contains `invoke_signed`
- a unit test directly calls `mint_to_cpi_boundary` on a wrong-PDA negative path
- that negative path fails before the internal `invoke_signed` call

Enabled-entrypoint-path statement:

- the currently enabled `process_instruction -> process_consume_gateway_mint`
  path does not reach SPL CPI
- it does not reach `mint_to_cpi_boundary`
- it does not reach `invoke_signed`
- it does not reach SPL Token `mint_to`

Phase 4 does not make dormant CPI helpers reachable.

### Bytes `194..208` implementation gate

Bytes `194..208` are currently reserved / unparsed / not zero-validated.

No code may read, interpret, or validate bytes `194..208` as named production
fields until their semantics are explicitly decided in a separate boundary
update.

This decision must occur before any implementation stage that touches
instruction parsing for those bytes, not merely before live route activation.

Allowed future decisions include:

- keep bytes `194..208` reserved and unconstrained, with a documented rationale
- require bytes `194..208` to be zero-checked reserved bytes, with rejection
  tests for non-zero trailing bytes
- assign explicit production semantics to those bytes, with replay / canonical
  event key implications reviewed before implementation

Until this decision exists, bytes `194..208` remain a design gap.

### Ignored Mollusk test resolution path

The 10 ignored Mollusk tests share the visible reason:

- `requires cargo build-sbf and target/deploy/xxxl_svm.so`

Before the first code implementation stage or before any complete Mollusk/SVM
coverage checkpoint, the project must decide whether these tests should be:

- run in a future SBF artifact / harness stage with `cargo build-sbf`
- converted to a non-ignored Mollusk-compatible harness
- replaced by equivalent non-ignored coverage with explicit rationale

Until then, ignored Mollusk test disposition remains an evidence gap.

### Defensive amount and source-chain-weight invariant

Phase 4 records two layers of amount / source-chain-weight validation:

- `prepare_consume_gateway_mint_cpi_boundary` validates decoded amount bounds
  and prepares a boundary with the amount downcast to `u64`
- `build_atomic_consume_gateway_mint_execution_plan` rechecks that the prepared
  boundary amount and source-chain weight match the original decoded values

This second check is a defensive invariant assertion.

If prepared boundary values ever diverge from decoded instruction values, that
indicates a logic error in boundary construction rather than a normal user
input error path.

### Wrong SPL Token program error mapping design question

The current wrong SPL Token program account key maps to `InvalidAccountOwner`.

This is code-confirmed in Phase 4, but may be counterintuitive because the token
program account is checked by key equality rather than by owner.

Phase 4 does not decide whether this error mapping is final API design.

Classification:

- `wrong SPL Token program error semantics`: design question / future API
  consistency review item

### Mollusk/SVM coverage checkpoint criteria remain undefined

Phase 4 does not define complete Mollusk/SVM coverage checkpoint criteria.

Before any on-chain upgrade or live-route activation readiness claim, a future
checkpoint must define which exact Mollusk/SVM tests are mandatory, including
at minimum the Phase 4 coverage gaps:

- Mint State mint mismatch
- encoded route account index byte `11`
- encoded guardian set account index byte `12`
- encoded mint state account index byte `13`
- amount overflow
- guardian set ID mismatch
- source-chain weight mismatch
- replay-sensitive version / pause / upgrade / source-fork scenarios
