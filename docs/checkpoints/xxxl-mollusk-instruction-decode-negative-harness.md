# Checkpoint: XXXL Mollusk Instruction Decode Negative Harness

Stage: stage-xxxl-mollusk-instruction-decode-negative-harness

Status: COMPLETED

## Goal

Extend the real Mollusk SBF harness with negative instruction decode tests.

## Completed

- Added three ignored negative Mollusk tests:
  - invalid instruction length
  - invalid discriminator
  - invalid layout version
- Kept the valid scaffold-only Mollusk test.
- Confirmed all four ignored Mollusk tests pass.
- Added exact pinned direct dev-dependency:
  - `solana-program-error = "=3.0.1"`

## Verified errors

The SBF-level negative tests verify:

- `InvalidInstruction` returns custom error `0x1`
- `InvalidDiscriminator` returns custom error `0x6`
- `InvalidVersion` returns custom error `0x7`

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
- 4 ignored Mollusk tests

Observed ignored Mollusk result:

- 4 passed
- 0 failed

## Boundary

This stage only verifies instruction decoding and the existing scaffold-only path.

This stage does not activate live route execution.

This stage does not connect full account validation to `process_instruction`.

This stage does not invoke SPL Token mint_to.

This stage does not mint XXXL.

This stage does not mutate runtime state.

## Decision

The instruction decode negative Mollusk harness is complete.

The next stage can begin moving toward guarded account validation through Mollusk while preserving disabled live mint execution.
