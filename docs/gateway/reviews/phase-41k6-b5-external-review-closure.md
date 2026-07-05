# Phase 41K.6 B5 — External review closure

## Purpose

This note records the external review closure for Phase 41K.6 B5 before opening B6.1.

B5 target:

watcher/relayer integration path for XXXL SVM ConsumeGatewayMint.

B5 status:

CLOSED.

## Current main checkpoint

Latest B5-related main checkpoint:

ee3682b Merge phase 41K.6 B5 TypeScript known-answer vector fix

## Review sources

External review inputs:

- Claude hostile B5 audit.
- Claude delta-only Q9.1 closure confirmation.
- Theo B5 architecture review-gate.

## Claude status

Claude initial verdict:

ACCEPT WITH NOTES.

Claude's only mandatory pre-B6.1 item:

- lock TS to Rust payload-hash parity through cross-language known-answer vectors.

Fixes merged:

- 11e48f4 Merge phase 41K.6 B5 cross-language known-answer vector
- ee3682b Merge phase 41K.6 B5 TypeScript known-answer vector fix

Claude delta-only follow-up status:

CONFIRMED CLOSED.

Claude confirmed:

- TypeScript known-answer assertions are present.
- Rust known-answer assertions are present.
- TS constants match Rust constants.
- Both digests were independently recomputed and are correct.
- Both sides assert the values against their own builders.
- Future drift in field order, domain, endianness, u64/u128 width, or field size will fail known-answer tests.
- Q9.1 is closed on both sides.
- Previous ACCEPT WITH NOTES verdict remains in force.
- B6.1 may open as a no-send/no-sign/no-key/no-submit boundary.

## Theo status

Theo verdict:

APPROVE WITH NOTES.

Theo confirmed:

- B5 is pure offline deterministic preparation.
- No deploy.
- No submit.
- No sign.
- No SOL spend.
- No private keys.
- No B4 gate removal.
- No production activation.
- TS to Rust payload-hash parity is locked via hardcoded known-answer vectors.
- Stage 2 watcher/relayer assumptions were reconciled correctly with the Phase 41K.6 B1-B4 handler boundary.
- guardianSetVersion to guardian_set_id bytes32 is clean.
- recipient owner vs recipient token account separation is clean.
- expectedMintedAmountOverride remains test-only.
- sourceFinalityState is watcher metadata only.
- operational ids remain non-payload-bound.
- B5.3 quorum package validation is correctly scoped as pre-submit only.
- The handler remains final authority.
- B5.4 relayer submission package shape is sufficient for B6.1.
- B5.5 negative matrix and closure checklist are sufficient.
- B5.6 closes TS to Rust parity drift concerns.

Theo conclusion:

B5 is architecturally closed.

B6.1 may be opened.

## Theo notes for B6.1

Theo added two non-blocking notes that must be reflected in the B6.1 spec:

1. B6.1 must explicitly preserve no-send/no-sign/no-key/no-SOL.
2. B6.1 must be defined as simulation/dry-run, not submission.

Theo also asked one explicit gate question:

Does B6.1 plan to remove the B1C7 compile_error guard or feature gate, or does it remain gated with actual relayer path being a separate deployment decision?

## Answer to Theo's gate question

B6.1 does not remove the B1C7 compile_error guard.

B6.1 does not weaken the B1C7 feature gate.

B6.1 does not open production gates.

B6.1 does not submit transactions.

B6.1 does not deploy.

B6.1 does not spend SOL.

B6.1 does not access private keys.

B6.1 is an explicit opening boundary and simulation/dry-run specification.

The actual relayer path, testnet submission, production gate opening, and any gate removal remain separate deliberate operator/project decisions.

Production or production-like gate opening must remain a separate deliberate operator/project decision, not an automatic consequence of B6.1 or any later B6 engineering step.

## B6.1 required conditions

B6.1 must include:

- explicit no-send/no-sign/no-key/no-SOL boundary,
- B1C7 compile_error and feature gate intact,
- simulation/dry-run only,
- any testnet RPC calls, if introduced later, must be read-only,
- no transactions submitted,
- B5 package shape consumed as input, not redefined,
- guardian signatures remain mock or pre-generated,
- no live key usage,
- clear B6.1 to B6.2 to production-gate progression,
- processed_event registry checks are read-only.

## Non-blocking notes for later B6 readiness

The following are useful but not B6.1 blockers:

- amountLeHex naming clarity,
- guardianSetPublicKeys to guardian_set_id linkage,
- dedupeKey,
- quorumMet meaning as structural readiness rather than cryptographic verification,
- optional B5.7 negative known-answer vectors.

## Final closure decision

B5 is externally reviewed and closed.

Claude confirmed the mandatory parity note closed.

Theo approved B5 closure with notes.

B6.1 may now be opened as:

Phase 41K.6 B6.1 — X1 testnet E2E opening boundary

B6.1 must remain no-send, no-sign, no-key, no-SOL, no-submit, and no-gate-removal.
