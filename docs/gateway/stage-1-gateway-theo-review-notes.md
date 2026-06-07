# Stage 1 gateway Theo review notes

This document records Theo's review of the Stage 1 XNTD-to-XXXL Gateway design chain.

This is a design / review notes document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Reviewed design chain

Theo reviewed the Stage 1 Gateway design chain after the following documents were added:

- docs/gateway/stage-1-ethereum-burn-event-schema.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-test-vectors.md
- docs/checkpoints/current-design-checkpoint.md

The reviewed Stage 1 route remains:

Ethereum XNTD burn -> immutable X1 XXXL mint core -> XXXL mint

## High-level review result

Theo's conclusion:

Stage 1 design is architecturally mature.

The main structural decisions are correct.

The strongest parts of the design are:

- immutable mint core as a structural guard against guardian overreach
- canonical encoding with double-binding
- route rules included in signed message and independently checked by mint core
- gateway framing as burn-to-mint, not a standard wrapped bridge
- domainSeparator to prevent cross-environment signature reuse
- keeping sourceNonce outside the replay key

## Guardian / immutable mint-core boundary

Theo confirmed that the guardian / immutable mint-core boundary is clean.

The design separates three layers:

1. verification work by guardians
2. monetary conversion rules in immutable mint core
3. relayer execution / transport without discretion

The key boundary is:

- guardians verify burn evidence and sign deterministic messages
- immutable mint core owns route rules and conversion rules
- relayer transports approvals but cannot define monetary values

Theo confirmed that guardians must not control XXXL monetary policy.

Theo also recommended making explicit that X1 mint core must validate guardian-provided route fields against hardcoded Stage 1 values.

This means the mint core should check:

- routeId matches the hardcoded Stage 1 route
- sourceChainWeightBps equals 10000
- xxxlMintAmount equals burnedAmount
- mintToken is XXXL
- sourceChainId is Ethereum mainnet
- sourceToken is expected Ethereum XNTD token

## Replay anchor

Theo confirmed that transaction hash plus log index is the right replay anchor for the Ethereum Stage 1 route.

The canonical replay key remains:

canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

This covers:

- replay between chains
- replay between source tokens
- replay of the same burn event
- multiple events in the same transaction

Theo noted that future non-Ethereum routes may need different event identity rules, but this is not a Stage 1 blocker.

## sourceNonce role

Theo confirmed that sourceNonce should remain outside the primary replay key.

sourceNonce is useful for:

- indexing
- user display
- event ordering visibility

sourceNonce should not be the replay anchor because:

- nonce may reset after contract redeployment
- nonce does not bind to a specific transaction
- nonce does not bind to a specific log event

The replay anchor should remain the exact emitted Ethereum log.

## x1RecipientHash and raw recipient bytes

Theo confirmed that using x1RecipientHash inside the signed payload and raw x1Recipient bytes in execution / evidence payload is a valid direction for Stage 1.

This solves the current open recipient-type problem:

- signed payload can stay fixed-width
- raw recipient bytes can still be used for execution
- X1 verifier can check hash(rawRecipientBytes) == signed x1RecipientHash

Theo's warning:

Recipient encoding must be normalized before implementation.

If X1 accepts multiple byte encodings for the same recipient, this can create recipient malleability.

This may not directly create unauthorized minting, but it can create ambiguity and should be avoided.

Future design must define:

- exact X1 recipient type
- exact recipient byte normalization
- zero / burn recipient policy
- recipient hash preimage hash function

## Double-binding route rules

Theo confirmed that double-binding sourceChainWeightBps and xxxlMintAmount is the correct safety model.

The signed message includes:

- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount

The immutable mint core independently verifies:

- sourceChainWeightBps == hardcoded Stage 1 route weight
- xxxlMintAmount == burnedAmount
- guardian message values do not override immutable route rules

This protects against guardian overreach.

Even if guardians sign a message with a wrong route weight or wrong mint amount, mint core must reject it.

## Replay / malleability / finality / cross-domain risks

Theo confirmed that the main replay and malleability boundaries are covered:

- fixed field order
- no JSON canonicalization
- no optional field omission
- messageType in signed payload
- routeId in signed payload
- mintToken in signed payload
- domainSeparator for cross-environment isolation
- canonicalEventKey based on exact Ethereum log

Theo identified the most important remaining risk:

messageHash encoding must be finalized before implementation.

If domainSeparator or preimage encoding remains ambiguous, independent implementations may derive different messageHash values from the same burn event.

Test vectors are mandatory before implementation.

## sourceBlockHash and finality

Theo recommended making sourceBlockHash mandatory in the signed message.

sourceBlockNumber and sourceBlockHash should be treated as required signed fields, not optional fields.

Reason:

- guardians should sign only finalized canonical Ethereum evidence
- signed message should bind to the observed canonical block
- reorged-out evidence must not remain silently valid
- production design must make finality assumptions explicit

The exact finality rule remains open and must be defined before production.

Potential directions:

- finalized block tag
- conservative confirmation depth
- multiple-provider finality check
- guardian-local finality policy

## X1 mint core immutability mechanism

Theo identified X1 mint core immutability as a blocker before implementation.

The design says immutable mint core, but implementation must define how immutability is enforced.

Open questions:

- how is the X1 mint core deployed?
- can route rules be changed after deployment?
- can mint authority be upgraded?
- can deployer key update the program / contract?
- is deployer authority removed?
- is there a timelock or governance path?
- if governance exists, which parts are mutable and which are not?

Theo's recommendation:

Define the immutability mechanism before Stage 1 code.

If route rules can be changed by governance or admin, the system is not an immutable mint core.

## Atomic processed-burn check and mint

Theo identified atomic processed-burn check-and-mint as a blocker before implementation.

Risk:

Two relayers may submit the same approved message for the same canonicalEventKey.

The X1 mint path must atomically:

1. verify the message and signatures
2. check canonicalEventKey is unprocessed
3. mark canonicalEventKey as processed
4. mint XXXL

The processed registry must not allow a race where two executions pass the unprocessed check before either marks the key.

The exact atomicity model depends on X1 runtime guarantees.

This must be specified before implementation.

## Zero / burn recipient policy

Theo identified zero-address or burn-address recipient handling as an open policy question.

If a user provides a null or burn recipient:

- Ethereum XNTD is burned
- XXXL may be minted to an unusable recipient
- the user may permanently lose the X1-side mint result

Open question:

Should Stage 1 reject zero / burn recipients?

This should be decided before implementation.

## Burn amount min/max policy

Theo identified burn amount min/max as an open policy question.

The current design rejects zero burned amount.

Open question:

Should Stage 1 define minimum or maximum burned amount?

This is not necessarily an implementation blocker, but it should be decided before production.

## Pre-implementation blockers

Theo's blocker list before code:

1. final hash function choice
2. final signature standard
3. final X1 recipient type
4. sourceBlockHash and sourceBlockNumber as mandatory signed fields
5. X1 mint core immutability mechanism
6. atomic processed-burn check-and-mint model
7. exact test vectors after hash / signature / recipient choices

## Production blockers

Theo's production-level blockers:

1. finality rule
2. recipient normalization
3. zero / burn recipient policy
4. burn amount min/max policy
5. independent implementation agreement on exact test vectors

## Current conclusion

Stage 1 Gateway design is ready to move from broad architecture into pre-implementation blocker resolution.

Implementation should still not begin yet.

The next recommended design document is:

docs/gateway/stage-1-gateway-pre-implementation-blockers.md

That document should convert Theo's review into an ordered blocker checklist and proposed decisions.
