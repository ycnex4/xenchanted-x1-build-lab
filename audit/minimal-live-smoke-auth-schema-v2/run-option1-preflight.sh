#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

SOURCE_COMMIT="6b3a2c6ffa1c7da3b61c0e080fc551ece49d716f"
SCRIPT="audit/minimal-live-smoke-auth-schema-v2/scripts/option1-preflight.js"
BASE="$HOME/xenchanted-stage20-activation-evidence-c332814"
EVIDENCE_DIR="$BASE/runtime-state-provisioning-minimal-live-smoke-option1-builder-preflight-no-execution-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$EVIDENCE_DIR/logs" "$EVIDENCE_DIR/results"

if [ ! -f "$SCRIPT" ]; then
  echo "STOP: preflight script not found: $SCRIPT"
  echo "Make sure you checked out audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z"
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

RPC_URL="${SOLANA_RPC_URL:-$(solana config get 2>/dev/null | sed -n 's/^RPC URL: //p' | tr -d '\r' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')}"
PAYER_KEYPAIR="${XXXL_PAYER_KEYPAIR:-$(solana config get 2>/dev/null | sed -n 's/^Keypair Path: //p' | tr -d '\r' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')}"

if [ -z "$RPC_URL" ]; then
  echo "STOP: RPC_URL missing. Set SOLANA_RPC_URL or solana config url."
  exit 1
fi

if echo "$RPC_URL" | grep -qi mainnet; then
  echo "STOP: RPC URL looks like mainnet: $RPC_URL"
  exit 1
fi

if [ -z "$PAYER_KEYPAIR" ]; then
  echo "STOP: payer keypair missing. Set XXXL_PAYER_KEYPAIR or solana config keypair."
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

export RPC_URL
export PAYER_KEYPAIR
export EVIDENCE_DIR

# The repository package.json uses "type": "module". The preflight script is
# intentionally CommonJS, so run a temporary .cjs copy from the evidence dir.
SCRIPT_CJS="$EVIDENCE_DIR/option1-preflight.cjs"
cp "$SCRIPT" "$SCRIPT_CJS"

echo "=== option1 preflight runner ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "script=$SCRIPT"
echo "runtime_script=$SCRIPT_CJS"
echo "transactions_executed=false"
echo

node "$SCRIPT_CJS" 2>&1 | tee "$EVIDENCE_DIR/logs/option1-preflight.log"

echo
echo "=== option1 preflight saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
[ -f "$EVIDENCE_DIR/results/option1_preflight_result.json" ] && echo "result_json=$EVIDENCE_DIR/results/option1_preflight_result.json"
