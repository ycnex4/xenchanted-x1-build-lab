# XXXL X1 Testnet Local Runtime Skeleton Phase 24 Guardian Signature Quorum Boundary

Status: TypeScript-only guardian approval and quorum verifier boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-24-guardian-signature-quorum-boundary`

Base:

- `a0fcc16 Merge XXXL phase 23 guardian payload encoding vectors`

## Purpose

Phase 24 implements a TypeScript-only XXXL guardian approval and quorum verifier
over the Phase 23 guardian payload hash.

Guardians sign the 32-byte Phase 23 payload hash.

Guardians do not sign the full Phase 23 hash preimage in this phase.

The verifier computes the payload hash from `XXXLGuardianPayloadFields` using
`hashXxxlGuardianPayload(fields)`.

The verifier does not accept a caller-supplied payload hash as authoritative.

Phase 24 adds signature and quorum verification only for the TypeScript vector
surface.

Phase 24 does not enable live route execution.

Phase 24 does not mutate runtime/account state.

## Files Added Or Changed

Added:

- `src/xxxl/guardian-approval-verifier.ts`
- `tests/xxxl/guardian-approval-verifier.test.ts`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-24-guardian-signature-quorum-boundary.md`

Changed:

- `src/index.ts`
- `docs/checkpoints/current-design-checkpoint.md`

No Phase 23 encoding file is changed.

No SVM runtime source file is changed.

No SVM runtime test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

## API Surface

Phase 24 exports:

- `XXXL_GUARDIAN_APPROVAL_ERROR`
- `type XXXLGuardianApprovalErrorCode`
- `type XXXLGuardianApproval`
- `type XXXLGuardianQuorumConfig`
- `type XXXLGuardianPayloadApprovalVerificationInput`
- `type XXXLGuardianApprovalResult`
- `type XXXLGuardianQuorumVerificationInput`
- `type XXXLGuardianQuorumVerificationResult`
- `validateXxxlGuardianQuorumConfig(config)`
- `verifyXxxlGuardianPayloadApproval(input)`
- `verifyXxxlGuardianPayloadQuorum(input)`

## Verification Behavior

For each approval, Phase 24:

- computes `payloadHash = hashXxxlGuardianPayload(fields)`
- converts `payloadHash` hex into exactly 32 bytes
- verifies the Ed25519 signature over those 32 payload hash bytes
- rejects malformed public keys
- rejects malformed signatures
- rejects invalid signatures

For quorum verification, Phase 24:

- rejects an empty guardian set
- rejects non-integer thresholds
- rejects thresholds `<= 0`
- rejects thresholds greater than guardian count
- rejects duplicate guardian public keys in the configured set
- rejects guardian public keys with length other than 32 bytes
- rejects guardian signatures with length other than 64 bytes
- rejects unknown guardian approvals
- rejects duplicate guardian approvals
- counts only accepted, known, non-duplicate, valid-signature approvals
- preserves the first valid approval count when a later duplicate is rejected
- reports `QUORUM_NOT_REACHED` when accepted approvals are below threshold

Payload field validation errors from Phase 23 are caught and returned as
`INVALID_PAYLOAD` instead of crashing verifier callers.

## Explicit Non-Goals

Phase 24 does not modify `programs/xxxl-svm/src`.

Phase 24 does not modify `programs/xxxl-svm/tests`.

Phase 24 does not modify `Cargo.toml`.

Phase 24 does not modify `Cargo.lock`.

Phase 24 does not modify `package.json`.

Phase 24 does not modify `package-lock.json`.

Phase 24 does not add dependencies.

Phase 24 does not run `npm install`.

Phase 24 does not run `cargo build-sbf`.

Phase 24 does not touch `target/deploy`.

Phase 24 does not inspect or touch keypair files.

Phase 24 does not inspect or touch `.local-keys`.

Phase 24 does not read `.env`.

Phase 24 does not add deploy commands.

Phase 24 does not add upgrade commands.

Phase 24 does not add Solana/network actions.

Phase 24 does not enable live route execution.

Phase 24 does not enable SPL CPI execution.

Phase 24 does not enable `invoke_signed`.

Phase 24 does not enable SPL Token `mint_to`.

Phase 24 does not change Phase 23 payload encoding.

Phase 24 does not derive `canonical_event_key` from `source_chain_id`.

Phase 24 does not mutate runtime/account state.

Phase 24 does not claim production readiness.

Phase 24 does not claim final immutability while upgrade authority exists.

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

Enabled `process_instruction` remains a disabled-plan no-op for live atomicity.

## Validation

Commands run:

- `npm run build`: passed
- `npm test -- --run`: passed, 98 test files passed, 882 tests passed

Required final workspace checks:

- `git diff --check`
- `git status --short --untracked-files=all`

No Cargo validation was run.

No SBF build was run.

No Solana command was run.
