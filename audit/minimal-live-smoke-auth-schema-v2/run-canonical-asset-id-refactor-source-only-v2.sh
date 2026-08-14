#!/usr/bin/env bash
set -euo pipefail

PACKAGE="runtime-source-canonical-asset-id-refactor-source-only-v2"
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

summary_fail() {
  local blocker="$1"
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "transactions_executed=$transactions_executed"
  echo "deploy_executed=$deploy_executed"
  echo "upgrade_executed=$upgrade_executed"
  echo "push_executed=$push_executed"
  echo "refactor_applied=false"
  echo "blocker=$blocker"
  exit 1
}

printf '=== %s started ===\n' "$PACKAGE"
printf 'evidence_dir=%s\n' "$EVIDENCE_DIR"
printf 'transactions_executed=%s deploy_executed=%s upgrade_executed=%s push_executed=%s\n' \
  "$transactions_executed" "$deploy_executed" "$upgrade_executed" "$push_executed"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[ -n "$ROOT" ] || summary_fail "not_inside_git_repo"
cd "$ROOT"

EXPECTED_BRANCH="audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z"
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
HEAD_SHA="$(git rev-parse HEAD)"
[ "$BRANCH" = "$EXPECTED_BRANCH" ] || summary_fail "wrong_branch_${BRANCH}_expected_${EXPECTED_BRANCH}"

if ! git diff --quiet -- .; then
  summary_fail "working_tree_has_uncommitted_changes_reset_before_rerun"
fi
if ! git diff --cached --quiet -- .; then
  summary_fail "index_has_staged_changes_reset_before_rerun"
fi

MANIFEST="programs/xxxl-svm/Cargo.toml"
[ -f "$MANIFEST" ] || summary_fail "missing_program_manifest_$MANIFEST"

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

def replace_required(text: str, old: str, new: str, label: str, min_count: int = 1) -> str:
    count = text.count(old)
    if count < min_count:
        raise SystemExit(f"missing_pattern={label}; count={count}; min={min_count}")
    changes.append({"label": label, "count": count})
    return text.replace(old, new)

# Step 1: semantic rename in source only. ABI offsets and lengths remain unchanged.
for rel in source_files:
    text = read(rel)
    count = text.count("mint_id")
    if count:
        changes.append({"label": f"{rel}: mint_id_to_canonical_asset_id", "count": count})
        text = text.replace("mint_id", "canonical_asset_id")
        write(rel, text)

# Step 2: execution plan carries the local target SPL mint pubkey, not canonical_asset_id.
rel = Path("programs/xxxl-svm/src/execution_plan.rs")
text = read(rel)
text = replace_required(
    text,
    "    pub mint: [u8; 32],\n",
    "    pub target_mint_pubkey: [u8; 32],\n",
    "execution_plan_struct_field_rename",
    min_count=1,
)
text = replace_required(
    text,
    "        mint: args.canonical_asset_id,\n",
    "        target_mint_pubkey: prepared.boundary.accounts.mint.key.to_bytes(),\n",
    "execution_plan_uses_prepared_boundary_spl_mint",
    min_count=1,
)
text = text.replace("execution_plan.mint", "execution_plan.target_mint_pubkey")
text = text.replace("plan.mint", "plan.target_mint_pubkey")
text = text.replace(".mint,", ".target_mint_pubkey,")
text = text.replace(".mint)", ".target_mint_pubkey)")
text = text.replace("execution_plan.mint)", "execution_plan.target_mint_pubkey)")

# Adjust tests that previously asserted plan == args canonical id.
old_assert = "        assert_eq!(plan.target_mint_pubkey, args.canonical_asset_id);"
if old_assert in text:
    text = text.replace(
        old_assert,
        "        assert_eq!(\n            plan.target_mint_pubkey,\n            prepared.boundary.accounts.mint.key.to_bytes()\n        );"
    )
    changes.append({"label": "execution_plan_test_asserts_target_mint_pubkey", "count": 1})
write(rel, text)

# Step 3: CPI planning compares against execution_plan.target_mint_pubkey.
rel = Path("programs/xxxl-svm/src/cpi.rs")
text = read(rel)
text = replace_required(
    text,
    "boundary.accounts.mint.key.to_bytes() != execution_plan.mint",
    "boundary.accounts.mint.key.to_bytes() != execution_plan.target_mint_pubkey",
    "cpi_compares_target_mint_pubkey",
    min_count=1,
)
write(rel, text)

# Step 4: processor preparation uses canonical_asset_id for MintState PDA identity
# and target_mint_pubkey from SPL mint/MintState/GatewayConfig for CPI/ATA/accounting.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
text = replace_required(
    text,
    "    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n\n    if mint_state.mint_pubkey() != args.canonical_asset_id\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n",
    "    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n    let target_mint_pubkey_bytes = spl_token_mint_account.key.to_bytes();\n\n    if mint_state.mint_pubkey() != target_mint_pubkey_bytes\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n",
    "processor_prepare_uses_target_mint_pubkey_bytes",
    min_count=1,
)
text = replace_required(
    text,
    "        || gateway_config.target_mint() != args.canonical_asset_id\n",
    "        || gateway_config.target_mint() != target_mint_pubkey_bytes\n",
    "processor_gateway_config_checks_target_mint_pubkey",
    min_count=1,
)
text = replace_required(
    text,
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.canonical_asset_id {\n",
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != target_mint_pubkey_bytes {\n",
    "processor_recipient_balance_checks_target_mint_pubkey",
    min_count=1,
)
text = replace_required(
    text,
    "    let mint_pubkey = Pubkey::new_from_array(args.canonical_asset_id);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "    let target_mint_pubkey = Pubkey::new_from_array(target_mint_pubkey_bytes);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "processor_ata_uses_target_mint_pubkey",
    min_count=1,
)
text = replace_required(
    text,
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &mint_pubkey)?;\n",
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &target_mint_pubkey)?;\n",
    "processor_ata_boundary_target_mint_pubkey",
    min_count=1,
)
write(rel, text)

# Step 5: targeted additional regression test, inserted once if not already present.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
needle = "fn handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint"
if needle not in text:
    insert_before = "    #[test]\n    fn handler_integration_rejects_wrong_account_count()"
    new_test = r'''    #[test]
    fn handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.args.canonical_asset_id = [0x42; 32];

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let prepared = prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)
            .expect("canonical asset id must be distinct from target SPL mint");

        assert_eq!(prepared.boundary.accounts.mint.key.to_bytes(), fixture.keys.spl_token_mint.to_bytes());
        assert_ne!(args.canonical_asset_id, fixture.keys.spl_token_mint.to_bytes());
    }

'''
    if insert_before not in text:
        raise SystemExit("missing_insert_anchor=handler_integration_rejects_wrong_account_count")
    text = text.replace(insert_before, new_test + insert_before)
    changes.append({"label": "processor_regression_test_canonical_asset_distinct_from_target_spl_mint", "count": 1})
write(rel, text)

# Save source-after snapshots and metadata.
for rel in source_files:
    dst = source_after / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(root / rel, dst)

subprocess.run(["git", "diff", "--", "programs/xxxl-svm/src"], cwd=root, text=True, stdout=(results / "canonical_asset_id_refactor.diff").open("w"), check=True)

result = {
    "package": "runtime-source-canonical-asset-id-refactor-source-only-v2",
    "transactions_executed": False,
    "deploy_executed": False,
    "upgrade_executed": False,
    "push_executed": False,
    "source_files": [str(p) for p in source_files],
    "changes": changes,
    "abi_offsets_changed": False,
    "semantic_rename": "mint_id -> canonical_asset_id",
    "cpi_identity": "target_mint_pubkey from prepared boundary / SPL mint account",
}
(results / "canonical_asset_id_refactor_result.json").write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
PY

set +e
cargo fmt --manifest-path "$MANIFEST" -- --check >"$LOG_DIR/cargo_fmt_check.log" 2>&1
fmt_code=$?
set -e

set +e
cargo test --manifest-path "$MANIFEST" >"$LOG_DIR/cargo_test.log" 2>&1
test_code=$?
set -e

changed_files="$(git diff --name-only -- programs/xxxl-svm/src | tr '\n' ' ')"
diff_stat="$(git diff --stat -- programs/xxxl-svm/src | tr '\n' ' ' | sed 's/[[:space:]]\+/ /g')"

refactor_applied=true
blocker=none
if [ "$fmt_code" -ne 0 ]; then
  blocker="cargo_fmt_check_failed"
fi
if [ "$test_code" -ne 0 ]; then
  if [ "$blocker" = "none" ]; then
    blocker="cargo_test_failed"
  else
    blocker="${blocker}_and_cargo_test_failed"
  fi
fi

{
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "head_before=$HEAD_SHA"
  echo "transactions_executed=$transactions_executed"
  echo "deploy_executed=$deploy_executed"
  echo "upgrade_executed=$upgrade_executed"
  echo "push_executed=$push_executed"
  echo "refactor_applied=$refactor_applied"
  echo "abi_offsets_changed=false"
  echo "semantic_rename=mint_id_to_canonical_asset_id"
  echo "cpi_uses_target_mint_pubkey=true"
  echo "working_tree_modified=true"
  echo "changed_files=$changed_files"
  echo "diff_stat=$diff_stat"
  echo "cargo_fmt_check_code=$fmt_code"
  echo "cargo_test_code=$test_code"
  echo "blocker=$blocker"
  echo "diff_path=$RESULTS_DIR/canonical_asset_id_refactor.diff"
  echo "result_json=$RESULTS_DIR/canonical_asset_id_refactor_result.json"
  echo "cargo_fmt_log=$LOG_DIR/cargo_fmt_check.log"
  echo "cargo_test_log=$LOG_DIR/cargo_test.log"
} > "$RESULTS_DIR/compact-summary.txt"

echo "=== canonical asset id refactor source-only compact summary for chat ==="
cat "$RESULTS_DIR/compact-summary.txt"

echo "=== canonical asset id refactor saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "diff_path=$RESULTS_DIR/canonical_asset_id_refactor.diff"
echo "result_json=$RESULTS_DIR/canonical_asset_id_refactor_result.json"
echo "cargo_fmt_log=$LOG_DIR/cargo_fmt_check.log"
echo "cargo_test_log=$LOG_DIR/cargo_test.log"

if [ "$blocker" != "none" ]; then
  echo "=== cargo fmt tail ==="
  tail -n 40 "$LOG_DIR/cargo_fmt_check.log" || true
  echo "=== cargo test tail ==="
  tail -n 80 "$LOG_DIR/cargo_test.log" || true
  exit 1
fi
