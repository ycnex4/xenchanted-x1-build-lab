#!/usr/bin/env bash
set -euo pipefail

PACKAGE="runtime-source-canonical-asset-id-refactor-source-only-v3-quiet-wrapper"
BASE_DIR="${XXXL_EVIDENCE_BASE:-$HOME/xenchanted-stage20-activation-evidence-c332814}"
TS="$(date -u +%Y%m%dT%H%M%SZ)"
EVIDENCE_DIR="$BASE_DIR/${PACKAGE}-${TS}"
LOG_DIR="$EVIDENCE_DIR/logs"
RESULTS_DIR="$EVIDENCE_DIR/results"
mkdir -p "$LOG_DIR" "$RESULTS_DIR"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [ -z "$ROOT" ]; then
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "transactions_executed=false"
  echo "deploy_executed=false"
  echo "upgrade_executed=false"
  echo "push_executed=false"
  echo "blocker=not_inside_git_repo"
  exit 1
fi
cd "$ROOT"

RUNNER="audit/minimal-live-smoke-auth-schema-v2/run-canonical-asset-id-refactor-source-only-v3.sh"
FULL_LOG="$LOG_DIR/canonical_asset_id_refactor_v3_full_output.log"
SUMMARY_FILE="$RESULTS_DIR/quiet_wrapper_summary.txt"
ERROR_SUMMARY_FILE="$RESULTS_DIR/quiet_wrapper_error_summary.txt"

if [ ! -f "$RUNNER" ]; then
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "transactions_executed=false"
  echo "deploy_executed=false"
  echo "upgrade_executed=false"
  echo "push_executed=false"
  echo "blocker=missing_child_runner_$RUNNER"
  exit 1
fi

set +e
bash "$RUNNER" >"$FULL_LOG" 2>&1
CHILD_CODE=$?
set -e

# Keep terminal compact. The full noisy output remains in FULL_LOG.
{
  echo "=== canonical asset id refactor source-only compact summary for chat ==="
  echo "package=$PACKAGE"
  echo "evidence_dir=$EVIDENCE_DIR"
  echo "child_runner=$RUNNER"
  echo "child_exit_code=$CHILD_CODE"
  echo "transactions_executed=false"
  echo "deploy_executed=false"
  echo "upgrade_executed=false"
  echo "push_executed=false"
  echo "full_child_log=$FULL_LOG"

  awk '
    /^=== canonical asset id refactor source-only compact summary for chat ===/ { in_summary=1; next }
    /^=== canonical asset id refactor saved files ===/ { in_summary=0; next }
    /^=== cargo / { in_summary=0; next }
    in_summary && /^[A-Za-z0-9_]+=/{ print "child_" $0 }
  ' "$FULL_LOG"

  if [ "$CHILD_CODE" -eq 0 ]; then
    echo "quiet_wrapper_verdict=child_runner_passed"
  else
    echo "quiet_wrapper_verdict=child_runner_failed_see_logs"
  fi
} | tee "$SUMMARY_FILE"

{
  grep -E '^(error\[|error:|warning:|blocker=|cargo_fmt_check_code=|cargo_test_code=)|unknown field|method not found|no field named|no field `|does not have this field|available fields are' "$FULL_LOG" | tail -n 60 || true
} > "$ERROR_SUMMARY_FILE"

if [ "$CHILD_CODE" -ne 0 ]; then
  echo "=== compact error summary ==="
  if [ -s "$ERROR_SUMMARY_FILE" ]; then
    cat "$ERROR_SUMMARY_FILE"
  else
    echo "no_compact_error_lines_found=true"
  fi
fi

echo "=== quiet wrapper saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "full_child_log=$FULL_LOG"
echo "summary_file=$SUMMARY_FILE"
echo "error_summary_file=$ERROR_SUMMARY_FILE"

exit "$CHILD_CODE"
