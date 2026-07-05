# Phase 41K.6 B6.6 — Local runtime capability inventory

## Purpose

This document records a local source-level capability inventory for the B6.6 testnet launch path.

This inventory inspects local repository files only.

This inventory does not use RPC.

This inventory does not sign.

This inventory does not submit transactions.

This inventory does not spend SOL.

This inventory does not access private keys.

This inventory does not load keypair files.

This inventory does not deploy.

This inventory does not upgrade a program.

This inventory does not initialize accounts.

This inventory does not remove the B1C7 compile_error guard.

This inventory does not weaken the B1C7 feature gate.

This inventory does not open production or production-like activation.

## Current main checkpoint

B6.6 testnet launch execution boundary is merged on main:

9840b7c Merge phase 41K.6 B6.6 testnet launch execution boundary

Current decision remains:

NO-GO.

## Inventory boundary

- observed_at_utc: 2026-07-05T18:06:43+00:00
- inventory_type: local_source_only
- live_action_class: Class A build-only/local artifact planning
- rpc_used: false
- signing_used: false
- submit_used: false
- sol_spend_used: false
- private_key_access_used: false

## Files inspected

- programs/xxxl-svm/src/lib.rs: present
- programs/xxxl-svm/src/processor.rs: present
- programs/xxxl-svm/src/instruction.rs: present
- programs/xxxl-svm/src/account_contract.rs: present
- programs/xxxl-svm/src/deployment_status.rs: present
- programs/xxxl-svm/src/program_id_status.rs: present
- programs/xxxl-svm/src/pda.rs: present
- programs/xxxl-svm/src/cpi.rs: present

## Detected readiness markers

- scaffold_only_not_deployable: true
- placeholder_program_id_boundary: true
- compile_error_present: true
- live_route_disabled_marker: true
- spl_cpi_gate_present: true
- consume_gateway_mint_instruction_present: true
- create_account_reference_present: false
- system_instruction_reference_present: false
- initialize_reference_present: false

## Capability conclusion

- conclusion: no_direct_state_initialization_readiness_detected

Interpretation:

- If no direct state initialization readiness is detected, the existing source should not be treated as ready for B6.6 state initialization.
- A later testnet launch path must choose between program upgrade, new deployment, or redesign.
- Any live action still requires a separate explicit GO form.

## Source evidence

### programs/xxxl-svm/src/lib.rs

- L23: pub mod processed_event_marking_boundary;
- L25: pub mod processed_event_marking_svm_harness;
- L37: pub const XXXL_RUNTIME_STATUS: &str = "SCAFFOLD_ONLY_NOT_DEPLOYABLE";

### programs/xxxl-svm/src/processor.rs

- L2: // This processor scaffold validates and plans around an already-initialized
- L3: // program-owned processed_event account. It is intentionally not a Phase
- L37:         assert_gateway_mint_authority_pda, guarded_mint_to_cpi_execution_gate_boundary,
- L38:         plan_mint_to_cpi_boundary, MintToCpiAccounts, MintToCpiBoundary, MintToCpiPlanningBoundary,
- L47:     processed_event_marking_boundary::{mark_processed_event_atomic, ProcessedEventMarkingWitness},
- L53:         assert_account_owner, assert_initialized_mint_account, assert_recipient_ata_boundary,
- L66: compile_error!(
- L72:     load_phase_41k_2_guardian_set_account_info, GuardianPublicKey,
- L76: pub const CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS: usize = 11;
- L78: pub const ACCOUNT_INDEX_MINT_STATE: usize = 0;
- L79: pub const ACCOUNT_INDEX_GATEWAY_CONFIG: usize = 1;
- L80: pub const ACCOUNT_INDEX_GUARDIAN_SET: usize = 2;
- L81: pub const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 3;
- L82: pub const ACCOUNT_INDEX_RECIPIENT_BALANCE: usize = 4;
- L94: compile_error!(
- L98: pub const LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED: bool = false;
- L108:     pub mint_to_cpi_plan: MintToCpiPlanningBoundary,
- L115:     pub mint_to_cpi_plan: MintToCpiPlanningBoundary,
- L124:     pub recipient_balance_after: u128,
- L132:     pub guardian_set_id: [u8; 32],
- L140:     pub guardian_set_account_readonly: bool,
- L141:     pub guardian_set_account_non_signer: bool,
- L143:     pub processed_event_marking_enabled: bool,
- L149: pub fn b1b_load_authoritative_guardian_set_for_consume_gateway_mint(
- L154:     let guardian_set_account = account_at(accounts, args.guardian_set_account_index as usize)?;
- L156:     let load = load_phase_41k_2_guardian_set_account_info(
- L157:         Some(guardian_set_account),
- L159:         &args.guardian_set_id,
- L166:     let guardian_set_id = load
- L167:         .guardian_set_id
- L173:     if guardian_set_id != args.guardian_set_id {
- L178:         guardian_set_id,
- L186:         guardian_set_account_readonly: load.guardian_set_account_readonly,
- L187:         guardian_set_account_non_signer: load.guardian_set_account_non_signer,
- L189:         processed_event_marking_enabled: false,
- L200:     pub processed_event_marking_enabled: bool,
- L218:         .get(..CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS)
- L241:         .get(..CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS)
- L251:     let guardian_set_account = account_at(accounts, args.guardian_set_account_index as usize)?;
- L252:     let instructions_sysvar_account = account_at(accounts, CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS)?;
- L254:     let guardian_set = load_phase_41k_2_guardian_set_account_info(
- L255:         Some(guardian_set_account),
- L257:         &args.guardian_set_id,
- L260:     if guardian_set.status
- L278:     let processed_event_account =
- L279:         account_at(accounts, args.processed_event_account_index as usize)?;
- L286:         processed_event: *processed_event_account.key,
- L291:         guardian_set_id: args.guardian_set_id,
- L295:         &guardian_set,
- L329:     if !crate::cpi::spl_mint_to_cpi_execution_enabled() {
- L341:             processed_event_marking_enabled: true,
- L355:         if crate::processed_event_marking_svm_harness::is_phase_41k4_marking_svm_harness_instruction(
- L358:             return crate::processed_event_marking_svm_harness::process_phase_41k4_marking_svm_harness_instruction(
- L425:         || execution_plan.mint_to_invocation_from_process_instruction_enabled
- L445:         || execution_plan.mint_to_invocation_from_process_instruction_enabled
- L450:     let mint_to_cpi_plan =
- L451:         plan_mint_to_cpi_boundary(program_id, &execution_plan, &prepared.boundary)?;
- L453:     if mint_to_cpi_plan.live_route_activation_enabled
- L454:         || mint_to_cpi_plan.invoke_signed_from_process_instruction_enabled
- L461:         mint_to_cpi_plan,
- L490:             .mint_to_invocation_from_process_instruction_enabled
- L492:             .mint_to_cpi_plan
- L495:             .mint_to_cpi_plan
- L501:     let processed_event_account =
- L502:         account_at(accounts, args.processed_event_account_index as usize)?;
- L503:     let recipient_balance_account =
- L504:         account_at(accounts, args.recipient_balance_account_index as usize)?;
- L506:     let recipient_balance_after = {
- L507:         let mut processed_event_data = processed_event_account.try_borrow_mut_data()?;
- L508:         let mut recipient_balance_data = recipient_balance_account.try_borrow_mut_data()?;
- L511:             &mut processed_event_data,
- L512:             &mut recipient_balance_data,
- L519:         recipient_balance_after,
- L537:         || execution_plan.mint_to_invocation_from_process_instruction_enabled
- L542:     let mint_to_cpi_plan =
- L543:         plan_mint_to_cpi_boundary(program_id, &execution_plan, &prepared.boundary)?;
- L545:     if mint_to_cpi_plan.live_route_activation_enabled
- L546:         || mint_to_cpi_plan.invoke_signed_from_process_instruction_enabled
- L551:     let processed_event_account =
- L552:         account_at(accounts, args.processed_event_account_index as usize)?;
- truncated_matches: 303

### programs/xxxl-svm/src/instruction.rs

- L20: pub const CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX: u8 = 2;
- L21: pub const CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX: u8 = 0;
- L22: pub const CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX: u8 = 3;
- L23: pub const CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX: u8 = 4;
- L35:     pub guardian_set_account_index: u8,
- L36:     pub mint_state_account_index: u8,
- L37:     pub processed_event_account_index: u8,
- L38:     pub recipient_balance_account_index: u8,
- L40:     pub guardian_set_id: [u8; 32],
- L70:         let guardian_set_account_index = input[12];
- L71:         let mint_state_account_index = input[13];
- L72:         let processed_event_account_index = input[14];
- L73:         let recipient_balance_account_index = input[15];
- L77:             || guardian_set_account_index != CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX
- L78:             || mint_state_account_index != CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX
- L79:             || processed_event_account_index != CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX
- L80:             || recipient_balance_account_index
- L81:                 != CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX
- L100:             guardian_set_account_index,
- L101:             mint_state_account_index,
- L102:             processed_event_account_index,
- L103:             recipient_balance_account_index,
- L105:             guardian_set_id: read_fixed_32(input, 48),
- L154:                 assert_eq!(args.guardian_set_account_index, 2);
- L155:                 assert_eq!(args.mint_state_account_index, 0);
- L156:                 assert_eq!(args.processed_event_account_index, 3);
- L157:                 assert_eq!(args.recipient_balance_account_index, 4);
- L159:                 assert_eq!(args.guardian_set_id, [0x22; 32]);
- L306:         bytes[12] = CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX;
- L307:         bytes[13] = CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX;
- L308:         bytes[14] = CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX;
- L309:         bytes[15] = CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX;

### programs/xxxl-svm/src/account_contract.rs

- L11: compile_error!("phase-41k6-b1-v3-account-contract-test-gate introduces the B1 V3 ConsumeGatewayMint account contract skeleton. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature.");
- L49:         name: "mint_state",
- L56:         name: "gateway_config",
- L63:         name: "guardian_set",
- L69:     // The processed_event account is the replay-protection PDA. It may enter
- L74:         name: "processed_event",
- L81:         name: "recipient_balance",
- L135:         name: "mint_state",
- L142:         name: "gateway_config",
- L149:         name: "guardian_set",
- L156:         name: "processed_event",
- L163:         name: "recipient_balance",
- L300:             CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX,
- L301:             CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
- L302:             CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
- L303:             CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX,
- L307:             ACCOUNT_INDEX_GATEWAY_CONFIG, ACCOUNT_INDEX_GUARDIAN_SET,
- L308:             ACCOUNT_INDEX_MINT_AUTHORITY_PDA, ACCOUNT_INDEX_MINT_STATE,
- L309:             ACCOUNT_INDEX_PROCESSED_EVENT, ACCOUNT_INDEX_RECIPIENT_BALANCE,
- L312:             ACCOUNT_INDEX_TOKEN_PROGRAM, CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS,
- L324:         assert_eq!(contract.len(), CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS);
- L333:         assert_entry(ACCOUNT_INDEX_MINT_STATE, "mint_state");
- L334:         assert_entry(ACCOUNT_INDEX_GATEWAY_CONFIG, "gateway_config");
- L335:         assert_entry(ACCOUNT_INDEX_GUARDIAN_SET, "guardian_set");
- L336:         assert_entry(ACCOUNT_INDEX_PROCESSED_EVENT, "processed_event");
- L337:         assert_entry(ACCOUNT_INDEX_RECIPIENT_BALANCE, "recipient_balance");
- L352:             ACCOUNT_INDEX_MINT_STATE as u8,
- L353:             CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX
- L356:             ACCOUNT_INDEX_GATEWAY_CONFIG as u8,
- L360:             ACCOUNT_INDEX_GUARDIAN_SET as u8,
- L361:             CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX
- L364:             ACCOUNT_INDEX_PROCESSED_EVENT as u8,
- L365:             CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX
- L368:             ACCOUNT_INDEX_RECIPIENT_BALANCE as u8,
- L369:             CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX
- L375:         assert_readonly(ACCOUNT_INDEX_MINT_STATE);
- L376:         assert_readonly(ACCOUNT_INDEX_GATEWAY_CONFIG);
- L377:         assert_readonly(ACCOUNT_INDEX_GUARDIAN_SET);
- L378:         assert_writable(ACCOUNT_INDEX_PROCESSED_EVENT);
- L379:         assert_writable(ACCOUNT_INDEX_RECIPIENT_BALANCE);
- L563:         assert_owner_model(ACCOUNT_INDEX_MINT_STATE, AccountOwnerModel::ProgramOwned);
- L565:             ACCOUNT_INDEX_GATEWAY_CONFIG,
- L568:         assert_owner_model(ACCOUNT_INDEX_GUARDIAN_SET, AccountOwnerModel::ProgramOwned);
- L570:             ACCOUNT_INDEX_PROCESSED_EVENT,
- L574:             ACCOUNT_INDEX_RECIPIENT_BALANCE,
- L603:             CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS

### programs/xxxl-svm/src/deployment_status.rs

- L69:         description: "SPL Token mint_to CPI execution remains disabled.",
- L70:         resolution: "Enable SPL Token mint_to CPI execution only after live route activation, PDA authority, account contract, and Mollusk coverage are complete.",
- L74:         code: "PRODUCTION_GUARDIAN_SET_UNSET",
- L95:         status_code: "SCAFFOLD_ONLY_NOT_DEPLOYABLE",
- L105:                 "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
- L132:                 "PRODUCTION_GUARDIAN_SET_UNSET"
- L148:                 "SPL Token mint_to CPI execution remains disabled."
- L177:                 "Enable SPL Token mint_to CPI execution only after live route activation, PDA authority, account contract, and Mollusk coverage are complete."
- L261: pub fn live_route_activation_from_process_instruction_enabled_for_deployment_status() -> bool {
- L262:     crate::processor::LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED
- L266:     crate::cpi::spl_mint_to_cpi_execution_enabled()
- L275:     fn runtime_status_is_scaffold_only_not_deployable() {
- L281:         assert_eq!(XXXL_RUNTIME_STATUS, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- L288:             "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
- L318:         assert_eq!(blockers[3].code(), "PRODUCTION_GUARDIAN_SET_UNSET");
- L336:             "SPL Token mint_to CPI execution remains disabled."
- L462:                 "PRODUCTION_GUARDIAN_SET_UNSET",
- L471:         assert!(!live_route_activation_from_process_instruction_enabled_for_deployment_status());
- L483:         assert_eq!(report.status_code, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- L493:         assert_eq!(report.blockers[3].code, "PRODUCTION_GUARDIAN_SET_UNSET");
- L512:         assert!(!live_route_activation_from_process_instruction_enabled_for_deployment_status());
- L520:                 assert_eq!(report.status_code, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- L552:         assert!(!live_route_activation_from_process_instruction_enabled_for_deployment_status());

### programs/xxxl-svm/src/program_id_status.rs

- L29:         status_code: "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- L41:             XxxlProgramIdReadinessStatus::Placeholder => "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- L86:         assert_eq!(status.code(), "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- L98:         assert_eq!(report.status_code, "PLACEHOLDER_PROGRAM_ID_BOUNDARY");

### programs/xxxl-svm/src/pda.rs

- no relevant markers found

### programs/xxxl-svm/src/cpi.rs

- L10: compile_error!(
- L22: compile_error!(
- L59: pub fn plan_mint_to_cpi_boundary(
- L65:         || execution_plan.mint_to_invocation_from_process_instruction_enabled
- L87:     build_mint_to_instruction(
- L123: pub fn spl_mint_to_cpi_execution_enabled() -> bool {
- L134: pub fn spl_mint_to_cpi_execution_enabled() -> bool {
- L139: pub fn guarded_mint_to_cpi_execution_gate_boundary(
- L146:         || execution_plan.mint_to_invocation_from_process_instruction_enabled
- L154:         plan_mint_to_cpi_boundary(program_id, execution_plan, boundary)?;
- L160:     if !spl_mint_to_cpi_execution_enabled() {
- L164:     mint_to_cpi_boundary(
- L179: pub fn build_mint_to_instruction(
- L190:     spl_token::instruction::mint_to(
- L223: pub fn mint_to_cpi_boundary(
- L233:     let instruction = build_mint_to_instruction(
- L273:     fn with_mint_to_cpi_boundary_fixture<T>(
- L364:             mint_to_invocation_from_process_instruction_enabled: false,
- L373:     fn guarded_mint_to_cpi_execution_gate_boundary_rejects_when_gate_disabled() {
- L380:         assert!(!spl_mint_to_cpi_execution_enabled());
- L382:         with_mint_to_cpi_boundary_fixture(
- L394:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
- L395:                         .expect("mint_to CPI planning boundary");
- L398:                     guarded_mint_to_cpi_execution_gate_boundary(
- L411:     fn guarded_mint_to_cpi_execution_gate_boundary_rejects_planning_boundary_mismatch() {
- L418:         with_mint_to_cpi_boundary_fixture(
- L430:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
- L431:                         .expect("mint_to CPI planning boundary");
- L435:                     guarded_mint_to_cpi_execution_gate_boundary(
- L448:     fn guarded_mint_to_cpi_execution_gate_boundary_rejects_live_route_flag_before_cpi() {
- L455:         with_mint_to_cpi_boundary_fixture(
- L467:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
- L468:                         .expect("mint_to CPI planning boundary");
- L472:                     guarded_mint_to_cpi_execution_gate_boundary(
- L485:     fn guarded_mint_to_cpi_execution_gate_boundary_rejects_wrong_pda_before_gate() {
- L501:         with_mint_to_cpi_boundary_fixture(
- L513:                     guarded_mint_to_cpi_execution_gate_boundary(
- L526:     fn mint_to_cpi_planning_boundary_builds_plan_without_invoke_signed() {
- L533:         with_mint_to_cpi_boundary_fixture(
- L544:                 let plan = plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary)
- L545:                     .expect("mint_to CPI planning boundary");
- L560:     fn mint_to_cpi_planning_boundary_rejects_amount_mismatch() {
- L566:         with_mint_to_cpi_boundary_fixture(
- L578:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L586:     fn mint_to_cpi_planning_boundary_rejects_zero_boundary_amount() {
- L592:         with_mint_to_cpi_boundary_fixture(
- L604:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L612:     fn mint_to_cpi_planning_boundary_rejects_live_route_flag() {
- L619:         with_mint_to_cpi_boundary_fixture(
- L631:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L639:     fn mint_to_cpi_planning_boundary_rejects_mint_to_flag() {
- L644:         execution_plan.mint_to_invocation_from_process_instruction_enabled = true;
- L646:         with_mint_to_cpi_boundary_fixture(
- L658:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L666:     fn mint_to_cpi_planning_boundary_rejects_wrong_token_program() {
- L672:         with_mint_to_cpi_boundary_fixture(
- L684:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L692:     fn mint_to_cpi_planning_boundary_rejects_wrong_mint_mapping() {
- L698:         with_mint_to_cpi_boundary_fixture(
- L710:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L718:     fn mint_to_cpi_planning_boundary_rejects_wrong_pda() {
- L724:         with_mint_to_cpi_boundary_fixture(
- L736:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L744:     fn mint_to_cpi_planning_boundary_rejects_wrong_bump() {
- L750:         with_mint_to_cpi_boundary_fixture(
- L762:                     plan_mint_to_cpi_boundary(&program_id, &execution_plan, boundary),
- L770:     fn mint_to_instruction_uses_spl_token_program_and_expected_accounts() {
- L775:         let instruction = build_mint_to_instruction(
- L782:         .expect("valid mint_to instruction");
- L795:     fn mint_to_instruction_rejects_zero_amount() {
- L796:         let result = build_mint_to_instruction(
- L855:     fn mint_to_boundary_rejects_wrong_pda_before_invoke_signed() {
- L927:         let result = mint_to_cpi_boundary(&program_id, boundary);

## Required next decision

Based on this inventory, the next B6.6 decision must choose one of:

1. Existing deployed program can initialize state as-is.

2. Program upgrade is required before state initialization.

3. New testnet deployment is required.

4. Stop and redesign the launch path.

This document does not choose a live-action strategy.

## Current decision

Current decision:

NO-GO.

This local runtime capability inventory does not authorize live action.
