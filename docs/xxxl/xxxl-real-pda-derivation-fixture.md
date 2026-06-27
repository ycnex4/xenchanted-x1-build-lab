# XXXL Real PDA Derivation Fixture

## Purpose

This stage adds the first real SVM PDA derivation fixture for XXXL runtime.

It follows the X1/SVM port scaffold.

The goal is to replace the model-only PDA boundary with a real Rust/SVM fixture using:

- `Pubkey::find_program_address`

## Status

Status:

- `REAL_PDA_DERIVATION_FIXTURE_BOUNDARY_ONLY`

This is still not deployment.

## Seeds

Gateway mint authority PDA seeds remain:

- `["xxxl", "gateway-mint-authority", "v1"]`

Canonical seed bytes:

- `xxxl` -> `7878786c`
- `gateway-mint-authority` -> `676174657761792d6d696e742d617574686f72697479`
- `v1` -> `7631`

## Rust fixture

Rust module:

- `programs/xxxl-svm/src/pda.rs`

Cargo test command:

- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml pda -- --nocapture`

The Rust fixture verifies:

- seed order
- seed bytes
- real `Pubkey::find_program_address`
- deterministic derivation for the same Program ID
- PDA changes when Program ID changes
- PDA is not equal to fixture Program ID

## Program ID boundary

The live Program ID is still not known.

Therefore:

- placeholder Program ID is not accepted as live
- model-only PDA is not accepted as live
- deploy-time Program ID remains required
- live PDA output depends on the final Program ID

## CPI dependency

This PDA is required for the future SPL Token `mint_to` CPI.

The later CPI fixture must use:

- real initialized Mint account
- real initialized recipient ATA
- PDA signer through `invoke_signed`
- real bump from `find_program_address`

## Non-goals

This stage does not deploy.

It does not submit transactions.

It does not perform SPL Token CPI.

It does not activate any route.

It does not finalize the live Program ID.
