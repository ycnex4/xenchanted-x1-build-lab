# Blocker C.4 — B1C7 invariant review package

Status:

BLOCKER_C_REVIEW_READY_B1C7_INVARIANTS_RECORDED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Purpose

Blocker C.4 records the B1C7 invariant review package.

C.4 is repo-only review evidence.

It does not activate the handler.

It does not change runtime code.

It does not build a deployable artifact.

It does not call RPC.

It does not use testnet.

It does not sign.

It does not deploy, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate any network.

## Reviewed evidence files

- docs/gateway/evidence/blocker-c-4-b1c7-invariant-review-package/metadata.txt
- docs/gateway/evidence/blocker-c-4-b1c7-invariant-review-package/invariant-grep.txt
- docs/gateway/evidence/blocker-c-4-b1c7-invariant-review-package/invariant-summary.txt
- docs/gateway/evidence/blocker-c-4-b1c7-invariant-review-package/line-map.txt

## Invariant summary

- handler_entrypoint_present: true
- handler_authorization_call_before_atomic_boundary_call: true
- authorization_account_contract_asserted: true
- authorization_guardian_set_loaded: true
- authorization_prior_instructions_loaded: true
- authorization_payload_context_constructed: true
- authorization_established_before_status_gate: true
- authorization_status_gate_before_mutation: false
- atomic_boundary_rechecks_authorized_status: true
- atomic_boundary_rechecks_fail_fast_before_mutation: true
- atomic_boundary_rechecks_prior_ed25519_evidence: true
- atomic_boundary_rechecks_payload_hash_binding: true
- atomic_boundary_rechecks_guardian_membership: true
- atomic_boundary_rechecks_quorum: true
- atomic_boundary_checks_cpi_gate_before_mark_and_mint_boundary: true
- atomic_mark_boundary_marks_before_guarded_cpi: true
- default_non_b1c7_path_fails_closed: true
- b1_v3_account_contract_present: true
- b1_v3_account_contract_has_instructions_sysvar: true
- b1_v3_account_contract_asserts_sysvar_key: true
- cpi_gate_false_default_present: true
- cpi_gate_true_requires_b1c7_and_dangerous_allows: true
- deployment_status_not_deployable: true
- program_id_placeholder_boundary_active: true

## Line map

- handler_line: 206
- auth_call_line: 213
- atomic_call_line: 221
- auth_fn_line: 232
- account_contract_assert_line: 238
- guardian_load_line: 254
- prior_load_line: 273
- payload_context_line: 285
- auth_establish_line: 294
- auth_status_gate_line: 300
- atomic_fn_line: 310
- atomic_status_gate_line: 318
- fail_fast_line: 320
- evidence_line: 321
- payload_hash_line: 322
- guardian_membership_line: 323
- quorum_line: 324
- cpi_gate_line: 329
- atomic_mark_mint_call_line: 334
- default_fail_closed_line: 392
- default_fail_error_line: 393
- atomic_mark_fn_line: 525
- mark_line: 556
- guarded_cpi_line: 571

## Review result

all_invariants_passed: false

## Invariants reviewed

C.4 reviews the following invariants:

1. Handler entrypoint is present.
2. Handler establishes authorization before calling the atomic mark+mint boundary.
3. Authorization asserts the B1 V3 account contract.
4. Authorization loads the guardian set.
5. Authorization loads checked prior instructions from the instructions sysvar boundary.
6. Authorization constructs the payload context before establishing final authorization.
7. Authorization status gate occurs before mutation.
8. Atomic boundary rechecks Authorized status.
9. Atomic boundary rechecks fail_fast_before_mutation.
10. Atomic boundary rechecks prior Ed25519 evidence.
11. Atomic boundary rechecks payload hash binding.
12. Atomic boundary rechecks guardian membership.
13. Atomic boundary rechecks quorum.
14. Atomic boundary checks the SPL CPI execution gate before calling atomic_mark_and_mint_boundary.
15. Inside atomic_mark_and_mint_boundary, processed_event marking occurs before guarded SPL CPI boundary call.
16. Default non-B1C7 ConsumeGatewayMint path fails closed with CpiBoundaryNotReady.
17. B1 V3 account contract includes instructions_sysvar.
18. CPI execution remains false by default.
19. deployment_status remains deployable=false.
20. Program ID placeholder boundary remains active.

## Interpretation

C.4 supports the conclusion that the B1C7 handler boundary has a coherent invariant structure in repo source.

However, C.4 does not approve direct dangerous test-gate deployment.

C.3 already rejected direct dangerous test-gate activation.

C.4 does not replace the need for a future reviewed testnet-intended handler route before any deployable artifact.

## Closure candidate prepared

C.4 prepares, but does not itself record, a narrow closure candidate for Blocker C:

B1C7_HANDLER_BOUNDARY_REVIEWED_DIRECT_DANGEROUS_ACTIVATION_REJECTED_FUTURE_TESTNET_ROUTE_REQUIRED

Meaning:

- current B1C7 invariants are reviewed at repo level
- direct dangerous test-gate activation remains rejected
- a future reviewed testnet-intended route remains required
- no deploy/upgrade/state/SPL/guardian/submit/mutation is approved

## Remaining dependencies

Even if Blocker C closes narrowly later, the following remain open:

- B — expected post-upgrade ProgramData hash
- D — state initialization design
- E — SPL mint authority architecture
- F — guardian descriptor
- G — rollback / recovery plan

## Non-closure statement

C.4 does not close Blocker C.

C.4 does not approve:

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

BLOCKER_C_REVIEW_READY_B1C7_INVARIANTS_RECORDED_NO_ACTIVATION

Current decision:

BLOCKER_C_NOT_CLOSED_PENDING_CLOSURE_DECISION

NO-GO REMAINS_FOR_HANDLER_ACTIVATION_SIGNING_KEYS_PROGRAM_UPGRADE_STATE_INIT_SPL_SETUP_GUARDIAN_PACKAGES_NETWORK_SUBMIT_MUTATION

## Next safe step

Blocker C.5 — B1C7 boundary closure decision record.

C.5 may close Blocker C narrowly only as a boundary/invariant closure with direct dangerous activation rejected and a future reviewed testnet-intended route required.

C.5 must not activate the handler, call RPC, build deployable artifacts, sign, upgrade, initialize state, configure SPL, construct guardian packages, submit, or mutate.

## Evidence preview

metadata:

```text
phase=blocker-c-4-b1c7-invariant-review-package
timestamp_utc=2026-07-06T17:25:32Z
repo_only=true
rpc_used=false
testnet_used=false
code_changed=false
handler_activated=false
build_executed=false
deployable_artifact_created=false
mutation_executed=false
```

grep preview:

```text
programs/xxxl-svm/src/account_contract.rs:131:pub const CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT: [ConsumeGatewayMintAccountContractEntry;
programs/xxxl-svm/src/account_contract.rs:212:        name: "instructions_sysvar",
programs/xxxl-svm/src/account_contract.rs:227:    &CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT
programs/xxxl-svm/src/account_contract.rs:234:    if accounts.len() != CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT.len() {
programs/xxxl-svm/src/account_contract.rs:238:    for entry in CONSUME_GATEWAY_MINT_B1_V3_ACCOUNT_CONTRACT {
programs/xxxl-svm/src/account_contract.rs:251:    let instructions_sysvar = accounts
programs/xxxl-svm/src/account_contract.rs:255:    if instructions_sysvar.key != &sysvar::instructions::id() {
programs/xxxl-svm/src/account_contract.rs:490:    fn b1_v3_account_contract_rejects_missing_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:499:    fn b1_v3_account_contract_rejects_wrong_instructions_sysvar_key() {
programs/xxxl-svm/src/account_contract.rs:508:    fn b1_v3_account_contract_rejects_writable_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:517:    fn b1_v3_account_contract_rejects_signer_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:526:    fn b1_v3_account_contract_adds_readonly_non_signer_instructions_sysvar() {
programs/xxxl-svm/src/account_contract.rs:550:        assert_eq!(sysvar_entry.name, "instructions_sysvar");
programs/xxxl-svm/src/cpi.rs:123:pub fn spl_mint_to_cpi_execution_enabled() -> bool {
programs/xxxl-svm/src/cpi.rs:124:    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_OPEN_AFTER_B1C7";
programs/xxxl-svm/src/cpi.rs:134:pub fn spl_mint_to_cpi_execution_enabled() -> bool {
programs/xxxl-svm/src/cpi.rs:135:    let _phase_41k5_d2_gate_marker = "PHASE_41K5_D2_SPL_CPI_GATE_CLOSED";
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
programs/xxxl-svm/src/deployment_status.rs:97:        deployable: false,
programs/xxxl-svm/src/deployment_status.rs:266:    crate::cpi::spl_mint_to_cpi_execution_enabled()
programs/xxxl-svm/src/error.rs:12:    CpiBoundaryNotReady = 8,
programs/xxxl-svm/src/phase_41k5_d15_atomic_mark_and_mint_svm_harness.rs:24:    processed_event_marking_boundary::mark_processed_event_atomic,
programs/xxxl-svm/src/phase_41k5_d15_atomic_mark_and_mint_svm_harness.rs:74:    mark_processed_event_atomic(
programs/xxxl-svm/src/processed_event_marking_boundary.rs:202:pub fn mark_processed_event_atomic<'a>(
programs/xxxl-svm/src/processed_event_marking_boundary.rs:720:            mark_processed_event_atomic(
programs/xxxl-svm/src/processed_event_marking_svm_harness.rs:20:use crate::{error::XxxlError, processed_event_marking_boundary::mark_processed_event_atomic};
programs/xxxl-svm/src/processed_event_marking_svm_harness.rs:61:    mark_processed_event_atomic(
programs/xxxl-svm/src/processor.rs:29:use crate::verifier::current_instruction_index_runtime_boundary::acquire_current_instruction_index_from_checked_instructions_sysvar;
programs/xxxl-svm/src/processor.rs:37:        assert_gateway_mint_authority_pda, guarded_mint_to_cpi_execution_gate_boundary,
programs/xxxl-svm/src/processor.rs:47:    processed_event_marking_boundary::{mark_processed_event_atomic, ProcessedEventMarkingWitness},
programs/xxxl-svm/src/processor.rs:206:pub fn b1c7_authorized_consume_gateway_mint_handler_boundary(
programs/xxxl-svm/src/processor.rs:213:    let authorization = establish_b1c7_consume_gateway_mint_authorization_from_handler_inputs(
programs/xxxl-svm/src/processor.rs:221:    b1c7_atomic_mark_and_mint_after_authorization_boundary(
programs/xxxl-svm/src/processor.rs:232:pub fn establish_b1c7_consume_gateway_mint_authorization_from_handler_inputs(
programs/xxxl-svm/src/processor.rs:252:    let instructions_sysvar_account = account_at(accounts, CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS)?;
programs/xxxl-svm/src/processor.rs:267:        acquire_current_instruction_index_from_checked_instructions_sysvar(Some(
programs/xxxl-svm/src/processor.rs:268:            instructions_sysvar_account,
programs/xxxl-svm/src/processor.rs:275:        Some(instructions_sysvar_account),
programs/xxxl-svm/src/processor.rs:300:    if authorization.status != B1C7HandlerAuthorizationStatus::Authorized
programs/xxxl-svm/src/processor.rs:310:pub fn b1c7_atomic_mark_and_mint_after_authorization_boundary(
programs/xxxl-svm/src/processor.rs:318:    if authorization.status != B1C7HandlerAuthorizationStatus::Authorized
programs/xxxl-svm/src/processor.rs:320:        || !authorization.fail_fast_before_mutation
programs/xxxl-svm/src/processor.rs:321:        || !authorization.evidence_from_prior_ed25519_instructions
programs/xxxl-svm/src/processor.rs:322:        || !authorization.payload_hash_bound
programs/xxxl-svm/src/processor.rs:323:        || !authorization.guardian_membership_validated
programs/xxxl-svm/src/processor.rs:324:        || !authorization.quorum_met
programs/xxxl-svm/src/processor.rs:329:    if !crate::cpi::spl_mint_to_cpi_execution_enabled() {
programs/xxxl-svm/src/processor.rs:330:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:334:        atomic_mark_and_mint_boundary(program_id, accounts, args, rent, consumed_slot)?;
programs/xxxl-svm/src/processor.rs:393:    Err(XxxlError::CpiBoundaryNotReady.into())
programs/xxxl-svm/src/processor.rs:405:    b1c7_authorized_consume_gateway_mint_handler_boundary(
programs/xxxl-svm/src/processor.rs:427:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:447:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:456:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:498:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:525:pub fn atomic_mark_and_mint_boundary(
programs/xxxl-svm/src/processor.rs:539:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:548:        return Err(XxxlError::CpiBoundaryNotReady.into());
programs/xxxl-svm/src/processor.rs:556:    let marking_witness = mark_processed_event_atomic(
programs/xxxl-svm/src/processor.rs:571:    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/processor.rs:625:    guarded_mint_to_cpi_execution_gate_boundary(
programs/xxxl-svm/src/processor.rs:1523:            XxxlError::CpiBoundaryNotReady,
programs/xxxl-svm/src/processor.rs:2352:            fail_fast_before_mutation: true,
programs/xxxl-svm/src/processor.rs:23
```
