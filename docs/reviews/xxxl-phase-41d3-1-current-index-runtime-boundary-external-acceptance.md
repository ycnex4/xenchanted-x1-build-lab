# XXXL Phase 41D3.1 Current Index Runtime Boundary — External Acceptance

Date: 2026-07-02

Current main under review:

`02cdaaa Merge XXXL phase 41D3 current index runtime boundary`

## Scope Accepted

Phase 41D3.1 introduced only the checked current-instruction index acquisition boundary.

Accepted scope:

- real runtime current-instruction index acquisition;
- `load_current_index_checked`;
- Instructions sysvar AccountInfo key check;
- missing/wrong account fail-closed;
- checked helper failure fail-closed;
- current index treated only as ordering data;
- no prior-instruction enumeration;
- no instruction loading;
- no Phase 41C3 descriptor construction;
- no proof/evidence/quorum/auth/replay/CPI/mint/handler/live route.

## Audit Demon Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking issues: none.

Accepted findings:

- scope violations: no;
- forbidden operations detected: no;
- blockers changed: no;
- trust-sensitive boundary drift: no;
- panic-safety: clean;
- next sub-step/code allowed: yes.

Important note:

`load_current_index_checked` may return `Ok(0)` for zeroed or degenerate instruction sysvar data. This is acceptable in Phase 41D3.1 because the result is ordering-only and is not proof, evidence, authorization, replay acceptance, mutation permission, CPI permission, mint permission, or live-route permission.

Downstream 41D3.2 integration must remain fail-closed for `current_index == 0`: there can be no prior instruction at an index `< 0`.

## Theo Verdict

Verdict: ACCEPT

Required fixes: none.

Blocking risks: none.

Theo accepted all reviewed boundary questions:

1. split 41D3.1 is acceptable as a narrow atomic current-index sub-step;
2. scope is limited to checked current-index acquisition;
3. no `load_instruction`, `load_instruction_at`, or `load_instruction_at_checked`;
4. no prior-instruction enumeration;
5. no raw Instructions sysvar byte parsing;
6. no Phase 41C3 candidate descriptors;
7. `load_current_index_checked` is the correct checked boundary helper;
8. missing/wrong account fails closed;
9. checked read failure fails closed;
10. current index is used only for ordering;
11. panic-safety is clean;
12. trust-sensitive flags remain false;
13. blockers remain untouched;
14. next sub-step may start.

## Runtime Boundary Status After Acceptance

| Layer | Status |
| --- | --- |
| AccountInfo presence/readability | Accepted earlier in 41D1 |
| Current instruction identity | Accepted earlier in 41D2 |
| Current index acquisition | Accepted in 41D3.1 |
| Prior instruction enumeration | Still deferred |
| Checked instruction loading | Still deferred |
| Phase 41C3 descriptors | Still deferred |

## Next Sub-Step Allowed

Phase 41D3.2 may start after this acceptance record is merged.

Allowed for 41D3.2:

- prior-instruction enumeration for indexes `< current_index`;
- checked prior-instruction loading via `load_instruction_at_checked`;
- prefiltering unrelated instructions;
- Phase 41C3 candidate descriptor construction;
- explicit same-index reject;
- explicit later-index reject;
- prior-index candidate only;
- `locates_prior_ed25519_instruction: true` flip;
- `load_instruction_called: true` flip if and only if the checked helper is used.

Still forbidden:

- Ed25519 cryptographic verification;
- verification evidence acceptance;
- quorum counting;
- authorization;
- replay writes;
- processed event marking;
- account mutation;
- CPI;
- `invoke_signed`;
- SPL Token `mint_to`;
- runtime handler;
- live route unlock;
- deployment readiness claims.
