#!/usr/bin/env bash
set -euo pipefail

PHASE="phase-41k6-b6-63-command-boundary-no-execution"
OUTDIR="tmp/local-validator-fixtures/phase-41k6-b6-local-only"

EXECUTE_REQUESTED="false"

for arg in "$@"; do
  case "$arg" in
    --execute)
      EXECUTE_REQUESTED="true"
      ;;
    --help|-h)
      cat <<'HELP'
B6.63 command-boundary no-execution script.

Default behavior:
  no execution
  local fixture verification only
  fail closed

Allowed:
  inspect tmp/local-validator-fixtures/phase-41k6-b6-local-only

Forbidden:
  local validator execution
  testnet
  live RPC
  signing
  real keys
  guardian packages
  SPL setup
  upgrade/init/submit
HELP
      exit 0
      ;;
    *)
      echo "ERROR: unsupported argument: $arg"
      exit 2
      ;;
  esac
done

if [ "${EXECUTE:-false}" = "true" ]; then
  EXECUTE_REQUESTED="true"
fi

echo "PHASE: $PHASE"
echo "MODE: COMMAND_BOUNDARY_NO_EXECUTION"
echo "OUTPUT_DIRECTORY: $OUTDIR"

if [ "$OUTDIR" != "tmp/local-validator-fixtures/phase-41k6-b6-local-only" ]; then
  echo "ERROR: output directory boundary mismatch"
  exit 10
fi

if [ ! -d "$OUTDIR" ]; then
  echo "ERROR: fixture directory missing"
  echo "NO_TESTNET_FALLBACK: true"
  exit 11
fi

python3 - "$OUTDIR" <<'PY'
import json
import pathlib
import re
import sys

out = pathlib.Path(sys.argv[1])

expected = [
    "README.local-only.txt",
    "accounts.json",
    "expected-snapshots.json",
    "failure-matrix.json",
    "instructions.json",
    "logs.json",
    "manifest.json",
    "mutation-invariance.json",
    "safety-report.json",
    "scenarios.json",
]

actual = sorted(p.name for p in out.iterdir() if p.is_file())
if actual != expected:
    raise SystemExit(f"ERROR: fixture file set mismatch: expected={expected} actual={actual}")

for path in sorted(out.glob("*.json")):
    json.loads(path.read_text(encoding="utf-8"))

forbidden_taxonomy = {
    "private_keys": r"BEGIN [A-Z ]*PRIVATE|private[_ -]?key\s*[:=]",
    "seed_phrases": r"seed[_ -]?phrase\s*[:=]|mnemonic\s*[:=]",
    "authenticated_or_real_rpc_endpoints": r"https?://|wss?://|rpc[_ -]?url\s*[:=]",
    "real_program_ids_or_upgrade_authority_markers": r"program[_ -]?id\s*[:=]|upgrade[_ -]?authority\s*[:=]",
    "credentials_or_tokens": r"credential\s*[:=]|token\s*[:=]|secret\s*[:=]|api[_ -]?key\s*[:=]",
    "keypair_paths": r"keypair[_ -]?path\s*[:=]",
}

for path in sorted(out.iterdir()):
    text = path.read_text(encoding="utf-8")
    for label, pattern in forbidden_taxonomy.items():
        if re.search(pattern, text, re.IGNORECASE):
            raise SystemExit(f"ERROR: forbidden material detected: {label} in {path.name}")

print("FIXTURE_FILE_COUNT: 10")
print("JSON_CHECK: OK")
print("FORBIDDEN_MATERIAL_TAXONOMY_SCAN: OK")
print("FIXTURE_BOUNDARY: LOCAL_TMP_ONLY")
PY

echo "NO_TESTNET_FALLBACK: true"
echo "MOCK_DATA_ONLY: true"
echo "LOCAL_VALIDATOR_EXECUTION: NOT_EXECUTED"
echo "TESTNET_ACTION: NOT_EXECUTED"
echo "SIGNING: NOT_EXECUTED"
echo "SPL_SETUP: NOT_EXECUTED"
echo "UPGRADE_INIT_SUBMIT: NOT_EXECUTED"

if [ "$EXECUTE_REQUESTED" != "true" ]; then
  echo "BLOCKER_H_NOT_CLOSED: local-validator dry-run requires explicit GO"
  echo "RESULT: NO_EXECUTION_DEFAULT_EXIT"
  exit 0
fi

echo "EXECUTE_REQUESTED: true"
echo "BLOCKER_H_NOT_CLOSED: local-validator dry-run requires explicit GO"
echo "RESULT: EXECUTION_REFUSED_BY_B6_63_NO_EXECUTION_BOUNDARY"
# Exit 63 = BLOCKER_H_NOT_CLOSED
exit 63
