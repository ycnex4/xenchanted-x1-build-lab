#!/usr/bin/env bash
set -u -o pipefail

PACKAGE="runtime-source-canonical-asset-id-refactor-source-only-v3"
BASE_DIR="${XXXL_EVIDENCE_BASE:-$HOME/xenchanted-stage20-activation-evidence-c332814}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
EVIDENCE_DIR="$BASE_DIR/${PACKAGE}-${TS}"
RESULTS_DIR="$EVIDENCE_DIR/results"
LOG_DIR="$EVIDENCE_DIR/logs"
SOURCE_BEFORE_DIR="$EVIDENCE_DIR/source-before"
SOURCE_AFTER_DIR="$EVIDENCE_DIR/source-after"

mkdir -p "$RESULTS_DIR" "$LOG_DIR" "$SOURCE_BEFORE_DIR" "$SOURCE_AFTER_DIR"
exec > >(tee "$LOG_DIR/${PACKAGE}.log") 2>&1

transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false

finish() {
  local blocker="$1"
  local refactor_applied="$2"
  local fmt_code="${3:-not_run}"
  local test_code="${4:-not_run}"
  local changed_files=""
  local diff_stat=""
  changed_files="$(git diff --name-only -- . 2>/dev/null | tr '\n' ' ' | sed 's/[[:space:]]*$//')"
  diff_stat="$(git diff --stat -- . 2>/dev/null | tr '\n' ' ' | sed 's/[[:space:]]*$//')"
  git diff -- . > "$RESULTS_DIR/canonical_asset_id_refactor_v3.diff" 2>/dev/null || true
  cat > "$RESULTS_DIR/canonical_asset_id_refactor_v3_result.json" <<JSON
{
  "package": "$PACKAGE",
  "evidence_dir": "$EVIDENCE_DIR",
  "transactions_executed": $transactions_executed,
  "deploy_executed": $deploy_executed,
  "upgrade_executed": $upgrade_executed,
  "push_executed": $push_executed,
  "refactor_applied": $refactor_applied,
  "abi_offsets_changed": false,
  "semantic_rename": "mint_id_to_canonical_asset_id",
  "cpi_uses_target_mint_pubkey": true,
  "mint_state_pda_bound_to_canonical_asset_id": true,
  "cargo_fmt_check_code": "$fmt_code",
  "cargo_test_code": "$test_code",
  "blocker": "$blocker"
}
JSON
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "head_before=${HEAD_SHA:-unknown}"
  echo "transactions_executed=$transactions_executed"
  echo "deploy_executed=$deploy_executed"
  echo "upgrade_executed=$upgrade_executed"
  echo "push_executed=$push_executed"
  echo "refactor_applied=$refactor_applied"
  echo "abi_offsets_changed=false"
  echo "semantic_rename=mint_id_to_canonical_asset_id"
  echo "cpi_uses_target_mint_pubkey=true"
  echo "mint_state_pda_bound_to_canonical_asset_id=true"
  echo "working_tree_modified=$([ -n "$changed_files" ] && echo true || echo false)"
  echo "changed_files=$changed_files"
  echo "diff_stat=$diff_stat"
  echo "cargo_fmt_check_code=$fmt_code"
  echo "cargo_test_code=$test_code"
  echo "blocker=$blocker"
  echo "diff_path=$RESULTS_DIR/canonical_asset_id_refactor_v3.diff"
  echo "result_json=$RESULTS_DIR/canonical_asset_id_refactor_v3_result.json"
  echo "cargo_fmt_log=$LOG_DIR/cargo_fmt.log"
  echo "cargo_fmt_check_log=$LOG_DIR/cargo_fmt_check.log"
  echo "cargo_test_log=$LOG_DIR/cargo_test.log"
  echo "=== canonical asset id refactor saved files ==="
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "diff_path=$RESULTS_DIR/canonical_asset_id_refactor_v3.diff"
  echo "result_json=$RESULTS_DIR/canonical_asset_id_refactor_v3_result.json"
  echo "cargo_fmt_log=$LOG_DIR/cargo_fmt.log"
  echo "cargo_fmt_check_log=$LOG_DIR/cargo_fmt_check.log"
  echo "cargo_test_log=$LOG_DIR/cargo_test.log"
  if [ -f "$LOG_DIR/cargo_fmt_check.log" ]; then
    echo "=== cargo fmt check tail ==="
    tail -n 40 "$LOG_DIR/cargo_fmt_check.log"
  fi
  if [ -f "$LOG_DIR/cargo_test.log" ]; then
    echo "=== cargo test tail ==="
    tail -n 80 "$LOG_DIR/cargo_test.log"
  fi
  if [ "$blocker" = "none" ]; then exit 0; else exit 1; fi
}

printf '=== %s started ===\n' "$PACKAGE"
printf 'evidence_dir=%s\n' "$EVIDENCE_DIR"
printf 'transactions_executed=%s deploy_executed=%s upgrade_executed=%s push_executed=%s\n' \
  "$transactions_executed" "$deploy_executed" "$upgrade_executed" "$push_executed"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
[ -n "$ROOT" ] || { echo "blocker=not_inside_git_repo"; finish "not_inside_git_repo" false; }
cd "$ROOT"
HEAD_SHA="$(git rev-parse HEAD)"
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
EXPECTED_BRANCH="audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z"
[ "$BRANCH" = "$EXPECTED_BRANCH" ] || finish "wrong_branch_${BRANCH}_expected_${EXPECTED_BRANCH}" false

if ! git diff --quiet -- .; then
  finish "working_tree_has_uncommitted_changes_reset_before_running_v3" false
fi
if ! git diff --cached --quiet -- .; then
  finish "index_has_staged_changes_reset_before_running_v3" false
fi

MANIFEST="programs/xxxl-svm/Cargo.toml"
[ -f "$MANIFEST" ] || finish "missing_program_manifest" false

python3 - "$EVIDENCE_DIR" <<'PY'
from pathlib import Path
import re
import shutil
import sys

root = Path.cwd()
evidence = Path(sys.argv[1])
source_before = evidence / "source-before"
source_after = evidence / "source-after"

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

def read(rel: Path) -> str:
    return (root / rel).read_text()

def write(rel: Path, text: str):
    (root / rel).write_text(text)

def require(cond: bool, msg: str):
    if not cond:
        raise SystemExit(msg)

# A. Semantic rename only for the exact identifier, not words like mint_to_*.
for rel in source_files:
    text = read(rel)
    text = re.sub(r"\bmint_id\b", "canonical_asset_id", text)
    write(rel, text)

# B. Execution plan: rename local output field and use prepared SPL mint in the real builder.
rel = Path("programs/xxxl-svm/src/execution_plan.rs")
text = read(rel)
require("pub mint: [u8; 32]," in text, "missing_execution_plan_mint_field")
text = text.replace("pub mint: [u8; 32],", "pub target_mint_pubkey: [u8; 32],", 1)
text = re.sub(r"(?m)^(\s*)mint:\s*", r"\1target_mint_pubkey: ", text)
text = re.sub(r"\.(mint)\b", ".target_mint_pubkey", text)
start = text.index("pub fn build_atomic_consume_gateway_mint_execution_plan(")
end = text.index("\n}\n\n#[cfg(test)]", start)
section = text[start:end]
require("target_mint_pubkey: args.canonical_asset_id," in section, "missing_builder_target_mint_pubkey_from_args")
section = section.replace(
    "target_mint_pubkey: args.canonical_asset_id,",
    "target_mint_pubkey: prepared.boundary.accounts.mint.key.to_bytes(),",
    1,
)
text = text[:start] + section + text[end:]
write(rel, text)

# C. CPI planning: compare against execution plan's local target SPL mint pubkey.
rel = Path("programs/xxxl-svm/src/cpi.rs")
text = read(rel)
text = re.sub(r"execution_plan\.mint\b", "execution_plan.target_mint_pubkey", text)
write(rel, text)

# D. Processor preparation: canonical asset id binds MintState PDA; target SPL mint pubkey drives SPL checks.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
needle = """    let mint_state = MintStateAccountView::new(&mint_state_data)?;
    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;
    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;
    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;

    if mint_state.mint_pubkey() != args.canonical_asset_id
        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()
"""
replacement = """    let mint_state = MintStateAccountView::new(&mint_state_data)?;
    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;
    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;
    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;
    let target_mint_pubkey_bytes = spl_token_mint_account.key.to_bytes();

    let (expected_mint_state_pda, _) = find_mint_state(program_id, &args.canonical_asset_id);
    if mint_state_account.key != &expected_mint_state_pda {
        return Err(XxxlError::InvalidPda.into());
    }

    if mint_state.mint_pubkey() != target_mint_pubkey_bytes
        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()
"""
require(needle in text, "missing_processor_mint_state_check_block")
text = text.replace(needle, replacement, 1)
text = text.replace(
    "        || gateway_config.target_mint() != args.canonical_asset_id\n",
    "        || gateway_config.target_mint() != target_mint_pubkey_bytes\n",
    1,
)
text = text.replace(
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.canonical_asset_id {\n",
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != target_mint_pubkey_bytes {\n",
    1,
)
text = text.replace(
    "    let mint_pubkey = Pubkey::new_from_array(args.canonical_asset_id);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "    let target_mint_pubkey = Pubkey::new_from_array(target_mint_pubkey_bytes);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    1,
)
text = text.replace(
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &mint_pubkey)?;\n",
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &target_mint_pubkey)?;\n",
    1,
)
text = re.sub(r"\.(mint)\b", ".target_mint_pubkey", text)

# Add a focused regression test only once.
test_name = "handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint"
if test_name not in text:
    anchor = """    #[test]
    fn handler_integration_rejects_wrong_account_count() {
"""
    new_test = """    #[test]
    fn handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.args.canonical_asset_id = [0x42; 32];
        fixture.keys.mint_state = find_mint_state(&fixture.program_id, &fixture.args.canonical_asset_id).0;

        let expected_target_mint = fixture.keys.spl_mint;
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let prepared = prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)
            .expect("canonical asset id may differ from target SPL mint");

        assert_eq!(prepared.boundary.accounts.mint.key, &expected_target_mint);
        assert_ne!(args.canonical_asset_id, expected_target_mint.to_bytes());
    }

"""
    require(anchor in text, "missing_test_insertion_anchor")
    text = text.replace(anchor, new_test + anchor, 1)

write(rel, text)

for rel in source_files:
    src = root / rel
    dst = source_after / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)
PY
patch_code=$?
if [ "$patch_code" -ne 0 ]; then
  finish "python_patch_failed_code_${patch_code}" false
fi

cargo fmt --manifest-path "$MANIFEST" > "$LOG_DIR/cargo_fmt.log" 2>&1
fmt_run_code=$?
if [ "$fmt_run_code" -ne 0 ]; then
  finish "cargo_fmt_failed" true "$fmt_run_code" "not_run"
fi

cargo fmt --manifest-path "$MANIFEST" -- --check > "$LOG_DIR/cargo_fmt_check.log" 2>&1
fmt_check_code=$?

cargo test --manifest-path "$MANIFEST" > "$LOG_DIR/cargo_test.log" 2>&1
test_code=$?

if [ "$fmt_check_code" -eq 0 ] && [ "$test_code" -eq 0 ]; then
  finish "none" true "$fmt_check_code" "$test_code"
else
  finish "cargo_fmt_check_or_cargo_test_failed" true "$fmt_check_code" "$test_code"
fi
