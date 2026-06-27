# XXXL Processed Event Mutation Boundary

Status: COMPLETED.

This stage adds the processed-event mutation boundary.

## Goal

Move from:

    execution plan -> stop

to a separately tested local mutation boundary:

    execution plan + processed_event account -> mark processed_event consumed

This stage does not connect mutation to `process_instruction`.

## What changed

A new boundary function was added:

    apply_processed_event_mutation_boundary

It accepts:

- mutable processed event account data
- `AtomicConsumeGatewayMintExecutionPlan`

It validates:

- fixed atomic step order
- live route flag is disabled
- mint_to flag is disabled
- amount is non-zero
- processed event is not already consumed
- canonical event key matches
- route id matches
- recipient matches

Then it marks the processed event as consumed and writes:

- consumed flag
- consumed amount
- consumed slot

## Safety boundary

This stage does not activate live route execution.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not credit recipient balance.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

This stage does not connect processed-event mutation to `process_instruction`.

## Tests added

The new tests cover:

- valid processed event mutation from execution plan
- replay rejected without changes
- wrong canonical event key rejected without changes
- wrong route rejected without changes
- wrong recipient rejected without changes
- zero amount plan rejected without changes
- live route flag rejected without changes
- mint_to flag rejected without changes
- reordered steps rejected without changes

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

The processed-event mutation boundary is accepted.

The next safe stage can focus on recipient-balance mutation boundary, still without SPL `mint_to` and without connecting mutation to live `process_instruction`.
