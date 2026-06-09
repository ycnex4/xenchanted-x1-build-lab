# Stage 2.2 Direct Mint Candidate Runtime Design

## Purpose

Stage 2.2 defines the direct mint candidate runtime design for the future X1-side gateway.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or choose direct mint as the final architecture.

The purpose is to describe the direct mint path as one candidate architecture so it can later be compared against the claim-based candidate.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.1 made the runtime assumption dependency surface explicit.

Direct mint was identified as the cleanest user-facing candidate, but also the candidate that requires the strongest runtime guarantees.

Direct mint is only safe if the runtime can guarantee that the processed burn mark and mint result commit atomically or fail atomically.

If that cannot be proven, direct mint must not be selected for implementation.

## Direct mint definition

Direct mint means:

- a relayer submits a verified gateway mint message
- the X1-side gateway runtime verifies the message and guardian quorum
- the runtime checks that the canonicalEventKey was not processed before
- the runtime marks the canonicalEventKey as processed
- the runtime mints the resulting X1-side token amount directly to the recipient token account
- the processed mark and mint result are committed in the same atomic transaction

There is no intermediate claim account in this path.

The user receives the token result directly if the transaction succeeds.

## What direct mint is not

Direct mint is not:

- a final architecture choice
- a production bridge
- a relayer trust model
- a claim-based flow
- a manual admin mint
- a validator-controlled mint amount
- a discretionary mint mechanism
- a frontend implementation
- a tokenomics decision
- an X1 deployment plan

Direct mint remains a candidate until the required runtime guarantees are confirmed.

## Candidate participants

The candidate direct mint flow has these participants:

- source user
- source chain
- watcher
- guardian set
- relayer
- X1-side gateway runtime
- X1-side token program
- recipient token account

The relayer is untrusted.

Guardians confirm source evidence.

The protocol applies deterministic rules.

The runtime must derive recipient and amount from the signed message, not from relayer input.

## Candidate accounts

The direct mint candidate may require these accounts:

| Account | Purpose |
| --- | --- |
| GatewayConfig | Stores global gateway configuration and protocol constants. |
| RouteConfig | Stores source route identity, domain, enabled status, and route version. |
| SourceCoefficientConfig | Stores coefficient and coefficient version for a source route. |
| GuardianSet | Stores guardian public keys, threshold, and guardian set version. |
| ProcessedBurnShard | Stores or indexes processed event entries. |
| ProcessedBurnEntry | Represents a processed canonicalEventKey. |
| MintState | Tracks total minted or other global mint accounting. |
| TokenMint | X1-side token mint controlled or invoked by the gateway. |
| RecipientTokenAccount | Destination token account for the recipient. |
| PauseState | Stores emergency pause state. |
| VersionState | Stores active runtime/config version if needed. |

The exact account names and layouts are not final.

Stage 2.2 only identifies candidate responsibilities.

## Candidate instruction

The direct mint path may use a primary instruction such as:

    submitGatewayMint

Candidate instruction responsibilities:

1. Load gateway configuration.

2. Load route configuration.

3. Load source coefficient configuration.

4. Load guardian set.

5. Load pause state.

6. Verify that the route is enabled.

7. Verify that the gateway is not paused.

8. Reconstruct the canonical message.

9. Verify message domain separation.

10. Verify source chain identity.

11. Verify route version binding.

12. Verify coefficient version binding.

13. Verify recipient hash.

14. Verify amount derivation.

15. Verify guardian signatures.

16. Verify guardian quorum.

17. Derive canonicalEventKey.

18. Check processed burn registry.

19. Mark canonicalEventKey as processed.

20. Mint token amount to recipient token account.

21. Update mint accounting.

22. Emit/log successful gateway mint event if supported.

The order is important.

No mint should occur before authorization succeeds.

No processed mark should persist if mint fails.

## Required atomicity

Direct mint requires strict atomicity.

The following must be impossible:

- processed entry exists but recipient did not receive minted tokens
- recipient received minted tokens but processed entry does not exist
- mint accounting updated but token mint failed
- token mint succeeded but mint accounting failed
- failed verification leaves partial state
- failed guardian quorum leaves partial state
- failed token account creation leaves processed entry
- failed CPI leaves processed entry

If any of these states are possible, direct mint is unsafe.

## Replay protection

Replay protection is the core safety anchor.

Direct mint must preserve:

- one canonicalEventKey produces at most one X1-side result
- canonicalEventKey is global across all routes
- route switching cannot replay the same source event
- coefficient version changes cannot replay the same source event
- guardian set changes cannot replay the same source event
- pause/unpause cannot replay the same source event
- upgrades cannot replay the same source event
- source chain forks cannot allow duplicate results

ProcessedBurnEntry must be treated as protocol-critical state.

## Message binding

The signed gateway message must bind:

- source chain identity
- source route
- source event identity
- canonicalEventKey inputs
- recipient hash
- amount source
- route version
- coefficient version
- guardian set version
- domain separator
- finality context, if required

The runtime must not reinterpret an old signed message under a newer coefficient or route configuration.

## Guardian verification

Direct mint must verify guardian approval without giving guardians monetary discretion.

Guardians confirm that a source event is valid.

They do not choose:

- recipient
- mint amount
- coefficient
- route
- replay status
- protocol rules

The runtime must reject:

- insufficient quorum
- duplicate signers
- unknown guardians
- signatures from inactive guardian set
- signatures over wrong domain
- signatures over wrong route version
- signatures over wrong coefficient version
- malformed signatures
- mismatched message hash

## Route and coefficient binding

Direct mint must ensure that mint amount is computed using the coefficient version bound to the signed message.

A coefficient update must not change the meaning of already signed messages.

A route update must not allow a previously processed source event to be processed again.

Route and coefficient state should be explicit, versioned, and auditable.

## Pause behavior

Pause behavior for direct mint should be conservative.

When paused:

- new direct mint submissions should be rejected
- existing processed entries should remain unchanged
- balances should remain unchanged
- total minted should remain unchanged
- route configuration should not be reinterpreted
- old messages should not become valid again
- replay protection must remain active

Pause must not undo valid history.

Pause must not enable replay.

## Upgradeability behavior

If the gateway runtime is upgradeable, direct mint requires strict upgrade boundaries.

An upgrade must not silently change:

- canonicalEventKey derivation
- replay registry meaning
- route version meaning
- coefficient version meaning
- guardian set version meaning
- recipient derivation
- amount derivation
- mint authority rules

If canonicalEventKey derivation ever changes, the migration strategy must be explicit before implementation.

## Source fork handling

Direct mint must define how source chain forks are handled.

The same source burn event must not be able to mint twice because it appears under competing fork evidence.

The design must answer:

- what finality means for each source route
- whether canonicalEventKey includes fork-specific data
- whether guardians can sign evidence from a non-canonical fork
- how fork ambiguity affects route acceptance
- whether fork risk affects source coefficients

If fork handling is unresolved, direct mint implementation must remain blocked.

## Failure states to reject

Direct mint must reject or make impossible:

1. Duplicate canonicalEventKey.

2. Wrong source chain.

3. Wrong route.

4. Wrong route version.

5. Wrong coefficient version.

6. Wrong guardian set version.

7. Insufficient guardian quorum.

8. Duplicate guardian signatures.

9. Unknown guardian signature.

10. Wrong recipient hash.

11. Wrong amount.

12. Paused gateway.

13. Disabled route.

14. Unfinalized source event.

15. Source fork ambiguity.

16. Token mint failure.

17. Recipient token account failure.

18. Processed mark failure.

19. Mint accounting failure.

20. Transaction size overflow.

21. Compute budget overflow.

## Direct mint blockers

Direct mint remains blocked until there is concrete evidence for:

- transaction atomicity
- CPI atomicity
- account write rollback
- token program interface
- token mint authority model
- recipient token account handling
- rent/storage model
- processed burn registry persistence
- deterministic account derivation
- route/coefficient version binding
- canonicalEventKey derivation immutability
- source chain fork handling
- compute budget
- transaction size limits

If these are not resolved, direct mint may still be documented but must not be implemented.

## Test plan for direct mint

Before direct mint implementation, the test plan should include:

- valid direct mint flow
- duplicate canonicalEventKey rejection
- cross-route replay rejection
- wrong coefficient version rejection
- wrong route version rejection
- wrong guardian set version rejection
- insufficient quorum rejection
- duplicate guardian rejection
- unknown guardian rejection
- wrong recipient rejection
- wrong amount rejection
- paused gateway rejection
- disabled route rejection
- token mint failure rollback
- processed mark failure rollback
- mint accounting failure rollback
- source fork replay rejection
- compute budget measurement
- transaction size measurement
- vector compatibility tests
- negative matrix tests

Tests must prove failure paths, not only positive paths.

## Comparison notes

Direct mint advantages:

- simplest user experience
- no claim account
- no second transaction for recipient
- easier mental model
- direct protocol result

Direct mint disadvantages:

- requires stronger atomicity
- depends on token CPI behavior
- depends on recipient account handling
- may have higher compute/transaction-size pressure
- has less room to split complexity across steps

## Current conclusion

Direct mint is a strong candidate only if X1 runtime guarantees are strong.

It should remain a candidate, not an implementation decision.

The next useful step is to draft the claim-based candidate runtime design and compare it against this direct mint design.
