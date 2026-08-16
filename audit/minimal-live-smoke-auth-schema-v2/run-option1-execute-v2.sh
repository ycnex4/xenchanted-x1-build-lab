#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

SOURCE_COMMIT="240e3e89100893939339ee5cc1476298e1ea4571"
SCRIPT="audit/minimal-live-smoke-auth-schema-v2/scripts/option1-execute.cjs"
BASE="$HOME/xenchanted-stage20-activation-evidence-c332814"
EVIDENCE_DIR="$BASE/runtime-state-provisioning-minimal-live-smoke-option1-structure-only-execution-v2-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$EVIDENCE_DIR/logs" "$EVIDENCE_DIR/results"

if [ ! -f "$SCRIPT" ]; then
  echo "STOP: execution script not found: $SCRIPT"
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

export RPC_URL
export PAYER_KEYPAIR
export EVIDENCE_DIR

SCRIPT_RUNTIME="$EVIDENCE_DIR/option1-execute-v2.cjs"
cp "$SCRIPT" "$SCRIPT_RUNTIME"

# B1 V3 account contract requires mint_state at account index 0 to be readonly.
# The original script made it writable, causing custom error 1 before the expected CPI gate.
perl -0pi -e 's/\{ pubkey: MINT_STATE_PDA, isSigner: false, isWritable: true \}/\{ pubkey: MINT_STATE_PDA, isSigner: false, isWritable: false \}/g' "$SCRIPT_RUNTIME"

if ! grep -q '{ pubkey: MINT_STATE_PDA, isSigner: false, isWritable: false }' "$SCRIPT_RUNTIME"; then
  echo "STOP: runtime patch did not apply"
  exit 1
fi

echo "=== option1 execution v2 runner ==="
echo "evidence_dir=$EVIDENCE_DIR"
echo "script=$SCRIPT"
echo "runtime_script=$SCRIPT_RUNTIME"
echo "patch=mint_state_readonly_per_b1_v3_account_contract"
echo "expected_result=tx_lands_then_CpiBoundaryNotReady_no_state_mutation"
echo "deploy_executed=false"
echo "upgrade_executed=false"
echo "push_executed=false"
echo

node "$SCRIPT_RUNTIME" 2>&1 | tee "$EVIDENCE_DIR/logs/option1-execute-v2.log"

echo
echo "=== option1 execution v2 saved files ==="
echo "evidence_dir=$EVIDENCE_DIR"
[ -f "$EVIDENCE_DIR/results/option1_execute_result.json" ] && echo "result_json=$EVIDENCE_DIR/results/option1_execute_result.json"
[ -f "$EVIDENCE_DIR/results/option1_tx_logs.txt" ] && echo "tx_logs=$EVIDENCE_DIR/results/option1_tx_logs.txt"
