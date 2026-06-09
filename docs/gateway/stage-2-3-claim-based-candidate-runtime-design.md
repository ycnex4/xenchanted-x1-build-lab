# Stage 2.3 Claim-Based Candidate Runtime Design

## Purpose

Stage 2.3 defines the claim-based candidate runtime design for the future X1-side gateway.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or choose claim-based flow as the final architecture.

The purpose is to describe the claim-based path as the second candidate architecture so it can be compared against the direct mint candidate from Stage 2.2.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.2 described the direct mint candidate runtime design.

Direct mint is clean and simple for users, but requires strong runtime guarantees because the processed burn mark and mint result must commit atomically.

Stage 2.3 describes the claim-based alternative.

Claim-based flow can reduce some direct mint risks by splitting gateway acceptance and token minting into two steps.

However, claim-based flow is not automatically safer.

It introduces new state, storage, indexing, lifecycle, and redemption risks.

## Claim-based definition

Claim-based flow means:

- a relayer submits a verified gateway message
- the X1-side gateway runtime verifies the message and guardian quorum
- the runtime checks that the canonicalEventKey was not processed before
- the runtime marks the canonicalEventKey as processed
- the runtime creates a claim account containing recipient, amount, route/coefficient version, and source event identity
- the recipient later redeems the claim through a separate instruction
- the claim redemption mints the token result to the recipient token account
- the claim is marked redeemed or closed according to explicit rules

In this path, gateway acceptance and token minting are separated.

The first transaction creates a claim.

The second transaction redeems the claim.

## What claim-based flow is not

Claim-based flow is not:

- a final architecture choice
- a production bridge
- a relayer trust model
- a manual admin claim
- a validator-controlled mint amount
- a discretionary mint mechanism
- an off-chain promise
- a frontend implementation
- a tokenomics decision
- an X1 deployment plan

Claim-based flow remains a candidate until its storage, ownership, lifecycle, and redemption semantics are fully specified.

## Candidate participants

The candidate claim-based flow has these participants:

- source user
- source chain
- watcher
- guardian set
- relayer
- X1-side gateway runtime
- claim account
- recipient
- X1-side token program
- recipient token account

The relayer is untrusted.

Guardians confirm source evidence.

The protocol applies deterministic rules.

The runtime must derive recipient, amount, route, coefficient version, and claim identity from the signed message.

## Candidate accounts

The claim-based candidate may require these accounts:

| Account | Purpose |
| --- | --- |
| GatewayConfig | Stores global gateway configuration and protocol constants. |
| RouteConfig | Stores source route identity, domain, enabled status, and route version. |
| SourceCoefficientConfig | Stores coefficient and coefficient version for a source route. |
| GuardianSet | Stores guardian public keys, threshold, and guardian set version. |
| ProcessedBurnShard | Stores or indexes processed event entries. |
| ProcessedBurnEntry | Represents a processed canonicalEventKey. |
| ClaimAccount | Stores recipient, amount, source identity, versions, and redemption status. |
| MintState | Tracks total minted or other global mint accounting. |
| TokenMint | X1-side token mint controlled or invoked by the gateway. |
| RecipientTokenAccount | Destination token account for claim redemption. |
| PauseState | Stores emergency pause state. |
| VersionState | Stores active runtime/config version if needed. |

The exact account names and layouts are not final.

Stage 2.3 only identifies candidate responsibilities.

## Candidate instruction flow

The claim-based candidate may use two main instructions:

1. createGatewayClaim
2. redeemGatewayClaim

Additional helper or administrative instructions may exist later, but they are not part of this planning decision.

## Step 1: createGatewayClaim

Candidate responsibilities:

1. Load gateway configuration.

2. Load route configuration.

3. Load source coefficient configuration.

4. Load guardian set.

5. Load pause state.

6. Verify that the route is enabled.

7. Verify that new claim creation is not paused.

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

18. Derive ClaimAccount address.

19. Check processed burn registry.

20. Mark canonicalEventKey as processed.

21. Create ClaimAccount.

22. Store claim data.

23. Emit/log claim created event if supported.

The processed mark and claim account creation must commit atomically.

A processed entry without a claim account is unsafe.

A claim account without a processed entry is unsafe.

## Step 2: redeemGatewayClaim

Candidate responsibilities:

1. Load ClaimAccount.

2. Verify claim exists.

3. Verify claim is not already redeemed.

4. Verify claimant is the intended recipient or authorized recipient account.

5. Load token mint.

6. Load recipient token account.

7. Mint token amount to recipient token account.

8. Update mint accounting.

9. Mark claim as redeemed or close claim according to explicit rules.

10. Emit/log claim redeemed event if supported.

Claim redemption must be replay-safe.

The token mint and claim status update must commit atomically.

## Claim account fields

A ClaimAccount may need to store:

- canonicalEventKey
- source chain identity
- source route
- source event identity
- route version
- coefficient version
- guardian set version
- recipient
- recipient hash
- mint amount
- claim created timestamp or slot, if available
- redeemed status
- redeemed timestamp or slot, if available
- claim version
- optional metadata hash for audit

The claim account must not store ambiguous data that can be reinterpreted under later route or coefficient versions.

## Required atomicity

Claim-based flow requires atomicity in two places.

### Claim creation atomicity

The following must be impossible:

- processed entry exists but ClaimAccount does not exist
- ClaimAccount exists but processed entry does not exist
- failed verification leaves partial claim state
- failed guardian quorum leaves partial claim state
- failed claim creation leaves processed entry
- failed processed mark leaves claim account

### Claim redemption atomicity

The following must be impossible:

- claim is marked redeemed but token mint failed
- token mint succeeded but claim remains redeemable
- mint accounting updated but token mint failed
- token mint succeeded but mint accounting failed
- failed recipient token account handling marks claim redeemed
- failed CPI leaves claim in ambiguous state

If either atomicity layer cannot be proven, claim-based implementation must remain blocked.

## Replay protection

Replay protection remains the core safety anchor.

Claim-based flow must preserve:

- one canonicalEventKey creates at most one claim
- one claim can be redeemed at most once
- canonicalEventKey is global across all routes
- route switching cannot create multiple claims
- coefficient version changes cannot create multiple claims
- guardian set changes cannot create multiple claims
- pause/unpause cannot recreate claims
- upgrades cannot recreate claims
- source chain forks cannot create multiple claims

ProcessedBurnEntry remains protocol-critical state.

ClaimAccount does not replace ProcessedBurnEntry.

ClaimAccount is the redeemable representation of an already accepted event.

## Claim ownership

Claim ownership must be explicit.

The design must answer:

- who may redeem the claim
- whether recipient must sign redemption
- whether a relayer may redeem on behalf of the recipient
- whether a delegated recipient is allowed
- whether recipient token account must belong to recipient
- whether claims can be transferred
- whether claims can be cancelled
- whether claims can expire

The conservative default is:

- claims are non-transferable
- claims do not expire
- claims can only be redeemed by the intended recipient or a clearly authorized recipient account
- claims cannot be cancelled by an admin
- claims cannot be reinterpreted after creation

Any deviation must be explicitly justified.

## Rent and storage

Claim-based flow introduces additional storage requirements.

The design must answer:

- who pays for ClaimAccount creation
- whether the relayer pays
- whether the recipient reimburses or pays during redemption
- whether claims can remain unredeemed forever
- whether unredeemed claims create storage pressure
- whether redeemed claims are closed or retained for audit
- whether closing a claim weakens auditability
- whether closing a claim can affect replay protection

ProcessedBurnEntry must remain even if ClaimAccount is closed after redemption.

Closing a claim must not enable replay.

## Pause behavior

Pause behavior for claim-based flow must distinguish between claim creation and claim redemption.

Possible conservative policy:

- pause blocks new claim creation
- pause does not delete existing claims
- pause does not delete processed entries
- pause does not reinterpret route or coefficient versions
- pause may or may not block claim redemption, but the rule must be explicit

If pause blocks redemption, users with valid claims may be delayed.

If pause allows redemption, a compromised claim creation path must still be contained.

The pause policy must be selected before implementation.

## Upgradeability behavior

If the gateway runtime is upgradeable, claim-based flow requires strict upgrade boundaries.

An upgrade must not silently change:

- canonicalEventKey derivation
- claim account derivation
- claim ownership rules
- claim redemption rules
- route version meaning
- coefficient version meaning
- guardian set version meaning
- recipient derivation
- amount derivation
- mint authority rules

If claim account layout changes, migration strategy must be explicit before implementation.

## Source fork handling

Claim-based flow must define source chain fork handling.

The same source burn event must not be able to create claims on competing forks.

The design must answer:

- what finality means for each source route
- whether canonicalEventKey includes fork-specific data
- whether guardians can sign evidence from a non-canonical fork
- how fork ambiguity affects claim creation
- whether fork risk affects source coefficients

If fork handling is unresolved, claim-based implementation must remain blocked.

## Failure states to reject

Claim creation must reject or make impossible:

1. Duplicate canonicalEventKey.

2. Existing ClaimAccount for the same canonicalEventKey.

3. Wrong source chain.

4. Wrong route.

5. Wrong route version.

6. Wrong coefficient version.

7. Wrong guardian set version.

8. Insufficient guardian quorum.

9. Duplicate guardian signatures.

10. Unknown guardian signature.

11. Wrong recipient hash.

12. Wrong amount.

13. Paused claim creation.

14. Disabled route.

15. Unfinalized source event.

16. Source fork ambiguity.

17. Claim account creation failure.

18. Processed mark failure.

19. Transaction size overflow.

20. Compute budget overflow.

Claim redemption must reject or make impossible:

1. Missing claim.

2. Already redeemed claim.

3. Unauthorized redeemer.

4. Wrong recipient token account.

5. Token mint failure.

6. Mint accounting failure.

7. Claim status update failure.

8. Replay redemption.

## Claim-based blockers

Claim-based flow remains blocked until there is concrete evidence for:

- transaction atomicity
- account write rollback
- token program interface
- token mint authority model
- claim account persistence
- processed burn registry persistence
- deterministic account derivation
- claim ownership rules
- claim rent/storage model
- route/coefficient version binding
- canonicalEventKey derivation immutability
- source chain fork handling
- claim redemption atomicity
- compute budget
- transaction size limits

If these are not resolved, claim-based flow may still be documented but must not be implemented.

## Test plan for claim-based flow

Before claim-based implementation, the test plan should include:

- valid claim creation
- valid claim redemption
- duplicate canonicalEventKey rejection
- duplicate claim rejection
- cross-route replay rejection
- wrong coefficient version rejection
- wrong route version rejection
- wrong guardian set version rejection
- insufficient quorum rejection
- duplicate guardian rejection
- unknown guardian rejection
- wrong recipient rejection
- wrong amount rejection
- paused claim creation rejection
- paused redemption behavior, according to selected policy
- disabled route rejection
- claim creation rollback on failure
- processed mark rollback on claim creation failure
- redemption rollback on token mint failure
- already redeemed claim rejection
- unauthorized redeemer rejection
- wrong recipient token account rejection
- source fork replay rejection
- compute budget measurement
- transaction size measurement
- vector compatibility tests
- negative matrix tests

Tests must prove failure paths, not only positive paths.

## Comparison notes

Claim-based advantages:

- separates source event acceptance from token minting
- can reduce direct CPI pressure in the first step
- can defer recipient token account setup to recipient action
- may reduce transaction size pressure per step
- gives a clearer pending state for indexing

Claim-based disadvantages:

- adds ClaimAccount storage
- adds second user or relayer action
- adds claim discovery/indexing requirements
- adds claim ownership rules
- adds claim lifecycle questions
- adds more state that must remain migration-safe
- can create UX complexity if claims remain unredeemed

## Current conclusion

Claim-based flow is a strong candidate when X1 runtime assumptions are incomplete or direct mint atomicity cannot be proven.

It is not automatically safer.

It trades direct mint atomicity pressure for claim-state complexity.

It should remain a candidate, not an implementation decision.

The next useful step is to compare direct mint and claim-based candidate designs side by side and define an architecture choice gate.
