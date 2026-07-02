# XXXL Phase 41F.0 Ed25519 Cryptographic Verification Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`39b1a94 Merge XXXL phase 41F Ed25519 verification plan`

## Scope Accepted

Phase 41F.0 is accepted as a docs-only plan.

No runtime code was introduced.

Accepted planning scope:

- plan the trust-sensitive Ed25519 signature validity boundary after Phase 41E completion;
- keep signature validity separate from proof acceptance;
- keep signature validity separate from evidence acceptance;
- keep signature validity separate from guardian validity;
- keep signature validity separate from quorum;
- keep signature validity separate from authorization;
- keep signature validity separate from replay writes, mutation, CPI, mint, handler, and live route;
- prefer the native Ed25519 instruction verification model unless external review says otherwise;
- defer local cryptographic verification unless explicitly reviewed;
- split checked byte extraction before signature verification.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- Phase 41F.0 is the correct next docs-only plan after Phase 41E;
- separation between signature validity and proof/evidence/auth is clear;
- Model A native Ed25519 instruction verification is the correct default;
- Model B local cryptographic verification is correctly deferred;
- checked extraction as a separate Phase 41F.1 gate is correct;
- message range verification is not message correctness;
- parsed public key is not guardian validity;
- fail-closed requirements are sufficient;
- active blockers are preserved;
- Phase 41F.1 may begin after acceptance.

Theo recommended Phase 41F.1 scope:

- checked fixed extraction of signature bytes;
- checked fixed extraction of public key bytes;
- checked borrowed message slice;
- no cryptographic verification yet;
- no proof/evidence/guardian/quorum/auth.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Native verification model preference acceptable: yes.

Checked extraction sub-phase acceptable: yes.

Signature/proof separation acceptable: yes.

Trust-sensitive boundary drift: no.

Next sub-phase allowed: yes.

Demon accepted:

- docs-only scope is clean;
- Phase 41F.0 is a correct next plan after Phase 41E completion;
- signature validity is clearly separated from proof/evidence/guardian/quorum/auth;
- Model A native Ed25519 instruction verification preference is acceptable;
- Model B local cryptographic verification is correctly deferred;
- checked byte extraction as Phase 41F.1 is acceptable;
- checked extraction requirements are sufficient;
- fail-closed requirements are sufficient;
- active blockers are preserved;
- Phase 41F.1 checked extraction may begin after acceptance;
- Phase 41F.2 cryptographic verification remains a separate review gate.

## Demon Note 1 — Model A Soundness Argument

Future Phase 41F.1 or Phase 41F.2 documentation must explicitly state the Model A soundness argument.

Model A rests on three conditions:

1. A prior native Ed25519 instruction is present in the executed transaction.
2. The current instruction was reached.
3. The native Ed25519 instruction would have aborted the transaction if its signature verification failed.

Therefore, under Model A:

- reaching the current instruction means the prior native Ed25519 verification already passed.

Future docs should avoid weaker wording such as:

- “would have verified if the transaction reached the current instruction.”

Preferred wording:

- “the native Ed25519 instruction already verified the signature because the transaction reached the current instruction.”

## Demon Note 2 — Self-Reference Invariant

Model A must continue to require the Phase 41E self-reference invariant:

- signature instruction index == `u16::MAX`;
- public key instruction index == `u16::MAX`;
- message instruction index == `u16::MAX`.

This invariant binds the bytes verified by the native Ed25519 instruction to the parsed signature/public-key/message ranges used by Phase 41F.

Any future support for non-self instruction references must remain a separate reviewed loading/binding gate.

## Demon Note 3 — Signature Validity Still Requires Later Binding

Even under Model A, signature validity does not prove:

- parsed public key is an active guardian;
- parsed message is the expected gateway payload;
- message hash matches the expected payload hash;
- route binding is correct;
- target mint binding is correct;
- recipient binding is correct;
- amount binding is correct;
- finality/expiration binding is correct;
- proof is accepted;
- evidence is accepted;
- quorum is reached;
- execution is authorized.

Those remain later separate gates.

## Demon Note 4 — Status Model Must Be Verification-Model Attributed

Future Phase 41F status models must distinguish Model A and Model B paths.

Under Model A:

- invalid signature is normally not reachable at runtime because the native Ed25519 instruction would abort the transaction before the current instruction executes.

Under Model B:

- invalid signature is reachable because local crypto verification may explicitly return invalid.

Therefore, statuses such as:

- `Ed25519SignatureInvalid`;
- `NativeEd25519VerificationNotEstablished`;

must be attributed to the selected verification model and must not create misleading dead paths or accidental proof acceptance.

## Accepted Sub-Phase Split

Accepted future split:

### Phase 41F.1 — Checked Byte Extraction Boundary

Allowed future scope:

- extract signature bytes only through checked access;
- extract public key bytes only through checked access;
- expose message only as a checked borrowed slice/reference;
- no attacker-sized message `Vec` copy;
- no crypto verification;
- no proof/evidence/guardian/quorum/auth.

### Phase 41F.2 — Ed25519 Signature Verification Boundary

Allowed only after separate review.

Possible future scope:

- establish signature validity under the selected model;
- deterministic invalid-signature or verification-not-established reject;
- still no proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live route.

## Still Forbidden

The following remain forbidden after Phase 41F.0 acceptance:

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

Additionally, no Phase 41F.0 runtime code was introduced.

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

## Next Gate

Phase 41F.0 is externally accepted.

Next allowed sub-phase:

- Phase 41F.1 — checked signature/public-key/message byte extraction boundary.

Phase 41F.1 must not perform cryptographic verification.

Phase 41F.1 must not flip `ed25519_signature_verification_performed`.

Phase 41F.1 must not accept proof, evidence, guardian validity, quorum, authorization, replay, mutation, CPI, mint, handler, or live route.
