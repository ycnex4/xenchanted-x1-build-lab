# XXXL Runtime Serialization Boundary Checkpoint

Stage XXXL Program v1 has entered runtime implementation planning.

New files:

- `src/xxxl/runtime-serialization-boundary.ts`
- `tests/xxxl/runtime-serialization-boundary.test.ts`
- `docs/xxxl/xxxl-runtime-serialization-boundary.md`

Purpose:

- start the runtime implementation track
- define account/instruction serialization boundary
- capture Theo's five non-blocking runtime-stage gaps
- prepare for runtime skeleton and deterministic vectors

Boundary decisions:

- account serialization encoding: CANONICAL_BINARY_V1
- instruction serialization encoding: CANONICAL_BINARY_V1
- serialized account kinds:
  - MINT_STATE
  - GATEWAY_CONFIG
  - GUARDIAN_SET
  - PROCESSED_EVENT
  - RECIPIENT_BALANCE
- serialized instruction:
  - CONSUME_GATEWAY_MINT
- mint authority PDA strategy:
  - GATEWAY_MINT_AUTHORITY_PDA
- mint authority PDA seeds:
  - xxxl
  - gateway-mint-authority
  - v1

Explicit runtime notes:

- CPI into SPL Token is atomic with the parent SVM transaction.
- Program upgrade authority and SPL Token mint authority are distinct authority surfaces.
- Authority freeze must cover both authority surfaces distinctly.
- XXXL runtime does not re-verify guardian signatures.
- XXXL runtime consumes the Stage 1 authorization result.
- Runtime skeleton should include a read-only supply audit function.

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 75 files / 526 tests passing
- Build: passing

Status:

- runtime serialization boundary only
- no final byte layout yet
- no live runtime program
- no RPC usage
- no secrets required
