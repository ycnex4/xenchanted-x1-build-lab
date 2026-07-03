# XXXL Phase 41K.1 — Real Instructions Sysvar Loading Plan Review Request

Date: 2026-07-03

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-1-instructions-sysvar-plan`

Base checkpoint:

`bd53ace Merge XXXL phase 41K live-wiring plan acceptance`

## Scope

Docs-only review for Phase 41K.1 real Instructions sysvar loading plan.

No Rust code is changed.

## Purpose

41K.1 plans the first runtime-derived authority source:

`real Instructions sysvar`

It replaces the previous boundary-model assumption:

`preloaded prior instruction data`

with:

`real Instructions sysvar -> checked current instruction index -> real prior Ed25519 instruction -> accepted 41F.1 / 41F.2 pipeline`

## Review Focus

Please check whether the plan correctly preserves the Model A live-wiring soundness precondition:

- XXXL verifier executes as the current instruction;
- current instruction index comes from the real Instructions sysvar;
- current index is not caller-provided;
- Ed25519 instruction is a real prior precompile instruction in the same transaction;
- prior index is strictly less than current index;
- prior instruction data comes from the real sysvar entry;
- reaching current instruction implies the prior Ed25519 precompile verification already passed.

## Questions

1. Is the Model A live-wiring soundness precondition explicit enough?
2. Is the current instruction index required to come from real Instructions sysvar?
3. Is caller-provided current index rejected?
4. Is the prior Ed25519 instruction required to be strictly prior to current instruction?
5. Is Ed25519 program id verification required?
6. Are fabricated instruction entries rejected?
7. Are frontend/watcher supplied proofs rejected as authority?
8. Does 41K.1 avoid guardian-set PDA loading?
9. Does 41K.1 avoid processed-registry PDA loading?
10. Does 41K.1 avoid mutation, CPI, mint, handler, and live route?
11. Is this plan sufficient before 41K.1 code?

## Expected Verdict

- Verdict: ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES
- Required fixes:
- Non-blocking notes:
- Is this sufficient before 41K.1 code:
