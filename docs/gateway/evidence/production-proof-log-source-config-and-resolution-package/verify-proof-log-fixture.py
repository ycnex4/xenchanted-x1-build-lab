from pathlib import Path
import json
import hashlib
import sys

def canonical(obj):
    return json.dumps(obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def sha256_obj(obj):
    return hashlib.sha256(canonical(obj).encode("utf-8")).hexdigest()

schema_path = Path("docs/gateway/proof-log/schema/gateway-mint-proof-v1.schema.json")
config_path = Path("docs/gateway/proof-log/config/x1-testnet-proof-log-config-v1.json")
fixture_path = Path("docs/gateway/proof-log/fixtures/gateway-mint-proof-v1-dry-run-record.json")
verification_path = Path("docs/gateway/proof-log/verification.md")

required_files = [schema_path, config_path, fixture_path, verification_path]
for path in required_files:
    if not path.exists():
        raise SystemExit(f"missing required file: {path}")

schema = json.loads(schema_path.read_text())
config = json.loads(config_path.read_text())
record = json.loads(fixture_path.read_text())

assert schema["title"] == "xEnchanted Gateway Mint Proof Log Record v1"
assert config["config_version"] == "production-proof-log-config-v1"
assert record["schema_version"] == "gateway-mint-proof-v1"
assert record["record_type"] == "gateway_mint_proof"
assert record["record_status"] == "dry_run_fixture"
assert record["route"] == "gateway_mint"
assert record["environment"] == "x1_testnet"

source_without_hash = dict(record["source_burn"])
expected_source_hash = source_without_hash.pop("canonical_event_hash")
assert hashlib.sha256(canonical(source_without_hash).encode("utf-8")).hexdigest() == expected_source_hash

assert record["guardian_quorum"]["guardian_set_version"] == 1
assert record["guardian_quorum"]["guardian_count"] == 5
assert record["guardian_quorum"]["threshold"] == 3
assert record["guardian_quorum"]["quorum_model"] == "3-of-5"
assert record["guardian_quorum"]["guardian_set_descriptor_hash_sha256"] == config["guardian_set"]["descriptor_hash_sha256"]
assert len(record["guardian_quorum"]["guardian_approvals"]) == 3

expected_transcript_hash = sha256_obj(record["guardian_quorum"]["guardian_approvals"])
assert record["guardian_quorum"]["guardian_quorum_transcript_hash"] == expected_transcript_hash

consumed_without_hash = dict(record["consumed_event"])
expected_marker_hash = consumed_without_hash.pop("consumed_event_marker_hash")
assert hashlib.sha256(canonical(consumed_without_hash).encode("utf-8")).hexdigest() == expected_marker_hash

record_zero_hash = dict(record)
record_zero_hash["record_hash"] = "0" * 64
assert sha256_obj(record_zero_hash) == record["record_hash"]

assert not config["safety"]["activation_authorized"]
assert not config["safety"]["deploy_authorized"]
assert not config["safety"]["route_enablement_authorized"]
assert not config["safety"]["spl_cpi_enablement_authorized"]
assert not config["safety"]["live_mint_execution_authorized"]
assert not config["safety"]["external_production_endpoint_publication_authorized"]
assert not config["safety"]["private_key_material_allowed"]
assert not config["safety"]["signing_package_construction_authorized"]

import re

text = fixture_path.read_text() + config_path.read_text() + verification_path.read_text()

secret_material_patterns = [
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
    r"-----END [A-Z ]*PRIVATE KEY-----",
    r"\\\"secretKey\\\"\\s*:",
    r"\\\"privateKey\\\"\\s*:",
    r"\\bseed_phrase\\s*[:=]",
    r"\\bseedPhrase\\s*[:=]",
    r"\\bmnemonic\\s*[:=]",
    r"solana-keygen\\s+(new|recover|grind)",
]

for pattern in secret_material_patterns:
    assert re.search(pattern, text) is None, pattern

print("proof_log_fixture_verification=PASS")
print(f"record_hash={record['record_hash']}")
print(f"guardian_set_descriptor_hash={record['guardian_quorum']['guardian_set_descriptor_hash_sha256']}")
