# XXXL Runtime Candidate Account and Instruction Schema Checkpoint

Stage XXXL Program v1 now has a concrete runtime candidate account layout and instruction schema.

New files:

- `src/xxxl/runtime-candidate.ts`
- `tests/xxxl/runtime-candidate.test.ts`
- `docs/xxxl/xxxl-runtime-candidate-account-instruction-schema.md`

Runtime candidate account kinds:

- Mint State
- Gateway Configuration
- Guardian Set
- Processed Event
- Recipient Balance

Canonical instruction:

    CONSUME_GATEWAY_MINT

Runtime write set:

- Mint State
- Processed Event
- Recipient Balance

Validation coverage:

- valid account layout
- valid instruction schema
- gateway-only mint authority mode
- active route and guardian set
- guardian set and quorum matching
- invalid quorum threshold
- instruction data mismatch
- documented write set

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 69 files / 468 tests passing
- Build: passing

Status:

- candidate schema only
- no production runtime code
- no deployment scripts
- no RPC usage
- no secrets required
