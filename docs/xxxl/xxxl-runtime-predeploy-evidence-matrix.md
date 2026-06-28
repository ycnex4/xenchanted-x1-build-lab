# XXXL Runtime Predeploy Evidence Matrix

Status: DRAFTED / BLOCKING.

This document maps every current XXXL runtime deployment blocker to the evidence required before the blocker can be removed.

It does not activate deployment.

It does not change runtime behavior.

It is a review and readiness matrix for future deployment-readiness stages.

## Current status

Runtime status:

    SCAFFOLD_ONLY_NOT_DEPLOYABLE

Current gate result:

    Blocked(report)

Current deploy allow value:

    false

Current blocker count:

    6

## Evidence rule

A blocker may only be removed when:

- the required evidence exists
- the evidence is linked from the relevant checkpoint
- the evidence has passed hard checks where code is involved
- the change is isolated in a reviewed stage
- the blocker removal is explicitly documented
- the predeploy gate remains blocked unless all blockers are removed and deployability is intentionally changed

## Matrix

| Blocker | Required evidence | Evidence artifact | Current status |
|---|---|---|---|
| `PLACEHOLDER_PROGRAM_ID` | Real Program ID selected, reviewed, documented, and all Program-ID-dependent PDA fixtures regenerated. | Future Program ID / PDA fixture checkpoint. | BLOCKED |
| `LIVE_ROUTE_DISABLED` | Live route activation design reviewed; activation isolated; negative tests prove invalid inputs fail before mutation/CPI. | Future live route activation checkpoint. | BLOCKED |
| `SPL_CPI_EXECUTION_DISABLED` | SPL Token `mint_to` CPI path reviewed; PDA authority validated; Mollusk positive/negative CPI coverage complete. | Future SPL CPI execution checkpoint. | BLOCKED |
| `PRODUCTION_GUARDIAN_SET_UNSET` | Guardian set, threshold, custody model, rotation policy, and emergency replacement policy documented. | Future production guardian policy checkpoint. | BLOCKED |
| `PRODUCTION_PROOF_LOG_UNSET` | Proof-log schema, retention policy, public audit trail, and publication flow documented. | Future production proof-log checkpoint. | BLOCKED |
| `EXTERNAL_REVIEW_INCOMPLETE` | External review scope complete; notes archived; findings resolved or explicitly accepted. | Future external review checkpoint. | BLOCKED |

## Evidence detail

### PLACEHOLDER_PROGRAM_ID

Required evidence:

- real Program ID selected
- Program ID documented
- Program-ID-dependent PDA fixtures regenerated
- tests updated to use final Program ID where required
- placeholder value remains impossible in deployable paths
- reviewer confirms Program ID boundary

Current status:

    BLOCKED

### LIVE_ROUTE_DISABLED

Required evidence:

- live route activation design documented
- live route activation isolated in one reviewed stage
- invalid instruction data still fails before state mutation
- invalid accounts still fail before state mutation
- invalid authorization still fails before state mutation or CPI
- replay still fails before state mutation or CPI
- reviewer confirms no hidden activation switch

Current status:

    BLOCKED

### SPL_CPI_EXECUTION_DISABLED

Required evidence:

- SPL Token mint account contract documented
- recipient token account validation documented
- mint authority PDA derivation reviewed
- signer seeds reviewed
- CPI instruction account order reviewed
- Mollusk positive CPI coverage added
- Mollusk negative CPI coverage added
- unauthorized signer and writable expansion tests pass
- reviewer confirms no bypass path exists

Current status:

    BLOCKED

### PRODUCTION_GUARDIAN_SET_UNSET

Required evidence:

- guardian identities or operator model documented
- threshold documented
- bootstrap trust model documented
- production trust model documented
- key custody model documented
- rotation policy documented
- emergency replacement policy documented
- watcher/operator responsibilities documented

Current status:

    BLOCKED

### PRODUCTION_PROOF_LOG_UNSET

Required evidence:

- proof-log schema documented
- canonical event key included
- Ethereum burn evidence included
- finality evidence included
- gateway message hash included
- guardian approvals included or linkable
- X1 mint evidence included
- retention policy documented
- public publication flow documented

Current status:

    BLOCKED

### EXTERNAL_REVIEW_INCOMPLETE

Required evidence:

- review scope documented
- reviewer notes archived
- live route reviewed
- SPL CPI path reviewed
- account contract reviewed
- replay protection reviewed
- guardian policy reviewed
- proof-log model reviewed
- deployment checklist reviewed
- findings resolved or explicitly accepted

Current status:

    BLOCKED

## Non-removal rule

A blocker must not be removed merely because code compiles.

A blocker is removed only when its evidence is complete and the corresponding checkpoint says it is complete.

## Safety boundary

No live route was activated.

No SPL CPI behavior was enabled.

No `invoke_signed` path was enabled.

No minting was enabled.

No deployment behavior was enabled.

No deployability predicate was changed.

The runtime remains scaffold-only and not deployable.

## Verification required for future blocker removal

For any blocker-removal stage involving code, the following checks are required:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

## Decision

The evidence matrix is the required evidence map for future blocker-removal stages.

All blockers remain active.
