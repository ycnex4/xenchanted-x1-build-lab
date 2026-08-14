#!/usr/bin/env bash
set -u

PACKAGE="runtime-source-canonical-asset-id-refactor-source-only-v4-quiet"
BASE_DIR="${XXXL_EVIDENCE_BASE:-$HOME/xenchanted-stage20-activation-evidence-c332814}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
EVIDENCE_DIR="$BASE_DIR/${PACKAGE}-${TS}"
RESULTS_DIR="$EVIDENCE_DIR/results"
LOG_DIR="$EVIDENCE_DIR/logs"
SOURCE_BEFORE_DIR="$EVIDENCE_DIR/source-before"
SOURCE_AFTER_DIR="$EVIDENCE_DIR/source-after"

mkdir -p "$RESULTS_DIR" "$LOG_DIR" "$SOURCE_BEFORE_DIR" "$SOURCE_AFTER_DIR"
FULL_LOG="$LOG_DIR/${PACKAGE}_full_output.log"
SUMMARY_FILE="$RESULTS_DIR/${PACKAGE}_summary.txt"
ERROR_SUMMARY_FILE="$RESULTS_DIR/${PACKAGE}_error_summary.txt"

transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false

run_body() {
  set -euo pipefail

  ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  [ -n "$ROOT" ] || { echo "blocker=not_inside_git_repo" > "$RESULTS_DIR/status.env"; exit 1; }
  cd "$ROOT"

  BRANCH="$(git rev-parse --abbrev-ref HEAD)"
  HEAD_SHA="$(git rev-parse HEAD)"
  EXPECTED_BRANCH="audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z"

  {
    echo "package=$PACKAGE"
    echo "evidence_dir=$EVIDENCE_DIR"
    echo "head_before=$HEAD_SHA"
    echo "transactions_executed=$transactions_executed"
    echo "deploy_executed=$deploy_executed"
    echo "upgrade_executed=$upgrade_executed"
    echo "push_executed=$push_executed"
  } > "$RESULTS_DIR/status.env"

  if [ "$BRANCH" != "$EXPECTED_BRANCH" ]; then
    echo "blocker=wrong_branch_${BRANCH}_expected_${EXPECTED_BRANCH}" >> "$RESULTS_DIR/status.env"
    exit 1
  fi
  if ! git diff --quiet -- .; then
    echo "blocker=working_tree_has_uncommitted_changes" >> "$RESULTS_DIR/status.env"
    exit 1
  fi
  if ! git diff --cached --quiet -- .; then
    echo "blocker=index_has_staged_changes" >> "$RESULTS_DIR/status.env"
    exit 1
  fi

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
    Path("programs/xxxl-svm/src/pda.rs"),
    Path("programs/xxxl-svm/src/execution_plan.rs"),
    Path("programs/xxxl-svm/src/cpi.rs"),
    Path("programs/xxxl-svm/src/processor.rs"),
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

def count_replace(text: str, old: str, new: str, label: str, min_count: int = 0) -> str:
    count = text.count(old)
    if count < min_count:
        raise SystemExit(f"missing_pattern={label}; count={count}; min={min_count}")
    if count:
        changes.append((label, count))
        text = text.replace(old, new)
    return text

def replace_exact_once(text: str, old: str, new: str, label: str) -> str:
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"unexpected_pattern_count={label}; count={count}; expected=1")
    changes.append((label, count))
    return text.replace(old, new)

def rename_field_in_initializers(text: str, struct_name: str, old_field: str, new_field: str) -> str:
    needle = struct_name + " {"
    out = []
    pos = 0
    replacements = 0
    while True:
        start = text.find(needle, pos)
        if start == -1:
            out.append(text[pos:])
            break
        out.append(text[pos:start])
        brace = text.find("{", start)
        depth = 0
        end = None
        i = brace
        while i < len(text):
            ch = text[i]
            if ch == "{":
                depth += 1
            elif ch == "}":
                depth -= 1
                if depth == 0:
                    end = i + 1
                    break
            i += 1
        if end is None:
            raise SystemExit(f"unclosed_initializer={struct_name}_at_{start}")
        block = text[start:end]
        new_block, n = re.subn(rf"(^\s*){re.escape(old_field)}\s*:", rf"\1{new_field}:", block, flags=re.M)
        replacements += n
        out.append(new_block)
        pos = end
    if replacements:
        changes.append((f"{struct_name}.{old_field}_to_{new_field}", replacements))
    return "".join(out)

# instruction.rs: ABI offsets stay unchanged; only semantic field names change.
rel = Path("programs/xxxl-svm/src/instruction.rs")
text = read(rel)
text = count_replace(text, "mint_id", "canonical_asset_id", "instruction_mint_id_to_canonical_asset_id", min_count=2)
write(rel, text)

# pda.rs: MintState PDA seed parameter is the canonical asset id.
rel = Path("programs/xxxl-svm/src/pda.rs")
text = read(rel)
text = count_replace(text, "mint_id", "canonical_asset_id", "pda_mint_id_to_canonical_asset_id", min_count=1)
write(rel, text)

# execution_plan.rs: execution plan carries the local target SPL mint, not the canonical asset id.
rel = Path("programs/xxxl-svm/src/execution_plan.rs")
text = read(rel)
text = count_replace(text, "args.mint_id", "args.canonical_asset_id", "execution_plan_args_field_rename", min_count=1)
text = replace_exact_once(text, "    pub mint: [u8; 32],\n", "    pub target_mint_pubkey: [u8; 32],\n", "execution_plan_struct_target_mint_pubkey")
text = rename_field_in_initializers(text, "AtomicConsumeGatewayMintExecutionPlan", "mint", "target_mint_pubkey")
text = count_replace(
    text,
    "target_mint_pubkey: args.canonical_asset_id,",
    "target_mint_pubkey: prepared.boundary.accounts.mint.key.to_bytes(),",
    "execution_plan_uses_prepared_boundary_target_mint",
    min_count=1,
)
text = count_replace(text, "execution_plan.mint", "execution_plan.target_mint_pubkey", "execution_plan_dot_mint_refs")
text = count_replace(text, "plan.mint", "plan.target_mint_pubkey", "plan_dot_mint_refs")
text = count_replace(text, "args.mint_id", "args.canonical_asset_id", "execution_plan_remaining_args_field_rename")
write(rel, text)

# cpi.rs: CPI planning compares against the execution plan's target SPL mint.
rel = Path("programs/xxxl-svm/src/cpi.rs")
text = read(rel)
text = count_replace(text, "execution_plan.mint", "execution_plan.target_mint_pubkey", "cpi_execution_plan_mint_refs", min_count=1)
write(rel, text)

# processor.rs: runtime preparation separates canonical asset id from target SPL mint.
rel = Path("programs/xxxl-svm/src/processor.rs")
text = read(rel)
text = count_replace(text, "args.mint_id", "args.canonical_asset_id", "processor_args_field_rename", min_count=1)
text = count_replace(text, "mint_id:", "canonical_asset_id:", "processor_struct_literal_field_rename")
text = count_replace(text, "execution_plan.mint", "execution_plan.target_mint_pubkey", "processor_execution_plan_mint_refs")
text = count_replace(text, "plan.mint", "plan.target_mint_pubkey", "processor_plan_mint_refs")
text = count_replace(text, "composition.execution_plan.mint", "composition.execution_plan.target_mint_pubkey", "processor_composition_plan_mint_refs")

old = """    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n\n    if mint_state.mint_pubkey() != args.canonical_asset_id\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n"""
new = """    let mint_state = MintStateAccountView::new(&mint_state_data)?;\n    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;\n    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;\n    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;\n    let target_mint_pubkey_bytes = spl_token_mint_account.key.to_bytes();\n\n    let (expected_mint_state_pda, _) = find_mint_state(program_id, &args.canonical_asset_id);\n    if mint_state_account.key != &expected_mint_state_pda {\n        return Err(XxxlError::InvalidPda.into());\n    }\n\n    if mint_state.mint_pubkey() != target_mint_pubkey_bytes\n        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()\n"""
text = replace_exact_once(text, old, new, "processor_prepare_target_mint_and_canonical_pda")
text = replace_exact_once(
    text,
    "        || gateway_config.target_mint() != args.canonical_asset_id\n",
    "        || gateway_config.target_mint() != target_mint_pubkey_bytes\n",
    "processor_gateway_config_target_mint_check",
)
text = replace_exact_once(
    text,
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != args.canonical_asset_id {\n",
    "    if recipient_balance.owner() != args.recipient || recipient_balance.mint() != target_mint_pubkey_bytes {\n",
    "processor_recipient_balance_target_mint_check",
)
text = replace_exact_once(
    text,
    "    let mint_pubkey = Pubkey::new_from_array(args.canonical_asset_id);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "    let target_mint_pubkey = Pubkey::new_from_array(target_mint_pubkey_bytes);\n    let recipient_owner = Pubkey::new_from_array(args.recipient);\n",
    "processor_target_mint_pubkey_pubkey_value",
)
text = replace_exact_once(
    text,
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &mint_pubkey)?;\n",
    "    assert_recipient_ata_boundary(recipient_token_account, &recipient_owner, &target_mint_pubkey)?;\n",
    "processor_ata_boundary_target_mint",
)

anchor = """    #[test]\n    fn consume_gateway_mint_v2_happy_path_matches_gateway_config() {\n        let mut fixture = HandlerFixture::new();\n        let program_id = fixture.program_id;\n        let args = fixture.args;\n        let rent = Rent::default();\n        let accounts = fixture.accounts();\n\n        prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)\n            .expect(\"v2 source_chain_id matches GatewayConfig\");\n\n        assert_eq!(args.source_chain_id, 1);\n    }\n"""
new_test = anchor + """\n    #[test]\n    fn handler_integration_allows_canonical_asset_id_distinct_from_target_spl_mint() {\n        let mut fixture = HandlerFixture::new();\n        let target_spl_mint = fixture.keys.spl_mint;\n        let (canonical_mint_state, _) = find_mint_state(&fixture.program_id, &[0x42; 32]);\n\n        fixture.keys.mint_state = canonical_mint_state;\n        fixture.args.canonical_asset_id = [0x42; 32];\n\n        let program_id = fixture.program_id;\n        let args = fixture.args;\n        let rent = Rent::default();\n        let accounts = fixture.accounts();\n\n        let prepared = prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent)\n            .expect(\"canonical asset id may differ from target SPL mint\");\n\n        assert_ne!(args.canonical_asset_id, target_spl_mint.to_bytes());\n        assert_eq!(\n            prepared.boundary.accounts.mint.key.to_bytes(),\n            target_spl_mint.to_bytes()\n        );\n    }\n"""
text = replace_exact_once(text, anchor, new_test, "processor_distinct_canonical_asset_regression_test")
write(rel, text)

# Copy after-images.
for rel in source_files:
    dst = source_after / rel
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(root / rel, dst)

summary = source_after / "refactor_changes.tsv"
with summary.open("w") as f:
    for label, count in changes:
        f.write(f"{label}\t{count}\n")
PY

  CHANGED_TARGETS="programs/xxxl-svm/src/instruction.rs programs/xxxl-svm/src/pda.rs programs/xxxl-svm/src/execution_plan.rs programs/xxxl-svm/src/cpi.rs programs/xxxl-svm/src/processor.rs"

  rustfmt --edition 2021 $CHANGED_TARGETS > "$LOG_DIR/rustfmt_changed_files.log" 2>&1
  rustfmt_code=$?

  rustfmt --edition 2021 --check $CHANGED_TARGETS > "$LOG_DIR/rustfmt_changed_files_check.log" 2>&1
  rustfmt_check_code=$?

  cargo test --manifest-path programs/xxxl-svm/Cargo.toml > "$LOG_DIR/cargo_test.log" 2>&1
  cargo_test_code=$?

  git diff -- $CHANGED_TARGETS > "$RESULTS_DIR/canonical_asset_id_refactor_v4.diff"
  git diff --stat -- $CHANGED_TARGETS | tr '\n' ' ' > "$RESULTS_DIR/diff_stat.txt"

  changed_files="$(git diff --name-only -- $CHANGED_TARGETS | tr '\n' ' ')"
  diff_stat="$(cat "$RESULTS_DIR/diff_stat.txt")"

  {
    echo "refactor_applied=true"
    echo "abi_offsets_changed=false"
    echo "semantic_rename=mint_id_to_canonical_asset_id"
    echo "pda_seed_semantics=canonical_asset_id"
    echo "cpi_uses_target_mint_pubkey=true"
    echo "recipient_balance_uses_target_mint_pubkey=true"
    echo "changed_files=$changed_files"
    echo "diff_stat=$diff_stat"
    echo "rustfmt_changed_files_code=$rustfmt_code"
    echo "rustfmt_changed_files_check_code=$rustfmt_check_code"
    echo "cargo_test_code=$cargo_test_code"
    echo "diff_path=$RESULTS_DIR/canonical_asset_id_refactor_v4.diff"
    echo "rustfmt_log=$LOG_DIR/rustfmt_changed_files.log"
    echo "rustfmt_check_log=$LOG_DIR/rustfmt_changed_files_check.log"
    echo "cargo_test_log=$LOG_DIR/cargo_test.log"
  } >> "$RESULTS_DIR/status.env"

  if [ "$rustfmt_code" -ne 0 ] || [ "$rustfmt_check_code" -ne 0 ] || [ "$cargo_test_code" -ne 0 ]; then
    echo "blocker=rustfmt_or_cargo_test_failed" >> "$RESULTS_DIR/status.env"
    exit 1
  fi

  echo "blocker=none" >> "$RESULTS_DIR/status.env"
}

set +e
run_body > "$FULL_LOG" 2>&1
child_code=$?
set -e

# Compact error summary from logs, without dumping full cargo output.
{
  if [ -f "$RESULTS_DIR/status.env" ]; then
    grep -E '^(blocker|rustfmt_changed_files_code|rustfmt_changed_files_check_code|cargo_test_code)=' "$RESULTS_DIR/status.env" || true
  fi
  if [ -f "$LOG_DIR/cargo_test.log" ]; then
    grep -E '^(error\[|error: could not compile|warning: build failed|\s*\|.*\^)' "$LOG_DIR/cargo_test.log" | head -n 30 || true
  fi
} > "$ERROR_SUMMARY_FILE"

{
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  if [ -f "$RESULTS_DIR/status.env" ]; then
    cat "$RESULTS_DIR/status.env"
  else
    echo "package=$PACKAGE"
    echo "evidence_dir=$EVIDENCE_DIR"
    echo "transactions_executed=$transactions_executed"
    echo "deploy_executed=$deploy_executed"
    echo "upgrade_executed=$upgrade_executed"
    echo "push_executed=$push_executed"
    echo "refactor_applied=false"
    echo "blocker=runner_failed_before_status_file"
  fi
  echo "full_runner_log=$FULL_LOG"
  if [ "$child_code" -ne 0 ]; then
    echo "quiet_runner_verdict=failed_see_logs"
    echo "=== compact error summary ==="
    cat "$ERROR_SUMMARY_FILE"
  else
    echo "quiet_runner_verdict=pass"
  fi
  echo "=== quiet refactor saved files ==="
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "summary_file=$SUMMARY_FILE"
  echo "error_summary_file=$ERROR_SUMMARY_FILE"
  echo "full_runner_log=$FULL_LOG"
} | tee "$SUMMARY_FILE"

exit "$child_code"
