# XXXL Runtime Local State Mutation Composition Boundary

Status: COMPLETED.

This stage composes runtime planning with local program-owned state mutation.

## Goal

Move from:

    runtime validation
    -> execution plan
    -> mint_to CPI plan
    -> stop

to:

    runtime validation
    -> execution plan
    -> mint_to CPI plan
    -> local state mutation
    -> stop

This stage mutates only local program-owned runtime accounts:

- processed event account
- recipient balance account

This stage does not call `invoke_signed`.

This stage does not invoke SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not connect live execution to `process_instruction`.

## What changed

A new composition type was added:

    RuntimeConsumeGatewayMintLocalStateMutationComposition

A new boundary function was added:

    build_runtime_consume_gateway_mint_local_state_mutation_composition_boundary

It accepts:

- program id
- accounts
- decoded `ConsumeGatewayMintArgs`
- rent
- consumed slot

It performs:

1. guarded account validation
2. CPI boundary preparation
3. atomic execution plan construction
4. SPL Token `mint_to` CPI planning
5. local state mutation composition

The local state mutation uses the already tested atomic mutation boundary:

    apply_atomic_state_mutation_composition_boundary

That boundary prechecks both local state accounts before writing, so recipient-balance failure does not mark the processed event.

## Safety boundary

This stage does not activate live route execution.

This stage does not call `invoke_signed`.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient SPL token balance.

This stage does not connect local state mutation to live `process_instruction`.

Only local program-owned state is mutated inside the explicit boundary function.

## Tests added

The new tests cover:

- valid local mutation marks processed event and credits recipient balance
- recipient balance overflow rejected before processed-event mark
- already consumed processed event rejected without recipient balance credit
- wrong recipient token account rejected before local mutation

The valid test also verifies that SPL mint supply remains unchanged.

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

The runtime local state mutation composition boundary is accepted.

The next safe stage can decide whether to add a guarded SPL CPI execution boundary as a separate explicit function, still not connected to live `process_instruction`.
