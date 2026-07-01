# XXXL X1 Testnet Local Runtime Skeleton Phase 36 Ed25519 Signature Evidence Boundary Spec

Status: Docs-only Ed25519 signature evidence boundary specification.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-36-ed25519-signature-evidence-boundary-spec`

Base context:

- Phase 32 added the read-only Rust/SVM verifier scaffold.
- Phase 33 added the Rust/SVM raw payload decoder.
- Phase 34 added Rust/SVM canonical payload hash/domain validation.
- Phase 35 added guardian membership and quorum structural verification.

## Purpose

Phase 36 defines the Ed25519 signature evidence boundary for the future Rust/SVM
runtime verifier.

It is a docs-only phase.

It does not add runtime code.

It does not implement Ed25519 verification.

It does not parse the Instructions sysvar.

It does not inspect ed25519 program instructions.

It does not unlock runtime execution.

## Preserved Security Decision

The preserved security decision remains:

~~~text
TS layer = preflight / model / watcher-side decision
Runtime = independent verifier
No authorized=true -> execute
~~~

TypeScript authorization output is not runtime authority.

TypeScript parity results are not runtime authority.

Guardian structural quorum is not runtime authority.

Caller-provided signature claims are not runtime authority.

## Selected Future Model

The selected future model is SVM ed25519 instruction evidence:

- a transaction includes one or more ed25519 program instructions;
- each ed25519 instruction verifies one guardian signature;
- the future runtime verifier inspects instruction evidence through the
  Instructions sysvar;
- each counted signature must bind the expected guardian public key to the exact
  Phase 34 canonical payload hash;
- only cryptographically verified and structurally valid guardian approvals may
  contribute toward quorum.

Phase 36 only documents this boundary.

## Required Binding

A future Ed25519 evidence verifier must bind each signature to:

- the expected guardian public key;
- the expected guardian set id;
- the exact Phase 34 canonical payload hash;
- the same message nonce;
- the same route;
- the same recipient;
- the same target mint;
- the same amount;
- the same expiration/finality boundary.

A signature over different bytes is not valid evidence.

A signature over a caller-provided hash is not valid evidence unless the runtime
recomputes the same hash through the Phase 34 canonical payload hash validator.

## Relationship To Phase 35

Phase 35 structural quorum answers only:

~~~text
Are these guardian public keys known, unique, and enough to meet threshold?
~~~

Phase 36 defines the future evidence boundary for answering:

~~~text
Did those guardians cryptographically sign the exact Phase 34 canonical payload hash?
~~~

Both are required.

A structurally valid quorum without Ed25519 evidence is not enough to authorize
minting.

Ed25519 evidence without guardian membership and quorum is also not enough.

Phase 36 alone cannot make `authorized=true`.

## Files Added Or Changed

Added:

- `docs/xxxl/xxxl-phase-36-ed25519-signature-evidence-boundary-spec.md`
- `docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-36-ed25519-signature-evidence-boundary-spec.md`

Changed:

- `docs/checkpoints/current-design-checkpoint.md`

No Rust source file is changed.

No TypeScript source file is changed.

No TypeScript test file is changed.

No Cargo file is changed.

No package manifest or lockfile is changed.

No dependency is added.

No `programs/xxxl-svm/src/lib.rs` change is required.

## Explicit Non-Goals

Phase 36 does not implement Ed25519 verification.

Phase 36 does not parse ed25519 instruction data.

Phase 36 does not parse the Instructions sysvar.

Phase 36 does not add an instruction handler.

Phase 36 does not parse runtime accounts.

Phase 36 does not implement source proof verification.

Phase 36 does not implement route config verification.

Phase 36 does not implement target mint account legitimacy verification.

Phase 36 does not implement amount cap enforcement.

Phase 36 does not implement replay storage.

Phase 36 does not implement replay checks.

Phase 36 does not implement replay writes.

Phase 36 does not enable live route execution.

Phase 36 does not enable SPL CPI.

Phase 36 does not enable `invoke_signed`.

Phase 36 does not enable SPL Token `mint_to`.

Phase 36 does not add mint execution.

Phase 36 does not mutate runtime/account state.

Phase 36 does not enable processed-event marking.

Phase 36 does not select a production Program ID.

Phase 36 does not regenerate production PDA fixtures.

Phase 36 does not remove deployment blockers.

Phase 36 does not claim production readiness.

Phase 36 does not claim final immutability while upgrade authority exists.

Phase 36 does not build SBF artifacts.

Phase 36 does not touch `target/deploy`.

Phase 36 does not read or modify keypair files.

Phase 36 does not read or modify `.env`.

Phase 36 does not inspect `.local-keys`.

Phase 36 does not run deploy commands.

Phase 36 does not run network commands.

Phase 36 does not spend SOL.

## Active Blockers Preserved

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

## Validation

Commands to run:

- `git diff --check`
- `npm run typecheck`
- `npm run build`
- `git status --short --untracked-files=all`

No Rust source file was changed.

No Cargo manifest was changed.

No Cargo lockfile was changed.

No SBF build was run.

No deploy or network command was run.

Recommended next stage:

- Phase 37 Rust/SVM Ed25519 instruction evidence layout model, still read-only,
  still without runtime unlock.
