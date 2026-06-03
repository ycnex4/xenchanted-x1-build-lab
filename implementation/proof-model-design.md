# Proof Model Design

## Branch

proof-model-design

## Purpose

This document defines the proof model direction for the post-MVP xEnchanted X1 Build Lab.

The current MVP is an in-memory state-transition model.

The proof model should define how external facts become validated payloads that can be safely passed into existing replay-protected instructions.

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Design boundary

The proof layer must not mutate BuildState directly.

The proof layer should validate external facts and produce canonical payloads for existing model transitions.

Recommended separation:

1. Source event exists outside the MVP model.
2. Proof layer validates the source event.
3. Proof layer derives a canonical event key.
4. Proof layer derives a canonical accounting payload.
5. MVP replay-protected transition applies the payload.

## Core principle

Proof validation and accounting mutation are separate concerns.

The proof layer answers:

- Is this external fact valid?
- Is it from the expected source?
- Is it final enough?
- What canonical key identifies it?
- What canonical payload should be applied?

The MVP transition layer answers:

- Was this key already used?
- Is the amount valid?
- Is the registrar message valid?
- Does this state transition preserve accounting invariants?

## Proof object categories

The model will likely need these proof categories:

- CoreRedeemProof
- XenBurnProof
- XntdLockProof
- XntdRelockProof
- X1FeeCheckpointProof
- GenesisOriginEligibilityProof

Each proof category should eventually have:

- source metadata
- source event identity
- canonical event key
- derived payload
- validation status
- validation timestamp
- optional proof references

## Canonical event key policy

Every externally sourced event must produce a deterministic key.

A canonical event key should include enough data to prevent collisions and replay across domains.

Recommended key components:

- source chain id
- source contract / program address
- source transaction hash or equivalent event id
- log index / instruction index / event index
- event kind
- relevant token id / Build id / user id when needed

Example shape:

sourceChainId:sourceContract:eventKind:txHash:logIndex

The exact encoding can be finalized later.

## Proof validation stages

A proof should pass through clear validation stages.

Recommended stages:

1. shape validation
2. source validation
3. event kind validation
4. finality validation
5. payload derivation
6. canonical key derivation
7. model-level replay protection

Only after these stages should the model transition be called.

## Core redeem proof

Purpose:

- validate that a Core redeem happened
- derive BLD amount
- derive redeem event key

Output should be compatible with:

- acceptCoreRedeemEvent
- applyCoreRedeemBld
- applyRegistrarCoreRedeem

Required future decisions:

- exact source chain
- source contract address
- event signature / ABI
- token id handling
- BLD derivation formula
- finality requirement
- canonical redeem key format

## XEN burn proof

Purpose:

- validate that a qualifying XEN burn happened
- derive XEN Burn Power amount
- derive XEN burn event key

Output should be compatible with:

- acceptXenBurnEvent
- applyXenBurnPower
- applyRegistrarXenBurn

Required future decisions:

- source XEN contract address
- burn event source
- accepted burn methods
- XBP derivation formula
- finality requirement
- canonical xenBurnKey format

## XNTD lock / relock proof

Purpose:

- validate that XNTD is locked or relocked according to current requirements
- derive lock amount
- derive lock epoch
- derive action timestamp

Output should be compatible with:

- lockXntd
- relockXntd
- applyRegistrarXntdLock
- applyRegistrarXntdRelock

Required future decisions:

- whether lock is native X1 escrow, Ethereum-side proof, or bridge-aware proof
- lock contract / program identity
- unlock conditions
- lock epoch source
- required amount formula
- proof key format
- relock event identity

## X1 fee checkpoint proof

Purpose:

- validate counted X1 fee contribution
- derive fee amount
- derive tx count
- derive countedUntilSlot

Output should be compatible with:

- applyX1FeeContributionCheckpoint
- applyRegistrarX1FeeCheckpoint

Required future decisions:

- fee source
- fee normalization
- counted tx definition
- slot finality policy
- checkpoint aggregation window
- who can submit checkpoints
- how duplicate windows are prevented

## Genesis Origin eligibility proof

Purpose:

- validate initial Genesis Origin eligibility if it ever comes from external records
- derive eligible historyBld or tier claim status

Current MVP:

- claimGenesisOriginBld derives tier from existing historyBld.
- no external proof is required in the current model.

Possible future proof use:

- prove pre-existing history
- prove snapshot membership
- prove one-time eligibility

Required future decisions:

- whether external Genesis proof is needed at all
- snapshot format
- claim key
- replay / duplicate claim policy outside BuildState.originBld

## Registrar relationship

The registrar layer may remain the bridge between proof validation and model transition.

Possible production flow:

1. watcher observes source event
2. proof layer validates source event
3. registrar signs or submits canonical message
4. model checks registrar authority and message replay
5. model applies transition

This keeps the MVP state-transition layer simple.

## Direct proof vs registrar proof

Two future patterns are possible.

### Direct proof pattern

The model receives proof objects directly and validates them.

Pros:

- less trusted registrar dependency
- more transparent verification

Cons:

- more complex model logic
- source-specific proof code enters model layer

### Registrar proof pattern

External proof validation happens outside the core model, and registrar messages submit canonical payloads.

Pros:

- keeps BuildState model clean
- easier MVP-to-production path
- source systems can evolve without rewriting accounting logic

Cons:

- registrar trust model must be explicit
- registrar signing / authority management becomes important

Current recommendation:

Use registrar proof pattern first.

Keep direct proof validation as a possible later hardening step.

## Finality policy

Each proof source must define finality before production use.

Examples:

- Ethereum event confirmations
- X1 block finality
- bridge watcher confirmation policy
- checkpoint aggregation delay

No model transition should rely on unfinalized source data in production.

## Proof replay policy

Proof replay protection must happen at the model level through canonical keys.

Required replay keys:

- redeemKey
- xenBurnKey
- future lock proof key if needed
- future fee checkpoint key or counted slot range

Registrar message replay is not enough by itself for source events that can be submitted under different message ids.

Current MVP already separates:

- registrar message replay
- Core redeem event replay
- XEN burn event replay

Future proof design should preserve this separation.

## Failure policy

Invalid proof must not mutate state.

Failure cases should include:

- malformed proof
- wrong source contract
- wrong event kind
- unfinalized event
- duplicate canonical key
- invalid derived amount
- invalid epoch
- invalid counted slot
- unauthorized submitter if applicable

## Testing policy

Each proof category should eventually include tests for:

- accepts valid proof
- rejects malformed proof
- rejects wrong source
- rejects wrong event kind
- rejects duplicate event key
- rejects unfinalized event
- derives expected payload
- does not mutate state on failure
- does not create unrelated accounting value

## Known non-goals for this milestone

This document does not implement:

- proof object TypeScript types
- proof validators
- ABI parsing
- event log parsing
- signature checks
- Merkle proofs
- bridge proofs
- storage
- API / CLI commands
- watcher code

## Main invariant

Proof validation should make external facts safe to use.

It must not weaken the MVP state-transition invariants.
