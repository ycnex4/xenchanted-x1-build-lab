# XXXL Phase 41K.2 — Guardian-Set Account/PDA Loading Plan Acceptance

Date: 2026-07-03

Status: accepted plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-plan`

Accepted commits:

- `7c0d176 Document phase 41K.2 guardian-set loading plan`
- `265740f Address phase 41K.2 guardian-set loading plan review notes`

Base main:

`20d65a8 Merge XXXL phase 41K.1 instructions sysvar implementation acceptance`

## Final Verdict

Phase 41K.2 real guardian-set account/PDA loading plan is accepted.

Required fixes: none.

## Reviewer Verdicts

Theo:

- Verdict: ACCEPT
- Required fixes: none
- Non-blocking notes: none
- Sufficient before 41K.2 code: yes

Audit Demon:

- Verdict: ACCEPT WITH NOTES
- Required fixes: none
- Sufficient before 41K.2 code: yes

## Notes Addressed

Demon notes were incorporated into the plan before acceptance:

- exact guardian-set PDA seed format must be fixed before 41K.2 code acceptance;
- guardian-set PDA must be program-derived under the expected XXXL program id / runtime authority;
- guardian-set account owner must be the expected XXXL program id;
- stored guardian_set_id must match the guardian_set_id implied by the PDA seed;
- future 41K.5 handler must pass the guardian-set account as read-only, non-writable, non-signer;
- uninitialized / zero-discriminator guardian-set accounts must be explicitly rejected.

## Accepted 41K.2 Scope

41K.2 plans only real guardian-set account/PDA loading.

Accepted runtime source:

`real guardian-set account/PDA -> checked account identity -> checked account data decode -> authoritative guardian-set wrapper`

The accepted output must be equivalent to:

`AuthoritativeGuardianSetSource::ProgramControlledOnChain`

## Still Out of Scope

41K.2 does not enable:

- processed-registry PDA loading;
- replay registry write;
- processed event marking;
- atomic check-mark-mint;
- account mutation;
- CPI;
- invoke_signed;
- SPL token mint;
- process instruction handler;
- live route;
- guardian-set governance;
- guardian-set update instruction;
- deployment of production guardian set.

## Future Gate

41K.2 code implementation must be separately reviewed before merge.
