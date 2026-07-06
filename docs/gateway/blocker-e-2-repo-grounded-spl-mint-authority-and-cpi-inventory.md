# Blocker E.2 — Repo-grounded SPL mint authority and CPI inventory

Status:

BLOCKER_E_OPEN_REPO_GROUNDED_SPL_MINT_AUTHORITY_AND_CPI_INVENTORY_COMPLETED_NO_SPL_SETUP

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker E.2 records a repo-grounded inventory of SPL mint authority and CPI boundaries.

E.2 is inventory-only.

It does not create an SPL mint.

It does not configure mint authority.

It does not transfer mint authority.

It does not set freeze authority.

It does not mint tokens.

It does not initialize state.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, construct guardian packages, submit, or mutate any network.

## Evidence files

- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/metadata.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/spl-mint-authority-cpi-grep.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/authority-inventory.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/cpi-inventory.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/account-contract-inventory.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/state-relationship-inventory.txt
- docs/gateway/evidence/blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory/inventory-summary.txt

## Authority inventory

- gateway_mint_authority PDA
  - seeds: xxxl / gateway-mint-authority / v1
  - depends_on_program_id: true
  - expected role: SPL Token mint authority for gateway-backed XXXL minting
  - current status: inventoried, not created, not activated by E.2

## CPI inventory

- token program model: classic SPL Token via spl_token::id()
- mint instruction model: spl_token::instruction::mint_to
- signer model: invoke_signed with gateway_mint_authority signer seeds
- mint authority check: assert_gateway_mint_authority_pda
- default CPI execution: disabled
- closed-gate result: CpiBoundaryNotReady
- open-gate route: requires D2 production-path test gate plus B1C7 handler integration test gate and both dangerous allow features
- E.2 execution status: no SPL mint setup, no mint authority transfer, no mint_to execution

## Account contract inventory

- spl_token_mint: writable, not signer, SplTokenOwned
- recipient_token_account: writable, not signer, SplTokenOwned
- mint_authority_pda: readonly, not signer, ProgramDerivedAddress
- token_program: readonly, not signer, SplTokenProgram

## MintState relationship inventory

- MintState records mint_pubkey
- MintState records gateway_mint_authority_pda
- MintState records gateway_mint_authority_bump
- MintState records total_supply
- E.2 does not prove SPL total supply reconciliation; this remains a future E invariant

## Inventory checks

- gateway_mint_authority_pda_inventory_present: true
- gateway_mint_authority_seeds_present: true
- gateway_mint_authority_depends_on_program_id: true
- gateway_mint_authority_derivation_function_present: true
- mint_to_cpi_accounts_present: true
- mint_to_cpi_boundary_present: true
- mint_to_cpi_planning_boundary_present: true
- classic_spl_token_program_asserted: true
- spl_token_mint_to_instruction_built: true
- mint_authority_pda_asserted_against_program_derivation: true
- gateway_mint_authority_signer_seeds_present: true
- mint_to_cpi_uses_invoke_signed: true
- spl_cpi_execution_disabled_by_default: true
- spl_cpi_gate_open_requires_d2_and_b1c7_dangerous_allows: true
- guarded_cpi_returns_cpi_boundary_not_ready_when_gate_closed: true
- account_contract_has_spl_mint: true
- account_contract_has_recipient_token_account: true
- account_contract_has_mint_authority_pda: true
- account_contract_has_token_program: true
- mint_state_records_mint_pubkey: true
- mint_state_records_gateway_mint_authority_pda: true
- mint_state_records_gateway_mint_authority_bump: true
- mint_state_records_total_supply: true
- execution_plan_keeps_live_route_flags_explicit: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

all_inventory_checks_passed: true

## E.2 interpretation

The current repo has a coherent planned SPL mint authority/CPI boundary:

- gateway_mint_authority PDA is the expected program-derived mint authority
- CPI planning validates token program, mint, amount, and PDA derivation
- SPL mint_to instruction construction exists
- invoke_signed is planned through gateway_mint_authority signer seeds
- CPI execution remains disabled by default
- guarded CPI returns CpiBoundaryNotReady when the gate is closed

However, E.2 does not select the final authority setup model and does not approve execution.

## Remaining gaps before Blocker E closure

E.2 does not yet resolve:

- canonical token program final decision
- canonical mint account model
- decimals
- initial supply rule
- setup authority model
- mint authority handoff model
- freeze authority model
- evidence that no human/admin mint authority remains
- total supply reconciliation between SPL mint and MintState
- final no-manual-mint/no-admin-supply-control invariant

## Non-closure statement

E.2 does not close Blocker E.

E.2 does not approve:

- SPL mint creation
- SPL mint initialization
- mint authority assignment
- mint authority transfer
- freeze authority assignment
- freeze authority disablement
- SPL CPI minting
- state initialization execution
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- guardian package construction
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_E_OPEN_REPO_GROUNDED_SPL_MINT_AUTHORITY_AND_CPI_INVENTORY_COMPLETED_NO_SPL_SETUP

Current decision:

BLOCKER_E_NOT_CLOSED

NO-GO REMAINS_FOR_SPL_MINT_SETUP_SPL_AUTHORITY_TRANSFER_SPL_CPI_MINTING_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker E.3 — SPL mint authority setup decision model.

E.3 should select or reject the candidate authority setup models.

E.3 must not create an SPL mint, configure authority, call RPC, use testnet, sign, deploy, upgrade, initialize state, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-e-2-repo-grounded-spl-mint-authority-and-cpi-inventory
timestamp_utc=2026-07-06T18:57:08Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
spl_mint_created=false
spl_authority_configured=false
spl_mint_to_executed=false
state_initialized=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```

grep preview:

```text
programs/xxxl-svm/src/account_contract.rs:28:    SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:29:    ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:30:    SplTokenProgram,
programs/xxxl-svm/src/account_contract.rs:88:        name: "spl_token_mint",
programs/xxxl-svm/src/account_contract.rs:91:        owner_model: AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:95:        name: "recipient_token_account",
programs/xxxl-svm/src/account_contract.rs:98:        owner_model: AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:102:        name: "mint_authority_pda",
programs/xxxl-svm/src/account_contract.rs:105:        owner_model: AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:109:        name: "token_program",
programs/xxxl-svm/src/account_contract.rs:112:        owner_model: AccountOwnerModel::SplTokenProgram,
programs/xxxl-svm/src/account_contract.rs:170:        name: "spl_token_mint",
programs/xxxl-svm/src/account_contract.rs:173:        owner_model: AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:177:        name: "recipient_token_account",
programs/xxxl-svm/src/account_contract.rs:180:        owner_model: AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:184:        name: "mint_authority_pda",
programs/xxxl-svm/src/account_contract.rs:187:        owner_model: AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:191:        name: "token_program",
programs/xxxl-svm/src/account_contract.rs:194:        owner_model: AccountOwnerModel::SplTokenProgram,
programs/xxxl-svm/src/account_contract.rs:338:        assert_entry(ACCOUNT_INDEX_SPL_TOKEN_MINT, "spl_token_mint");
programs/xxxl-svm/src/account_contract.rs:341:            "recipient_token_account",
programs/xxxl-svm/src/account_contract.rs:343:        assert_entry(ACCOUNT_INDEX_MINT_AUTHORITY_PDA, "mint_authority_pda");
programs/xxxl-svm/src/account_contract.rs:344:        assert_entry(ACCOUNT_INDEX_TOKEN_PROGRAM, "token_program");
programs/xxxl-svm/src/account_contract.rs:579:            AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:583:            AccountOwnerModel::SplTokenOwned,
programs/xxxl-svm/src/account_contract.rs:587:            AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:591:            AccountOwnerModel::SplTokenProgram,
programs/xxxl-svm/src/account_order_skeleton.rs:25:    ProgramDerivedAddress,
programs/xxxl-svm/src/account_order_skeleton.rs:28:    SplTokenProgram,
programs/xxxl-svm/src/account_order_skeleton.rs:146:        name: "gateway_mint_authority_pda",
programs/xxxl-svm/src/account_order_skeleton.rs:149:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
programs/xxxl-svm/src/account_order_skeleton.rs:154:        name: "token_program",
programs/xxxl-svm/src/account_order_skeleton.rs:157:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
programs/xxxl-svm/src/account_order_skeleton.rs:229:        name: "recipient_token_account",
programs/xxxl-svm/src/account_order_skeleton.rs:237:        name: "gateway_mint_authority_pda",
programs/xxxl-svm/src/account_order_skeleton.rs:240:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
programs/xxxl-svm/src/account_order_skeleton.rs:245:        name: "token_program",
programs/xxxl-svm/src/account_order_skeleton.rs:248:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
programs/xxxl-svm/src/account_validation_skeleton.rs:214:                expected: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
programs/xxxl-svm/src/account_validation_skeleton.rs:242:            XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress
programs/xxxl-svm/src/account_validation_skeleton.rs:246:            XxxlAccountOwnerExpectationSkeleton::SplTokenProgram
programs/xxxl-svm/src/consume_execution_plan_skeleton.rs:121:            mint_authority_pda: Pubkey::new_unique(),
programs/xxxl-svm/src/consume_execution_plan_skeleton.rs:123:            token_program: Pubkey::new_unique(),
programs/xxxl-svm/src/consume_state_transition_skeleton.rs:119:            mint_authority_pda: Pubkey::new_unique(),
programs/xxxl-svm/src/consume_state_transition_skeleton.rs:121:            token_program: Pubkey::new_unique(),
programs/xxxl-svm/src/consume_state_transition_skeleton.rs:163:            output.next_mint_state.mint_authority_pda,
programs/xxxl-svm/src/consume_state_transition_skeleton.rs:164:            mint_state.mint_authority_pda
programs/xxxl-svm/src/cpi.rs:2:    account_info::AccountInfo, instruction::Instruction, program::invoke_signed,
programs/xxxl-svm/src/cpi.rs:29:        find_gateway_mint_authority, GATEWAY_MINT_AUTHORITY_SEED_0, GATEWAY_MINT_AUTHORITY_SEED_1,
programs/xxxl-svm/src/cpi.rs:30:        GATEWAY_MINT_AUTHORITY_SEED_2,
programs/xxxl-svm/src/cpi.rs:34:pub struct MintToCpiAccounts<'a, 'b> {
programs/xxxl-svm/src/cpi.rs:35:    pub token_program: &'a AccountInfo<'b>,
programs/xxxl-svm/src/cpi.rs:37:    pub recipient_token_account: &'a AccountInfo<'b>,
programs/xxxl-svm/src/cpi.rs:38:    pub mint_authority_pda: &'a AccountInfo<'b>,
programs/xxxl-svm/src/cpi.rs:41:pub struct MintToCpiBoundary<'a, 'b> {
programs/xxxl-svm/src/cpi.rs:42:    pub accounts: MintToCpiAccounts<'a, 'b>,
programs/xxxl-svm/src/cpi.rs:48:pub struct MintToCpiPlanningBoundary {
programs/xxxl-svm/src/cpi.rs:49:    pub token_program: Pubkey,
programs/xxxl-svm/src/cpi.rs:51:    pub recipient_token_account: Pubkey,
programs/xxxl-svm/src/cpi.rs:52:    pub mint_authority_pda: Pubkey,
programs/xxxl-svm/src/cpi.rs:55:    pub live_route_activation_enabled: bool,
programs/xxxl-svm/src/cpi.rs:56:    pub invoke_signed_from_process_instruction_enabled: bool,
programs/xxxl-svm/src/cpi.rs:59:pub fn plan_mint_to_cpi_boundary(
programs/xxxl-svm/src/cpi.rs:62:    boundary: &MintToCpiBoundary<'_, '_>,
programs/xxxl-svm/src/cpi.rs:63:) -> Result<MintToCpiPlanningBoundary, ProgramError> {
programs/xxxl-svm/src/cpi.rs:64:    if execution_plan.live_route_activation_enabled
programs/xxxl-svm/src/cpi.rs:65:        || execution_plan.mint_to_invocation_from_process_instruction_enabled
programs/xxxl-svm/src/cpi.rs:73:    if boundary.accounts.token_program.key != &spl_token::id() {
programs/xxxl-svm/src/cpi.rs:81:    assert_gateway_mint_authority_pda(
programs/xxxl-svm/src/cpi.rs:83:        boundary.accounts.mint_authority_pda.key,
programs/xxxl-svm/src/cpi.rs:88:        boundary.accounts.token_program.key,
programs/xxxl-svm/src/cpi.rs:90:        boundary.accounts.recipient_token_account.key,
programs/xxxl-svm/src/cpi.rs:91:        boundary.accounts.mint_authority_pda.key,
programs/xxxl-svm/src/cpi.rs:95:    let signer_seeds = gateway_mint_authority_signer_seeds(&boundary.mint_authority_bump);
programs/xxxl-svm/src/cpi.rs:97:    if signer_seeds[0] != GATEWAY_MINT_AUTHORITY_SEED_0
programs/xxxl-svm/src/cpi.rs:98:        || signer_seeds[1] != GATEWAY_MINT_AUTHORITY_SEED_1
programs/xxxl-svm/src/cpi.rs:99:        || signer_seeds[2] != GATEWAY_MINT_AUTHORITY_SEED_2
programs/xxxl-svm/src/cpi.rs:105:    Ok(MintToCpiPlanningBoundary {
programs/xxxl-svm/src/cpi.rs:106:        token_program: *boundary.accounts.token_program.key,
programs/xxxl-svm/src/cpi.rs:108:        recipient_token_account: *boundary.accounts.recipient_token_account.key,
programs/xxxl-svm/src/cpi.rs:109:        mint_authority_pda: *boundary.accounts.mint_authority_pda.key,
programs/xxxl-svm/src/cpi.rs:112:        live_route_activation_enabled: false,
programs/xxxl-svm/src/cpi.rs:113:        invoke_signed_from_process_instruction_enabled: false,
programs/xxxl-svm/src/cpi.rs:123:pub fn spl_mint_to_cpi_execution_enabled() -> bool {
programs/xxxl-svm/src/cpi.rs:124:    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_OPEN_AFTER_B1C7";
programs/
```
