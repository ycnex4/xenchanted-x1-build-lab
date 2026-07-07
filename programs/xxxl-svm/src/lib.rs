#![allow(unexpected_cfgs)] // Solana entrypoint! macro emits custom cfgs under host clippy/rustc check-cfg.

//! XXXL SVM runtime source boundary.
//!
//! This build has moved past the scaffold-only marker at source level.
//! It remains activation-blocked: no RPC mutation, no deploy/upgrade,
//! no route execution, and no SPL CPI execution are authorized here.
//!
//! Program ID, ProgramData, and upgrade-authority bindings are not changed
//! by this source-only package.

pub mod account_contract;
pub mod account_order_skeleton;
pub mod account_validation_skeleton;
pub mod consume_execution_plan_skeleton;
pub mod consume_state_transition_skeleton;
pub mod cpi;
pub mod deployment_status;
pub mod dispatch_skeleton;
pub mod entrypoint;
pub mod error;
pub mod execution_plan;
pub mod initialization_execution_plan_skeleton;
pub mod instruction;
pub mod instruction_codec_skeleton;
pub mod instruction_payload_skeleton;
pub mod local_execution_plan_skeleton;
pub mod local_execution_scenario_skeleton;
pub mod local_fixture_file_emitter_skeleton;
pub mod local_fixture_generator_skeleton;
pub mod local_guardian_descriptor_skeleton;
pub mod local_guardian_failure_matrix_skeleton;
pub mod local_guardian_fixture_integration_skeleton;
pub mod pda;
#[cfg(feature = "phase-41k5-spl-mint-to-cpi-test-gate")]
pub mod phase_41k5_d15_atomic_mark_and_mint_svm_harness;
pub mod processed_event_marking_boundary;
#[cfg(feature = "phase-41k4-svm-test-harness")]
pub mod processed_event_marking_svm_harness;
pub mod processor;
pub mod program_id_status;
pub mod safety_invariants;
pub mod state;
pub mod state_account_layout_skeleton;
pub mod state_initialization_skeleton;
pub mod state_instruction_skeleton;
pub mod typed_instruction_skeleton;
pub mod validated_dispatch_skeleton;
pub mod validation;
pub mod verifier;

pub const XXXL_PROGRAM_ID_PLACEHOLDER: &str = "XXXLProgram111111111111111111111111111111111";

pub const XXXL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const XXXL_RUNTIME_STATUS: &str = "SOURCE_BOUNDARY_READY_ACTIVATION_BLOCKED";
