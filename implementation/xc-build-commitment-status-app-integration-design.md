# XC Build commitment status app integration design

This document defines how the application/service layer should integrate the XC Build commitment status model.

This milestone is design-only.

No runtime code is added in this milestone.

No dependencies are changed in this milestone.

No real RPC is executed in this milestone.

No RPC URL, API key, private key, mnemonic, seed phrase, `.env` content, or raw environment content is recorded here.

## Background

The project now has a read-only commitment status model:

    getBuildCommitmentStatus()

The helper returns:

    COMMITTED
    UNCOMMITTED
    UNKNOWN

with reason codes:

    COMMITMENT_CURRENT
    NO_HISTORY
    NO_COMMITMENT
    COMMITMENT_BELOW_REQUIRED
    RECOMMITMENT_REQUIRED
    UNKNOWN_NO_CURRENT_CONTEXT

The model is read-only and non-mutating.

It does not change Build state.

It does not enforce commitment status.

It does not change appSubmitProof, watcher, registrar, or proof payload behavior.

## Design goal

Define how app/service code should expose commitment status without turning it into mandatory enforcement too early.

The goal is:

    expose commitmentStatus as optional current XNTD commitment context

not:

    reject valid historical proofs because commitmentStatus is UNCOMMITTED

## Core principle

Commitment status should be exposed, not enforced globally.

The application layer may provide commitment status to callers.

The application layer should not automatically reject:

- Core redeem proof
- historical contribution
- history_bld reads
- available_bld reads
- Build state inspection

only because commitmentStatus is UNCOMMITTED.

UNCOMMITTED means:

    current XNTD commitment is missing, insufficient, or not established

UNCOMMITTED does not mean:

    Build history is invalid

## Recommended integration shape

Recommended app/service helper:

    appGetBuildView()

Reason:

The app layer may later expose more read-only Build context than commitment status alone.

Avoid naming this:

    appGetBuildValidity()
    appCheckBuildEligibility()
    appValidateActiveBuild()

Those names imply enforcement.

## Proposed app view shape

A future app/service read helper may return:

    interface AppBuildView {
      readonly build: BuildState;
      readonly commitmentStatus: BuildCommitmentStatus;
    }

Optional future fields:

    readonly protocolContext?: XcBuildValidationContext;
    readonly generatedAt?: bigint;

The MVP should keep this minimal.

## Input shape

Recommended input shape:

    interface AppGetBuildViewInput {
      readonly build: BuildState;
      readonly currentEpoch?: bigint;
      readonly currentRequiredXntdLock?: bigint;
      readonly requireCurrentEpoch?: boolean;
    }

The helper should pass status-related inputs to:

    getBuildCommitmentStatus()

## Current context source

The app/service helper should not fetch RPC directly.

Current context should be passed in from already designed context layers, such as:

- XcBuildValidationContext
- XcProtocolParams
- static test source
- future adapter output

The app integration should remain dependency-injected.

## No real RPC boundary

The app/service integration must not:

- read process.env
- create a public client
- create a wallet client
- call real RPC directly
- import viem or ethers directly
- execute transactions

If current context is unavailable, callers may omit it or request strict status and receive `UNKNOWN`.

## Relationship to appSubmitProof

The first app integration should not change appSubmitProof behavior.

Proof submission should continue to process valid proofs according to existing registrar rules.

Commitment status can be exposed next to Build state, but should not become a hidden proof rejection rule.

Do not make this change:

    appSubmitProof rejects because commitmentStatus is UNCOMMITTED

unless a future explicit enforcement milestone decides that for a specific proof type.

## Relationship to registrar

Registrar handlers should not depend on commitment status in this milestone.

Registrar mutation rules should remain unchanged.

Commitment status is read-only interpretation.

Registrar state transitions remain explicit and proof-driven.

## Relationship to watcher / proof payloads

Watcher candidate shapes should not change.

Proof payload shapes should not change.

Commitment status should be derived from existing Build state and optional current context.

Do not add commitmentStatus to proof payloads in this milestone.

## Relationship to external X1 projects

External X1 projects may use commitment status as optional context.

They may:

- ignore commitmentStatus
- display commitmentStatus
- give COMMITTED Builds a bonus
- require COMMITTED status for their own feature

The app/service layer should expose the signal cleanly, but not force external policy.

## Relationship to historical contribution

Historical contribution remains valid regardless of commitment status.

The app/service integration should preserve this distinction:

    Build history is historical.
    Commitment status is current XNTD commitment context.

Therefore, app views should show both when possible:

- historyBld
- availableBld
- commitmentStatus

This prevents users and external projects from confusing UNCOMMITTED status with erased history.

## UX / API wording

Recommended wording:

    commitmentStatus

Avoid names like:

    activeStatus
    validity
    validBuild
    invalidBuild
    eligibility

Reason:

`activeStatus` created the wrong effect.

It could imply that inactive Build means invalid, disabled, or punished Build.

`commitmentStatus` describes the real meaning:

    current XNTD commitment signal

## Recommended future implementation

A future implementation milestone may add:

    src/app/build-view.ts
    tests/app-build-view.test.ts

Possible export:

    export * from "./app/build-view.js";

Possible helper:

    appGetBuildView(input)

The helper should:

1. accept BuildState
2. accept optional current context
3. call getBuildCommitmentStatus()
4. return BuildState plus commitmentStatus
5. not mutate BuildState
6. not call RPC
7. not change registrar/appSubmitProof behavior

## Tests for future implementation

Future tests should verify:

- committed Build view includes COMMITTED status
- uncommitted Build view includes UNCOMMITTED status
- strict missing context returns UNKNOWN
- helper does not mutate Build state
- appSubmitProof behavior is unchanged
- no registrar behavior changes
- no proof payload changes

## Non-goals

This milestone does not:

- add runtime code
- add tests
- add dependencies
- execute real RPC
- change appSubmitProof behavior
- change watcher behavior
- change registrar behavior
- change proof payload behavior
- enforce commitment status
- define external project policy
- erase UNCOMMITTED Build history
- introduce Forge requirements
- add unlock mechanics
- change BLD transfer/sale rules
- add CLI commands

## Recommended future milestones

After this design is reviewed, recommended future milestones are:

1. app integration design review

    xc-build-commitment-status-app-integration-design-review

2. app Build view implementation

    xc-build-commitment-status-app-view

3. app Build view review

    xc-build-commitment-status-app-view-review

4. final MVP Build validation rule design

    xc-build-validation-mvp-rule-design

Forge is intentionally not included in this sequence.

## Decision

MVP app integration direction:

    expose commitmentStatus as optional current XNTD commitment context
    do not enforce commitmentStatus globally
    do not reject historical proofs because commitmentStatus is UNCOMMITTED
    keep appSubmitProof, watcher, registrar, and proof payload behavior unchanged
    keep external X1 project usage optional

Recommended next step:

    xc-build-commitment-status-app-integration-design-review
