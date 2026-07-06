# Blocker D.2 — Repo-grounded state layout and PDA inventory

Status:

BLOCKER_D_OPEN_REPO_GROUNDED_STATE_LAYOUT_AND_PDA_INVENTORY_COMPLETED_NO_INITIALIZATION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker D.2 records a repo-grounded inventory of state layouts, discriminators, and PDA boundaries.

D.2 is inventory-only.

It does not initialize any account.

It does not create any account.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate any network.

## Evidence files

- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/metadata.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/source-file-list.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/state-layout-pda-grep.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/layout-inventory.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/discriminator-inventory.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/pda-inventory.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/state-categories.txt
- docs/gateway/evidence/blocker-d-2-repo-grounded-state-layout-and-pda-inventory/inventory-summary.txt

## Account layout inventory

- MINT_STATE_ACCOUNT_LEN: 176
- GATEWAY_CONFIG_ACCOUNT_LEN: 256
- GUARDIAN_SET_ACCOUNT_LEN: 320
- PROCESSED_EVENT_ACCOUNT_LEN: 144
- RECIPIENT_BALANCE_ACCOUNT_LEN: 144

## Discriminator inventory

- MINT_STATE_ACCOUNT_DISCRIMINATOR
- GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR
- GUARDIAN_SET_ACCOUNT_DISCRIMINATOR
- PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
- RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR

## PDA inventory

- gateway_mint_authority
  - seeds: xxxl / gateway-mint-authority / v1
  - depends_on_program_id: true
  - purpose: SPL Token mint authority for gateway-backed XXXL minting

## State category inventory

- MintState: long-lived protocol state
- GatewayConfig: long-lived route/config state
- GuardianSet: long-lived guardian quorum state
- ProcessedEvent: per-event replay/consumption state
- RecipientBalance: per-recipient accounting state
- gateway_mint_authority: derived authority PDA
- SPL mint and token accounts: Blocker E scope

## Inventory checks

- all_expected_account_lengths_present: true
- all_expected_account_lengths_match_current_values: true
- runtime_layout_version_is_1: true
- all_expected_discriminators_present: true
- mint_state_view_present: true
- gateway_config_view_present: true
- guardian_set_view_present: true
- processed_event_view_present: true
- recipient_balance_view_present: true
- legacy_processed_event_helper_marked_not_live: true
- gateway_mint_authority_pda_inventory_present: true
- gateway_mint_authority_seeds_present: true
- gateway_mint_authority_depends_on_program_id: true
- processed_event_marking_boundary_report_present: true
- processed_event_marking_requires_system_owned_empty_entry: true
- processed_event_marking_accepts_lamport_dusted_empty_pda: true
- processed_event_marking_writes_final_consumed_image: true
- processed_event_marking_redecodes_after_write: true
- processed_event_marking_spl_mint_disabled: true
- processed_event_marking_live_route_disabled: true
- processed_event_marking_function_present: true
- account_contract_processed_event_system_or_program_pda: true
- account_contract_mint_authority_pda_program_derived: true
- account_contract_rent_payer_signer_present: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

all_inventory_checks_passed: true

## ProcessedEvent initialization boundary

D.2 confirms that the legacy pre-41K.4 processed-event helper is not the live initialization model.

The current Phase 41K.4 marking boundary records:

- requires_system_owned_empty_data_entry: true
- accepts_lamport_dusted_empty_pda: true
- writes_final_consumed_image: true
- redecodes_with_41k3_loader_after_write: true
- spl_token_mint_to_enabled: false
- live_route_enabled: false

Interpretation:

ProcessedEvent initialization is a per-event replay-protection boundary and must remain separated from general protocol initialization and from SPL mint setup.

## D.2 design implications

D.2 implies the next design step must separate:

- one-time protocol initialization
- derived PDA authority boundaries
- per-event ProcessedEvent creation/marking
- per-recipient RecipientBalance initialization
- SPL mint and token account setup under Blocker E

## Remaining gaps before Blocker D closure

D.2 does not yet resolve:

- exact initializer authority model
- exact one-time init instruction design
- exact reinitialization rejection rule
- exact rent payer model for long-lived protocol state
- exact recipient-balance lazy initialization model
- exact D/E boundary for SPL mint setup
- exact no-admin-balance-write invariant
- exact local evidence for initialization behavior

## Non-closure statement

D.2 does not close Blocker D.

D.2 does not approve:

- state initialization execution
- account creation
- PDA creation
- SPL mint setup
- SPL CPI minting
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

BLOCKER_D_OPEN_REPO_GROUNDED_STATE_LAYOUT_AND_PDA_INVENTORY_COMPLETED_NO_INITIALIZATION

Current decision:

BLOCKER_D_NOT_CLOSED

NO-GO REMAINS_FOR_STATE_INITIALIZATION_EXECUTION_SIGNING_KEYS_PROGRAM_UPGRADE_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker D.3 — state initialization authority and one-time guard decision model.

D.3 should decide who/what can initialize long-lived protocol state, how reinitialization is rejected, and how this remains separate from SPL setup in Blocker E.

D.3 must not initialize state, call RPC, use testnet, sign, deploy, upgrade, configure SPL, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-d-2-repo-grounded-state-layout-and-pda-inventory
timestamp_utc=2026-07-06T18:02:07Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
state_initialized=false
accounts_created=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```

grep preview:

```text
programs/xxxl-svm/src/account_contract.rs:29:    ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:31:    SystemOwnedOrProgramOwnedPda,
programs/xxxl-svm/src/account_contract.rs:32:    RentPayer,
programs/xxxl-svm/src/account_contract.rs:77:        owner_model: AccountOwnerModel::SystemOwnedOrProgramOwnedPda,
programs/xxxl-svm/src/account_contract.rs:105:        owner_model: AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:119:        owner_model: AccountOwnerModel::RentPayer,
programs/xxxl-svm/src/account_contract.rs:159:        owner_model: AccountOwnerModel::SystemOwnedOrProgramOwnedPda,
programs/xxxl-svm/src/account_contract.rs:187:        owner_model: AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:201:        owner_model: AccountOwnerModel::RentPayer,
programs/xxxl-svm/src/account_contract.rs:571:            AccountOwnerModel::SystemOwnedOrProgramOwnedPda,
programs/xxxl-svm/src/account_contract.rs:587:            AccountOwnerModel::ProgramDerivedAddress,
programs/xxxl-svm/src/account_contract.rs:593:        assert_owner_model(ACCOUNT_INDEX_RENT_PAYER, AccountOwnerModel::RentPayer);
programs/xxxl-svm/src/account_order_skeleton.rs:25:    ProgramDerivedAddress,
programs/xxxl-svm/src/account_order_skeleton.rs:149:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
programs/xxxl-svm/src/account_order_skeleton.rs:240:        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
programs/xxxl-svm/src/account_validation_skeleton.rs:242:            XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress
programs/xxxl-svm/src/cpi.rs:29:        find_gateway_mint_authority, GATEWAY_MINT_AUTHORITY_SEED_0, GATEWAY_MINT_AUTHORITY_SEED_1,
programs/xxxl-svm/src/cpi.rs:30:        GATEWAY_MINT_AUTHORITY_SEED_2,
programs/xxxl-svm/src/cpi.rs:97:    if signer_seeds[0] != GATEWAY_MINT_AUTHORITY_SEED_0
programs/xxxl-svm/src/cpi.rs:98:        || signer_seeds[1] != GATEWAY_MINT_AUTHORITY_SEED_1
programs/xxxl-svm/src/cpi.rs:99:        || signer_seeds[2] != GATEWAY_MINT_AUTHORITY_SEED_2
programs/xxxl-svm/src/cpi.rs:202:        GATEWAY_MINT_AUTHORITY_SEED_0,
programs/xxxl-svm/src/cpi.rs:203:        GATEWAY_MINT_AUTHORITY_SEED_1,
programs/xxxl-svm/src/cpi.rs:204:        GATEWAY_MINT_AUTHORITY_SEED_2,
programs/xxxl-svm/src/cpi.rs:214:    let (expected_pda, expected_bump) = find_gateway_mint_authority(program_id);
programs/xxxl-svm/src/cpi.rs:375:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:413:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:450:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:487:        let (_pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:528:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:562:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:588:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:614:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:641:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:668:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:694:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:720:        let (_pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:746:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:823:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:832:        let (_pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:845:        let (pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/cpi.rs:857:        let (_pda, bump) = find_gateway_mint_authority(&program_id);
programs/xxxl-svm/src/deployment_status.rs:97:        deployable: false,
programs/xxxl-svm/src/execution_plan.rs:16:    state::{credit_recipient_balance, ProcessedEventAccountView, RecipientBalanceAccountView},
programs/xxxl-svm/src/execution_plan.rs:148:        let processed_event = ProcessedEventAccountView::new(processed_event_data)?;
programs/xxxl-svm/src/execution_plan.rs:160:        let recipient_balance = RecipientBalanceAccountView::new(recipient_balance_data)?;
programs/xxxl-svm/src/execution_plan.rs:191:        let processed_event = ProcessedEventAccountView::new(processed_event_data)?;
programs/xxxl-svm/src/execution_plan.rs:203:        let recipient_balance = RecipientBalanceAccountView::new(recipient_balance_data)?;
programs/xxxl-svm/src/execution_plan.rs:241:            GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
programs/xxxl-svm/src/execution_plan.rs:242:            PROCESSED_EVENT_ACCOUNT_LEN, RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
programs/xxxl-svm/src/execution_plan.rs:243:            RECIPIENT_BALANCE_ACCOUNT_LEN, RUNTIME_LAYOUT_VERSION,
programs/xxxl-svm/src/execution_plan.rs:452:            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
programs/xxxl-svm/src/execution_plan.rs:606:            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");
programs/xxxl-svm/src/execution_plan.rs:751:            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
programs/xxxl-svm/src/execution_plan.rs:753:            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");
programs/xxxl-svm/src/execution_plan.rs:996:            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
programs/xxxl-svm/src/execution_plan.rs:998:            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");
programs/xxxl-svm/src/execution_plan.rs:1027:            RecipientBalanceAccountView::new(&recipient_balance_data).expect("recipient balance");
programs/xxxl-svm/src/execution_plan.rs:1049:            ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
programs/xxxl-svm/src/execution_plan.rs:1111:            PROCESSED_EVENT_ACCOUNT_LEN,
programs/xxxl-svm/src/execution_plan.rs:1112:            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
programs/xxxl-svm/src/execution_plan.rs:1125:            RECIPIENT_BALANCE_ACCOUNT_LEN,
programs/xxxl-svm/src/execution_plan.rs:1126:            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
programs/xxxl-svm/src/execution_plan.rs:1140:        data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
programs/xxxl-svm/src/execution_plan.rs:1170:        assert_eq!(GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR.len(), 8);
programs/xxxl-svm/src/pda.rs:3:pub const GATEWAY_MINT_AUTHORITY_SEED_0: &[u8] = b"xxxl";
programs/xxxl-svm/src/pda.rs:4:pub const GATEWAY_MINT_AUTHORITY_SEED_1: &[u8] = b"gateway-mint-authority";
programs/xxxl-svm/src/pda.rs:5:pub const GATEWAY_MINT_AUTHORITY_SEED_2: &[u8] = b"v1";
programs/xxxl-svm/src/pda.rs:41:pub const GATEWAY_MINT_AUTHORITY_SEEDS: [&[u8]; 3] = [
programs/xxxl-svm/src/pda.rs:42:    GATEWAY_MINT_AUTHORITY_SEED_0,
programs/xxxl-svm/src/pda.rs:43:    GATEWAY_MINT_AUTHORITY_SEED_1,
programs/xxxl-svm/src/pda.rs:44:    GATEWAY_MINT_AUTHORITY_SEED_2,
programs/xxxl-svm/src/pda.rs:47:pub const XXXL_PDA_DERIVATION_INVENTORY: [XxxlPdaDerivationInventoryEntry; 1] =
programs/xxxl-svm/src/pda.rs:51:        seeds: GATEWAY_MINT_AUTHORITY_SEEDS,
programs/xxxl-svm/src/pda.rs:59:    GATEWAY_MINT_AUTHORITY_SEEDS
programs/xxxl-svm/src/pda.rs:62:pub fn find_gateway_mint_authority(program_id: &Pubkey) -> (Pubkey, u8) {
programs/xxxl-svm/src/pda.rs:69:    l
```
