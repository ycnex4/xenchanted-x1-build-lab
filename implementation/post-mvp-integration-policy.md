# Post-MVP Integration Policy

## Branch

post-mvp-integration-policy

## Purpose

This document defines the policy for integrating post-MVP layers into the xEnchanted X1 Build Lab model.

The current MVP is an in-memory state-transition model.

Post-MVP work must preserve the core invariants already established:

- separate accounting layers
- explicit replay protection
- non-mutating failure paths
- registrar message atomicity
- no unrelated value creation

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Current MVP boundary

The MVP currently models trusted state transitions.

It does not yet decide how external facts become trusted inputs.

The current MVP assumes that registrar messages or direct instruction calls already contain validated facts.

Examples:

- Core redeem BLD amount is already known.
- XEN Burn Power amount is already known.
- XNTD lock amount and epoch are already known.
- X1 fee contribution checkpoint values are already known.

Post-MVP work must define how those facts are proven, indexed, serialized, stored, and exposed.

## Post-MVP integration order

Recommended order:

1. Proof model design
2. Storage / serialization model
3. API / CLI surface design
4. End-to-end scenario tests
5. External indexer / watcher integration
6. Signature / authority hardening
7. Production deployment policy

## Integration principle

New layers must wrap the MVP model.

They should not blur the accounting rules inside BuildState.

A post-MVP layer may:

- derive validated inputs
- verify proofs
- load persisted state
- save updated state
- expose API / CLI commands
- coordinate external watchers

A post-MVP layer must not:

- silently create accounting value
- bypass replay protection
- mutate BuildState before validation succeeds
- mix proof validation with unrelated accounting logic
- change MVP transition rules without explicit milestone and tests

## Proof layer policy

Proof validation should be modeled as a separate layer.

The proof layer should produce canonical validated payloads that can be passed into existing instructions.

Examples:

- validated Core redeem event
- validated XEN burn event
- validated XNTD lock event
- validated X1 fee checkpoint event

Proof validation must not directly mutate BuildState.

Recommended pattern:

1. parse source event
2. validate source chain / source contract / source event type
3. derive canonical event key
4. derive accounting payload
5. pass payload into existing replay-protected transition

## Storage layer policy

Storage should be separate from state transition logic.

Storage layer responsibilities:

- load BuildState
- load registries
- load replay-protection sets
- persist updated state after successful transition
- support snapshots / backups
- provide deterministic serialization

Storage layer must not:

- change accounting values
- skip transition validation
- rewrite history fields
- erase replay-protection state without explicit migration policy

## Serialization policy

Serialization should be deterministic and versioned.

Required future decisions:

- bigint encoding format
- Set encoding format
- null handling
- schema versioning
- migration policy
- canonical JSON ordering if needed
- snapshot integrity checks

Recommended rule:

All persisted state should include a schema version.

## API / CLI policy

API and CLI layers should call validated model instructions.

They should not contain independent accounting logic.

API / CLI responsibilities:

- input validation
- command routing
- user-friendly errors
- loading state
- invoking model transitions
- persisting results
- presenting output

API / CLI must not duplicate core accounting formulas.

## Registrar integration policy

Registrar integrations should continue to follow the current atomicity model:

1. validate message kind
2. validate registrar authority
3. validate messageId replay protection
4. validate domain-specific replay protection / transition rules
5. apply underlying transition
6. record registrar message only after success

Future registrar improvements may include:

- typed payload per message kind
- detached signatures
- message digest / hash canonicalization
- registrar rotation policy
- multi-registrar threshold policy
- source proof references

Any of these changes should be introduced as separate milestones.

## Event key policy

Event keys must become canonical before production integration.

Each source event type should have a deterministic key.

Possible key components:

- source chain id
- source contract address
- transaction hash
- log index
- event name / kind
- source token id or Build id when needed

Event key derivation must be documented and tested.

Replay protection should use canonical event keys only.

## Watcher / indexer policy

Watchers should observe external systems and create candidate events.

Watchers should not directly mutate BuildState.

Recommended watcher flow:

1. observe external event
2. normalize event
3. derive canonical key
4. validate confirmations / finality
5. submit proof or registrar message
6. model layer validates replay and applies transition

Watcher failures should be recoverable.

Duplicate watcher submissions should be safe because replay protection must reject duplicates.

## Migration policy

Any change to BuildState shape or persisted schema must be explicit.

Required for migrations:

- migration document
- before / after schema
- migration test
- rollback note if possible
- checkpoint update

No silent state shape changes after storage is introduced.

## Security policy

Post-MVP integration must preserve these security expectations:

- invalid inputs do not mutate state
- replayed inputs do not mutate state
- unrelated accounting layers do not change
- registrar authority is checked before acceptance
- source proof validity is checked before transition
- persisted replay state is never optional in production paths

## Testing policy

Each post-MVP layer should add tests before merge.

Minimum tests per new layer:

- accepts valid input
- rejects malformed input
- rejects duplicate input
- rejects unauthorized input if applicable
- does not mutate state on failure
- does not create unrelated accounting values
- preserves existing test suite

The baseline command remains:

- npm run typecheck
- npm test

## Explicit non-goals

Post-MVP integration policy does not yet define:

- exact proof formats
- exact storage adapter
- exact API framework
- exact CLI command names
- exact deployment process
- production registrar key management
- bridge production architecture

These should be handled in separate milestones.

## Main invariant

Post-MVP layers may make the MVP usable with real external data.

They must not weaken the MVP's state-transition invariants.
