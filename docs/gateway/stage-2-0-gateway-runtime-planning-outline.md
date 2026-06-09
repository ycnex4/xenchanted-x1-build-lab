# Stage 2.0 Gateway Runtime Planning Outline

## Purpose

Stage 2.0 starts gateway runtime planning.

This is a planning-only checkpoint.

It does not implement runtime code, deploy an X1 program, create production mint authority, create production guardian operations, enable real cross-chain minting, or choose final direct mint / claim-based architecture.

The purpose is to turn the completed Stage 1 design checkpoints into a structured Stage 2 planning map.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 1 established a deterministic gateway model and expanded it through:

- Stage 1 gateway baseline
- Stage 1.5 runtime mapping
- Stage 1.6 guardian set management design
- Stage 1.7 X1 account/storage layout design
- Stage 1.8 X1 runtime assumptions checkpoint
- Stage 1.9 Stage 2 planning readiness checkpoint

Stage 1.9 concluded that the project is ready for Stage 2 planning, but not yet ready for Stage 2 implementation.

Stage 2.0 follows that boundary.

## Stage 2 scope

Stage 2 may include:

- runtime architecture planning
- account layout planning
- instruction planning
- direct mint candidate planning
- claim-based candidate planning
- token interface planning
- processed burn registry planning
- guardian verification planning
- source coefficient planning
- pause planning
- upgradeability planning
- runtime assumption dependency mapping
- test planning
- audit planning
- implementation gate definition

Stage 2 should produce enough clarity that a minimal implementation can later be built without silently relying on unknown runtime behavior.

## What Stage 2.0 is not

Stage 2.0 is not:

- runtime implementation
- production deployment
- token launch
- bridge launch
- relayer launch
- watcher launch
- guardian operations launch
- frontend launch
- final architecture approval
- final security approval
- final source coefficient selection
- final X1 tokenomics decision

## Inputs from Stage 1

### Stage 1 baseline inputs

Stage 1 provides:

- canonical message encoding
- domain separation
- route validation
- evidence validation
- recipient validation
- amount validation
- guardian signature verification
- guardian quorum verification
- processed burn replay protection
- mint authorization
- mint core state mutation
- positive end-to-end flow
- negative rejection matrix
- generated vectors

Stage 2 planning should preserve these deterministic properties.

### Stage 1.5 inputs

Stage 1.5 provides runtime mapping boundaries:

- verification before authorization
- authorization before mint mutation
- replay check before mint or claim creation
- processed burn mark atomic with result creation
- untrusted relayer boundary
- runtime verification context must be explicit
- split-phase execution is dangerous unless claim semantics are explicit

### Stage 1.6 inputs

Stage 1.6 provides guardian set design boundaries:

- guardian set identity
- guardian set versioning
- signed message binding to guardian set version
- guardian rotation concerns
- compromise recovery concerns
- emergency pause boundary
- guardian quorum does not own monetary rules

### Stage 1.7 inputs

Stage 1.7 provides account/storage layout responsibilities:

- gateway config state
- route config state
- source coefficient state
- guardian set references
- processed burn registry
- processed burn sharding
- mint state
- recipient token state
- pause state
- version/config state
- audit/indexing state

### Stage 1.8 inputs

Stage 1.8 provides runtime assumptions and blockers:

- transaction atomicity
- CPI atomicity
- account write rollback
- token program interface
- token mint authority
- recipient token account behavior
- account creation flow
- rent/storage model
- processed burn registry persistence
- deterministic account derivation
- program upgradeability
- pause authority
- guardian set account behavior
- route/coefficient version binding
- source chain identity
- source coefficient criteria
- finality assumptions
- compute budget / transaction size limits
- canonicalEventKey derivation immutability
- source chain fork handling

### Stage 1.9 inputs

Stage 1.9 provides the readiness boundary:

- Stage 2 planning is allowed
- Stage 2 implementation is not yet allowed
- design anchors are stable enough for planning
- unresolved runtime assumptions remain blockers before implementation
- direct mint vs claim-based must be decided using concrete X1 facts

## Planning tracks

Stage 2 planning should proceed through parallel candidate tracks before choosing implementation.

## Track A: Direct mint candidate

The direct mint candidate is the simplest user-facing model if X1 runtime assumptions are strong.

In direct mint:

- relayer submits verified gateway message
- runtime verifies message and guardian quorum
- runtime checks processed burn registry
- runtime marks canonicalEventKey as processed
- runtime mints token result directly to recipient token account
- mint result and processed mark commit atomically

Direct mint depends on strong assumptions:

- CPI mint is atomic with caller state
- token program interface is reliable
- recipient token account creation is clear
- account/rent payer model is clear
- compute budget fits verification plus mint
- transaction size fits required accounts and signatures

Direct mint is not safe if processed mark can commit without mint or mint can commit without processed mark.

## Track B: Claim-based candidate

The claim-based candidate is safer if X1 runtime assumptions are incomplete.

In claim-based flow:

- relayer submits verified gateway message
- runtime verifies message and guardian quorum
- runtime checks processed burn registry
- runtime marks canonicalEventKey as processed
- runtime creates a claim account for recipient and amount
- recipient later claims/mints through a separate instruction

Claim-based flow may reduce CPI risk but introduces claim-state complexity.

Claim-based flow depends on:

- permanent claim account storage
- deterministic claim account derivation
- claim ownership rules
- claim cancellation rules, if any
- claim expiry rules, if any
- claim replay protection
- claim-to-mint atomicity in the second step
- account/rent payer model

Claim-based flow is not automatically safer unless claim semantics are explicit and replay-safe.

## Runtime assumption dependency table

Stage 2 planning should create a dependency table with these columns:

- assumption
- current status
- affected design area
- direct mint impact
- claim-based impact
- blocker level
- evidence needed
- planned resolution

Initial assumptions to include:

1. Transaction atomicity.
2. CPI atomicity.
3. Account write rollback.
4. Token program interface.
5. Token mint authority model.
6. Recipient token account creation.
7. Rent/storage model.
8. Processed burn registry persistence.
9. Deterministic account derivation.
10. Program upgradeability.
11. Pause authority.
12. Guardian set account behavior.
13. Route/coefficient version binding.
14. Source chain identity.
15. Source chain finality.
16. Source chain fork handling.
17. canonicalEventKey derivation immutability.
18. Compute budget.
19. Transaction size limits.
20. Event/log/error support.

## Account planning areas

Stage 2 planning should define candidate accounts for:

- GatewayConfig
- RouteConfig
- SourceCoefficientConfig
- GuardianSet
- ProcessedBurnShard
- ProcessedBurnEntry
- MintState
- RecipientTokenAccount or RecipientBalance
- ClaimAccount, if claim-based flow is used
- PauseState
- VersionState
- Audit/Indexing state, if needed

Each account should define:

- purpose
- authority
- derivation
- fields
- mutability
- persistence
- size estimate
- rent/storage implication
- replay safety role
- migration behavior
- relationship to tests

## Instruction planning areas

Stage 2 planning should define candidate instructions for:

- initializeGateway
- initializeRoute
- updateRoute, if allowed
- updateCoefficient, if allowed
- initializeGuardianSet
- rotateGuardianSet
- pauseGateway
- unpauseGateway
- submitGatewayMint
- createGatewayClaim
- claimGatewayMint
- markProcessedBurn
- readGatewayState

Not all instructions should necessarily exist.

Instruction planning should decide which instructions are needed, which are unsafe, and which are only administrative or test-only.

## Replay protection planning

Replay protection remains the core security anchor.

Stage 2 planning must preserve:

- global canonicalEventKey registry across all routes
- no duplicate result for one canonicalEventKey
- no route-switch replay
- no coefficient-version replay
- no guardian-set-version replay
- no pause/unpause replay
- no upgrade replay
- no fork replay
- no migration replay

The processed burn registry must be treated as protocol-critical state.

## Guardian verification planning

Stage 2 planning must define:

- guardian public key format
- signature format
- guardian set account format
- threshold representation
- guardian set version binding
- expired set behavior
- rotation behavior
- compromised set behavior
- duplicate signer rejection
- invalid signer rejection
- guardian quorum failure behavior
- relationship between guardian quorum and pause authority

Guardian verification must not become monetary discretion.

Validators confirm events.

Protocol applies rules.

## Source coefficient planning

Stage 2 planning must define:

- coefficient storage
- coefficient versioning
- coefficient activation
- coefficient deactivation
- signed message binding
- route binding
- source chain binding
- finality binding
- fork handling
- coefficient change impact on old messages
- coefficient change impact on pending messages

A signed message must not be reinterpreted under a later coefficient.

## Pause planning

Stage 2 planning must define:

- pause authority
- pause account
- paused state effects
- unpause effects
- whether pause affects all routes or specific routes
- whether pause affects claim redemption
- whether pause affects only new submissions
- what remains readable during pause
- what state must never be modified by pause

Pause must not delete processed entries, modify balances, modify total minted, reinterpret messages, enable replay, or undo valid history.

## Upgradeability planning

Stage 2 planning must define:

- whether the runtime program is upgradeable
- who controls upgrade authority
- whether upgrade authority can be renounced
- how canonicalEventKey derivation is protected
- how processed burn registry compatibility is preserved
- how account layout changes are handled
- how route/coefficient versions survive upgrades
- how upgrades are disclosed
- how upgrades are tested

Upgradeability must not silently change replay semantics.

## Direct mint review gate

Before direct mint can be selected, the following must be answered:

- Is CPI mint atomic with processed burn mark?
- Can recipient token account creation be safely handled?
- Who pays storage/rent?
- Can verification fit compute budget?
- Can required accounts and signatures fit transaction size?
- Is mint authority safely bound to the gateway program?
- Can mint result and processed mark never diverge?
- Can failed mint never leave processed entry behind?
- Can failed processed mark never allow minted result?

If any answer is unknown, direct mint should not be selected as first implementation.

## Claim-based review gate

Before claim-based flow can be selected, the following must be answered:

- Can claim accounts be permanently and deterministically stored?
- Who pays claim account storage/rent?
- Can claim creation be atomic with processed mark?
- Can claim redemption be replay-safe?
- Can claim ownership be unambiguous?
- Can claims be indexed and audited?
- Can expired/cancelled claims exist, or are claims permanent?
- Can claim state create griefing or storage abuse?
- Does claim flow preserve user clarity?

If any answer is unknown, claim-based flow needs more planning before implementation.

## Test planning

Stage 2 planning should define tests before implementation.

Required test categories:

- canonical encoding parity tests
- vector compatibility tests
- guardian signature tests
- guardian quorum tests
- duplicate signer rejection tests
- invalid signer rejection tests
- route validation tests
- coefficient version tests
- processed burn replay tests
- cross-route replay tests
- source fork replay tests
- pause tests
- upgrade/migration compatibility tests
- direct mint atomicity tests, if direct mint is used
- claim creation atomicity tests, if claim-based flow is used
- claim redemption replay tests, if claim-based flow is used
- account derivation tests
- account size tests
- compute budget tests
- transaction size tests
- negative matrix tests

Tests must prove rejection paths, not only positive flows.

## Documentation planning

Before implementation, Stage 2 should produce:

- runtime assumption dependency table
- candidate account layout
- candidate instruction layout
- direct mint candidate design
- claim-based candidate design
- replay protection design
- guardian verification runtime design
- coefficient/version runtime design
- pause/upgradeability design
- test plan
- implementation gate checklist

## Review gates

Stage 2 planning should have at least three review gates.

### Gate 1: Planning completeness

Confirms that all runtime assumptions and candidate tracks are documented.

### Gate 2: Architecture choice

Confirms whether direct mint or claim-based flow is selected for first implementation.

### Gate 3: Implementation readiness

Confirms that no blocker assumption remains unresolved before runtime code begins.

Implementation should begin only after Gate 3.

## Current conclusion

Stage 2.0 begins gateway runtime planning.

It does not begin gateway runtime implementation.

The repository remains in design/planning mode for the X1-side gateway.

The next useful step is a runtime assumption dependency table that compares direct mint and claim-based flow against concrete X1 runtime facts.
