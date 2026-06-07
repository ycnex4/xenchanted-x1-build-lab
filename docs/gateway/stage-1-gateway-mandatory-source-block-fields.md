# Stage 1 gateway mandatory source block fields

This document defines the Stage 1 decision that `sourceBlockNumber` and `sourceBlockHash` are mandatory signed fields.

This is a design decision document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Theo's review recommended making source block data mandatory in the Stage 1 signed gateway message.

This document closes that pre-implementation blocker.

Decision:

`sourceBlockNumber` and `sourceBlockHash` are mandatory signed fields for Stage 1.

They must be included in:

- gateway message schema
- canonical field order
- canonical encoded message
- messageHash preimage
- guardian signed payload
- test vectors
- guardian acceptance rules
- finality rule design

## Source context

This document builds on:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/gateway/stage-1-gateway-pre-implementation-blockers.md
- docs/gateway/stage-1-gateway-hash-signature-recipient-decisions.md

## Why the fields are mandatory

Guardians must sign only finalized canonical Ethereum burn evidence.

The signed message should bind to the observed canonical Ethereum block.

If source block data is absent, a signed mint approval is less tightly bound to the finality evidence that guardians verified.

Mandatory source block fields help prevent ambiguity around:

- reorged-out events
- non-finalized evidence
- provider disagreement
- stale event observations
- evidence replay with incomplete source context

## Mandatory fields

The Stage 1 signed message must include:

- sourceBlockNumber
- sourceBlockHash

Field meaning:

- `sourceBlockNumber` is the Ethereum block number containing the accepted burn event
- `sourceBlockHash` is the Ethereum block hash containing the accepted burn event

Both values must come from the same canonical finalized Ethereum block that contains the accepted burn event.

## Relationship to replay protection

Replay protection remains based on:

canonicalEventKey = keccak256(ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex))

The replay key does not change.

`sourceBlockNumber` and `sourceBlockHash` are not the primary replay key.

Their role is to bind the signed message to the finalized source block context that guardians accepted.

## Guardian acceptance rules

Guardians must reject evidence if:

- sourceBlockNumber is missing
- sourceBlockHash is missing
- sourceBlockHash is not a 32-byte Ethereum block hash
- sourceBlockNumber does not match the block containing the burn event
- sourceBlockHash does not match the block containing the burn event
- the block is not canonical according to the chosen finality rule
- the block is not finalized enough according to the chosen finality rule
- the burn event is not present in that block
- the transaction receipt block hash differs from sourceBlockHash
- providers disagree and the finality policy cannot resolve the disagreement

Guardians must not sign a message built from incomplete source block evidence.

## Message schema implications

The Stage 1 gateway message schema should treat the following fields as required:

- sourceBlockNumber
- sourceBlockHash

They are not optional.

They are not display-only metadata.

They are part of the signed message.

The signed message must bind:

- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash

## Canonical encoding implications

The canonical field order remains:

1. messageType
2. schemaVersion
3. routeId
4. sourceChainId
5. sourceToken
6. sourceSender
7. sourceBurnTxHash
8. sourceBurnEventIndex
9. sourceBlockNumber
10. sourceBlockHash
11. sourceNonce
12. canonicalEventKey
13. x1RecipientHash
14. burnedAmount
15. sourceChainWeightBps
16. xxxlMintAmount
17. mintToken
18. deadlineOrFinalityBlock
19. messageNonce

Because sourceBlockNumber and sourceBlockHash are already in the field order, no field order change is needed.

The decision is that fields 9 and 10 are mandatory and cannot be omitted or zero-filled as unused optional fields.

## Invalid encoding cases

The following must be invalid:

- sourceBlockNumber omitted
- sourceBlockHash omitted
- sourceBlockNumber encoded as a decimal string
- sourceBlockHash encoded as a hex display string instead of bytes
- sourceBlockHash with wrong length
- sourceBlockHash not matching the transaction receipt block hash
- sourceBlockNumber not matching the transaction receipt block number
- sourceBlockHash from a reorged-out block
- sourceBlockHash from a non-canonical block
- sourceBlockNumber and sourceBlockHash from different blocks

## Test vector implications

Exact test vectors must include:

- sourceBlockNumber
- sourceBlockHash
- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey
- full encoded message bytes
- messageHash

Invalid vectors must include:

- missing sourceBlockNumber
- missing sourceBlockHash
- wrong sourceBlockHash
- wrong sourceBlockNumber
- sourceBlockHash wrong length
- sourceBlockHash from different block
- reorged-out sourceBlockHash scenario note

## Finality rule dependency

This document does not define the finality rule.

It prepares for the finality rule by making the signed message bind to block identity.

The future finality rule document must define:

- what "finalized" means for Ethereum Stage 1
- whether finalized block tag is used
- whether confirmation depth is used
- whether multiple providers are required
- how guardians handle provider disagreement
- how guardians handle reorgs
- how frontend displays finalized / pending / rejected evidence

## Frontend implications

The frontend should eventually be able to show:

- source transaction hash
- source block number
- source block hash, shortened for display
- finality status
- finalized / not finalized / rejected status

The frontend must not present a burn as ready for mint approval until the finality rule is satisfied.

## Current conclusion

Stage 1 requires sourceBlockNumber and sourceBlockHash as mandatory signed fields.

This closes the mandatory source block field blocker.

Implementation should still not begin until the remaining blockers are resolved and exact test vectors are produced.
