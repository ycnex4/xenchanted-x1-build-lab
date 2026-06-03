# API / CLI Surface Design

## Branch

api-cli-surface-design

## Purpose

This document defines the future API and CLI surface direction for the post-MVP xEnchanted X1 Build Lab.

The current MVP is an in-memory state-transition model.

API and CLI layers should expose the model safely without duplicating accounting logic.

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Design boundary

API and CLI layers should be adapters around the MVP model.

They may:

- parse user input
- validate command shape
- load state
- invoke model instructions
- persist updated state
- display results
- return structured errors

They must not:

- implement independent accounting formulas
- bypass replay protection
- mutate BuildState directly
- mark messages or events as processed without successful transition
- create unrelated accounting values
- silently repair invalid state

## Core principle

The model remains the source of truth for state transitions.

API / CLI should be thin orchestration layers.

Recommended flow:

1. parse request / command
2. validate input shape
3. load persisted state
4. call existing model instruction
5. persist updated state only after success
6. return result / error

## API surface categories

Future API surface may be grouped into these categories:

- Build queries
- Build creation
- Registrar message submission
- Proof submission
- Snapshot / storage operations
- Health / diagnostics
- Adminless read-only status endpoints

## Build query API

Possible endpoints:

- GET /builds/:buildId
- GET /owners/:owner/build
- GET /ethereum/:ethereumIdentity/build
- GET /builds/:buildId/accounting
- GET /builds/:buildId/commitment
- GET /builds/:buildId/fees

Purpose:

- expose BuildState
- expose accounting layers
- expose lock / relock status
- expose fee contribution status

Must not mutate state.

## Build creation API

Possible endpoint:

- POST /builds

Expected input:

- owner
- buildId
- optional ethereumIdentity
- createdAt or server timestamp policy

Underlying model call:

- createRegisteredBuild or createBuild depending on storage / registry design

Required behavior:

- reject duplicate buildId
- reject duplicate owner
- reject duplicate ethereumIdentity if provided
- return created BuildState

## Registrar message API

Possible endpoints:

- POST /registrar/core-redeem
- POST /registrar/xen-burn
- POST /registrar/xntd-lock
- POST /registrar/xntd-relock
- POST /registrar/x1-fee-checkpoint

Underlying model calls:

- applyRegistrarCoreRedeem
- applyRegistrarXenBurn
- applyRegistrarXntdLock
- applyRegistrarXntdRelock
- applyRegistrarX1FeeCheckpoint

Required behavior:

- validate message payload
- validate build exists
- call registrar integration
- persist only after success
- return structured error on failure

## Proof submission API

Possible endpoints:

- POST /proofs/core-redeem
- POST /proofs/xen-burn
- POST /proofs/xntd-lock
- POST /proofs/xntd-relock
- POST /proofs/x1-fee-checkpoint

This layer should remain separate from raw registrar endpoints.

Expected flow:

1. receive proof object
2. validate proof
3. derive canonical payload
4. call registrar or model instruction
5. persist successful transition

Proof endpoints are future work and should not be implemented before proof model types are defined.

## Snapshot / storage API

Possible endpoints:

- GET /snapshot
- POST /snapshot/restore
- GET /health/storage
- GET /health/replay-state

These are optional and should be restricted if exposed.

Snapshot restore is dangerous and should not be public by default.

## Health / diagnostics API

Possible endpoints:

- GET /health
- GET /version
- GET /metrics
- GET /checkpoint

Purpose:

- show service health
- show schema version
- show current model version
- show replay key counts
- show current checkpoint document version if useful

Must not expose secrets.

## CLI surface categories

Future CLI commands may be grouped into:

- build
- registrar
- proof
- snapshot
- inspect
- validate

## Build CLI commands

Possible commands:

- build create
- build show
- build accounting
- build commitment
- build fees

Examples:

- build create --owner <owner> --build-id <id>
- build show --build-id <id>
- build accounting --build-id <id>

## Registrar CLI commands

Possible commands:

- registrar core-redeem
- registrar xen-burn
- registrar xntd-lock
- registrar xntd-relock
- registrar x1-fee-checkpoint

The CLI should build structured input and pass it to the same application service used by API.

CLI must not duplicate transition logic.

## Proof CLI commands

Possible commands:

- proof core-redeem validate
- proof xen-burn validate
- proof xntd-lock validate
- proof x1-fee-checkpoint validate
- proof submit

Proof CLI should be introduced only after proof model implementation.

## Snapshot CLI commands

Possible commands:

- snapshot export
- snapshot import
- snapshot verify
- snapshot migrate

Snapshot import should require explicit confirmation in production-like environments.

## Inspect CLI commands

Possible commands:

- inspect registry
- inspect registrar
- inspect redeem-events
- inspect xen-burn-events
- inspect replay-counts

These commands help diagnose state without mutating it.

## Error response policy

API and CLI should expose structured errors based on BuildErrorCode.

Recommended shape:

- code
- message
- details
- requestId if API
- command if CLI

Do not rely on raw stack traces for user-facing output.

Stack traces may be logged internally in development only.

## Input validation policy

API and CLI should validate:

- required fields
- string fields
- bigint decimal strings
- positive amounts
- message kind
- ids
- timestamps
- slot values

But final business validation remains in model instructions.

Example:

API can reject malformed amount string.

Model instruction still rejects amount <= 0.

## BigInt input policy

External API / CLI input should accept bigint values as decimal strings.

Reason:

- avoids JavaScript number precision loss
- matches storage serialization policy
- keeps payloads stable

Examples:

- amountBld: "11"
- amountXbp: "100"
- amountXntd: "500"
- feeAmount: "1000"

Parsing must reject:

- empty string
- decimal fractions
- negative values where not allowed
- scientific notation unless explicitly supported
- unsafe JSON numbers for bigint fields

## Output serialization policy

API / CLI output should serialize bigint values as decimal strings.

This keeps output consistent with storage serialization.

## Idempotency policy

Registrar message submission should be idempotent only if explicitly designed.

Current model rejects duplicate messageId.

Future API may return a clear duplicate error.

Do not silently treat duplicates as success unless a separate idempotency policy is created.

## Security policy

API / CLI must not print or expose secrets.

Sensitive values may include:

- registrar private keys
- signer private keys
- API keys
- RPC keys
- database credentials
- seed-like values

Commands and logs should avoid printing secrets.

Environment checks should show only whether a variable exists, not its value.

## Logging policy

Logs should include:

- request / command id
- message kind
- build id
- success / failure status
- error code
- timing

Logs should not include:

- private keys
- raw secrets
- full sensitive environment values

## Testing policy

Future API / CLI tests should cover:

- valid command / request
- malformed input
- invalid bigint string
- missing required field
- duplicate message
- unauthorized registrar
- failed transition does not persist
- successful transition persists
- error shape
- no unrelated accounting value creation

## Recommended implementation order

Recommended order:

1. shared application service layer
2. serialization helpers
3. local file storage adapter
4. CLI read-only commands
5. CLI mutation commands
6. API read-only endpoints
7. API mutation endpoints
8. proof submission endpoints

## Current known exclusions

This milestone does not implement:

- API server
- CLI commands
- application service layer
- serializers
- storage adapter
- proof validators
- authentication
- authorization
- rate limiting
- deployment configuration

## Main invariant

API and CLI should make the model usable.

They must not become a second source of protocol logic.
