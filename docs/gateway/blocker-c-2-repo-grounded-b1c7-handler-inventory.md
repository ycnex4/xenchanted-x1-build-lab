# Blocker C.2 — Repo-grounded B1C7 handler inventory

Status:

BLOCKER_C_OPEN_REPO_GROUNDED_B1C7_HANDLER_INVENTORY_COMPLETED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker C.2 records a repo-grounded inventory of the B1C7 handler path.

C.2 is inventory-only.

It does not activate the handler.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate any network.

## Inventory evidence files

- docs/gateway/evidence/blocker-c-2-repo-grounded-b1c7-handler-inventory/metadata.txt
- docs/gateway/evidence/blocker-c-2-repo-grounded-b1c7-handler-inventory/source-file-list.txt
- docs/gateway/evidence/blocker-c-2-repo-grounded-b1c7-handler-inventory/b1c7-inventory-grep.txt
- docs/gateway/evidence/blocker-c-2-repo-grounded-b1c7-handler-inventory/inventory-summary.txt

## Source files inventoried

- programs/xxxl-svm/Cargo.toml
- programs/xxxl-svm/src/lib.rs
- programs/xxxl-svm/src/processor.rs
- programs/xxxl-svm/src/account_contract.rs
- programs/xxxl-svm/src/cpi.rs
- programs/xxxl-svm/src/execution_plan.rs
- programs/xxxl-svm/src/processed_event_marking_boundary.rs
- programs/xxxl-svm/src/instruction.rs
- programs/xxxl-svm/src/state.rs
- programs/xxxl-svm/src/deployment_status.rs
- programs/xxxl-svm/src/program_id_status.rs

## Static inventory summary

- cargo_b1c7_feature_present: true
- cargo_b1c7_dangerous_allow_present: true
- processor_compile_error_for_b1c7_without_dangerous_allow: true
- processor_live_route_flag_false: true
- processor_default_path_fails_cpi_not_ready: true
- processor_b1c7_handler_boundary_present: true
- processor_b1c7_authorization_from_inputs_present: true
- processor_b1c7_atomic_boundary_present: true
- account_contract_b1_v3_12_account_contract_present: true
- cpi_execution_false_default_present: true
- cpi_execution_true_requires_b1c7_and_dangerous_allows: true
- deployment_still_not_deployable: true
- program_id_placeholder_active: true
- handler_calls_authorization_before_atomic_mark_and_mint: true
- atomic_boundary_checks_cpi_gate_before_atomic_mark_and_mint_call: true
- atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: true

## Handler path inventory

C.2 identifies the following B1C7 path:

1. process_instruction decodes XxxlInstruction::ConsumeGatewayMint.
2. Without the B1C7 feature gate, process_consume_gateway_mint fails closed with CpiBoundaryNotReady.
3. With the B1C7 feature gate, process_consume_gateway_mint obtains Rent and Clock and calls b1c7_authorized_consume_gateway_mint_handler_boundary.
4. The B1C7 handler first establishes authorization from handler inputs.
5. Authorization loads and validates the B1 V3 account contract, CPI preparation boundary, guardian set account, instructions sysvar, bounded prior instructions, processed_event account, amount, and payload context.
6. Authorization must return Authorized before mutation can proceed.
7. The atomic B1C7 boundary rechecks authorization status, fail-fast-before-mutation, prior Ed25519 evidence, payload hash binding, guardian membership, and quorum.
8. The atomic B1C7 boundary checks the SPL CPI execution gate before calling atomic_mark_and_mint_boundary.
9. Inside atomic_mark_and_mint_boundary, processed_event marking is performed before the guarded SPL CPI call.
10. The guarded SPL CPI call remains closed unless the explicit SPL CPI and B1C7 dangerous test-gate feature combination is enabled.

## Account contract inventory

C.2 identifies two account-contract layers:

- legacy 11-account ConsumeGatewayMint contract
- B1 V3 12-account contract behind phase-41k6-b1-v3-account-contract-test-gate

The B1 V3 contract adds instructions_sysvar at index 11.

The B1 V3 account contract remains test-gated and has a compile_error guard unless the explicit dangerous SBF build allow feature is present.

## CPI gate inventory

C.2 identifies the SPL mint_to CPI gate:

- spl_mint_to_cpi_execution_enabled() returns false by default
- true requires phase-41k5-d2-production-path-test-gate
- true also requires dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build
- true also requires phase-41k6-b1c7-handler-integration-test-gate
- true also requires dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build

Therefore, SPL CPI remains closed in the normal source configuration.

## Current safety interpretation

The current repo has a meaningful B1C7 handler path, but it is still an integration/test-gated path.

Current safety posture:

- default path fails closed
- live route activation remains false
- SPL CPI execution remains false by default
- Program ID placeholder boundary remains active
- deployment_status remains deployable=false

## Current gaps before Blocker C closure

Blocker C cannot close yet because the following remain unresolved:

- no production/testnet handler activation decision
- no decision on whether test-gated features become testnet-intended features or remain test-only
- no deployable artifact hash
- no local-validator program-load handler evidence
- D/E/F/B/G still open
- no final testnet GO package

## Non-closure statement

C.2 does not close Blocker C.

C.2 does not approve:

- handler activation
- live route activation
- signing
- keypair use
- deploy
- write-buffer
- set-upgrade-authority
- close
- upgrade
- state initialization
- SPL mint setup
- SPL CPI minting
- guardian package construction
- transaction submit
- mutation
- production activation

## Result

Current status:

BLOCKER_C_OPEN_REPO_GROUNDED_B1C7_HANDLER_INVENTORY_COMPLETED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker C.3 — B1C7 production/testnet activation decision model.

C.3 should decide what must change, if anything, for the current test-gated B1C7 path to become a reviewed testnet-intended path.

C.3 must not activate the handler, call RPC, build deployable artifacts, sign, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-c-2-repo-grounded-b1c7-handler-inventory
timestamp_utc=2026-07-06T17:16:10Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
handler_activated=false
build_executed=false
mutation_executed=false
```

source file list:

```text
programs/xxxl-svm/Cargo.toml
programs/xxxl-svm/examples/emit_local_fixtures_b6_58.rs
programs/xxxl-svm/src/account_contract.rs
programs/xxxl-svm/src/account_contract.rs
programs/xxxl-svm/src/account_order_skeleton.rs
programs/xxxl-svm/src/account_order_skeleton.rs
programs/xxxl-svm/src/account_validation_skeleton.rs
programs/xxxl-svm/src/account_validation_skeleton.rs
programs/xxxl-svm/src/consume_execution_plan_skeleton.rs
programs/xxxl-svm/src/consume_execution_plan_skeleton.rs
programs/xxxl-svm/src/consume_state_transition_skeleton.rs
programs/xxxl-svm/src/consume_state_transition_skeleton.rs
programs/xxxl-svm/src/cpi.rs
programs/xxxl-svm/src/cpi.rs
programs/xxxl-svm/src/deployment_status.rs
programs/xxxl-svm/src/deployment_status.rs
programs/xxxl-svm/src/dispatch_skeleton.rs
programs/xxxl-svm/src/dispatch_skeleton.rs
programs/xxxl-svm/src/entrypoint.rs
programs/xxxl-svm/src/entrypoint.rs
programs/xxxl-svm/src/error.rs
programs/xxxl-svm/src/error.rs
programs/xxxl-svm/src/execution_plan.rs
programs/xxxl-svm/src/execution_plan.rs
programs/xxxl-svm/src/initialization_execution_plan_skeleton.rs
programs/xxxl-svm/src/initialization_execution_plan_skeleton.rs
programs/xxxl-svm/src/instruction.rs
programs/xxxl-svm/src/instruction.rs
programs/xxxl-svm/src/instruction_codec_skeleton.rs
programs/xxxl-svm/src/instruction_codec_skeleton.rs
programs/xxxl-svm/src/instruction_payload_skeleton.rs
programs/xxxl-svm/src/instruction_payload_skeleton.rs
programs/xxxl-svm/src/lib.rs
programs/xxxl-svm/src/lib.rs
programs/xxxl-svm/src/local_execution_plan_skeleton.rs
programs/xxxl-svm/src/local_execution_plan_skeleton.rs
programs/xxxl-svm/src/local_execution_scenario_skeleton.rs
programs/xxxl-svm/src/local_execution_scenario_skeleton.rs
programs/xxxl-svm/src/local_fixture_file_emitter_skeleton.rs
programs/xxxl-svm/src/local_fixture_file_emitter_skeleton.rs
programs/xxxl-svm/src/local_fixture_generator_skeleton.rs
programs/xxxl-svm/src/local_fixture_generator_skeleton.rs
programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs
programs/xxxl-svm/src/local_guardian_descriptor_skeleton.rs
programs/xxxl-svm/src/local_guardian_failure_matrix_skeleton.rs
programs/xxxl-svm/src/local_guardian_failure_matrix_skeleton.rs
programs/xxxl-svm/src/local_guardian_fixture_integration_skeleton.rs
programs/xxxl-svm/src/local_guardian_fixture_integration_skeleton.rs
programs/xxxl-svm/src/pda.rs
programs/xxxl-svm/src/pda.rs
programs/xxxl-svm/src/phase_41k5_d15_atomic_mark_and_mint_svm_harness.rs
programs/xxxl-svm/src/phase_41k5_d15_atomic_mark_and_mint_svm_harness.rs
programs/xxxl-svm/src/processed_event_marking_boundary.rs
programs/xxxl-svm/src/processed_event_marking_boundary.rs
programs/xxxl-svm/src/processed_event_marking_svm_harness.rs
programs/xxxl-svm/src/processed_event_marking_svm_harness.rs
programs/xxxl-svm/src/processor.rs
programs/xxxl-svm/src/processor.rs
programs/xxxl-svm/src/program_id_status.rs
programs/xxxl-svm/src/program_id_status.rs
programs/xxxl-svm/src/safety_invariants.rs
programs/xxxl-svm/src/safety_invariants.rs
programs/xxxl-svm/src/state.rs
programs/xxxl-svm/src/state.rs
programs/xxxl-svm/src/state_account_layout_skeleton.rs
programs/xxxl-svm/src/state_account_layout_skeleton.rs
programs/xxxl-svm/src/state_initialization_skeleton.rs
programs/xxxl-svm/src/state_initialization_skeleton.rs
programs/xxxl-svm/src/state_instruction_skeleton.rs
programs/xxxl-svm/src/state_instruction_skeleton.rs
programs/xxxl-svm/src/typed_instruction_skeleton.rs
programs/xxxl-svm/src/typed_instruction_skeleton.rs
programs/xxxl-svm/src/validated_dispatch_skeleton.rs
programs/xxxl-svm/src/validated_dispatch_skeleton.rs
programs/xxxl-svm/src/validation.rs
programs/xxxl-svm/src/validation.rs
programs/xxxl-svm/src/verifier/b1c7_handler_authorization_boundary.rs
programs/xxxl-svm/src/verifier/b1c_connect_ed25519_evidence_adapter.rs
programs/xxxl-svm/src/verifier/b1c_ed25519_evidence_authorization_result.rs
programs/xxxl-svm/src/verifier/b1c_ed25519_evidence_parser.r
```

grep preview:

```text
programs/xxxl-svm/Cargo.toml:17:  "phase-41k6-b1c7-handler-integration-test-gate",
programs/xxxl-svm/Cargo.toml:20:  "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build",
programs/xxxl-svm/Cargo.toml:22:phase-41k6-b1-v3-account-contract-test-gate = []
programs/xxxl-svm/Cargo.toml:23:dangerously-allow-phase-41k6-b1-v3-account-contract-test-gate-sbf-build = []
programs/xxxl-svm/Cargo.toml:24:phase-41k6-b1b-guardian-set-loading-test-gate = []
programs/xxxl-svm/Cargo.toml:25:dangerously-allow-phase-41k6-b1b-guardian-set-loading-test-gate-sbf-build = []
programs/xxxl-svm/Cargo.toml:26:phase-41k6-b1c-ed25519-evidence-wiring-test-gate = []
programs/xxxl-svm/Cargo.toml:27:dangerously-allow-phase-41k6-b1c-ed25519-evidence-wiring-test-gate-sbf-build = []
programs/xxxl-svm/Cargo.toml:28:phase-41k6-b1c7-handler-integration-test-gate = [
programs/xxxl-svm/Cargo.toml:29:  "phase-41k6-b1-v3-account-contract-test-gate",
programs/xxxl-svm/Cargo.toml:30:  "phase-41k6-b1b-guardian-set-loading-test-gate",
programs/xxxl-svm/Cargo.toml:31:  "phase-41k6-b1c-ed25519-evidence-wiring-test-gate",
programs/xxxl-svm/Cargo.toml:33:dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build = [
programs/xxxl-svm/Cargo.toml:34:  "dangerously-allow-phase-41k6-b1-v3-account-contract-test-gate-sbf-build",
programs/xxxl-svm/Cargo.toml:35:  "dangerously-allow-phase-41k6-b1b-guardian-set-loading-test-gate-sbf-build",
programs/xxxl-svm/Cargo.toml:36:  "dangerously-allow-phase-41k6-b1c-ed25519-evidence-wiring-test-gate-sbf-build",
programs/xxxl-svm/src/account_contract.rs:1:#[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:8:    feature = "phase-41k6-b1-v3-account-contract-test-gate",
programs/xxxl-svm/src/account_contract.rs:9:    not(feature = "dangerously-allow-phase-41k6-b1-v3-account-contract-test-gate-sbf-build")
programs/xxxl-svm/src/account_contract.rs:11:compile_error!("phase-41k6-b1-v3-account-contract-test-gate introduces the B1 V3 ConsumeGatewayMint account contract skeleton. It is a non-production integration gate and must never be included in deploy artifacts without the explicit dangerous test allow feature.");
programs/xxxl-svm/src/account_contract.rs:130:#[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:212:        name: "instructions_sysvar",
programs/xxxl-svm/src/account_contract.rs:224:#[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:230:#[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:231:pub fn assert_b1_v3_consume_gateway_mint_account_contract(
programs/xxxl-svm/src/account_contract.rs:251:    let instructions_sysvar = accounts
programs/xxxl-svm/src/account_contract.rs:255:    if instructions_sysvar.key != &sysvar::instructions::id() {
programs/xxxl-svm/src/account_contract.rs:401:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:432:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:444:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:468:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:471:            let result = assert_b1_v3_consume_gateway_mint_account_contract(&infos);
programs/xxxl-svm/src/account_contract.rs:477:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:483:            assert_b1_v3_consume_gateway_mint_account_contract(&infos)
programs/xxxl-svm/src/account_contract.rs:488:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:490:    fn b1_v3_account_contract_rejects_missing_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:497:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:499:    fn b1_v3_account_contract_rejects_wrong_instructions_sysvar_key() {
programs/xxxl-svm/src/account_contract.rs:506:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:508:    fn b1_v3_account_contract_rejects_writable_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:515:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:517:    fn b1_v3_account_contract_rejects_signer_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:524:    #[cfg(feature = "phase-41k6-b1-v3-account-contract-test-gate")]
programs/xxxl-svm/src/account_contract.rs:526:    fn b1_v3_account_contract_adds_readonly_non_signer_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:550:        assert_eq!(sysvar_entry.name, "instructions_sysvar");
programs/xxxl-svm/src/cpi.rs:20:    not(feature = "phase-41k6-b1c7-handler-integration-test-gate")
programs/xxxl-svm/src/cpi.rs:23:    "phase-41k5-d2-production-path-test-gate cannot open SPL mint CPI without      phase-41k6-b1c7-handler-integration-test-gate. B1 closure requires guardian      authorization before any live mark+mint path."
programs/xxxl-svm/src/cpi.rs:120:    feature = "phase-41k6-b1c7-handler-integration-test-gate",
programs/xxxl-svm/src/cpi.rs:121:    feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
programs/xxxl-svm/src/cpi.rs:123:pub fn spl_mint_to_cpi_execution_enabled() -> bool {
programs/xxxl-svm/src/cpi.rs:124:    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_OPEN_AFTER_B1C7";
programs/xxxl-svm/src/cpi.rs:131:    feature = "phase-41k6-b1c7-handler-integration-test-gate",
programs/xxxl-svm/src/cpi.rs:132:    feature = "dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build"
programs/xxxl-svm/src/cpi.rs:134:pub fn spl_mint_to_cpi_execution_enabled() -> bool {
programs/xxxl-svm/src/cpi.rs:139:pub fn guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/cpi.rs:150:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/cpi.rs:160:    if !spl_mint_to_cpi_execution_enabled() {
programs/xxxl-svm/src/cpi.rs:161:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/cpi.rs:373:    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_when_gate_disabled() {
programs/xxxl-svm/src/cpi.rs:380:        assert!(!spl_mint_to_cpi_execution_enabled());
programs/xxxl-svm/src/cpi.rs:398:                    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/cpi.rs:404:                    XxxlError::CpiBoundaryNotReady,
programs/xxxl-svm/src/cpi.rs:411:    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_planning_boundary_mismatch() {
programs/xxxl-svm/src/cpi.rs:435:                    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/cpi.rs:448:    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_live_route_flag_before_cpi() {
programs/xxxl-svm/src/cpi.rs:472:                    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/cpi.rs:478:                    XxxlError::CpiBoundaryNotReady,
programs/xxxl-svm/src/cpi.rs:485:    fn guarded_mint_to_cpi_execution_gate_boundary_rejects_wrong_pda_before_gate() {
programs/xxxl-svm/src/cpi.rs:513:                    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/deployment_status.rs:262:    crate::processor::LIVE_ROUTE_ACTIVATION_FROM_PROCESS_INSTRUCTION_ENABLED
programs/xxxl-svm/src/deployment_status.rs:266:    crate::cpi::spl_mint_to_cpi_execution_enabled()
programs/xxxl-svm/src/error.rs:12:    CpiBoundaryNotReady = 8,
programs/xxxl-svm/src/local_fixture_generator_skeleton.rs:62:    pub b1c7_guard_intact: bool,
programs/xxxl-svm/src/local_fixture_generator_skeleton.rs:
```

## C.2R order-check correction

The original C.2 static check reported:

atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: false

This was a tooling artifact, not a runtime gap.

Reason:

The check used a whole-file string index and matched the import/use occurrence of guarded_mint_to_cpi_execution_gate_boundary before the function body.

Corrected function-scoped check:

- function: atomic_mark_and_mint_boundary
- mark_processed_event_atomic_call_line: 556
- guarded_mint_to_cpi_execution_gate_boundary_call_line: 571
- mark_before_guarded_cpi_call: true

Corrected status:

atomic_boundary_marks_before_guarded_cpi_inside_atomic_function: true

C.2 remains inventory-only and still does not close Blocker C.

