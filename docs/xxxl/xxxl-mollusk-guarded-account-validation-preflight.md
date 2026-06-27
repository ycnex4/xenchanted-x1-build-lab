# XXXL Mollusk Guarded Account Validation Preflight

Status: COMPLETED.

This stage connects the guarded account validation preflight to the real `process_instruction` path.

The goal is to move beyond instruction decoding and scaffold-only execution while still keeping live mint execution disabled.

## Goal

Validate `consume_gateway_mint` accounts through the real SBF/Mollusk path before any live minting or state mutation is enabled.

This stage proves that the runtime can decode the instruction, validate the account boundary, and then stop before live route execution.

## What changed

`process_instruction` still decodes `consume_gateway_mint`, but the handler now performs guarded preflight validation before returning success.

The runtime flow is now:

1. decode instruction data
2. read Rent sysvar
3. prepare the guarded CPI boundary
4. validate account count and account indexes
5. validate program-owned account owners and rent exemption
6. validate mint state account
7. validate gateway config account
8. validate guardian set account
9. validate processed event account
10. validate recipient balance account
11. validate SPL mint account
12. validate recipient token account boundary
13. validate amount bounds
14. return success only after preflight passes
15. keep live route execution disabled

## Runtime safety boundary

This stage does not activate live route execution.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mark the processed event as consumed.

This stage does not mutate recipient balance state.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

## Host unit test adjustment

A helper path was added for host unit tests:

- `process_consume_gateway_mint_with_rent`

The real runtime path still uses:

- `Rent::get()`

The helper exists only so host tests can inject `Rent::default()` without depending on SVM sysvar availability outside Mollusk.

## Mollusk coverage

The ignored Mollusk SBF harness now covers 9 cases.

Valid case:

- valid `consume_gateway_mint` preflight succeeds without state mutation

Decode negative cases:

- invalid instruction length
- invalid discriminator
- invalid layout version

Account validation negative cases:

- wrong account count
- wrong program-owned account owner
- consumed processed event
- wrong recipient token owner
- zero amount

## Observed success log

The valid SBF/Mollusk case emits:

    XXXL consume_gateway_mint preflight validated; live route execution is not activated

This confirms that account validation is now reached through the real SBF path while live execution remains disabled.

## Observed custom errors

The ignored Mollusk tests observed expected custom errors, including:

- `0x1` for `InvalidInstruction`
- `0x2` for `InvalidAccountOwner`
- `0x4` for `InvalidRecipientAta`
- `0x6` for `InvalidDiscriminator`
- `0x7` for `InvalidVersion`

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
- 9 ignored Mollusk integration tests

Observed ignored Mollusk tests:

- 9 passed
- 0 failed

## Audit and deny status

cargo audit exits 0 with existing allowed warnings:

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

The guarded account validation preflight is accepted.

The runtime now has a real SBF-tested preflight boundary before any live mint execution.

The next stage can prepare the atomic mutation boundary or SPL Token CPI execution plan, while still preserving the live-route-disabled safety switch until the final activation stage.
