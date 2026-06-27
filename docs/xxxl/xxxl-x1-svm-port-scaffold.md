# XXXL X1/SVM Port Scaffold

## Purpose

This stage starts the real X1/SVM port scaffold for XXXL runtime.

It follows Theo approval of the final model/pre-port checkpoint.

This is the first Rust/SVM-facing structure, but it is still not deployable.

## Status

Status:

- `X1_SVM_PORT_SCAFFOLD_ONLY_NOT_DEPLOYABLE`

This stage creates native SVM scaffold structure under:

- `programs/xxxl-svm`

## Files

Scaffold files:

- `programs/xxxl-svm/Cargo.toml`
- `programs/xxxl-svm/src/lib.rs`
- `programs/xxxl-svm/src/entrypoint.rs`
- `programs/xxxl-svm/src/processor.rs`
- `programs/xxxl-svm/src/instruction.rs`
- `programs/xxxl-svm/src/state.rs`
- `programs/xxxl-svm/src/pda.rs`
- `programs/xxxl-svm/src/cpi.rs`
- `programs/xxxl-svm/src/validation.rs`
- `programs/xxxl-svm/src/error.rs`

## Boundaries

Preserved boundaries:

- no deployment
- no live transaction submission
- no route activation
- no Avalanche activation
- no guardian signature verification inside XXXL runtime
- no authority freeze execution

## Program ID boundary

The Program ID remains a placeholder boundary.

A real Program ID must be introduced in a later deploy-time decision stage.

## PDA fixture plan

Gateway mint authority PDA seeds remain fixed:

- `["xxxl", "gateway-mint-authority", "v1"]`

A later fixture must use real SVM `find_program_address`.

The model-only PDA must not be accepted as live PDA.

## Decode fixture plan

The scaffold prepares for:

- real account discriminators
- real instruction discriminator
- real account byte parsing
- real instruction byte parsing

## SPL Token CPI fixture plan

A later fixture must prove:

- real initialized Mint account
- real initialized recipient ATA
- matching mint
- PDA signer through `invoke_signed`
- SPL Token `mint_to` CPI

## Runtime checks

The real port must add:

- account owner checks
- rent exemption checks
- recipient ATA validation
- clock/slot source

## Non-goals

This stage does not deploy.

It does not submit transactions.

It does not activate routes.

It does not complete real PDA fixture.

It does not complete real SPL Token CPI.

It only creates the first native SVM scaffold boundary.
