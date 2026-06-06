# XC Build validation app service context design

This document designs how `XcBuildValidationContext` should enter the app/proof submission flow.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The project now has a pure context helper:

    createXcBuildValidationContextFromProtocolParams()

It creates:

    {
      protocolParams,
      requirements
    }

from already-loaded `XcProtocolParams`.

The context includes derived requirements such as:

    requiredBaseNominal = currentBaseNominal
    requiredXenBurnAmount = currentXenBurnAmount
    requiredXntdLockMinimum = currentBaseNominal
    requiredForgeMinimum = currentBaseNominal * 5

## Design goal

Decide how app/proof submission should optionally receive `XcBuildValidationContext`.

The integration must preserve current behavior unless context is explicitly provided.

The integration must not directly modify watcher candidate generation.

The integration must not directly modify registrar state transition semantics in the same branch.

The integration must not execute real RPC.

## Core principle

`XcBuildValidationContext` is protocol context.

It is not proof of user action.

Proof validation remains based on existing proof, watcher, registrar, and replay logic.

The app service may use context to make validation stricter, but the first integration should only pass context through safely.

## Recommended app-level boundary

The app/proof submission layer is the right place to accept optional context because it already orchestrates:

- proof input
- registrar payload building
- registrar app helpers
- Build state mutation
- replay protection

Recommended target files for a future implementation:

- `src/app/proof-submission.ts`
- possibly `src/app/build-service.ts`
- tests around app proof submission and e2e flow

## Backward compatibility requirement

The future implementation must preserve existing callers.

If no `xcBuildValidationContext` is provided, existing behavior should remain unchanged.

This avoids breaking:

- existing unit tests
- existing app proof submission flow
- existing watcher-to-proof-to-registrar e2e flow
- trusted registrar MVP behavior

## Suggested input shape

A future app/proof submission input may add an optional field:

    xcBuildValidationContext?: XcBuildValidationContext

or a shorter name:

    validationContext?: XcBuildValidationContext

Recommended name:

    xcBuildValidationContext

Reason:

- explicit
- avoids confusion with generic validation
- clearly tied to XC protocol params and Build requirements

## First integration behavior

The first runtime implementation should only accept and carry the context.

It should not enforce all requirements globally yet.

Possible minimal behavior:

- accept optional `xcBuildValidationContext`
- make it available to local validation checks
- preserve existing behavior when absent
- add tests proving no behavior changes without context
- add tests proving context can be passed without mutation side effects

## What should not happen yet

The first app-service context implementation should not:

- reject historical proofs based on currentEpoch
- enforce Forge participation globally
- enforce Core redeem nominal globally
- enforce current XEN burn amount globally
- change lock/relock registrar behavior
- change watcher candidates
- change proof payload types unless explicitly needed
- call real RPC
- call `XcProtocolParamsSource`
- read process.env
- add scripts
- add dependencies

## Relationship to existing XNTD lock validation

Existing XNTD lock / relock validation already has an authoritative epoch minimum path.

`XcBuildValidationContext` should not replace that path immediately.

Instead, future integration should decide whether the app service can use context to supply or verify the same required lock value.

Do not duplicate or contradict existing registrar-level authoritative validation.

## Relationship to observedRequiredXntdLock

The existing proof/watcher/registrar chain already carries observed required lock data in the relevant XNTD lock/relock path.

Future app integration should be careful:

- observed value comes from watcher/proof payload
- authoritative value comes from protocol context or authoritative source
- registrar/app validation should compare them only in a focused enforcement milestone

This design does not change that logic.

## Suggested first implementation branch

Recommended next implementation branch after review:

    xc-build-validation-app-service-context

Expected files may include:

- `src/app/proof-submission.ts`
- `tests/app-proof-submission.test.ts`
- possibly `tests/e2e-watcher-proof-registrar-scenario.test.ts`
- `src/index.ts` only if a new exported type/helper is added

But the first implementation should be as small as possible.

## Alternative safer implementation

If touching `app/proof-submission.ts` feels too broad, create a separate app-level helper first:

    src/app/xc-build-validation-context-service.ts

Possible helper:

    attachXcBuildValidationContextToSubmission()

However, this may be over-abstraction if the app input can accept the optional field directly.

Recommended path:

    add optional context to existing app proof submission input

only if the change is small and backwards-compatible.

## Minimal future tests

Future implementation tests should cover:

1. existing proof submission still passes without context
2. app proof submission accepts context
3. context object is not mutated
4. derived requirements remain available during submission
5. no real RPC is called
6. no `XcProtocolParamsSource` is called
7. no process.env is read
8. registrar replay behavior remains unchanged
9. watcher proof conversion remains unchanged
10. e2e watcher-proof-registrar scenario remains green

## Enforcement milestones after context wiring

After app service can receive context, later milestones can decide enforcement rules.

Recommended later design milestones:

- `xc-build-validation-epoch-policy-design`
- `xc-build-validation-core-redeem-rule-design`
- `xc-build-validation-forge-participation-rule-design`
- `xc-build-validation-xntd-lock-rule-design`

These should not be collapsed into the first app-service context branch.

## MVP impact

This step moves the project closer to MVP because it creates the bridge between:

- authoritative XC protocol context
- derived Build requirements
- app-level proof submission orchestration

But MVP enforcement still requires separate rule milestones.

## Boundary checklist

Future implementation review should verify:

    grep -RniE "from ['\"]viem['\"]|from ['\"]ethers['\"]|createPublicClient|http\\(|process\\.env|privateKey|mnemonic|walletClient|writeContract|sendTransaction" src tests package.json || true

Expected result:

- no new real RPC path
- no secret-bearing config
- no wallet/transaction path
- no new dependency
- no scripts

## Decision

The app/proof submission layer should receive `XcBuildValidationContext` as an optional input.

The first implementation should be backwards-compatible and should not enforce global Build validity rules yet.

Recommended next branch after review:

    xc-build-validation-app-service-context
