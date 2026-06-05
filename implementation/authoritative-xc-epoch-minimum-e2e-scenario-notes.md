# Authoritative XC Epoch Minimum E2E Scenario Notes

## Branch

authoritative-xc-epoch-minimum-e2e-scenario

## Purpose

This milestone wires the deterministic authoritative XC epoch minimum source into the end-to-end watcher proof registrar scenario.

It verifies the full test runtime path:

watcher candidate
-> proof conversion
-> appSubmitProof(..., xcEpochMinimumSource)
-> app service
-> registrar handler
-> assertAuthoritativeXcEpochMinimum()
-> Build state

## Updated test

Updated:

- tests/e2e-watcher-proof-registrar-scenario.test.ts

## Runtime code changes

This milestone does not change runtime code.

It only strengthens the existing e2e test scenario.

## Test source

The e2e scenario now creates a deterministic static source:

- epoch 1 -> 500n
- epoch 2 -> 250n

This source is passed into appSubmitProof() for:

- XNTD_LOCK_PROOF
- XNTD_RELOCK_PROOF

## Verified path

The existing e2e scenario now validates that XNTD lock / relock proof submission can flow through:

1. watcher candidate creation
2. proof conversion
3. appSubmitProof()
4. app service wrapper
5. registrar lock / relock handler
6. authoritative epoch minimum validation
7. Build state mutation

## Compatibility

Non-XNTD proof submissions in the same scenario are unchanged:

- CORE_REDEEM
- XEN_BURN
- X1_FEE_CHECKPOINT

## Scope boundary

This milestone does not update:

- app runtime code
- proof submission runtime code
- registrar runtime code
- snapshot schema
- storage serialization
- CLI output
- real Ethereum RPC integration
- XC Core / Lens ABI integration
- persistent app state source ownership

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 30 test files passed
- 194 tests passed

## Next step

After this milestone, the authoritative XC epoch minimum validation chain is covered from watcher candidate to Build state in tests.

A later production-readiness layer can decide how the authoritative source is created in real integration:

- trusted integration source
- finalized Ethereum RPC / Lens read
- checkpoint source
- bridge-provided source
- X1-native verified source

Do not introduce real RPC or persistent source ownership in this e2e test milestone.
