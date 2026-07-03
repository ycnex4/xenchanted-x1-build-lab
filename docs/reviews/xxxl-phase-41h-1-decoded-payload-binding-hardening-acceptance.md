# XXXL Phase 41H.1 — Decoded Payload Binding Hardening Acceptance

Date: 2026-07-03

## Accepted Main

`302fb0d Merge XXXL phase 41H decoded payload binding hardening`

## Implementation Commit

`8414733 Harden phase 41H decoded payload binding`

## Parent Gate

`500ae42 Merge XXXL phase 41H guardian membership boundary acceptance`

## Changed File

`programs/xxxl-svm/src/verifier/guardian_membership_validation_boundary.rs`

## Final Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Phase 41H.1 decoded payload binding hardening is accepted.

## Validation Evidence

Targeted:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml guardian_membership_validation_boundary`

Result:

- 23 passed
- 0 failed
- 0 ignored

Full:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml`

Result:

- 525 passed + 1 ignored
- 7 passed
- 3 passed
- 55 passed + 10 ignored

Diff check:

`git diff --check`

Result:

- OK

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Blocking risks: none
- decoded_payload ↔ 41G binding gap closed: yes
- free decoded payload removed: yes
- internal raw payload decode correct: yes
- internal payload hash binding correct: yes
- guardian_set_id linkage sound: yes
- error handling sufficient: yes
- tests sufficient: yes
- false flags preserved: yes
- forbidden runtime surfaces absent: yes
- 41H.1 accepted: yes

Audit Demon:

- Verdict: ACCEPT
- Required fixes: none
- decoded_payload ↔ 41G binding closed: yes
- raw payload is single source of decoded fields: yes
- signed hash binding fail-closed: yes
- guardian_set_id linkage sound: yes
- substitution gap closed: yes
- error model fail-closed: yes
- tests sufficient: yes
- false flags preserved: yes
- forbidden runtime surfaces absent: yes
- 41H.1 accepted: yes

## Accepted Closure

Previous gap:

41H accepted `PayloadHashBindingEstablished` and `DecodedGuardianPayloadRaw` as separate inputs.

That allowed the decoded payload to be caller-supplied rather than proven to come from the same raw payload bytes hash-bound by 41G.

Accepted closure:

- 41H no longer accepts free `DecodedGuardianPayloadRaw`;
- 41H now accepts `raw_payload_bytes` and `signed_message_bytes`;
- 41H internally calls `establish_payload_hash_binding(raw_payload_bytes, signed_message_bytes, phase_41f_result)`;
- 41H internally calls `decode_guardian_payload_raw(raw_payload_bytes)`;
- guardian set ID linkage uses only the internally decoded payload;
- `accepts_free_decoded_payload_input` is false;
- substitution test proves alternate raw payload with old signed hash fails at `PayloadHashBindingNotEstablished`.

## Defensive-Only Note

`RawPayloadDecodeFailed` is accepted as defensive-only.

Because `establish_payload_hash_binding` already decodes the same `raw_payload_bytes`, a second decode failure after successful binding should be unreachable.

The error kind remains useful for fail-closed defensive completeness.

## Forward Note

Future guardian-set account-loading must preserve authoritative wrapper unforgeability.

The production path must not introduce a wide public constructor that lets caller data become authoritative.

The future account-loading boundary must construct the authoritative wrapper only after real on-chain verification.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41I — Quorum counting / threshold authorization planning.

41I must remain a separate reviewed gate.

No replay write, mutation, CPI, mint, handler, or live route may be introduced by 41I planning.
