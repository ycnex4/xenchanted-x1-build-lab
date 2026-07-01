# XXXL Phase 35 Rust/SVM Guardian Quorum Structural Verifier

Status: Narrow Rust/SVM guardian membership and quorum structural verifier.

## Purpose

Phase 35 adds a bounded Rust/SVM verifier helper for guardian set membership
and quorum counting.

It follows the Phase 24/25 TypeScript guardian approval model only at the
structural layer.

Phase 35 does not implement Ed25519 cryptographic signature verification.

Phase 35 does not unlock runtime execution.

## Inputs

Phase 35 depends on these prior boundaries:

- Phase 31 docs-only runtime verifier boundary specification.
- Phase 32 read-only Rust/SVM verifier scaffold.
- Phase 33 Rust/SVM raw payload decoder.
- Phase 34 Rust/SVM canonical payload hash/domain validation.
- TypeScript guardian approval and quorum model in
  `src/xxxl/guardian-approval-verifier.ts`.

Phase 34 remains the canonical payload hash source:

~~~text
keccak256(keccak256("XXXL_GUARDIAN_PAYLOAD_HASH_V1") || payload_bytes)
~~~

Phase 35 does not call the Phase 34 hash helper in its quorum path.

Phase 35 only reports that the Phase 34 hash validator remains available and
that it recomputes caller hashes.

## Source Boundary

New Rust module:

- `programs/xxxl-svm/src/verifier/guardian_quorum.rs`

Updated Rust verifier export:

- `programs/xxxl-svm/src/verifier/mod.rs`

No `programs/xxxl-svm/src/lib.rs` change is required.

No Cargo manifest change is required.

No dependency is added.

## Verifier API

The Phase 35 marker is:

~~~text
GUARDIAN_QUORUM_STRUCTURAL_VERIFIER_PHASE_35
~~~

The verifier version is:

~~~text
1
~~~

The exposed Rust API includes:

- `GuardianPublicKey`
- `GuardianSetRef`
- `GuardianApprovalClaim`
- `GuardianApprovalRef`
- `GuardianQuorumStructuralResult`
- `GuardianQuorumStructuralError`
- `GuardianQuorumStructuralErrorKind`
- `GuardianQuorumStructuralReport`
- `verify_guardian_quorum_structural`
- `guardian_quorum_structural_report`

The approval type is a structural guardian approval claim.

It does not carry signature bytes.

It does not claim cryptographic proof.

## Structural Behavior

The verifier rejects:

- empty guardian set
- threshold equal to zero
- threshold greater than guardian set size
- duplicate guardian public keys in the guardian set
- empty approvals
- approval guardian set id mismatch
- unknown guardian public key
- duplicate guardian public key approval
- not enough unique known approvals to meet threshold

The verifier accepts:

- unique known guardian approvals where count is equal to the threshold
- unique known guardian approvals where count is greater than the threshold

The success result returns:

- guardian set id
- threshold
- guardian count
- unique known approval count
- quorum reached flag
- Ed25519 signature verification performed flag set to false
- cryptographic signature proof accepted flag set to false

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Caller-provided signatures are not accepted as cryptographic proof in Phase 35.

A quorum of structurally known guardians is not enough to authorize minting.

A future phase must prove cryptographic signatures over the Phase 34 canonical
payload hash.

Phase 35 alone cannot make `authorized=true`.

## Explicit Non-Goals

Phase 35 does not implement Ed25519 verification.

Phase 35 does not parse signature bytes.

Phase 35 does not implement instruction sysvar parsing.

Phase 35 does not implement ed25519 program instruction validation.

Phase 35 does not implement source proof verification.

Phase 35 does not implement route config verification.

Phase 35 does not implement target mint account legitimacy verification.

Phase 35 does not implement amount cap enforcement.

Phase 35 does not implement replay storage.

Phase 35 does not implement replay checks.

Phase 35 does not implement replay writes.

Phase 35 does not parse runtime accounts.

Phase 35 does not add an instruction handler.

Phase 35 does not enable live route execution.

Phase 35 does not enable SPL CPI.

Phase 35 does not enable `invoke_signed`.

Phase 35 does not enable SPL Token `mint_to`.

Phase 35 does not add mint execution.

Phase 35 does not mutate runtime/account state.

Phase 35 does not mark processed events.

Phase 35 does not select a production Program ID.

Phase 35 does not regenerate production PDA fixtures.

Phase 35 does not remove deployment blockers.

Phase 35 does not claim production readiness.

Phase 35 does not claim final immutability while upgrade authority exists.

Phase 35 does not change Cargo manifests.

Phase 35 does not change package manifests.

Phase 35 does not build SBF artifacts.

Phase 35 does not touch `target/deploy`.

Phase 35 does not read or modify keypair files.

Phase 35 does not read or modify `.env`.

Phase 35 does not inspect `.local-keys`.

Phase 35 does not run deploy commands.

Phase 35 does not run network commands.

Phase 35 does not spend SOL.

## Preserved Blockers

The following blockers remain active:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

No blocker is removed, weakened, renamed, or satisfied by Phase 35.
