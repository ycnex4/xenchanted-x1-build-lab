# XXXL Phase 41D3.2.3 Prefilter + Phase 41C3 Candidate Descriptor Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`85f74f6 Merge XXXL phase 41D3 prefilter descriptor boundary`

Code commit:

`e40b241 Add phase 41D3 prefilter descriptor boundary`

## Scope Accepted

Phase 41D3.2.3 is accepted as a runtime code boundary for structural prefiltering and Phase 41C3 candidate descriptor construction.

Accepted scope:

- consume loaded prior instructions from Phase 41D3.2.2;
- process runtime-data-only entries;
- iterate loaded entries by reference;
- discard unrelated non-candidates immediately;
- prefilter by Ed25519 program id only;
- construct Phase 41C3 candidate descriptors;
- delegate duplicate, ambiguous, ordering, same-index, and later-index handling to Phase 41C3;
- keep descriptors non-authorizing;
- flip `locates_prior_ed25519_instruction: true` only as structural lookup/candidate-location boundary signal.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- implementation is limited to structural prefiltering and descriptor construction;
- loaded entries are iterated by reference;
- unrelated non-candidates are discarded immediately;
- descriptor storage avoids cloning full `Instruction`;
- candidate descriptors are non-authorizing;
- duplicate/ambiguous/ordering handling is delegated to Phase 41C3;
- same-index candidates are rejected through Phase 41C3;
- later-index candidates are rejected through Phase 41C3;
- `locates_prior_ed25519_instruction` has no semantic drift;
- cryptographic verification is absent;
- proof acceptance is absent;
- evidence/quorum/auth/replay/mutation/CPI/mint/live boundaries remain closed;
- all trust-sensitive flags except structural lookup/location remain false;
- next phase may start after acceptance.

Theo semantic note:

`locates_prior_ed25519_instruction: true` means that the structural lookup layer was activated and completed.

It does not mean:

- a valid Ed25519 instruction was accepted;
- signature was verified;
- proof was accepted;
- evidence was accepted;
- guardian was valid;
- quorum was reached;
- execution was authorized;
- mint was allowed.

## Audit Demon Verdict

Verdict: ACCEPT WITH NOTES

Required fixes: none.

Blocking issue: none.

Scope violations: no.

Forbidden operations detected: no.

Trust-sensitive boundary drift: no.

Descriptor boundary acceptable: yes.

41C3 delegation acceptable: yes.

Next phase allowed: yes.

Accepted findings:

- 6 expected files changed;
- `mod.rs` delta is one `pub mod`;
- `current-design` update is append-only;
- runtime module consumes already loaded Phase 41D3.2.2 result;
- no new `instructions::load_*`, `AccountInfo`, or sysvar access;
- no raw parser;
- no instruction data slicing;
- no Ed25519 verify;
- no keccak;
- no quorum/auth/replay/CPI/invoke_signed/mint_to/handler/live route;
- no panic/unwrap/expect/unsafe/unchecked indexing;
- fail-closed behavior is preserved;
- all evidence/proof/quorum/auth/replay/processed/mutation/CPI/invoke_signed/mint/handler/live flags remain false;
- descriptors are non-authorizing metadata only;
- Phase 41D3.2.3 delegates descriptor evaluation to the accepted Phase 41C3 model.

## Demon Note 1 — Result vs Capability Semantics

Audit Demon noted that the plan wording said:

`locates_prior_ed25519_instruction: true`

only when a prior Ed25519 structural candidate is located.

The implemented code uses the flag when Phase 41C3 delegation is reached and completed, including not-found, same/later reject, or ambiguous cases.

This is accepted as non-blocking because:

- evidence acceptance remains false;
- authorization remains false;
- mutation/mint/live route remain false;
- no trust is conferred by the flag.

Accepted interpretation for Phase 41D3.2.3:

`locates_prior_ed25519_instruction: true`

means structural lookup capability/layer was activated and completed.

It must not be used as a downstream evidence gate.

Downstream phases must gate only on both:

- `status == PriorEd25519InstructionStructurallyLocated`;
- `matched_instruction_index.is_some()`.

Downstream phases must not gate on:

- `locates_prior_ed25519_instruction`.

## Demon Note 2 — Descriptor Boolean Placeholders

Audit Demon noted that descriptor booleans are set without inspecting instruction data bytes:

- `structurally_well_formed_candidate: true`;
- `guardian_evidence_unique: true`;
- `matches_expected_current_identity_binding: true`.

This is accepted for Phase 41D3.2.3 because this phase does not parse proof bytes and does not accept evidence.

However, future evidence parsing must not treat those booleans as validated facts.

Future evidence parsing must:

- read and validate real Ed25519 instruction bytes;
- independently validate structural well-formedness;
- independently validate guardian identity/binding;
- independently validate uniqueness/ambiguity rules;
- not rely on Phase 41D3.2.3 descriptor booleans as evidence.

Recommended future consideration:

- rename or reinterpret the booleans as program-id-match-only placeholders before the evidence phase consumes them.

## Phase 41C3 Delegation

Accepted boundary:

- Phase 41D3.2.3 performs structural prefiltering by Ed25519 program id only;
- Phase 41D3.2.3 creates minimal candidate descriptors;
- Phase 41D3.2.3 calls `locate_prior_ed25519_lookup_ordering_boundary`;
- Phase 41C3 remains authoritative for ordering, same/later rejection, duplicate handling, and ambiguous candidate handling.

Phase 41D3.2.3 must not become an authorization layer.

## Validation Accepted

Local validation passed before merge:

- forbidden trust/execution check: OK
- raw sysvar/loading check: OK
- panic token check: OK
- unchecked index/slice check: OK
- `cargo test prefilter_phase_41c3_candidate_descriptor_runtime_boundary --lib`: OK
- `cargo test verifier --lib`: OK
- `cargo test --lib --locked`: OK
- `npm run typecheck`: OK
- `npm run build`: OK

## Phase 41D3 Completion

With Phase 41D3.2.3 accepted, the structural prior-instruction lookup pipeline is complete:

- Phase 41D1 — AccountInfo presence/readability;
- Phase 41D2 — current instruction identity;
- Phase 41D3.1 — current index acquisition;
- Phase 41D3.2.1 — prior index range;
- Phase 41D3.2.2 — checked prior instruction loading;
- Phase 41D3.2.3 — prefilter + Phase 41C3 descriptors.

## Still Forbidden

The following remain forbidden after this acceptance:

- Ed25519 cryptographic verification;
- signature proof acceptance;
- verification evidence acceptance;
- guardian quorum counting;
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

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.3.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

The next phase may start only as a separate reviewed micro-phase.

Expected focus for the next trust-sensitive phase:

- do not gate on `locates_prior_ed25519_instruction`;
- gate on located status plus matched instruction index;
- do not trust descriptor booleans as validated evidence;
- parse and validate real Ed25519 instruction bytes under a separate audit boundary;
- keep proof/evidence/quorum/auth/replay/mutation/CPI/mint/live route closed until their own acceptance gates.
