# XXXL Phase 41D3.2.3.0 Prefilter + Phase 41C3 Candidate Descriptor Plan — External Acceptance

Date: 2026-07-02

Current main under review:

`5147b32 Merge XXXL phase 41D3 prefilter descriptor plan`

## Scope Accepted

Phase 41D3.2.3.0 is accepted as a docs-only plan before implementing prefiltering and Phase 41C3 candidate descriptor construction.

No runtime code was introduced.

Accepted planning scope:

- consume loaded prior instructions from Phase 41D3.2.2;
- process only loaded entries marked runtime-data-only;
- prefilter unrelated loaded prior instructions;
- identify Ed25519 program-id candidates structurally;
- construct Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- keep descriptors non-authorizing;
- allow `locates_prior_ed25519_instruction: true` only as structural candidate location.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Scope violations: no.

Descriptor boundary acceptable: yes.

Trust-sensitive wording acceptable: yes.

Next code sub-step allowed: yes.

Accepted findings:

- docs-only scope is clean;
- runtime code was not changed;
- descriptor boundary is acceptable;
- candidate descriptor means only structural location, not proof/evidence/auth;
- prefiltering by Ed25519 program id is acceptable as a structural non-authorizing step;
- explicit same-index reject is required;
- explicit later-index reject is required;
- malformed structural candidates remain deterministic and non-authorizing;
- duplicate or ambiguous structural candidates remain deterministic and non-authorizing;
- `locates_prior_ed25519_instruction: true` is the only new trust-sensitive flag allowed;
- crypto/proof/evidence/quorum/auth/replay/CPI/mint/live-route boundaries remain closed;
- all active blockers remain.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Accepted findings:

- docs-only plan is appropriate before the first `locates_prior_ed25519_instruction: true` flip;
- the distinction between structural location and evidence acceptance is clear;
- prefilter by program id is safe at this phase because it is only structural matching;
- candidate descriptors are correctly non-authorizing;
- same-index reject is required as defense in depth;
- later-index reject is required as defense in depth;
- streaming / heap guidance is sufficient;
- descriptor storage should avoid cloning full instruction data;
- malformed candidates must remain deterministic and non-authorizing;
- duplicate/ambiguous candidates must remain deterministic and non-authorizing;
- `locates_prior_ed25519_instruction` is the only new sensitive flag;
- all other boundaries remain closed;
- Phase 41D3.2.3 code may start after acceptance.

## Trust-Sensitive Meaning

Accepted meaning of:

`locates_prior_ed25519_instruction: true`

It may mean only:

- a prior instruction with the Ed25519 program id was structurally located;
- a non-authorizing candidate descriptor was created.

It must not mean:

- Ed25519 signature was verified;
- cryptographic signature proof was accepted;
- verification evidence was accepted;
- guardian quorum was counted;
- execution was authorized;
- replay registry may be written;
- runtime state may mutate.

## 41C3 Delegation Note

Audit Demon noted that Phase 41D3.2.3 should not re-invent duplicate, ambiguous, or ordering logic.

Accepted code boundary for Phase 41D3.2.3:

- construct structural candidate descriptors;
- feed those descriptors into the already reviewed Phase 41C3 ordering/ambiguity model;
- keep Phase 41C3 authoritative for:
  - `DuplicateGuardianEvidence`;
  - `AmbiguousCandidateEvidence`;
  - ordering cases.

Phase 41D3.2.3 must not duplicate or fork this logic.

## Naming Note For Future 41E

Theo noted that `locates_prior_ed25519_instruction` is acceptable for Phase 41D3.2.3, but the word `locates` may be semantically broad when later entering verification phases.

Non-blocking future consideration for Phase 41E or equivalent verification boundary:

- consider renaming or aliasing to `structurally_locates_prior_ed25519_instruction`; or
- consider `candidate_prior_ed25519_instruction_located`.

This is not a blocker for Phase 41D3.2.3.

## Streaming / Heap Note

Accepted implementation guidance for Phase 41D3.2.3:

- iterate loaded prior entries by reference;
- prefilter immediately;
- discard non-candidates immediately;
- store only minimal candidate metadata;
- avoid cloning full `Instruction` data unless bounded and justified;
- avoid holding all loaded instructions and full candidate copies simultaneously.

## Minimum Safe Phase 41D3.2.3 Boundary

Phase 41D3.2.3 may start after this acceptance record is merged.

Allowed:

- consume loaded prior instructions from Phase 41D3.2.2;
- prefilter unrelated instructions by program id;
- identify Ed25519 program-id candidates structurally;
- construct non-authorizing Phase 41C3 candidate descriptors;
- explicitly reject same-index candidates;
- explicitly reject later-index candidates;
- flip `locates_prior_ed25519_instruction: true`.

Still forbidden:

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
- handler;
- live route unlock.

## Active Blockers Remain

No blocker is removed, weakened, or reinterpreted by Phase 41D3.2.3.0.

Active blockers remain:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`
- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

## Next Gate

Phase 41D3.2.3 remains gated under its own code review before merge.
