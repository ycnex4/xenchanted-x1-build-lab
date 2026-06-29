# XXXL Real Program ID Selection Procedure

Status: Completed
Branch: `stage-xxxl-real-program-id-selection-procedure`
Base: `abda874 Add XXXL real Program ID readiness plan`

## Purpose

This document defines the procedure for selecting and recording the future real XXXL SVM Program ID.

This is a procedure stage only.

This stage does not select a real Program ID.

This stage does not record a final Program ID.

This stage does not replace the placeholder Program ID.

This stage does not regenerate production PDA fixtures.

This stage does not verify production PDA fixtures.

This stage does not remove `PLACEHOLDER_PROGRAM_ID`.

This stage does not remove any deployment blocker.

This stage does not make the runtime deployable.

## Current State

The runtime still exposes a placeholder Program ID boundary.

Current placeholder:

- `XXXL_PROGRAM_ID_PLACEHOLDER`
- `XXXLProgram111111111111111111111111111111111`

Current Program ID readiness state:

- status: `Placeholder`
- status code: `PLACEHOLDER_PROGRAM_ID_BOUNDARY`
- deployable path ready: `false`
- linked blocker: `PLACEHOLDER_PROGRAM_ID`

Current Program-ID-dependent PDA inventory:

- `gateway_mint_authority`

The `gateway_mint_authority` PDA depends on the final Program ID.

Therefore, the final Program ID must be selected before production PDA fixtures can be finalized.

## Procedure Goal

The goal of this procedure is to make the future Program ID selection auditable, reproducible, and safe.

A future Program ID selection must clearly answer:

- what Program ID was selected
- why it is the final Program ID
- who or what process selected it
- where it is recorded
- why it is not the placeholder value
- why it is not a local-only fixture value
- which PDA fixtures must be regenerated from it
- which blockers remain active after it is recorded

## Selection Preconditions

Before a future `stage-xxxl-real-program-id-selection-record` may be created, the following must be true:

1. the intended final Program ID is known
2. the Program ID is available as an exact string
3. the Program ID is not `XXXLProgram111111111111111111111111111111111`
4. the Program ID is not `11111111111111111111111111111111`
5. the Program ID is not a local fixture such as `BPFLoaderUpgradeab1e11111111111111111111111`
6. the Program ID is tied to the intended runtime artifact or deployment authority path
7. the selection can be reviewed independently
8. the repository can record the selection without exposing private keys or deployment secrets

## Forbidden Selection Values

The future real Program ID must not be any of the following:

- `XXXLProgram111111111111111111111111111111111`
- `11111111111111111111111111111111`
- `BPFLoaderUpgradeab1e11111111111111111111111`
- SPL Token Program ID
- System Program ID
- any test-only fixture Program ID
- any placeholder value used only for local derivation examples
- any value that is not tied to the intended XXXL SVM runtime

## Required Selection Record

A future selection record must include:

- final Program ID string
- branch name
- commit SHA
- date of selection
- selection source
- reviewer or review source
- statement that no private key is disclosed
- statement that no deployment secret is disclosed
- statement that the selected value is not a placeholder
- statement that the selected value is not a local fixture
- statement that Program-ID-dependent PDA fixtures must be regenerated after selection

## Required Repository Changes In Selection Record Stage

The future selection record stage should be docs-first.

It may add:

- `docs/xxxl/xxxl-real-program-id-selection-record.md`
- `docs/checkpoints/xxxl-real-program-id-selection-record.md`
- update to `docs/checkpoints/current-design-checkpoint.md`

It should not immediately remove `PLACEHOLDER_PROGRAM_ID`.

It should not immediately update PDA fixtures.

It should not immediately update `program_id_status.rs` unless the stage explicitly includes code changes and tests proving the updated readiness model.

## Required Verification Before Recording

Before recording the real Program ID, the future stage must verify:

- Program ID string is present
- Program ID string is valid for SVM/Solana pubkey use
- Program ID string differs from the current placeholder
- Program ID string differs from local fixture Program IDs
- Program ID string differs from SPL Token Program ID
- Program ID string differs from System Program ID
- no private key material appears in the record
- no deployment secret appears in the record

## Required PDA Follow-Up

After Program ID selection is recorded, the next PDA-related stages must:

1. regenerate `gateway_mint_authority` production PDA fixture from the final Program ID
2. record the derived PDA and bump
3. verify the fixture against the final Program ID
4. reject wrong Program ID
5. reject wrong PDA
6. reject wrong bump
7. reject wrong name
8. reject wrong kind
9. reject wrong report count

No production PDA fixture should be accepted before the final Program ID is recorded.

## Required Blocker Preservation

The Program ID selection record stage must preserve deployment safety.

Recording a Program ID alone must not remove:

- `PLACEHOLDER_PROGRAM_ID`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

`PLACEHOLDER_PROGRAM_ID` may only be removed in a later dedicated transition stage after:

- final Program ID is selected
- production PDA fixtures are regenerated
- production PDA fixtures are verified
- Program ID readiness model is updated
- safety invariants are updated
- tests prove the blocker is absent
- tests prove all remaining blockers stay active
- tests prove runtime remains not deployable

## Required Safety Statements

A future selection record must explicitly state:

- this records Program ID selection only
- this does not deploy the program
- this does not activate live route execution
- this does not enable SPL CPI execution
- this does not enable `invoke_signed`
- this does not enable SPL Token `mint_to`
- this does not configure production guardians
- this does not configure production proof logs
- this does not complete external review
- this does not make the runtime deployable

## Non-Goals

This procedure does not authorize:

- deployment
- runtime release
- live route activation
- SPL CPI execution
- PDA fixture finalization
- guardian production configuration
- proof-log production configuration
- external review closure
- removal of any deployment blocker

## Recommended Next Stage

After this procedure, the next stage depends on whether the final Program ID is known.

If the final Program ID is known:

- `stage-xxxl-real-program-id-selection-record`

If the final Program ID is not known:

- pause Program ID transition work
- continue with non-Program-ID preparation work
- avoid any fake Program ID record

## Result

This stage defines the procedure for selecting the future real Program ID.

No Program ID is selected.

No blocker is removed.

No blocker is transitioned.

`PLACEHOLDER_PROGRAM_ID` remains active.

The runtime remains scaffold-only, locked, unreleasable, and not deployable.
