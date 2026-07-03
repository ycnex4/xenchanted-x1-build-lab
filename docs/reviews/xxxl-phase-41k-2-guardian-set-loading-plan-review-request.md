# XXXL Phase 41K.2 — Guardian-Set Account/PDA Loading Plan Review Request

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-plan`

Base checkpoint:

`20d65a8 Merge XXXL phase 41K.1 instructions sysvar implementation acceptance`

## Scope

Docs-only review for Phase 41K.2 real guardian-set account/PDA loading plan.

No Rust code is changed.

## Purpose

41K.2 plans the second runtime-loading surface of 41K:

`real guardian-set account/PDA -> checked account identity -> checked account data decode -> authoritative guardian-set wrapper`

The goal is to replace abstract guardian-set input with a real program-controlled on-chain guardian-set source.

## Review Focus

Please review whether the plan correctly preserves the accepted 41H/41I boundary:

- guardian-set data must come from a real program-controlled account/PDA;
- caller-supplied guardian sets must remain rejected;
- frontend/watcher/relayer guardian sets must not become authority;
- PDA/account identity must be checked before data trust;
- owner/program authority must be checked;
- account schema/discriminator/version must be checked;
- threshold, count, active flag, and duplicate guardian keys must be validated;
- loaded guardian-set id must remain bindable by 41H to decoded payload guardian_set_id;
- output must be an internal authoritative wrapper equivalent to `ProgramControlledOnChain`;
- 41K.2 must not enable registry, replay write, mutation, CPI, mint, handler, or live route.

## Questions

1. Is the real guardian-set account/PDA source rule explicit enough?
2. Is caller-supplied guardian-set data rejected?
3. Is frontend/watcher/relayer guardian-set data rejected as authority?
4. Is PDA/account identity verification required before data trust?
5. Is owner/program authority verification required?
6. Is schema/discriminator/version validation required?
7. Is guardian_set_id binding preserved for 41H?
8. Is threshold validation sufficient?
9. Are empty guardian sets rejected?
10. Are duplicate guardian public keys rejected?
11. Is inactive/deprecated guardian-set handling explicit enough?
12. Is the authoritative wrapper rule explicit?
13. Does 41K.2 avoid processed-registry PDA loading?
14. Does 41K.2 avoid replay write / processed event marking?
15. Does 41K.2 avoid mutation, CPI, mint, handler, and live route?
16. Are deployment blockers preserved?
17. Is this plan sufficient before 41K.2 code?

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.2 code:
