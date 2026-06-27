# XXXL Mollusk Instruction Decode Negative Harness

Status: COMPLETED.

This stage extends the XXXL Mollusk SBF harness with negative instruction decode tests.

The goal is to verify the earliest runtime boundary currently reachable through `process_instruction`: instruction unpacking.

The live gateway route remains disabled.

## Goal

Add SBF-level Mollusk coverage for invalid `consume_gateway_mint` instruction data before the scaffold-only handler is reached.

This stage verifies that malformed instruction data is rejected by the runtime decoder with stable custom error codes.

## Why this stage exists

The current `process_instruction` flow is:

1. unpack instruction data
2. route valid `consume_gateway_mint` to the scaffold-only handler
3. return success only for the valid scaffold path
4. keep live route execution disabled

Because account validation and live mutation are not yet connected to `process_instruction`, the correct next Mollusk boundary is instruction decoding, not account mutation.

## What was added

Extended integration test:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

Added three ignored negative SBF tests:

- `invalid_consume_gateway_mint_length_rejects_before_scaffold_path`
- `invalid_consume_gateway_mint_discriminator_rejects_before_scaffold_path`
- `invalid_consume_gateway_mint_version_rejects_before_scaffold_path`

The existing positive scaffold-only test remains:

- `valid_consume_gateway_mint_scaffold_succeeds_without_state_mutation`

## New dev dependency

Added direct pinned dev-dependency:

- `solana-program-error = "=3.0.1"`

This is required because `mollusk-svm-result` expects the split-crate `solana_program_error::ProgramError` type for `Check::err`.

The version is pinned to the version already used by `mollusk-svm-result = "0.13.4"`.

## Verified decode failures

The negative tests verify:

- invalid instruction length returns `InvalidInstruction`
- invalid discriminator returns `InvalidDiscriminator`
- invalid layout version returns `InvalidVersion`

Observed runtime custom errors:

- `0x1` for `InvalidInstruction`
- `0x6` for `InvalidDiscriminator`
- `0x7` for `InvalidVersion`

## Verified valid scaffold behavior

The valid test still executes through Mollusk and emits:

    XXXL consume_gateway_mint scaffold reached; live route execution is not activated

It still verifies that state accounts remain unchanged.

## Harness type

The Mollusk integration tests remain ignored by default because they require a local SBF artifact:

    cargo build-sbf
    cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture

Default `cargo test` remains independent from local SBF artifacts.

## Runtime behavior unchanged

This stage does not activate live route execution.

This stage does not connect account validation to `process_instruction`.

This stage does not invoke SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate processed event state.

This stage does not mutate recipient balance state.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

## Verification

Hard checks passed:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

Observed default tests:

- 65 passed
- 0 failed
- 4 ignored Mollusk integration tests

Observed ignored Mollusk tests:

- 4 passed
- 0 failed

## Audit and deny status

cargo audit exits 0 with the existing allowed warnings:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

cargo deny remains green:

- licenses: pass
- bans: pass
- sources: pass

Duplicate crate warnings remain present in bans output and are accepted by current policy.

## Decision

The XXXL Mollusk instruction decode negative harness is accepted.

The next runtime stage can move toward connecting guarded account validation into the Mollusk execution path, while still keeping live mint execution disabled.
