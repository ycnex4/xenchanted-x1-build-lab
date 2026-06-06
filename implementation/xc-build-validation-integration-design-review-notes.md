# XC Build validation integration design review

This document reviews the XC Build validation integration design milestone.

Reviewed branch:

    xc-build-validation-integration-design-review

Reviewed design milestone:

    xc-build-validation-integration-design

Reviewed files:

- implementation/xc-build-validation-integration-design.md
- docs/checkpoints/current-design-checkpoint.md

## Review summary

The XC Build validation integration design is accepted.

The design correctly treats `XcProtocolParams` and derived requirements as protocol context, not proof of user action.

The design correctly avoids directly modifying registrar, watcher, or app proof behavior in the next implementation milestone.

The design correctly recommends a pure validation context layer first.

## Design-only boundary review

The reviewed milestone is design-only.

Diff from pre-design baseline to current HEAD shows only:

- docs/checkpoints/current-design-checkpoint.md
- implementation/xc-build-validation-integration-design.md

No runtime code changed.

No tests changed.

No package dependency changed.

No script changed.

No real RPC was executed in the design milestone.

## Architecture review

The design correctly separates:

- protocol context
- user action proof validation
- registrar state transitions
- watcher proof ingestion
- app service orchestration
- future real RPC script edge

This separation is important because protocol params describe the current XC economic context, while user action proof still comes from event replay, watcher proof pipelines, proof registrar builders, registrar validation, and app proof submission flow.

## Integration point review

The design correctly says broader app/service integration should come later at the orchestration layer.

The immediate next implementation should not directly modify:

- registrar handlers
- watcher candidate generation
- app proof submission behavior
- global Build requirement enforcement

This avoids mixing context plumbing with enforcement rules.

## Context layer review

The recommended next milestone is accepted:

    xc-build-validation-context

Expected files:

- src/model/xc-build-validation-context.ts
- tests/xc-build-validation-context.test.ts
- src/index.ts

This is the right next step because it creates a stable pure object for later app/service integration.

## Proof boundary review

The design correctly states that `XcProtocolParams` and derived requirements are not proof of user action.

The design also correctly states that pure validation should not call `XcProtocolParamsSource` directly.

Future validation should receive already-loaded protocol params or already-derived requirements.

## Epoch caution review

The design correctly warns that current protocol params represent current deployed XC context, while submitted proofs may be historical.

The later epoch policy milestone is justified:

    xc-build-validation-epoch-policy-design

This should decide when currentEpoch must match proof epoch, whether historical Core redeem proof is accepted, whether Forge participation must be current epoch or any epoch, and how relock/update should behave across epoch changes.

## Boundary review

The future first implementation should not add:

- real RPC execution
- scripts
- dependencies
- direct `XcProtocolParamsSource` calls inside pure model code
- registrar state transition changes
- watcher candidate generation changes
- global Build requirement enforcement
- epoch policy finalization
- lock/relock rule changes
- bridge logic
- viem imports
- ethers imports
- createPublicClient
- http transport
- process.env reads
- private key support
- mnemonic support
- wallet client support
- writeContract
- sendTransaction

## Grep review

The review grep found architecture and boundary terms inside design/checkpoint text only.

That is expected.

No runtime files were added or changed by this design milestone.

## Validation baseline

Review baseline:

- npm run typecheck passed
- npm test passed: 39 test files, 309 tests
- npm run build passed
- npm audit --audit-level=moderate found 0 vulnerabilities

Manual refusal without env remained safe:

    Missing required Ethereum script secret config: XC_ETHEREUM_RPC_URL

## Review decision

The XC Build validation integration design is accepted.

No design changes are required before merging this review checkpoint.

Recommended next milestone after merge:

    xc-build-validation-context
