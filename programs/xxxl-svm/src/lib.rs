#![allow(unexpected_cfgs)] // Solana entrypoint! macro emits custom cfgs under host clippy/rustc check-cfg.

//! XXXL SVM runtime source boundary.
//!
//! This build has moved past the scaffold-only marker at source level.
//! It remains activation-blocked: no RPC mutation, no deploy/upgrade,
//! no route execution, and no SPL CPI execution are authorized here.
//!
//! X1 testnet Program ID binding is recorded at source level.
//! Activation remains blocked: this source package performs no RPC mutation,
//! no deploy/upgrade, no route enablement, and no SPL CPI enablement.

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
pub mod production_guardian_set_v1;
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
pub const XXXL_TESTNET_PROGRAM_ID: &str = "D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my";
pub const XXXL_BOUND_PROGRAM_ID: &str = XXXL_TESTNET_PROGRAM_ID;
pub const XXXL_TESTNET_PROGRAMDATA_ADDRESS: &str = "9tuesaPoJhrifF49vJewcg6PSWZeHAJiqQ97pq3LMW9T";
pub const XXXL_TESTNET_UPGRADE_AUTHORITY: &str = "DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc";
pub const XXXL_GATEWAY_MINT_AUTHORITY_PDA: &str = "BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG";
pub const XXXL_GATEWAY_MINT_AUTHORITY_BUMP: u8 = 252;

pub const XXXL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const XXXL_RUNTIME_STATUS: &str = "SOURCE_BOUNDARY_READY_ACTIVATION_BLOCKED";
