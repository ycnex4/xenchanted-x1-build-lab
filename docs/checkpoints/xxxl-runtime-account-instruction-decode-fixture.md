# Checkpoint: XXXL Runtime Account/Instruction Decode Fixture

Stage: `stage-xxxl-runtime-account-instruction-decode-fixture`

Status: `RUST_DECODE_FIXTURE_ONLY_NOT_DEPLOYABLE`

## Completed

- Added real Rust instruction discriminator check for `consume_gateway_mint`.
- Kept instruction length fixed at 208 bytes.
- Kept instruction version fixed at `1`.
- Parsed account meta count and canonical account indexes.
- Parsed route id, guardian set id, mint id, canonical event key, recipient, amount, and source-chain weight.
- Added account view length, discriminator, and version checks.
- Added Rust tests for malformed instruction/account bytes.
- Added TypeScript fixture metadata and validation tests.
- Updated README and current design checkpoint.

## Still not included

- no SPL Token CPI
- no recipient ATA validation
- no live Program ID
- no deployment
- no route activation
- no authority freeze execution

## Required checks

- `npm run typecheck`
- `npm test -- --reporter=dot`
- `npm run build`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml instruction::tests -- --nocapture`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml state::tests -- --nocapture`
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml pda::tests -- --nocapture`

## Next likely stage

`stage-xxxl-spl-token-mint-to-cpi-fixture`
