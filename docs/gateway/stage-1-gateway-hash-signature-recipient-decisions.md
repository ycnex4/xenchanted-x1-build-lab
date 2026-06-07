# Stage 1 gateway hash, signature, and recipient decisions

This document defines the Stage 1 Gateway decisions for:

1. hash function
2. guardian signature standard
3. X1 recipient type and normalization

This is a design decision document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway pre-implementation blocker list identified three first blockers:

1. final hash function choice
2. final signature standard
3. final X1 recipient type and normalization

This document resolves those three blockers for Stage 1 design.

Implementation should still not begin until the remaining blockers are resolved and exact cryptographic test vectors are produced.

## Source context

This document builds on:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/gateway/stage-1-gateway-pre-implementation-blockers.md

Theo clarified that X1 is SVM-compatible.

This matters because Stage 1 verification should respect both environments:

- Ethereum is the source chain and uses keccak256 naturally
- X1 is the execution / mint environment and uses SVM-native account and signature assumptions

The goal is not to emulate EVM on X1.

The goal is to use each environment where it is strongest.

## Final Stage 1 decisions

Stage 1 preferred decisions:

- Hash function: keccak256
- Guardian signature standard: Ed25519
- X1 recipient type: 32 raw bytes X1 / SVM public key
- x1RecipientHash: keccak256(x1RecipientBytes)
- display format: base58 is display-only and not canonical protocol encoding
- zero recipient: 32 zero bytes must be rejected

## Decision 1: hash function

Decision:

Stage 1 gateway commitments use keccak256.

This applies to:

- canonicalEventKey
- x1RecipientHash
- domain constants
- domainSeparator
- messageHash

Rationale:

- Stage 1 source chain is Ethereum
- Ethereum burn evidence and tooling naturally use keccak256
- canonicalEventKey is derived from Ethereum burn event identity
- viem / ethers / Ethereum indexers can generate vectors easily
- message payloads are small enough for practical SVM keccak verification
- using one hash model avoids mixed-hash complexity

Theo noted that X1 / SVM supports keccak256 through `sol_keccak256`.

Theo also noted that SHA-256 is cheaper on SVM, but using SHA-256 for X1-side commitments while Ethereum-side evidence is keccak-native would introduce a mixed hash model.

Stage 1 chooses consistency and audit clarity over the cheaper SHA-256 path.

## Hash function non-decision: SHA-256

SHA-256 is not selected for Stage 1 gateway commitments.

Reason:

- it is cheaper on SVM, but less natural for Ethereum-source event commitments
- it would create a mixed Ethereum/SVM hash model
- test vectors and tooling are simpler if one hash function is used
- Stage 1 payload sizes are small enough that keccak256 cost is acceptable

SHA-256 can be reconsidered only if future X1 runtime constraints make keccak256 impractical.

## canonicalEventKey decision

Stage 1 replay key remains:

canonicalEventKey = keccak256(ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex))

The replay anchor remains the exact Ethereum log.

sourceNonce is not part of the replay key.

sourceNonce remains useful for:

- indexing
- user display
- event ordering visibility

But sourceNonce does not bind the mint to a specific Ethereum transaction and log.

## x1RecipientHash decision

Stage 1 recipient hash is:

x1RecipientHash = keccak256(x1RecipientBytes)

Where:

- x1RecipientBytes is exactly 32 raw bytes
- x1RecipientBytes represents the X1 / SVM recipient public key
- x1RecipientBytes is not a string
- x1RecipientBytes is not base58 text
- x1RecipientBytes is not variable length

The signed gateway message includes x1RecipientHash.

The execution / evidence payload carries the raw 32-byte x1RecipientBytes.

The X1 verifier must check:

keccak256(x1RecipientBytes) == x1RecipientHash

## Decision 2: guardian signature standard

Decision:

Stage 1 guardians use Ed25519 signatures.

Rationale:

- X1 is SVM-compatible
- Ed25519 is native to SVM
- Ed25519 verification is cheaper and simpler than secp256k1 recovery on SVM
- guardians are infrastructure operators, not ordinary Ethereum users
- guardians can use fresh X1-native keys
- there is no hard requirement to reuse existing EVM guardian keys
- this avoids unnecessary EVM emulation in the X1 mint path

Guardians sign the deterministic Stage 1 messageHash.

The exact Ed25519 signature payload and verification format still require exact test vectors before implementation.

## Signature fallback: secp256k1

secp256k1 is not selected as the Stage 1 default.

It remains a documented fallback only if reusing existing EVM guardian keys becomes a hard requirement.

Rationale for not selecting secp256k1 as default:

- secp256k1 recovery on SVM is more expensive than Ed25519 verification
- it adds runtime complexity
- guardians do not need EVM-user compatibility
- Stage 1 can use fresh infrastructure keys

If secp256k1 is ever selected later, the decision must be documented explicitly and test vectors must be regenerated.

## Decision 3: X1 recipient type

Decision:

Stage 1 X1 recipient is a 32-byte raw X1 / SVM public key.

Canonical protocol encoding:

- exactly 32 raw bytes
- no base58 in signed payload
- no display string in signed payload
- no variable-length recipient bytes
- no checksum/casing rules
- no alternate encodings for the same recipient

Display encoding:

- base58 may be used in UI
- base58 may be used in logs or human-readable views
- base58 is not canonical protocol encoding
- base58 must be decoded to exactly 32 bytes before hashing or verification

## Recipient rejection rules

Stage 1 must reject:

- empty recipient
- recipient length not equal to 32 bytes
- malformed recipient bytes
- x1RecipientHash mismatch
- 32 zero bytes recipient

Policy still open:

- known burn recipient handling beyond 32 zero bytes
- whether any other reserved or invalid SVM public keys should be rejected

Minimum accepted rule for Stage 1:

x1RecipientBytes must be exactly 32 bytes and must not be all zero bytes.

## Recipient malleability prevention

Recipient malleability is addressed by using exactly 32 raw bytes as the canonical recipient encoding.

This avoids:

- casing ambiguity
- base58 display ambiguity
- variable-length byte ambiguity
- multiple encodings for the same recipient

The X1 mint core should not accept a recipient display string as canonical input.

The frontend may accept display strings from users, but must convert them to canonical 32 raw bytes before burn / message construction.

## Message schema implications

The Stage 1 message schema should be interpreted with these decisions:

- x1RecipientHash is mandatory
- sourceBlockNumber is mandatory
- sourceBlockHash is mandatory
- canonicalEventKey uses keccak256
- x1RecipientHash uses keccak256
- domainSeparator uses keccak256
- messageHash uses keccak256
- guardian signatures are Ed25519 signatures over messageHash
- x1RecipientBytes are supplied as execution / evidence payload

## Canonical encoding implications

The canonical encoding document should be interpreted with these decisions:

- hash placeholder means keccak256
- x1RecipientBytes are exactly 32 bytes
- x1RecipientHash is fixed-width 32 bytes
- messageHash is fixed-width 32 bytes
- domain constants are fixed-width keccak256 outputs
- optional field omission remains invalid
- display strings remain non-canonical

## Test vector implications

Exact test vectors must include:

- keccak256 domain constants
- keccak256 x1RecipientHash for a 32-byte dummy recipient
- keccak256 canonicalEventKey
- keccak256 domainSeparator
- full encoded message bytes
- keccak256 messageHash
- Ed25519 test guardian public key
- Ed25519 signature over messageHash
- valid verification case
- invalid wrong hash case
- invalid wrong signature case
- invalid recipient length case
- invalid all-zero recipient case
- invalid base58-as-canonical case

No implementation should begin until at least one complete valid vector can be reproduced independently.

## Remaining blockers after this decision

This document resolves:

- final hash function choice
- final signature standard
- final X1 recipient type and normalization

Remaining blockers:

1. sourceBlockNumber and sourceBlockHash mandatory field updates
2. X1 mint core immutability mechanism
3. atomic processed-burn check-and-mint model
4. finality rule
5. zero / burn recipient policy beyond 32 zero bytes
6. burn amount min/max policy
7. exact cryptographic test vectors

## Current conclusion

Stage 1 adopts:

keccak256 + Ed25519 + 32-byte X1 / SVM recipient

This is the preferred balance between:

- Ethereum-native source evidence
- X1/SVM-native execution verification
- simple test vector generation
- low recipient malleability
- no unnecessary EVM emulation on X1

Implementation should still not begin until the remaining blockers are resolved and exact test vectors are produced.
