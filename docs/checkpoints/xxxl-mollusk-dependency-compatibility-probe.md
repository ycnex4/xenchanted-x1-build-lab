# Checkpoint: XXXL Mollusk Dependency Compatibility Probe

Stage: stage-xxxl-mollusk-dependency-compatibility-probe

Status: COMPLETED

## Goal

Probe whether mollusk-svm can be added as a temporary dev-dependency against the current XXXL SVM dependency stack.

## Completed

- Confirmed baseline hard checks.
- Confirmed current runtime pins:
  - solana-program = 2.3.0
  - spl-token = 5.0.2
- Ran cargo search for Mollusk packages.
- Created a temporary repository copy under /tmp.
- Added mollusk-svm only in the temporary copy.
- Verified temporary cargo test.
- Verified temporary cargo clippy --all-targets -- -D warnings.
- Verified temporary cargo audit.
- Verified temporary cargo deny licenses/bans/sources.
- Confirmed the real repository remains unchanged except local logs.
- Documented dependency footprint increase.
- Documented audit warning increase.

## Results

Temporary dependency added:

- mollusk-svm = 0.13.4

Temporary verification:

- cargo add --dev mollusk-svm: pass
- cargo test: pass
- cargo clippy --all-targets -- -D warnings: pass
- cargo audit: pass
- cargo deny check licenses: pass
- cargo deny check bans: pass
- cargo deny check sources: pass

Observed tests:

- 65 passed
- 0 failed

## Dependency footprint

cargo audit dependency scan count:

- baseline: 196 crates
- with temporary mollusk-svm: 404 crates

## Audit warnings

Baseline allowed warnings:

- bincode 1.3.3
- libsecp256k1 0.6.0
- rand 0.7.3

Temporary Mollusk allowed warnings:

- bincode 1.3.3
- derivative 2.2.0
- libsecp256k1 0.6.0
- paste 1.0.15
- proc-macro-error2 2.0.1
- rand 0.7.3

## Decision

Mollusk is compatible enough for a future dedicated harness stage.

Do not add Mollusk to the repository in this probe stage.

Do not change runtime code in this probe stage.

Do not activate live route execution in this probe stage.

## Next likely stage

Add mollusk-svm as an explicit dev-dependency and implement the first scaffold-only Mollusk harness in a separate stage.
