#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

SCRIPT="audit/minimal-live-smoke-auth-schema-v2/scripts/option1-diagnose-transaction.cjs"
BASE="$HOME/xenchanted-stage20-activation-evidence-c332814"
EVIDENCE_DIR="$BASE/runtime-state-provisioning-minimal-live-smoke-option1-diagnose-transaction-no-execution-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$EVIDENCE_DIR/logs" "$EVIDENCE_DIR/results"

if [ ! -f "$SCRIPT" ]; then
  echo "STOP: diagnose script not found: $SCRIPT"
  echo "Make sure you pulled the latest audit branch."
  exit 1
fi

if [ "${1:-}" != "" ]; then
  TX_SOURCE_DIR="$1"
else
  TX_SOURCE_DIR="$(find "$BASE" -maxdepth 1 -type d -name 'runtime-state-provisioning-minimal-live-smoke-option1-structure-only-execution-v2-*' | sort | tail -n 1)"
fi

if [ -z "$TX_SOURCE_DIR" ]; then
  echo "STOP: no option1 execution-v2 evidence dir found. Pass TX_SOURCE_DIR as first arg."
  exit 1
fi

if [ ! -f "$TX_SOURCE_DIR/results/option1_transaction_bytes.b64" ]; then
  echo "STOP: transaction bytes not found in: $TX_SOURCE_DIR/results/option1_transaction_bytes.b64"
  exit 1
fi

NODE_DEPS="$EVIDENCE_DIR/node-deps"
mkdir -p "$NODE_DEPS"

if ! node -e "require('@solana/web3.js')" >/dev/null 2>&1; then
  echo "Installing local @solana/web3.js into evidence dir. Log: $EVIDENCE_DIR/logs/npm-install.log"
  npm --prefix "$NODE_DEPS" install @solana/web3.js@1 > "$EVIDENCE_DIR/logs/npm-install.log" 2>&1 || {
    echo "STOP: npm install failed. Log tail:"
    tail -n 80 "$EVIDENCE_DIR/logs/npm-install.log" || true
    exit 1
  }
  export NODE_PATH="$NODE_DEPS/node_modules${NODE_PATH:+:$NODE_PATH}"
fi

export TX_SOURCE_DIR
export EVIDENCE_DIR

echo "=== option1 diagnose transaction runner ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "tx_source_dir=$TX_SOURCE_DIR"
echo "script=$SCRIPT"
echo "transactions_executed=false"
echo

node "$SCRIPT" 2>&1 | tee "$EVIDENCE_DIR/logs/option1-diagnose-transaction.log"

echo
echo "=== option1 diagnose transaction saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
[ -f "$EVIDENCE_DIR/results/option1_diagnose_transaction_result.json" ] && echo "result_json=$EVIDENCE_DIR/results/option1_diagnose_transaction_result.json"
[ -f "$EVIDENCE_DIR/results/option1_consume_key_diagnostics.json" ] && echo "key_diagnostics=$EVIDENCE_DIR/results/option1_consume_key_diagnostics.json"
