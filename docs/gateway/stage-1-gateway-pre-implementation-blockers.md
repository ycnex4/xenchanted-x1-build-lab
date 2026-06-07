# Stage 1 gateway pre-implementation blockers

This document defines the pre-implementation blockers for the Stage 1 XNTD-to-XXXL Gateway.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway design chain has reached the point where broad architecture is documented and externally reviewed.

Before implementation begins, the remaining open decisions must be converted into explicit blockers.

This document defines those blockers and the decision standard for each one.

The Stage 1 route remains:

Ethereum XNTD burn -> immutable X1 XXXL mint core -> XXXL mint

The core boundary remains:

- gateway guardians = verification layer
- immutable mint core / route rules = monetary conversion rules
- relayer = execution / transport layer without discretion

Guardians must not control XXXL monetary policy.

## Source documents

This document builds on:

- docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md
- docs/gateway/stage-1-xxxl-gateway-architecture.md
- docs/gateway/stage-1-xxxl-gateway-implementation-plan.md
- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/checkpoints/current-design-checkpoint.md

## Current implementation status

Implementation has not started.

The repository currently contains design, planning, schemas, review notes, and checkpoints.

There is no Stage 1 gateway runtime.

There is no Ethereum burn contract implementation for the gateway.

There is no X1 XXXL mint core implementation.

There is no guardian runtime.

There is no relayer runtime.

There is no watcher runtime.

There is no production deployment approval.

## Blocker 1: final hash function choice

Status: open.

The design currently uses placeholder notation such as:

- HASH(value)
- canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)
- messageHash = hash(domainSeparator, encodedGatewayMintMessage)

Before implementation, the final hash function must be selected.

Decision required:

- hash function for canonicalEventKey
- hash function for x1RecipientHash
- hash function for domain constants
- hash function for domainSeparator
- hash function for messageHash

Required decision output:

- exact hash algorithm name
- exact input byte encoding
- exact output byte length
- reason for compatibility with target X1 runtime
- reason for compatibility with guardian signing stack
- test vector examples

Open options:

- Ethereum-style keccak256
- X1-native hash function
- separate Ethereum-side and X1-side hash functions with explicit conversion rules

Preferred direction:

Use one hash model for all gateway message commitments if possible.

Avoid mixed hashing unless X1 runtime constraints require it.

Implementation must not begin until the final hash function choice is documented.

## Blocker 2: final signature standard

Status: open.

The design currently states that guardians sign deterministic messageHash.

Before implementation, the guardian signature standard must be selected.

Decision required:

- signature algorithm
- public key / signer identity format
- signature byte format
- guardian set representation
- threshold rule
- signature ordering / deduplication rules
- malleability rejection rules
- verification behavior on X1

Required decision output:

- exact signature standard
- exact messageHash signing boundary
- exact signer identity encoding
- exact guardian set id / version rule if used
- exact threshold rule
- test vector with deterministic test keys

Open options:

- secp256k1 style signatures
- Ed25519 style signatures
- X1-native signature verification standard
- external guardian verification runtime with X1-submitted approval proof

Implementation must not begin until the signature standard is documented.

## Blocker 3: final X1 recipient type and normalization

Status: open.

The current design uses:

- x1RecipientHash inside signed payload
- raw x1Recipient bytes in execution / evidence payload

This is accepted as a Stage 1 direction, but the final recipient type is still open.

Decision required:

- exact X1 recipient type
- exact recipient byte encoding
- exact normalization rule
- exact invalid recipient cases
- exact zero / burn recipient policy
- exact hash preimage for x1RecipientHash

Required decision output:

- recipient type name
- valid byte length or variable-length rule
- normalization before hashing
- normalization before minting
- rejection rules for malformed recipients
- rejection rules for zero / burn recipients, if adopted
- test vectors for valid and invalid recipients

Theo warning:

If X1 accepts multiple byte encodings for the same recipient, this can create recipient malleability.

Preferred direction:

The X1 verifier should accept exactly one canonical byte encoding for a recipient.

Implementation must not begin until recipient type and normalization are documented.

## Blocker 4: sourceBlockNumber and sourceBlockHash as mandatory signed fields

Status: decision accepted, document update required.

Theo recommended making sourceBlockNumber and sourceBlockHash mandatory signed fields.

Reason:

- guardians should sign only finalized canonical Ethereum evidence
- signed message should bind to the observed canonical block
- reorged-out evidence must not remain silently valid
- production design must make finality assumptions explicit

Decision:

Stage 1 signed message should treat sourceBlockNumber and sourceBlockHash as required fields.

Required updates before implementation:

- message schema must clearly mark sourceBlockNumber and sourceBlockHash as mandatory
- canonical encoding must include both in fixed field order
- test vectors must include both
- guardian acceptance rules must reject missing block number or block hash
- finality rule must reference these fields

Implementation must not begin until the mandatory field status is reflected across the design documents or superseded by a more precise pre-implementation spec.

## Blocker 5: X1 mint core immutability mechanism

Status: open.

The architecture depends on an immutable X1 mint core.

The design says guardians must not control XXXL monetary policy, but implementation must define how immutability is enforced.

Decision required:

- how X1 mint core is deployed
- whether code can be upgraded
- whether route rules can be changed
- whether mint authority can be changed
- whether deployer authority exists after deployment
- how deployer authority is removed or disabled
- whether any governance / timelock path exists
- which parameters are immutable forever
- which parameters, if any, are allowed to be operationally configurable

Required decision output:

- X1 deployment model
- immutability mechanism
- authority removal / disabling procedure
- list of immutable route rules
- list of non-monetary operational settings, if any
- security rationale

Stage 1 immutable route rules include:

- source chain is Ethereum mainnet
- source token is expected Ethereum XNTD token
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- replay key is canonicalEventKey

Preferred direction:

Route rules and monetary conversion logic must not be upgradeable by guardians, relayers, or an admin key.

Implementation must not begin until the immutability mechanism is documented.

## Blocker 6: atomic processed-burn check-and-mint

Status: open.

The X1 mint path must prevent duplicate minting for one canonicalEventKey.

Risk:

Two relayers may submit the same approved message.

Required behavior:

1. verify message and signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL

The check, mark, and mint path must be atomic or protected by X1 runtime guarantees.

Decision required:

- processed registry data model
- atomicity guarantee
- ordering of check / mark / mint
- behavior on mint failure
- behavior on duplicate submission
- behavior under concurrent relayer submissions
- emitted X1 event / record for processed burns

Required decision output:

- exact processed key storage model
- exact check-and-mark rule
- exact failure behavior
- duplicate rejection test vector
- explanation of X1 runtime atomicity assumptions

Implementation must not begin until the atomic processed-burn model is documented.

## Blocker 7: finality rule

Status: open.

Guardians must only sign finalized canonical Ethereum burn evidence.

Decision required:

- finality model
- minimum confirmation rule, if used
- whether finalized block tag is used
- whether multiple providers are required
- behavior during reorgs
- behavior if providers disagree
- handling of sourceBlockHash mismatch
- handling of source burn event disappearing after reorg

Required decision output:

- exact finality rule
- exact guardian acceptance rule
- exact rejected evidence cases
- finality status mapping for frontend
- test vectors or scenario notes

Potential directions:

- finalized block tag if supported
- conservative confirmation depth
- multiple-provider canonicality check
- guardian-local finality policy

Implementation must not begin until finality rule is documented.

## Blocker 8: zero / burn recipient policy

Status: open.

Risk:

A user may burn Ethereum XNTD and provide an unusable X1 recipient.

Possible result:

- Ethereum XNTD is burned
- XXXL is minted to an unusable recipient
- user permanently loses the X1-side mint result

Decision required:

- should zero recipients be rejected?
- should known burn recipients be rejected?
- should malformed recipients be rejected on Ethereum side, X1 side, or both?
- can Ethereum burn function validate X1 recipient format?
- should frontend warn before burn?

Required decision output:

- recipient rejection policy
- responsibility split between Ethereum burn path, guardian runtime, X1 verifier, and frontend
- valid / invalid recipient vectors

Preferred direction:

At minimum, Stage 1 should reject empty recipient bytes.

Zero / burn address policy should be decided before implementation.

## Blocker 9: burn amount min/max policy

Status: open.

The current design rejects zero burned amount.

Open question:

Should Stage 1 define minimum or maximum burned amount?

Decision required:

- zero amount rejection
- optional minimum burn amount
- optional maximum burn amount
- whether min/max belongs on Ethereum burn side, guardian side, X1 mint side, or frontend
- whether min/max is monetary policy and must be immutable

Required decision output:

- amount policy
- exact rejection cases
- route rule implications
- valid / invalid amount vectors

Preferred direction:

Reject zero amount.

Do not add arbitrary min/max unless there is a clear security, UX, or spam-control reason.

If min/max is added, decide whether it is immutable route policy.

## Blocker 10: exact cryptographic test vectors

Status: requirements documented, exact vectors not yet produced.

The existing test vectors document defines placeholder vectors.

Before implementation, exact vectors must be created after the following decisions are finalized:

- hash function
- signature standard
- X1 recipient type
- canonical binary encoding
- domain separator
- target mint core identity format

Required exact vectors:

- domain constants
- x1RecipientHash example
- canonicalEventKey example
- full message encoded bytes example
- messageHash example
- guardian signature example
- invalid field order example
- invalid optional omission example
- invalid amount string example
- invalid wrong route id example
- invalid wrong mint token example
- invalid duplicate canonicalEventKey example
- invalid cross-domain replay example

Implementation must not begin until at least one complete valid vector can be encoded and hashed identically by independent code.

## Ordered blocker resolution plan

Recommended order:

1. choose target X1 recipient type and normalization
2. choose hash function
3. choose signature standard
4. define domain separator and mint core identity format
5. update message schema to mark sourceBlockNumber and sourceBlockHash mandatory
6. define mint core immutability mechanism
7. define atomic processed-burn check-and-mint model
8. define finality rule
9. define zero / burn recipient policy
10. define burn amount min/max policy
11. produce exact cryptographic test vectors
12. review final pre-implementation package
13. begin implementation only after review

## Current implementation gate

Implementation is blocked until the following are complete:

- final hash function documented
- final signature standard documented
- final X1 recipient type documented
- sourceBlockNumber and sourceBlockHash mandatory status reflected
- X1 mint core immutability mechanism documented
- atomic processed-burn check-and-mint model documented
- finality rule documented
- zero / burn recipient policy decided
- burn amount min/max policy decided
- exact cryptographic test vectors produced
- independent implementation agreement on vectors confirmed

## Current conclusion

Stage 1 Gateway design is strong enough to move into pre-implementation decision resolution.

It is not ready for code yet.

The next recommended document is a decision document for hash function, signature standard, and X1 recipient type.
