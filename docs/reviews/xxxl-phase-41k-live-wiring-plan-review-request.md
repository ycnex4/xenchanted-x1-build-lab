# XXXL Phase 41K — Live-Wiring Plan Review Request

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-live-wiring-plan`

Base checkpoint:

`03b1e4f Merge XXXL phase 41J replay protection implementation acceptance`

## Scope

Docs-only review request for Phase 41K live-wiring plan.

No Rust code is changed.

## What 41K Plans

41K is the transition from accepted boundary model to future runtime wiring.

The plan explicitly treats 41K as a high-risk gate and splits it into sub-gates:

- 41K.1 real Instructions sysvar loading;
- 41K.2 real guardian-set account/PDA loading;
- 41K.3 real processed-registry PDA loading;
- 41K.4 atomic check-mark-mint design;
- 41K.5 handler / CPI / live route.

## Required Review Focus

Please review whether the plan correctly preserves the accepted chain:

`real Instructions sysvar -> checked extraction -> native Ed25519 verification -> payload hash binding -> guardian membership -> quorum -> same raw payload decode -> canonicalEventKey -> real processed-registry PDA -> atomic check-mark-mint`

Please specifically check:

1. Is 41K correctly treated as a high-risk live-wiring gate?
2. Is the split into sub-gates sufficient?
3. Are AccountInfo/sysvar/PDA/CPI/handler/live-route surfaces still separately gated?
4. Is atomicity specified strongly enough?
5. Are marked-but-not-minted and minted-but-not-marked windows forbidden?
6. Is payload substitution across quorum/replay/mint forbidden?
7. Is authoritative guardian-set construction constrained to real account/PDA loading?
8. Is authoritative processed-registry construction constrained to real PDA loading?
9. Are caller/watcher/frontend supplied authority sources rejected?
10. Is this plan sufficient before any 41K code?

## Expected Verdict Format

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.1 plan/code:
