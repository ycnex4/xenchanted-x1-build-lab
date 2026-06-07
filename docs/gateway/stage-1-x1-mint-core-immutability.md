# Stage 1 X1 mint core immutability

This document defines the Stage 1 immutability requirements for the X1 XXXL mint core.

This is a design / readiness document only.

No runtime code is changed.

No contracts or X1 programs are implemented here.

No deployment is approved by this document.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Purpose

The Stage 1 Gateway architecture depends on a strict boundary:

- gateway guardians = verification layer
- immutable mint core / route rules = monetary conversion rules
- relayer = execution / transport layer without discretion

This document defines the immutability requirements for the X1 mint core so guardians, relayers, deployers, or administrators cannot control XXXL monetary policy.

## Source context

This document builds on:

- docs/gateway/stage-1-xxxl-gateway-architecture.md
- docs/gateway/stage-1-xxxl-gateway-implementation-plan.md
- docs/gateway/stage-1-gateway-message-schema.md
- docs/gateway/stage-1-gateway-canonical-encoding.md
- docs/gateway/stage-1-gateway-theo-review-notes.md
- docs/gateway/stage-1-gateway-pre-implementation-blockers.md
- docs/gateway/stage-1-gateway-hash-signature-recipient-decisions.md
- docs/gateway/stage-1-gateway-mandatory-source-block-fields.md

Theo identified X1 mint core immutability as a pre-implementation blocker.

This document closes the requirement-definition layer, but the exact X1 deployment mechanism must still be confirmed before production implementation.

## Core principle

The X1 mint core must enforce monetary conversion rules independently from guardian signatures.

Guardian signatures prove that source burn evidence was verified.

Guardian signatures must not define monetary policy.

The mint core must reject any message that violates immutable Stage 1 route rules, even if guardians signed it.

## Immutable Stage 1 route rules

The following Stage 1 route rules must be immutable:

- source chain is Ethereum mainnet
- source token is the expected Ethereum XNTD token
- routeId is the Stage 1 Ethereum XNTD to X1 XXXL route
- sourceChainWeightBps = 10000
- xxxlMintAmount = burnedAmount
- mint token is XXXL
- canonicalEventKey is derived from sourceChainId, sourceToken, sourceBurnTxHash, and sourceBurnEventIndex
- hash function is keccak256
- guardian signature standard is Ed25519
- X1 recipient is 32 raw bytes
- x1RecipientHash = keccak256(x1RecipientBytes)
- sourceBlockNumber and sourceBlockHash are mandatory signed fields

These rules must not be changeable by guardians, relayers, or ordinary operators.

## Mint core verification requirements

Before minting XXXL, the X1 mint core must verify:

- messageType is expected
- schemaVersion is supported
- routeId is expected Stage 1 route
- sourceChainId is Ethereum mainnet
- sourceToken is expected Ethereum XNTD token
- sourceChainWeightBps equals 10000
- xxxlMintAmount equals burnedAmount
- mintToken is XXXL
- burnedAmount is greater than zero
- x1RecipientBytes length is exactly 32 bytes
- x1RecipientBytes is not 32 zero bytes
- keccak256(x1RecipientBytes) equals x1RecipientHash
- sourceBlockNumber is present
- sourceBlockHash is present and exactly 32 bytes
- canonicalEventKey is derived correctly
- guardian signatures satisfy the required Ed25519 threshold
- canonicalEventKey has not been processed before

The mint core must not trust guardian-supplied monetary values unless they match immutable route rules.

## Guardian overreach rejection

The mint core must reject guardian-approved messages if any immutable route value is wrong.

Examples:

- guardians sign sourceChainWeightBps = 9000
- guardians sign sourceChainWeightBps = 50000
- guardians sign xxxlMintAmount higher than burnedAmount
- guardians sign xxxlMintAmount lower than burnedAmount
- guardians sign a different source token
- guardians sign a different source chain
- guardians sign a different mint token
- guardians sign a malformed recipient hash
- guardians sign a message missing sourceBlockHash

Guardian threshold is necessary but not sufficient.

A valid guardian threshold cannot override route rules.

## Relayer limitations

The relayer must not be able to change monetary values.

The relayer may submit:

- signed message
- guardian signatures
- raw x1RecipientBytes
- evidence references
- execution metadata

The relayer must not be able to change:

- burnedAmount
- xxxlMintAmount
- sourceChainWeightBps
- routeId
- sourceToken
- sourceChainId
- mintToken
- canonicalEventKey
- x1RecipientHash

If relayer-submitted execution data conflicts with the signed message, mint core must reject.

## Deployment authority requirement

The exact X1 deployment model is still open.

However, the production requirement is clear:

Route rules and monetary conversion logic must not remain controllable by a deployer, admin, guardian set, relayer, or mutable governance path after production deployment.

Before implementation or production approval, the project must define:

- whether X1 programs / contracts are upgradeable
- whether deployer authority exists
- how deployer authority is removed, disabled, or constrained
- whether mint authority exists separately from program authority
- whether mint authority can be changed
- whether any governance / timelock path exists
- which values are immutable forever
- which values, if any, can be operationally configured
- how users can verify the deployed immutability state

## Acceptable immutability outcomes

Acceptable production outcomes may include:

- non-upgradeable mint core
- upgrade authority permanently removed
- route rules hardcoded in deployed code
- mint authority constrained to the immutable mint core
- mutable operational settings separated from monetary route rules
- public verification procedure showing no mutable authority can alter route rules

The exact mechanism depends on X1 runtime capabilities.

## Unacceptable immutability outcomes

The following outcomes are not acceptable for production Stage 1:

- guardians can change route weight
- guardians can change mint formula
- relayer can choose mint amount
- deployer can upgrade route rules after launch
- admin can change source token
- admin can change source chain
- admin can change mint token
- governance can change monetary conversion without a new explicit route / deployment
- mint authority can mint XXXL outside verified gateway messages
- processed burn registry can be bypassed by privileged authority

If any of these remain possible, the mint core is not immutable enough for Stage 1 production.

## Operational configuration boundary

Some operational configuration may be acceptable only if it does not affect monetary conversion.

Potentially acceptable operational settings:

- guardian set rotation mechanism, if explicitly separated from route rules
- emergency pause of intake, if it cannot mint or alter conversion
- frontend display configuration
- watcher endpoint configuration
- relayer configuration
- monitoring thresholds

Not acceptable as operational settings:

- sourceChainWeightBps
- xxxlMintAmount formula
- source token
- source chain
- mint token
- replay key formula
- ability to mark arbitrary burns as processed without verification
- ability to mint without processed burn evidence

Any operational mutability must be documented separately and must not compromise first-principles monetary immutability.

## Guardian set mutability distinction

Guardian set management is not the same as monetary policy.

It may be acceptable to rotate guardians for security reasons if:

- guardian rotation cannot change route rules
- guardian rotation cannot change mint formula
- guardian rotation cannot mint without valid burn evidence
- guardian rotation process is transparent and documented
- guardian set version is included in signed / verified context if required

This is a separate design topic and must not be confused with mint core monetary immutability.

## User verification requirement

Before production, users should be able to verify:

- deployed mint core identity
- routeId
- source chain
- source token
- sourceChainWeightBps
- mint formula
- mint token
- processed burn registry address / account
- guardian verification model
- whether upgrade authority exists
- whether mint authority can be changed
- whether any admin can alter route rules

The production readiness checklist must include these checks.

## Test and review implications

Before implementation, the following must be documented:

- exact X1 deployment model
- exact immutability mechanism
- exact mint authority model
- exact upgrade authority status
- exact public verification commands or checks
- exact failure modes for unauthorized route changes
- exact tests for guardian overreach rejection
- exact tests for relayer manipulation rejection

## Remaining open work

This document defines immutability requirements.

It does not yet define the exact X1 deployment procedure.

Remaining work:

- confirm X1 program / contract deployment model
- confirm upgrade authority mechanics
- confirm mint authority mechanics
- define public immutability verification procedure
- define whether any operational pause exists
- define guardian set rotation separately

## Current conclusion

Stage 1 requires an immutable X1 mint core whose route rules and monetary conversion logic cannot be changed by guardians, relayers, deployers, administrators, or mutable governance after production deployment.

This closes the immutability requirement-definition blocker.

Implementation should still not begin until the exact X1 deployment and authority model is documented and exact test vectors are produced.
