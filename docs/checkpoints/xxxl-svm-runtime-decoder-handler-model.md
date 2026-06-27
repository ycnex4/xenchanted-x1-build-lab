# XXXL SVM Runtime Decoder Handler Model Checkpoint

Stage XXXL Program v1 now has a deterministic SVM runtime decoder/handler model.

New files:

- `src/xxxl/svm-runtime-decoder-handler-model.ts`
- `tests/xxxl/svm-runtime-decoder-handler-model.test.ts`
- `docs/xxxl/xxxl-svm-runtime-decoder-handler-model.md`

The model verifies:

- bytes -> decode/validate -> handler input -> skeleton execution boundary

Decoded layouts:

- Mint State account
- Gateway Config account
- Guardian Set account
- Processed Event account
- Recipient Balance account
- `consume_gateway_mint` instruction

Validation coverage:

- byte length
- discriminator
- version
- canonical bytes
- field ranges
- decoded field values
- required account presence
- instruction presence
- CPI preparation only after valid decode

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 87 files / 702 tests passing
- Build: passing
