# XXXL X1 Testnet Local Runtime Skeleton Phase 2 Account Layout Reconciliation

Status: Docs-only reconciliation complete — all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-2-account-layout-reconciliation`

Base:

- `f5dd2b6 Add X1 testnet local runtime skeleton phase 1 inventory`

This checkpoint reconciles the Phase 2 account layout objective with the
runtime skeleton already present in `programs/xxxl-svm/src/**` and the current
Mollusk tests. It does not implement runtime code.

## Inputs Reviewed

- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-1-inventory.md`
- `docs/xxxl/xxxl-x1-testnet-local-runtime-skeleton-implementation-plan.md`
- `docs/xxxl/xxxl-x1-testnet-runtime-upgrade-implementation-boundary.md`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/account_contract.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/execution_plan.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/validation.rs`
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

## Summary

The current skeleton already has concrete local account views for Mint State,
Gateway Config, Guardian Set, Processed Event, and Recipient Balance. Each view
uses a fixed discriminator, fixed runtime layout version, and exact byte length.
The enabled processor boundary consumes these views through
`prepare_consume_gateway_mint_cpi_boundary`, after account contract checks,
program-owner checks for program-owned accounts, rent checks, SPL account
validation, PDA validation, and relationship checks.

This stage made no runtime code changes and changed no tests.

The runtime remains non-deployed, non-live, and unable to mint through the
currently enabled executable entrypoint path.

## Common Account Layout Rules

- Layout discriminators are defined in `state.rs`.
- Account lengths are defined in `state.rs`.
- `assert_account_layout` enforces exact length, discriminator, and
  `RUNTIME_LAYOUT_VERSION`.
- The current local layouts do not have a separate serialized `kind` field.
  The discriminator is the effective account-kind identity.
- `account_contract.rs` defines the 9-account `consume_gateway_mint` contract
  and enforces account count, writable flags, and no external signer accounts.
- `account_contract.rs` records owner models, but owner enforcement happens
  later in `processor.rs` and `validation.rs`.
- `prepare_consume_gateway_mint_cpi_boundary` consumes all five local account
  views before returning a prepared boundary.
- The enabled `process_instruction` route builds a disabled execution plan and
  returns without state mutation, SPL CPI, `invoke_signed`, or SPL Token
  `mint_to`.

## Mint State

- Current source representation:
  - `state.rs`: `MINT_STATE_DISCRIMINATOR`, `MINT_STATE_ACCOUNT_LEN`,
    `MintStateAccountView`
  - `account_contract.rs`: account index 0, readonly, non-signer,
    program-owned model
  - `processor.rs`: loaded from `args.mint_state_account_index`
- Discriminator checks:
  - enforced by `MintStateAccountView::new`
- Version checks:
  - enforced by `assert_account_layout` against `RUNTIME_LAYOUT_VERSION`
- Kind checks:
  - no separate kind field; discriminator is the current type identity
- Owner checks:
  - `prepare_consume_gateway_mint_cpi_boundary` requires the account owner to
    equal `program_id`
- Length checks:
  - exact `MINT_STATE_ACCOUNT_LEN`
- Relationship checks:
  - `mint_state.mint_pubkey()` must equal `args.mint_id`
  - `mint_state.gateway_mint_authority_pda()` must equal account 7
  - `mint_state.gateway_mint_authority_bump()` must derive the gateway mint
    authority PDA for the active `program_id`
  - SPL mint authority must match the same PDA through SPL mint validation
- Processor/validation path consumes it:
  - `prepare_consume_gateway_mint_cpi_boundary`
  - `assert_gateway_mint_authority_pda`
  - `assert_initialized_mint_account`
- Mollusk tests cover:
  - `mollusk_rejects_wrong_mint_state_owner_without_live_route`
  - `mollusk_rejects_wrong_mint_state_discriminator_without_live_route`
  - `mollusk_rejects_low_rent_mint_state_without_live_route`
  - `mollusk_rejects_wrong_mint_authority_pda_without_live_route`
  - `mollusk_rejects_wrong_mint_authority_bump_without_live_route`
  - `mollusk_rejects_mint_authority_pda_for_wrong_program_id_without_live_route`
  - `mollusk_rejects_mint_authority_pda_semantic_mismatch_without_live_route`
- Lower-level coverage also exists for:
  - handler integration mint authority and bump rejection in `processor.rs`
  - state-view parsing in `state.rs`
- Gaps/deferrals:
  - no separate serialized kind field exists to test independently
  - live mint supply mutation remains disabled and out of scope

## Gateway Config

- Current source representation:
  - `state.rs`: `GATEWAY_CONFIG_DISCRIMINATOR`,
    `GATEWAY_CONFIG_ACCOUNT_LEN`, `GatewayConfigAccountView`
  - `account_contract.rs`: account index 1, readonly, non-signer,
    program-owned model
  - `processor.rs`: loaded from `args.route_account_index`
- Discriminator checks:
  - enforced by `GatewayConfigAccountView::new`
- Version checks:
  - enforced by `assert_account_layout` against `RUNTIME_LAYOUT_VERSION`
- Kind checks:
  - no separate kind field; discriminator is the current type identity
- Owner checks:
  - `prepare_consume_gateway_mint_cpi_boundary` requires the account owner to
    equal `program_id`
- Length checks:
  - exact `GATEWAY_CONFIG_ACCOUNT_LEN`
- Relationship checks:
  - `gateway_config.route_id()` must equal `args.route_id`
  - `gateway_config.guardian_set_id()` must equal `args.guardian_set_id`
  - `gateway_config.target_mint()` must equal `args.mint_id`
  - `gateway_config.source_chain_weight_bps()` must equal
    `args.source_chain_weight_bps`
- Processor/validation path consumes it:
  - `prepare_consume_gateway_mint_cpi_boundary`
  - `build_atomic_consume_gateway_mint_execution_plan`
- Mollusk tests cover:
  - `mollusk_rejects_wrong_gateway_config_owner_without_live_route`
  - `mollusk_rejects_truncated_gateway_config_without_live_route`
  - `mollusk_rejects_low_rent_gateway_config_without_live_route`
- Lower-level coverage also exists for:
  - `handler_integration_rejects_gateway_route_mismatch`
  - `handler_integration_rejects_gateway_config_guardian_set_id_mismatch`
  - `handler_integration_rejects_gateway_config_target_mint_mismatch`
  - `handler_integration_rejects_gateway_config_source_chain_weight_mismatch`
  - state-view parsing in `state.rs`
- Gaps/deferrals:
  - current Mollusk coverage does not separately exercise all Gateway Config
    relationship-field mismatches
  - route coefficient/version replay policy is not represented in this account
    layout yet and remains deferred

## Guardian Set

- Current source representation:
  - `state.rs`: `GUARDIAN_SET_DISCRIMINATOR`,
    `GUARDIAN_SET_ACCOUNT_LEN`, `GuardianSetAccountView`
  - `account_contract.rs`: account index 2, readonly, non-signer,
    program-owned model
  - `processor.rs`: loaded from `args.guardian_set_account_index`
- Discriminator checks:
  - enforced by `GuardianSetAccountView::new`
- Version checks:
  - enforced by `assert_account_layout` against `RUNTIME_LAYOUT_VERSION`
- Kind checks:
  - no separate kind field; discriminator is the current type identity
- Owner checks:
  - `prepare_consume_gateway_mint_cpi_boundary` requires the account owner to
    equal `program_id`
- Length checks:
  - exact `GUARDIAN_SET_ACCOUNT_LEN`
- Relationship checks:
  - `guardian_set.guardian_set_id()` must equal `args.guardian_set_id`
- Processor/validation path consumes it:
  - `prepare_consume_gateway_mint_cpi_boundary`
- Mollusk tests cover:
  - `mollusk_rejects_wrong_guardian_set_owner_without_live_route`
  - `mollusk_rejects_low_rent_guardian_set_without_live_route`
- Lower-level coverage also exists for:
  - `handler_integration_rejects_wrong_guardian_set_id`
  - state-view parsing in `state.rs`
- Gaps/deferrals:
  - current Mollusk coverage does not separately exercise guardian set ID
    mismatch
  - guardian set version replay rejection is not represented yet and remains
    deferred

## Processed Event

- Current source representation:
  - `state.rs`: `PROCESSED_EVENT_DISCRIMINATOR`,
    `PROCESSED_EVENT_ACCOUNT_LEN`, `ProcessedEventAccountView`
  - `account_contract.rs`: account index 3, writable, non-signer,
    program-owned model
  - `processor.rs`: loaded from `args.processed_event_account_index`
  - `execution_plan.rs`: local mutation helper can mark the event consumed in
    disabled composition tests
- Discriminator checks:
  - enforced by `ProcessedEventAccountView::new`
- Version checks:
  - enforced by `assert_account_layout` against `RUNTIME_LAYOUT_VERSION`
- Kind checks:
  - no separate kind field; discriminator is the current type identity
- Owner checks:
  - `prepare_consume_gateway_mint_cpi_boundary` requires the account owner to
    equal `program_id`
- Length checks:
  - exact `PROCESSED_EVENT_ACCOUNT_LEN`
- Relationship checks:
  - `processed_event.consumed()` must be false
  - `processed_event.canonical_event_key()` must equal
    `args.canonical_event_key`
  - `processed_event.route_id()` must equal `args.route_id`
  - `processed_event.recipient()` must equal `args.recipient`
- Processor/validation path consumes it:
  - `prepare_consume_gateway_mint_cpi_boundary`
  - disabled local mutation composition can call `mark_processed_event_consumed`
    outside the enabled entrypoint route
- Mollusk tests cover:
  - `mollusk_rejects_low_rent_processed_event_without_live_route`
  - `mollusk_consumed_processed_event_rejection_leaves_mutable_accounts_unchanged`
  - `mollusk_rejects_consumed_processed_event_replay_without_live_route`
  - `mollusk_rejects_wrong_processed_event_canonical_event_key_without_live_route`
  - `mollusk_rejects_wrong_processed_event_route_id_without_live_route`
  - `mollusk_rejects_wrong_processed_event_recipient_without_live_route`
  - `mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged`
- Lower-level coverage also exists for:
  - handler integration processed-event relationship rejections in
    `processor.rs`
  - replay and mutation rejection tests in `state.rs` and `execution_plan.rs`
- Gaps/deferrals:
  - current coverage confirms local processed-event replay rejection, but not
    coefficient version replay, guardian set version replay, pause/unpause
    replay, upgrade replay, or source fork replay
  - enabled entrypoint does not mutate the processed event

## Recipient Balance

- Required clarification:
  - Recipient Balance is local model-level accounting only.
  - Recipient Balance is not an SPL Token account.
  - Recipient Balance is not the recipient ATA.
  - The actual recipient token balance is managed by the recipient SPL token
    account or ATA, outside this Phase 2 checkpoint.
- Current source representation:
  - `state.rs`: `RECIPIENT_BALANCE_DISCRIMINATOR`,
    `RECIPIENT_BALANCE_ACCOUNT_LEN`, `RecipientBalanceAccountView`
  - `account_contract.rs`: account index 4, writable, non-signer,
    program-owned model
  - `processor.rs`: loaded from `args.recipient_balance_account_index`
  - `execution_plan.rs`: local mutation helper can credit this model account
    in disabled composition tests
- Discriminator checks:
  - enforced by `RecipientBalanceAccountView::new`
- Version checks:
  - enforced by `assert_account_layout` against `RUNTIME_LAYOUT_VERSION`
- Kind checks:
  - no separate kind field; discriminator is the current type identity
- Owner checks:
  - `prepare_consume_gateway_mint_cpi_boundary` requires the account owner to
    equal `program_id`
- Length checks:
  - exact `RECIPIENT_BALANCE_ACCOUNT_LEN`
- Relationship checks:
  - `recipient_balance.owner()` must equal `args.recipient`
  - `recipient_balance.mint()` must equal `args.mint_id`
- Processor/validation path consumes it:
  - `prepare_consume_gateway_mint_cpi_boundary`
  - disabled local mutation composition can call `credit_recipient_balance`
    outside the enabled entrypoint route
- Mollusk tests cover:
  - `mollusk_rejects_low_rent_recipient_balance_without_live_route`
  - `mollusk_rejects_wrong_recipient_balance_owner_without_live_route`
  - `mollusk_rejects_wrong_recipient_balance_mint_without_live_route`
  - `mollusk_valid_scaffold_entrypoint_leaves_mutable_accounts_unchanged`
- Lower-level coverage also exists for:
  - handler integration recipient-balance binding rejections in `processor.rs`
  - local mutation and overflow tests in `state.rs` and `execution_plan.rs`
- Gaps/deferrals:
  - enabled entrypoint does not credit Recipient Balance
  - SPL token settlement remains outside this local model account

## Related SPL Accounts and PDA

Although Phase 2 focuses on the five local account layouts, the enabled
processor boundary also validates the adjacent SPL/PDA accounts:

- `spl_token_mint`:
  - account index 5
  - writable, non-signer, SPL Token-owned model
  - rent checked
  - owner and initialized mint checked by `assert_initialized_mint_account`
  - mint authority must equal the gateway mint authority PDA
  - Mollusk covers low rent, wrong owner, wrong authority, and uninitialized
    mint
- `recipient_token_account`:
  - account index 6
  - writable, non-signer, SPL Token-owned model
  - rent checked
  - owner, mint, and initialized token state checked by
    `assert_recipient_ata_boundary`
  - Mollusk covers low rent, wrong mint, wrong owner, and uninitialized token
    account
- `mint_authority_pda`:
  - account index 7
  - readonly, non-signer, program-derived-address model
  - checked through deterministic PDA derivation and bump validation
  - Mollusk covers wrong PDA, wrong bump, wrong program ID derivation, and
    semantic mismatch
- `token_program`:
  - account index 8
  - readonly, non-signer, SPL Token program model
  - must equal `spl_token::id()`
  - handler integration covers wrong token program ID

These validations prepare a boundary only. They do not make SPL CPI reachable
from the enabled entrypoint.

## Phase 1 Follow-Up Findings

### Dormant CPI helper call sites

`cpi.rs` currently defines dormant/future-boundary helpers:

- `plan_mint_to_cpi_boundary`
- `spl_mint_to_cpi_execution_enabled`
- `guarded_mint_to_cpi_execution_gate_boundary`
- `build_mint_to_instruction`
- `mint_to_cpi_boundary`

Call-site review found:

- `processor.rs` imports the planning and guarded execution helpers.
- `build_runtime_consume_gateway_mint_planning_composition_boundary` calls
  `plan_mint_to_cpi_boundary`.
- `build_runtime_consume_gateway_mint_disabled_spl_cpi_gate_boundary` calls
  `guarded_mint_to_cpi_execution_gate_boundary`.
- The enabled `process_instruction -> process_consume_gateway_mint` path does
  not call either composition boundary.
- `guarded_mint_to_cpi_execution_gate_boundary` returns
  `CpiBoundaryNotReady` while `spl_mint_to_cpi_execution_enabled()` is false,
  before it can call `mint_to_cpi_boundary`.
- `deployment_status.rs` reads `spl_mint_to_cpi_execution_enabled()` for
  blocker reporting.
- `cpi.rs` unit tests directly call planning, guard, instruction-building, and
  one direct `mint_to_cpi_boundary` negative path.
- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs` does not directly
  call the dormant CPI helpers.
- No `benches`, `examples`, or utility directories were found under
  `programs/xxxl-svm` during this review.

Finding: dormant CPI helpers have source-level and unit-test call sites outside
the enabled `process_instruction` route, but no reviewed enabled entrypoint path
currently reaches SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

### Mollusk test count and latest run

Current Mollusk integration file:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`
- `#[test]` entries: 54
- ignored tests: 10
- non-ignored tests: 44

Latest local command checked during this stage:

```bash
RUST_LOG=off cargo test --manifest-path programs/xxxl-svm/Cargo.toml --test mollusk_consume_gateway_mint -- --format terse
```

Result:

- 44 passed
- 0 failed
- 10 ignored
- 0 measured
- 0 filtered out

An additional filtered Mollusk run was also checked:

```bash
cargo test --manifest-path programs/xxxl-svm/Cargo.toml mollusk
```

Result observed:

- unit filter: 3 passed, 0 failed
- integration filter: 44 passed, 0 failed, 10 filtered out
- total filtered run: 47 passed, 0 failed

### Replay coverage

Confirmed current coverage:

- consumed processed-event replay rejection
- wrong processed-event canonical event key
- wrong processed-event route ID
- wrong processed-event recipient
- lower-level processed-event mutation replay rejection before balance credit

Deferred replay coverage:

- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay rejection

These deferred replay dimensions remain future skeleton or implementation-plan
work. They are not closed by this docs-only reconciliation checkpoint.

### Execution-plan boundary return structure

`build_runtime_consume_gateway_mint_execution_plan_boundary` does more than
report disabled-route and disabled-CPI flags. It validates and returns an
`AtomicConsumeGatewayMintExecutionPlan` containing:

- ordered steps:
  - `ValidateAndPrepareCpi`
  - `MarkProcessedEventConsumed`
  - `CreditRecipientBalance`
  - `KeepLiveRouteDisabled`
- `canonical_event_key`
- `route_id`
- `recipient`
- `mint`
- `amount` converted to `u64`
- `consumed_slot`
- `source_chain_weight_bps`
- `live_route_activation_enabled = false`
- `mint_to_invocation_from_process_instruction_enabled = false`

The enabled entrypoint builds this plan and returns. It does not execute local
mutation steps and does not execute SPL CPI.

## Safety Confirmation

Phase 2 made no runtime code changes.

Phase 2 changed no tests.

Phase 2 did not edit `programs/xxxl-svm/src/**`.

Phase 2 did not edit `programs/xxxl-svm/tests/**`.

Phase 2 did not deploy or upgrade.

Phase 2 did not submit transactions.

Phase 2 did not spend SOL.

Phase 2 did not touch `.local-keys/**`, keypair JSON files, `.env`,
`target/deploy/**`, or `.so` artifacts.

Phase 2 did not add deployment scripts, upgrade scripts, or CI/CD workflows
that deploy, upgrade, submit transactions, or spend SOL.

No blocker was removed.

No production readiness is claimed.

No immutability is claimed while upgrade authority exists.

The runtime remains non-deployed, non-live, and unable to mint through the currently enabled executable entrypoint path.

`LIVE_ROUTE_DISABLED` remains active.

`SPL_CPI_EXECUTION_DISABLED` remains active.

Dormant CPI helpers have source-level and unit-test call sites, but remain unreachable from the currently enabled executable entrypoint path.

`invoke_signed` remains absent from the currently enabled executable
`process_instruction` path.

SPL Token `mint_to` remains absent from the currently enabled executable
`process_instruction` path.

## Active Blockers

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

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-3-instruction-decode-reconciliation`

Before any future runtime implementation stage, separately decide whether to
add explicit Mollusk coverage for the Gateway Config and Guardian Set
relationship mismatches and how to model the deferred replay dimensions:
coefficient version, guardian set version, pause/unpause, upgrade, and source
fork replay.

## External Audit Minor Notes Addressed

A strict external audit returned `ACCEPT WITH MINOR NOTES`.

The following clarifications address those notes before commit.

### CPI helper and `invoke_signed` clarification

`mint_to_cpi_boundary` body contains `invoke_signed` internally:

- `true`

This does not change the enabled-entrypoint safety claim.

The enabled `process_instruction -> process_consume_gateway_mint` path still does not reach SPL CPI, `invoke_signed`, or SPL Token `mint_to`.

The whole codebase does contain dormant CPI helper code and unit-test/source call sites.

Current `mint_to_cpi_boundary` call-site map:

- `programs/xxxl-svm/src/cpi.rs:39` — call-or-reference: `pub fn plan_mint_to_cpi_boundary(`
- `programs/xxxl-svm/src/cpi.rs:116` — call-or-reference: `plan_mint_to_cpi_boundary(program_id, execution_plan, boundary)?;`
- `programs/xxxl-svm/src/cpi.rs:126` — call-or-reference: `mint_to_cpi_boundary(`
- `programs/xxxl-svm/src/cpi.rs:185` — definition: `pub fn mint_to_cpi_boundary(`
- `programs/xxxl-svm/src/cpi.rs:235` — call-or-reference: `fn with_mint_to_cpi_boundary_fixture<T>(`
- `programs/xxxl-svm/src/cpi.rs:344` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:356` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)`
- `programs/xxxl-svm/src/cpi.rs:380` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:392` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)`
- `programs/xxxl-svm/src/cpi.rs:417` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:429` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)`
- `programs/xxxl-svm/src/cpi.rs:463` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:495` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:506` — call-or-reference: `let plan = plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)`
- `programs/xxxl-svm/src/cpi.rs:528` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:540` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:554` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:566` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:581` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:593` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:608` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:620` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:634` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:646` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:660` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:672` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:686` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:698` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:712` — call-or-reference: `with_mint_to_cpi_boundary_fixture(`
- `programs/xxxl-svm/src/cpi.rs:724` — call-or-reference: `plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),`
- `programs/xxxl-svm/src/cpi.rs:889` — call-or-reference: `let result = mint_to_cpi_boundary(&program_id, boundary);`
- `programs/xxxl-svm/src/processor.rs:10` — call-or-reference: `plan_mint_to_cpi_boundary, MintToCpiAccounts, MintToCpiBoundary, MintToCpiPlanningBoundary,`
- `programs/xxxl-svm/src/processor.rs:132` — call-or-reference: `plan_mint_to_cpi_boundary(program_id, &execution_plan, &prepared.boundary)?;`

Current `invoke_signed` source map:

- `programs/xxxl-svm/src/cpi.rs:2` — `account_info::AccountInfo, instruction::Instruction, program::invoke_signed,`
- `programs/xxxl-svm/src/cpi.rs:36` — `pub invoke_signed_from_process_instruction_enabled: bool,`
- `programs/xxxl-svm/src/cpi.rs:93` — `invoke_signed_from_process_instruction_enabled: false,`
- `programs/xxxl-svm/src/cpi.rs:110` — `|| planning_boundary.invoke_signed_from_process_instruction_enabled`
- `programs/xxxl-svm/src/cpi.rs:205` — `invoke_signed(`
- `programs/xxxl-svm/src/cpi.rs:460` — `invoke_signed_from_process_instruction_enabled: false,`
- `programs/xxxl-svm/src/cpi.rs:488` — `fn mint_to_cpi_planning_boundary_builds_plan_without_invoke_signed() {`
- `programs/xxxl-svm/src/cpi.rs:516` — `assert!(!plan.invoke_signed_from_process_instruction_enabled);`
- `programs/xxxl-svm/src/cpi.rs:817` — `fn mint_to_boundary_rejects_wrong_pda_before_invoke_signed() {`
- `programs/xxxl-svm/src/processor.rs:54` — `pub invoke_signed_from_process_instruction_enabled: bool,`
- `programs/xxxl-svm/src/processor.rs:61` — `pub invoke_signed_from_process_instruction_enabled: bool,`
- `programs/xxxl-svm/src/processor.rs:135` — `|| mint_to_cpi_plan.invoke_signed_from_process_instruction_enabled`
- `programs/xxxl-svm/src/processor.rs:144` — `invoke_signed_from_process_instruction_enabled: false,`
- `programs/xxxl-svm/src/processor.rs:164` — `|| planning_composition.invoke_signed_from_process_instruction_enabled`
- `programs/xxxl-svm/src/processor.rs:176` — `.invoke_signed_from_process_instruction_enabled`
- `programs/xxxl-svm/src/processor.rs:201` — `invoke_signed_from_process_instruction_enabled: false,`
- `programs/xxxl-svm/src/processor.rs:793` — `.invoke_signed_from_process_instruction_enabled`
- `programs/xxxl-svm/src/processor.rs:797` — `assert!(!composition.invoke_signed_from_process_instruction_enabled);`
- `programs/xxxl-svm/src/processor.rs:929` — `assert!(!composition.invoke_signed_from_process_instruction_enabled);`
- `programs/xxxl-svm/src/processor.rs:934` — `.invoke_signed_from_process_instruction_enabled`

If a unit test directly invokes `mint_to_cpi_boundary`, that must be treated as a test-only CPI presence path, not as enabled gateway execution.

Any future change that makes this path reachable from the enabled entrypoint requires separate review and explicit blocker transition.

### Ignored Mollusk tests

The current Mollusk integration file contains the following ignored tests:

- `invalid_consume_gateway_mint_account_count_rejects_before_live_route` (`#[ignore]` line 735, fn line 736)
- `invalid_consume_gateway_mint_readonly_account_passed_writable_rejects_before_validation` (`#[ignore]` line 756, fn line 757)
- `invalid_consume_gateway_mint_required_writable_account_passed_readonly_rejects_before_validation` (`#[ignore]` line 776, fn line 777)
- `invalid_consume_gateway_mint_unexpected_signer_rejects_before_validation` (`#[ignore]` line 797, fn line 798)
- `invalid_consume_gateway_mint_wrong_program_account_owner_rejects_before_live_route` (`#[ignore]` line 817, fn line 818)
- `invalid_consume_gateway_mint_wrong_recipient_token_owner_rejects_before_live_route` (`#[ignore]` line 1041, fn line 1042)
- `invalid_consume_gateway_mint_zero_amount_rejects_before_live_route` (`#[ignore]` line 1064, fn line 1065)
- `invalid_consume_gateway_mint_length_rejects_before_scaffold_path` (`#[ignore]` line 1089, fn line 1090)
- `invalid_consume_gateway_mint_discriminator_rejects_before_scaffold_path` (`#[ignore]` line 1111, fn line 1112)
- `invalid_consume_gateway_mint_version_rejects_before_scaffold_path` (`#[ignore]` line 1133, fn line 1134)

These ignored tests must not be counted as active passing coverage until they are explicitly enabled and passing.

### Account kind field decision

The absence of a separate serialized `kind` field is recorded as the current skeleton design decision, not an unresolved blocker.

The account discriminator currently serves as the account-kind identity.

Adding a separate serialized `kind` field later would require a dedicated account-layout transition stage and updated tests.

### Deferred replay assignment

Deferred replay coverage is assigned to a future replay / processed-event local model phase.

Deferred scenarios:

- coefficient version replay rejection
- guardian set version replay rejection
- pause/unpause replay rejection
- upgrade replay rejection
- source fork replay rejection

No phase before that replay-focused stage may claim complete replay readiness.

### Stage 1 authorization consumer boundary assignment

Stage 1 authorization consumer boundary modeling is assigned to a future Stage 1 consumer modeling phase.

It was not addressed in Phase 1 or Phase 2.

That future phase must define how XXXL runtime consumes Stage 1 authorization results without bypassing or expanding Stage 1 responsibility.

### Execution plan CPI naming clarification

Any execution-plan step named `ValidateAndPrepareCpi` or similar refers to boundary preparation and validation only while `SPL_CPI_EXECUTION_DISABLED` remains active.

It must not be interpreted as actual SPL CPI execution.

### Record hierarchy

The full Phase 2 checkpoint document is the authoritative record.

The current-design checkpoint section is a condensed summary for the design checkpoint index only and does not supersede the full checkpoint.
