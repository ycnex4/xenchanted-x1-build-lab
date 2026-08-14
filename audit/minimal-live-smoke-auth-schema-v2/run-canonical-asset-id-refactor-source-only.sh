#!/usr/bin/env bash
set -euo pipefail

PACKAGE="runtime-source-canonical-asset-id-refactor-source-only"
BASE_DIR="${XXXL_EVIDENCE_BASE:-$HOME/xenchanted-stage20-activation-evidence-c332814}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
EVIDENCE_DIR="$BASE_DIR/${PACKAGE}-${TS}"
RESULTS_DIR="$EVIDENCE_DIR/results"
SOURCE_BEFORE_DIR="$EVIDENCE_DIR/source-before"
SOURCE_AFTER_DIR="$EVIDENCE_DIR/source-after"
LOG_DIR="$EVIDENCE_DIR/logs"

mkdir -p "$RESULTS_DIR" "$SOURCE_BEFORE_DIR" "$SOURCE_AFTER_DIR" "$LOG_DIR"

exec > >(tee "$LOG_DIR/${PACKAGE}.log") 2>&1

transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false

fail() {
  echo "blocker=$1"
  echo "refactor_applied=false"
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "transactions_executed=$transactions_executed"
  echo "deploy_executed=$deploy_executed"
  echo "upgrade_executed=$upgrade_executed"
  echo "push_executed=$push_executed"
  echo "refactor_applied=false"
  echo "blocker=$1"
  exit 1
}

printf '=== %s started ===\n' "$PACKAGE"
printf 'evidence_dir=%s\n' "$EVIDENCE_DIR"
printf 'transactions_executed=%s deploy_executed=%s upgrade_executed=%s push_executed=%s\n' \
  "$transactions_executed" "$deploy_executed" "$upgrade_executed" "$push_executed"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[ -n "$ROOT" ] || fail "not_inside_git_repo"
cd "$ROOT"

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
HEAD_SHA="$(git rev-parse HEAD)"

EXPECTED_BRANCH="audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z"
if [ "$BRANCH" != "$EXPECTED_BRANCH" ]; then
  fail "wrong_branch_${BRANCH}_expected_${EXPECTED_BRANCH}"
fi

if ! git diff --quiet -- .; then
  fail "working_tree_has_uncommitted_changes"
fi
if ! git diff --cached --quiet -- .; then
  fail "index_has_staged_changes"
fi

MANIFEST="programs/xxxl-svm/Cargo.toml"
[ -f "$MANIFEST" ] || fail "missing_program_manifest_$MANIFEST"

python3 - "$EVIDENCE_DIR" <<'PY'
from pathlib import Path
import json
import shutil
import subprocess
import sys

root = Path.cwd()
evidence = Path(sys.argv[1])
source_before = evidence / "source-before"
source_after = evidence / "source-after"
results = evidence / "results"

source_files = [
    Path("programs/xxxl-svm/src/instruction.rs"),
    Path("programs/xxxl-svm/src/processor.rs"),
    Path("programs/xxxl-svm/src/execution_plan.rs"),
    Path("programs/xxxl-svm/src/cpi.rs"),
    Path("programs/xxxl-svm/src/pda.rs"),
]

for rel in source_files:
    src = root / rel
    if not src.exists():
        raise SystemExit(f"missing_source_file={rel}")
    dst = source_before / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)

changes = []

def read(rel: Path) -> str:
    return (root / rel).read_text()

def write(rel: Path, text: str):
    (root / rel).write_text(text)

def replace_exact(text: str, old: str, new: str, label: str, min_count: int = 1, max_count: int | None = None) -> str:
    count = text.count(old)
    if count < min_count:
        raise SystemExit(f"missing_pattern={label}; count={count}; min={min_count}")
    if max_count is not None and count > max_count:
        raise SystemExit(f"too_many_pattern={label}; count={count}; max={max_count}")
    changes.append({"label": label, "count": count})
    return text.replace(old, new)

# 1) Global semantic rename in program source: mint_id -> canonical_asset_id.
# This is a source-level rename only; ABI offsets remain unchanged.
for rel in source_files:
    text = read(rel)
    count = text.count("mint_id")
    if count:
        changes.append({"label": f"{rel}: mint_id_to_canonical_asset_id", "count": count})
        text = text.replace("mint_id", "canonical_asset_id")
        write(rel, text)

# 2) Execution plan must carry the local X1/SPL mint pubkey, not canonical_asset_id.
rel = Path("programs/xxxl-svm/src/execution_plan.rs")
text = read(rel)
text = replace_exact(
    text,
    "    pub mint: [u8; 32],\n",
    "    pub target_mint_pubkey: [u8; 32],\n",
    "execution_plan_struct_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
text = replace_exact(
    text,
    "        mint: args.canonical_asset_id,\n",
    "        target_mint_pubkey: prepared.boundary.accounts.mint.key.to_bytes(),\n",
    "execution_plan_uses_prepared_boundary_spl_mint",
    min_count=1,
    max_count=1,
)
text = text.replace("execution_plan.mint", "execution_plan.target_mint_pubkey")
text = text.replace("plan.mint", "plan.target_mint_pubkey")
text = replace_exact(
    text,
    "        assert_eq!(plan.target_mint_pubkey, args.canonical_asset_id);",
    "        assert_eq!(\n            plan.target_mint_pubkey,\n            prepared.boundary.accounts.mint.key.to_bytes()\n        );",
    "execution_plan_test_asserts_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
write(rel, text)

# 3) CPI planning compares against execution_plan.target_mint_pubkey.
rel = Path("programs/xxxl-svm/src/cpi.rs")
text = read(rel)
text = replace_exact(
    text,
    "boundary.accounts.mint.key.to_bytes() != execution_plan.mint",
    "boundary.accounts.mint.key.to_bytes() != execution_plan.target_mint_pubkey",
    "cpi_compares_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
write(rel, text)

# 4) Runtime preparation must use canonical_asset_id for MintState PDA identity,
# but target_mint_pubkey from MintState/GatewayConfig/SPL account for CPI + ATA + balance.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
text = replace_exact(
    text,
    "    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n\n    if mint_state.mint_pubkey() != args.canonical_asset_id\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n",
    "    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n    let target_mint_pubkey = spl_token_mint_account.key.to_bytes();\n\n    if mint_state.mint_pubkey() != target_mint_pubkey\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n",
    "processor_prepare_defines_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
text = replace_exact(
    text,
    "        || gateway_config.target_mint() != args.canonical_asset_id\n",
    "        || gateway_config.target_mint() != target_mint_pubkey\n",
    "processor_gateway_config_checks_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
text = replace_exact(
    text,
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.canonical_asset_id {\n",
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != target_mint_pubkey {\n",
    "processor_recipient_balance_checks_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
text = replace_exact(
    text,
    "    let mint_pubkey = Pubkey::new_from_array(args.canonical_asset_id);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "    let target_mint_pubkey = Pubkey::new_from_array(target_mint_pubkey);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "processor_ata_uses_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
text = replace_exact(
    text,
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &mint_pubkey)?;\n",
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &target_mint_pubkey)?;\n",
    "processor_ata_boundary_target_mint_pubkey",
    min_count=1,
    max_count=1,
)
write(rel, text)

# 5) Add a focused regression test where canonical_asset_id differs from target SPL mint.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
new_test = r'''
    #[test]
    fn handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.args.canonical_asset_id = [0x42; 32];

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let prepared = prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)
            .expect("canonical_asset_id is domain identity; SPL mint comes from MintState/GatewayConfig/accounts");

        assert_eq!(prepared.boundary.accounts.mint.key, &fixture.keys.spl_mint);
    }
'''
anchor = "    #[test]\n    fn handler_integration_rejects_wrong_account_count() {\n"
if new_test not in text:
    text = replace_exact(
        text,
        anchor,
        new_test + "\n" + anchor,
        "insert_processor_canonical_asset_id_distinct_test",
        min_count=1,
        max_count=1,
    )
write(rel, text)

# 6) Save post-change source snapshots and result metadata.
for rel in source_files:
    src = root / rel
    dst = source_after / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)

remaining = subprocess.run(
    ["grep", "-R", "-n", "mint_id", "programs/xxxl-svm/src"],
    cwd=root,
    text=True,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
)
(results / "remaining_mint_id_grep.txt").write_text(remaining.stdout)

(results / "canonical_asset_id_refactor_changes.json").write_text(json.dumps({
    "changes": changes,
    "remaining_mint_id_grep_exit_code": remaining.returncode,
    "remaining_mint_id_grep_line_count": 0 if not remaining.stdout else len(remaining.stdout.splitlines()),
}, indent=2))

print("python_refactor_applied=true")
print(f"change_entries={len(changes)}")
print(f"remaining_mint_id_grep_exit_code={remaining.returncode}")
print(f"remaining_mint_id_grep_line_count={0 if not remaining.stdout else len(remaining.stdout.splitlines())}")
PY

cargo_fmt_status=not_run
cargo_test_status=not_run

if command -v cargo >/dev/null 2>&1; then
  echo "=== cargo fmt check ==="
  if cargo fmt --manifest-path "$MANIFEST" -- --check > "$LOG_DIR/cargo-fmt-check.log" 2>&1; then
    cargo_fmt_status=pass
  else
    cargo_fmt_status=fail
    echo "cargo_fmt_status=fail"
    tail -80 "$LOG_DIR/cargo-fmt-check.log" || true
  fi

  echo "=== cargo test ==="
  if cargo test --manifest-path "$MANIFEST" > "$LOG_DIR/cargo-test.log" 2>&1; then
    cargo_test_status=pass
  else
    cargo_test_status=fail
    echo "cargo_test_status=fail"
    tail -120 "$LOG_DIR/cargo-test.log" || true
  fi
else
  cargo_fmt_status=cargo_missing
  cargo_test_status=cargo_missing
fi

git diff -- programs/xxxl-svm/src > "$RESULTS_DIR/canonical_asset_id_refactor.diff" || true
git status --short > "$RESULTS_DIR/git_status_short.txt" || true

remaining_mint_id_line_count="$(wc -l < "$RESULTS_DIR/remaining_mint_id_grep.txt" | tr -d ' ')"
changed_files="$(git diff --name-only -- programs/xxxl-svm/src | tr '\n' ' ' | sed 's/[[:space:]]*$//')"

cat > "$RESULTS_DIR/canonical_asset_id_refactor_summary.txt" <<EOF
package=$PACKAGE
evidence_dir=$EVIDENCE_DIR
branch=$BRANCH
head_sha_before=$HEAD_SHA
transactions_executed=$transactions_executed
deploy_executed=$deploy_executed
upgrade_executed=$upgrade_executed
push_executed=$push_executed
refactor_applied=true
abi_size_changed=false
consume_field_80_112_semantics=canonical_asset_id
mint_state_pda_identity=canonical_asset_id
target_spl_mint_source=MintState/GatewayConfig/SPL mint account
execution_plan_target_mint_source=prepared.boundary.accounts.mint.key
cargo_fmt_status=$cargo_fmt_status
cargo_test_status=$cargo_test_status
remaining_mint_id_line_count=$remaining_mint_id_line_count
changed_files=$changed_files
diff_path=$RESULTS_DIR/canonical_asset_id_refactor.diff
status_path=$RESULTS_DIR/git_status_short.txt
EOF

echo "=== canonical asset id refactor source-only compact summary for chat ==="
cat "$RESULTS_DIR/canonical_asset_id_refactor_summary.txt"
echo "=== canonical asset id refactor saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "summary=$RESULTS_DIR/canonical_asset_id_refactor_summary.txt"
echo "diff=$RESULTS_DIR/canonical_asset_id_refactor.diff"
echo "cargo_fmt_log=$LOG_DIR/cargo-fmt-check.log"
echo "cargo_test_log=$LOG_DIR/cargo-test.log"
echo "remaining_mint_id_grep=$RESULTS_DIR/remaining_mint_id_grep.txt"

if [ "$cargo_fmt_status" != "pass" ] || [ "$cargo_test_status" != "pass" ]; then
  exit 1
fi
