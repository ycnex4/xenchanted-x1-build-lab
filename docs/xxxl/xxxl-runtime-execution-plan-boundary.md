# XXXL Runtime Execution Plan Boundary

Status: COMPLETED.

This stage connects the runtime execution plan boundary after guarded account validation.

## Goal

Move the real SBF `process_instruction` path from:

    decode -> guarded account validation -> stop

to:

    decode -> guarded account validation -> build execution plan -> stop

This stage still does not enable live route execution.

## What changed

After successful `consume_gateway_mint` decode and guarded account validation, the runtime now builds an `AtomicConsumeGatewayMintExecutionPlan`.

The execution plan records:

- canonical event key
- route id
- recipient
- mint
- amount
- consumed slot
- source chain weight
- fixed atomic step order
- disabled live route flag
- disabled `mint_to` flag

## Runtime path

The runtime path now:

1. decodes `consume_gateway_mint`
2. reads Rent sysvar
3. reads Clock sysvar
4. validates all guarded accounts
5. prepares the CPI boundary
6. builds the execution plan
7. confirms live route execution is still disabled
8. returns success without mutation

## Safety boundary

This stage does not activate live route execution.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mark processed events as consumed.

This stage does not credit recipient balance.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

## SBF/Mollusk coverage

The ignored Mollusk harness still covers 9 cases:

- valid execution-plan path without state mutation
- invalid instruction length
- invalid discriminator
- invalid layout version
- wrong account count
- wrong program-owned account owner
- consumed processed event
- wrong recipient token owner
- zero amount

The valid SBF path emits:

    XXXL consume_gateway_mint execution plan built; live route execution is not activated

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
- 9 ignored Mollusk tests

Observed ignored Mollusk tests:

- 9 passed
- 0 failed

## Decision

The runtime execution plan boundary is accepted.

The program can now deterministically describe what would be executed after validation, while still refusing to perform live mint execution or state mutation.
