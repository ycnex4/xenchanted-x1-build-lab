# Checkpoint: XXXL Manual Account Constraint Audit Checklist

Stage: stage-xxxl-manual-account-constraint-audit-checklist

Status: COMPLETED

## Goal

Record the manual account-constraint checklist required before live handler wiring.

## Completed

- Captured current hard-check baseline.
- Confirmed cargo fmt --check passes.
- Confirmed cargo test passes with 63 tests.
- Confirmed cargo clippy --all-targets -- -D warnings passes.
- Confirmed cargo audit exits 0 with only allowed warnings.
- Confirmed cargo deny licenses/bans/sources exit 0.
- Captured source snapshot for runtime/account audit.
- Captured grep snapshot for account/runtime terms.
- Documented canonical 9-account order.
- Documented program-owned account constraints.
- Documented SPL Token program constraint.
- Documented SPL mint account constraints.
- Documented recipient token account / ATA boundary.
- Documented gateway mint authority PDA constraints.
- Documented route/gateway config constraints.
- Documented guardian set boundary.
- Documented processed-event replay boundary.
- Documented recipient-balance boundary.
- Documented amount constraints.
- Documented atomicity and no-state-change-on-failure policy.
- Documented additional writable/executable constraints required before live activation.

## Decision

This stage is documentation and audit only.

Do not activate the live gateway route.

Do not wire process_instruction into live mint execution in this stage.

Do not move guardian signature verification into runtime in this stage.

Do not change runtime behavior in this stage.

## Current status

The live route remains scaffold-only.

Prepared CPI boundary exists.

SPL mint_to CPI boundary exists.

Atomic mutation fixtures exist.

Manual account constraints are now explicitly recorded before any live handler wiring.

## Next likely stage

A guarded live-handler wiring stage may be considered later, but only if it preserves this checklist and keeps route activation policy explicit.
