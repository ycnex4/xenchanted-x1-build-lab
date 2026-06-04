# Application proof submission notes

This milestone adds the application service proof submission layer.

## Added files

- `src/app/proof-submission.ts`
- `tests/app-proof-submission.test.ts`

## Updated files

- `src/index.ts`

## Purpose

The new application proof submission helper connects the validated proof layer to the existing registrar application service layer.

The submission path is:

BuildProof
  -> buildRegistrarPayloadFromProof
  -> find Build by buildId
  -> call the matching appApplyRegistrar... helper
  -> return AppResult<BuildState>

## Implemented helper

- `appSubmitProof(app, proof, input)`

The helper accepts:

- `BuildApplicationState`
- `BuildProof`
- submission metadata:
  - `submittedBy`
  - `createdAt`
  - optional `messageId`

## Architectural boundary

This layer does not duplicate accounting logic.

It does not apply proof payloads directly to `BuildState`.

It does not bypass registrar replay protection.

It only routes a validated proof through:

- `buildRegistrarPayloadFromProof`
- `appApplyRegistrarCoreRedeem`
- `appApplyRegistrarXenBurn`
- `appApplyRegistrarXntdLock`
- `appApplyRegistrarXntdRelock`
- `appApplyRegistrarX1FeeCheckpoint`

## Error handling

The helper returns structured `AppResult` values.

Covered rejection paths:

- non-validated proof
- Genesis Origin proof
- missing Build
- registrar rejection
- duplicate proof submission through existing registrar replay protection

## Genesis Origin proof policy

Genesis Origin eligibility proof is intentionally not mapped through registrar payload submission.

Genesis Origin remains a separate application action and does not use registrar message replay state.

## Test coverage

The new tests cover:

- Core redeem proof submission
- XEN burn proof submission
- XNTD lock proof submission
- XNTD relock proof submission
- X1 fee checkpoint proof submission
- non-validated proof rejection without state mutation
- missing Build rejection without registrar replay writes
- Genesis Origin proof rejection
- duplicate proof submission rejection through existing replay protection

## Current verification result

After the code commit:

- 26 test files passed
- 149 tests passed
