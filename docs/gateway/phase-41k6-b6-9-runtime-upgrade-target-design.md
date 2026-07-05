# Phase 41K.6 B6.9 — Runtime upgrade target design and handler inventory

## Purpose

This document records the local-only runtime upgrade target design and handler inventory for Strategy 2.

Strategy 2 remains viable for planning after B6.7 manual placeholder boundary resolution.

B6.8 recorded that all eight upgrade GO blockers remain open.

This document starts blocker C and blocker D planning, but closes no GO blocker by itself.

This document does not approve signing.

This document does not approve transaction submission.

This document does not approve SOL spend.

This document does not approve private-key handling.

This document does not approve deploy.

This document does not approve program upgrade.

This document does not approve account initialization.

This document does not approve SPL mint setup.

This document does not approve guardian package construction.

This document does not approve submit rehearsal.

This document does not remove the B1C7 compile_error guard.

This document does not weaken the B1C7 feature gate.

This document does not open production or production-like activation.

## Current main checkpoint

B6.8 Strategy 2 blocker closure plan is merged on main:

72ab89a Merge phase 41K.6 B6.8 Strategy 2 blocker closure plan

Current decision remains:

NO-GO.

## Local source scan markers

- observed_at_utc: 2026-07-05T18:45:57+00:00
- process_instruction_present: true
- consume_gateway_mint_reference_present: true
- guardian_reference_present: true
- processed_event_reference_present: true
- mint_state_reference_present: true
- gateway_config_reference_present: true
- spl_mint_to_reference_present: true
- compile_error_guard_present: true
- b1c7_reference_present: true
- placeholder_boundary_present: true
- scaffold_status_present: true
- initialize_reference_present: false

## Upgrade target handler inventory

The Strategy 2 upgrade target must expose the following runtime handlers.

### Required initialization handlers

- InitializeGatewayConfig
- InitializeGuardianSet
- InitializeMintState

Optional or derived initialization may be added only if it is explicit and tested:

- InitializeProcessedEventRegistry
- InitializeRouteConfig

### Required execution handler

- ConsumeGatewayMint

The execution handler must preserve the Stage 1 model boundary:

- verify canonical message
- verify route
- verify source chain
- verify source token
- verify canonical event key
- verify recipient hash
- verify guardian quorum
- check processed event before mint
- mint through SPL Token CPI
- mark processed event atomically with mint
- reject replay
- reject wrong accounts
- reject wrong SPL Token program
- reject wrong mint authority PDA

## Required state accounts

The upgrade target must define and test these state accounts:

- gateway_config
- guardian_set
- mint_state
- processed_event

Each state account must define:

- PDA seeds
- bump handling
- owner model
- exact serialized size
- discriminator or version field
- idempotency guard
- AlreadyInitialized behavior
- wrong owner behavior
- wrong PDA behavior

## Required PDA design

Minimum PDA inventory for Strategy 2:

- gateway_mint_authority: existing seed family b"xxxl" / b"gateway-mint-authority" / b"v1"
- gateway_config: required, seed family to be finalized
- guardian_set: required, seed family to be finalized
- mint_state: required, seed family to be finalized
- processed_event: required, seed family to be finalized

The existing gateway mint authority PDA remains program-id-parametric and is not structurally bound to the placeholder program id.

## Required instruction design

The instruction enum or instruction tag layout must be explicit and stable.

Minimum instruction set:

- InitializeGatewayConfig
- InitializeGuardianSet
- InitializeMintState
- ConsumeGatewayMint

Each instruction must define:

- binary tag
- payload layout
- account list order
- signer requirements
- writable account requirements
- owner requirements
- PDA requirements
- failure cases

## Required tests before artifact hashing

Before any deployable artifact hash can be treated as meaningful, the local code must prove:

- instruction encoding and decoding round trip
- InitializeGatewayConfig success
- InitializeGatewayConfig double-call rejection
- InitializeGuardianSet success
- InitializeGuardianSet double-call rejection
- InitializeMintState success
- InitializeMintState double-call rejection
- ConsumeGatewayMint handler is present
- ConsumeGatewayMint rejects missing guardian quorum
- ConsumeGatewayMint rejects unknown guardian
- ConsumeGatewayMint rejects duplicate guardian
- ConsumeGatewayMint rejects replay
- ConsumeGatewayMint checks before marking processed event
- ConsumeGatewayMint marks processed event atomically with mint
- SPL mint_to CPI boundary validates mint authority PDA
- wrong SPL Token program is rejected
- wrong gateway mint authority PDA is rejected
- B1C7 dangerous compile gate remains protected

## Blocker impact

- Blocker C, B1C7 handler presence verification: open, design started
- Blocker D, state initialization instruction design: open, design started
- Blocker E, SPL mint authority architecture: open
- Blocker F, guardian set testnet descriptor: open
- Blocker H, local validator dry-run: open

No blocker is closed by this document.

## Source references detected

### Handler and instruction references

- programs/xxxl-svm/src/account_contract.rs:L11: compile_error!("phase-41k6-b1-v3-account-contract-test-gate introduces the B1 V3 ConsumeGatewayMint account contract skeleton. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature.");
- programs/xxxl-svm/src/account_contract.rs:L34:     InstructionsSysvar,
- programs/xxxl-svm/src/account_contract.rs:L38: pub struct ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L46: pub const CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT: [ConsumeGatewayMintAccountContractEntry; 11] = [
- programs/xxxl-svm/src/account_contract.rs:L47:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L54:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L61:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L72:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L79:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L86:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L93:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L100:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L107:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L114:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L121:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L131: pub const CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT: [ConsumeGatewayMintAccountContractEntry;
- programs/xxxl-svm/src/account_contract.rs:L133:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L140:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L147:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L154:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L161:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L168:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L175:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L182:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L189:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L196:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L203:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L210:     ConsumeGatewayMintAccountContractEntry {
- programs/xxxl-svm/src/account_contract.rs:L212:         name: "instructions_sysvar",
- programs/xxxl-svm/src/account_contract.rs:L215:         owner_model: AccountOwnerModel::InstructionsSysvar,
- programs/xxxl-svm/src/account_contract.rs:L219: pub fn consume_gateway_mint_account_contract() -> &'static [ConsumeGatewayMintAccountContractEntry]
- programs/xxxl-svm/src/account_contract.rs:L221:     &CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT
- programs/xxxl-svm/src/account_contract.rs:L225: pub fn b1_v3_consume_gateway_mint_account_contract(
- programs/xxxl-svm/src/account_contract.rs:L226: ) -> &'static [ConsumeGatewayMintAccountContractEntry] {
- programs/xxxl-svm/src/account_contract.rs:L227:     &CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT
- programs/xxxl-svm/src/account_contract.rs:L231: pub fn assert_b1_v3_consume_gateway_mint_account_contract(
- programs/xxxl-svm/src/account_contract.rs:L234:     if accounts.len() != CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT.len() {
- programs/xxxl-svm/src/account_contract.rs:L235:         return Err(XxxlError::InvalidInstruction.into());
- programs/xxxl-svm/src/account_contract.rs:L238:     for entry in CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT {
- programs/xxxl-svm/src/account_contract.rs:L241:             .ok_or_else(|| ProgramError::from(XxxlError::InvalidInstruction))?;
- programs/xxxl-svm/src/account_contract.rs:L247:             return Err(XxxlError::InvalidInstruction.into());
- programs/xxxl-svm/src/account_contract.rs:L251:     let instructions_sysvar = accounts
- programs/xxxl-svm/src/account_contract.rs:L253:         .ok_or_else(|| ProgramError::from(XxxlError::InvalidInstruction))?;
- programs/xxxl-svm/src/account_contract.rs:L255:     if instructions_sysvar.key != &sysvar::instructions::id() {
- programs/xxxl-svm/src/account_contract.rs:L256:         return Err(XxxlError::InvalidInstruction.into());
- programs/xxxl-svm/src/account_contract.rs:L262: pub fn consume_gateway_mint_account_contract_entry(
- programs/xxxl-svm/src/account_contract.rs:L264: ) -> Option<ConsumeGatewayMintAccountContractEntry> {
- programs/xxxl-svm/src/account_contract.rs:L265:     CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT
- programs/xxxl-svm/src/account_contract.rs:L271: pub fn assert_consume_gateway_mint_account_contract(
- programs/xxxl-svm/src/account_contract.rs:L274:     if accounts.len() != CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT.len() {
- programs/xxxl-svm/src/account_contract.rs:L275:         return Err(XxxlError::InvalidInstruction.into());
- programs/xxxl-svm/src/account_contract.rs:L278:     for entry in CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT {
- programs/xxxl-svm/src/account_contract.rs:L281:             .ok_or_else(|| ProgramError::from(XxxlError::InvalidInstruction))?;
- programs/xxxl-svm/src/account_contract.rs:L287:             return Err(XxxlError::InvalidInstruction.into());
- programs/xxxl-svm/src/account_contract.rs:L298:         instruction::{
- programs/xxxl-svm/src/account_contract.rs:L299:             CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT,
- programs/xxxl-svm/src/account_contract.rs:L300:             CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L301:             CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L302:             CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L303:             CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX,

### State and PDA references

- programs/xxxl-svm/src/account_contract.rs:L49:         name: "mint_state",
- programs/xxxl-svm/src/account_contract.rs:L56:         name: "gateway_config",
- programs/xxxl-svm/src/account_contract.rs:L63:         name: "guardian_set",
- programs/xxxl-svm/src/account_contract.rs:L69:     // The processed_event account is the replay-protection PDA. It may enter
- programs/xxxl-svm/src/account_contract.rs:L74:         name: "processed_event",
- programs/xxxl-svm/src/account_contract.rs:L135:         name: "mint_state",
- programs/xxxl-svm/src/account_contract.rs:L142:         name: "gateway_config",
- programs/xxxl-svm/src/account_contract.rs:L149:         name: "guardian_set",
- programs/xxxl-svm/src/account_contract.rs:L156:         name: "processed_event",
- programs/xxxl-svm/src/account_contract.rs:L300:             CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L301:             CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L302:             CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
- programs/xxxl-svm/src/account_contract.rs:L307:             ACCOUNT_INDEX_GATEWAY_CONFIG, ACCOUNT_INDEX_GUARDIAN_SET,
- programs/xxxl-svm/src/account_contract.rs:L308:             ACCOUNT_INDEX_MINT_AUTHORITY_PDA, ACCOUNT_INDEX_MINT_STATE,
- programs/xxxl-svm/src/account_contract.rs:L309:             ACCOUNT_INDEX_PROCESSED_EVENT, ACCOUNT_INDEX_RECIPIENT_BALANCE,
- programs/xxxl-svm/src/account_contract.rs:L333:         assert_entry(ACCOUNT_INDEX_MINT_STATE, "mint_state");
- programs/xxxl-svm/src/account_contract.rs:L334:         assert_entry(ACCOUNT_INDEX_GATEWAY_CONFIG, "gateway_config");
- programs/xxxl-svm/src/account_contract.rs:L335:         assert_entry(ACCOUNT_INDEX_GUARDIAN_SET, "guardian_set");
- programs/xxxl-svm/src/account_contract.rs:L336:         assert_entry(ACCOUNT_INDEX_PROCESSED_EVENT, "processed_event");
- programs/xxxl-svm/src/account_contract.rs:L352:             ACCOUNT_INDEX_MINT_STATE as u8,
- programs/xxxl-svm/src/account_contract.rs:L353:             CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX
- programs/xxxl-svm/src/account_contract.rs:L356:             ACCOUNT_INDEX_GATEWAY_CONFIG as u8,
- programs/xxxl-svm/src/account_contract.rs:L360:             ACCOUNT_INDEX_GUARDIAN_SET as u8,
- programs/xxxl-svm/src/account_contract.rs:L361:             CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX
- programs/xxxl-svm/src/account_contract.rs:L364:             ACCOUNT_INDEX_PROCESSED_EVENT as u8,
- programs/xxxl-svm/src/account_contract.rs:L365:             CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX
- programs/xxxl-svm/src/account_contract.rs:L375:         assert_readonly(ACCOUNT_INDEX_MINT_STATE);
- programs/xxxl-svm/src/account_contract.rs:L376:         assert_readonly(ACCOUNT_INDEX_GATEWAY_CONFIG);
- programs/xxxl-svm/src/account_contract.rs:L377:         assert_readonly(ACCOUNT_INDEX_GUARDIAN_SET);
- programs/xxxl-svm/src/account_contract.rs:L378:         assert_writable(ACCOUNT_INDEX_PROCESSED_EVENT);
- programs/xxxl-svm/src/account_contract.rs:L563:         assert_owner_model(ACCOUNT_INDEX_MINT_STATE, AccountOwnerModel::ProgramOwned);
- programs/xxxl-svm/src/account_contract.rs:L565:             ACCOUNT_INDEX_GATEWAY_CONFIG,
- programs/xxxl-svm/src/account_contract.rs:L568:         assert_owner_model(ACCOUNT_INDEX_GUARDIAN_SET, AccountOwnerModel::ProgramOwned);
- programs/xxxl-svm/src/account_contract.rs:L570:             ACCOUNT_INDEX_PROCESSED_EVENT,
- programs/xxxl-svm/src/cpi.rs:L815:         assert_eq!(seeds[1], b"gateway-mint-authority");
- programs/xxxl-svm/src/deployment_status.rs:L74:         code: "PRODUCTION_GUARDIAN_SET_UNSET",
- programs/xxxl-svm/src/deployment_status.rs:L132:                 "PRODUCTION_GUARDIAN_SET_UNSET"
- programs/xxxl-svm/src/deployment_status.rs:L318:         assert_eq!(blockers[3].code(), "PRODUCTION_GUARDIAN_SET_UNSET");
- programs/xxxl-svm/src/deployment_status.rs:L462:                 "PRODUCTION_GUARDIAN_SET_UNSET",
- programs/xxxl-svm/src/deployment_status.rs:L493:         assert_eq!(report.blockers[3].code, "PRODUCTION_GUARDIAN_SET_UNSET");
- programs/xxxl-svm/src/execution_plan.rs:L10: use crate::state::mark_processed_event_consumed_legacy_planning_only;
- programs/xxxl-svm/src/execution_plan.rs:L87: pub fn apply_processed_event_mutation_boundary(
- programs/xxxl-svm/src/execution_plan.rs:L88:     processed_event_data: &mut [u8],
- programs/xxxl-svm/src/execution_plan.rs:L100:     mark_processed_event_consumed_legacy_planning_only(
- programs/xxxl-svm/src/execution_plan.rs:L101:         processed_event_data,
- programs/xxxl-svm/src/execution_plan.rs:L134:     processed_event_data: &mut [u8],
- programs/xxxl-svm/src/execution_plan.rs:L148:         let processed_event = ProcessedEventAccountView::new(processed_event_data)?;
- programs/xxxl-svm/src/execution_plan.rs:L150:         if processed_event.consumed()
- programs/xxxl-svm/src/execution_plan.rs:L151:             || processed_event.canonical_event_key() != execution_plan.canonical_event_key
- programs/xxxl-svm/src/execution_plan.rs:L152:             || processed_event.route_id() != execution_plan.route_id
- programs/xxxl-svm/src/execution_plan.rs:L153:             || processed_event.recipient() != execution_plan.recipient
- programs/xxxl-svm/src/execution_plan.rs:L174:     apply_processed_event_mutation_boundary(processed_event_data, execution_plan)?;
- programs/xxxl-svm/src/execution_plan.rs:L181:     processed_event_data: &mut [u8],
- programs/xxxl-svm/src/execution_plan.rs:L191:         let processed_event = ProcessedEventAccountView::new(processed_event_data)?;
- programs/xxxl-svm/src/execution_plan.rs:L193:         if processed_event.consumed()
- programs/xxxl-svm/src/execution_plan.rs:L194:             || processed_event.canonical_event_key() != args.canonical_event_key
- programs/xxxl-svm/src/execution_plan.rs:L195:             || processed_event.route_id() != args.route_id
- programs/xxxl-svm/src/execution_plan.rs:L196:             || processed_event.recipient() != args.recipient
- programs/xxxl-svm/src/execution_plan.rs:L215:     mark_processed_event_consumed_legacy_planning_only(
- programs/xxxl-svm/src/execution_plan.rs:L216:         processed_event_data,
- programs/xxxl-svm/src/execution_plan.rs:L241:             GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
- programs/xxxl-svm/src/execution_plan.rs:L242:             PROCESSED_EVENT_ACCOUNT_LEN, RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
- programs/xxxl-svm/src/execution_plan.rs:L443:     fn processed_event_mutation_boundary_marks_event_from_execution_plan() {
- programs/xxxl-svm/src/execution_plan.rs:L446:         let mut processed_event_data = valid_processed_event_data(&args, false);
- programs/xxxl-svm/src/execution_plan.rs:L448:         apply_processed_event_mutation_boundary(&mut processed_event_data, &plan)
- programs/xxxl-svm/src/execution_plan.rs:L451:         let processed_event =
- programs/xxxl-svm/src/execution_plan.rs:L452:             ProcessedEventAccountView::new(&processed_event_data).expect("processed event");
- programs/xxxl-svm/src/execution_plan.rs:L454:         assert!(processed_event.consumed());
- programs/xxxl-svm/src/execution_plan.rs:L455:         assert_eq!(processed_event.consumed_amount(), 1_000);
- programs/xxxl-svm/src/execution_plan.rs:L456:         assert_eq!(read_u64_le(&processed_event_data, 128), 77);
- programs/xxxl-svm/src/execution_plan.rs:L460:     fn processed_event_mutation_boundary_rejects_replay_without_changes() {
- programs/xxxl-svm/src/execution_plan.rs:L463:         let mut processed_event_data = valid_processed_event_data(&args, true);
- programs/xxxl-svm/src/execution_plan.rs:L464:         let before = processed_event_data.clone();
- programs/xxxl-svm/src/execution_plan.rs:L467:             apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),
- programs/xxxl-svm/src/execution_plan.rs:L471:         assert_eq!(processed_event_data, before);
- programs/xxxl-svm/src/execution_plan.rs:L475:     fn processed_event_mutation_boundary_rejects_wrong_event_key_without_changes() {
- programs/xxxl-svm/src/execution_plan.rs:L478:         let mut processed_event_data = valid_processed_event_data(&args, false);
- programs/xxxl-svm/src/execution_plan.rs:L479:         processed_event_data[16] ^= 0xff;
- programs/xxxl-svm/src/execution_plan.rs:L480:         let before = processed_event_data.clone();
- programs/xxxl-svm/src/execution_plan.rs:L483:             apply_processed_event_mutation_boundary(&mut processed_event_data, &plan),

### Guard references

- programs/xxxl-svm/src/account_contract.rs:L11: compile_error!("phase-41k6-b1-v3-account-contract-test-gate introduces the B1 V3 ConsumeGatewayMint account contract skeleton. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature.");
- programs/xxxl-svm/src/cpi.rs:L10: compile_error!(
- programs/xxxl-svm/src/cpi.rs:L20:     not(feature = "phase-41k6-b1c7-handler-integration-test-gate")
- programs/xxxl-svm/src/cpi.rs:L22: compile_error!(
- programs/xxxl-svm/src/cpi.rs:L23:     "phase-41k5-d2-production-path-test-gate cannot open SPL mint CPI without      phase-41k6-b1c7-handler-integration-test-gate. B1 closure requires guardian      authorization before any live mark+mint path."
- programs/xxxl-svm/src/cpi.rs:L120:     feature = "phase-41k6-b1c7-handler-integration-test-gate",
- programs/xxxl-svm/src/cpi.rs:L121:     feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
- programs/xxxl-svm/src/cpi.rs:L124:     let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_OPEN_AFTER_B1C7";
- programs/xxxl-svm/src/cpi.rs:L131:     feature = "phase-41k6-b1c7-handler-integration-test-gate",
- programs/xxxl-svm/src/cpi.rs:L132:     feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
- programs/xxxl-svm/src/deployment_status.rs:L95:         status_code: "SCAFFOLD_ONLY_NOT_DEPLOYABLE",
- programs/xxxl-svm/src/deployment_status.rs:L105:                 "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
- programs/xxxl-svm/src/deployment_status.rs:L275:     fn runtime_status_is_scaffold_only_not_deployable() {
- programs/xxxl-svm/src/deployment_status.rs:L281:         assert_eq!(XXXL_RUNTIME_STATUS, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- programs/xxxl-svm/src/deployment_status.rs:L288:             "SCAFFOLD_ONLY_NOT_DEPLOYABLE"
- programs/xxxl-svm/src/deployment_status.rs:L483:         assert_eq!(report.status_code, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- programs/xxxl-svm/src/deployment_status.rs:L520:                 assert_eq!(report.status_code, "SCAFFOLD_ONLY_NOT_DEPLOYABLE");
- programs/xxxl-svm/src/lib.rs:L37: pub const XXXL_RUNTIME_STATUS: &str = "SCAFFOLD_ONLY_NOT_DEPLOYABLE";
- programs/xxxl-svm/src/phase_41k5_d15_atomic_mark_and_mint_svm_harness.rs:L2: compile_error!(
- programs/xxxl-svm/src/processed_event_marking_svm_harness.rs:L2: compile_error!(
- programs/xxxl-svm/src/processor.rs:L16: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L19: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L21:     establish_b1c7_handler_authorization_before_mark_and_mint, B1C7HandlerAuthorizationResult,
- programs/xxxl-svm/src/processor.rs:L22:     B1C7HandlerAuthorizationStatus, B1CAuthorizationPayloadContext,
- programs/xxxl-svm/src/processor.rs:L25: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L28: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L31: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L66: compile_error!(
- programs/xxxl-svm/src/processor.rs:L91:     feature = "phase-41k6-b1c7-handler-integration-test-gate",
- programs/xxxl-svm/src/processor.rs:L92:     not(feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build")
- programs/xxxl-svm/src/processor.rs:L94: compile_error!(
- programs/xxxl-svm/src/processor.rs:L95:     "phase-41k6-b1c7-handler-integration-test-gate wires the guardian-authorized ConsumeGatewayMint mark+mint path. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature."
- programs/xxxl-svm/src/processor.rs:L195: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L196: pub struct RuntimeConsumeGatewayMintB1C7AuthorizedMarkAndMintComposition {
- programs/xxxl-svm/src/processor.rs:L197:     pub authorization: B1C7HandlerAuthorizationResult,
- programs/xxxl-svm/src/processor.rs:L205: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L206: pub fn b1c7_authorized_consume_gateway_mint_handler_boundary(
- programs/xxxl-svm/src/processor.rs:L212: ) -> Result<RuntimeConsumeGatewayMintB1C7AuthorizedMarkAndMintComposition, ProgramError> {
- programs/xxxl-svm/src/processor.rs:L213:     let authorization = establish_b1c7_consume_gateway_mint_authorization_from_handler_inputs(
- programs/xxxl-svm/src/processor.rs:L221:     b1c7_atomic_mark_and_mint_after_authorization_boundary(
- programs/xxxl-svm/src/processor.rs:L231: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L232: pub fn establish_b1c7_consume_gateway_mint_authorization_from_handler_inputs(
- programs/xxxl-svm/src/processor.rs:L237: ) -> Result<B1C7HandlerAuthorizationResult, ProgramError> {
- programs/xxxl-svm/src/processor.rs:L294:     let authorization = establish_b1c7_handler_authorization_before_mark_and_mint(
- programs/xxxl-svm/src/processor.rs:L300:     if authorization.status != B1C7HandlerAuthorizationStatus::Authorized
- programs/xxxl-svm/src/processor.rs:L309: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L310: pub fn b1c7_atomic_mark_and_mint_after_authorization_boundary(
- programs/xxxl-svm/src/processor.rs:L316:     authorization: B1C7HandlerAuthorizationResult,
- programs/xxxl-svm/src/processor.rs:L317: ) -> Result<RuntimeConsumeGatewayMintB1C7AuthorizedMarkAndMintComposition, ProgramError> {
- programs/xxxl-svm/src/processor.rs:L318:     if authorization.status != B1C7HandlerAuthorizationStatus::Authorized
- programs/xxxl-svm/src/processor.rs:L337:         RuntimeConsumeGatewayMintB1C7AuthorizedMarkAndMintComposition {
- programs/xxxl-svm/src/processor.rs:L386: #[cfg(not(feature = "phase-41k6-b1c7-handler-integration-test-gate"))]
- programs/xxxl-svm/src/processor.rs:L392:     msg!("XXXL consume_gateway_mint requires B1C7 guardian authorization before mark + mint");
- programs/xxxl-svm/src/processor.rs:L396: #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L405:     b1c7_authorized_consume_gateway_mint_handler_boundary(
- programs/xxxl-svm/src/processor.rs:L794:     #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L795:     use crate::verifier::B1C7HandlerAuthorizationRejectionKind;
- programs/xxxl-svm/src/processor.rs:L2342:     #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L2343:     fn b1c7_rejected_authorization_result() -> B1C7HandlerAuthorizationResult {
- programs/xxxl-svm/src/processor.rs:L2344:         B1C7HandlerAuthorizationResult {
- programs/xxxl-svm/src/processor.rs:L2345:             status: B1C7HandlerAuthorizationStatus::Rejected,
- programs/xxxl-svm/src/processor.rs:L2346:             rejection_kind: Some(B1C7HandlerAuthorizationRejectionKind::QuorumRejected),
- programs/xxxl-svm/src/processor.rs:L2364:     #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L2365:     fn b1c7_authorized_result_for_cpi_gate_test() -> B1C7HandlerAuthorizationResult {
- programs/xxxl-svm/src/processor.rs:L2366:         B1C7HandlerAuthorizationResult {
- programs/xxxl-svm/src/processor.rs:L2367:             status: B1C7HandlerAuthorizationStatus::Authorized,
- programs/xxxl-svm/src/processor.rs:L2386:     #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L2388:     fn b1c7_rejected_authorization_blocks_mark_and_mint_before_mutation() {
- programs/xxxl-svm/src/processor.rs:L2402:             b1c7_atomic_mark_and_mint_after_authorization_boundary(
- programs/xxxl-svm/src/processor.rs:L2408:                 b1c7_rejected_authorization_result(),
- programs/xxxl-svm/src/processor.rs:L2424:     #[cfg(feature = "phase-41k6-b1c7-handler-integration-test-gate")]
- programs/xxxl-svm/src/processor.rs:L2426:     fn b1c7_authorized_result_still_blocks_before_mutation_when_cpi_gate_closed() {
- programs/xxxl-svm/src/processor.rs:L2440:             b1c7_atomic_mark_and_mint_after_authorization_boundary(
- programs/xxxl-svm/src/processor.rs:L2446:                 b1c7_authorized_result_for_cpi_gate_test(),
- programs/xxxl-svm/src/program_id_status.rs:L29:         status_code: "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L41:             XxxlProgramIdReadinessStatus::Placeholder => "PLACEHOLDER_PROGRAM_ID_BOUNDARY",
- programs/xxxl-svm/src/program_id_status.rs:L86:         assert_eq!(status.code(), "PLACEHOLDER_PROGRAM_ID_BOUNDARY");
- programs/xxxl-svm/src/program_id_status.rs:L98:         assert_eq!(report.status_code, "PLACEHOLDER_PROGRAM_ID_BOUNDARY");

## Recommended next boundary

The next boundary should be:

B6.10 — State account and instruction design skeleton

B6.10 may introduce local code skeletons and tests for instruction/state design only.

B6.10 must not create a deployable artifact GO.

B6.10 must not sign, submit, spend SOL, deploy, upgrade, or initialize testnet accounts.

## Current decision

Current decision:

NO-GO.

This B6.9 runtime upgrade target design does not authorize live action.

## B6.10 state and instruction design skeleton

B6.10 state and instruction design skeleton is recorded in:

docs/gateway/phase-41k6-b6-10-state-instruction-design-skeleton.md

This advances planning for blockers C, D, E, and F, but closes no GO blocker.

Current decision remains:

NO-GO.
