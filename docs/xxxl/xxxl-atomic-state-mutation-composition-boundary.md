# XXXL Atomic State Mutation Composition Boundary

Status: COMPLETED.

This stage composes the processed-event mutation boundary and recipient-balance mutation boundary into one atomic state-mutation boundary.

## Goal

Move from two separately tested local mutation boundaries:

    execution_plan + processed_event -> mark event consumed

and:

    execution_plan + recipient_balance -> credit balance

to one composed boundary:

    execution_plan + processed_event + recipient_balance
    -> precheck both accounts
    -> mark processed_event consumed
    -> credit recipient_balance

This stage does not connect mutation to `process_instruction`.

## What changed

A new boundary function was added:

    apply_atomic_state_mutation_composition_boundary

It accepts:

- mutable processed event account data
- mutable recipient balance account data
- `AtomicConsumeGatewayMintExecutionPlan`

It validates before writing:

- fixed atomic step order
- live route flag is disabled
- mint_to flag is disabled
- amount is non-zero
- processed event is not consumed
- processed event canonical event key matches
- processed event route id matches
- processed event recipient matches
- recipient balance owner matches
- recipient balance mint matches
- recipient balance addition does not overflow

Only after all prechecks pass, it applies:

1. processed-event consumed mutation
2. recipient-balance credit mutation

## Atomicity property

The key property of this stage is precheck-before-write.

If recipient-balance validation fails, processed-event state remains unchanged.

If processed-event validation fails, recipient-balance state remains unchanged.

This prevents a partial local state result such as:

    processed_event consumed
    recipient_balance not credited

## Safety boundary

This stage does not activate live route execution.

This stage does not call SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not mutate SPL mint supply.

This stage does not mutate recipient token balance.

This stage does not connect atomic mutation to `process_instruction`.

## Tests added

The new tests cover:

- valid atomic composition marks event and credits balance
- recipient balance overflow rejected before event mark
- wrong recipient owner rejected before event mark
- wrong mint rejected before event mark
- replay rejected before balance credit
- wrong event key rejected before balance credit
- zero amount rejected without changes
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

The atomic state-mutation composition boundary is accepted.

The next safe stage can prepare SPL Token `mint_to` CPI planning, while still keeping live route execution disabled and not connecting mint execution to `process_instruction`.
