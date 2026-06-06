# XC Build validation integration design

This document designs where `deriveCurrentXcBuildRequirements()` should be used in the broader X1 Build validation flow.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The project now has two separate protocol params pieces:

1. Source layer:

    XcProtocolParamsSource

2. Pure derivation layer:

    deriveCurrentXcBuildRequirements()

The source layer reads deployed XC Lens protocol params through:

    getProtocolParams()

The derivation layer converts `XcProtocolParams` into current Build requirement context.

Current derivation:

    requiredBaseNominal = currentBaseNominal
    requiredXenBurnAmount = currentXenBurnAmount
    requiredXntdLockMinimum = currentBaseNominal
    requiredForgeMinimum = currentBaseNominal * 5

## Design goal

Decide where the derived XC Build requirements should be consumed in the broader Build validation flow.

The design must preserve separation between:

- protocol context
- user action proof validation
- registrar state transitions
- watcher proof ingestion
- app service orchestration
- future real RPC script edge

## Core rule

`XcProtocolParams` and derived requirements are context.

They are not proof of user action.

User action proof still comes from:

- event replay
- watcher proof pipelines
- proof registrar builders
- registrar validation
- existing app proof submission flow

Protocol params should explain the current economic rules used during validation.

They should not replace proof verification.

## Recommended integration point

The first integration should happen at the app/service orchestration layer, not inside low-level proof validators.

Recommended target:

    src/app/build-service.ts

or a new adjacent service module:

    src/app/xc-build-validation-service.ts

The app/service layer is the right place because it can combine:

- submitted proof
- current or provided protocol params
- derived requirements
- existing registrar/proof logic
- future UI-facing validation result

## Why not integrate directly into registrar first

Registrar logic should remain focused on deterministic state transitions and replay protection.

If protocol params are wired directly into registrar too early, it risks mixing:

- protocol context
- user proof validation
- state mutation

The first integration should keep requirements as explicit input.

Registrar integration may come later after the boundary is proven.

## Why not integrate directly into watcher first

Watcher logic should remain focused on discovering or converting external events into proof candidates.

Protocol params may help annotate or filter candidates later.

But initial integration should not make watcher correctness depend on current protocol params.

Watcher integration can be a later milestone.

## Why not call source from pure validation

Pure validation should not call `XcProtocolParamsSource` directly.

Pure validation should receive already-derived requirements or already-loaded protocol params.

This preserves:

- deterministic tests
- no RPC inside validation
- no hidden environment dependency
- replayability
- clear source boundary

## Proposed layering

Recommended future flow:

1. Script/UI/app edge obtains or injects `XcProtocolParams`
2. App/service layer calls `deriveCurrentXcBuildRequirements(protocolParams)`
3. App/service layer passes derived requirements into validation/check logic
4. Proof validation checks user action evidence separately
5. Registrar/app state transition happens only after both context and proof checks pass

Suggested conceptual flow:

    XcProtocolParamsSource
        -> XcProtocolParams
        -> deriveCurrentXcBuildRequirements()
        -> XcBuildRequirementsFromProtocolParams
        -> app/proof validation orchestration
        -> registrar/app state update

## Proposed validation context

Future implementation may introduce:

    XcBuildValidationContext

Suggested fields:

- requirements
- protocolParamsSnapshot
- sourceLabel
- observedAtMs
- observedBlockNumber
- validationMode

The first implementation should keep this smaller if possible.

Recommended minimal first shape:

    {
      requirements
    }

or:

    {
      protocolParams,
      requirements
    }

## Proposed first integration helper

Recommended future pure helper:

    createXcBuildValidationContextFromProtocolParams()

Possible output:

    {
      protocolParams,
      requirements
    }

This helper would not call RPC.

It would only combine already-loaded params and derived requirements.

## App service integration direction

A later implementation may add an optional protocol params context to app proof submission or build creation flows.

Possible approach:

- keep existing flow backward-compatible
- add optional `xcBuildRequirements` or `xcProtocolParams` input
- if provided, validate proof against derived requirements
- if absent, preserve existing behavior until full enforcement milestone

This avoids breaking existing tests and allows incremental enforcement.

## Requirement checks to add later

Future validation checks may include:

- proof nominal >= requiredBaseNominal
- proof XEN burn amount matches or is consistent with requiredXenBurnAmount
- XNTD lock amount >= requiredXntdLockMinimum
- Forge proof burn amount >= requiredForgeMinimum
- proof epoch matches currentEpoch or an accepted epoch policy

The exact checks should be implemented in separate milestones.

This document does not finalize enforcement.

## Important caution on current epoch

Current protocol params represent current deployed XC context.

A historical proof may have been created in an earlier epoch.

Therefore integration must distinguish:

- current requirement for new Build activation/update
- historical context of a submitted proof
- replayed proof event values

Do not reject valid historical proofs merely because current epoch has changed unless the Build rule explicitly requires current-epoch participation.

This needs a separate epoch policy milestone.

## Suggested epoch policy milestone

Recommended later milestone:

    xc-build-validation-epoch-policy-design

Purpose:

- decide when currentEpoch must match proof epoch
- decide whether historical Core redeem proof is accepted
- decide whether Forge participation must be current epoch or any epoch
- decide how relock/update should behave across epoch changes

## XNTD lock integration

The current derivation says:

    requiredXntdLockMinimum = currentBaseNominal

This should be integrated with existing XNTD lock/relock logic later.

But first integration should only pass the requirement object through.

Do not immediately mutate lock/relock rules without a focused review.

## Forge participation integration

The current derivation says:

    requiredForgeMinimum = currentBaseNominal * 5

This reflects XC Forge minimum direction.

Later proof validation may require evidence of Forge participation or XNTD burn.

But current integration design should not force Forge proof into every flow until the rule is finalized.

## Recommended first implementation milestone

Recommended next implementation branch:

    xc-build-validation-context

Expected files:

- src/model/xc-build-validation-context.ts
- tests/xc-build-validation-context.test.ts
- src/index.ts

Purpose:

- create a pure context helper
- combine `XcProtocolParams` and `XcBuildRequirementsFromProtocolParams`
- avoid changing existing app/registrar behavior
- create a stable object for later integration

## Alternative implementation milestone

If we want to integrate directly into app service instead:

    xc-build-validation-app-service-context

But this is riskier because it touches existing app flow.

Recommended approach remains:

    xc-build-validation-context

## Testing plan

First implementation should use mocked params only.

Tests should cover:

1. creates validation context from protocol params
2. includes derived requirements
3. keeps original protocol params snapshot
4. preserves currentEpoch
5. preserves requiredXntdLockMinimum
6. preserves requiredForgeMinimum
7. rejects invalid protocol params through existing derivation
8. does not call real RPC
9. does not import viem
10. does not read process.env
11. does not add wallet/transaction paths

## Non-goals

The first integration milestone should not:

- execute real RPC
- add scripts
- add dependencies
- call `XcProtocolParamsSource` inside pure model code
- modify registrar state transitions
- modify watcher candidate generation
- enforce all Build requirements globally
- finalize epoch policy
- change lock/relock rules
- add bridge logic

## Boundary checklist

Future implementation review should verify:

    grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|process\\.env|privateKey|mnemonic|walletClient|writeContract|sendTransaction" src tests package.json || true

Expected result:

- no new dangerous runtime matches
- no real RPC path
- no secret-bearing config
- no wallet/transaction support

## Decision

The next step should be a pure context integration milestone, not direct app/registrar/watcher enforcement.

Recommended next branch after review:

    xc-build-validation-context

The broader app service integration should come after the validation context object is stable.
