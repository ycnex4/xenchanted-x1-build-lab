# Canonical Asset ID Refactor — Daemon Review Package

## Review target

Repository: ycnex4/xenchanted-x1-build-lab

Branch: audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z

Primary code commit to review:

705d957 Refactor consume mint identity to canonical asset id

This package is for source review only.

## Hard boundaries

The operator reports:

transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false before this GitHub publication step

This review package must not be interpreted as approval to deploy or upgrade.

Do not propose or perform deployment as part of this review.

Do not include or require:

target/deploy artifacts
.so files
keypairs
private keys
seed material

## Intent of the refactor

The consume gateway mint flow must separate two identities:

1. canonical_asset_id

Canonical cross-chain asset identity.

Used for:

instruction field at offset 80..112
MintState PDA seed

2. target_mint_pubkey

Actual local SPL mint pubkey on X1/Solana runtime.

Used for:

SPL mint account validation
recipient token account mint validation
recipient balance mint validation
gateway config target mint validation
CPI mint_to boundary

## Previous issue

The previous runtime semantics conflated mint_id with the target SPL mint.

That was incorrect for the intended multichain model, where a canonical asset identity and the local SPL mint are distinct values.

The desired model is:

canonical_asset_id is not necessarily equal to target_mint_pubkey
MintState PDA seed = canonical_asset_id
MintState stored target mint = target_mint_pubkey
CPI/token-state mint = target_mint_pubkey

## What the daemon should verify

### 1. ABI stability

Verify that the consume gateway mint instruction layout remains stable:

instruction length = 208
account meta count unchanged
account index bytes unchanged
reserved byte behavior unchanged
offset 80..112 remains the same physical field

Semantic rename:

old semantic name: mint_id
new semantic name: canonical_asset_id

The daemon should confirm this is a semantic correction, not an ABI-breaking layout change.

### 2. PDA semantics

Verify:

MintState PDA is derived from canonical_asset_id
PDA seed byte layout remains stable
PDA naming/semantic change is intentional

### 3. Runtime target mint semantics

Verify that all local token-state and CPI checks use target_mint_pubkey, not canonical_asset_id.

Specifically:

MintState stored target mint check
GatewayConfig target mint check
RecipientBalance mint check
SPL mint account check
Recipient token account mint check
CPI mint_to boundary

### 4. Execution plan semantics

Verify that execution plans use target_mint_pubkey for local SPL mint operations.

The execution plan must not use canonical_asset_id as if it were the SPL mint pubkey.

### 5. Test integrity

Verify that updated tests were not weakened.

Important test areas:

programs/xxxl-svm/tests/disabled_cpi_reachability.rs
programs/xxxl-svm/tests/instruction_reserved_bytes.rs
programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs
programs/xxxl-svm/tests/phase_41k5_d2_production_path_gated_mark_and_mint_e2e.rs
programs/xxxl-svm/tests/phase_41k5_d3_negative_failure_modes.rs
programs/xxxl-svm/tests/phase_41k6_b2_valid_quorum_live_gated_success.rs
programs/xxxl-svm/tests/phase_41k6_b3_hostile_live_gated_matrix.rs

Particular Mollusk note:

The local SBF artifact was stale before rebuild.

After local cargo build-sbf, Mollusk tests moved from broad failures to one malformed-instruction expectation issue.

The remaining malformed test was fixed by making the truncated payload carry the valid consume discriminator, so it tests length/malformed handling rather than wrong-discriminator handling.

### 6. Safety gate

Verify that default/non-live behavior remains safe:

CpiBoundaryNotReady remains expected safety gate
no unexpected mutation before CPI live gate
feature-gated behavior remains guarded

### 7. Repository hygiene

Verify that the reviewed commit does not include:

target/deploy
xxxl_svm.so
xxxl_svm-keypair.json
private keys
seed phrases

## Local evidence reported by operator

The operator reports the following local evidence after commit 705d957:

rustfmt_code=0
rustfmt_check_code=0
cargo_build_sbf_code=0
cargo_test_code=0
git_diff_check_code=0
working_tree_clean_after_commit=true

Post-commit sanity reported:

HEAD=705d957
705d957 Refactor consume mint identity to canonical asset id
no target deploy staged/tracked in this commit

Changed files in code commit:

programs/xxxl-svm/src/cpi.rs
programs/xxxl-svm/src/execution_plan.rs
programs/xxxl-svm/src/instruction.rs
programs/xxxl-svm/src/pda.rs
programs/xxxl-svm/src/processor.rs
programs/xxxl-svm/tests/disabled_cpi_reachability.rs
programs/xxxl-svm/tests/instruction_reserved_bytes.rs
programs/xxxl-svm/tests/mollusk_consume_gateway_mint.rs
programs/xxxl-svm/tests/phase_41k5_d2_production_path_gated_mark_and_mint_e2e.rs
programs/xxxl-svm/tests/phase_41k5_d3_negative_failure_modes.rs
programs/xxxl-svm/tests/phase_41k6_b2_valid_quorum_live_gated_success.rs
programs/xxxl-svm/tests/phase_41k6_b3_hostile_live_gated_matrix.rs

## Requested daemon output

Please return:

PASS / FAIL / NEEDS_CHANGES

Include:

exact risky files/lines if any
whether the refactor is safe to share with Theo
whether a new live smoke should be attempted now
or whether another source review step is required first
