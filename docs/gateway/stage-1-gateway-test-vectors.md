# Stage 1 gateway test vectors

This document defines the Stage 1 gateway test vector requirements for the XNTD-to-XXXL Gateway.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 gateway design now defines:

1. Ethereum burn event schema
2. deterministic gateway message schema
3. canonical encoding direction

This document defines the test vector layer that should exist before implementation.

The goal is to make independent implementations agree on:

- source burn event normalization
- canonical event key preimage
- gateway message field order
- recipient hash handling
- message hash preimage
- invalid encoding rejection cases
- route rule validation
- processed burn replay handling

## Relationship to existing design documents

This document builds on:

- `docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md`
- `docs/gateway/stage-1-xxxl-gateway-architecture.md`
- `docs/gateway/stage-1-xxxl-gateway-implementation-plan.md`
- `docs/gateway/stage-1-ethereum-burn-event-schema.md`
- `docs/gateway/stage-1-gateway-message-schema.md`
- `docs/gateway/stage-1-gateway-canonical-encoding.md`

The message schema defines the fields.

The canonical encoding document defines the preferred encoding direction.

This document defines the future vector set that should prove the encoding is unambiguous.

## Non-goals

This document does not provide final cryptographic hashes yet.

This document does not choose final hash function.

This document does not choose final signature standard.

This document does not implement vector generation.

This document does not implement verification.

This document does not approve production launch.

Final numeric hashes and signatures should be added only after:

- target X1 hash function is selected
- target X1 address / recipient type is selected
- guardian signature standard is selected
- canonical binary encoding is finalized
- domain separator is finalized
- target mint core identity format is finalized

## Core principle

A valid test vector must allow independent implementations to derive the same result from the same input.

If two implementations produce different canonical bytes or hashes for the same vector, the design is not ready for implementation.

If an invalid vector can be accepted by any conforming implementation, the design is not ready for implementation.

## Placeholder notation

Because final hash and signature choices are not finalized yet, this document uses placeholders:

- `HASH(value)`
- `BYTES32(value)`
- `ADDRESS20(value)`
- `UINT(value)`
- `UINT256(value)`
- `ENCODE(fields...)`
- `DOMAIN_SEPARATOR(fields...)`
- `MESSAGE_HASH(domainSeparator, encodedMessage)`
- `SIGNATURE(messageHash)`

These placeholders are not implementation syntax.

They describe what final vectors must later replace with exact bytes, hashes, and signatures.

## Required vector categories

Stage 1 should have at least these vector categories:

1. valid source burn event normalization
2. valid canonicalEventKey derivation
3. valid x1RecipientHash derivation
4. valid domain constants
5. valid domain separator
6. valid gateway message field order
7. valid messageHash preimage
8. valid full mint approval message
9. invalid wrong source chain
10. invalid wrong source token
11. invalid zero burned amount
12. invalid empty X1 recipient
13. invalid recipient hash mismatch
14. invalid sourceChainWeightBps
15. invalid xxxlMintAmount
16. invalid canonicalEventKey
17. invalid optional field omission
18. invalid field order
19. invalid string amount encoding
20. invalid JSON-dependent encoding
21. invalid replay / duplicate canonicalEventKey
22. invalid cross-domain signature reuse

## Sample source burn event

A future valid vector should start from one complete source event.

Placeholder fields:

- eventName: `XntdBurnedForX1Gateway`
- sourceSender: deterministic dummy Ethereum address
- x1Recipient: deterministic dummy X1 recipient
- x1RecipientBytes: exact recipient bytes
- burnedAmount: exact integer token amount
- sourceChainId: `1`
- sourceToken: expected dummy Ethereum XNTD token address
- sourceNonce: deterministic dummy nonce
- sourceBurnTxHash: deterministic dummy 32-byte transaction hash
- sourceBurnEventIndex: deterministic dummy log index
- sourceBlockNumber: deterministic dummy block number
- sourceBlockHash: deterministic dummy 32-byte block hash

The final vector should use deterministic dummy values, not live user data.

## Normalized source fields

The vector should define normalized fields after event parsing:

- sourceChainId as unsigned integer
- sourceToken as exact Ethereum address bytes
- sourceSender as exact Ethereum address bytes
- sourceBurnTxHash as exact 32 bytes
- sourceBurnEventIndex as unsigned integer
- sourceBlockNumber as unsigned integer
- sourceBlockHash as exact 32 bytes
- sourceNonce as unsigned integer
- burnedAmount as exact integer token units
- x1RecipientBytes as exact bytes
- x1RecipientHash as `HASH(x1RecipientBytes)`

Normalization rules:

- Ethereum addresses are bytes, not strings
- checksum casing is display-only
- transaction hash is exact 32 bytes
- block hash is exact 32 bytes
- amount is exact integer token units
- x1Recipient is exact bytes
- x1RecipientHash is derived from exact bytes

## canonicalEventKey vector

The vector should define:

`canonicalEventKeyPreimage = ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)`

Expected result:

`canonicalEventKey = HASH(canonicalEventKeyPreimage)`

The final vector must include:

- exact encoded preimage bytes
- exact resulting canonicalEventKey

The vector must prove that `sourceNonce` is not part of the replay key.

## x1RecipientHash vector

The vector should define:

`x1RecipientHash = HASH(x1RecipientBytes)`

The final vector must include:

- exact recipient bytes
- exact hash preimage
- exact x1RecipientHash

Invalid recipient vectors should include:

- empty recipient bytes
- recipient bytes changed after signing
- recipient hash mismatch

## Domain constants vector

The vector should define final constants:

- messageType = `HASH("X1_GATEWAY_MINT")`
- routeId = `HASH("ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1")`
- mintToken = `HASH("XXXL")`

The final vector must include exact values for:

- messageType
- routeId
- mintToken

If the target runtime uses native constant encoding instead of hash-derived constants, the final vector must document the exact alternative.

## Domain separator vector

The vector should define:

`domainSeparator = HASH(protocolName, gatewayVersion, targetX1NetworkId, targetMintCoreId)`

Required final vector fields:

- protocol name
- gateway version
- target X1 network id
- target mint core identifier
- exact encoded domain preimage
- exact domain separator

The domain separator must prevent replay across:

- testnet and mainnet
- different X1 deployments
- different mint cores
- different gateway message families

## Full message field order vector

The valid vector must encode fields in this exact order:

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

The final vector must include:

- each normalized field value
- exact encoded bytes for each field
- complete encoded message bytes
- messageHash

## Valid Stage 1 route values

The valid vector should use:

- schemaVersion = `1`
- sourceChainId = `1`
- sourceChainWeightBps = `10000`
- xxxlMintAmount = burnedAmount
- deadlineOrFinalityBlock = `0` if unused
- messageNonce = `0` if unused

This is Stage 1 full-weight Ethereum conversion.

This does not mean XXXL is Ethereum XNTD.

This does not create a price peg.

This is not a wrapped bridge.

## messageHash vector

The message hash vector should define:

`messageHashPreimage = ENCODE(domainSeparator, encodedGatewayMintMessage)`

`messageHash = HASH(messageHashPreimage)`

The final vector must include:

- domain separator
- encoded gateway mint message
- complete message hash preimage
- exact messageHash

Guardian signatures should be over `messageHash`.

## Guardian signature vector

Once signature standard is selected, final vectors should include:

- guardian public key / address
- guardian set id or signer set version if used
- messageHash
- signature
- expected verification result

The signature vector should not use real production guardian keys.

It should use deterministic test keys.

No private keys should be committed unless they are clearly dummy test-only keys and never used for production.

## Valid mint approval vector

A complete valid vector should include:

- source event
- normalized fields
- canonicalEventKey preimage
- canonicalEventKey
- x1RecipientHash preimage
- x1RecipientHash
- domain constants
- domain separator
- message fields
- encoded message bytes
- messageHash preimage
- messageHash
- guardian signatures
- expected X1 verification result: accepted
- expected processed key: canonicalEventKey
- expected mint recipient: x1Recipient
- expected mint amount: xxxlMintAmount

Expected result:

- accepted = true
- processedKey = canonicalEventKey
- mintToken = XXXL
- mintAmount = burnedAmount
- recipient = x1Recipient

## Invalid vectors

Wrong source chain:

- mutation: sourceChainId is not Ethereum mainnet
- expected result: rejected as wrong source chain

Wrong source token:

- mutation: sourceToken is not expected Ethereum XNTD token
- expected result: rejected as wrong source token

Zero burned amount:

- mutation: burnedAmount = 0
- expected result: rejected as zero burned amount

Empty recipient:

- mutation: x1RecipientBytes is empty
- expected result: rejected as empty recipient

Recipient hash mismatch:

- mutation: x1RecipientBytes and x1RecipientHash do not match
- expected result: rejected as recipient hash mismatch

Wrong sourceChainWeightBps:

- mutation: sourceChainWeightBps is not 10000
- expected result: rejected as wrong route weight

Wrong xxxlMintAmount:

- mutation: xxxlMintAmount does not equal burnedAmount
- expected result: rejected as wrong mint amount

Wrong canonicalEventKey:

- mutation: canonicalEventKey does not include log index or uses wrong preimage
- expected result: rejected as canonical event key mismatch

Optional field omitted:

- mutation: deadlineOrFinalityBlock or messageNonce omitted
- expected result: rejected as non-canonical encoding

Wrong field order:

- mutation: fields encoded in a different order
- expected result: rejected as non-canonical field order

Amount encoded as string:

- mutation: burnedAmount encoded as decimal string
- expected result: rejected as non-canonical amount encoding

JSON-dependent encoding:

- mutation: message encoded as JSON object as canonical payload
- expected result: rejected as unsupported canonical encoding

Duplicate canonicalEventKey:

- mutation: canonicalEventKey already processed on X1
- expected result: rejected as duplicate source burn

Cross-domain replay:

- mutation: same signature attempted against different targetMintCoreId, network, or message family
- expected result: rejected as domain mismatch

## Future fixture file layout

When implementation begins, fixture files may be added under:

`fixtures/gateway/stage-1/`

Potential fixture files:

- valid-ethereum-xntd-burn-to-xxxl.json
- invalid-wrong-source-chain.json
- invalid-wrong-source-token.json
- invalid-zero-burned-amount.json
- invalid-empty-recipient.json
- invalid-recipient-hash-mismatch.json
- invalid-wrong-route-weight.json
- invalid-wrong-mint-amount.json
- invalid-wrong-canonical-event-key.json
- invalid-optional-field-omission.json
- invalid-wrong-field-order.json
- invalid-string-amount.json
- invalid-json-canonicalization.json
- invalid-duplicate-canonical-event-key.json
- invalid-cross-domain-replay.json

These files should be added only when final encoding and hash choices are made.

## Production readiness implication

Production implementation should not begin until:

- final hash function is chosen
- final signature standard is chosen
- final X1 recipient type is chosen
- final canonical binary encoding is chosen
- final domain separator is chosen
- valid vector produces exact bytes and hashes
- invalid vectors are rejected by independent implementations
- guardian runtime, relayer, watcher, and X1 verifier share the same vector expectations

## Current preferred direction

Preferred Stage 1 vector direction:

- document placeholder vectors now
- add exact cryptographic vectors later
- never use live user data in vectors
- never include real secrets
- require independent implementations to match exact bytes and hashes
- require invalid vectors to fail deterministically
- use vectors as the bridge between design and implementation

Implementation should still not begin until final encoding, hash function, signature standard, finality rule, X1 recipient type, and exact test vectors are reviewed.
