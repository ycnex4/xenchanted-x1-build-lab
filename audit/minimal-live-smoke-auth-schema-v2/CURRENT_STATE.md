# Minimal Live Smoke — Current State

Last updated: 2026-08-14T20:08Z

## Scope

This file is the short operational source of truth for the `minimal-live-smoke-auth-schema-v2` audit package.

Branch:

```text
audit/minimal-live-smoke-auth-schema-v2-20260814T165358Z
```

Repository:

```text
ycnex4/xenchanted-x1-build-lab
```

Runtime source commit under audit:

```text
6b3a2c6ffa1c7da3b61c0e080fc551ece49d716f
```

## Boundaries

Approved scope is testnet-only minimal live smoke / structure-only verification.

Do not do any of the following without a separate explicit approval:

```text
deploy
upgrade
push
mainnet execution
D2/B1C7 dangerous-gated build deployment
state-mutating mark+mint smoke
raw keypair / seed / secret publication
```

Secrets are not part of this audit package. Guardian private keypairs and payer keypair must remain local only and must not be committed.

## Program and public constants

```text
program_id=D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my
upgrade_authority=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
mint_authority_pda=BLVsQPYXnDsTmfMW9wrXHBFpcmexM47BcAvVcibRtRYG
spl_token_program=TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
target_spl_mint=g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM
recipient_owner=DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc
recipient_ata=9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k
```

State PDAs:

```text
gateway_config_pda=G79vy8cNu4eLoK42Aj15KJ7cW9n27DQxFkfYf8qcb67D
guardian_set_pda=4yNzJ6cB6ecAovH2e12p2SC54WUio7HbThLqSRPMwuba
mint_state_pda=57GckP3TXGQmyuFh6KcHqhL7NbsXuRdwW741FaJQtfQG
recipient_balance_pda=5YtuhQQJRBCi3Z2W25s2VUnX22hxXsu2o4ikHXmUT1MB
```

Guardian set:

```text
guardian_set_id=4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83
quorum=3
guardian0=7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf
guardian1=GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp
guardian2=6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih
guardian3=9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY
guardian4=UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg
```

Route / smoke constants:

```text
route_id=d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c
legacy_mint_id=479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458
source_chain_id_u64=1
source_chain_weight_bps=10000
amount_atomic=1
canonical_event_key=e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102
processed_event_pda=7X6393SVpagX3mNpFDLYy51EedUY5AF6SrRfJYSbphi9
signed_message_hex=02959e687c74756661cac356cc65bdff184df5810437a01558144c45384e2823
```

Signed-message schema confirmed for this package:

```text
sha256(domain || processed_event || route_id || mint || recipient_token_account || amount_u64_le || guardian_set_id)
domain="consume_gateway_mint_authorization_v2"
mint=target_spl_mint
recipient_token_account=recipient_ata
```

## Preflight status

Latest successful preflight:

```text
evidence_dir=/home/sergey/xenchanted-stage20-activation-evidence-c332814/runtime-state-provisioning-minimal-live-smoke-option1-builder-preflight-no-execution-20260814T191742Z
transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false
guardian_keypairs_ready=true
expected_guardians_present_onchain=true
pda_matches_known_constants=true
accounts_ready=true
prestate_zero=true
ready_for_option1_execution_builder=true
blocker=none
```

Prestate at that point:

```text
spl_mint_supply_before=0
recipient_ata_amount_before=0
mint_state_total_supply_before=0
recipient_balance_amount_before=0
processed_event_exists_before=false
```

## Executions performed

Two structure-only execution attempts were sent on X1 testnet. Both landed and both failed safely with no state mutation.

Attempt 1:

```text
tx_signature=51PCJG925dU7Re94asZ6dL2WurN9w49afSUDAEB3YoN8KjaPyE49QknrJ4bA2sjmjZuaySkDYc2zbgFQk9yaRjuG
result=InvalidInstruction
custom_error_code=1
state_and_token_accounts_unchanged=true
supply_and_balance_values_unchanged=true
processed_event_exists_after=false
```

Attempt 2:

```text
tx_signature=3RUVE5AVDnSnnZp3hXq4Xnb1vTw9ZHUHjfAGDyuGA67MzhYtF2TwCR7wKTQpgjk6Hs1nwToTjfmhDxBoWF2g9DmN
result=InvalidInstruction
custom_error_code=1
state_and_token_accounts_unchanged=true
supply_and_balance_values_unchanged=true
processed_event_exists_after=false
```

Expected Option 1 success condition was not reached yet:

```text
expected_failure=CpiBoundaryNotReady
expected_custom_error_code=8
actual_custom_error_code=1
option1_success=false
```

## Diagnosis status

Diagnose-only transaction inspection was run on the second transaction bytes.

```text
evidence_dir=/home/sergey/xenchanted-stage20-activation-evidence-c332814/runtime-state-provisioning-minimal-live-smoke-option1-diagnose-transaction-no-execution-20260814T200028Z
transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false
instruction_count=4
ed25519_instruction_count_assuming_consume_index_3=3
consume_instruction_index=3
consume_program_matches=true
consume_key_count=12
account_contract_flags_match=true
account_contract_mismatches=[]
consume_data_layout_ok=true
route_id_matches=true
guardian_set_id_matches=true
canonical_event_key_matches_dummy_burn_hash=true
recipient_field_matches_recipient_owner=true
mint_field_equals_legacy_mint_id=true
mint_field_equals_target_spl_mint=false
```

Current likely cause:

```text
consume instruction data currently uses legacy_mint_id at bytes 80..112.
Runtime execution plan uses args.mint_id as execution_plan.mint.
CPI planning compares execution_plan.mint with the real SPL mint account key.
Therefore legacy_mint_id != target_spl_mint can explain InvalidInstruction=1 before CpiBoundaryNotReady.
```

Important correction:

```text
The diagnose script field likely_current_failure=not_account_contract_flags is stale/incorrect for v2 bytes, because account_contract_flags_match=true and account_contract_mismatches=[] in the same output.
```

## Current decision

Do not run a third on-chain execution yet.

Next safe step:

```text
Add v3 candidate no-execution script.
It should build candidate transaction bytes with mint_field = target_spl_mint.
It must verify layout, signatures, account metas, payload hash, prestate assumptions, and expected error target.
It must not send any transaction.
```

Only after v3 candidate no-execution is clean should another on-chain Option 1 execution be considered.
