# Production Proof Log Instantiation Plan

Package: production-proof-log-instantiation-plan
Approval: APPROVE_PRODUCTION_PROOF_LOG_INSTANTIATION_PLAN_NO_ACTIVATION
Started at UTC: 2026-07-11T05:04:52Z
Base commit: e281c88
Lane: pre-go-operational-readiness
Package type: documentation/evidence planning only
Activation authorized: false
Deploy authorized: false
RPC mutation authorized: false
Route enablement authorized: false
SPL CPI enablement authorized: false
Proof-log instantiation authorized: false
Blocker removal authorized: false
Source code mutation authorized: false

## 1. Purpose

The production proof log is the public audit trail for the future gateway mint route.

Its purpose is to prove, for every future production gateway mint, that the mint is bound to:

1. a canonical source burn event,
2. the approved guardian set and threshold model,
3. a guardian quorum attestation over the canonical mint message,
4. the destination X1 mint transaction,
5. the consumed-event / replay-protection marker,
6. an append-only public record that can be independently verified.

The proof log is not an activation mechanism. It does not enable the live route, does not execute SPL CPI, does not deploy, and does not authorize minting.

## 2. Non-goals for this package

This package does not instantiate the proof log.

This package does not:
- create runtime proof-log accounts,
- create on-chain state,
- mutate source code,
- remove ProductionProofLogUnset,
- enable the live route,
- enable SPL CPI execution,
- deploy or upgrade any program,
- construct signing packages,
- collect signatures,
- publish private key or keypair material,
- authorize activation.

## 3. Current preserved blocker state

| Item | State after this plan |
| --- | --- |
| ExternalReviewIncomplete | REMOVED |
| PlaceholderProgramId | REMOVED |
| ProductionGuardianSetUnset | REMOVED |
| LiveRouteDisabled | ACTIVE |
| SplCpiExecutionDisabled | ACTIVE |
| ProductionProofLogUnset | ACTIVE |
| runtime_deployable | false |
| predeploy_gate | blocked |
| execution | blocked |

## 4. Canonical proof-log record fields

A future production proof-log record must be deterministic and schema-versioned.

Required top-level fields:

| Field | Meaning |
| --- | --- |
| schema_version | Proof-log record schema version |
| record_type | Expected value: gateway_mint_proof |
| record_status | pending, accepted, rejected, superseded, or correction |
| environment | x1_testnet or future approved environment |
| route | gateway_mint |
| created_at_utc | Publication timestamp |
| sequence_number | Monotonic append-only sequence |
| previous_record_hash | Hash of previous accepted proof-log record, or null for genesis |
| record_hash | Hash of this canonical proof-log record |
| batch_id | Optional batch identifier |
| batch_merkle_root | Optional batch root when records are published in batches |

Required source burn fields:

| Field | Meaning |
| --- | --- |
| source_chain_id | Source chain identifier |
| source_chain_name | Source chain name |
| source_block_number | Source block number containing the burn |
| source_block_hash | Source block hash |
| source_tx_hash | Source transaction hash |
| source_log_index | Source log index or deterministic event index |
| source_event_signature | Event signature or ABI event identifier |
| source_burn_contract | Contract that emitted the burn event |
| source_token | Burned token address / identifier |
| source_burner | Address that performed the burn or authorized it |
| burn_amount | Burn amount in canonical units |
| canonical_event_key | Deterministic replay-protection key |
| canonical_event_hash | Hash of normalized source event fields |

Required destination mint fields:

| Field | Meaning |
| --- | --- |
| x1_program_id | Bound X1 program id |
| gateway_mint_authority_pda | Bound gateway mint authority PDA |
| gateway_mint_authority_bump | PDA bump |
| destination_mint | Destination SPL mint |
| destination_token_program | SPL Token or Token-2022 program id |
| destination_recipient | Recipient token account or owner binding |
| mint_amount | Amount minted in canonical units |
| mint_message_hash | Hash of canonical gateway mint message |
| mint_instruction_hash | Hash of destination mint instruction payload |
| x1_mint_tx_signature | X1 transaction signature |
| x1_slot | X1 slot |
| x1_instruction_index | Instruction index in X1 transaction |

Required guardian fields:

| Field | Meaning |
| --- | --- |
| guardian_set_version | Guardian set version |
| guardian_set_descriptor_hash_sha256 | Hash of public guardian set descriptor |
| guardian_count | Number of guardians in the set |
| threshold | Required quorum threshold |
| quorum_model | Example: 3-of-5 |
| signature_domain | Domain used for guardian attestations |
| guardian_approval_count | Number of accepted guardian approvals |
| guardian_quorum_transcript_hash | Hash of ordered guardian approval transcript |
| guardian_approvals | Ordered list of guardian approval metadata |

Each guardian approval metadata record must include:

| Field | Meaning |
| --- | --- |
| guardian_index | Index from the approved descriptor order |
| guardian_public_key | Public Ed25519 / Solana key |
| attestation_payload_hash | Hash of signed canonical payload |
| signature_hash | Hash of signature bytes, not private material |
| verification_status | accepted or rejected |
| rejection_reason | Optional reason when rejected |

Required consumed-event / replay-protection fields:

| Field | Meaning |
| --- | --- |
| consumed_event_key | Replay-protection key consumed by the gateway |
| consumed_event_marker_hash | Hash of consumed marker state or deterministic marker evidence |
| consumed_status | consumed, rejected, or pending |
| consumed_at_x1_slot | Slot where consumed marker became effective, when available |

Required publication fields:

| Field | Meaning |
| --- | --- |
| publication_location | Public URL, repo path, or content-addressed reference |
| publication_commit | Git commit, release id, or equivalent immutable publication reference |
| publisher_identity | Operator identity or public publisher id |
| reviewer_identity | Optional reviewer identity |
| correction_of_record_hash | Optional record hash corrected by this record |
| notes | Optional bounded notes |

## 5. Required hashes

A future proof-log record must include or derive the following hashes:

| Hash | Purpose |
| --- | --- |
| canonical_event_hash | Binds normalized burn event fields |
| canonical_event_key_hash | Binds replay-protection identity |
| mint_message_hash | Binds burn event to destination mint request |
| guardian_set_descriptor_hash_sha256 | Binds guardian set version and keys |
| guardian_attestation_payload_hash | Binds what each guardian signs |
| guardian_quorum_transcript_hash | Binds ordered quorum evidence |
| mint_instruction_hash | Binds X1 instruction payload |
| consumed_event_marker_hash | Binds replay-protection marker evidence |
| record_hash | Binds the full canonical proof-log record |
| previous_record_hash | Links to previous accepted record |
| batch_merkle_root | Optional batch-level tamper evidence |

Known guardian set v1 descriptor hash from the previous package:

guardian_set_descriptor_hash_sha256=4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83

## 6. Link model

The proof-log link model is:

1. Source burn event is normalized.
2. Normalized burn fields produce canonical_event_key and canonical_event_hash.
3. Canonical mint message binds canonical_event_key, burn data, destination mint, recipient, amount, route, domain, and guardian set version.
4. Guardians sign the canonical mint message under the approved signature domain.
5. Guardian approvals are ordered by descriptor order and threshold-validated.
6. X1 mint transaction executes only in a future separately approved activation path.
7. Consumed-event marker proves the event cannot be replayed.
8. Proof-log record binds source burn, guardian quorum, mint transaction, consumed marker, and previous proof record.
9. Public verifier can recompute all hashes and reject inconsistent records.

## 7. Publication location/model

The recommended future publication model is public, append-only, and deterministic.

Preferred model for the next separately approved package:

| Component | Recommendation |
| --- | --- |
| Public manifest | docs/gateway/proof-log/production/x1-testnet/manifest.json |
| Append-only records | docs/gateway/proof-log/production/x1-testnet/records/*.jsonl |
| Batch summaries | docs/gateway/proof-log/production/x1-testnet/batches/*.json |
| Schema | docs/gateway/proof-log/schema/gateway-mint-proof-v1.schema.json |
| Verification notes | docs/gateway/proof-log/verification.md |
| Evidence package | docs/gateway/evidence/production-proof-log-source-config-package/ |

A future package may also mirror records to IPFS, Arweave, or another content-addressed system. That mirror is optional unless separately approved.

This plan does not create those production proof-log paths.

## 8. Retention policy

Production proof-log records must be retained indefinitely.

Rules:

- accepted records are never deleted,
- corrections are appended as new records,
- superseded records remain visible,
- rejected records may be retained for audit if they do not expose sensitive material,
- private keys, keypairs, seed phrases, mnemonics, or signing secrets are never retained or published,
- proof-log batches must remain verifiable from public data,
- any publication migration must preserve old hashes or publish a migration proof.

## 9. Operator and publication responsibility

The future operator or relayer is responsible for publishing proof-log records after the corresponding future gateway mint event.

The operator does not gain mint authority from the proof log.

Responsibilities:

| Actor | Responsibility |
| --- | --- |
| Operator / relayer | Publish records and batch manifests |
| Guardians | Provide attestations only; they are not fee payers by default |
| Reviewer | Verify schema, hashes, quorum, and state linkage |
| Public verifier | Recompute and audit records from public data |

The proof log must never require guardian private keys to be present in the repo.

## 10. Append-only and tamper-evidence expectations

A future instantiated proof log must be append-only by convention and tamper-evident by hash.

Minimum expectations:

- monotonically increasing sequence_number,
- each accepted record contains previous_record_hash,
- record_hash is computed from canonical normalized JSON,
- batch_merkle_root is published when batching is used,
- corrections reference correction_of_record_hash,
- no in-place mutation of accepted record content,
- publication commit or content-addressed hash is recorded,
- schema changes require a new schema_version,
- verifier rejects ambiguous field ordering or non-canonical serialization.

## 11. Verification checklist

A verifier must be able to check:

1. schema_version is known,
2. record_hash recomputes correctly,
3. previous_record_hash links to the previous accepted record,
4. source burn fields recompute canonical_event_key and canonical_event_hash,
5. mint_message_hash recomputes from canonical payload,
6. guardian_set_version and descriptor hash match approved public guardian set,
7. signature_domain matches approved domain,
8. guardian approvals are unique and known,
9. threshold is satisfied,
10. guardian_quorum_transcript_hash recomputes,
11. mint_instruction_hash matches destination mint fields,
12. x1_mint_tx_signature exists and matches the reported instruction,
13. consumed_event_marker_hash proves replay protection,
14. no private key or keypair material appears in the record,
15. publication commit/path matches manifest,
16. record does not conflict with prior records for the same canonical_event_key.

## 12. Criteria for future source/config package

The next separately approved package must define, at minimum:

- exact proof-log schema file,
- canonical JSON normalization rule,
- record hash rule,
- previous-record hash rule,
- batch hash / Merkle rule if batching is used,
- public publication path,
- sample non-production fixture record,
- verification checklist implementation or documented command,
- material safety guard,
- no source code mutation unless separately approved,
- no blocker removal unless separately approved,
- no proof-log instantiation unless separately approved.

Potential package name:

production-proof-log-source-config-package

## 13. Criteria for ProductionProofLogUnset resolution

ProductionProofLogUnset may be removed only after a separate approval confirms all of the following:

1. public proof-log location is defined,
2. schema is defined and versioned,
3. canonical hash rules are defined,
4. proof-log publication model is instantiated,
5. verifier checklist passes on fixture or approved dry-run records,
6. publication retention policy is documented,
7. append-only tamper-evidence model is documented,
8. guardian set descriptor hash linkage is present,
9. consumed-event / replay-protection linkage is present,
10. material safety guards prove no private key/keypair material,
11. source/config/tests/evidence are complete for the approved scope,
12. LiveRouteDisabled remains ACTIVE unless separately approved,
13. SplCpiExecutionDisabled remains ACTIVE unless separately approved,
14. runtime_deployable remains false unless separately approved,
15. no activation is implied.

## 14. Exit state of this plan package

After this plan package:

| Item | Expected state |
| --- | --- |
| ProductionProofLogUnset | ACTIVE |
| LiveRouteDisabled | ACTIVE |
| SplCpiExecutionDisabled | ACTIVE |
| runtime_deployable | false |
| predeploy_gate | blocked |
| execution | blocked |
| next required action | production-proof-log-source-config-package or equivalent separately approved package |
