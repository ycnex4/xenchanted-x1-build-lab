# Stage 1.6 Guardian Set Management Design

## Purpose

Stage 1.6 documents the guardian set management design questions that should be resolved before future X1 runtime implementation.

This is a design-only milestone.

Stage 1.6 does not implement:

- production guardian operations
- production keys
- runtime governance
- deployed X1 programs
- deployed Ethereum contracts
- relayer runtime
- watcher runtime
- emergency pause implementation
- guardian rotation implementation
- Stage 2 runtime code

The goal is to define how guardian set identity, versioning, rotation, compromise recovery, and pending message validity should be framed before any X1 gateway runtime is written.

Stage 1 proved deterministic verification for a given guardian set.

Stage 1.5 mapped Stage 1 invariants to future runtime concerns.

Stage 1.6 focuses specifically on the guardian set design boundary.

## Baseline from Stage 1 and Stage 1.5

Stage 1 Gateway uses guardian public keys as deterministic test configuration.

Stage 1 proves that, for a given guardian set and threshold:

- guardian approvals can be verified deterministically
- duplicate guardian approvals are rejected
- unknown guardians are rejected
- invalid signatures are rejected
- quorum-not-reached is rejected
- valid quorum allows mint authorization to continue
- processed burn replay protection remains separate from signature verification

Stage 1.5 added the runtime mapping requirement:

- guardian set management must be explicit before production runtime
- guardian signatures should authorize canonical source-chain burn evidence
- guardians should not arbitrarily choose recipient, amount, route, or monetary rules outside the signed canonical message
- guardian rotation, compromise recovery, and emergency pause belong to the runtime / transport safety boundary, not mutable monetary policy

Stage 1.6 does not replace these conclusions. It expands the guardian-specific design questions.

## Guardian role boundary

Guardians are evidence attestators.

A guardian signs a canonical gateway mint message after verifying source-chain burn evidence according to the accepted gateway rules.

Guardians should not be treated as arbitrary mint authorities.

The runtime should preserve this boundary:

Guardian can help prove:

- source burn evidence was observed
- route is accepted
- recipient is encoded in the canonical message
- amount is encoded in the canonical message
- message hash is signed
- quorum is reached

Guardian should not be able to choose outside the signed message:

- arbitrary recipient
- arbitrary mint amount
- arbitrary source chain
- arbitrary source token
- arbitrary conversion rate
- arbitrary replay override
- arbitrary processed burn reset
- arbitrary monetary policy

Core framing:

    Guardian quorum authorizes evidence acceptance.
    Guardian quorum does not own the monetary rules.

## Guardian set identity

A future runtime should give every guardian set a stable identity.

Possible identity models:

- numeric guardianSetVersion
- hash of sorted guardian public keys and threshold
- runtime account address / PDA-like account
- config epoch
- explicit guardianSetId included in gateway config

A guardian set identity should make it possible to answer:

- which guardian set signed this message?
- which guardian set was active when the message was produced?
- which guardian set is accepted when the message is submitted?
- whether old guardian sets remain valid during a transition window
- whether a compromised guardian set can be revoked
- whether message validity depends on signing time or submission time

Preferred design direction:

    A signed message should bind to a guardian set identity or version.

Without guardian set identity, runtime implementation may have to infer which key set applies to a message, which can create ambiguity during rotation.

## Guardian set versioning

Guardian set versioning is important because guardian membership may change over time.

Open questions:

- does every rotation create a new guardianSetVersion?
- is guardianSetVersion monotonic?
- can guardianSetVersion be reused?
- can multiple guardianSetVersions be accepted at once?
- how long does an old guardianSetVersion remain valid?
- does the signed message include guardianSetVersion?
- does the processed burn registry include guardianSetVersion?
- does the domain separator include guardianSetVersion?
- does message hash include guardianSetVersion?

Safer default:

    guardianSetVersion should be part of the signed message domain or message body.

This makes signatures unambiguous across rotations.

However, adding guardianSetVersion to the signed message has consequences:

- generated vectors may need a future versioned message schema
- verifier helpers may need version-aware validation
- old messages may need a validity window
- guardian rotation policy must be defined before production

Stage 1.6 does not change Stage 1 vectors. It records that production runtime should avoid ambiguous guardian set selection.

## Signed message binding

A gateway mint message should be bound to all values that affect authorization.

Current Stage 1 signed message binds the core gateway mint data through canonical encoding and hashing.

Future guardian set design may require binding:

- guardianSetVersion
- route version
- gateway config version
- source finality policy version
- message expiry / TTL
- emergency pause epoch or runtime config version, if applicable

The design must avoid two dangerous patterns.

Dangerous pattern 1:

    A message signed under old assumptions is accepted under new assumptions.

Dangerous pattern 2:

    A message signed by one guardian set is interpreted as valid under another guardian set.

If guardianSetVersion is not included in the signed payload, runtime must provide another deterministic way to bind approvals to the intended guardian set.

## Rotation models

Guardian rotation can be designed in different ways.

### No rotation

The guardian set is fixed forever.

Advantages:

- simplest model
- closest to immutable configuration
- no governance / admin surface
- no ambiguity around old messages

Disadvantages:

- no recovery if guardians lose keys
- no recovery if a key is compromised
- no ability to improve guardian decentralization
- no operational flexibility

This may be too rigid for production bridge infrastructure.

### Scheduled rotation

Guardian set changes only at predefined intervals or epochs.

Advantages:

- predictable
- easier to document
- old messages can have a known transition window
- users can reason about rotation timing

Disadvantages:

- weak emergency recovery
- compromised keys may remain active until the next rotation
- requires runtime support for schedule validation

### Governance-controlled rotation

A governance or managed transport layer can rotate guardians.

Advantages:

- practical recovery path
- can respond to compromised keys
- can improve guardian set over time

Disadvantages:

- introduces managed authority
- must not become monetary control
- must be clearly separated from immutable mint rules

### Guardian-controlled rotation

Existing guardians approve the next guardian set.

Advantages:

- continuity from current trust set
- no separate admin key
- can use threshold-based transition

Disadvantages:

- compromised threshold could rotate to malicious set
- liveness issues if current guardians disappear
- requires careful version transition design

### Hybrid rotation

A combination of scheduled, guardian-approved, and emergency governance-controlled rotation.

Advantages:

- practical
- can separate normal rotation from emergency recovery
- can allow different thresholds for routine vs emergency actions

Disadvantages:

- more complex
- must be documented carefully
- increases runtime config surface

Stage 1.6 does not choose the final model. It records the tradeoffs and the need for explicit selection before Stage 2 runtime implementation.

## Compromise and recovery

Guardian key compromise is a production-critical scenario.

Runtime design must define what happens if one or more guardian keys are compromised while messages may still be pending.

Questions to resolve:

- what counts as a compromised guardian?
- who can declare a guardian compromised?
- can a compromised guardian be removed quickly?
- does removal take effect immediately or after a delay?
- what happens to pending messages already signed by that guardian?
- what happens to messages signed before compromise but submitted after compromise?
- does compromise invalidate the full guardian set or only one key?
- can a guardian set be revoked?
- can revocation happen without changing monetary rules?
- can users verify which guardian set was active for a message?
- how are stale approvals handled?

Important distinction:

    Compromise recovery is a transport and verification safety issue.
    It should not give any actor arbitrary mint, burn, balance, or monetary-policy control.

## Pending message validity

Pending messages are messages that have valid signatures but have not yet been submitted to X1 runtime.

Pending message rules must be explicit.

Possible validity models:

### Submission-time validity

A message is valid only if its guardian set is still active at submission time.

Advantages:

- easier compromise response
- old compromised sets can be disabled

Disadvantages:

- valid users may be blocked if message submission is delayed
- relayer delay becomes more dangerous
- old signatures may become unusable after rotation

### Signing-time validity

A message is valid if it was signed by a guardian set that was active at signing time.

Advantages:

- fairer to users
- relayer delay less harmful
- preserves validity of already-approved messages

Disadvantages:

- requires proving signing time or finality context
- compromised guardian detection becomes harder
- old signed messages may remain valid too long

### Bounded validity window

A message is valid only for a specific TTL or finality window.

Advantages:

- limits stale approvals
- supports rotation
- limits damage from old signatures

Disadvantages:

- introduces expiry edge cases
- users may need re-approval if relayer fails
- requires timestamp/block/finality design

Preferred direction for production design:

    Pending message validity should be bounded by either guardianSetVersion policy, TTL policy, or both.

Stage 1.6 does not choose the exact TTL. It records that pending message rules must be chosen before runtime.

## Emergency pause boundary

An emergency pause may be needed for severe transport-layer incidents.

Possible incidents:

- guardian key compromise
- guardian threshold compromise
- source-chain indexing bug
- message encoding bug
- replay-risk discovery
- verifier bug
- relayer exploit pattern
- route misconfiguration
- source finality issue

An emergency pause should be narrowly scoped.

It may pause:

- new message submission
- specific route acceptance
- specific guardianSetVersion acceptance
- specific source chain acceptance
- specific relayer path, if relayers are permissioned

It should not allow:

- arbitrary minting
- arbitrary burning
- arbitrary balance changes
- changing source-chain weight silently
- changing conversion rate silently
- deleting processed burn history
- replaying processed burns
- confiscating user balances

Core framing:

    Emergency pause may protect transport safety.
    Emergency pause must not become mutable monetary control.

Open design questions:

- who can pause?
- who can unpause?
- can pause be triggered by guardian threshold?
- can pause be triggered by governance?
- can pause be route-specific?
- can pause be guardian-set-specific?
- are already-processed burns always final?
- what happens to valid but unsubmitted messages during pause?
- does pause interact with TTL?

Stage 1.6 records the boundary but does not implement a pause.

## Threshold safety

Guardian threshold configuration must avoid invalid or unsafe settings.

Stage 1 already rejects:

- empty guardian set
- invalid threshold
- quorum not reached
- duplicate guardian approval
- unknown guardian approval
- invalid signature

Runtime design should preserve these checks and may need additional production constraints:

- minimum guardian count
- minimum threshold
- maximum threshold
- threshold cannot exceed guardian count
- threshold cannot be zero
- threshold cannot be one if production requires multi-party safety
- rotation cannot reduce threshold below safety minimum
- emergency action threshold may differ from message approval threshold

Open design question:

    Should normal mint approval and guardian set rotation use the same threshold?

A higher threshold for rotation may reduce malicious rotation risk.

A lower threshold for emergency pause may improve response speed.

These choices must be explicit because they change the trust and safety model.

## Runtime storage implications

Guardian set management affects runtime account/storage design.

Possible runtime state:

- current guardianSetVersion
- active guardian set account
- previous guardian set accounts
- accepted guardian set versions
- revoked guardian set versions
- guardian public keys
- threshold
- activation timestamp/block
- expiration timestamp/block
- emergency pause state
- route-specific guardian set binding
- pending transition state
- guardian set hash

Storage design must support answering:

- is this guardian set active?
- is this guardian set expired?
- is this guardian set revoked?
- is this message bound to this guardian set?
- is this route using this guardian set?
- does this guardian set version satisfy the required threshold?
- does emergency pause affect this message?

Storage design should avoid relying on off-chain assumptions that the runtime cannot verify.

## Interaction with processed burn registry

Guardian set design may affect the processed burn registry.

The canonicalEventKey should remain the replay-protection anchor for the source burn event.

Open questions:

- should processed burn entries store only canonicalEventKey?
- should they also store guardianSetVersion?
- should they store route version?
- should they store source chain and source tx reference?
- should they store mint amount for auditability?
- should guardianSetVersion affect replay identity or only verification identity?

Important principle:

    The same source burn event must not be mintable twice just because guardian set version changes.

Therefore, guardianSetVersion should not allow bypassing canonicalEventKey replay protection.

If guardianSetVersion is stored with processed burn entries, it should be for auditability or verification context, not for permitting duplicate mints.

## What remains out of scope

Stage 1.6 does not implement:

- guardian contracts
- guardian operations
- production key custody
- production guardian rotation
- production emergency pause
- runtime governance
- X1 account layout implementation
- X1 gateway program
- XXXL token runtime
- watcher runtime
- relayer runtime
- Ethereum contract changes
- frontend changes
- deployment scripts

Stage 1.6 also does not modify Stage 1 test vectors.

## Current conclusion

Stage 1.6 identifies guardian set management as a required design layer before future runtime implementation.

The most important design decisions are:

- guardian set identity
- guardian set versioning
- whether guardianSetVersion is signed
- pending message validity
- rotation model
- compromise recovery
- emergency pause boundary
- threshold safety
- runtime storage implications
- processed burn registry interaction

Stage 2 runtime implementation should not begin until guardian set management and X1 account/storage layout are both designed clearly enough to avoid hardcoding unstable assumptions.

