#![allow(unexpected_cfgs)] // Solana entrypoint! macro emits custom cfgs under host clippy/rustc check-cfg.

//! XXXL SVM runtime scaffold.
//!
//! This is the first real Rust/SVM-facing scaffold after the model-layer
//! port readiness package.
//!
//! It is not deployable yet.
//! It intentionally keeps the Program ID as a placeholder boundary.
//! Real Program ID, real PDA fixture, real decode fixtures, and real SPL Token
//! CPI fixtures must be completed before deployment.

pub mod account_contract;
pub mod account_order_skeleton;
pub mod account_validation_skeleton;
pub mod cpi;
pub mod deployment_status;
pub mod dispatch_skeleton;
pub mod validated_dispatch_skeleton;
pub mod entrypoint;
pub mod error;
pub mod execution_plan;
pub mod instruction;
pub mod instruction_codec_skeleton;
pub mod instruction_payload_skeleton;
pub mod typed_instruction_skeleton;
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
pub mod state_instruction_skeleton;
pub mod state_initialization_skeleton;
pub mod consume_state_transition_skeleton;
pub mod state_account_layout_skeleton;
pub mod validation;
pub mod verifier;

pub const XXXL_PROGRAM_ID_PLACEHOLDER: &str = "XXXLProgram111111111111111111111111111111111";

pub const XXXL_TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const XXXL_RUNTIME_STATUS: &str = "SCAFFOLD_ONLY_NOT_DEPLOYABLE";
