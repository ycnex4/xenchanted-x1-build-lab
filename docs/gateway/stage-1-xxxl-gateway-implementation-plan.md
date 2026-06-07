# Stage 1 XXXL Gateway implementation plan

This document defines the implementation planning sequence for Stage 1 of the XNTD-to-XXXL Gateway.

This is a planning document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 architecture is already defined in:

- `docs/gateway/stage-1-xxxl-gateway-architecture.md`

This document translates that architecture into an implementation plan.

It does not start implementation.

It defines the future work order, components, safety checks, and review gates.

## Stage 1 scope

Stage 1 implements only:

    Ethereum XNTD burn -> X1-native XXXL mint

Stage 1 does not implement:

- reverse XXXL -> XNTD conversion
- sidechain source routes
- mutable source-chain coefficients
- X1 Forge
- X1 Stake
- Build actor
- full Build program
- BLD marketplace
- production slashing mechanics
- multi-chain expansion

## Core implementation principle

Stage 1 must preserve the architecture boundary:

    immutable mint rules
    governed / operational verification

Meaning:

    XXXL mint core defines monetary conversion rules.
    gateway guardians only verify burn evidence and approve deterministic mint messages.

Guardians must not control XXXL monetary policy.

Guardians must not be able to change the Ethereum source weight.

Guardians must not be able to mint XXXL without valid Ethereum burn evidence.

## Implementation components

Stage 1 requires the following components:

1. Ethereum burn contract / function
2. Ethereum burn event format
3. X1 XXXL token / mint core
4. X1 processed burn registry
5. deterministic gateway message format
6. guardian signing format
7. guardian verification runtime
8. relayer runtime
9. read-only watcher / indexer
10. frontend gateway flow
11. monitoring and incident response
12. staging test environment
13. production readiness checklist

## Component 1: Ethereum burn function

The Ethereum-side burn function should create the source event for gateway minting.

Possible direction:

    burnForX1Gateway(x1Recipient, amount)

The final name should use gateway terminology if possible.

The burn function must:

- burn Ethereum XNTD
- emit a deterministic event
- include the X1 recipient
- include burned amount
- include source sender
- be easy for watchers to index
- not require users to paste secrets anywhere
- not create escrow custody

Open design questions:

- Should this be added to existing XNTD logic or use a dedicated burn gateway contract?
- Can the burn path be immutable / no-admin?
- How is XNTD burn authorization handled?
- Does the user need approve + burn, or can this be one transaction?

## Component 2: Ethereum burn event format

The burn event should contain enough data to deterministically create an X1 mint message.

Required fields direction:

- source sender
- X1 recipient
- burned amount
- source chain id
- source token
- nonce or canonical burn id

Derived fields:

- sourceBurnTxHash
- sourceBurnEventIndex
- canonicalEventKey

Potential canonical key:

    canonicalEventKey = hash(sourceChainId, sourceToken, sourceBurnTxHash, sourceBurnEventIndex)

Open design questions:

- Should event index be enough for uniqueness?
- Should an explicit nonce be included?
- Should x1Recipient be bytes/string/address-like?
- How should invalid X1 recipients be prevented?

## Component 3: X1 XXXL token / mint core

The X1 XXXL mint core is the most important Stage 1 component.

It should be immutable or immutable-equivalent after deployment.

It should define:

- token name: XXXL
- Ethereum as the only Stage 1 source chain
- XNTD as the only Stage 1 source token
- source weight: 10000 bps
- mint formula
- processed burn replay protection
- accepted guardian threshold
- deterministic mint validation

Stage 1 formula:

    xxxlMintAmount = burnedAmount

This is equivalent to:

    xxxlMintAmount = burnedAmount * 10000 / 10000

This does not mean XXXL is the same asset as XNTD.

It means the Ethereum route has a full-weight conversion coefficient.

Open design questions:

- What language / framework should be used for the X1 mint program?
- How does X1 represent token mint authority?
- Can mint authority be restricted to immutable program logic?
- How are guardian public keys stored?
- Can guardian set rotation be separated from monetary rules?
- Can emergency pause be limited to mint execution only?

## Component 4: Processed burn registry

The X1 mint core must reject replay.

It must track processed source burns.

Required behavior:

    one accepted Ethereum burn event -> one XXXL mint operation

The registry should mark a burn as processed only when mint succeeds.

A failed mint must not consume the burn evidence.

Open design questions:

- Is processed state stored in the mint program?
- Is it stored in a separate account/registry?
- How is canonicalEventKey represented?
- What are storage costs on X1?
- Can processed status be queried by frontend?

## Component 5: Gateway message format

Guardian signatures must be over a deterministic message.

The message should include at least:

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

The message must prevent:

- replay across chains
- replay across recipients
- replay across amounts
- replay across source tokens
- replay across gateway versions

Open design questions:

- Should the format be EIP-712-like?
- What signature scheme does X1 best support?
- Should guardian signatures be Ethereum-style secp256k1?
- Does X1 support efficient secp256k1 verification?
- Should there be a signature expiry?
- Should the finality policy be included in the signed message?

## Component 6: Guardian signing format

Stage 1 target:

    5 gateway guardians
    3-of-5 approval threshold

Guardians sign only after verifying:

- burn event exists
- burn event is finalized
- burn event matches expected format
- burned amount is correct
- x1Recipient is valid
- sourceChainWeightBps is 10000
- xxxlMintAmount is correct
- source burn was not already processed

Guardians must not:

- alter recipient
- alter amount
- choose custom coefficient
- mint without burn evidence
- act as protocol governance over XC core

Open design questions:

- How are guardian public keys selected?
- How are the first 5 guardians bootstrapped?
- How is bootstrapped trust disclosed?
- What is the key rotation process?
- What is the lost-key recovery path?
- What is the guardian set update threshold?
- What is the timelock duration?

## Component 7: Guardian verification runtime

Each guardian should run a verification runtime.

The runtime should:

- read Ethereum burn events
- wait for finality / confirmations
- derive canonicalEventKey
- compute expected xxxlMintAmount
- check X1 processed status
- sign deterministic mint message
- publish or submit guardian approval

The runtime should not:

- hold user funds
- print secrets
- expose private keys
- make arbitrary mint decisions
- change monetary rules

Open design questions:

- Should guardians use independent RPC providers?
- Should guardians publish signatures to a shared API?
- Should signatures be gossiped peer-to-peer?
- Should a guardian also run a relayer?
- How should monitoring detect guardian disagreement?

## Component 8: Relayer runtime

The relayer submits approved mint messages to X1.

The relayer is not trusted.

The X1 mint core must verify everything.

Relayer responsibilities:

- collect guardian signatures
- submit mint transaction to X1
- retry failed submissions
- expose status to frontend
- avoid printing secrets

Relayer must not:

- mint without threshold signatures
- alter mint messages
- decide monetary rules
- mark source burns as processed off-chain

Open design questions:

- Can anyone be a relayer?
- Should the frontend be able to submit a completed proof?
- Should there be multiple relayers?
- Who pays X1 transaction fees?
- How are failed mints retried?

## Component 9: Watcher / indexer

A read-only watcher may track:

- Ethereum burn events
- confirmation/finality status
- guardian approvals
- X1 mint status
- processed burn status

The watcher should be read-only.

It should not be required as a trusted authority.

Open design questions:

- Is this separate from guardian runtime?
- Is it public infrastructure?
- Does frontend query it?
- How is data recovered after downtime?
- How are reorgs handled in stored state?

## Component 10: Frontend gateway flow

The existing xEnchanted frontend may provide Stage 1 UX.

Frontend flow:

1. connect Ethereum wallet
2. show Ethereum XNTD balance
3. enter X1 recipient
4. enter burn amount
5. show expected XXXL amount
6. show gateway disclosures
7. submit Ethereum burn
8. show waiting for confirmations
9. show guardian approval progress
10. show X1 mint pending / completed
11. show processed burn status

Frontend must disclose:

- this is a gateway, not a wrapped bridge
- XNTD is burned
- XXXL is minted
- XXXL is a different asset from XNTD
- there is no price peg
- there is no reverse conversion
- burn-to-mint is one-way and irreversible
- finality waiting is required
- gateway risk exists
- Stage 1 guardians are a trust assumption

Frontend must not be source of truth.

## Component 11: Monitoring and incident response

Stage 1 requires monitoring before production use.

Monitor:

- Ethereum burn event ingestion
- guardian signing health
- guardian disagreement
- relayer failures
- X1 mint failures
- replay attempts
- reorg events
- stuck burns
- delayed mints

Incident response must define:

- who can pause minting
- how pause is triggered
- what pause can and cannot do
- how users are informed
- how stuck valid burns are retried
- how guardian key compromise is handled
- how lost guardian keys are handled

## Component 12: Staging environment

Before production, Stage 1 should be tested in staging.

Staging should test:

- normal burn -> mint flow
- invalid recipient rejection
- invalid amount rejection
- replay rejection
- wrong source token rejection
- wrong source chain rejection
- insufficient guardian signatures
- duplicate mint attempt
- guardian disagreement
- relayer failure and retry
- mint failure recovery
- finality waiting
- reorg simulation if possible
- emergency pause behavior

## Component 13: Production readiness checklist

Before production deployment, require:

- Ethereum burn path reviewed
- X1 mint core reviewed
- immutable Stage 1 rules confirmed
- guardian set selected
- bootstrapped trust disclosed
- guardian key management documented
- guardian rotation documented
- lost-key recovery documented
- reorg/finality policy finalized
- deterministic message format finalized
- signature verification finalized
- replay protection tested
- mint retry tested
- emergency pause designed
- fee model disclosed
- frontend disclosure implemented
- monitoring implemented
- incident response documented
- external review / audit plan completed

## Suggested implementation order

Recommended order:

1. finalize Ethereum burn event schema
2. finalize deterministic mint message schema
3. design X1 XXXL mint core
4. design processed burn registry
5. design guardian key / signature model
6. design guardian runtime
7. design relayer runtime
8. design frontend flow
9. design monitoring / incident response
10. build local prototype
11. build staging prototype
12. run full staging tests
13. external review / audit
14. production readiness decision

## Current decision

Stage 1 implementation should not begin until these are reviewed:

- Ethereum burn event format
- X1 immutable mint core design
- guardian signature format
- processed burn replay protection
- guardian rotation / key recovery
- reorg/finality policy
- mint retry policy
- frontend disclosure flow
- incident response boundary

This plan defines the future implementation sequence.

It does not implement Stage 1.
