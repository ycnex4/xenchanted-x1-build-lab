# XXXL Runtime Instruction Serialization Vectors Checkpoint

Stage XXXL Program v1 now has deterministic runtime instruction serialization vector planning.

New files:

- `src/xxxl/runtime-instruction-serialization-vectors.ts`
- `tests/xxxl/runtime-instruction-serialization-vectors.test.ts`
- `docs/xxxl/xxxl-runtime-instruction-serialization-vectors.md`

Instruction covered:

- CONSUME_GATEWAY_MINT

Encoding boundary:

- CANONICAL_BINARY_V1

Account meta order:

1. MINT_STATE
2. GATEWAY_CONFIG
3. GUARDIAN_SET
4. PROCESSED_EVENT
5. RECIPIENT_BALANCE
6. SPL_TOKEN_MINT
7. RECIPIENT_TOKEN_ACCOUNT
8. MINT_AUTHORITY_PDA
9. TOKEN_PROGRAM

Field order:

1. instruction
2. version
3. routeId
4. guardianSetId
5. mintId
6. canonicalEventKey
7. recipient
8. amount

Important runtime boundary:

- mint authority PDA is a CPI signer for SPL Token `mint_to`
- mint authority PDA is not a parent instruction signer
- token program account is read-only
- account meta order is deterministic

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 77 files / 551 tests passing
- Build: passing

Status:

- instruction serialization vectors only
- no final byte layout yet
- no live runtime program
- no RPC usage
- no secrets required
