# Registrar Mutation Order and Assumptions Notes

## Branch

registrar-mutation-order-assumptions

## Purpose

This milestone addresses the registrar mutation ordering finding from external review.

The review noted that Core Redeem and XEN Burn registrar handlers marked the event key before marking the registrar message.

Previous successful mutation order:

- acceptCoreRedeemEvent / acceptXenBurnEvent
- acceptRegistrarMessage

New successful mutation order:

- acceptRegistrarMessage
- acceptCoreRedeemEvent / acceptXenBurnEvent

## Important precondition discovery

A direct reorder initially exposed an important invariant:

- invalid BLD amount must not mark registrar message
- invalid XBP amount must not mark registrar message
- invalid amount must not mark event key

Amount validation was previously happening inside:

- acceptCoreRedeemEvent -> applyCoreRedeemBld
- acceptXenBurnEvent -> applyXenBurnPower

After moving acceptRegistrarMessage earlier, invalid amount would have marked messageId before failing.

## Implemented fix

The registrar handlers now validate positive amounts before any mutation:

- applyRegistrarCoreRedeem checks amountBld > 0 before acceptRegistrarMessage
- applyRegistrarXenBurn checks amountXbp > 0 before acceptRegistrarMessage

The handlers preserve all existing preconditions:

- message kind check
- registrar authority check
- duplicate registrar message check
- duplicate event key check

Then the successful mutation path is:

- acceptRegistrarMessage
- acceptCoreRedeemEvent / acceptXenBurnEvent

## Safety result

This preserves the previous invalid-input invariant:

- invalid amount does not mark messageId
- invalid amount does not mark redeemKey / xenBurnKey
- invalid amount does not mutate Build state

And improves the successful path:

- registrar message is marked before event key
- if an event-level mutation unexpectedly fails after message acceptance, retry fails on duplicate registrar message first, which is a clearer operational failure mode than a used event key with an unprocessed message

## Validation result

After the code change:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

## Scope boundary

This milestone does not add new replay protection models.

It does not change XNTD lock / relock behavior.

It does not change fee checkpoint behavior.

It does not change snapshot behavior.

It only improves registrar preconditions and mutation order for Core Redeem and XEN Burn.
