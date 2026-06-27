# XXXL SVM Serialized Runtime Vectors Checkpoint

Stage XXXL Program v1 now has deterministic SVM serialized runtime vectors.

New files:

- `src/xxxl/svm-serialized-runtime-vectors.ts`
- `tests/xxxl/svm-serialized-runtime-vectors.test.ts`
- `docs/xxxl/xxxl-svm-serialized-runtime-vectors.md`

Defined serialized vectors:

- Mint State account
- Gateway Config account
- Guardian Set account
- Processed Event account
- Recipient Balance account
- `consume_gateway_mint` instruction

Each vector includes:

- layout kind
- byte length
- canonical hex
- selected field probes
- field offsets
- field sizes
- field hex slices

Bundle coverage:

- handler: `consume_gateway_mint`
- SPL Token Program ID
- gateway mint authority PDA model
- canonical account meta roles
- CPI prepared flag
- CPI atomic-with-parent-transaction flag

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 86 files / 680 tests passing
- Build: passing
