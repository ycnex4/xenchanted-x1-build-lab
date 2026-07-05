# Phase 41K.6 B5 — Claude hostile audit request

Claude, I need a real hostile security / correctness review, not a high-level summary review.

Target:
Phase 41K.6 B5 — watcher/relayer integration path for XXXL SVM ConsumeGatewayMint.

Repository:
ycnex4/xenchanted-x1-build-lab

Audit base:
b11359f27472340325452287fd54c94a3f3bd6c8

Current merged B5 checkpoint:
77104e9 Merge phase 41K.6 B5 watcher relayer integration path

Review branch containing this request:
stage-41k6-b5-theo-claude-review-package

Important:
Please audit the code from the raw links below. Do not rely only on my summary.

B5 is not a live submit stage.
B5 is not a deployment stage.
B5 is not a production activation stage.

B5 is an offline, deterministic, no-send watcher/relayer preparation layer that should be safe enough to open B6.1, which itself is only an explicit X1 testnet E2E opening boundary.

Expected verdict format:
ACCEPT / ACCEPT WITH NOTES / BLOCK

If BLOCK:
Give exact exploit or correctness failure path, exact file/line references, and the minimum required fix.
Do not block on production hardening recommendations unless they are immediate blockers for opening B6.1.

Main audit question:
Is B5 correct and safe enough to open B6.1 as a no-send/no-sign/no-key/no-submit testnet E2E opening boundary?

Scope:
Review only the B5 off-chain package preparation layer and its parity with the Rust SVM handler payload binding.

In scope:
- TypeScript payload v2 hash builder.
- TypeScript quorum package validation.
- TypeScript relayer submission package assembly.
- B5 tests.
- Rust payload hash model used by the handler.
- B5 docs only if needed to understand intended boundaries.

Out of scope unless it creates an immediate B5 blocker:
- frontend,
- UI,
- production deployment,
- mainnet,
- old Stage 2 runtime implementation details,
- watcher live RPC implementation,
- real guardian operations,
- private key management,
- X1 testnet deploy mechanics,
- already-reviewed B1/B2/B3 handler authorization bypass questions.

Do not spend limits re-auditing the full SVM handler authorization path unless you find a direct B5-to-handler mismatch that would make B5 unsafe.

Primary audit questions:

Q1 — Rust parity:
Does src/gateway/phase41k6PayloadV2.ts exactly match the Rust SVM handler payload hash semantics in programs/xxxl-svm/src/verifier/b1c_payload_hash_binding.rs?

Check:
- domain string,
- field order,
- processed_event,
- route_id,
- mint,
- recipient token account,
- amount as u64 little-endian,
- guardian_set_id,
- sha256/hashv-style concatenation.

Q2 — Amount encoding:
Is amount encoded correctly as u64 little-endian?
Is amountLeHex reporting misleading because it pads 8 bytes into a bytes32-looking display value?
If this is only a reporting issue, classify as ACCEPT WITH NOTES, not BLOCK, unless it can cause package misuse.

Q3 — Solana Pubkey representation:
Is representing SVM Pubkey-like 32-byte values as 0x-prefixed bytes32 hex acceptable at this B5 boundary?
If not, what exact representation should be used before B6.1?

Q4 — Payload-bound fields:
Are these sufficient for the handler-bound signed payload before B6.1?
- processed_event,
- route_id,
- mint,
- recipient token account,
- amount,
- guardian_set_id.

Should canonicalEventKey, sourceChainId, sourceTxHash, sourceLogIndex, sourceBlockHash, or finality metadata be payload-bound now?
Do not block unless omission makes B6.1 unsafe.

Q5 — Source observation validation:
B5 currently keeps sourceObservation mostly as candidate metadata.
Is this acceptable before B6.1, or must B5 add stricter sourceObservation validation first?

Q6 — Quorum package:
Does src/gateway/phase41k6QuorumPackage.ts correctly reject:
- unknown guardians,
- duplicate guardian evidence,
- insufficient quorum,
- invalid threshold,
- signed message mismatch,
- malformed signature,
- invalid source instruction index?

Are there missing hostile cases that should be added before B6.1?

Q7 — Guardian set identity:
Does B5 correctly prevent guardian_set_id drift between candidate payload and quorum package?
Are guardianSetPublicKeys and guardian_set_id linked strongly enough for B6.1, or should B5 add an explicit guardian-set descriptor hash or digest?

Q8 — Stale signature detection:
Does src/gateway/phase41k6RelayerSubmissionPackage.ts correctly reject stale signatures when handler-bound candidate fields change after signing?

Q9 — Operational metadata:
Is it correct that eventId and journalId do not change payload hash?
Should dedupeKey be added before B6.1, or can it be deferred to B6 readiness inventory?

Q10 — No-send boundary:
Does B5 accidentally introduce any live RPC, signing, submit, key access, transaction serialization, or SOL-spend path?
If yes, BLOCK with exact file/line references.

Q11 — Test coverage:
Are the B5 tests enough to justify opening B6.1?
If not, give the minimum additional tests required.

Q12 — B6.1 readiness:
Can we open B6.1 as an explicit testnet E2E opening boundary after B5, with no submit/sign/deploy yet?

Raw links — package / TypeScript config:

https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/package.json
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/tsconfig.json

Raw links — B5 implementation:

https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/src/gateway/phase41k6PayloadV2.ts
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/src/gateway/phase41k6QuorumPackage.ts
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/src/gateway/phase41k6RelayerSubmissionPackage.ts

Raw links — B5 tests:

https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/tests/phase41k6_b5_candidate_payload_hash.test.ts
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/tests/phase41k6_b5_quorum_package.test.ts
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/tests/phase41k6_b5_relayer_submission_package.test.ts

Raw links — Rust parity target:

https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/programs/xxxl-svm/src/verifier/b1c_payload_hash_binding.rs

Optional raw links — only if needed for context, do not start here:

https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-watcher-relayer-integration-path.md
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-1-schema-reconciliation-inventory.md
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-2-candidate-payload-hash-conversion.md
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-3-quorum-package-boundary.md
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-4-relayer-submission-package-boundary.md
https://raw.githubusercontent.com/ycnex4/xenchanted-x1-build-lab/b11359f27472340325452287fd54c94a3f3bd6c8/docs/gateway/phase-41k6-b5-5-negative-matrix-closure-checklist.md

Validation already run on B5 before merge:

npm run typecheck: passed
npm test: 106 test files passed, 954 tests passed
cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib: 610 passed, 0 failed, 1 ignored

Focused B5 validation:

npm test -- tests/phase41k6_b5_candidate_payload_hash.test.ts tests/phase41k6_b5_quorum_package.test.ts tests/phase41k6_b5_relayer_submission_package.test.ts

Focused B5 result before merge:
3 B5 test files passed.
15 B5 tests passed.

What B5 claims:

B5.2:
candidate -> payload_v2_hash

B5.3:
payload_v2_hash + guardian evidence -> quorum package

B5.4:
candidate + quorum package -> no-send relayer submission package

B5 explicitly preserves:
- no live RPC,
- no signing,
- no submit,
- no SOL spend,
- no private-key access,
- no production activation,
- no gate removal.

What B5 does not claim:

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

Please optimize review effort:
Start with the 3 TS implementation files, 3 TS test files, and Rust payload hash file.
Only open docs if code intent is unclear.
Do not audit the entire repository.

Final question:
Does B5 fully close the watcher/relayer preparation layer enough to open B6.1, or is there any immediate correctness/security blocker in payload hash parity, quorum package validation, stale-signature detection, or no-send boundary?
