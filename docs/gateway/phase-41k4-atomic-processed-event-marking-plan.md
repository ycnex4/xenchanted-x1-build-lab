# Phase 41K.4 — Atomic Processed-Event Marking Plan

## Status

Draft.

This document defines the design boundary for Phase 41K.4.

Phase 41K.4 must not implement SPL minting, live routing, or a full instruction handler.

Its purpose is narrower:

Atomic create, initialize, and consume of the processed-event PDA so replay protection can become real, not merely read-only.

## Background

Phase 41K.3 introduced a read-only processed-registry AccountInfo loader.

It can classify a processed-event PDA as:

- unprocessed: expected PDA exists as system-owned empty data, regardless of lamports;
- processed: XXXL-owned initialized account with consumed == true;
- rejected: malformed, wrong-owner, wrong-PDA, wrong-discriminator, wrong-version, identity-mismatch, signer, executable, or initialized-unconsumed cases.

Phase 41K.3 intentionally does not:

- create accounts;
- initialize accounts;
- mark events as consumed;
- mutate account data;
- invoke CPI;
- mint SPL tokens;
- expose a live route.

Therefore Phase 41K.4 is the next required boundary before 41K.3 can be wired into live replay protection.

## Core problem

The accepted 41K.3 invariant rejects initialized processed-event accounts with:

    consumed == false

This means Phase 41K.4 must never leave durable runtime state in this intermediate form.

The runtime must not produce this sequence as durable state:

    create/init processed-event account with consumed == false
    later mark consumed == true

Instead, the account must become initialized and consumed in the same successful execution path, or the whole path must fail without leaving an initialized-unconsumed processed-event account.

## Required successful outcome

For a successful burn-to-mint route, the processed-event account must end the transaction as:

    owner = XXXL program
    data_len = PROCESSED_EVENT_ACCOUNT_LEN
    discriminator = PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
    version = RUNTIME_LAYOUT_VERSION
    consumed = true
    canonical_event_key = authorized payload canonical_event_key
    route_id = authorized payload route_id
    recipient = authorized payload x1_recipient
    consumed_amount = authorized payload burned_amount or explicitly selected marked amount
    consumed_slot = current runtime slot

For every failure path, the runtime must not leave durable initialized account state with:

    owner = XXXL program
    data_len = PROCESSED_EVENT_ACCOUNT_LEN
    discriminator = PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
    consumed = false

## Non-goals

Phase 41K.4 does not implement:

- SPL Token mint_to;
- ATA validation;
- user-visible gateway instruction routing;
- live route enablement;
- relayer service;
- guardian proof collection;
- frontend gateway flow;
- production deployment;
- production guardian set selection.

Those belong to later phases.

## Safety flags

Phase 41K.4 may introduce a narrowly scoped account mutation plan for the processed-event PDA.

Phase 41K.4 must still keep these disabled:

- SPL mint;
- CPI minting;
- live route;
- production route;
- frontend route;
- production deployment.

## Processed-event PDA state model

Phase 41K.4 recognizes only these runtime states for the expected processed-event PDA.

### State A — Missing account

The expected AccountInfo is not supplied.

This is invalid for 41K.4 mutation.

41K.3 may classify a missing account as rejected.
41K.4 must also reject because it cannot safely create or mutate an account that was not supplied to the instruction.

### State B — System-owned empty-data PDA

The expected PDA is supplied.

Properties:

    key = expected processed-event PDA
    owner = system program
    data_len = 0
    lamports may be zero or nonzero

This is the only unprocessed state accepted by 41K.3.

41K.4 must be able to turn this state into a consumed processed-event account atomically.

Lamports must not determine replay classification.

If lamports are lower than rent-exempt minimum, the transaction must top up before allocation/assignment/initialization.

If lamports are already sufficient, the transaction may use existing lamports.

### State C — XXXL-owned consumed processed-event account

The expected PDA is supplied.

Properties:

    owner = XXXL program
    data_len = PROCESSED_EVENT_ACCOUNT_LEN
    discriminator = PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
    version = RUNTIME_LAYOUT_VERSION
    consumed = true

This is already processed.

41K.4 must reject it as replay and must not mutate it.

### State D — XXXL-owned initialized but unconsumed account

The expected PDA is supplied.

Properties:

    owner = XXXL program
    data_len = PROCESSED_EVENT_ACCOUNT_LEN
    discriminator = PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR
    consumed = false

This state is invalid by design.

41K.4 must never create this as durable state.

If encountered, it must reject and must not attempt to repair it silently.

### State E — Any malformed or foreign state

Examples:

- wrong PDA;
- signer account;
- executable account;
- system-owned nonzero data;
- non-system, non-XXXL owner;
- XXXL-owned wrong length;
- XXXL-owned zero discriminator;
- XXXL-owned wrong discriminator;
- unsupported version;
- canonical_event_key mismatch;
- route_id mismatch;
- recipient mismatch;
- invalid consumed flag.

41K.4 must reject and must not mutate.

## Allowed transition

Only one state transition is allowed:

    State B -> State C

The transition must happen in one successful execution path.

No other transition is allowed.

In particular:

    Missing -> CreatedButUnconsumed

is forbidden.

    SystemOwnedEmpty -> InitializedConsumed

is allowed only if the final account data is written as consumed == true before the instruction can succeed.

    InitializedUnconsumed -> Consumed

is forbidden as a repair path.


## Atomic mutation strategy

The preferred 41K.4 strategy is not naive create_account.

Naive create_account is insufficient because the expected PDA may already exist as a system-owned empty-data account with nonzero lamports.

That lamport-dusted state is accepted by 41K.3 as unprocessed and must remain safe.

The safe strategy is:

1. Require the expected processed-event PDA AccountInfo to be supplied.
2. Verify that the supplied account is exactly the canonical PDA.
3. Verify that the supplied account is not signer and not executable.
4. Classify it with the 41K.3 loader.
5. Continue only if the loader returns the unprocessed witness.
6. Compute rent-exempt minimum for PROCESSED_EVENT_ACCOUNT_LEN.
7. If the PDA has insufficient lamports, transfer the shortfall from the fee payer or designated rent payer.
8. Allocate the PDA to PROCESSED_EVENT_ACCOUNT_LEN.
9. Assign the PDA to the XXXL program.
10. Write the full processed-event account image with consumed == true.
11. Re-read or re-decode the account data and assert it now classifies as processed.
12. Only after this point may later phases consider SPL minting.

The entire sequence must occur in one instruction execution path.

The transaction must either commit the final consumed account or fail completely.

## Lamport-dusted PDA rule

A lamport-dusted PDA is:

    key = expected processed-event PDA
    owner = system program
    data_len = 0
    lamports > 0

This state must not cause a replay false-positive.

It must not cause account creation to fail permanently.

41K.4 must not assume that the PDA has zero lamports.

Therefore the design must use an allocation/assignment path that can work with an already-funded system-owned empty account.

The plan must explicitly avoid relying only on system create_account.

## Rent payer rule

41K.4 must identify a rent payer account.

The rent payer must be a signer.

The rent payer must be writable if lamports need to be transferred.

If the processed-event PDA already has enough lamports for rent exemption, no top-up is required.

If a top-up is required and the rent payer cannot fund it, the instruction must fail before leaving initialized-unconsumed state.

## Data write rule

The processed-event account data must be written as a final consumed image.

There must be no helper that first writes:

    consumed = false

and then later flips it to:

    consumed = true

The write path must construct the final byte image before committing it into account data.

The final byte image must include:

    discriminator
    version
    consumed = true
    reserved bytes
    canonical_event_key
    route_id
    recipient
    consumed_amount
    consumed_slot
    remaining reserved bytes

The existing mark_processed_event_consumed helper is not accepted as 41K.4 live semantics unless separately reviewed and proven compatible with this atomic model.


## Authorized payload binding

41K.4 must not accept free caller-supplied identity fields for marking.

The following values must come from the same internally decoded, quorum-authorized payload used by 41J:

- canonical_event_key;
- route_id;
- recipient;
- burned_amount;
- xxxl_mint_amount;
- target_mint;
- guardian_set_id.

The processed-event PDA expected key must be derived from the authorized canonical_event_key.

The loader expected_canonical_event_key must be the same authorized canonical_event_key that 41J uses for replay membership.

The loader expected_route_id must be the authorized route_id.

The loader expected_recipient must be the authorized x1_recipient.

This is a hard integration prerequisite.

If the loader is called with a key derived from any source other than the authorized payload, replay protection is invalid.

## Marked amount rule

41K.4 must define which authorized amount is written into consumed_amount.

Possible options:

- burned_amount;
- xxxl_mint_amount;
- a dedicated processed-mark amount derived from the payload.

The selected value must be deterministic and must be bound to the quorum-authorized payload.

The selected value must not be caller-supplied independently.

The selected value must not be inferred from account state.

The initial recommendation is:

    consumed_amount = xxxl_mint_amount

Reason:

The processed-event account is part of the X1 mint route.
It should record the amount this event authorized for the XXXL mint path.

However, this recommendation must be reviewed before implementation.

## Slot rule

consumed_slot must be the current runtime slot at the time of successful marking.

consumed_slot is metadata.

Replay protection must not depend on consumed_slot.

If current slot cannot be read safely in the selected runtime context, 41K.4 must explicitly decide whether to:

- require Clock sysvar;
- defer consumed_slot;
- store zero as a non-authoritative placeholder.


## Failure behavior

41K.4 must fail before mutation whenever possible.

Mandatory pre-mutation failures:

- missing processed-event AccountInfo;
- wrong PDA;
- signer processed-event account;
- executable processed-event account;
- malformed processed-event account;
- already processed event;
- initialized but unconsumed processed-event account;
- wrong owner;
- wrong discriminator;
- unsupported version;
- identity mismatch;
- quorum failure;
- payload decode failure;
- payload binding mismatch;
- insufficient rent payer funds.

If a failure occurs after lamport top-up but before final write, the Solana transaction rollback model should revert the full instruction transaction.

41K.4 design review must explicitly verify that no partial durable state remains after any failed path.

## Required tests

The 41K.4 implementation must include tests for:

1. system-owned empty PDA with zero lamports can become consumed processed-event account;
2. system-owned empty PDA with nonzero lamports can become consumed processed-event account;
3. system-owned empty PDA with insufficient lamports is topped up before allocation;
4. top-up failure does not leave initialized-unconsumed state;
5. wrong PDA rejects before mutation;
6. signer rejects before mutation;
7. executable rejects before mutation;
8. already consumed account rejects as replay and is not mutated;
9. initialized consumed == false account rejects and is not repaired;
10. system-owned nonzero-data account rejects and is not mutated;
11. wrong discriminator rejects and is not mutated;
12. unsupported version rejects and is not mutated;
13. canonical_event_key mismatch rejects and is not mutated;
14. route_id mismatch rejects and is not mutated;
15. recipient mismatch rejects and is not mutated;
16. invalid consumed flag rejects and is not mutated;
17. marked amount equals the selected authorized payload amount;
18. loader expected_canonical_event_key equals 41J payload-derived canonical_event_key;
19. final account re-decodes as processed using the 41K.3 loader;
20. no SPL mint occurs before successful processed-event marking.

## 41K.4 review gates

Before implementation, reviewers must answer:

1. Is the chosen create/allocate/assign/top-up path valid for a system-owned empty PDA with zero lamports?
2. Is it valid for a system-owned empty PDA with nonzero lamports?
3. Does it avoid naive create_account assumptions?
4. Can it ever leave durable consumed == false state?
5. Is the final byte image written as consumed == true from the start?
6. Is marked amount bound to the authorized payload?
7. Is the loader expected key identical to the 41J payload-derived canonical_event_key?
8. Are all safety flags still disabled except the narrow processed-event account mutation?
9. Is existing mark_processed_event_consumed unused or explicitly re-reviewed?
10. Is the design sufficient before 41K.3 to 41J live wiring?

## Current recommendation

Do not wire 41K.3 into live 41J eligibility before 41K.4 is accepted.

A read-only scaffold may be allowed only if it is explicitly marked:

- non-live;
- no-write;
- no-route;
- no-SPL-mint;
- not production replay protection.

The next step after this document is review, not implementation.
