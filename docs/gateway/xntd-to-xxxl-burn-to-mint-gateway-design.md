# XNTD-to-XXXL Burn-to-Mint Gateway design

This document defines the initial design direction for converting source-chain XNTD burns into X1-native XXXL.

This is a design document only.

No runtime code is changed.

No contracts are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Core idea

The gateway should use a burn-to-mint model.

Primary route:

    Ethereum XNTD burn -> X1 XXXL mint

The user burns XNTD on Ethereum.

After the burn is finalized and verified, the gateway mints XXXL to the selected X1 recipient.

This is not a lock/wrapped bridge and not a transfer of the same token.

There should be no permanent Ethereum escrow treasury holding source-chain XNTD.

## Why burn-to-mint

Burn-to-mint is cleaner than lock/wrap for this model.

It avoids a permanent locked-token pool on Ethereum.

It reduces custody risk.

It fits the first-principles framing better:

    source-chain XNTD is destroyed
    XXXL is created as a separate X1-native token from verified burn evidence

XXXL has different supply, different utility, and different market dynamics from Ethereum XNTD.

Gateway conversion does not create a price peg between XNTD and XXXL.

There is no reverse direction in this design.

XXXL cannot be converted back to XNTD through the gateway.

The gateway is still an optional infrastructure layer.

It is not part of immutable XC core protocol governance.

Users who do not accept gateway risk can choose not to use the gateway.

Gateway mints are one-way and irreversible by design.

Burn-to-mint has no undo function.

Once XNTD is burned on a source chain, the gateway must either mint the corresponding XXXL or keep a deterministic retry path available until the valid mint succeeds.

Gateway security must meet production-grade verification before any real value moves.

## Unified XXXL

The gateway should mint one unified X1-native XXXL token.

The gateway should not create separate token classes such as:

- XXXL_ETH
- XXXL_CHAIN_A
- XXXL_CHAIN_B

Instead:

    all accepted source-chain burns mint the same XXXL token

Source-chain differences are reflected only through deterministic mint coefficients and gateway history.

## Source-chain coefficients

Ethereum should be the primary full-weight source.

Initial route:

    Ethereum XNTD burn -> X1 XXXL mint with 100% coefficient

Future routes may support other source chains with reduced coefficients.

Formula:

    xxxlMintAmount = burnedAmount * sourceChainWeightBps / 10000

Example coefficients:

    Ethereum: 10000 bps
    Source chain A: 5000 bps
    Source chain B: 2500 bps
    Experimental source chain: 1000 bps

The exact future coefficients are not defined in this document.

## Immutable Stage 1 XXXL mint core

For Stage 1, the Ethereum route should be immutable in the X1 mint core.

The Stage 1 X1 mint core should hardcode or otherwise immutably define:

- source chain: Ethereum
- source token: XNTD
- source-chain weight: 10000 bps
- mint formula: `xxxlMintAmount = burnedAmount * 10000 / 10000`
- processed burn replay protection
- one accepted source burn event -> one XXXL mint operation

Gateway guardians must not control XXXL monetary policy.

Gateway guardians should only verify burn evidence and approve deterministic mint messages.

No guardian vote should be able to change the Ethereum Stage 1 coefficient.

No guardian vote should be able to mint XXXL without valid burn evidence.

Future source routes should be added through separately reviewed source adapters or route definitions, not by mutating the Ethereum route.

Existing route rules must not be changed retroactively for already-processed burns.

## Coefficient governance principle

Source-chain coefficients are monetary conversion parameters.

They must not be changed casually or silently.

Future coefficient changes should require:

- gateway guardian supermajority
- public risk assessment
- public notice period
- non-retroactive application

Coefficients must not be changed retroactively for already-processed burns.

Gateway guardians must not choose custom coefficients for individual users or individual burns.

This document only defines the principle:

    Ethereum first.
    Other chains later.
    Other chains may have reduced source-chain weights.

## Why one X1 token

One unified XXXL token is better for:

- user experience
- liquidity
- integrations
- future Build usage
- market simplicity

Users should not need to manage many versions of XXXL on X1.

Origin remains visible through gateway event history, not through different token classes.

## Gateway history fields

Every accepted gateway mint should preserve source history.

A gateway record should include at least:

- sourceChainId
- sourceToken
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBurnEventId / canonicalEventKey
- sourceSender
- x1Recipient
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount
- finality policy
- guardian signature set / approval proof
- mintedAt / processedAt

One source burn event must map to one X1 mint operation.

Replay must be impossible.

## Ethereum-side burn path

The Ethereum-side burn should not be a meaningless public self-burn.

The burn should have protocol meaning:

    burn XNTD for XXXL mint

The burn event should include enough information for the X1 gateway mint:

- sender
- x1Recipient
- amount
- source chain id
- bridge nonce or canonical burn id

The exact Ethereum-side function name is not defined here.

Potential direction:

    burnForX1Bridge(x1Recipient, amount)

or similar.

## X1-side mint path

The X1-side gateway mint should mint unified XXXL.

The mint should require verified burn evidence.

A mint instruction/message should include:

- sourceChainId
- sourceBurnTxHash
- sourceBurnEventIndex
- x1Recipient
- burnedAmount
- sourceChainWeightBps
- xxxlMintAmount
- guardian approvals / threshold proof

The X1 gateway must reject already processed source burn events.

## Reorg safety policy

Gateway guardians must wait for sufficient source-chain finality before approving a burn event.

For Ethereum, guardians must not approve a burn immediately after inclusion.

The exact confirmation depth must be defined before implementation.

Initial guidance:

- low-value testing may use a lower confirmation depth
- production value transfer should use a conservative confirmation policy
- high-value burns may require deeper finality, such as 64+ confirmations or a separately defined finality rule

A burn event that disappears because of a reorg must never result in XXXL minting.

## Mint failure recovery

A valid burn must not require a second source-chain burn if the X1 mint transaction fails.

If a guardian-approved X1 mint operation fails because of gas, runtime, network, or program execution issues, the gateway must support deterministic retry using the same burn evidence.

Retry rules must preserve:

- same source burn event
- same x1Recipient
- same burnedAmount
- same sourceChainWeightBps
- same xxxlMintAmount
- same canonicalEventKey

A failed mint must not mark the source burn as processed unless the X1 mint actually succeeded.

## Guardian / signer model

The gateway should not rely on one server.

Initial target model:

    5 gateway guardians
    3-of-5 threshold

The 700+ X1 validators are not the quorum.

They are a future candidate pool for finding the first small group of willing gateway / Build infrastructure operators.

The first guardian set may be bootstrapped by the project.

This is acceptable for Stage 1 only if the trust model is disclosed clearly.

A later stage should transition toward more community-selected or independently selected guardians.

Initial gateway guardians may come from:

- X1 validators
- technical community operators
- trusted infrastructure contributors
- project operators

The goal is:

    not one server
    not 700 signatures
    small independent committee first
    expandable validator set later

## Guardian independence and set changes

Gateway guardians should be independent from each other.

No single entity should control more than one gateway guardian seat.

Gateway guardians should also be operationally separated from XC core protocol maintainers where possible.

At minimum, gateway guardian control must not become hidden control over:

- XC core smart contracts
- Build protocol rules
- token emission schedules
- Build history
- source-chain coefficients
- user balances outside deterministic gateway rules

Guardian set changes must not be controlled by the normal mint threshold alone.

A future guardian set update should require:

- stricter approval than normal mint approval
- public notice
- timelock before activation
- clear emergency procedure

Example direction:

    normal mint approval: 3-of-5
    guardian set change: 4-of-5 plus timelock

The exact rule is not finalized in this document, but guardian rotation must be designed before Stage 1 production deployment.

Guardian key recovery and rotation must also be defined before production use.

If guardian keys are lost, the gateway should not become permanently stuck.

Lost-key recovery must still preserve the stricter guardian-set-change rule, public notice, and timelock expectations.

## Guardian responsibilities

Gateway guardians verify:

- source burn event exists
- source burn event is finalized
- event matches bridge format
- source burn event was not processed before
- x1Recipient is included
- burnedAmount is correct
- sourceChainWeightBps is the configured value for that source chain
- xxxlMintAmount is correctly derived
- gateway message matches deterministic rules

Gateway guardians must not:

- change recipient
- change amount
- manually choose a custom coefficient for one user
- rewrite gateway history
- mint without burn evidence
- act as protocol governance over XC core rules

## Funding model

Gateway infrastructure has real operating costs.

Funding may cover:

- watcher nodes
- gateway guardian nodes
- RPC providers
- monitoring
- backups
- API / dashboard infrastructure
- incident response operations

Funding must not create hidden control over XC core protocol or Build history.

Principle:

    infrastructure funding is service-layer funding
    not protocol ownership

Possible future funding mechanisms:

- voluntary community support
- project sponsorship
- hosted API fees
- integration fees
- gateway service fee
- guardian/operator compensation

This document does not define a final fee model.

## Gateway fee options

Potential fee models:

1. no protocol fee initially
2. fixed service fee
3. percentage fee deducted from minted amount
4. source-chain-specific fee
5. operator-funded staging, fee later

Fee design must be a separate milestone.

Any fee must be transparent before the user burns XNTD.

## Relationship to Build

The XNTD-to-XXXL gateway is separate from Build actor.

The gateway may later support Build-related flows, but it should not be blocked by Build actor.

The gateway can help X1 users receive XXXL.

Build can later use XXXL for lock, participation, or other mechanics if designed.

Do not mix the first bridge design with Build actor scope.

## Relationship to XC core protocol

The gateway is an optional burn-to-mint conversion layer.

It does not change immutable XC core protocol rules.

It does not give gateway operators admin power over XC core.

It does not modify Ethereum-side XC history.

It only observes source-chain XNTD burns and mints corresponding XXXL according to gateway rules.

## Initial stage

Stage 1 should be Ethereum-only:

    Ethereum XNTD burn -> X1 XXXL mint
    sourceChainWeightBps = 10000

No other chains in the first gateway version.

No reduced coefficients in the first implementation.

Reduced coefficients are a future multi-chain extension.

## Future multi-chain stage

Stage 2 may add other source chains.

Future source-chain support should prefer separate immutable source adapters or route definitions.

A source adapter / route should define:

- source chain
- source token
- burn event format
- source-chain weight
- mint calculation
- finality policy
- replay key format

Changing coefficients through a mutable global map is not the preferred model.

If a future source route needs different rules, a new reviewed route / adapter should be added instead of retroactively changing old rules.

Each source chain must have:

- chain id
- token address
- burn event format
- finality policy
- sourceChainWeightBps
- risk assessment
- watcher support
- validator verification support

Other chains should have reduced coefficients unless separately justified.

## Risks

Main risks:

- validator collusion
- signer key compromise
- wrong source-chain coefficient
- replay bug
- bad finality assumption
- wrong recipient parsing
- RPC inconsistency
- incident response delay
- user misunderstanding gateway risk
- user misunderstanding XXXL as pegged or reversible XNTD
- guardian control over monetary policy if mint rules are not immutable
- guardian key loss without recovery / rotation path

These risks require separate security and incident-response design before production deployment.

Required gateway-risk items before implementation:

- reorg safety policy
- mint failure recovery
- coefficient governance rule
- guardian independence rule
- guardian set update timelock
- emergency pause design
- transparent fee schedule
- audit requirement
- explicit no-reverse-direction UX disclosure
- Stage 1 immutable mint core / immutable Ethereum route rules

Items that may remain separate implementation/security milestones:

- exact emergency pause mechanics
- economic bond / slashing
- final fee model
- full audit process

## Non-goals

This document does not implement:

- Ethereum burn contract
- X1 mint contract / program
- gateway watcher runtime
- guardian signing runtime
- fee model
- staking / slashing
- Build actor
- multi-chain support
- production deployment

## Decision

The preferred gateway direction is:

    XNTD burn on Ethereum
    verified burn event
    threshold guardian approval
    unified XXXL mint

The first version should be Ethereum-only with 100% coefficient.

Future source chains may be added with reduced coefficients.

The gateway should mint one unified XXXL token, not multiple origin-specific token classes.
