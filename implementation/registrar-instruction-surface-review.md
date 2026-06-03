# Registrar / Instruction Surface Review

## Branch

registrar-instruction-surface-review

## Purpose

This document reviews the current registrar and instruction surface of the MVP model.

The goal is to make the implemented transition surface explicit before moving to proof model, storage, serialization, API, or CLI design.

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Registrar message kinds

Current RegistrarMessageKind values:

- CORE_REDEEM
- XEN_BURN
- LOCK_XNTD
- RELOCK_XNTD
- X1_FEE_CHECKPOINT

## Registrar state

RegistrarState currently contains:

- registrarAuthority
- processedMessages

Purpose:

- registrarAuthority defines the accepted registrar identity.
- processedMessages prevents messageId replay.

## Registrar integration surface

| Registrar message kind | Integration helper | Underlying transition | Replay layers |
| --- | --- | --- | --- |
| CORE_REDEEM | applyRegistrarCoreRedeem | acceptCoreRedeemEvent -> applyCoreRedeemBld | processedMessages + usedRedeemEvents |
| XEN_BURN | applyRegistrarXenBurn | acceptXenBurnEvent -> applyXenBurnPower | processedMessages + usedXenBurnEvents |
| LOCK_XNTD | applyRegistrarXntdLock | lockXntd | processedMessages |
| RELOCK_XNTD | applyRegistrarXntdRelock | relockXntd | processedMessages |
| X1_FEE_CHECKPOINT | applyRegistrarX1FeeCheckpoint | applyX1FeeContributionCheckpoint | processedMessages + countedUntilSlot monotonicity |

## Non-registrar instruction surface

| Instruction | Purpose |
| --- | --- |
| createBuild | Creates a new BuildState |
| applyCoreRedeemBld | Applies BLD from Core redeem history |
| applyXenBurnPower | Applies XEN Burn Power |
| claimGenesisOriginBld | Claims one-time Genesis Origin BLD based on historyBld |
| lockXntd | Activates XC commitment by recording XNTD lock state |
| relockXntd | Updates active XC commitment for a new epoch / requirement |
| applyX1FeeContributionCheckpoint | Applies X1 fee contribution checkpoint |

## Model / state helper surface

| Helper | Purpose |
| --- | --- |
| createEmptyBuildRegistry | Creates empty Build registry |
| createRegisteredBuild | Creates and registers a Build while enforcing uniqueness |
| createRegistrarState | Creates registrar replay-protection state |
| acceptRegistrarMessage | Records accepted registrar message after validation |
| createRedeemEventState | Creates Core redeem replay-protection state |
| acceptCoreRedeemEvent | Applies Core redeem event through replay protection |
| createXenBurnEventState | Creates XEN burn replay-protection state |
| acceptXenBurnEvent | Applies XEN burn event through replay protection |

## Atomicity pattern

Registrar integrations currently follow this pattern:

1. validate message kind
2. validate registrar authority
3. validate messageId is not already processed
4. validate event-specific replay protection or transition-specific rules
5. apply underlying transition
6. record registrar message

Important rule:

The registrar message should be recorded only after the underlying transition succeeds.

This prevents invalid messages from being burned into processedMessages.

## Current failure behavior

Implemented registrar flows are expected to reject without mutation for:

- wrong message kind
- unauthorized registrar
- duplicate registrar message
- duplicate Core redeem event key
- duplicate XEN burn event key
- invalid BLD amount
- invalid XBP amount
- invalid XNTD lock amount
- relock without active commitment
- relock with insufficient availableBld
- invalid fee amount
- invalid tx count
- non-increasing fee checkpoint slot

## Current accounting separation

Registrar and instruction flows should not create unrelated value.

Current expectations:

- CORE_REDEEM can change BLD accounting only.
- XEN_BURN can change XBP accounting only.
- LOCK_XNTD / RELOCK_XNTD can change XNTD commitment state only.
- X1_FEE_CHECKPOINT can change fee contribution accounting only.
- Genesis Origin BLD can set originBld and increase availableBld, but must not increase historyBld.

## Surface consistency observations

The current surface is coherent for an in-memory MVP model.

Strong points:

- registrar message replay is centralized through processedMessages
- event replay is separated by domain
- accounting transitions are small and explicit
- registrar integrations wrap underlying transitions without mixing domains
- test coverage repeatedly checks non-mutating failure paths

Potential later improvements:

- factor common registrar pre-check logic into a shared helper
- introduce typed message payloads per message kind
- separate message metadata from event payload
- define canonical event key derivation rules
- define proof object types
- define serialization formats for state and messages
- define storage boundaries
- define API / CLI schemas

## Current known exclusions

The current surface does not include:

- registrar signature validation
- on-chain proof validation
- Merkle proof validation
- bridge proof validation
- external storage adapters
- serialization / deserialization
- API request schemas
- CLI command schemas
- canonical source event key derivation
- unlock flow
- BLD transfer / burn mechanics
- BLD minting from X1 fees

## Recommended next milestones

Recommended next milestones:

1. Post-MVP integration policy
2. Proof model design
3. Storage / serialization model
4. API / CLI surface design
5. End-to-end scenario tests

## Main conclusion

The current registrar and instruction surface is ready as an MVP state-transition layer.

The next major design step should not add new accounting behavior immediately.

The next step should define how external facts become trusted registrar messages or proofs.
