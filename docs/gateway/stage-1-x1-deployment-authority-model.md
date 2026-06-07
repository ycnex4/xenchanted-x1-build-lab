# Stage 1 X1 deployment authority model

This document defines the Stage 1 deployment authority requirements for the X1 XXXL mint core.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway depends on a clear authority boundary.

The X1 mint core must enforce monetary route rules independently from guardians, relayers, deployers, administrators, or mutable governance.

This document defines the deployment authority model requirements that must be satisfied before implementation or production approval.

The exact X1 runtime deployment mechanism must still be confirmed before code.

## Source context

This document builds on:

- docs/gateway/stage-1-xxxl-gateway-architecture.md
- docs/gateway/stage-1-xxxl-gateway-implementation-plan.md
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
- docs/gateway/stage-1-recipient-safety-policy.md
- docs/gateway/stage-1-burn-amount-policy.md

## Core authority principle

Stage 1 authority model must preserve this boundary:

- guardians verify Ethereum burn evidence
- relayers transport approved messages
- mint core enforces immutable monetary route rules
- no operator can mint XXXL outside verified gateway messages

The mint core must be the only path that can mint Stage 1 XXXL from Ethereum XNTD burn evidence.

## Immutable route rules

The following route rules must not be changeable after production deployment:

- source chain is Ethereum mainnet
- source token is expected Ethereum XNTD token
- routeId is the Stage 1 Ethereum XNTD to X1 XXXL route
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- hash function is keccak256
- guardian signature standard is Ed25519
- X1 recipient is 32 raw bytes
- x1RecipientHash = keccak256(x1RecipientBytes)
- sourceBlockNumber and sourceBlockHash are mandatory signed fields
- replay protection uses canonicalEventKey
- processed registry prevents duplicate minting

## Deployment authority requirement

Before production approval, the project must know exactly:

- how X1 programs / contracts are deployed
- whether deployed programs / contracts are upgradeable
- who controls upgrade authority at deployment
- how upgrade authority is removed, disabled, or constrained
- whether token mint authority is separate from program authority
- who controls token mint authority at deployment
- how token mint authority is constrained to the mint core
- whether any admin, deployer, guardian, relayer, or governance path can mint outside the verified route
- how users can independently verify the authority state

## Acceptable production outcomes

Acceptable outcomes may include:

- non-upgradeable mint core
- upgrade authority permanently removed
- route rules hardcoded in deployed code
- mint authority owned only by immutable mint core
- mint authority unable to mint outside verified gateway messages
- public verification procedure for authority state
- separate operational configuration that cannot change monetary route rules

## Unacceptable production outcomes

The following are not acceptable for Stage 1 production:

- deployer can upgrade route rules after launch
- admin can change source token
- admin can change source chain
- admin can change sourceChainWeightBps
- admin can change xxxlMintAmount formula
- admin can change mint token
- guardian set can change monetary policy
- relayer can choose mint amount
- token mint authority can mint XXXL outside the mint core
- governance can silently change Stage 1 route rules
- processed registry can be bypassed by privileged authority
- emergency function can create supply outside verified burn evidence

## Mint authority model

Stage 1 must define how XXXL mint authority works.

Required properties:

- mint authority cannot be used by a human operator to mint arbitrary XXXL
- mint authority cannot be used by relayers to mint arbitrary XXXL
- mint authority cannot be used by guardians to mint arbitrary XXXL
- mint authority is constrained to verified Stage 1 gateway execution
- mint authority cannot bypass processed registry
- mint authority cannot bypass immutable route validation
- mint authority cannot bypass guardian threshold verification

Preferred direction:

The X1 XXXL token mint authority should be controlled by the mint core or by an authority mechanism that only the mint core can exercise under verified message rules.

## Upgrade authority model

Stage 1 must define whether the X1 mint core is upgradeable.

If non-upgradeable:

- document how immutability is achieved
- document how users verify no upgrade authority exists
- document route rule identity

If upgradeable during development:

- production deployment must define when upgrade authority is removed or disabled
- development upgradeability must not be confused with production immutability
- users must be able to distinguish test / development deployments from production deployments

If upgradeability remains in production:

- Stage 1 cannot be considered fully immutable
- this must be treated as a separate trust assumption
- route-rule mutation risk must be disclosed
- first-principles claim must be limited accordingly

Preferred production direction:

No production upgrade authority should be able to alter Stage 1 monetary route rules.

## Guardian authority boundary

Guardians may sign approvals only after verifying source burn evidence.

Guardians must not be able to:

- change route rules
- change sourceChainWeightBps
- choose arbitrary xxxlMintAmount
- change source token
- change source chain
- change mint token
- bypass processed registry
- mint without burn evidence

Guardian rotation, if supported, must be separated from monetary route rules.

## Relayer authority boundary

Relayers may submit approved messages and execution payloads.

Relayers must not be able to:

- change recipient
- change amount
- change source chain
- change source token
- change routeId
- change canonicalEventKey
- bypass signature verification
- bypass processed registry
- mint without valid guardian approval

Relayer failure or censorship should affect transport availability, not monetary rules.

## Emergency controls

Emergency controls, if any, must be carefully limited.

Potentially acceptable:

- pause new mint executions
- pause relayer frontend submission
- pause guardian signing
- publish incident status

Not acceptable:

- admin mint
- admin route rewrite
- admin processed-registry bypass
- admin recipient rewrite
- admin amount rewrite
- admin replay override
- admin mint after failed verification

If any emergency control exists, it must not create XXXL supply outside verified gateway messages.

## Public verification checklist

Before production, the project must publish a public verification checklist.

The checklist should allow users and reviewers to verify:

- deployed mint core identity
- deployed XXXL mint identity
- routeId
- source chain
- source token
- mint token
- sourceChainWeightBps
- mint formula
- guardian signature standard
- guardian threshold model
- processed registry identity
- upgrade authority status
- mint authority status
- whether admin mint exists
- whether route rules are mutable
- whether emergency controls can mint
- whether processed registry can be bypassed

## Test implications

Future tests or verification scripts should include:

- mint core rejects wrong sourceChainWeightBps
- mint core rejects wrong xxxlMintAmount
- mint core rejects wrong source token
- mint core rejects wrong source chain
- mint core rejects wrong mint token
- mint core rejects duplicate canonicalEventKey
- no admin mint path exists
- no relayer amount override exists
- no guardian monetary override exists
- upgrade authority status is verifiable
- mint authority status is verifiable

## Current conclusion

Stage 1 requires a deployment authority model where route rules and monetary conversion cannot be changed by deployers, administrators, guardians, relayers, or mutable governance after production deployment.

The exact X1 runtime deployment mechanism still must be confirmed before implementation.

Production readiness requires public verification of upgrade authority, mint authority, route rule immutability, and absence of admin mint paths.

This closes the deployment authority model requirement-definition layer.

Implementation should still not begin until exact cryptographic test vectors are documented and the X1 runtime authority mechanics are confirmed.
