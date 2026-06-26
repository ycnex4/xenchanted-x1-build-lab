# XXXL Runtime Account Serialization Vectors Checkpoint

Stage XXXL Program v1 now has deterministic runtime account serialization vector planning.

New files:

- `src/xxxl/runtime-account-serialization-vectors.ts`
- `tests/xxxl/runtime-account-serialization-vectors.test.ts`
- `docs/xxxl/xxxl-runtime-account-serialization-vectors.md`

Account kinds covered:

- MINT_STATE
- GATEWAY_CONFIG
- GUARDIAN_SET
- PROCESSED_EVENT
- RECIPIENT_BALANCE

Encoding boundary:

- CANONICAL_BINARY_V1

Layout rules:

- every layout starts with `kind`
- every layout uses `version` as second field
- every account kind has an explicit field order
- bigint values serialize to canonical JSON as decimal strings
- vectors bind account kind, version, encoding, discriminator, field order, account fixture, and canonical JSON

Vector ids:

- XXXL_RUNTIME_MINT_STATE_ACCOUNT_V1
- XXXL_RUNTIME_GATEWAY_CONFIG_ACCOUNT_V1
- XXXL_RUNTIME_GUARDIAN_SET_ACCOUNT_V1
- XXXL_RUNTIME_PROCESSED_EVENT_ACCOUNT_V1
- XXXL_RUNTIME_RECIPIENT_BALANCE_ACCOUNT_V1

Expected validation baseline after this stage:

- TypeScript typecheck: passing
- Tests: 76 files / 538 tests passing
- Build: passing

Status:

- account serialization vectors only
- no instruction serialization vectors yet
- no final byte layout yet
- no live runtime program
- no RPC usage
- no secrets required
