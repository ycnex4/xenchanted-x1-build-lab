# Stage 1 recipient safety policy

This document defines the Stage 1 recipient safety policy for the XNTD-to-XXXL Gateway.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 burns Ethereum XNTD before minting XXXL on X1.

If a user provides an invalid, malformed, empty, zero, or unusable X1 recipient, the user may permanently burn XNTD without receiving a usable X1-side result.

This document defines the recipient safety policy for Stage 1.

The core rule is:

Invalid X1 recipients must be rejected before guardian approval and before X1 mint execution.

Frontend validation should also reject them before the Ethereum burn transaction whenever possible.

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

## Canonical recipient format

Stage 1 X1 recipient format is:

- exactly 32 raw bytes
- X1 / SVM public key format
- not a string
- not base58 text
- not variable-length bytes
- not an Ethereum address
- not an EIP-55 address
- not checksum-casing dependent

Base58 may be used only as display / input format.

Protocol-level recipient bytes must be exactly 32 raw bytes.

## Recipient hash

Stage 1 recipient hash remains:

x1RecipientHash = keccak256(x1RecipientBytes)

Where:

- x1RecipientBytes is exactly 32 raw bytes
- x1RecipientHash is included in the signed gateway message
- x1RecipientBytes are carried in execution / evidence payload
- X1 mint core verifies keccak256(x1RecipientBytes) == x1RecipientHash

A hash match is necessary but not sufficient.

The recipient bytes must also pass recipient safety validation.

## Mandatory rejection cases

Stage 1 must reject:

- empty recipient
- missing recipient
- recipient length not equal to 32 bytes
- malformed recipient bytes
- base58 string used directly as canonical bytes
- Ethereum address used as X1 recipient bytes
- 32 zero bytes recipient
- x1RecipientHash mismatch
- recipient format that cannot be decoded into exactly 32 bytes
- recipient value forbidden by the final X1 runtime policy

These rejection cases apply to:

- frontend validation
- guardian validation
- X1 mint core execution validation where possible

## 32 zero bytes policy

Stage 1 must reject the 32-byte all-zero recipient.

Reason:

- it is a common null / default value
- it is not a safe user-controlled recipient
- minting XXXL to it may permanently lose the user's X1-side mint result
- it is easy to detect consistently

Policy:

x1RecipientBytes == 0x0000000000000000000000000000000000000000000000000000000000000000 must be rejected.

## Known burn / blackhole recipient policy

Stage 1 must reject known protocol-forbidden burn / blackhole recipients if the X1 runtime or community standard defines such addresses.

Current Stage 1 minimum policy:

- reject 32 zero bytes
- treat additional known burn / blackhole recipient list as an implementation-time policy item

Before production, the project must decide whether to maintain a small static forbidden-recipient list.

If such a list exists, it must be:

- deterministic
- public
- documented
- small
- immutable or governed by a clearly separated safety process
- unable to censor ordinary valid recipients arbitrarily

The recipient safety policy must not become a discretionary blacklist.

## No discretionary recipient censorship

Recipient safety checks exist to prevent user loss and malformed execution.

They must not become a general censorship mechanism.

Unacceptable policy:

- arbitrary admin-controlled recipient blacklist
- guardian-controlled recipient censorship
- relayer-controlled recipient filtering that changes protocol meaning
- mutable hidden recipient denylist

Acceptable policy:

- reject malformed recipient
- reject wrong-length recipient
- reject 32 zero bytes
- reject known protocol-defined burn / blackhole recipients if documented before production

## Frontend validation

Frontend should validate recipient before the user burns Ethereum XNTD.

Frontend should reject:

- empty input
- invalid base58 input
- decoded recipient not exactly 32 bytes
- all-zero recipient
- known forbidden burn / blackhole recipients if policy exists
- Ethereum address pasted as recipient
- malformed copied value

Frontend should clearly explain that:

- XXXL is minted on X1
- recipient must be an X1 / SVM address
- Ethereum address is not a valid X1 recipient
- base58 is display format only
- protocol uses decoded 32-byte recipient bytes

Frontend validation is a UX safety layer.

It does not replace guardian or mint core validation.

## Ethereum burn function validation

The Ethereum burn function may receive x1Recipient as bytes, string, or another format depending on implementation.

Preferred direction:

- accept recipient input in a format that can be validated before burn
- reject empty recipient
- reject malformed recipient if format permits
- reject all-zero 32-byte recipient if raw bytes are supplied
- emit both x1Recipient and x1RecipientHash according to the event schema

Open implementation detail:

Whether Ethereum-side validation can fully validate X1 / SVM recipient format depends on the final function parameter type.

Minimum Ethereum-side requirement:

Do not allow obviously empty recipient evidence.

Preferred Ethereum-side requirement:

Reject anything that cannot be decoded into exactly 32 recipient bytes before burning.

## Guardian validation

Guardians must reject burn evidence if:

- x1Recipient is missing
- x1Recipient cannot be decoded into exactly 32 raw bytes
- x1RecipientBytes are 32 zero bytes
- x1RecipientHash does not equal keccak256(x1RecipientBytes)
- x1Recipient appears to be a malformed display string
- event payload is incomplete
- event payload is ambiguous
- final forbidden-recipient policy rejects the recipient

Guardians must not sign a message for an invalid recipient.

## X1 mint core validation

X1 mint core must reject execution if:

- raw x1RecipientBytes are missing
- raw x1RecipientBytes length is not exactly 32 bytes
- raw x1RecipientBytes are 32 zero bytes
- keccak256(x1RecipientBytes) does not equal signed x1RecipientHash
- recipient violates final X1 runtime recipient policy

The mint core should not trust frontend validation.

The mint core should not trust relayer-provided recipient data unless it matches the signed hash and safety checks.

## Relayer behavior

Relayers must not modify recipient data.

Relayers must submit:

- signed x1RecipientHash
- raw x1RecipientBytes matching that hash
- evidence references

If relayer-submitted x1RecipientBytes do not match the signed x1RecipientHash, X1 mint core must reject.

If recipient validation fails, relayer should surface a clear rejected / invalid recipient state.

## Watcher / indexer behavior

Watchers should detect and label invalid-recipient evidence.

Possible states:

- recipient missing
- recipient malformed
- recipient wrong length
- recipient zero bytes
- recipient hash mismatch
- recipient forbidden by policy
- recipient valid

Watchers should not promote invalid-recipient evidence to guardian approval.

## Frontend user-facing states

Frontend should be able to show:

- invalid X1 recipient
- Ethereum address is not a valid X1 recipient
- recipient decodes to wrong byte length
- zero recipient rejected
- recipient hash mismatch
- recipient accepted
- burn submitted
- waiting for finality
- guardian approval pending
- XXXL minted

The most important safety UX rule:

Do not let the user burn XNTD while recipient input is clearly invalid.

## Event schema implications

The Ethereum burn event direction remains:

- x1RecipientHash
- x1Recipient

Stage 1 design should ensure these values can be checked deterministically.

If x1Recipient is emitted as a string, the canonical decoding rule must be defined before implementation.

If x1Recipient is emitted as bytes, it should represent canonical recipient bytes or an unambiguous encoded form.

Preferred production direction:

Emit data in a way that allows guardians to derive exactly one x1RecipientBytes value.

## Test vector implications

Future tests and vectors must include:

- valid 32-byte recipient
- valid base58 display decoded into 32 bytes
- empty recipient rejected
- missing recipient rejected
- wrong-length recipient rejected
- 32 zero bytes rejected
- Ethereum address used as X1 recipient rejected
- base58 string used directly as canonical bytes rejected
- x1RecipientHash mismatch rejected
- known forbidden recipient rejected if policy list exists
- relayer recipient substitution rejected
- frontend invalid recipient before burn scenario

## Current conclusion

Stage 1 recipient safety policy requires canonical 32-byte X1 / SVM recipient bytes.

Stage 1 must reject empty, malformed, non-32-byte, all-zero, hash-mismatched, and policy-forbidden recipients.

Frontend should prevent clearly invalid recipient burns before the Ethereum transaction.

Guardians must reject invalid recipient evidence.

X1 mint core must reject invalid recipient execution payloads.

This closes the recipient safety policy requirement-definition blocker.

Implementation should still not begin until burn amount min/max policy, exact X1 deployment authority model, and exact cryptographic test vectors are documented.
