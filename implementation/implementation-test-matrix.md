# Implementation Test Matrix

## Branch

implementation-test-matrix

## Purpose

This document maps the current MVP implementation to its test coverage.

The goal is to make it easy to see:

- which model layers are implemented
- which test files cover each layer
- which invariants are currently protected
- which areas remain outside MVP scope

This milestone is documentation-only.

No TypeScript model logic is changed in this branch.

## Current validation baseline

At the start of this milestone:

- npm run typecheck: passed
- npm test: passed
- 16 test files passed
- 96 tests passed

## Test matrix

| Area | Implementation files | Test files | Covered properties |
| --- | --- | --- | --- |
| Scaffold | project setup | tests/scaffold.test.ts | test runner sanity |
| BuildState defaults | src/model/build-state.ts | tests/build-state.test.ts | initial field values, accounting defaults, nullable checkpoint fields |
| createBuild | src/instructions/create-build.ts | tests/create-build.test.ts | Build creation, owner/buildId/timestamps, no accidental accounting value |
| Build registry | src/model/build-registry.ts | tests/build-registry.test.ts | duplicate buildId, duplicate owner, duplicate Ethereum identity, empty registry |
| Registrar replay | src/model/registrar.ts | tests/registrar-replay.test.ts | authority check, processedMessages replay protection, rejected messages do not mutate |
| Core redeem BLD | src/instructions/core-redeem.ts | tests/core-redeem-bld.test.ts | positive BLD validation, historyBld accumulation, availableBld accumulation, unrelated layers unchanged |
| Redeem event replay | src/model/redeem-events.ts | tests/redeem-event-replay.test.ts | usedRedeemEvents replay protection, duplicate redeemKey blocked, invalid BLD does not mark key |
| Registrar CORE_REDEEM | src/instructions/registrar-core-redeem.ts | tests/registrar-core-redeem.test.ts | registrar validation, CORE_REDEEM kind check, message replay, redeem event replay, non-mutating failures |
| XEN Burn Power | src/instructions/xen-burn-power.ts | tests/xen-burn-power.test.ts | positive XBP validation, earnedXbp accumulation, availableXbp accumulation, unrelated layers unchanged |
| XEN burn event replay | src/model/xen-burn-events.ts | tests/xen-burn-event-replay.test.ts | usedXenBurnEvents replay protection, duplicate xenBurnKey blocked, invalid XBP does not mark key |
| Registrar XEN_BURN | src/instructions/registrar-xen-burn.ts | tests/registrar-xen-burn.test.ts | registrar validation, XEN_BURN kind check, message replay, XEN burn event replay, non-mutating failures |
| Genesis Origin BLD | src/instructions/genesis-origin-bld.ts | tests/genesis-origin-bld.test.ts | tier calculation, one-time claim, not-eligible rejection, availableBld increase, historyBld unchanged |
| XNTD lock / relock | src/instructions/xntd-lock.ts | tests/xntd-lock-relock.test.ts | positive lock amount, active commitment, relock integrity availableBld >= historyBld, unrelated layers unchanged |
| Registrar LOCK_XNTD / RELOCK_XNTD | src/instructions/registrar-xntd-lock.ts | tests/registrar-xntd-lock.test.ts | registrar validation, message kinds, message replay, lock/relock application, non-mutating failures |
| X1 Fee Contribution | src/instructions/x1-fee-contribution.ts | tests/x1-fee-contribution.test.ts | positive fee amount, positive tx count, increasing slot, fee/tx accumulation, unrelated layers unchanged |
| Registrar X1_FEE_CHECKPOINT | src/instructions/registrar-x1-fee-checkpoint.ts | tests/registrar-x1-fee-checkpoint.test.ts | registrar validation, message kind, message replay, fee checkpoint application, non-mutating failures |

## Cross-cutting invariants covered

### No unrelated value creation

Current tests repeatedly verify that transitions do not create unrelated accounting values.

Examples:

- Core redeem does not create XBP, XNTD lock state, or X1 fee contribution.
- XEN burn does not create BLD, XNTD lock state, or X1 fee contribution.
- XNTD lock / relock does not create BLD, XBP, or X1 fee contribution.
- X1 fee checkpoints do not create BLD, XBP, or XNTD commitment.
- Genesis Origin does not create XBP, XNTD lock state, or X1 fee contribution.

### Replay protection

Current tests cover replay protection for:

- registrar messageId
- Core redeem event key
- XEN burn event key

### Atomicity

Current registrar integration tests verify that invalid or rejected messages do not partially mutate state.

Covered failure types include:

- wrong message kind
- unauthorized registrar
- duplicate registrar message
- duplicate event key
- invalid amount
- invalid relock state
- non-increasing fee checkpoint slot

## Known MVP gaps

The current test matrix does not yet cover real external proof validation.

Still out of scope:

- Ethereum XEN burn log proof validation
- Core redeem proof validation
- XNTD lock escrow proof validation
- X1 fee transaction proof validation
- registrar signature validation
- Merkle proof verification
- bridge proof verification
- persistent storage behavior
- serialization / deserialization behavior
- API / CLI integration
- concurrency / race-condition behavior outside in-memory model assumptions

## Recommended next tests after MVP proof design

Future test matrix extensions should include:

1. proof key derivation tests
2. proof replay tests
3. malformed proof rejection tests
4. storage round-trip tests
5. serialization boundary tests
6. API request validation tests
7. CLI command behavior tests
8. end-to-end scenario tests

## Main conclusion

The current MVP implementation has coherent unit coverage for all implemented in-memory state transitions.

The test suite validates the most important current invariant:

No accepted transition should create value in an unrelated accounting layer.
