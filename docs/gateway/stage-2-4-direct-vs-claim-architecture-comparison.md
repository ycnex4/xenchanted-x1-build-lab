# Stage 2.4 Direct Mint vs Claim-Based Architecture Comparison

## Purpose

Stage 2.4 compares the two Stage 2 gateway runtime candidates:

- Stage 2.2 direct mint candidate runtime design
- Stage 2.3 claim-based candidate runtime design

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or choose a final gateway architecture.

The purpose is to compare both candidates side by side and define an architecture choice gate.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.2 described direct mint.

Direct mint gives the cleanest user experience, but depends heavily on runtime atomicity, token CPI behavior, recipient token account handling, transaction size, and compute budget.

Stage 2.3 described claim-based flow.

Claim-based flow separates source event acceptance from token redemption, but introduces claim account storage, claim ownership rules, claim lifecycle rules, indexing requirements, and redemption complexity.

Stage 2.4 compares them as candidates, not as final decisions.

## Candidate summary

| Candidate | Basic idea |
| --- | --- |
| Direct mint | A verified gateway message directly mints the resulting X1-side token amount to the recipient token account in the same atomic transaction that marks the source event as processed. |
| Claim-based | A verified gateway message creates a claim account first. The recipient later redeems the claim through a separate instruction that mints the resulting X1-side token amount. |

Both candidates require deterministic message verification, guardian quorum validation, replay protection, route/coefficient binding, and source fork handling.

Neither candidate is safe until unresolved X1 runtime assumptions are confirmed.

## High-level comparison

| Area | Direct mint | Claim-based |
| --- | --- | --- |
| User experience | Best. One successful transaction produces the token result directly. | More complex. Claim creation and claim redemption are separate steps. |
| Runtime dependency | Higher. Requires strong atomicity across processed mark, mint, and accounting in one transaction. | Still high, but split across claim creation and redemption transactions. |
| State complexity | Lower. No claim account lifecycle. | Higher. Requires claim account lifecycle, storage, ownership, and redemption status. |
| Replay protection | ProcessedBurnEntry is the primary replay barrier. | ProcessedBurnEntry remains primary, plus ClaimAccount must be single-use. |
| Failure recovery | Harder if atomicity is not guaranteed. | Easier to expose pending state, but more states can become ambiguous if rules are weak. |
| Indexing | Simpler successful result indexing. | Requires claim discovery, pending claim indexing, and redeemed claim indexing. |
| Storage/rent pressure | Lower. | Higher. Claims consume storage and may remain unredeemed. |
| Recipient token account handling | Must be solved during the direct mint transaction. | Can be deferred to redemption, but still must be solved safely. |
| Testing burden | Focused but strict atomicity and rollback tests. | Broader tests because both claim creation and redemption must be proven. |
| UX risk | Lower if runtime assumptions are strong. | Higher because users may not understand pending claims. |
| Implementation risk | Concentrated in one instruction. | Distributed across more state and more lifecycle rules. |
| Migration risk | Lower state surface. | Higher because claim layout and lifecycle must remain stable. |

## User experience

Direct mint has the strongest user-facing model.

A user performs or is represented by a source-side event, the gateway verifies it, and the resulting X1-side token is minted directly to the recipient token account.

There is no pending claim state.

There is no second redemption action.

This is easiest to explain.

Claim-based flow is more complex.

The gateway accepts the source event and creates a claim.

The recipient or an authorized actor later redeems the claim.

This creates a visible pending state.

That pending state can be useful for indexing and recovery, but it adds UX burden.

If claim discovery is weak, users may not know they have a claim.

If redemption is delayed, users may think the bridge failed.

## Atomicity requirements

Direct mint requires a single transaction to guarantee that these effects commit together or fail together:

- processed burn mark
- token mint
- mint accounting update
- event/log emission if used

The unsafe states are:

- processed entry without minted tokens
- minted tokens without processed entry
- mint accounting mismatch
- partial state after failed verification
- partial state after failed CPI

Claim-based flow splits atomicity into two layers.

Claim creation must atomically commit:

- processed burn mark
- ClaimAccount creation
- claim data storage

Claim redemption must atomically commit:

- token mint
- mint accounting update
- claim redeemed status or claim closure

This reduces pressure on one transaction, but creates more total atomicity surfaces.

## Replay protection

Both candidates require canonicalEventKey replay protection.

Direct mint:

- canonicalEventKey must be processed at most once
- processed mark must be global across all routes
- mint cannot happen if canonicalEventKey is already processed

Claim-based:

- canonicalEventKey must create at most one claim
- one claim must be redeemed at most once
- processed entry must remain authoritative even if claim is closed
- claim closure must not allow source event replay

Claim-based flow does not remove the need for ProcessedBurnEntry.

ClaimAccount is not a replacement for processed burn registry.

ClaimAccount is the redeemable representation of an already accepted event.

## Storage and rent pressure

Direct mint has lower storage pressure.

It needs persistent replay protection and possibly mint accounting, but it does not need claim lifecycle storage.

Claim-based flow has higher storage pressure.

It must answer:

- who pays claim account creation cost
- who pays or receives rent if applicable
- whether unredeemed claims can remain forever
- whether redeemed claims are closed
- whether closing claims weakens auditability
- whether claim closure can ever affect replay protection

The conservative rule is:

    Closing a claim must never remove replay protection.

ProcessedBurnEntry must outlive ClaimAccount.

## Recipient token account handling

Direct mint must solve recipient token account handling inside the gateway mint transaction.

That may create pressure around:

- recipient account existence
- recipient account creation
- token account ownership
- token program constraints
- compute budget
- transaction size

Claim-based flow can defer recipient token account handling to redemption.

This may be safer if the recipient should prepare or provide the correct account.

However, it introduces a second transaction and requires clear redemption rules.

The design must define whether a relayer can redeem on behalf of the recipient or whether recipient signature is required.

## Failure recovery

Direct mint has fewer visible intermediate states.

That is good when the transaction succeeds.

It is dangerous if runtime atomicity is unclear.

If a failure can leave a processed mark without mint, recovery becomes difficult and may require forbidden manual intervention.

Claim-based flow has a visible pending state.

This can help recovery because accepted events become claims.

However, claim-based flow can create new failure states:

- claim exists but is undiscovered
- claim exists but recipient cannot redeem
- claim exists but token account rules changed
- claim remains forever and creates storage pressure
- claim redemption fails after claim creation succeeded
- claim layout becomes hard to migrate safely

## Indexing and discovery

Direct mint indexing is simpler.

A successful gateway event can show:

- source event
- canonicalEventKey
- recipient
- amount
- route
- coefficient version
- mint result

Claim-based indexing is more complex.

Indexers must track:

- claim created
- claim pending
- claim redeemed
- claim expired, if expiry exists
- claim closed, if closing exists
- failed redemption attempts, if exposed

Claim-based flow requires stronger UI/indexing support.

A hidden claim is poor UX.

## Guardian responsibilities

Guardian responsibilities should remain the same in both candidates.

Guardians confirm source evidence.

They do not choose:

- recipient
- amount
- route
- coefficient
- claim ownership
- mint authority
- replay status
- pause state
- final architecture

The protocol must derive deterministic results from verified messages and stored rules.

This is true for both direct mint and claim-based flow.

## Runtime assumption dependency

Direct mint depends most heavily on:

- transaction atomicity
- CPI atomicity
- account write rollback
- token mint authority model
- recipient token account handling
- compute budget
- transaction size limits

Claim-based flow depends most heavily on:

- transaction atomicity
- account write rollback
- claim account persistence
- deterministic account derivation
- claim ownership rules
- rent/storage model
- token mint authority model
- redemption atomicity
- claim lifecycle safety

Both depend on:

- processed burn registry persistence
- canonicalEventKey derivation immutability
- route/coefficient version binding
- guardian set version binding
- source chain finality
- source fork handling
- logs/events/indexing
- migration/upgrade rules

## Implementation blockers

Direct mint must remain blocked until these are confirmed:

- processed mark and mint can commit atomically
- token CPI behavior is safe
- failed CPI rolls back gateway state
- recipient token account handling is deterministic and safe
- compute and transaction size are acceptable
- mint authority model supports immutable/no-admin rules

Claim-based flow must remain blocked until these are confirmed:

- processed mark and claim creation can commit atomically
- claim redemption and mint can commit atomically
- claim account derivation is deterministic
- claim ownership is clear
- claim storage/rent model is acceptable
- claim closure cannot weaken replay protection
- claim migration risk is acceptable

If these blockers are unresolved, neither candidate should move to implementation.

## Testing burden

Direct mint test burden:

- valid direct mint
- duplicate canonicalEventKey rejection
- cross-route replay rejection
- wrong route/coefficient/guardian version rejection
- wrong recipient rejection
- wrong amount rejection
- failed mint rollback
- failed processed mark rollback
- mint accounting rollback
- paused gateway behavior
- compute and transaction size measurement

Claim-based test burden:

- valid claim creation
- valid claim redemption
- duplicate canonicalEventKey rejection
- duplicate claim rejection
- unauthorized redemption rejection
- already redeemed claim rejection
- wrong recipient token account rejection
- claim creation rollback
- redemption rollback
- paused claim creation behavior
- paused redemption behavior
- claim closure safety
- claim discovery/indexing behavior
- compute and transaction size measurement

Claim-based flow likely requires more tests overall.

Direct mint likely requires fewer tests, but stricter atomicity proof.

## Migration and upgrade risk

Direct mint has a smaller state surface.

That can reduce migration and upgrade risk.

Claim-based flow has a larger state surface because claim accounts can remain pending.

If runtime code changes while claims exist, the protocol must preserve:

- claim account layout compatibility
- claim redemption rules
- claim ownership rules
- amount interpretation
- route/coefficient version interpretation
- replay protection

Claim-based flow therefore has higher migration risk unless the claim model is very stable.

## Architecture choice gate

The architecture should not be chosen until the following gate is satisfied.

### Direct mint can be selected only if:

- transaction atomicity is confirmed
- CPI atomicity is confirmed
- account write rollback is confirmed
- token mint authority model is compatible with immutable/no-admin protocol rules
- recipient token account handling is deterministic and safe
- compute and transaction size are acceptable
- failure tests prove no partial state
- replay protection is global and durable
- source fork handling is defined

### Claim-based flow can be selected only if:

- claim creation atomicity is confirmed
- claim redemption atomicity is confirmed
- claim account persistence is confirmed
- claim ownership rules are final
- rent/storage model is acceptable
- claim closure cannot weaken replay protection
- pending claim discovery is solved
- failure tests prove no partial state
- replay protection is global and durable
- source fork handling is defined

### Neither can be selected if:

- X1 runtime atomicity is still unconfirmed
- token mint authority model is still unclear
- processed burn registry persistence is still unclear
- canonicalEventKey derivation is still unstable
- source fork handling is still undefined
- upgrade/migration behavior would allow rule reinterpretation

## Current recommendation

Direct mint should remain the preferred candidate if X1 runtime guarantees are strong enough.

It better matches the desired user experience and keeps state surface smaller.

Claim-based flow should remain the fallback candidate if direct mint atomicity, recipient account handling, or transaction size constraints cannot be proven.

Claim-based flow is not weaker by default, but it is more complex and requires stronger lifecycle discipline.

The next useful step is a gateway risk review that uses this comparison to identify the highest-risk unresolved assumptions before any runtime implementation begins.
