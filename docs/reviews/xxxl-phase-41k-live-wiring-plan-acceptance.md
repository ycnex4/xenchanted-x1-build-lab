# XXXL Phase 41K — Live-Wiring Plan Acceptance

Date: 2026-07-03

Status: accepted plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-live-wiring-plan`

Accepted commits:

- `2a0d36f Document phase 41K live-wiring plan`
- `6f000ac Address phase 41K live-wiring plan review notes`

Base main:

`03b1e4f Merge XXXL phase 41J replay protection implementation acceptance`

## Final Verdict

Phase 41K live-wiring master plan is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Sufficient before 41K.1 plan/code: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Sufficient before 41K.1 plan/code: yes

## Notes Addressed

Demon notes were incorporated into the plan:

- Model A live-wiring soundness precondition was explicitly added to 41K.1.
- Current instruction index must come from the real Instructions sysvar, not caller input.
- Prior Ed25519 instruction must be a real prior precompile in the same transaction.
- Future 41K.5 handler wiring must enforce the same Model A precondition.
- Active deployment blockers were mapped to 41K sub-gates.
- Current design checkpoint was updated.

## Accepted 41K Decomposition

41K remains split into high-risk sub-gates:

- 41K.1 real Instructions sysvar loading;
- 41K.2 real guardian-set account/PDA loading;
- 41K.3 real processed-registry PDA loading;
- 41K.4 atomic check-mark-mint design;
- 41K.5 handler / CPI / live route.

## Accepted Core Rule

41K must preserve the full chain:

`real Instructions sysvar -> checked extraction -> native Ed25519 verification -> payload hash binding -> guardian membership -> quorum -> same raw payload decode -> canonicalEventKey -> real processed-registry PDA -> atomic check-mark-mint`

Any implementation path that lets the caller inject, substitute, or desynchronize any element is invalid.

## Important Boundary

This acceptance is for the master plan only.

It does not implement or enable:

- AccountInfo;
- Instructions sysvar loading;
- real guardian-set PDA loading;
- real processed-registry PDA loading;
- registry write;
- processed event marking write;
- CPI;
- mint;
- handler;
- live route.

Each 41K sub-gate requires separate review and acceptance.
