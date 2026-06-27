# XXXL SPL mint_to CPI Planning Boundary

Status: COMPLETED.

This stage adds a planning boundary for SPL Token `mint_to` CPI.

## Goal

Move from:

    execution_plan + prepared CPI boundary -> stop

to:

    execution_plan + prepared CPI boundary -> mint_to CPI plan -> stop

This stage does not call `invoke_signed`.

This stage does not connect SPL minting to `process_instruction`.

## What changed

A new planning type was added:

    MintToCpiPlanningBoundary

A new planning function was added:

    plan_mint_to_cpi_boundary

It accepts:

- program id
- `AtomicConsumeGatewayMintExecutionPlan`
- `MintToCpiBoundary`

It validates:

- live route flag is disabled
- mint_to invocation flag is disabled
- execution plan amount is non-zero
- boundary amount is non-zero
- boundary amount matches execution plan amount
- token program is the SPL Token program
- boundary mint matches execution plan mint
- gateway mint authority PDA matches expected PDA
- gateway mint authority bump matches expected bump
- `build_mint_to_instruction` succeeds
- gateway mint authority signer seeds match the expected seed layout

Then it returns a pure planning result with:

- token program
- mint
- recipient token account
- mint authority PDA
- mint authority bump
- amount
- live route disabled flag
- invoke_signed disabled flag

## Safety boundary

This stage does not activate live route execution.

This stage does not call `invoke_signed`.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate processed-event state.

This stage does not mutate recipient-balance state.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

This stage does not connect CPI planning to `process_instruction`.

## Tests added

The new tests cover:

- valid mint_to CPI plan without invoke_signed
- amount mismatch rejected
- zero boundary amount rejected
- live route flag rejected
- mint_to flag rejected
- wrong token program rejected
- wrong mint mapping rejected
- wrong PDA rejected
- wrong bump rejected

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

## Decision

The SPL `mint_to` CPI planning boundary is accepted.

The next safe stage can compose:

    atomic state mutation boundary
    + mint_to CPI planning boundary

while still not invoking `invoke_signed` and still not connecting live execution to `process_instruction`.
