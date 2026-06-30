# XXXL X1 Testnet Local Runtime Skeleton Phase 25 Verifier Runtime Authorization Boundary

Status: TypeScript-only verifier-to-runtime authorization boundary model.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-25-verifier-runtime-authorization-boundary`

Base:

- `777a8e8 Merge XXXL phase 24 guardian signature quorum verifier`

## Purpose

Phase 25 composes the Phase 23 guardian payload encoding layer and the Phase 24
guardian signature/quorum verifier into a pure authorization decision model.

The model answers whether a guardian-approved payload can be authorized for a
future runtime mint boundary when supplied with:

- `XXXLGuardianPayloadFields`
- guardian quorum config
- guardian approvals
- current slot or current unix timestamp
- processed canonical event registry snapshot
- expected route/runtime bindings

Phase 25 models the authorization decision only.

Phase 25 does not mint.

Phase 25 does not mutate runtime state.

Phase 25 does not mark processed events.

Phase 25 does not call SPL CPI.

Phase 25 does not touch SVM code.

## Files Added Or Changed

Added:

- `src/xxxl/gateway-authorization-boundary.ts`
- `tests/xxxl/gateway-authorization-boundary.test.ts`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-25-verifier-runtime-authorization-boundary.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No Phase 23 encoding file is changed.

No Phase 24 verifier file is changed.

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## API Surface

Phase 25 exports:

- `XXXL_GATEWAY_AUTHORIZATION_ERROR`
- `type XXXLGatewayAuthorizationErrorCode`
- `XXXL_GATEWAY_AUTHORIZATION_STATUS`
- `type XXXLGatewayAuthorizationStatus`
- `type XXXLGatewayProcessedRegistrySnapshot`
- `type XXXLGatewayRuntimeBindingExpectations`
- `type XXXLGatewayAuthorizationInput`
- `type XXXLGatewayReplayCheckResult`
- `type XXXLGatewayExpirationCheckResult`
- `type XXXLGatewayRouteBindingCheckResult`
- `type XXXLGatewayAuthorizationResult`
- `authorizeXxxlGatewayMintBoundary(input)`

## Authorization Checks

Phase 25 internally runs `verifyXxxlGuardianPayloadQuorum`.

Phase 25 does not trust caller-supplied payload hashes.

Phase 25 requires guardian quorum result `ok`.

Phase 25 rejects if `canonical_event_key` is already present in the supplied
processed registry snapshot.

Phase 25 rejects if `currentTimeOrSlot > expiration_slot_or_unix_ts`.

Phase 25 accepts the exact expiration boundary when
`currentTimeOrSlot == expiration_slot_or_unix_ts`.

Phase 25 checks expected route/runtime bindings:

- `route_id`
- `source_chain_id`
- `target_mint`
- `guardian_set_id`
- `source_chain_weight_bps`

Phase 25 relies on Phase 23 validation for positive `burned_amount` and
positive `xxxl_mint_amount`.

Payload validation failures are returned as `INVALID_PAYLOAD` and fail closed.

## Result Shape

The authorization result includes:

- `ok`
- `payloadHash`
- `authorizationStatus`
- `errors`
- nested quorum result
- replay check result
- expiration check result
- route binding check result

The result explicitly distinguishes:

- authorization decision
- future runtime mutation
- processed-event marking
- mint execution

## Explicit Non-Goals

Phase 25 does not modify `programs/xxxl-svm/src`.

Phase 25 does not modify `programs/xxxl-svm/tests`.

Phase 25 does not modify `Cargo.toml`.

Phase 25 does not modify `Cargo.lock`.

Phase 25 does not modify `package.json`.

Phase 25 does not modify `package-lock.json`.

Phase 25 does not add dependencies.

Phase 25 does not run `npm install`.

Phase 25 does not run `cargo build-sbf`.

Phase 25 does not touch `target/deploy`.

Phase 25 does not inspect or touch keypair files.

Phase 25 does not inspect or touch `.local-keys`.

Phase 25 does not read `.env`.

Phase 25 does not add deploy commands.

Phase 25 does not add upgrade commands.

Phase 25 does not add Solana/network actions.

Phase 25 does not enable live route execution.

Phase 25 does not enable SPL CPI execution.

Phase 25 does not enable `invoke_signed`.

Phase 25 does not enable SPL Token `mint_to`.

Phase 25 does not change Phase 23 payload encoding.

Phase 25 does not change Phase 24 signature/quorum verifier behavior.

Phase 25 does not derive `canonical_event_key` from `source_chain_id`.

Phase 25 does not mutate runtime/account state.

Phase 25 does not mark processed events.

Phase 25 does not claim production readiness.

Phase 25 does not claim final immutability while upgrade authority exists.

## Safety Status

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

`source_chain_weight_bps` remains signed and dual-source:

- guardian payload field
- runtime instruction field
- `GatewayConfig` binding field

`canonical_event_key` is not derived from `source_chain_id`.

GatewayConfig layout is unchanged.

Live route remains disabled.

SPL CPI remains disabled.

`invoke_signed` remains disabled.

SPL Token `mint_to` remains disabled.

Enabled `process_instruction` remains a disabled-plan no-op for live atomicity.

## Validation

Commands run:

- `npm run build`: passed
- `npm test -- --run`: passed, 99 test files passed, 899 tests passed

Required final workspace checks:

- `git diff --check`
- `git status --short --untracked-files=all`

No Cargo validation was run.

No SBF build was run.

No Solana command was run.
