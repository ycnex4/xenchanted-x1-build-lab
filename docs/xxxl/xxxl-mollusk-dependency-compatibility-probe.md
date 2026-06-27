# XXXL Mollusk Dependency Compatibility Probe

Status: COMPLETED.

This stage records a dependency compatibility probe for adding Mollusk to the XXXL SVM runtime test layer.

It is intentionally doc-only.

No Mollusk dependency is added to the repository in this stage.

## Goal

Check whether `mollusk-svm` can be added as a dev-dependency against the current XXXL SVM dependency stack without immediately breaking:

- cargo test
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

## Current runtime dependency pins

Current XXXL SVM runtime dependency pins:

- solana-program = 2.3.0
- spl-token = 5.0.2 with no-entrypoint feature

## Baseline before probe

Baseline hard checks passed before the temporary Mollusk probe:

- cargo fmt --check
- cargo test
- cargo clippy --all-targets -- -D warnings
- cargo audit
- cargo deny check licenses
- cargo deny check bans
- cargo deny check sources

Observed baseline:

- Rust tests: 65 passed, 0 failed
- Cargo.lock dependency count scanned by cargo audit: 196 crates
- cargo audit allowed warnings: 3

Baseline allowed cargo audit warnings:

- bincode 1.3.3
- libsecp256k1 0.6.0
- rand 0.7.3

## Probe method

A temporary copy of the repository was created under /tmp.

The probe added Mollusk only in the temporary copy:

    cargo add --dev mollusk-svm

The real repository was not modified except for local logs.

## Discovered Mollusk package

cargo search found:

- mollusk-svm = 0.13.4
- mollusk-svm-error = 0.13.4
- mollusk-svm-result = 0.13.4
- mollusk-svm-programs-token = 0.13.4
- mollusk-svm-bencher = 0.13.4

The temp probe added:

- mollusk-svm = 0.13.4

## Temporary compatibility result

The temporary Mollusk dependency probe succeeded.

Observed temporary checks:

- cargo add --dev mollusk-svm: success
- cargo test: success
- cargo clippy --all-targets -- -D warnings: success
- cargo audit: success
- cargo deny check licenses: success
- cargo deny check bans: success
- cargo deny check sources: success

Observed temporary tests:

- 65 passed
- 0 failed

## Dependency footprint

Adding mollusk-svm significantly increases the dependency surface.

Observed cargo audit scan count:

- before Mollusk: 196 crate dependencies
- after temporary Mollusk add: 404 crate dependencies

This is acceptable for a future dev-dependency stage only if documented explicitly.

## Temporary cargo audit warnings

After the temporary Mollusk add, cargo audit still exited 0, but allowed warnings increased.

Temporary allowed warnings:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

Warning count changed:

- before Mollusk: 3 allowed warnings
- after temporary Mollusk add: 6 allowed warnings

## Compatibility notes

The temporary lockfile pulled in additional Solana/Agave split crates, including Solana program-runtime and Solana program-error 3.x family crates.

The current runtime crate still compiles and tests pass with the temporary dev-dependency.

This suggests Mollusk is compatible enough for a future isolated harness stage, but the larger dependency graph must be treated as a deliberate testing dependency, not as a silent runtime change.

## Decision

Mollusk is compatible enough to justify a future harness implementation stage.

Do not add Mollusk to the repository in this probe stage.

Do not change runtime code in this probe stage.

Do not activate live route execution.

Do not invoke SPL mint_to from process_instruction.

Do not change runtime dependency pins in this stage.

## Recommended next stage

A future stage may add Mollusk as a dev-dependency and introduce the first scaffold-only Mollusk harness.

Recommended first harness target:

- valid consume_gateway_mint instruction
- canonical 9 accounts
- process_instruction returns success
- no processed_event mutation
- no recipient_balance mutation
- no SPL mint_to execution
- live route remains disabled

## Conclusion

The dependency probe passed.

Mollusk can be introduced later as a dev-dependency with clear documentation of the increased dependency footprint and additional allowed audit warnings.
