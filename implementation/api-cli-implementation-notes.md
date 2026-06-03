# API / CLI Implementation Notes

## Branch

api-cli-implementation

## Purpose

This branch starts the real API / CLI implementation layer for the xEnchanted X1 Build Lab.

The first implementation step adds a shared application service layer.

This service layer is intended to become the common foundation for future API handlers and CLI commands.

It does not add an HTTP server, CLI binary, storage adapter, or external dependencies yet.

## Implemented files

- src/app/build-service.ts
- tests/app-build-service.test.ts

Updated:

- src/index.ts

## Implemented application state

The branch adds BuildApplicationState, which groups the current MVP state objects:

- BuildRegistry
- RegistrarState
- RedeemEventState
- XenBurnEventState

This gives API / CLI code a single application-level context without changing the underlying model objects.

## Implemented result type

The branch adds AppResult<T>:

- ok: true with value
- ok: false with structured error

This allows future API / CLI layers to call model transitions without raw exceptions leaking into user-facing command or request handling.

## Implemented service helpers

The branch adds application-level helpers for:

- creating application state
- creating a registered Build
- querying a Build by buildId
- claiming Genesis Origin BLD
- applying registrar Core redeem
- applying registrar XEN burn
- applying registrar XNTD lock
- applying registrar XNTD relock
- applying registrar X1 fee checkpoint

## Error behavior

BuildError instances are converted into structured errors:

- code
- message

Unknown errors are converted into:

- code: UnknownError
- message

The model still remains responsible for actual business validation.

## Accounting policy

The application service layer does not duplicate accounting logic.

It delegates to existing model / instruction helpers.

This preserves the existing MVP invariant:

- API / CLI should not become a second source of protocol logic.

## Test coverage

Added test file:

- tests/app-build-service.test.ts

Covered cases:

- Build creation through application service
- Build query through application service
- structured duplicate Build error
- full lifecycle through application service
- structured registrar rejection error
- failed registrar flow does not mutate BuildState
- failed registrar flow does not mark replay sets

## Validation

Before commit:

- npm run typecheck: passed
- npm test: passed
- 19 test files passed
- 107 tests passed

## Current known exclusions

This milestone does not implement:

- HTTP API server
- CLI binary
- command parser
- JSON request / response schemas
- persistent storage adapter
- snapshot load / save
- authentication
- authorization
- rate limiting
- proof validators
- watcher integration

## Recommended next API / CLI steps

Possible next steps:

1. Add input parsing helpers for decimal bigint strings.
2. Add read-only CLI command design or implementation.
3. Add JSON command envelope types.
4. Add application service integration with storage snapshots.
5. Add CLI smoke tests.

## Main invariant

The application service layer makes the model easier to call.

It must not change the meaning of any protocol state transition.
