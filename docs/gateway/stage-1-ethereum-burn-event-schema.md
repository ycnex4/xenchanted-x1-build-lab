# Stage 1 Ethereum burn event schema

This document defines the Stage 1 Ethereum-side burn event schema for the XNTD-to-XXXL Gateway.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 Gateway requires a deterministic Ethereum burn event.

That event becomes the source evidence for minting XXXL on X1.

The Stage 1 route is:

    Ethereum XNTD burn -> X1-native XXXL mint

This document defines what the Ethereum-side burn event should contain and how downstream gateway components should interpret it.

## Relationship to Stage 1 architecture

This document refines the Ethereum-side part of:

- `docs/gateway/stage-1-xxxl-gateway-architecture.md`
- `docs/gateway/stage-1-xxxl-gateway-implementation-plan.md`

The Stage 1 architecture boundary remains:

    Ethereum XNTD burn
    -> immutable X1 XXXL mint core
    -> XXXL mint

The Ethereum burn event is only the source evidence.

It does not mint XXXL by itself.

It does not authorize arbitrary minting.

It does not create a wrapped token.

## Core requirements

The burn event must allow guardians and X1 mint logic to verify:

- which source chain produced the burn
- which source token was burned
- who initiated the burn
- which X1 recipient should receive XXXL
- how much XNTD was burned
- which exact Ethereum transaction and log created the evidence
- whether the same burn was already processed
- what deterministic XXXL amount should be minted

The event must be easy to index.

The event must be deterministic.

The event must not require off-chain interpretation beyond the defined schema.

## Proposed Ethereum function

Preferred function name:

    burnForX1Gateway(x1Recipient, amount)

Conceptual signature:

    burnForX1Gateway(bytes x1Recipient, uint256 amount)

Alternative if X1 recipient has a fixed address format:

    burnForX1Gateway(bytes32 x1Recipient, uint256 amount)

The final recipient type is not decided in this document.

The name should use gateway terminology, not bridge terminology.

## Function behavior

The burn function should:

1. receive an X1 recipient identifier
2. receive an XNTD amount
3. validate amount > 0
4. burn XNTD from the caller or from an approved balance
5. emit the Stage 1 gateway burn event
6. not lock funds in escrow
7. not mint anything on Ethereum
8. not create wrapped XNTD
9. not create a reverse redemption claim

## One transaction vs approve + burn

Open question:

    Should the user need ERC-20 approve + burnForX1Gateway?

Preferred UX direction:

    one user transaction if technically possible

However, if the current XNTD architecture requires allowance-based burn from a separate gateway contract, then the Stage 1 UX may require:

    approve -> burnForX1Gateway

This must be reviewed before implementation.

If approve is required, the frontend must clearly explain the two-step flow and must not hide spending-cap risk from users.

## Proposed event

Preferred event name:

    XntdBurnedForX1Gateway

Conceptual event:

    event XntdBurnedForX1Gateway(
        address indexed sourceSender,
        bytes indexed x1RecipientHash,
        bytes x1Recipient,
        uint256 burnedAmount,
        uint256 sourceChainId,
        address indexed sourceToken,
        uint256 sourceNonce
    );

## Event fields

### sourceSender

Ethereum address that initiated the burn.

Type:

    address

Indexing:

    indexed

Purpose:

- identifies Ethereum-side user
- helps wallets and explorers filter burns
- helps support/debugging

### x1RecipientHash

Hash of the X1 recipient identifier.

Type:

    bytes32

Indexing:

    indexed

Purpose:

- allows indexed filtering by recipient hash
- avoids depending on dynamic indexed bytes behavior
- helps watchers locate events for a recipient

Derived as:

    x1RecipientHash = keccak256(x1Recipient)

### x1Recipient

Raw X1 recipient identifier.

Type:

    bytes

Indexing:

    not indexed

Purpose:

- provides exact destination recipient data
- allows X1-side mint message to include the destination
- avoids lossy conversion

The exact X1 recipient format is not finalized here.

### burnedAmount

Amount of Ethereum XNTD burned.

Type:

    uint256

Purpose:

- source amount for XXXL mint calculation
- Stage 1 uses full-weight coefficient

Stage 1 formula:

    xxxlMintAmount = burnedAmount

Equivalent full formula:

    xxxlMintAmount = burnedAmount * 10000 / 10000

This does not mean XXXL is the same asset as XNTD.

It only means Ethereum Stage 1 route uses a 10000 bps source weight.

### sourceChainId

Ethereum chain id.

Type:

    uint256

Expected Stage 1 value:

    1

Purpose:

- prevents cross-chain replay
- makes evidence self-describing
- supports future multi-source route structure

For Ethereum mainnet Stage 1, this must be Ethereum mainnet chain id.

### sourceToken

Ethereum XNTD token address.

Type:

    address

Indexing:

    indexed

Purpose:

- binds the burn evidence to the expected source token
- prevents accepting burns from the wrong token
- supports future source adapter patterns

For Stage 1, the X1 mint core / route definition should accept only the immutable Ethereum XNTD source token.

### sourceNonce

Optional monotonic nonce emitted by the burn contract.

Type:

    uint256

Purpose:

- helps event uniqueness
- helps off-chain indexing
- gives a simple human-readable burn id

This nonce is helpful but not strictly required if canonicalEventKey uses transaction hash and log index.

## Derived fields

The event itself does not need to emit every downstream field.

Watchers and guardians can derive:

- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey
- xxxlMintAmount

### sourceBurnTxHash

Ethereum transaction hash containing the burn event.

Derived from Ethereum log metadata.

### sourceBurnEventIndex

Log index of the burn event inside the Ethereum transaction receipt.

Derived from Ethereum log metadata.

### canonicalEventKey

Canonical replay key for X1 processing.

Preferred conceptual form:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Purpose:

- uniquely identifies one Ethereum burn event
- prevents replay
- avoids relying only on user-provided nonce
- binds evidence to exact source chain and token

The exact hash function / encoding must match the X1 mint core implementation.

## Why not rely only on nonce

A sourceNonce is useful for indexing and user display.

However, replay protection should not rely only on a mutable or contract-local nonce.

The canonical event key should include Ethereum log identity:

    sourceBurnTxHash
    sourceBurnEventIndex

This ties the X1 mint to the exact emitted Ethereum log.

## Recipient validation

The burn function should validate that x1Recipient is not empty.

Minimum validation:

    x1Recipient.length > 0

Open questions:

- What is the canonical X1 address format?
- Should the Ethereum contract validate X1 address length?
- Should invalid X1 recipients be rejected on Ethereum?
- Should X1 recipient validation happen only in frontend and X1 mint core?
- Should bytes32 be used instead of bytes?

Conservative direction:

    Ethereum should reject obviously empty recipients.
    X1 mint core should validate final recipient format if possible.
    Frontend should prevent malformed recipients before burn.

## Amount validation

The burn function must reject:

    amount == 0

Open questions:

- Should there be a minimum burn amount?
- Should there be a maximum burn amount per transaction?
- Should limits be enforced on Ethereum, X1, frontend, or all?
- Should high-value burns require deeper finality policy?

Stage 1 architecture currently does not define min/max burn amounts.

## Event acceptance by guardians

Guardians may accept an Ethereum burn event only if:

- event name matches expected Stage 1 event
- source chain is Ethereum mainnet
- source token is the expected XNTD token
- burn transaction succeeded
- event exists in a canonical finalized block
- sourceSender is present
- x1Recipient is present and non-empty
- burnedAmount > 0
- sourceChainId matches Ethereum mainnet
- canonicalEventKey is derived correctly
- source burn has not already been processed on X1
- expected xxxlMintAmount is derived correctly

## Event rejection by guardians

Guardians must reject:

- events from the wrong chain
- events from the wrong token
- failed transactions
- unfinalized / insufficiently confirmed events
- reorged-out events
- zero amount burns
- empty recipient burns
- malformed recipient data
- duplicate canonicalEventKey
- logs that do not match the expected event schema
- manually edited or incomplete evidence

## X1 mint message mapping

The Ethereum burn event should map into the deterministic X1 mint message.

Mapping direction:

    sourceSender -> sourceSender
    x1Recipient -> x1Recipient
    burnedAmount -> burnedAmount
    sourceChainId -> sourceChainId
    sourceToken -> sourceToken
    tx hash -> sourceBurnTxHash
    log index -> sourceBurnEventIndex
    canonicalEventKey -> canonicalEventKey
    sourceChainWeightBps -> 10000
    xxxlMintAmount -> burnedAmount

The X1 mint core must verify the final deterministic message and threshold guardian approvals.

## Finality policy

The burn event must not be approved immediately after inclusion.

Guardians must wait for the Stage 1 finality policy.

The exact finality rule is not defined here.

Initial guidance:

- testing may use a lower confirmation count
- production should use conservative finality
- high-value burns may require deeper confirmation rules

A burn event that disappears due to reorg must never mint XXXL.

## User-facing disclosure

Before a user calls the burn function, the frontend must disclose:

- XNTD will be burned on Ethereum
- XXXL will be minted on X1 only after verification
- this is not a wrapped bridge
- XXXL is a different asset from XNTD
- there is no price peg
- there is no reverse conversion through the gateway
- burn-to-mint is one-way and irreversible
- finality waiting is required
- gateway guardian trust model exists
- mint may be delayed if guardians / relayer / X1 submission have issues

## Non-goals

This document does not define:

- final Ethereum contract code
- final X1 mint program code
- final X1 recipient format
- final guardian signature format
- final relayer implementation
- final frontend implementation
- final finality policy
- final emergency pause model
- production deployment

## Open questions

Open questions before implementation:

1. Should the burn function live in XNTD or a dedicated gateway burn contract?
2. Can the burn path be no-admin / immutable?
3. Does Stage 1 require approve + burn, or can it be one transaction?
4. What exact type should x1Recipient use?
5. Should sourceNonce be included?
6. What minimum recipient validation should Ethereum enforce?
7. Should burn amount min/max exist?
8. What finality rule should guardians use?
9. What exact canonicalEventKey encoding should X1 use?
10. How should the frontend show pending / finalized / approved / minted states?

## Decision

The Stage 1 Ethereum burn event should be deterministic, self-describing, and easy to index.

Preferred function direction:

    burnForX1Gateway(x1Recipient, amount)

Preferred event direction:

    XntdBurnedForX1Gateway

The event should include:

- sourceSender
- x1RecipientHash
- x1Recipient
- burnedAmount
- sourceChainId
- sourceToken
- sourceNonce

The canonical replay key should be derived from Ethereum log identity:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

This schema is not implementation approval.

It is the first technical design layer for Stage 1 review.
