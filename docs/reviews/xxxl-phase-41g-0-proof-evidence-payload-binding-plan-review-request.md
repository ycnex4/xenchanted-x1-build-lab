# XXXL Phase 41G.0 Review Request — Proof / Evidence / Payload Binding Plan

Date: 2026-07-02

## Current Main

`72951e8 Merge XXXL phase 41F focused crypto boundary audit acceptance`

## Review Target

Plan:

`docs/xxxl/xxxl-phase-41g-0-proof-evidence-payload-binding-plan.md`

Checkpoint:

`docs/checkpoints/xxxl-x1-testnet-local-runtime-skeleton-phase-41g-0-proof-evidence-payload-binding-plan.md`

## Scope

Docs-only planning checkpoint.

No runtime code.

No verification logic change.

## Purpose

Plan Phase 41G payload binding after Phase 41F closed the native Ed25519 verification boundary.

Phase 41G should establish only:

- the SVM-verified Ed25519 message bytes are bound to the expected gateway payload hash.

Phase 41G must not establish:

- guardian validity;
- guardian set membership;
- quorum;
- authorization;
- replay writes;
- mutation;
- CPI;
- mint;
- handler;
- live route.

## Requested Review

Please check:

1. Is Phase 41G.0 the correct next step after accepted Phase 41F?
2. Is payload binding correctly separated from guardian/quorum/auth?
3. Is the preferred `signed_message_bytes == expected_gateway_payload_hash_bytes` model acceptable?
4. Is `keccak256(canonical_gateway_payload_bytes)` the right planned hash model?
5. Is the canonical field list complete and consistent with Stage 1?
6. Are route/source/burn/recipient/amount/mint/finality/nonce bindings complete?
7. Is public key handling correctly deferred to guardian validation?
8. Are negative cases sufficient?
9. Are all forbidden operations still forbidden?
10. Can Phase 41G.1 payload evidence shape begin after acceptance?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES / BLOCKED
- Required fixes:
- Non-blocking notes:
- Scope violations: yes/no
- Correct next phase after 41F: yes/no
- Payload binding separation acceptable: yes/no
- Signed-message-equals-payload-hash model acceptable: yes/no
- Hash model acceptable: yes/no
- Canonical field list acceptable: yes/no
- Binding requirements acceptable: yes/no
- Guardian validation deferral acceptable: yes/no
- Negative matrix sufficient: yes/no
- Forbidden operations preserved: yes/no
- Phase 41G.1 allowed after acceptance: yes/no
