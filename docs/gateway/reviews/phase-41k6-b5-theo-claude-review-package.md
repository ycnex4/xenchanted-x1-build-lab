# Phase 41K.6 B5 — Theo / Claude review package

## Purpose

This review package asks Theo and Claude to review Phase 41K.6 B5 before we open B6.

B5 is closed on main as:

77104e9 Merge phase 41K.6 B5 watcher relayer integration path

B5 closed the watcher/relayer integration path as an offline, deterministic, no-send preparation layer.

B5 did not remove gates.

B5 did not deploy to X1 testnet.

B5 did not submit transactions.

B5 did not sign transactions.

B5 did not access private keys.

B5 did not spend SOL.

B5 did not activate production runtime execution.

## Current checkpoint list

Closed:

- B1: guardian quorum authorization.
- B2: valid quorum live-gated success test.
- B3: hostile live-gated matrix.
- B4: activation gate decision / production-readiness boundary.
- B5: watcher/relayer integration path.

Not opened yet:

- B6: X1 testnet deploy + end-to-end Ethereum burn -> X1 mint.

## B5 closure gates

B5 closure gates passed before merge:

- Full TypeScript test suite: 106 passed; 0 failed.
- Full TypeScript test count: 954 passed; 0 failed.
- Full xxxl-svm lib test suite: 610 passed; 0 failed; 1 ignored.

Post-merge checks also passed:

- npm run typecheck
- npm test
- cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib

## B5 files added or updated

Main B5 checkpoint:

- docs/gateway/phase-41k6-b5-watcher-relayer-integration-path.md

B5.1 schema reconciliation:

- docs/gateway/phase-41k6-b5-1-schema-reconciliation-inventory.md

B5.2 candidate payload v2 hash conversion:

- src/gateway/phase41k6PayloadV2.ts
- tests/phase41k6_b5_candidate_payload_hash.test.ts
- docs/gateway/phase-41k6-b5-2-candidate-payload-hash-conversion.md

B5.3 quorum package boundary:

- src/gateway/phase41k6QuorumPackage.ts
- tests/phase41k6_b5_quorum_package.test.ts
- docs/gateway/phase-41k6-b5-3-quorum-package-boundary.md

B5.4 relayer submission package boundary:

- src/gateway/phase41k6RelayerSubmissionPackage.ts
- tests/phase41k6_b5_relayer_submission_package.test.ts
- docs/gateway/phase-41k6-b5-4-relayer-submission-package-boundary.md

B5.5 negative matrix and closure checklist:

- docs/gateway/phase-41k6-b5-5-negative-matrix-closure-checklist.md

## Important B5 context

Before B5, the repository already had older watcher/relayer evidence under docs/gateway/evidence.

Important previous evidence chain:

- Stage 2.18 watcher-event normalized task adapter.
- Stage 2.19 watcher-event full submit pipeline.
- Stage 2.20 watcher-event submit idempotency / retry.
- Stage 2.21 ambiguous recovery.
- Stage 2.22 watcher-event operational submit wrapper.
- Stage 2.23 watcher-event batch / queue processing.
- Stage 2.24 durable relayer journal model.
- Stage 2.25 watcher-to-relayer contract boundary.
- Stage 4 no-send / no-SOL readiness chain.
- Stage 5 external wallet live-send path.

B5 did not duplicate that old path blindly.

B5 reconciled the old watcher/relayer concepts with the new Phase 41K.6 B1-B4 SVM handler boundary.

## New Phase 41K.6 handler reality

The current handler boundary is now based on:

- B1C payload v2 hash binding.
- Guardian membership.
- Unique guardian quorum.
- processed_event PDA identity.
- route_id binding.
- target SPL mint binding.
- recipient token account binding.
- amount binding.
- guardian_set_id bytes32 binding.
- strictly prior Ed25519 evidence instruction model.
- B3 hostile rejection matrix.
- B4 decision that the handler path remains gated.

## Payload hash alignment

The Rust SVM handler computes the B1C expected authorization payload hash from:

- domain: consume_gateway_mint_authorization_v2
- processed_event
- route_id
- mint
- recipient
- amount as u64 little-endian
- guardian_set_id

The Rust module labels the hash algorithm as sha256 and uses Solana hashv-style concatenation.

B5.2 mirrors this in TypeScript.

Review point:

Please verify that the TypeScript builder is semantically aligned with the Rust handler payload hash model.

## B5.1 decisions to review

B5.1 classified old Stage 2 watcher/relayer fields.

Key decisions:

- guardianSetVersion should become guardian_set_id bytes32.
- recipientBase58 must be clarified into recipient owner versus recipient token account.
- expectedMintedAmountOverride remains test-only unless later policy explicitly reintroduces it.
- sourceFinalityState remains watcher/finality metadata, not standalone handler authorization.
- relayer operational ids remain non-payload-bound metadata.
- payload-bound fields must not drift after guardian signing.

Review point:

Please verify that these decisions are correct and that no old Stage 2 field is being carried forward in a stale or unsafe way.

## B5.2 behavior to review

B5.2 added pure TypeScript candidate-to-payload-hash conversion.

It verifies:

- deterministic payload hash construction,
- amount encoded as u64 little-endian,
- changing processed_event changes payload hash,
- changing route_id changes payload hash,
- changing mint changes payload hash,
- changing recipient token account changes payload hash,
- changing amount changes payload hash,
- changing guardian_set_id changes payload hash,
- watcher-only operational metadata does not change payload hash,
- malformed bytes32 fields reject,
- invalid u64 amount rejects.

Review points:

- Is it correct that watcher-only operational metadata is not payload-bound at this layer?
- Should source observation fields be bound in a future handler revision, or is candidate-level validation enough for B6?
- Is the amount little-endian display/reporting shape clear enough, or should amountLeHex be represented as exactly 8 bytes instead of a padded bytes32-style report field?

## B5.3 behavior to review

B5.3 added pure TypeScript quorum package validation.

It verifies:

- valid unique guardian quorum is accepted,
- duplicate guardian evidence rejects,
- unknown guardian evidence rejects,
- insufficient quorum rejects,
- signed message drift rejects,
- invalid threshold rejects,
- invalid source instruction index rejects,
- malformed signature hex rejects.

Review points:

- Is this the right off-chain pre-submit validation boundary?
- Is it correct that this does not replace handler-side B3 checks?
- Are there missing negative cases before B6?

## B5.4 behavior to review

B5.4 added no-send relayer submission package assembly.

It assembles:

candidate
-> payload_v2_hash
-> quorum package
-> relayer submission package

It records:

- eventId,
- journalId,
- payload v2 hash result,
- quorum package,
- processed_event,
- route_id,
- mint,
- recipient token account,
- amount,
- guardian_set_id,
- prior evidence instruction count,
- no-send/no-sign/no-RPC/no-SOL/no-private-key boundary.

It rejects:

- empty eventId,
- empty journalId,
- guardian_set_id drift between candidate and quorum package,
- stale signatures after handler-bound candidate mutation.

It allows:

- relayer-only operational id changes without changing payload hash.

Review points:

- Is this package shape sufficient before B6 dry-run/testnet rehearsal?
- Are eventId and journalId too loosely defined for B6?
- Should dedupeKey be added now, or deferred to B6 readiness inventory?

## B5.5 negative matrix to review

B5.5 records the B5 negative matrix and closure checklist.

Failure classes covered include:

- malformed handler-bound bytes32 fields,
- amount outside u64,
- payload-bound field drift,
- stale signatures,
- duplicate guardian evidence,
- unknown guardian evidence,
- insufficient quorum,
- signed message mismatch,
- invalid threshold,
- invalid source instruction index,
- malformed signature,
- empty eventId,
- empty journalId.

Review point:

Please check whether the matrix is enough to justify opening B6, or whether another B5.x hardening step is needed first.

## What B5 explicitly does not prove

B5 does not prove:

- live X1 testnet execution,
- actual deployed program compatibility,
- actual X1 runtime Ed25519 precompile behavior,
- actual instructions sysvar behavior on X1 testnet,
- real Ethereum burn observation,
- real guardian signing,
- real relayer transaction submission,
- real external wallet flow,
- real processed_event account lifecycle on testnet,
- real SPL Token MintTo execution on testnet,
- production activation readiness.

Those belong to B6 or later.

## B6 should not start as automatic submit

If B6 opens, it should start with:

B6.1 — X1 testnet E2E opening boundary

B6.1 should only document:

- allowed work,
- forbidden work,
- no-secret rules,
- signer boundary,
- operator confirmation boundary,
- testnet-only assumptions,
- stop conditions.

B6.1 should not deploy, submit, sign, spend SOL, or access private keys.

## Questions for Theo

1. Is the B5 reconciliation between old Stage 2 watcher/relayer evidence and new Phase 41K.6 handler boundary conceptually correct?
2. Are the payload-bound fields complete for B6 rehearsal?
3. Is it acceptable that source observation fields remain outside the B1C payload hash for now?
4. Are guardian_set_id, processed_event, route_id, mint, recipient token account, and amount sufficient as the signed handler-bound payload?
5. Is the B5.3 quorum package validation boundary correctly scoped as pre-submit only?
6. Does B5.4 give enough structure to open B6.1 safely?
7. Should dedupeKey/canonicalEventKey be more explicitly represented in the relayer submission package before B6?
8. Are there any stale Stage 2 assumptions that must be removed before B6?
9. Does B5 preserve the B4 activation gate decision strongly enough?
10. Do we need another B5 hardening pass before B6?

## Questions for Claude

1. Review src/gateway/phase41k6PayloadV2.ts for TypeScript correctness, strict typing, encoding bugs, and Rust parity risks.
2. Review src/gateway/phase41k6QuorumPackage.ts for validation gaps, edge cases, duplicate checks, and threshold logic.
3. Review src/gateway/phase41k6RelayerSubmissionPackage.ts for package assembly risks and stale-signature detection.
4. Review tests/phase41k6_b5_candidate_payload_hash.test.ts for missing payload drift cases.
5. Review tests/phase41k6_b5_quorum_package.test.ts for missing quorum hostile cases.
6. Review tests/phase41k6_b5_relayer_submission_package.test.ts for missing package-level hostile cases.
7. Check whether amountLeHex should be an 8-byte report field rather than a 32-byte padded display field.
8. Check whether bytes32 hex is the right TS representation for Solana Pubkey-style 32-byte values at this boundary.
9. Check whether sourceObservation fields need stronger validation before B6.
10. Check whether npm run typecheck, npm test, and cargo test coverage are enough for B5 closure.

## Suggested review commands

From repo root:

npm run typecheck

npm test

cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib

Focused B5 tests:

npm test -- tests/phase41k6_b5_candidate_payload_hash.test.ts tests/phase41k6_b5_quorum_package.test.ts tests/phase41k6_b5_relayer_submission_package.test.ts

Files to inspect:

src/gateway/phase41k6PayloadV2.ts

src/gateway/phase41k6QuorumPackage.ts

src/gateway/phase41k6RelayerSubmissionPackage.ts

tests/phase41k6_b5_candidate_payload_hash.test.ts

tests/phase41k6_b5_quorum_package.test.ts

tests/phase41k6_b5_relayer_submission_package.test.ts

docs/gateway/phase-41k6-b5-watcher-relayer-integration-path.md

docs/gateway/phase-41k6-b5-1-schema-reconciliation-inventory.md

docs/gateway/phase-41k6-b5-5-negative-matrix-closure-checklist.md

## Requested review output

Please answer with:

1. Approved / approved with changes / not approved.
2. Critical blockers before B6, if any.
3. Recommended B5.x hardening steps, if any.
4. Recommended B6.1 opening conditions.
5. Any stale assumptions from older Stage 2/4/5 evidence.
6. Any mismatch between TypeScript off-chain preparation and Rust SVM handler behavior.
