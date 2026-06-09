# Stage 1.9 Stage 2 Planning Readiness Checkpoint

## Purpose

Stage 1.9 documents readiness for future Stage 2 gateway planning.

This is a design-only checkpoint.

Stage 1.9 does not implement:

- X1 gateway runtime code
- deployed X1 programs
- deployed Ethereum contracts
- production keys
- production guardian operations
- relayer runtime
- watcher runtime
- frontend flow
- token deployment
- direct mint flow
- claim-based flow
- pause runtime
- account allocation scripts
- Stage 2 implementation

The goal is to summarize what has been completed across Stage 1.5 through Stage 1.8, identify what can be considered stable for planning, list what remains unresolved before implementation, and define the boundary between Stage 2 planning and Stage 2 implementation.

Stage 2 planning is not Stage 2 implementation.

Stage 2 planning should prepare an implementation plan based on explicit runtime assumptions.

Stage 2 implementation should not begin until the remaining runtime assumptions are either confirmed or marked as blockers.

## Background

The gateway model is being developed carefully because it touches protocol-critical concerns:

- source event verification
- guardian quorum
- message encoding
- replay protection
- source coefficients
- mint or claim result creation
- account-write atomicity
- route versioning
- coefficient versioning
- guardian set versioning
- pause boundaries
- program upgradeability
- source chain identity
- source chain fork handling
- X1 runtime guarantees

The goal is not to rush into runtime code.

The goal is to avoid implementing hidden assumptions that could later weaken replay protection, mint correctness, or first-principles protocol boundaries.

## Stage 1 gateway baseline status

Stage 1 established the deterministic gateway model in pure TypeScript.

It covered:

- canonical message encoding
- domain separation
- message hash calculation
- canonical event key calculation
- recipient hash calculation
- route validation
- evidence validation
- recipient validation
- amount validation
- guardian signature verification
- guardian quorum verification
- processed burn replay protection
- mint authorization
- mint core state mutation
- positive state-backed end-to-end flow
- negative state-backed rejection matrix
- generated vectors
- regression tests

Stage 1 proved the core deterministic logic without requiring runtime assumptions.

Stable planning conclusions:

- message encoding must remain canonical
- domain separation is required
- canonicalEventKey is the replay-protection anchor
- recipient and amount must be derived from signed message data
- guardian quorum authorizes evidence acceptance
- mint core should only mutate state after authorization succeeds
- duplicate canonicalEventKey must be rejected

Stage 1 does not prove:

- X1 account layout
- X1 transaction atomicity
- token program behavior
- CPI behavior
- rent/storage model
- upgrade authority behavior
- production guardian operations
- relayer implementation
- watcher implementation

## Stage 1.5 runtime mapping status

Stage 1.5 mapped the pure deterministic model to future runtime concerns.

It established that future runtime design must preserve:

- verification before authorization
- authorization before mint mutation
- replay check before mint or claim creation
- processed burn mark atomic with mint or claim result
- untrusted relayer boundary
- guardian quorum requirement
- recipient and amount derived from canonical signed message
- CPI / cross-program all-or-nothing behavior

Stable planning conclusions:

- relayer must remain untrusted
- relayer must not choose recipient or amount
- runtime must reconstruct verification context from explicit state
- processed mark and mint/claim result must commit together or fail together
- split-phase execution is dangerous unless claim semantics are explicit
- CPI behavior is a runtime assumption, not a pure-model fact

Stage 1.5 does not choose:

- direct mint vs claim-based flow
- final account layout
- exact token program
- exact X1 program structure

## Stage 1.6 guardian set management status

Stage 1.6 defined guardian set management design.

It established:

- guardian role boundary
- guardian set identity
- guardian set versioning
- signed message binding to guardian set identity/version
- guardian rotation concerns
- compromise and recovery concerns
- pending message concerns
- emergency pause boundary
- threshold safety concerns
- processed burn registry interaction

Stable planning conclusions:

- guardian set version must be explicit
- guardian set version helps verify signatures
- guardian set version must not permit duplicate minting
- guardian changes must not bypass canonicalEventKey replay protection
- guardian quorum does not own monetary rules
- guardian quorum does not choose coefficients
- guardian quorum does not arbitrarily mint
- emergency pause protects runtime safety but must not own monetary rules

Unresolved before implementation:

- exact guardian set account layout
- whether multiple guardian set versions can be accepted during rotation
- expiration rules for old guardian signatures
- revocation rules for compromised guardian sets
- operational key recovery model
- relationship between pause authority and guardian set authority

## Stage 1.7 account/storage layout status

Stage 1.7 defined future X1-side account and storage responsibilities.

It separated state into:

- gateway config state
- route / source-chain config state
- source coefficient state
- guardian set reference state
- processed burn registry
- processed burn shard / entry accounts
- mint state
- recipient balance or token account state
- pause / emergency state
- versioning / config epoch state
- indexing / audit state

Stage 1.7 review refinements strengthened:

- global cross-route replay principle
- coefficient version binding
- pause boundary
- invalid atomicity state for canonicalEventKey / recipient mismatch

Stable planning conclusions:

- processed burn registry must be global across all routes
- canonicalEventKey processed under any route must not be processable under any other route
- signed message must bind to route version and/or coefficient version
- mint core must use coefficient version from signed message
- coefficient changes apply only to messages signed after activation
- pause prevents new mints but does not modify historical state
- pause must not enable replay
- audit state explains what happened
- authorization state decides what may happen
- extra audit fields must not become alternative authorization rules

Unresolved before implementation:

- exact processed burn sharding strategy
- direct mint vs claim-based representation
- recipient balance/token account model
- account creation and rent responsibility
- CPI behavior
- exact pause account layout
- exact route/coefficient account layout
- exact migration model
- exact indexing/event model

## Stage 1.8 runtime assumptions status

Stage 1.8 defined runtime assumptions that must be clarified before Stage 2 implementation.

It covered:

- transaction atomicity
- CPI atomicity
- account write rollback
- token program interface
- token mint authority
- recipient token account behavior
- account creation flow
- rent / storage model
- processed burn registry persistence
- deterministic account derivation
- program upgradeability
- pause authority
- guardian set account behavior
- route / coefficient version binding
- source chain identity
- source coefficient criteria
- finality assumptions
- direct mint decision criteria
- claim-based decision criteria
- fallback decision rule
- minimum questions before Stage 2

Stage 1.8 review refinements added:

- compute budget / transaction size limits
- canonicalEventKey derivation immutability under upgradeability
- source chain fork handling

Stable planning conclusions:

- direct mint is viable only if runtime assumptions are strong
- claim-based flow is safer if CPI/account creation assumptions are unclear
- processed burn registry persistence is critical
- replay-protection state must not disappear
- upgradeability must not silently break replay protection or mint rules
- upgradeability must not silently change canonicalEventKey meaning
- source fork handling must not allow one burn event to produce multiple X1-side results
- compute budget and transaction size can become practical implementation constraints

Unresolved before implementation:

- actual X1 transaction atomicity guarantees
- actual CPI rollback behavior
- actual token program interface
- actual recipient account creation model
- actual rent/storage behavior
- actual upgrade authority policy
- actual pause authority model
- actual source finality policy
- actual compute budget and transaction size limits
- actual fork-disambiguation strategy

## Stable design anchors

The following design anchors are stable enough for Stage 2 planning:

### Canonical message and domain separation

Gateway messages must be canonical and domain-separated.

The runtime must not rely on ambiguous off-chain interpretation.

### canonicalEventKey as replay anchor

canonicalEventKey remains the replay-protection anchor.

It must be global across all routes.

It must remain effective across guardian rotations, route changes, coefficient changes, pause/unpause, upgrades, and migrations.

### Guardian quorum boundary

Guardian quorum authorizes evidence acceptance.

Guardian quorum does not own monetary rules.

Guardian quorum does not choose coefficients per message.

Guardian quorum must not bypass replay protection.

### Source coefficient boundary

Source coefficients must be explicit and versioned.

Mint amount must be derived from the route/coefficient version bound to the signed message.

Config changes must not reinterpret already signed messages under different coefficients.

### Pause boundary

Pause protects runtime safety.

Pause prevents new mints.

Pause does not undo past mints.

Pause does not modify processed burn registry entries, recipient balances, or totalMinted.

Pause does not enable replay.

### Upgradeability boundary

Upgradeability must not silently break replay protection or mint rules.

Upgradeability must not silently change canonicalEventKey meaning in a way that enables replay or makes processed entries unreachable.

### Atomicity boundary

Processed burn mark and mint/claim result must commit together or fail together.

Partial states must remain impossible.

### Direct mint vs claim-based boundary

Direct mint is not automatically chosen.

Claim-based flow is not automatically chosen.

The decision depends on concrete X1 runtime facts.

## Stage 2 planning boundary

Stage 2 planning may define:

- proposed runtime architecture
- candidate program/account layout
- instruction sequence
- account list per instruction
- direct mint candidate flow
- claim-based candidate flow
- runtime assumption dependency table
- implementation blockers
- test plan
- vector reuse plan
- negative test matrix
- migration assumptions
- audit checklist
- open questions for X1 runtime confirmation

Stage 2 planning must not yet:

- implement production gateway runtime
- deploy X1 programs
- deploy token mint
- introduce production keys
- choose production guardian operators
- enable real minting
- create live bridge-like claims
- create frontend production flow
- assume unconfirmed runtime guarantees
- hardcode unresolved account/rent/CPI assumptions

## Stage 2 implementation boundary

Stage 2 implementation should begin only after planning answers:

- direct mint or claim-based first implementation
- whether CPI is atomic with caller state
- whether token mint can be safely composed with processed mark
- whether recipient token account creation is reliable
- who pays account creation/rent
- whether processed entries can be permanent
- how upgrade authority is handled
- how pause authority is handled
- how canonicalEventKey derivation is frozen or migrated
- how source chain fork ambiguity is handled
- how compute budget and transaction size limits affect instruction design
- what events/logs/errors are needed
- what account constraints are enforced
- what tests prove replay protection
- what tests prove atomicity

## Blockers before Stage 2 implementation

These are blockers before implementation:

1. Unknown CPI atomicity.

2. Unknown transaction rollback behavior.

3. Unknown token program interface.

4. Unknown recipient token account creation model.

5. Unknown processed burn registry persistence.

6. Unknown upgrade authority model.

7. Unknown canonicalEventKey derivation immutability/migration model.

8. Unknown source chain fork handling.

9. Unknown compute budget / transaction size constraints.

10. Unknown account rent/storage model.

11. Unknown route/coefficient version binding in runtime message format.

12. Unknown finality policy per source route.

If these remain unknown, Stage 2 should stay in planning mode.

## Non-blocking open questions

These can remain open during early Stage 2 planning, but should be resolved before production:

- exact indexing dashboard format
- final event/log naming
- final error code naming
- final relayer UX
- final watcher deployment topology
- final guardian operational runbook
- final source coefficient numeric values
- final frontend display of gateway state
- final analytics/audit views

## Recommended next sequence

Recommended next sequence:

1. Stage 2 planning outline.

2. Runtime assumption dependency table.

3. Direct mint vs claim-based comparison against concrete X1 facts.

4. Candidate account layout for direct mint.

5. Candidate account layout for claim-based flow.

6. Instruction-level planning.

7. Test plan and negative matrix.

8. Review checkpoint before implementation.

9. Only then begin minimal runtime prototype if assumptions are clear.

## Current conclusion

Stage 1.9 marks the transition from Stage 1 design checkpoints toward Stage 2 planning readiness.

Stage 1.5 through Stage 1.8 created enough design structure to begin Stage 2 planning.

They did not create enough certainty to begin Stage 2 implementation.

The next appropriate step is Stage 2 planning, not runtime deployment.

The strongest current rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

