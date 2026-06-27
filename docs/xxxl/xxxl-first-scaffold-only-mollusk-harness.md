# XXXL First Scaffold-Only Mollusk Harness

Status: COMPLETED.

This stage adds the first real Mollusk execution harness for the XXXL SVM runtime.

The harness executes the compiled SBF program through Mollusk, but it intentionally verifies the current scaffold-only boundary.

No live gateway mint route is activated in this stage.

## Goal

Add a minimal SVM execution test that proves the current `consume_gateway_mint` runtime entry path can be executed through Mollusk with a real SBF artifact while preserving the disabled live-route boundary.

## What was added

Added integration test:

- `programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs`

Added direct dev-dependencies required by the integration test type surface:

- `solana-account = "=3.4.0"`
- `solana-instruction = "=3.3.0"`
- `solana-pubkey = "=4.1.0"`

These versions are pinned to the versions already used by `mollusk-svm = "0.13.4"`.

This avoids pulling latest incompatible Solana split-crate versions into the test dependency graph.

## Harness type

The test is an ignored integration test:

    cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture

It is ignored by default because it requires:

- `cargo build-sbf`
- `target/deploy/xxxl_svm.so`

This keeps normal `cargo test` independent from local SBF build artifacts.

## SBF artifact

The SBF artifact is produced by:

    cargo build-sbf

Observed artifact:

- `target/deploy/xxxl_svm.so`

The generated keypair file exists locally but is not used by the test and must not be printed or committed.

## Mollusk execution path

The test uses:

- `Mollusk::new(&program_id, "xxxl_svm")`
- `Instruction::new_with_bytes`
- canonical 9 account metas for `consume_gateway_mint`
- `process_and_validate_instruction`
- `Check::success`
- account data checks for state that must remain unchanged

## Verified scaffold-only behavior

The ignored Mollusk test passes and emits the expected runtime log:

    XXXL consume_gateway_mint scaffold reached; live route execution is not activated

This proves the SBF program is executed through Mollusk, but the live route remains disabled.

## State mutation boundary

The test verifies that the following accounts remain unchanged:

- processed event account
- recipient balance account
- SPL mint account
- recipient token account

This preserves the current scaffold-only runtime boundary:

- no processed event mutation
- no recipient balance mutation
- no SPL mint supply mutation
- no recipient token balance mutation

## Runtime behavior unchanged

This stage does not activate live route execution.

This stage does not invoke SPL Token `mint_to`.

This stage does not mint XXXL.

This stage does not introduce manual minting.

This stage does not introduce hidden emission.

This stage does not grant any Build-derived supply right.

## Verification

Hard checks passed:

- `cargo build-sbf`
- `cargo fmt --check`
- `cargo test`
- `cargo test --test mollusk_consume_gateway_mint -- --ignored --nocapture`
- `cargo clippy --all-targets -- -D warnings`
- `cargo audit`
- `cargo deny check licenses`
- `cargo deny check bans`
- `cargo deny check sources`

Observed default tests:

- 65 passed
- 0 failed
- 1 ignored Mollusk integration test

Observed ignored Mollusk test:

- 1 passed
- 0 failed

## Audit and deny status

cargo audit exits 0 with the existing allowed warnings:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

cargo deny remains green:

- licenses: pass
- bans: pass
- sources: pass

Duplicate crate warnings remain present in bans output and are accepted by current policy.

## Decision

The first scaffold-only Mollusk harness is accepted.

The next runtime stage can begin moving from scaffold-only execution toward guarded state mutation tests, but live route activation must remain disabled until the full atomic SPL CPI path is implemented and verified.
