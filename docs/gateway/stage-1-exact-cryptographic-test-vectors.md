# Stage 1 exact cryptographic test vectors

This document defines the exact Stage 1 cryptographic test vector profile for the XNTD-to-XXXL Gateway.

This is a design / test-vector specification document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway design now has all requirement-definition blockers closed.

This document turns the Stage 1 message, encoding, hash, signature, recipient, finality, replay, amount, and authority decisions into an exact test-vector profile.

The goal is:

Independent implementations must derive the same bytes, hashes, and signatures from the same Stage 1 sample burn event.

## Source context

This document builds on:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/gateway/stage-1-gateway-pre-implementation-blockers.md
- docs/gateway/stage-1-gateway-hash-signature-recipient-decisions.md
- docs/gateway/stage-1-gateway-mandatory-source-block-fields.md
- docs/gateway/stage-1-x1-mint-core-immutability.md
- docs/gateway/stage-1-processed-burn-atomicity.md
- docs/gateway/stage-1-ethereum-finality-rule.md
- docs/gateway/stage-1-recipient-safety-policy.md
- docs/gateway/stage-1-burn-amount-policy.md
- docs/gateway/stage-1-x1-deployment-authority-model.md

## Stage 1 cryptographic decisions

Final Stage 1 cryptographic decisions:

- hash function: keccak256
- guardian signature standard: Ed25519
- X1 recipient type: 32 raw bytes X1 / SVM public key
- x1RecipientHash: keccak256(x1RecipientBytes)
- canonicalEventKey: keccak256(encoded canonical event key preimage)
- messageHash: keccak256(domainSeparator || encodedGatewayMintMessage)
- guardian signature payload: messageHash
- sourceChainId: 1
- sourceChainWeightBps: 10000
- xxxlMintAmount: burnedAmount

## Exact encoding profile

This vector profile uses a simple fixed-width binary encoding.

All signed message fields are encoded as 32-byte words.

Rules:

- every message field is exactly 32 bytes
- field order is fixed
- no field may be omitted
- unused optional fields are encoded as zero
- all unsigned integers are encoded as uint256 big-endian 32-byte words
- big-endian means most significant byte first
- all bytes32 values are encoded as exact 32 bytes
- Ethereum addresses are encoded as 20 bytes left-padded with zero bytes to 32 bytes
- right-padding Ethereum addresses is invalid
- x1RecipientBytes are not included directly in the signed message
- x1RecipientHash is included in the signed message
- base58 is display-only and never canonical payload encoding
- hex strings are display-only and never canonical payload encoding
- no JSON canonicalization is used
- no string concatenation is used
- no decimal string amount encoding is used

## X1 / SVM review notes

Theo reviewed this exact vector profile through the X1 / SVM lens.

Conclusion:

- no X1-specific blocker was identified
- keccak256 is acceptable for this Stage 1 payload size
- Ed25519 signing over messageHash is acceptable and natural for SVM verification
- 32-byte fixed-width signed payload encoding is practical
- raw x1RecipientBytes bound through x1RecipientHash is the right approach

Required clarifications before vector generation:

- all uint256 fields are encoded as 32-byte big-endian values
- big-endian means most significant byte first
- Ethereum addresses are 20-byte values left-padded with zero bytes to 32 bytes
- X1 / SVM mint core deserialization must convert big-endian signed payload numbers into the native runtime format before arithmetic or comparisons
- this custom fixed-width signed payload encoding is not Borsh and is not account storage encoding
- the byte order applies to signed payload verification, not necessarily to X1 account storage
- canonicalEventKeyPreimage length is 128 bytes
- encodedGatewayMintMessage length is 608 bytes
- messageHashPreimage length is 640 bytes
- Ed25519 test signatures must use deterministic test-only keys

## Domain constant hashing

String labels are converted to bytes32 constants using:

keccak256(utf8(label))

Stage 1 constants:

- messageType = keccak256("X1_GATEWAY_MINT")
- routeId = keccak256("ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1")
- mintToken = keccak256("XXXL")
- protocolNameHash = keccak256("xEnchanted XNTD-to-XXXL Gateway")
- gatewayVersionHash = keccak256("Stage1")
- messageTypeFamilyHash = keccak256("X1GatewayMintMessage")

## Domain separator profile

The domain separator is:

domainSeparator = keccak256(
    protocolNameHash ||
    gatewayVersionHash ||
    targetX1NetworkId ||
    targetMintCoreId ||
    messageTypeFamilyHash
)

Where:

- protocolNameHash is bytes32
- gatewayVersionHash is bytes32
- targetX1NetworkId is uint256 encoded as 32 bytes
- targetMintCoreId is bytes32
- messageTypeFamilyHash is bytes32

The exact targetX1NetworkId and targetMintCoreId used for sample vectors are test-vector-only values.

They do not approve production deployment.

## Canonical event key profile

The canonical event key preimage is:

sourceChainId ||
sourceToken ||
sourceBurnTxHash ||
sourceBurnEventIndex

The canonicalEventKeyPreimage length is:

4 * 32 = 128 bytes

Where:

- sourceChainId is uint256 encoded as 32 bytes
- sourceToken is Ethereum address left-padded to 32 bytes
- sourceBurnTxHash is exact 32 bytes
- sourceBurnEventIndex is uint256 encoded as 32 bytes

canonicalEventKey = keccak256(canonicalEventKeyPreimage)

## Gateway mint message field order

The encodedGatewayMintMessage is the concatenation of these 19 fields in this exact order:

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

Each field is exactly 32 bytes.

The total encodedGatewayMintMessage length is:

19 * 32 = 608 bytes

X1 / SVM implementation note:

The signed payload uses custom fixed-width big-endian encoding.

The X1 mint core must deserialize these 32-byte big-endian numeric fields and convert them into the native runtime representation before arithmetic, range checks, or comparisons.

This encoding is for signature verification and test vectors.

It is not a requirement for X1 account storage layout.

## Message hash profile

The message hash preimage is:

domainSeparator || encodedGatewayMintMessage

The total messageHash preimage length is:

32 + 608 = 640 bytes

messageHash = keccak256(messageHashPreimage)

## Sample Vector 1: valid Stage 1 gateway mint message

Vector ID:

STAGE1_GATEWAY_VALID_001

### Human-readable input values

messageTypeLabel:

X1_GATEWAY_MINT

schemaVersion:

1

routeIdLabel:

ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1

sourceChainId:

1

sourceToken:

0x1111111111111111111111111111111111111111

sourceSender:

0x2222222222222222222222222222222222222222

sourceBurnTxHash:

0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa

sourceBurnEventIndex:

7

sourceBlockNumber:

19000000

sourceBlockHash:

0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb

sourceNonce:

42

x1RecipientBytes:

0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20

burnedAmount:

1000000000000000000000

sourceChainWeightBps:

10000

xxxlMintAmount:

1000000000000000000000

mintTokenLabel:

XXXL

deadlineOrFinalityBlock:

0

messageNonce:

0

targetX1NetworkId:

1001

targetMintCoreId:

0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc

### Derived constants to compute

The implementation script must compute:

- messageType = keccak256("X1_GATEWAY_MINT")
- routeId = keccak256("ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1")
- mintToken = keccak256("XXXL")
- protocolNameHash = keccak256("xEnchanted XNTD-to-XXXL Gateway")
- gatewayVersionHash = keccak256("Stage1")
- messageTypeFamilyHash = keccak256("X1GatewayMintMessage")
- x1RecipientHash = keccak256(x1RecipientBytes)
- canonicalEventKeyPreimage
- canonicalEventKey
- domainSeparator
- encodedGatewayMintMessage
- messageHashPreimage
- messageHash

### Exact expected output values

Exact computed values must be generated by the follow-up vector script.

This document locks the input values and binary encoding profile.

The follow-up script must write the exact hex outputs back into this document or into a generated vector JSON file.

## Ed25519 signing vector

The guardian signature vector must use deterministic test-only Ed25519 keys.

Required signing vector fields:

- guardianPrivateKeySeed
- guardianPublicKey
- messageHash
- guardianSignature

Rules:

- guardian private key material must be test-only
- test key must never be used in production
- signature payload is exactly messageHash
- signature must verify against guardianPublicKey and messageHash
- wrong messageHash must fail verification
- wrong public key must fail verification
- altered signature must fail verification

The exact Ed25519 test key and signature must be generated by the follow-up vector script.

## Required valid vectors

The final generated vector set must include:

1. valid Stage 1 gateway mint message
2. valid x1RecipientHash
3. valid canonicalEventKey
4. valid domainSeparator
5. valid encodedGatewayMintMessage
6. valid messageHash
7. valid Ed25519 guardian signature over messageHash

## Required invalid vectors

The final generated vector set must include invalid cases for:

- wrong field order
- omitted optional field
- deadlineOrFinalityBlock omitted instead of zero-filled
- messageNonce omitted instead of zero-filled
- amount encoded as decimal string
- wrong sourceChainId
- wrong sourceToken
- wrong sourceBurnTxHash
- wrong sourceBurnEventIndex
- wrong sourceBlockNumber
- wrong sourceBlockHash
- wrong canonicalEventKey
- wrong x1RecipientHash
- empty x1RecipientBytes
- non-32-byte x1RecipientBytes
- 32 zero bytes x1RecipientBytes
- burnedAmount equals zero
- xxxlMintAmount differs from burnedAmount
- sourceChainWeightBps differs from 10000
- wrong mintToken
- wrong routeId
- wrong domainSeparator
- wrong targetX1NetworkId
- wrong targetMintCoreId
- wrong messageHash
- wrong Ed25519 signature
- valid signature over a different messageHash
- duplicate canonicalEventKey already processed

## Implementation gate

Implementation must not begin until:

- exact vector script exists
- exact hashes are generated
- exact Ed25519 signature is generated
- independent implementation can reproduce the same outputs
- docs and generated vector data agree
- tests cover valid and invalid vectors

## Current conclusion

This document locks the Stage 1 exact cryptographic test vector profile:

- keccak256
- Ed25519
- 32-byte X1 recipient
- 32-byte fixed-width field encoding
- 19-field message order
- 608-byte encoded message
- 640-byte messageHash preimage
- canonicalEventKey derived from source chain, source token, source transaction hash, and source log index
- messageHash derived from domainSeparator plus encoded gateway mint message

This milestone defines exact vector inputs and encoding.

The next milestone should add a deterministic vector generation script and generated expected output values.
