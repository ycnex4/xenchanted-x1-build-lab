# Stage 1 Ethereum finality rule

This document defines the Stage 1 Ethereum finality rule for the XNTD-to-XXXL Gateway.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 Gateway guardians must sign only canonical and finalized Ethereum burn evidence.

This document defines the finality requirement for Ethereum-side XNTD burn events before guardians may approve X1 XXXL minting.

The core rule is:

Guardians must not sign burn evidence until the source Ethereum block is finalized enough under the Stage 1 finality policy.

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

## Finality objective

The finality rule protects against:

- signing reorged-out burn events
- signing non-canonical burn events
- signing events before sufficient Ethereum finality
- provider disagreement
- stale receipt data
- sourceBlockHash mismatch
- transaction hash / log index evidence without stable block context

The finality rule does not replace X1 replay protection.

Replay protection remains:

canonicalEventKey = keccak256(ENCODE(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex))

Finality protects the source evidence before a valid message is signed.

## Required source block binding

Every signed Stage 1 message must include:

- sourceBlockNumber
- sourceBlockHash

Guardian verification must confirm:

- transaction receipt exists
- transaction succeeded
- receipt block number equals sourceBlockNumber
- receipt block hash equals sourceBlockHash
- burn log exists in that receipt
- burn log index equals sourceBurnEventIndex
- block is canonical
- block satisfies Stage 1 finality rule

If any of these checks fail, guardians must not sign.

## Preferred finality direction

Preferred Stage 1 finality direction:

Use Ethereum finalized block status when reliable RPC support exists.

A guardian may accept a burn event as finalized if:

- the burn receipt block is at or before the latest finalized Ethereum block
- sourceBlockHash matches the canonical block hash at sourceBlockNumber
- the burn transaction succeeded
- the expected event exists in the receipt
- all Stage 1 guardian acceptance rules pass

This is the preferred direction because Ethereum finality after proof-of-stake finalization is stronger than a simple confirmation count.

## Conservative fallback direction

If finalized block tag support is unavailable, inconsistent, or unreliable, guardians may use a conservative confirmation-depth fallback.

Fallback requirement:

- source block must be at least N confirmations deep
- N must be chosen conservatively before implementation
- provider responses must agree on sourceBlockHash
- provider responses must agree that the source block is canonical
- provider responses must agree that the transaction receipt is in that block

The exact N is not fixed by this document.

Production implementation must choose and document the exact confirmation depth if fallback mode is used.

## Provider agreement

Guardians should not rely on a single weak provider response when finality is uncertain.

Preferred provider policy:

- use at least two independent Ethereum RPC providers for finality-critical checks
- confirm sourceBlockHash for sourceBlockNumber
- confirm transaction receipt blockHash
- confirm latest finalized block or confirmation depth
- reject or delay if providers disagree

Provider disagreement should not be resolved by guessing.

If providers disagree about source block identity, receipt inclusion, or finality, guardians must delay or reject the evidence until the disagreement is resolved.

## Reorg handling

Guardians must reject evidence if:

- sourceBlockHash is no longer canonical
- transaction receipt is no longer found
- receipt blockHash differs from signed sourceBlockHash
- burn log is missing from the canonical receipt
- sourceBurnEventIndex no longer identifies the expected burn event
- source block was reorged out before approval

If a burn was seen but later reorged out before guardian approval, no approval should be signed.

If an approval was signed based on a block later discovered to be non-canonical before X1 mint execution, relayers and watchers should treat it as rejected evidence and not submit it.

## Guardian acceptance finality checks

Before signing, guardians must verify:

- sourceChainId is Ethereum mainnet
- sourceToken is expected Ethereum XNTD token
- burn transaction succeeded
- expected burn event exists
- sourceBurnTxHash matches the receipt
- sourceBurnEventIndex matches the expected log
- sourceBlockNumber matches receipt block number
- sourceBlockHash matches receipt block hash
- sourceBlockHash matches canonical block lookup at sourceBlockNumber
- block satisfies finalized block rule or conservative confirmation fallback
- burnedAmount > 0
- x1RecipientBytes are valid
- x1RecipientHash is correct
- canonicalEventKey is correct
- canonicalEventKey is not already processed on X1, if checkable at signing time

## Relayer finality responsibility

The relayer should not attempt to submit an approval for evidence that is known to be non-final, reorged, rejected, or disputed.

However, finality verification is primarily guardian responsibility.

The X1 mint core cannot independently verify Ethereum finality unless a separate on-chain light client or proof system exists.

For Stage 1, finality is enforced by guardian verification policy and signed message discipline.

## Watcher / indexer states

Watchers should track source burns through clear finality states:

- observed
- confirmed
- waiting for finality
- finalized
- guardian approval pending
- guardian approved
- relayer submitted
- minted on X1
- rejected
- reorged out
- provider disagreement
- already processed

A burn should not move to guardian-approved state until finality policy is satisfied.

## Frontend states

The frontend should not present a burn as ready for mint approval until finality is satisfied.

Suggested frontend statuses:

- Burn submitted
- Burn confirmed
- Waiting for Ethereum finality
- Finalized
- Guardian approval pending
- Guardian approved
- Relayer submitted
- XXXL minted
- Rejected evidence
- Reorged out
- Already processed

The frontend should make clear that Ethereum confirmation is not the same as final gateway approval.

## Invalid finality cases

The following must be invalid:

- burn transaction failed
- burn transaction not found
- expected burn event not found
- sourceBlockNumber missing
- sourceBlockHash missing
- sourceBlockHash wrong length
- receipt blockHash differs from sourceBlockHash
- canonical block hash at sourceBlockNumber differs from sourceBlockHash
- source block is newer than finalized block
- source block has insufficient fallback confirmations
- providers disagree about source block identity
- burn event is reorged out
- burn log index points to a different event
- source chain is not Ethereum mainnet
- source token is not expected XNTD token

## Relationship to sourceBlockHash

sourceBlockHash is required because finality is not only about transaction hash.

The signed message must bind to:

- exact transaction hash
- exact log index
- exact block number
- exact block hash

This gives guardians, watchers, relayers, and auditors a complete source evidence identity.

## Relationship to processed registry

Finality determines whether a burn can be approved.

Processed registry determines whether an approved burn has already minted.

They are separate protections:

- finality protects source evidence correctness
- processed registry protects X1 replay / duplicate minting

Both are required.

## Production decision still required

This document defines the finality model.

Before implementation, the project must still choose the exact operational rule:

- finalized block tag only
- finalized block tag preferred with confirmation fallback
- fixed confirmation depth
- multi-provider policy
- exact number of providers
- exact confirmation depth N if fallback is used
- exact behavior for provider disagreement

Recommended production direction:

- prefer finalized block tag when available and reliable
- use conservative confirmation-depth fallback only when finalized block data is unavailable or unreliable
- require provider agreement for finality-critical fields

## Test vector implications

Future tests and scenario vectors must include:

- valid finalized burn
- burn not finalized yet
- wrong sourceBlockHash
- wrong sourceBlockNumber
- receipt blockHash mismatch
- provider disagreement
- reorged-out burn event
- insufficient confirmations
- finalized block older than burn block
- duplicate processed burn after finalized approval

## Current conclusion

Stage 1 guardians must sign only canonical Ethereum burn evidence that satisfies the Stage 1 finality rule.

The signed message must include sourceBlockNumber and sourceBlockHash.

The preferred finality direction is Ethereum finalized block status, with a conservative confirmation-depth fallback only if finalized block support is unavailable or unreliable.

This closes the Ethereum finality rule requirement-definition blocker.

Implementation should still not begin until exact provider policy, fallback confirmation depth, X1 authority model, and exact test vectors are documented.
