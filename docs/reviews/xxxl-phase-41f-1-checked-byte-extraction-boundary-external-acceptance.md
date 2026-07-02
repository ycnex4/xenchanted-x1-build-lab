# XXXL Phase 41F.1 Checked Ed25519 Byte Extraction Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`698b1df Merge XXXL phase 41F checked Ed25519 byte extraction boundary`

## Scope Accepted

Phase 41F.1 is accepted as a checked byte extraction boundary.

Accepted scope:

- consume Phase 41E parsed offsets;
- consume already loaded prior instruction data;
- require `Ed25519InstructionBytesParsed`;
- require matched instruction index;
- require parsed offsets;
- find matching loaded prior instruction by index;
- re-check loaded entry is runtime-data-only;
- check loaded data length matches the Phase 41E parse result;
- extract signature as checked `&[u8; 64]`;
- extract public key as checked `&[u8; 32]`;
- extract message as borrowed checked `&[u8]`;
- avoid attacker-sized message `Vec` copy;
- keep the boundary non-authorizing.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted:

- Phase 41F.1 correctly implements checked extraction boundary;
- signature and public key fixed array references are safe;
- message is a borrowed slice without attacker-sized copy;
- there is no crypto verification drift;
- there is no proof/evidence/guardian/quorum/auth drift;
- Phase 41F.2 planning may begin after acceptance.

Theo highlighted the intended pipeline:

- Phase 41E — parse;
- Phase 41F.1 — extract;
- Phase 41F.2 — verify.

Theo confirmed the extraction primitives:

- `checked_fixed_array_slice::<LEN>` checks `range.len == LEN`;
- `checked_slice` performs checked offset arithmetic and checked `.get(range)`;
- `.try_into().ok()` keeps fixed-array conversion fail-closed;
- message is borrowed and not copied.

## Audit Demon Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Scope violations: no.

Checked extraction acceptable: yes.

Message borrow/no-copy acceptable: yes.

Forbidden operations detected: no.

Signature verification drift: no.

Trust-sensitive boundary drift: no.

Next phase allowed: yes.

Demon accepted:

- extraction is memory-safe;
- extraction is zero-copy;
- all trust/execution flags remain false;
- there is no weakening;
- the boundary is panic-safe;
- checked fixed-array extraction is acceptable;
- borrowed message extraction is acceptable;
- no crypto verification was introduced;
- no native Ed25519 verification establishment was introduced;
- no proof/evidence/guardian/quorum/auth/replay/mutation/CPI/mint/live drift was introduced.


## Demon Note 1 — SAFETY_FLAGS Semantics

Demon noted a non-blocking convention mismatch.

Phase 41F.1 sets several capability flags to local-false:

- `account_info_parser_implemented: false`;
- `load_instruction_called: false`;
- `load_instruction_enabled: false`;
- `concrete_runtime_api_selected: false`;
- `current_instruction_identity_derived_from_runtime: false`.

Earlier Phase 41D3.2.2 enabled these as part of the cumulative runtime pipeline, and Phase 41E.1 preserved them as true.

This is not a security weakening because Phase 41F.1 only moves capability flags downward and all trust/execution flags remain false.

However, future work must clarify whether `SAFETY_FLAGS` are:

- cumulative pipeline capability flags; or
- local module capability flags.

Future phases should bring 41E.1 and 41F.1 to one consistent convention before flipping any signature-verification flag.

## Demon Note 2 — Program ID Defense-In-Depth Re-Check

Demon noted that Phase 41F.1 re-checks:

- runtime-data-only loaded entry;
- `!is_evidence`;
- `!authorizes_execution`;
- instruction data length matches the Phase 41E parse result.

But Phase 41F.1 does not re-check:

- `loaded_entry.instruction.program_id == ed25519_program::id()`.

This is not a vulnerability in the accepted boundary because:

- Phase 41F.1 is non-authorizing;
- extracted bytes do not establish signature validity;
- pairing of loading result and parsing result is caller-controlled inside the staged pipeline, not attacker-controlled proof acceptance;
- length mismatch catches many forms of stale pairing.

However, a future defense-in-depth hardening phase should add or consider a program-id re-check for consistency with Phase 41E.1.

## Accepted Validation

Validation run before merge:

- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml`;
- `cargo fmt --manifest-path programs/xxxl-svm/Cargo.toml --check`;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml checked_ed25519_byte_extraction_boundary --lib` — 11/11 OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml verifier --lib` — OK;
- `cargo test --manifest-path programs/xxxl-svm/Cargo.toml --lib --locked` — OK;
- `npm run typecheck` — OK;
- `npm run build` — OK.

## Still Forbidden

The following remain forbidden after Phase 41F.1 acceptance:

- local cryptographic verification;
- native Ed25519 verification establishment;
- signature validity acceptance;
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

No blocker is removed, weakened, or reinterpreted by Phase 41F.1.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41F.1 is externally accepted.

Next allowed step:

- Phase 41F.2 — Ed25519 signature verification boundary plan.

Phase 41F.2 must be a separate review gate.

Before any signature-verification flag is flipped, future work must address the non-blocking 41F.1 notes:

- clarify cumulative-vs-local `SAFETY_FLAGS` semantics;
- document Model A soundness;
- preserve self-reference binding;
- keep signature validity separate from proof/evidence/guardian/quorum/auth;
- consider program-id defense-in-depth re-check.

