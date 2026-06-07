# Stage 1 gateway message schema

This document defines the deterministic Stage 1 gateway message schema for X1 XXXL mint approval.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 Gateway requires a deterministic message that guardians can sign and X1 mint verification logic can validate.

The Stage 1 route is:

    Ethereum XNTD burn -> X1-native XXXL mint

The Ethereum burn event is the source evidence.

The gateway message is the deterministic approval object derived from that evidence.

The X1 mint core should accept only messages that match immutable Stage 1 route rules.

## Relationship to Stage 1 architecture

This document refines the message layer described in:

- `docs/gateway/stage-1-xxxl-gateway-architecture.md`
- `docs/gateway/stage-1-xxxl-gateway-implementation-plan.md`
- `docs/gateway/stage-1-ethereum-burn-event-schema.md`

The architecture boundary remains:

    gateway guardians = verification layer
    immutable mint core / route rules = monetary conversion rules

Guardians verify Ethereum burn evidence.

Guardians sign deterministic mint messages.

Guardians must not control XXXL monetary policy.

Guardians must not be able to change the Stage 1 source weight.

Guardians must not be able to mint XXXL without valid Ethereum XNTD burn evidence.

## Stage 1 scope

Stage 1 message schema covers only:

    Ethereum XNTD burn -> X1 XXXL mint approval

Stage 1 message schema does not cover:

- reverse XXXL -> XNTD conversion
- sidechain routes
- mutable source-chain coefficients
- X1 Forge
- X1 Stake
- Build memory / state layer
- BLD issuance
- marketplace logic
- production slashing mechanics

## Message design goals

The message must be:

- deterministic
- replay-protected
- easy to verify
- easy to index
- derived from one canonical Ethereum burn event
- bound to the exact source transaction and log index
- bound to the expected source chain
- bound to the expected source token
- bound to the expected X1 recipient
- bound to the expected XXXL mint amount
- domain-separated from other future routes and message types

The message must not rely on ambiguous off-chain interpretation.

The message must not allow guardians to choose a mint amount independently.

The message must not imply that XXXL is the same asset as Ethereum XNTD.

The message must not imply a price peg between XNTD and XXXL.

## Source event dependency

The message is derived from the preferred Ethereum-side event:

    XntdBurnedForX1Gateway

Preferred event fields:

- sourceSender
- x1RecipientHash
- x1Recipient
- burnedAmount
- sourceChainId
- sourceToken
- sourceNonce

Derived Ethereum log fields:

- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBlockNumber
- sourceBlockHash

The replay key direction remains:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

The exact hash function and binary encoding must be finalized before implementation.

## Proposed message name

Preferred message type:

    X1GatewayMintMessage

Alternative shorter name:

    GatewayMintMessage

The preferred name is more explicit because the message is intended for X1-side XXXL mint approval.

## Proposed message fields

Conceptual schema:

    messageType
    schemaVersion
    routeId
    sourceChainId
    sourceToken
    sourceSender
    sourceBurnTxHash
    sourceBurnEventIndex
    sourceBlockNumber
    sourceBlockHash
    sourceNonce
    canonicalEventKey
    x1Recipient
    x1RecipientHash
    burnedAmount
    sourceChainWeightBps
    xxxlMintAmount
    mintToken
    deadlineOrFinalityBlock
    messageNonce

## Field definitions

### messageType

Constant domain field.

Preferred value:

    X1_GATEWAY_MINT

Purpose:

- separates this message from other future signed objects
- prevents signature reuse across unrelated systems
- makes signed payloads human-readable in tooling

### schemaVersion

Message schema version.

Preferred Stage 1 value:

    1

Purpose:

- allows future schema evolution
- prevents old signatures from being interpreted as newer formats

Stage 1 should not use schema upgrades in production without explicit review.

### routeId

Immutable route identifier.

Preferred Stage 1 value:

    ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1

Purpose:

- binds the message to the Ethereum Stage 1 route
- separates this route from future sidechain or alternative source routes

The route id should be treated as part of the monetary rule boundary.

Guardians must not choose arbitrary route ids.

### sourceChainId

Source chain identifier.

Stage 1 value:

    1

Meaning:

    Ethereum mainnet

Guardians must reject messages where `sourceChainId` is not Ethereum mainnet.

### sourceToken

Ethereum source token address.

Stage 1 value:

    expected Ethereum XNTD token address

Purpose:

- binds the message to the correct burned token
- prevents using unrelated burns as XXXL mint evidence

Guardians must reject messages where `sourceToken` is not the expected XNTD token.

### sourceSender

Ethereum address that initiated the burn.

Derived from the Ethereum burn event.

Purpose:

- supports user display
- supports auditing
- supports future analytics and history

`sourceSender` is not the X1 recipient unless the user chooses an equivalent recipient mapping.

### sourceBurnTxHash

Ethereum transaction hash containing the burn event.

Purpose:

- binds the message to a specific source transaction
- supports explorer lookup
- contributes to replay protection through `canonicalEventKey`

### sourceBurnEventIndex

Ethereum log index for the burn event inside the transaction receipt.

Purpose:

- distinguishes multiple burn events in one transaction
- contributes to replay protection through `canonicalEventKey`

### sourceBlockNumber

Ethereum block number containing the burn event.

Purpose:

- supports finality checks
- supports indexing
- supports watcher and frontend state display

This field is not sufficient for replay protection by itself.

### sourceBlockHash

Ethereum block hash containing the burn event.

Purpose:

- binds the message to the observed canonical block
- helps detect reorged-out evidence
- supports guardian finality checks

Guardians must reject evidence that is not in the canonical finalized chain.

### sourceNonce

Ethereum-side nonce emitted by the burn event.

Purpose:

- improves indexing
- improves user display
- helps detect local event ordering issues

`sourceNonce` should not be the primary replay key.

Replay protection should bind to the exact Ethereum log through `canonicalEventKey`.

### canonicalEventKey

Canonical replay key for the source burn event.

Direction:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Purpose:

- uniquely identifies the burn event evidence
- drives X1 processed burn registry checks
- prevents duplicate minting for the same Ethereum burn log

Open encoding question:

- exact binary encoding must be finalized before implementation

Preferred direction:

- use fixed-width canonical encoding
- avoid string concatenation
- include explicit domain separation if the target runtime requires it

### x1Recipient

Recipient on X1 that should receive minted XXXL.

Derived from the Ethereum burn event.

Purpose:

- binds the mint to the user-provided X1 recipient
- prevents guardians or relayers from redirecting the mint

Open type question:

- `bytes`
- `bytes32`
- X1-native address type if available

Stage 1 should avoid assuming an address format before X1 runtime constraints are confirmed.

### x1RecipientHash

Hash of the X1 recipient payload.

Purpose:

- provides fixed-width indexed identity
- helps indexers and watchers search recipient activity
- provides compact display / comparison key

The hash must be derived from the exact `x1Recipient` bytes.

### burnedAmount

Amount of Ethereum XNTD burned.

Derived from the Ethereum burn event.

Purpose:

- source economic input for deterministic XXXL mint amount

Guardians must reject zero burned amount.

### sourceChainWeightBps

Route weight in basis points.

Stage 1 value:

    10000

Meaning:

    full-weight conversion for Ethereum Stage 1 route

This does not mean XXXL is the same asset as XNTD.

This does not establish a price peg.

This only means the Ethereum Stage 1 route uses full source weight.

The X1 mint core should treat this as an immutable route rule.

Guardians must not be able to change this value.

### xxxlMintAmount

Amount of XXXL to mint on X1.

Stage 1 rule:

    xxxlMintAmount = burnedAmount

Because Stage 1 source weight is `10000 bps`.

The value must be derived deterministically.

Guardians must not choose this amount manually.

Relayers must not modify this amount.

X1 mint verification must reject messages where `xxxlMintAmount` does not match the immutable Stage 1 route rule.

### mintToken

X1 token being minted.

Stage 1 value:

    XXXL

Purpose:

- separates XXXL mint messages from future X1 assets
- prevents signature reuse for another token

The exact identifier depends on X1 runtime conventions.

### deadlineOrFinalityBlock

Optional safety field.

Potential meanings:

- latest X1 block where this message may be submitted
- Ethereum finality reference used by guardians
- expiration boundary for guardian signatures

Open question:

- whether Stage 1 should include expiration at the message level

If included, it must not allow guardians to change monetary policy.

If omitted, replay protection must still be fully enforced through processed `canonicalEventKey`.

### messageNonce

Optional message-level nonce.

Potential use:

- signature set tracking
- relayer deduplication
- tooling convenience

This should not replace `canonicalEventKey`.

The source burn event remains the canonical replay anchor.

## Deterministic derivation

Given a finalized Ethereum burn event, the deterministic derivation is:

    sourceSender           = event.sourceSender
    x1Recipient           = event.x1Recipient
    x1RecipientHash       = hash(event.x1Recipient)
    burnedAmount          = event.burnedAmount
    sourceChainId         = event.sourceChainId
    sourceToken           = event.sourceToken
    sourceNonce           = event.sourceNonce
    sourceBurnTxHash      = receipt.transactionHash
    sourceBurnEventIndex  = log.logIndex
    sourceBlockNumber     = receipt.blockNumber
    sourceBlockHash       = receipt.blockHash
    canonicalEventKey     = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)
    sourceChainWeightBps  = 10000
    xxxlMintAmount        = burnedAmount
    routeId               = ETHEREUM_XNTD_TO_X1_XXXL_STAGE_1
    messageType           = X1_GATEWAY_MINT
    schemaVersion         = 1
    mintToken             = XXXL

Any participant deriving the message from the same finalized Ethereum log should produce the same payload.

## Guardian signing responsibility

Guardians should sign the message only after verifying:

- the event name matches expected Stage 1 event
- source chain is Ethereum mainnet
- source token is expected XNTD token
- burn transaction succeeded
- event exists in canonical finalized block
- source block hash matches canonical chain data
- `x1Recipient` is present and non-empty
- `x1RecipientHash` matches `x1Recipient`
- `burnedAmount > 0`
- `canonicalEventKey` is derived correctly
- source burn has not already been processed on X1
- `sourceChainWeightBps == 10000`
- `xxxlMintAmount == burnedAmount`
- message type, schema version, route id, and mint token are expected

Guardians must reject:

- wrong-chain evidence
- wrong-token evidence
- failed transactions
- unfinalized evidence
- reorged-out evidence
- zero-amount burns
- empty recipients
- malformed recipients
- malformed message payloads
- duplicate evidence
- incomplete evidence
- messages with guardian-selected mint amounts
- messages with mutable or unexpected route weights

## Relayer responsibility

The relayer submits signed guardian approvals to X1.

The relayer must not be trusted to define monetary values.

The relayer may transport:

- message payload
- guardian signatures
- Ethereum evidence references
- optional user display metadata

The relayer must not be able to:

- change `x1Recipient`
- change `burnedAmount`
- change `xxxlMintAmount`
- change `sourceChainWeightBps`
- change `canonicalEventKey`
- reuse signatures for a different event
- mint without sufficient guardian approval

## X1 verification responsibility

The X1-side verification / mint path should check:

- message type is expected
- schema version is supported
- route id is expected
- source chain id is Ethereum mainnet
- source token is expected XNTD token
- mint token is XXXL
- source chain weight is the immutable Stage 1 value
- XXXL mint amount is derived correctly
- canonical event key is not already processed
- guardian signature threshold is satisfied
- signatures are over the exact canonical message payload
- recipient is non-empty and valid for the chosen X1 format

Only after successful verification should the X1 mint path:

1. mark `canonicalEventKey` as processed
2. mint `xxxlMintAmount` XXXL to `x1Recipient`

The processed flag should be written before or atomically with mint completion according to the target X1 runtime safety model.

## Frontend state mapping

The frontend can show gateway state as:

1. Burn submitted
2. Burn confirmed
3. Burn finalized
4. Guardian approval pending
5. Guardian approved
6. Relayer submitted
7. XXXL minted
8. Already processed / duplicate
9. Rejected evidence

Suggested user-facing data:

- source Ethereum transaction
- source sender
- X1 recipient
- burned XNTD amount
- expected XXXL mint amount
- finality status
- guardian approval status
- X1 mint transaction / execution reference
- processed canonical event key, shortened for display

The frontend must not present XXXL as wrapped XNTD.

The frontend must not imply a price peg.

The frontend should explain that Stage 1 uses full-weight conversion for the Ethereum route.

## Security notes

The gateway message is a monetary approval object.

Small schema ambiguity can become a minting vulnerability.

Before implementation, the following must be reviewed carefully:

- canonical encoding
- domain separation
- hash function
- signature format
- guardian threshold
- finality rule
- replay registry behavior
- duplicate log handling
- reorg handling
- recipient validation
- exact X1 mint execution order
- failure recovery

Wrapped-bridge assumptions are not sufficient.

This gateway is a burn-to-mint conversion route into X1-native fuel.

Independent security analysis is required before production.

## Open questions before implementation

1. What exact binary encoding should define the canonical signed payload?
2. Which hash function should be used for `canonicalEventKey` on X1?
3. Which signature standard should guardians use?
4. Should signatures be over the full message or a typed message hash?
5. Should X1 store the full message or only processed `canonicalEventKey` plus mint record?
6. Should `deadlineOrFinalityBlock` be mandatory or omitted in Stage 1?
7. Should `messageNonce` exist, or is `canonicalEventKey` sufficient?
8. What exact X1 recipient type should be used?
9. What minimum recipient validation can Ethereum enforce before X1 format is final?
10. What finality depth or finalized-block rule should guardians use?
11. Should guardian signatures include signer set epoch / version?
12. Should mint verification require evidence metadata or only signed messages?
13. How should frontend display rejected or reorged evidence?
14. How should relayer retries be represented without creating duplicate risk?

## Current preferred direction

Preferred Stage 1 direction:

- derive one canonical message from one finalized Ethereum burn log
- use transaction hash + log index as the replay anchor
- use `canonicalEventKey` as the processed burn registry key
- keep Ethereum Stage 1 source weight immutable at `10000 bps`
- derive `xxxlMintAmount` from route rules, not guardian discretion
- sign a domain-separated X1 gateway mint message
- keep guardians in the verification layer
- keep monetary conversion rules in immutable X1 mint logic

Implementation should not begin until this schema, encoding, signature format, and X1 recipient type are reviewed.
