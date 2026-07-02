# XXXL Phase 41F.0 — Ed25519 Cryptographic Verification Plan

Date: 2026-07-02

## Status

Planning document only.

No runtime code is introduced in Phase 41F.0.

## Parent Gate

Phase 41E is complete.

Accepted checkpoint:

`2f759b7 Merge XXXL phase 41E offset table hardening acceptance record`

Accepted Phase 41E pipeline:

- Phase 41E.0 — Ed25519 byte parsing plan;
- Phase 41E.1 — Ed25519 byte parsing code boundary;
- Phase 41E.2 — offset-table alias hardening.

## Purpose

Phase 41F.0 plans the next trust-sensitive boundary after byte parsing:

Ed25519 cryptographic verification.

This must be introduced without accidentally accepting proof, accepting evidence, accepting guardian validity, counting quorum, authorizing execution, writing replay state, mutating accounts, enabling CPI, minting, adding a handler, or unlocking live route.

## Critical Separation

Phase 41F is allowed to establish only signature validity for a parsed Ed25519 instruction.

Signature validity is not the same as:

- proof acceptance;
- guardian validity;
- evidence acceptance;
- quorum;
- authorization;
- replay write permission;
- state mutation permission;
- CPI permission;
- mint permission;
- live route permission.

## Verification Model To Review

Because the target runtime is SVM, Phase 41F must explicitly choose and document the verification model before code.

Two candidate models must be reviewed:

### Model A — Native Ed25519 Instruction Verification Boundary

Under this model, cryptographic verification is provided by a prior native Ed25519 program instruction.

The runtime parser confirms that the prior instruction:

- is the Ed25519 program;
- is structurally prior to the current instruction;
- was loaded through the accepted checked prior-instruction pipeline;
- has parsed signature/public-key/message ranges;
- has no cross-instruction references;
- has no out-of-bounds ranges;
- has no range overlap;
- has no offset-table aliasing.

Then Phase 41F may model the fact that the native Ed25519 instruction would have verified the signature if the transaction reached the current instruction.

This model must still not accept proof/evidence/guardian/quorum/auth.

### Model B — Local Cryptographic Verification Boundary

Under this model, program-side code performs Ed25519 signature verification over parsed bytes.

This model may require additional library/runtime review.

If Model B is selected later, the code phase must explicitly review:

- supported crypto crate;
- SVM/BPF compatibility;
- deterministic behavior;
- compute cost;
- no heap surprises;
- no panic paths;
- no side effects;
- no proof/evidence/auth drift.

Model B must not be introduced accidentally.

## Preferred Planning Direction

Phase 41F.0 prefers Model A unless external review says otherwise.

Reason:

- Phase 41D/41E are already built around prior Ed25519 instruction introspection;
- the Ed25519 instruction is already a native SVM verification mechanism;
- adding local crypto may introduce avoidable runtime/library risk.

The plan does not finalize the model without review.

## Required Input From Phase 41E

Future Phase 41F code may consume only accepted Phase 41E parsed output.

Required conditions:

- parser status indicates bytes parsed successfully;
- parsed signature range exists;
- parsed public key range exists;
- parsed message range exists;
- cross-instruction references were rejected;
- offset-table aliasing was rejected;
- overlapping ranges were rejected;
- message is still represented as bounded indices;
- no attacker-sized message copy is required.

## Required Checked Extraction

Any future extraction of signature/public-key/message bytes must use checked access only.

Allowed future extraction pattern:

- fixed-size signature range: 64 bytes;
- fixed-size public key range: 32 bytes;
- variable-length message range: borrowed checked slice or bounded reference;
- no unchecked indexing;
- no unchecked slicing;
- no `unwrap`;
- no `expect`;
- no `panic!`;
- no `unsafe`.

Future code must not copy attacker-sized message bytes into a new `Vec`.

## Message Range Guardrail

Future Phase 41F may verify signature validity over the parsed message range.

It must not yet claim that the message is the correct gateway payload.

Message correctness, payload hash matching, route binding, target mint binding, recipient binding, amount binding, finality binding, and expiration binding must remain separate future gates.

## Public Key Guardrail

Future Phase 41F may use the parsed public key as the key involved in signature verification.

It must not yet claim that the public key is an active guardian.

Guardian-set membership must remain a later separate gate.

## Signature Guardrail

Future Phase 41F may determine whether the parsed signature is valid for the parsed public key and parsed message bytes.

It must not yet claim:

- proof accepted;
- evidence accepted;
- guardian accepted;
- quorum reached;
- execution authorized.

## Proposed Future Status Model

A future Phase 41F code boundary should use explicit non-authorizing statuses, such as:

- `Ed25519BytesNotParsed`;
- `ParsedSignatureRangeUnavailable`;
- `ParsedPublicKeyRangeUnavailable`;
- `ParsedMessageRangeUnavailable`;
- `CheckedSignatureSliceUnavailable`;
- `CheckedPublicKeySliceUnavailable`;
- `CheckedMessageSliceUnavailable`;
- `UnsupportedVerificationModel`;
- `NativeEd25519VerificationNotEstablished`;
- `Ed25519SignatureInvalid`;
- `Ed25519SignatureVerified`.

Final names may change during implementation, but the status model must stay deterministic and must not imply proof/evidence/auth.

## Proposed Future Flags

A future Phase 41F code boundary may introduce a narrow flag such as:

- `ed25519_signature_verification_performed: true`.

If this flag is introduced, it may mean only that signature verification was performed or established under the selected model.

It must not mean:

- proof accepted;
- evidence accepted;
- guardian validity accepted;
- quorum reached;
- authorization enabled;
- replay write enabled;
- state mutation enabled;
- CPI enabled;
- mint enabled;
- live route enabled.

All of those must remain false.

## Fail-Closed Requirements

Future Phase 41F code must fail closed on:

- Phase 41E parse status not successful;
- missing parsed ranges;
- checked range extraction failure;
- invalid fixed-size signature range;
- invalid fixed-size public key range;
- missing or empty message range;
- unsupported verification model;
- invalid signature;
- any crypto/library error;
- any integer overflow risk;
- any unchecked slice/index need.

Failure must be deterministic and non-authorizing.

## Still Forbidden In Phase 41F.0

Phase 41F.0 does not permit code.

Phase 41F planning does not permit:

- proof acceptance;
- verification evidence acceptance;
- guardian validity acceptance;
- guardian set membership acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- process instruction handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41F.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Suggested Sub-Phase Split

To avoid trust-boundary drift, Phase 41F should probably be split:

### Phase 41F.1 — Checked Byte Extraction Boundary

Possible future scope:

- extract fixed signature bytes through checked access;
- extract fixed public key bytes through checked access;
- expose message as checked bounded slice/reference;
- no crypto verification yet;
- no proof/evidence/auth.

### Phase 41F.2 — Ed25519 Signature Verification Boundary

Possible future scope:

- establish signature validity under the selected verification model;
- deterministic invalid-signature reject;
- still no proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live route.

This split is recommended but must be confirmed by review.

## Review Questions

External review should confirm:

1. Is Phase 41F.0 the correct next docs-only plan after Phase 41E completion?
2. Is the separation between signature validity and proof/evidence/auth clear enough?
3. Should Phase 41F prefer the native Ed25519 instruction verification model?
4. Should local cryptographic verification be deferred unless explicitly reviewed?
5. Is checked byte extraction a separate 41F.1 gate before signature verification?
6. Is message range verification clearly not message correctness/proof acceptance?
7. Is parsed public key use clearly not guardian validity?
8. Are fail-closed requirements sufficient?
9. Are active blockers preserved?
10. Can Phase 41F.1 checked byte extraction planning/code begin after acceptance?

## Next Gate

If this plan is externally accepted, the next recommended step is:

Phase 41F.1 — checked signature/public-key/message byte extraction boundary.

That boundary should still perform no cryptographic verification unless reviewers explicitly request combining extraction and verification.

Any cryptographic verification code must remain under a separate review gate.
