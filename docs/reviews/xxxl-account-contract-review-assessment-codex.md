# XXXL account contract review assessment - Codex

## 1. Summary verdict

The evidence package is sufficient to proceed to the next account contract
review closure boundary, provided that the closure stage remains limited to
reviewing and recording the 9-account `consume_gateway_mint` contract. It is
not sufficient to unlock runtime execution, SPL CPI execution, deployment, mint
readiness, or release readiness.

No blocking account-contract test gap was found under the accepted coverage
policy: lower-level tests may cover account count, account meta count,
writable/readonly flags, signer requirements, owner model classification,
instruction index mapping, and CPI planning-only boundaries, while
security-sensitive account substitution cases are covered at the processor
preparation boundary.

`ACCOUNT_CONTRACT_UNREVIEWED` should remain active until a separate reviewed
closure boundary explicitly accepts the evidence. This assessment does not
remove any blocker.

## 2. Confirmed strengths

- The 9-account contract is stated clearly for `consume_gateway_mint`:
  `mint_state`, `gateway_config`, `guardian_set`, `processed_event`,
  `recipient_balance`, `spl_token_mint`, `recipient_token_account`,
  `mint_authority_pda`, and `token_program`.
- The documented account indexes align with the implementation constants,
  instruction metadata, and contract tests.
- Writable account requirements are consistent with the contract:
  `processed_event`, `recipient_balance`, `spl_token_mint`, and
  `recipient_token_account` are writable; the remaining accounts are readonly.
- The no-external-signer policy is explicit and tested: none of the nine
  accounts is expected to be a transaction signer, including
  `mint_authority_pda`.
- The evidence package correctly separates role classification from runtime
  unlock status. The account contract review evidence is repeatedly described
  as evidence only, not as deploy approval.
- High-priority account-substitution cases are now covered at the processor
  preparation boundary, including wrong account order, wrong program owner,
  wrong SPL Token program id, wrong SPL mint authority, wrong mint authority
  bump, gateway config mismatches, guardian set id mismatch, processed event
  key/route/recipient mismatches, recipient balance owner/mint mismatches, and
  an amount larger than the SPL Token `u64` range.
- Lower-level coverage claims are reasonable for this stage where the rejected
  condition is naturally a static account-contract, instruction-decoding, or
  CPI-planning invariant rather than a full processor account substitution.

## 3. Findings / gaps

### Finding 1: no blocking account-contract test gap found

The current evidence appears complete enough for an account contract review
closure boundary. The strongest reason is that the recent negative tests cover
the substitution cases that matter most at `consume_gateway_mint` processor
preparation, while lower-level tests cover static matrix and instruction
encoding invariants.

No additional evidence/test gap stage appears required before starting the
review closure boundary.

### Finding 2: owner model evidence must not be read as enforcement in
`account_contract.rs` alone

The owner model entries in `account_contract.rs` are useful contract
classification evidence, but the actual owner enforcement is in processor
preparation and SPL/CPI planning checks. This is acceptable for the current
package because the evidence maps owner-sensitive threats to processor tests,
but the closure review should preserve that distinction.

Risk if overclaimed: a reviewer could incorrectly believe that
`assert_consume_gateway_mint_account_contract` enforces program ownership,
SPL ownership, PDA derivation, or SPL Token program identity by itself.

### Finding 3: processed event replay boundary is adequate for the current
locked runtime, but not a live-route proof

The processed event boundary checks the consumed flag, canonical event key,
route id, and recipient before producing the mint CPI plan, and the atomic
composition path retains pre-mutation checks. This is sufficient evidence for
the account-contract closure boundary.

Remaining scope boundary: this does not prove production live-route replay
safety after runtime execution is enabled. That remains blocked by the separate
runtime, live-route, SPL CPI, and Mollusk/SBF coverage gates.

### Finding 4: recipient balance validation is adequately covered for closure

The package covers recipient balance account substitution through direct
processor tests for wrong owner and wrong mint. The processor path also checks
the program-owned account boundary before decoding and planning. No blocking
recipient-balance gap was found for account-contract review closure.

Optional future hardening could add more per-field processor negatives if the
review wants exhaustive account-state mutation evidence, but that would exceed
the current account-contract closure requirement.

### Finding 5: SPL Token account and authority validation is adequately
covered for closure

The token program id, SPL mint account, recipient token account, mint authority
PDA, and mint authority bump are tied to processor-boundary checks and CPI
planning tests. The recent processor tests cover the highest-risk substitution
cases: wrong token program id, wrong SPL mint authority, and wrong mint
authority bump.

Residual boundary: this is still planning/evidence for a locked runtime. It is
not permission to enable SPL CPI execution, `invoke_signed`, or SPL Token
`mint_to`.

### Finding 6: SPL Token `u64` amount boundary is sufficient for closure

The direct processor-boundary test for an amount larger than `u64::MAX` closes
the main SPL Token amount compatibility gap for this stage. Existing zero and
planning checks provide complementary lower-level coverage.

### Finding 7: no dangerous deployability overclaim found

The reviewed docs consistently state that the package is evidence only, that
the runtime remains scaffold-only and locked, and that no blocker is removed.
No claim was found that the runtime is deployable, release-ready,
live-route-ready, mint-ready, or ready to unlock.

## 4. Required fixes before review closure

No blocking test or documentation fix is required before starting the review
closure boundary.

The review closure boundary itself must still do the following:

- Keep `ACCOUNT_CONTRACT_UNREVIEWED` active unless the separate reviewed
  closure explicitly accepts the account contract.
- Avoid any runtime code change.
- Avoid any deployability predicate change.
- Avoid enabling live route execution, SPL CPI execution, `invoke_signed`, or
  SPL Token `mint_to`.
- State that account contract closure, if granted, is not deployment approval
  and not release/unlock approval.

## 5. Optional improvements

- In the closure document, explicitly state that `account_contract.rs` owner
  model entries are classification evidence, while owner enforcement is
  validated through processor preparation and SPL/CPI boundary tests.
- Add a short trace table from each high-priority substitution threat to the
  exact processor test name, so reviewers do not need to infer coverage from
  surrounding prose.
- Consider future direct processor negatives for uninitialized SPL mint,
  uninitialized recipient token account, or wrong recipient token account owner
  if the closure reviewer wants more SPL account-state evidence. These do not
  appear required for account-contract closure under the current accepted
  coverage policy.
- Keep Mollusk/SBF live-execution coverage as a separate blocker. It should not
  be folded into account contract closure.

## 6. Safety confirmation

- No Rust source change is required or recommended by this assessment.
- No runtime behavior should change.
- `deployment_status.rs` and `safety_invariants.rs` should remain unchanged.
- `ACCOUNT_CONTRACT_UNREVIEWED` should remain active until a separate reviewed
  closure boundary explicitly accepts the account contract.
- Live route execution must remain disabled.
- SPL CPI execution must remain disabled.
- `invoke_signed` must remain disabled.
- SPL Token `mint_to` must remain disabled.
- Program ID must not change.
- Deployability predicates must not change.
- Runtime deployment status must not change.

## 7. Recommendation

Proceed to review closure boundary.

Do not run another evidence/test gap stage first unless the closure reviewer
adds a new requirement beyond the accepted lower-level coverage policy.
