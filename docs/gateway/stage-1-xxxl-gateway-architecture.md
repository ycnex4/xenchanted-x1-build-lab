# Stage 1 XXXL Gateway architecture

This document defines the Stage 1 architecture boundary for the XNTD-to-XXXL Gateway.

This is a design document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

Stage 1 should define the first concrete gateway architecture before implementation.

The goal is to make the first implementation boundary narrow, reviewable, and safe.

Stage 1 is not the full future system.

Stage 1 is the first production-oriented architecture target for:

    Ethereum XNTD burn -> X1-native XXXL mint

## Core framing

This is not a wrapped bridge.

This is not a transfer of the same token.

This is not XNTD moving from Ethereum to X1.

The Stage 1 model is:

    Ethereum XNTD is burned.
    XXXL is minted on X1.

XXXL is a separate X1-native token.

XXXL has different supply, different utility, and different market dynamics from Ethereum XNTD.

Gateway conversion does not create a price peg between XNTD and XXXL.

There is no reverse direction in Stage 1.

XXXL cannot be converted back to XNTD through the gateway.

## Stage 1 route

Stage 1 supports only one source route:

    Ethereum XNTD -> XXXL

Source route:

    sourceChain = Ethereum
    sourceToken = XNTD
    sourceChainWeightBps = 10000

Mint formula:

    xxxlMintAmount = burnedAmount * 10000 / 10000

Equivalent simplified formula:

    xxxlMintAmount = burnedAmount

This does not mean XXXL is the same asset as XNTD.

It only means that the Stage 1 Ethereum source route has a full-weight conversion coefficient.

## Immutable Stage 1 mint core

The Stage 1 X1 mint core should be immutable or immutable-equivalent after deployment.

It should hardcode or otherwise immutably define:

- Ethereum as the only Stage 1 source chain
- XNTD as the only Stage 1 source token
- source-chain weight: `10000 bps`
- mint formula
- processed burn replay protection
- one source burn event maps to one XXXL mint operation

Gateway guardians must not control XXXL monetary policy.

Gateway guardians must not be able to change the Ethereum route coefficient.

Gateway guardians must not be able to add new source chains in Stage 1.

Gateway guardians must not be able to mint XXXL without valid burn evidence.

## Governance boundary

Stage 1 should separate two kinds of power:

    verification work
    monetary conversion rules

Gateway guardians handle verification work.

The immutable mint core / immutable route rules define monetary conversion rules.

Gateway guardians may verify and approve:

- source burn event exists
- source burn event is finalized
- event matches expected Ethereum XNTD burn format
- x1Recipient is present
- burnedAmount is correct
- xxxlMintAmount is correctly derived
- source burn event was not processed before

Gateway guardians must not decide monetary rules.

## Guardian model

Stage 1 target guardian model:

    5 gateway guardians
    3-of-5 approval threshold

This is a narrow starting model.

The 700+ X1 validators are not the quorum.

They are a future candidate pool for finding willing gateway / Build infrastructure operators.

The first guardian set may be bootstrapped by the project.

This is acceptable for Stage 1 only if disclosed clearly.

A later stage should transition toward more community-selected or independently selected guardians.

## Guardian independence

Gateway guardians should be independent from each other.

No single entity should control more than one guardian seat.

Gateway guardians should be operationally separated from XC core protocol maintainers where possible.

Gateway guardian control must not become hidden control over:

- XC core smart contracts
- Build protocol rules
- token emission schedules
- Build history
- source-chain coefficients
- user balances outside deterministic gateway rules

## Guardian set changes

Guardian set changes must not use the normal mint threshold alone.

Normal mint approval may be:

    3-of-5

Guardian set changes should require stricter approval and delay.

Example direction:

    guardian set change = 4-of-5 + public notice + timelock

The exact rule is not finalized here.

Guardian rotation must be designed before production use.

Guardian key recovery must be designed before production use.

Lost guardian keys must not permanently freeze the gateway.

Lost-key recovery must preserve stricter approval, public notice, and timelock expectations.

## Burn evidence

The Ethereum-side burn event should contain enough information to support deterministic X1 minting.

Required evidence direction:

- sourceChainId
- sourceToken
- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey
- sourceSender
- x1Recipient
- burnedAmount

The exact Ethereum-side function name is not finalized.

Possible direction:

    burnForX1Gateway(x1Recipient, amount)

or:

    burnForX1Bridge(x1Recipient, amount)

The final name should reflect gateway terminology if possible.

## Mint message

A Stage 1 mint message should be deterministic.

It should include at least:

- gatewayVersion
- destinationChainId
- sourceChainId
- sourceToken
- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey
- x1Recipient
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount

Guardian signatures must be over the exact deterministic message.

A signature for one recipient, amount, burn event, or chain must not be reusable for another.

## Replay protection

The X1 mint core must reject already processed source burns.

Replay protection should be based on a canonical event key.

Conceptual key:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Exact key format is not finalized here.

Important rule:

    one accepted source burn event -> one XXXL mint operation

## Reorg safety

Gateway guardians must wait for sufficient Ethereum finality before approving a burn event.

Guardians must not approve a burn immediately after inclusion.

The exact confirmation depth must be finalized before implementation.

Initial guidance:

- low-value testing may use a lower confirmation depth
- production value transfer should use conservative confirmation rules
- high-value burns may require deeper finality, such as 64+ confirmations or a separately defined finality rule

A burn event that disappears because of a reorg must never result in XXXL minting.

## Mint failure recovery

A valid Ethereum burn must not require a second Ethereum burn if the X1 mint transaction fails.

If a guardian-approved X1 mint operation fails because of gas, runtime, network, or program execution issues, the gateway must support deterministic retry using the same burn evidence.

Retry must preserve:

- same source burn event
- same canonicalEventKey
- same x1Recipient
- same burnedAmount
- same sourceChainWeightBps
- same xxxlMintAmount
- same guardian approvals, unless approvals expire by an explicit rule

A failed mint must not mark the source burn as processed unless the X1 mint actually succeeded.

## Relayer model

The relayer should not be trusted.

A relayer only submits an already-approved mint operation to X1.

The X1 mint core must verify:

- deterministic message
- guardian threshold
- replay protection
- route rules
- mint amount

Any relayer should be able to submit a valid approved mint message.

If one relayer fails, another relayer should be able to retry.

## Emergency pause

Stage 1 should define an emergency pause before production use.

The pause should be limited to gateway minting operations.

Emergency pause must not give control over:

- XC core protocol
- Ethereum XNTD
- already-processed burns
- Build history
- arbitrary XXXL minting

The exact pause model is not finalized here.

Emergency pause design should be a separate security milestone.

## Fees

Stage 1 may launch with no gateway fee or with a transparent fee.

Any fee must be disclosed before the user burns XNTD.

Fee rules must not be hidden.

Fee rules must not be changed retroactively for already-submitted burns.

The exact fee model is not finalized here.

## User disclosure

Stage 1 user-facing UX must clearly disclose:

- this is a gateway, not a wrapped bridge
- XNTD is burned on Ethereum
- XXXL is minted on X1
- XXXL is a different asset from XNTD
- there is no price peg
- there is no reverse conversion through the gateway
- burn-to-mint is one-way and irreversible
- mint may require finality waiting time
- gateway risk exists
- Stage 1 guardian trust model exists

## Frontend role

The frontend may provide the user interface for Stage 1.

The frontend should help users:

- connect Ethereum wallet
- view Ethereum XNTD balance
- enter X1 recipient
- preview burn amount
- preview expected XXXL mint amount
- see gateway disclosures
- submit Ethereum burn
- track confirmation / finality status
- track guardian approval status
- track X1 mint status

The frontend must not be the source of truth.

The X1 mint core and gateway verification rules must be the source of truth.

## What Stage 1 does not include

Stage 1 does not include:

- reverse XXXL -> XNTD conversion
- sidechain source routes
- mutable source-chain coefficients
- X1 Forge implementation
- X1 Stake implementation
- Build actor
- full Build program
- BLD marketplace
- public multi-chain expansion
- production slashing mechanics
- final fee model
- full audit completion

## Relationship to X1 Forge / Stake

Stage 1 Gateway provides XXXL as X1-native fuel.

X1 Forge and X1 Stake remain future layers.

Future direction:

    Gateway brings energy into X1.
    XXXL carries that energy.
    X1 Forge transforms liquid XXXL into long-term positions.
    X1 Stake gives those positions slow productive value.
    Build records participation / history / state later.

Stage 1 should not implement X1 Forge or X1 Stake.

## Relationship to Build

Build actor is not required for Stage 1 Gateway.

However, a minimal Build event recorder may be considered in parallel if useful.

Build should not block Stage 1 Gateway.

Stage 1 Gateway should not depend on full Build actor scope.

## Production readiness blockers

Before any production deployment, Stage 1 still requires:

- Ethereum burn contract / function design
- X1 XXXL mint core design
- guardian signature format
- guardian key management model
- guardian rotation / recovery model
- reorg/finality policy
- mint retry policy
- emergency pause design
- fee disclosure design
- frontend disclosure design
- monitoring / alerting
- incident response plan
- audit plan

## Decision

Stage 1 architecture should be narrow:

    Ethereum XNTD burn -> immutable X1 XXXL mint core -> XXXL mint

The Ethereum route should be immutable.

The Ethereum source-chain weight should be `10000 bps`.

Gateway guardians should verify burn evidence and approve deterministic mint messages.

Gateway guardians should not control XXXL monetary policy.

There should be no reverse direction.

Future source chains, X1 Forge, X1 Stake, and Build actor remain separate future stages.
