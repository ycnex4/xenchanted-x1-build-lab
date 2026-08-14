#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

SOURCE_COMMIT="6b3a2c6ffa1c7da3b61c0e080fc551ece49d716f"
SCRIPT="audit/minimal-live-smoke-auth-schema-v2/scripts/option1-v3-candidate-no-execution.cjs"
BASE="$HOME/xenchanted-stage20-activation-evidence-c332814"
EVIDENCE_DIR="$BASE/runtime-state-provisioning-minimal-live-smoke-option1-v3-candidate-no-execution-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$EVIDENCE_DIR/logs" "$EVIDENCE_DIR/results"

if [ ! -f "$SCRIPT" ]; then
  echo "STOP: v3 candidate script not found: $SCRIPT"
  echo "Make sure you pulled the latest audit branch."
  exit 1
fi

if ! git diff --quiet "$SOURCE_COMMIT" -- programs/xxxl-svm/src; then
  echo "STOP: programs/xxxl-svm/src differs from audited source commit $SOURCE_COMMIT"
  git status --short -- programs/xxxl-svm/src || true
  exit 1
fi

if [ -n "$(git status --short -- programs/xxxl-svm/src)" ]; then
  echo "STOP: uncommitted changes under programs/xxxl-svm/src"
  git status --short -- programs/xxxl-svm/src
  exit 1
fi

PAYER_KEYPAIR="${XXXL_PAYER_KEYPAIR:-$(solana config get 2>/dev/null | sed -n 's/^Keypair Path: //p' | tr -d '\r' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')}"

if [ -z "$PAYER_KEYPAIR" ]; then
  echo "STOP: payer keypair missing. Set XXXL_PAYER_KEYPAIR or solana config keypair."
  exit 1
fi

for v in XXXL_GUARDIAN_0_KEYPAIR XXXL_GUARDIAN_1_KEYPAIR XXXL_GUARDIAN_2_KEYPAIR; do
  if [ -z "${!v:-}" ]; then
    echo "STOP: $v is not set"
    exit 1
  fi
  if [ ! -f "${!v}" ]; then
    echo "STOP: $v path is not readable"
    exit 1
  fi
done

NODE_DEPS="$EVIDENCE_DIR/node-deps"
mkdir -p "$NODE_DEPS"

if ! node -e "require('@solana/web3.js'); require('tweetnacl')" >/dev/null 2>&1; then
  echo "Installing local node deps into evidence dir. Log: $EVIDENCE_DIR/logs/npm-install.log"
  npm --prefix "$NODE_DEPS" install @solana/web3.js@1 tweetnacl > "$EVIDENCE_DIR/logs/npm-install.log" 2>&1 || {
    echo "STOP: npm install failed. Log tail:"
    tail -n 80 "$EVIDENCE_DIR/logs/npm-install.log" || true
    exit 1
  }
  export NODE_PATH="$NODE_DEPS/node_modules${NODE_PATH:+:$NODE_PATH}"
fi

export PAYER_KEYPAIR
export EVIDENCE_DIR

echo "=== option1 v3 candidate no-execution runner ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "script=$SCRIPT"
echo "transactions_executed=false"
echo "deploy_executed=false"
echo "upgrade_executed=false"
echo "push_executed=false"
echo

node "$SCRIPT" 2>&1 | tee "$EVIDENCE_DIR/logs/option1-v3-candidate-no-execution.log"

echo
echo "=== option1 v3 candidate no-execution saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
[ -f "$EVIDENCE_DIR/results/option1_v3_candidate_result.json" ] && echo "result_json=$EVIDENCE_DIR/results/option1_v3_candidate_result.json"
[ -f "$EVIDENCE_DIR/results/option1_v3_candidate_transaction_bytes.b64" ] && echo "candidate_tx_bytes=$EVIDENCE_DIR/results/option1_v3_candidate_transaction_bytes.b64"
[ -f "$EVIDENCE_DIR/results/option1_v3_candidate_consume_keys.json" ] && echo "consume_keys=$EVIDENCE_DIR/results/option1_v3_candidate_consume_keys.json"
