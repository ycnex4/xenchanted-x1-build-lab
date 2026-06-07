# XNTD Burn-to-Mint Bridge to X1 design

This document defines the initial design direction for bridging XNTD from Ethereum / source chains into X1.

This is a design document only.

No runtime code is changed.

No contracts are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Core idea

The bridge should use a burn-to-mint model.

Primary route:

    Ethereum XNTD burn -> X1 XNTD mint

The user burns XNTD on Ethereum.

After the burn is finalized and verified, the bridge mints X1 XNTD to the selected X1 recipient.

This is not a lock/wrapped bridge.

There should be no permanent Ethereum escrow treasury holding bridged XNTD.

## Why burn-to-mint

Burn-to-mint is cleaner than lock/wrap for this model.

It avoids a permanent locked-token pool on Ethereum.

It reduces custody risk.

It fits the first-principles framing better:

    source-chain XNTD is destroyed
    X1 XNTD is created from verified burn evidence

The bridge is still an optional infrastructure layer.

It is not part of immutable XC core protocol governance.

Users who do not accept bridge risk can choose not to use the bridge.

## Unified X1 XNTD

The bridge should mint one unified X1 XNTD token.

The bridge should not create separate token classes such as:

- XNTD_ETH
- XNTD_CHAIN_A
- XNTD_CHAIN_B

Instead:

    all accepted source-chain burns mint the same X1 XNTD token

Source-chain differences are reflected only through deterministic mint coefficients and bridge history.

## Source-chain coefficients

Ethereum should be the primary full-weight source.

Initial route:

    Ethereum XNTD burn -> X1 XNTD mint with 100% coefficient

Future routes may support other source chains with reduced coefficients.

Formula:

    x1MintAmount = burnedAmount * sourceChainWeightBps / 10000

Example coefficients:

    Ethereum: 10000 bps
    Source chain A: 5000 bps
    Source chain B: 2500 bps
    Experimental source chain: 1000 bps

The exact future coefficients are not defined in this document.

This document only defines the principle:

    Ethereum first.
    Other chains later.
    Other chains may have reduced source-chain weights.

## Why one X1 token

One unified X1 XNTD is better for:

- user experience
- liquidity
- integrations
- future Build usage
- market simplicity

Users should not need to manage many versions of XNTD on X1.

Origin remains visible through bridge event history, not through different token classes.

## Bridge history fields

Every accepted bridge mint should preserve source history.

A bridge record should include at least:

- sourceChainId
- sourceToken
- sourceBurnTxHash
- sourceBurnEventIndex
- sourceBurnEventId / canonicalEventKey
- sourceSender
- x1Recipient
- burnedAmount
- sourceChainWeightBps
- mintedAmount
- finality policy
- validator signature set / approval proof
- mintedAt / processedAt

One source burn event must map to one X1 mint operation.

Replay must be impossible.

## Ethereum-side burn path

The Ethereum-side burn should not be a meaningless public self-burn.

The burn should have protocol meaning:

    burn XNTD for X1 mint

The burn event should include enough information for the X1 bridge mint:

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

The X1-side bridge mint should mint unified X1 XNTD.

The mint should require verified burn evidence.

A mint instruction/message should include:

- sourceChainId
- sourceBurnTxHash
- sourceBurnEventIndex
- x1Recipient
- burnedAmount
- sourceChainWeightBps
- mintedAmount
- validator approvals / threshold proof

The X1 bridge must reject already processed source burn events.

## Validator / signer model

The bridge should not rely on one server.

Initial target model:

    5 bridge validators
    3-of-5 threshold

The 700+ X1 validators are not the quorum.

They are a future candidate pool for finding the first small group of willing bridge / Build infrastructure operators.

Initial bridge validators may come from:

- X1 validators
- technical community operators
- trusted infrastructure contributors
- project operators

The goal is:

    not one server
    not 700 signatures
    small independent committee first
    expandable validator set later

## Validator responsibilities

Bridge validators verify:

- source burn event exists
- source burn event is finalized
- event matches bridge format
- source burn event was not processed before
- x1Recipient is included
- burnedAmount is correct
- sourceChainWeightBps is the configured value for that source chain
- mintedAmount is correctly derived
- bridge message matches deterministic rules

Bridge validators must not:

- change recipient
- change amount
- manually choose a custom coefficient for one user
- rewrite bridge history
- mint without burn evidence
- act as protocol governance over XC core rules

## Funding model

Bridge infrastructure has real operating costs.

Funding may cover:

- watcher nodes
- bridge validator nodes
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
- bridge service fee
- validator/operator compensation

This document does not define a final fee model.

## Bridge fee options

Potential fee models:

1. no protocol fee initially
2. fixed service fee
3. percentage fee deducted from minted amount
4. source-chain-specific fee
5. operator-funded staging, fee later

Fee design must be a separate milestone.

Any fee must be transparent before the user burns XNTD.

## Relationship to Build

The XNTD bridge is separate from Build actor.

The bridge may later support Build-related flows, but it should not be blocked by Build actor.

The bridge can help X1 users receive X1 XNTD.

Build can later use X1 XNTD for lock, participation, or other mechanics if designed.

Do not mix the first bridge design with Build actor scope.

## Relationship to XC core protocol

The bridge is an optional transport / conversion layer.

It does not change immutable XC core protocol rules.

It does not give bridge operators admin power over XC core.

It does not modify Ethereum-side XC history.

It only observes burns and mints corresponding X1 XNTD according to bridge rules.

## Initial stage

Stage 1 should be Ethereum-only:

    Ethereum XNTD burn -> X1 XNTD mint
    sourceChainWeightBps = 10000

No other chains in the first bridge version.

No reduced coefficients in the first implementation.

Reduced coefficients are a future multi-chain extension.

## Future multi-chain stage

Stage 2 may add other source chains.

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
- user misunderstanding bridge risk

These risks require separate security and incident-response design before production deployment.

## Non-goals

This document does not implement:

- Ethereum burn contract
- X1 mint contract / program
- bridge watcher runtime
- validator signing runtime
- fee model
- staking / slashing
- Build actor
- multi-chain support
- production deployment

## Decision

The preferred bridge direction is:

    XNTD burn on Ethereum
    verified burn event
    threshold validator approval
    unified X1 XNTD mint

The first version should be Ethereum-only with 100% coefficient.

Future source chains may be added with reduced coefficients.

The bridge should mint one unified X1 XNTD token, not multiple origin-specific token classes.
