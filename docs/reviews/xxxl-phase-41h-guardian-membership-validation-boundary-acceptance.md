# XXXL Phase 41H — Guardian Membership Validation Boundary Acceptance

Date: 2026-07-03

## Accepted Main

`801f26a Merge XXXL phase 41H guardian membership validation boundary`

## Implementation Commit

`bda8be8 Add phase 41H guardian membership validation boundary`

## Parent Gate

`73b7f5d Merge XXXL phase 41H guardian membership implementation plan acceptance`

## Changed Files

- `programs/xxxl-svm/src/verifier/guardian_membership_validation_boundary.rs`
- `programs/xxxl-svm/src/verifier/mod.rs`

## Final Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Blocking risks: none for the current non-authorizing boundary model.

41H guardian membership validation boundary is accepted as a narrow non-authorizing model.

## Validation Evidence

Targeted test:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml guardian_membership_validation_boundary`

Result:

- 22 passed
- 0 failed
- 0 ignored

Full test:

`cargo test --manifest-path programs/xxxl-svm/Cargo.toml`

Result:

- full test OK
- 524 passed + 1 ignored
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
- Implementation matches accepted 41H boundary: yes
- Signer provenance bound to 41F.1/41F.2: yes
- Range/index/length binding checks: yes
- Phase 41G payload hash binding: yes
- AuthoritativeGuardianSetRef constrained: yes
- Authoritative constructor non-public: yes
- Caller-supplied/unauthenticated guardian sets rejected: yes
- Payload guardian_set_id linkage present: yes
- Configured guardian_set_id check present: yes
- Structural checks complete: yes
- Phase 35 reuse constrained: yes
- Error model fail-closed: yes
- Tests sufficient: yes
- False flags preserved: yes
- Forbidden operations absent: yes
- 41H code boundary accepted: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Scope drift: no
- Wrapper constructor constraint sufficient: yes
- Caller-supplied authoritative wrapper forgery prevented: yes
- Payload guardian_set_id linkage present but not yet soundly bound to 41G bytes: note required
- Signer provenance binding sufficient: yes
- Check ordering acceptable: yes
- Error model fail-closed: yes
- Tests sufficient: yes
- False flags preserved: yes
- Forbidden runtime surfaces absent: yes
- 41H code boundary accepted: yes as non-authorizing membership model

## Accepted Boundary

The accepted 41H code boundary is:

`verified_signer_public_key ∈ authoritative_guardian_set`

This means one SVM-verified signer public key is checked for membership in one authoritative guardian set.

It does not mean:

- quorum;
- authorization;
- replay safety;
- account mutation;
- processed event marking;
- CPI;
- mint;
- handler wiring;
- live route enablement.

## Accepted Positive Properties

The implementation correctly provides:

- Phase 41F.2 native Ed25519 verification status and flag gate;
- Phase 41F.1 checked extraction gate;
- extracted public key slice existence check;
- verified range existence check;
- public key range equality check;
- matched instruction index equality check;
- instruction data length equality check;
- Phase 41G payload hash binding status gate;
- caller-supplied guardian set rejection;
- unauthenticated guardian set rejection;
- empty guardian set rejection;
- threshold zero rejection;
- threshold greater than guardian count rejection;
- duplicate guardian public key rejection;
- configured guardian set ID check;
- payload guardian set ID check against authoritative guardian set ID;
- verified signer membership check;
- downstream false flags preserved.

## Mandatory Note 1 — Decoded Payload Must Be Bound To 41G Hash-Bound Bytes

Audit Demon identified a new binding gap.

Current 41H accepts:

- `PayloadHashBindingEstablished`
- `DecodedGuardianPayloadRaw`

as separate inputs.

`PayloadHashBindingEstablished` is currently only a status marker.

It does not carry the raw payload bytes.

It does not carry a decoded-payload commitment.

It does not prove that the `DecodedGuardianPayloadRaw` input was decoded from the same raw payload bytes that Phase 41G hash-bound to the signed message.

Therefore, the current `decoded_payload.guardian_set_id == authoritative_guardian_set.guardian_set_id` check is present, but its soundness depends on caller discipline.

Required closure before handler or live wiring:

- 41H must either accept `raw_payload_bytes` and re-decode internally; or
- 41G must expose a commitment that binds decoded fields to the hash-bound bytes.

Preferred immediate closure:

- add Phase 41H.1 decoded payload binding hardening;
- change 41H so it receives `raw_payload_bytes`;
- internally call `decode_guardian_payload_raw(raw_payload_bytes)`;
- use the internally decoded payload for guardian_set_id linkage;
- remove free `DecodedGuardianPayloadRaw` from the public 41H function input.

This mirrors the already strong 41F.1 ↔ 41F.2 binding discipline.

## Mandatory Note 2 — Future Authoritative Wrapper Construction Must Remain Unforgeable

The current code correctly makes `from_program_controlled_on_chain_source` private.

The current public constructors only create rejected variants.

Future guardian-set account loading must preserve this invariant.

The future production path must not introduce a wide public constructor that allows caller data to be marked authoritative.

The future authenticated account-loading gate must be the only production path that can construct an authoritative guardian set wrapper.

## Minor Style Note

`duplicate_guardian_public_key_index` currently uses a provably in-bounds slice expression.

This is not a safety blocker.

However, future hardening may replace it with a `.get(..index)` pattern to better match the project discipline against unchecked slicing.

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

Do not proceed directly to 41I yet.

Next gate:

Phase 41H.1 — Decoded Payload Binding Hardening.

Goal:

Bind the decoded payload used by 41H to the raw payload bytes hash-bound by 41G.

Recommended implementation direction:

- 41H receives raw payload bytes;
- 41H internally decodes them using the existing raw payload decoder;
- 41H uses only the internally decoded payload for guardian set ID linkage;
- free caller-provided decoded payload is removed from the 41H public function boundary;
- no quorum/auth/replay/mutation/CPI/mint/handler/live route is introduced.
