# Stage 1 processed burn atomicity

This document defines the Stage 1 processed-burn registry and atomic check-and-mint requirements for the XNTD-to-XXXL Gateway.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway must prevent one Ethereum XNTD burn event from minting XXXL more than once on X1.

This document defines the processed-burn registry and atomic check-and-mint model.

The key rule is:

One canonicalEventKey can produce at most one successful XXXL mint.

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

Theo identified atomic processed-burn check-and-mint as a pre-implementation blocker.

This document closes the requirement-definition layer for that blocker.

## Core principle

The X1 mint path must atomically:

1. verify the message and guardian signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL to x1RecipientBytes

These steps must be atomic or protected by X1 runtime guarantees.

No execution path may allow two successful mints for the same canonicalEventKey.

## Processed registry key

The processed-burn registry must key by:

canonicalEventKey

Where:

canonicalEventKey = keccak256(ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex))

The registry must not key by:

- sourceNonce alone
- sourceSender alone
- x1RecipientHash alone
- burnedAmount alone
- sourceBurnTxHash alone without sourceBurnEventIndex
- sourceBlockHash alone
- guardian messageHash alone
- relayer transaction id

The replay anchor remains the exact Ethereum log identity.

## Required processed record

A successful processed-burn record should include enough data for verification, indexing, and audit.

Recommended processed record fields:

- canonicalEventKey
- sourceChainId
- sourceToken
- sourceSender
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash
- sourceNonce
- x1RecipientHash
- x1RecipientBytes
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount
- mintToken
- messageHash
- guardianSetId or signerSetVersion if used
- processedAtSlot or processedAtBlock
- x1MintTxId or execution id if available

The exact X1 storage layout may differ, but the processed state must be auditable.

## Atomic sequence

The mint core must execute the successful path as one atomic state transition:

1. parse canonical message
2. verify canonical encoding / messageHash
3. verify Ed25519 guardian threshold
4. verify immutable route rules
5. verify x1RecipientBytes
6. derive canonicalEventKey
7. check processedRegistry[canonicalEventKey] is empty
8. write processedRegistry[canonicalEventKey]
9. mint XXXL to x1RecipientBytes
10. emit / record successful mint event

If X1 runtime requires a different internal ordering, the final implementation must preserve the same safety property:

A failed transaction must not allow a duplicate mint.

A duplicate transaction must not mint again.

## Check-before-mark risk

The dangerous anti-pattern is:

1. check canonicalEventKey is unprocessed
2. perform external or separately scheduled mint
3. mark canonicalEventKey as processed later

This is not acceptable.

If two relayers submit the same approved message, both could pass the unprocessed check before either marks the key.

Stage 1 must not use a non-atomic check-then-later-mark pattern.

## Mark-before-mint risk

Another dangerous pattern is:

1. mark canonicalEventKey as processed
2. mint fails
3. user cannot retry
4. Ethereum XNTD remains burned but XXXL was not minted

This is not acceptable unless X1 runtime guarantees rollback of all state changes when mint fails.

Required rule:

If mint fails, the processed mark must also fail / roll back.

There must be no stuck processed record without the corresponding mint.

## Duplicate submission behavior

Duplicate submissions are expected and safe if handled correctly.

Possible duplicate sources:

- same relayer retries
- multiple relayers submit the same signed message
- user resubmits mint approval
- watcher detects same finalized burn again
- network retry after uncertain transaction status

Required duplicate behavior:

- first valid execution may mint
- every later execution for the same canonicalEventKey must fail or return already processed without minting
- duplicate rejection must not alter mint amount
- duplicate rejection must not alter recipient
- duplicate rejection must not overwrite processed record
- duplicate rejection must not require guardian intervention

## Relayer race behavior

If two relayers submit the same approval at nearly the same time:

- at most one succeeds
- at most one mint occurs
- processed registry ends in exactly one processed state
- losing submission receives duplicate / already processed result
- no partial mint is possible
- no double mint is possible

The exact race behavior depends on X1 runtime transaction ordering and account locking, but the safety property must hold.

## Failure behavior

If verification fails before processed mark:

- no processed record is written
- no mint occurs

If route rule validation fails:

- no processed record is written
- no mint occurs

If guardian threshold fails:

- no processed record is written
- no mint occurs

If recipient validation fails:

- no processed record is written
- no mint occurs

If mint fails:

- no processed record remains unless the mint also succeeded
- failed execution must be retryable after the issue is corrected, if correction is possible
- no burned Ethereum event should be permanently blocked by a failed X1-side partial state update

## No privileged bypass

There must be no privileged path that can:

- mark arbitrary canonicalEventKey as processed without verification
- unmark processed burns
- overwrite processed records
- mint XXXL outside verified gateway messages
- mint again for an already processed canonicalEventKey
- bypass guardian threshold
- bypass immutable route rules

If an emergency mechanism exists, it must not be able to create monetary supply outside the verified path.

## Processed record immutability

After a canonicalEventKey is processed, its record should be immutable.

A processed record must not be overwritten to change:

- recipient
- amount
- source transaction
- source event index
- source block identity
- route rule values
- messageHash
- mint token

If corrections are ever needed, they should be represented as separate audit records, not mutation of the original processed mint record.

## Event / log requirements

The X1 mint core should emit or record a successful mint event containing:

- canonicalEventKey
- x1RecipientHash
- x1RecipientBytes or recipient account
- burnedAmount
- xxxlMintAmount
- sourceChainId
- sourceToken
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash

This event supports:

- frontend display
- watcher reconciliation
- audit
- incident investigation
- user support

## Watcher / indexer implications

Watchers should treat processed registry state as the X1-side source of truth for whether a burn was already minted.

Watcher states should include:

- source burn seen
- source burn finalized
- guardian approved
- relayer submitted
- X1 mint confirmed
- already processed
- rejected evidence
- failed execution

If X1 says canonicalEventKey is already processed, watchers must not attempt to produce another mint for that key.

## Frontend implications

Frontend should handle duplicate / retry states gracefully.

Suggested user-facing states:

- pending source burn
- waiting for finality
- waiting for guardian approval
- ready to relay
- relay submitted
- XXXL minted
- already processed
- rejected
- failed / retry possible

The UI must not imply that a duplicate successful mint is possible.

## Test vector implications

Future exact test vectors and tests must include:

- valid first mint for canonicalEventKey
- duplicate submission rejected
- duplicate relayer race scenario note
- wrong canonicalEventKey rejected
- already processed canonicalEventKey rejected
- failed signature does not mark processed
- failed route rule does not mark processed
- failed recipient validation does not mark processed
- failed mint rolls back processed mark
- processed record cannot be overwritten
- privileged bypass impossible or not present

## Current conclusion

Stage 1 requires an atomic processed-burn check-and-mint model.

The processed registry must key by canonicalEventKey.

For each canonicalEventKey, at most one successful XXXL mint may occur.

This closes the atomic processed-burn requirement-definition blocker.

Implementation should still not begin until the exact X1 runtime atomicity model, finality rule, deployment authority model, and exact test vectors are documented.
