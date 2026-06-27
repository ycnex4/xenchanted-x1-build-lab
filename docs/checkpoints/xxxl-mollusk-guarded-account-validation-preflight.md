# Checkpoint: XXXL Mollusk Guarded Account Validation Preflight

Stage: stage-xxxl-mollusk-guarded-account-validation-preflight

Status: COMPLETED

## Goal

Connect guarded account validation to `process_instruction` and verify it through real SBF/Mollusk execution.

## Completed

- `process_instruction` now reaches guarded account validation after successful instruction decode.
- Runtime still reads Rent via `Rent::get()`.
- Host unit tests use a rent-injected helper to avoid host sysvar dependence.
- Mollusk now verifies account-validation preflight through the real SBF artifact.
- Live route execution remains disabled.

## Mollusk coverage

The ignored Mollusk integration harness now covers 9 cases:

- valid preflight success with no state mutation
- invalid instruction length
- invalid discriminator
- invalid layout version
- wrong account count
- wrong program-owned account owner
- consumed processed event
- wrong recipient token owner
- zero amount

## Verified runtime boundary

The valid Mollusk case emits:

    XXXL consume_gateway_mint preflight validated; live route execution is not activated

This confirms that the SBF path reaches guarded account validation and then stops before live execution.

## Verified custom errors

Observed expected custom errors:

- `InvalidInstruction` -> `0x1`
- `InvalidAccountOwner` -> `0x2`
- `InvalidRecipientAta` -> `0x4`
- `InvalidDiscriminator` -> `0x6`
- `InvalidVersion` -> `0x7`

## Verification

Hard checks passed:

- cargo build-sbf
- cargo fmt --check
- cargo test
- cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

Observed default test result:

- 65 passed
- 0 failed
- 9 ignored Mollusk tests

Observed ignored Mollusk result:

- 9 passed
- 0 failed

## Safety boundary

This stage does not activate live route execution.

This stage does not invoke SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate processed event state.

This stage does not mutate recipient balance state.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

## Decision

The guarded account validation preflight is complete.

This is a suitable checkpoint for external review before the next stage moves toward atomic mutation or SPL CPI execution planning.
