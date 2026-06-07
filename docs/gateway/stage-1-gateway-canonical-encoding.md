# Stage 1 gateway canonical encoding

This document defines the preferred canonical encoding direction for Stage 1 XNTD-to-XXXL Gateway messages.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 gateway message schema defines what fields guardians sign.

This document defines the preferred direction for how those fields should become canonical bytes and hashes.

The goal is to prevent ambiguous encodings, signature reuse, replay bugs, and mint approval malleability.

The Stage 1 route remains:

    Ethereum XNTD burn -> X1-native XXXL mint

The encoding must preserve the architecture boundary:

    gateway guardians = verification layer
    immutable mint core / route rules = monetary conversion rules

Guardians verify evidence and sign canonical messages.

Guardians must not control XXXL monetary policy.

## Relationship to existing design documents

This document builds on:

- `docs/gateway/xntd-to-xxxl-burn-to-mint-gateway-design.md`
- `docs/gateway/stage-1-xxxl-gateway-architecture.md`
- `docs/gateway/stage-1-xxxl-gateway-implementation-plan.md`
- `docs/gateway/stage-1-ethereum-burn-event-schema.md`
- `docs/gateway/stage-1-gateway-message-schema.md`

The message schema defines the conceptual fields.

This document defines how those conceptual fields should be encoded.

## Non-goals

This document does not implement:

- Ethereum burn contract
- X1 XXXL token
- X1 mint core
- processed burn registry
- guardian runtime
- relayer runtime
- watcher runtime
- frontend gateway flow
- production deployment

This document does not choose final production libraries.

This document does not approve production launch.

## Main security principle

Every participant deriving the same Stage 1 message from the same finalized Ethereum burn log must produce the same canonical bytes.

If two honest implementations can produce different bytes for the same intended message, the encoding is not acceptable.

If two different messages can produce the same bytes or hash, the encoding is not acceptable.

If signatures can be reused across routes, chains, tokens, schemas, or message types, the encoding is not acceptable.

## Preferred encoding style

Preferred direction:

- explicit field order
- fixed-width numeric fields
- fixed-width hashes
- explicit dynamic-bytes hashing
- bytes32 constants for domain fields
- no string concatenation
- no JSON canonicalization
- no locale-dependent formatting
- no decimal string amount encoding
- no implicit field omission
- no unordered maps
- no optional-field ambiguity

The signed payload should be a deterministic typed binary payload or deterministic typed hash.

The final choice depends on the target X1 runtime and guardian signing stack.

## Canonical field order

The Stage 1 message should use one fixed field order.

Preferred order:

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

The raw `x1Recipient` may be included separately as evidence / execution data, but the signed fixed-width payload should include `x1RecipientHash`.

If the target X1 runtime supports safe dynamic bytes inside the typed signed payload, `x1Recipient` can be included directly with a length prefix. Otherwise, the preferred fixed-width signed field is `x1RecipientHash = hash(x1RecipientBytes)`.

## Required fields vs optional fields

The message schema lists `deadlineOrFinalityBlock` and `messageNonce` as optional design fields.

For canonical encoding, optional fields are dangerous if omitted inconsistently.

Preferred Stage 1 direction:

- include all fields in fixed order
- if a field is unused, encode it as zero
- do not omit fields from the canonical payload

Therefore:

    deadlineOrFinalityBlock = 0 if unused
    messageNonce = 0 if unused

This avoids multiple valid encodings for the same message.

## Domain constants

String labels should not be encoded as raw variable-length strings in the signed payload.

Preferred direction:

    messageType = hash("X1_GATEWAY_MINT")
    routeId     = hash("ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1")
    mintToken   = hash("XXXL")

These values should become fixed-width constants.

The exact hash function must match the X1 verification runtime.

If the X1 runtime uses a different native hash, the constants must be regenerated deterministically and documented with test vectors before implementation.

## Numeric encoding

Numeric fields should use fixed-width unsigned integer encoding.

Preferred conceptual widths:

- schemaVersion: uint32 or uint64
- sourceChainId: uint64 or uint256
- sourceBurnEventIndex: uint64 or uint256
- sourceBlockNumber: uint64 or uint256
- sourceNonce: uint64 or uint256
- burnedAmount: uint256
- sourceChainWeightBps: uint32 or uint64
- xxxlMintAmount: uint256
- deadlineOrFinalityBlock: uint64 or uint256
- messageNonce: uint64 or uint256

The final widths must be chosen to match the target X1 runtime.

Amounts should not be encoded as decimal strings.

Amounts should not be encoded as floating-point numbers.

Amounts should preserve exact integer token units.

## Address and token encoding

Ethereum addresses should be encoded as fixed-width 20-byte values or a clearly defined left-padded fixed-width representation if the target runtime requires 32-byte fields.

Preferred conceptual rules:

- sourceToken is the exact Ethereum XNTD token address
- sourceSender is the exact Ethereum sender address from the burn event
- addresses are normalized as bytes, not case-sensitive strings
- checksum casing is display-only and not part of canonical encoding

If 32-byte fields are required:

    address32 = leftPad20ByteAddressTo32Bytes(address)

The padding rule must be explicit.

Right-padding and left-padding must not both be accepted.

## Hash field encoding

The following should be fixed-width hash fields:

- sourceBurnTxHash
- sourceBlockHash
- canonicalEventKey
- x1RecipientHash
- messageHash

For Ethereum transaction hash and block hash:

- use the exact 32-byte hash from Ethereum
- do not encode as hex string in the canonical payload
- hex string is display-only

## x1Recipient encoding

`x1Recipient` is still an open type.

Potential forms:

- bytes
- bytes32
- X1-native address type

Until the final X1 recipient type is chosen, the safest design direction is:

    x1RecipientBytes = exact user-provided recipient bytes
    x1RecipientHash  = hash(x1RecipientBytes)

The signed payload includes `x1RecipientHash`.

The execution payload may include `x1RecipientBytes`.

X1 verification should check:

    hash(x1RecipientBytes) == x1RecipientHash

Then mint to the decoded / validated X1 recipient.

This avoids embedding an unstable recipient format directly into the monetary signed payload before X1 address constraints are finalized.

## canonicalEventKey encoding

The canonical replay key direction remains:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Preferred canonical input order:

1. sourceChainId
2. sourceToken
3. sourceBurnTxHash
4. sourceBurnEventIndex

Preferred rules:

- use typed binary encoding
- use fixed-width chain id
- use fixed-width token address representation
- use exact 32-byte sourceBurnTxHash
- use fixed-width event index
- do not use string concatenation
- do not include sourceNonce as replay key material
- do not include x1Recipient as replay key material

Reason:

The replay anchor should be the exact Ethereum log.

A transaction hash plus log index identifies the emitted burn event.

The sourceNonce remains useful for indexing and user display, but the canonical replay key should bind to the actual log evidence.

## messageHash encoding

Preferred direction:

    messageHash = hash(domainSeparator, encodedGatewayMintMessage)

The message hash should bind:

- protocol / system domain
- message type
- schema version
- route id
- source chain
- source token
- source event identity
- recipient hash
- burned amount
- source chain weight
- XXXL mint amount
- mint token

The signed hash should not allow replay into:

- another message type
- another schema version
- another route
- another source chain
- another source token
- another mint token
- another X1 deployment environment

## Domain separator

Preferred domain separator contents:

- protocol name
- gateway version
- target chain / X1 network id
- target mint core identifier
- message type family

Conceptual example:

    domainSeparator = hash(
        "xEnchanted XNTD-to-XXXL Gateway",
        "Stage1",
        targetX1NetworkId,
        targetMintCoreId
    )

The exact encoding must be finalized before implementation.

The domain separator should prevent guardian signatures from being reused across:

- testnet and mainnet
- old and new mint cores
- different X1 deployments
- different gateway message families

## Guardian signed payload boundary

Preferred signing boundary:

    signedPayload = messageHash

Guardians should sign the deterministic message hash, not loosely structured JSON.

The guardian runtime may display decoded fields for human review, but the signature must bind to exact canonical bytes.

If a typed-data signing standard is used, the typed-data domain and struct definition must be documented and locked.

If a raw binary hash is used, the preimage format must be documented and locked.

## Evidence payload vs signed payload

The system may need two payload layers:

1. signed message payload
2. evidence payload

The signed message payload should be compact, canonical, and fixed.

The evidence payload can include additional data needed for verification, display, or audit.

Evidence payload may include:

- raw x1Recipient bytes
- Ethereum transaction hash
- Ethereum log proof data
- event ABI decode
- block number
- block hash
- finality metadata
- guardian set metadata
- relayer metadata

The evidence payload must not change the signed monetary meaning.

The X1 verifier must not trust unsiged evidence fields to override signed fields.

## Invalid encoding examples

The following should be invalid:

- JSON object where field order matters implicitly
- JSON object where numeric amounts are decimal strings
- hex strings without length normalization
- mixed-case address strings treated as canonical bytes
- string concatenation such as `chainId + token + txHash + logIndex`
- payloads where optional fields are omitted by one implementation and zero-filled by another
- payloads where sourceNonce replaces transaction hash + log index
- payloads where guardians choose xxxlMintAmount manually
- payloads where route weight is omitted from the signed payload
- payloads where mintToken is omitted from the signed payload
- payloads without domain separation
- payloads reusable across testnet and mainnet
- payloads reusable across different mint cores

## Route rule binding

The immutable Stage 1 route rule is:

    sourceChainWeightBps = 10000
    xxxlMintAmount = burnedAmount

The canonical message should include both values.

The X1 mint core should also independently derive or verify them from immutable route rules.

This double-binding is intentional:

- the signed message states the expected value
- the mint core checks that the value matches the route rule

Guardians do not get discretion over the conversion rule.

## Finality binding

The canonical message includes source block fields:

- sourceBlockNumber
- sourceBlockHash

Guardians must sign only finalized canonical Ethereum evidence.

The exact finality rule remains open.

Potential approaches:

- finalized block tag if supported by the Ethereum provider stack
- conservative confirmation depth
- multiple-provider canonicality check
- guardian-local finality policy

Whatever rule is chosen, signatures should bind to the observed block hash so reorged-out evidence cannot remain silently valid.

## Processed burn registry binding

X1 processed burn registry should key by:

    canonicalEventKey

Preferred behavior:

1. verify message and signatures
2. check `canonicalEventKey` is unprocessed
3. mark `canonicalEventKey` processed
4. mint XXXL to recipient

The exact atomicity model depends on X1 runtime.

The registry should not key by:

- sourceNonce alone
- sourceSender alone
- x1Recipient alone
- burnedAmount alone
- transaction hash alone without log index

## Test vector requirement before implementation

Before implementation, this document should be followed by test vectors.

Required test vectors:

- domain constants
- x1RecipientHash example
- canonicalEventKey example
- full message encoded bytes example
- messageHash example
- invalid field order example
- invalid optional omission example
- invalid amount string example
- invalid wrong route id example
- invalid wrong mint token example

No implementation should begin until at least one complete example message can be encoded and hashed identically by independent code.

## Current preferred direction

Preferred Stage 1 canonical encoding direction:

- fixed field order
- fixed-width numeric and hash fields
- bytes32 domain constants
- x1RecipientHash inside signed payload
- raw x1Recipient bytes as execution / evidence payload
- canonicalEventKey derived from source chain, source token, transaction hash, and log index
- messageHash derived from domain separator plus encoded message
- guardian signatures over messageHash
- X1 verification checks immutable route rules independently
- no JSON canonicalization
- no string concatenation
- no implicit optional field omission

Implementation should still not begin until encoding, hash function, signature standard, finality rule, and X1 recipient type are reviewed.
