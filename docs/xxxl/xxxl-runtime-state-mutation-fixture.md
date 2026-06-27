# XXXL Runtime State Mutation Fixture

Status: RUNTIME_STATE_MUTATION_FIXTURE_ONLY_NOT_LIVE_ROUTE.

This stage adds deterministic state mutation helpers for the XXXL runtime account layout.

## Purpose

Previous stages connected instruction decode, account views, validation, and CPI preparation. This stage adds the next state layer:

- mark a processed event as consumed
- write consumed amount
- write consumed slot
- credit recipient balance
- write last canonical event key

## ProcessedEvent mutation

`mark_processed_event_consumed` checks:

- account layout and discriminator
- event is not already consumed
- canonical event key matches
- route ID matches
- recipient matches
- consumed amount is non-zero

Then it writes:

- consumed flag
- consumed amount
- consumed slot

## RecipientBalance mutation

`credit_recipient_balance` checks:

- account layout and discriminator
- owner matches expected recipient
- mint matches expected XXXL mint
- amount is non-zero
- balance addition does not overflow

Then it writes:

- new balance
- last canonical event key

## Non-goals

This stage does not add:

- live route activation
- mint_to invocation from handler
- process_instruction state mutation
- deployment
- authority freeze execution

## Next likely stage

The next likely stage is an atomic execution-plan fixture that combines CPI preparation and state mutation while keeping route activation explicitly gated.
