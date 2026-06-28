# XXXL Account Contract Review Evidence Package

Status: EVIDENCE PACKAGE ONLY.

Review status: not reviewed yet.

No blocker is removed by this stage.

This package is evidence only.

## 1. Purpose and scope

This document gathers the current evidence for future review of the XXXL SVM
`consume_gateway_mint` account contract.

It collects:

- the account contract being reviewed
- the accounts used by the instruction
- writable and readonly policy
- ownership and PDA expectations
- account substitution threats
- test evidence for protected invariants
- lower-level coverage already available
- validation commands that should remain evidence for this stage
- remaining blockers and reviewer questions

This document is not an approval.

This document does not remove `ACCOUNT_CONTRACT_UNREVIEWED`.

This document does not make the runtime deployable.

This document does not unlock live route execution, SPL CPI execution,
`invoke_signed`, or SPL Token `mint_to`.

## 2. Current runtime status

Current runtime status:

- runtime scaffold-only
- locked
- unreleasable
- not deployable
- `ACCOUNT_CONTRACT_UNREVIEWED` remains active

The release decision remains blocked by `RUNTIME_SAFETY_LOCK_ACTIVE`.

## 3. `consume_gateway_mint` account list

The current `consume_gateway_mint` account contract has 9 accounts:

| Index | Account |
| --- | --- |
| 0 | `mint_state` |
| 1 | `gateway_config` |
| 2 | `guardian_set` |
| 3 | `processed_event` |
| 4 | `recipient_balance` |
| 5 | `spl_token_mint` |
| 6 | `recipient_token_account` |
| 7 | `mint_authority_pda` |
| 8 | `token_program` |

The current instruction account meta count is also 9.

The runtime must reject an instruction if the caller-supplied account list or
encoded account meta count does not match the expected contract.

## 4. Writable / readonly policy

Writable accounts:

- `processed_event`
- `recipient_balance`
- `spl_token_mint`
- `recipient_token_account`

Readonly accounts:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `mint_authority_pda`
- `token_program`

Writable does not mean currently production-mutated. The runtime remains
locked, live route execution remains disabled, and SPL CPI execution remains
disabled.

## 5. Ownership / PDA model

Program-owned accounts:

- `mint_state`
- `gateway_config`
- `guardian_set`
- `processed_event`
- `recipient_balance`

SPL Token owned accounts:

- `spl_token_mint`
- `recipient_token_account`

PDA account:

- `mint_authority_pda`

SPL Token program account:

- `token_program`

External signer accounts:

- none

The mint authority PDA is not an external signer. Any future `invoke_signed`
path must remain disabled until a separate reviewed boundary explicitly allows
it.

## 6. Account substitution threat model

The review evidence package tracks the following account substitution and
binding threats:

- wrong account order
- wrong program-owned account owner
- wrong SPL Token program id
- wrong SPL Token mint authority
- wrong mint authority PDA bump
- wrong gateway config guardian set id
- wrong gateway config target mint
- wrong source chain weight
- wrong guardian set id
- wrong processed event canonical event key
- wrong processed event route id
- wrong processed event recipient
- wrong recipient balance owner
- wrong recipient balance mint
- amount larger than SPL Token `u64` range

These threats are relevant because caller-supplied accounts must be treated as
untrusted until all account contract checks pass.

## 7. Evidence table

The latest test gap closure stage added direct processor-boundary negative tests
for the high-risk account substitution and binding cases below.

| Threat or invariant | Evidence test(s) | Evidence boundary |
| --- | --- | --- |
| wrong account order | `handler_integration_rejects_wrong_account_order` | processor preparation |
| wrong program-owned account owner | `handler_integration_rejects_wrong_program_owner_for_program_owned_account` | processor preparation |
| wrong SPL Token program id | `handler_integration_rejects_wrong_spl_token_program_id` | processor preparation |
| wrong SPL Token mint authority | `handler_integration_rejects_wrong_spl_mint_authority` | processor preparation |
| wrong mint authority bump | `handler_integration_rejects_wrong_mint_authority_bump` | processor preparation |
| wrong gateway config guardian set id | `handler_integration_rejects_gateway_config_guardian_set_id_mismatch` | processor preparation |
| wrong gateway config target mint | `handler_integration_rejects_gateway_config_target_mint_mismatch` | processor preparation |
| wrong gateway config source chain weight | `handler_integration_rejects_gateway_config_source_chain_weight_mismatch` | processor preparation |
| wrong guardian set id | `handler_integration_rejects_wrong_guardian_set_id` | processor preparation |
| wrong processed event canonical event key | `handler_integration_rejects_wrong_processed_event_canonical_event_key` | processor preparation |
| wrong processed event route id | `handler_integration_rejects_wrong_processed_event_route_id` | processor preparation |
| wrong processed event recipient | `handler_integration_rejects_wrong_processed_event_recipient` | processor preparation |
| wrong recipient balance owner | `handler_integration_rejects_wrong_recipient_balance_owner` | processor preparation |
| wrong recipient balance mint | `handler_integration_rejects_wrong_recipient_balance_mint` | processor preparation |
| amount larger than SPL Token `u64` range | `handler_integration_rejects_amount_larger_than_spl_token_u64_range` | processor preparation |

The latest stage also preserved existing direct processor integration coverage
for:

- `handler_integration_rejects_wrong_account_count`
- `handler_integration_rejects_gateway_route_mismatch`
- `handler_integration_rejects_consumed_processed_event`
- `handler_integration_rejects_wrong_mint_authority_pda`
- `handler_integration_rejects_wrong_spl_mint_owner`
- `handler_integration_rejects_wrong_recipient_token_mint`
- `handler_integration_rejects_zero_amount`

## 8. Existing lower-level coverage

The following items are already covered by existing lower-level tests:

| Item | Existing evidence |
| --- | --- |
| account count | `handler_integration_rejects_wrong_account_count` |
| encoded account meta count | `consume_gateway_mint_rejects_wrong_account_meta_count` |
| index mapping | `consume_gateway_mint_account_contract_matches_processor_indices`, `consume_gateway_mint_account_contract_matches_instruction_indices`, `consume_gateway_mint_rejects_wrong_account_index_boundary` |
| writable/readonly flags | `consume_gateway_mint_account_contract_marks_only_mutable_accounts_writable` |
| required writable / readonly constraints | `runtime_account_contract_rejects_missing_required_writable_account`, `runtime_account_contract_rejects_unnecessary_writable_readonly_account` |
| unexpected signer | `runtime_account_contract_rejects_unexpected_external_signer` |
| owner model classification | `consume_gateway_mint_account_contract_documents_owner_models` |
| wrong recipient token account / owner / mint | `handler_integration_rejects_wrong_recipient_token_mint`, `runtime_planning_composition_boundary_rejects_wrong_recipient_token_account_without_mutation`, `runtime_disabled_spl_cpi_gate_boundary_rejects_wrong_recipient_token_account_without_mutation` |
| wrong gateway config route id | `handler_integration_rejects_gateway_route_mismatch` |
| already consumed processed event | `handler_integration_rejects_consumed_processed_event`, `runtime_planning_composition_boundary_rejects_consumed_event_without_mutation` |
| zero amount | `handler_integration_rejects_zero_amount`, `runtime_planning_composition_boundary_rejects_zero_amount_without_mutation` |
| CPI planning-only rejection boundaries | `mint_to_cpi_planning_boundary_rejects_wrong_token_program`, `mint_to_cpi_planning_boundary_rejects_wrong_mint_mapping`, `mint_to_cpi_planning_boundary_rejects_wrong_pda`, `mint_to_cpi_planning_boundary_rejects_wrong_bump`, `mint_to_cpi_planning_boundary_rejects_amount_mismatch`, `mint_to_cpi_planning_boundary_rejects_zero_boundary_amount`, `guarded_mint_to_cpi_execution_gate_boundary_rejects_when_gate_disabled` |

This evidence does not itself clear the account contract blocker. It exists so
a future reviewer can decide whether the account contract is ready for a
separate reviewed transition stage.

## 9. Validation evidence

The following commands should remain evidence for this stage:

```bash
cargo fmt --check
cargo test account_contract --lib
cargo test instruction --lib
cargo test processor --lib
cargo test cpi --lib
cargo test deployment_status --lib
cargo test safety_invariant --lib
cargo test --lib
git diff --check
```

Run cargo commands from:

```bash
cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm
```

Run git commands from:

```bash
cd /mnt/c/Users/user/xenchanted-x1-build-lab
```

## 10. Safety non-changes

This stage records the following non-changes:

- no live route execution was enabled
- no SPL CPI execution was enabled
- no `invoke_signed` path was enabled
- no SPL Token `mint_to` path was enabled
- no Program ID was changed
- no production PDA fixtures were regenerated
- no deployability predicates were changed
- no runtime deployment status was changed
- `ACCOUNT_CONTRACT_UNREVIEWED` remains active

The runtime remains scaffold-only, locked, unreleasable, and not deployable.

## 11. Reviewer questions

Reviewer questions:

1. Is the 9-account contract sufficient?
2. Are owner, writable, readonly, and signer constraints sufficient?
3. Is the `processed_event` replay boundary sufficient?
4. Is `recipient_balance` validation sufficient?
5. Is the mint authority PDA boundary sufficient before future `invoke_signed` planning?
6. Is the `u64` SPL Token amount boundary sufficient?
7. Are any additional processor-boundary tests required before account contract review closure?
8. Is anything missing before `ACCOUNT_CONTRACT_UNREVIEWED` can be considered for a separate transition stage?

## 12. Review outcome placeholder

- Review status: not reviewed yet.
- No blocker removed in this stage.
- This package is evidence only.

Future review may use this package as input. That future review must remain a
separate boundary and must not silently become a runtime unlock.
