//! XXXL SVM runtime scaffold.
//!
//! This is the first real Rust/SVM-facing scaffold after the model-layer
//! port readiness package.
//!
//! It is not deployable yet.
//! It intentionally keeps the Program ID as a placeholder boundary.
//! Real Program ID, real PDA fixture, real decode fixtures, and real SPL Token
//! CPI fixtures must be completed before deployment.

pub mod cpi;
pub mod entrypoint;
pub mod error;
pub mod execution_plan;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;
pub mod validation;

pub const XXXL_PROGRAM_ID_PLACEHOLDER: &str =
    "XXXLProgram111111111111111111111111111111111";

pub const XXXL_TOKEN_PROGRAM_ID: &str =
    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub const XXXL_RUNTIME_STATUS: &str = "SCAFFOLD_ONLY_NOT_DEPLOYABLE";
