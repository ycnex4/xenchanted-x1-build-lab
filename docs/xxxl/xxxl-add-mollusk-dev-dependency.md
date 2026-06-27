# XXXL Add Mollusk Dev Dependency

Status: COMPLETED.

This stage adds `mollusk-svm` as an explicit dev-dependency for the XXXL SVM runtime test layer.

This stage does not add a Mollusk harness yet.

## Goal

Introduce the Mollusk dependency separately from the first harness implementation so that the dependency footprint, lockfile changes, audit warnings, and deny results are visible and reviewed in isolation.

## Added dependency

Added to `programs/xxxl-svm/Cargo.toml`:

    mollusk-svm = "0.13.4"

Dependency type:

- dev-dependency only

Runtime dependency pins remain:

- solana-program = 2.3.0
- spl-token = 5.0.2 with no-entrypoint feature

## Why this is separate from the harness

Mollusk significantly increases the test dependency surface.

This stage intentionally isolates the dependency addition before adding runtime harness code.

This makes it easier to review:

- Cargo.toml change
- Cargo.lock change
- audit warning delta
- deny warning delta
- Solana/Agave transitive dependency shape
- local Mollusk source/API availability

## Verification

Hard checks passed after adding Mollusk:

- cargo fmt --check
- cargo test
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

Observed tests:

- 65 passed
- 0 failed

## Dependency footprint

Cargo audit dependency scan count after adding Mollusk:

- 404 crate dependencies

Previously observed before Mollusk:

- 196 crate dependencies

This increase is expected and accepted for dev-dependency test harness work only.

## Audit warnings

cargo audit exits 0 with allowed warnings.

Allowed warnings after adding Mollusk:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

Warning count:

- before Mollusk: 3 allowed warnings
- after Mollusk: 6 allowed warnings

## Deny status

cargo deny results after adding Mollusk:

- licenses: pass
- bans: pass
- sources: pass

Duplicate crate warnings remain present in bans output and are accepted as warnings in the current policy.

## Local Mollusk source

The local registry source was located after dependency install.

Observed package:

- mollusk-svm-0.13.4

Useful source areas identified for the next harness stage:

- src/lib.rs
- src/program.rs
- src/account_store.rs
- src/compile_accounts.rs
- tests/process_fixture.rs
- tests/instruction_chain.rs
- tests/system_program.rs
- tests/bpf_program.rs

## Current runtime behavior

This stage does not change runtime behavior.

Current policy remains:

- process_instruction remains scaffold-only
- live route activation remains disabled
- SPL mint_to is not invoked from process_instruction
- no live gateway minting is enabled
- no manual mint is introduced
- no hidden emission is introduced
- no Build-derived supply right is introduced

## Decision

Mollusk is now available as a dev-dependency for a future harness stage.

Do not add the first harness in this stage.

Do not activate live route execution in this stage.

Do not change runtime code in this stage.

## Next likely stage

Implement the first scaffold-only Mollusk harness.

Recommended first harness target:

- valid consume_gateway_mint instruction
- canonical 9 account metas
- process_instruction returns success
- processed_event remains unchanged
- recipient_balance remains unchanged
- SPL mint supply remains unchanged
- recipient token balance remains unchanged
- live route remains disabled

## Conclusion

The Mollusk dev-dependency has been added intentionally and verified.

The next stage can focus on a minimal scaffold-only harness without mixing dependency-surface review into harness logic.
