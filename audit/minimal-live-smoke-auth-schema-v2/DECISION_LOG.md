# Minimal Live Smoke — Decision Log

Last updated: 2026-08-14T20:08Z

This log records operational decisions for the `minimal-live-smoke-auth-schema-v2` audit package.

## D-001 — Use GitHub audit branch as compact source of truth

Decision:

```text
Keep code, scripts, public constants, short evidence summaries, and decision records in GitHub.
Keep raw evidence and secrets outside GitHub.
```

Reason:

```text
GitHub files can be fetched and cited precisely.
Long terminal output is noisy and error-prone.
Chat history alone is not a stable source of truth.
```

Secret-handling boundary:

```text
Do not commit payer keypair, guardian keypairs, seed phrases, .env with secret paths, or raw private evidence.
Public keys, PDAs, program IDs, tx signatures, hashes, and sanitized summaries are allowed.
```

## D-002 — Preflight passed; execution was allowed only for Option 1 structure-only smoke

Decision:

```text
Run Option 1 structure-only smoke after preflight showed blocker=none.
```

Preflight evidence:

```text
evidence_dir=/home/sergey/xenchanted-stage20-activation-evidence-c332814/runtime-state-provisioning-minimal-live-smoke-option1-builder-preflight-no-execution-20260814T191742Z
ready_for_option1_execution_builder=true
blocker=none
prestate_zero=true
accounts_ready=true
guardian_keypairs_ready=true
```

Expected outcome:

```text
transaction lands
ed25519 authorization structure is valid
current default artifact reaches CpiBoundaryNotReady=8
no state mutation
```

## D-003 — First execution attempt failed safely but did not reach expected boundary

Decision:

```text
Do not treat first attempt as success.
Inspect logs and source before retrying.
```

Result:

```text
tx_signature=51PCJG925dU7Re94asZ6dL2WurN9w49afSUDAEB3YoN8KjaPyE49QknrJ4bA2sjmjZuaySkDYc2zbgFQk9yaRjuG
tx_landed=true
custom_error_code=1
expected_custom_error_code=8
state_and_token_accounts_unchanged=true
```

Initial diagnosis:

```text
Execution script used mint_state as writable while B1 V3 account contract requires mint_state readonly.
```

## D-004 — V2 execution corrected account flags but still failed safely with InvalidInstruction

Decision:

```text
Do not treat v2 attempt as success.
Do not continue with repeated on-chain retries.
Move to no-execution transaction diagnosis.
```

Result:

```text
tx_signature=3RUVE5AVDnSnnZp3hXq4Xnb1vTw9ZHUHjfAGDyuGA67MzhYtF2TwCR7wKTQpgjk6Hs1nwToTjfmhDxBoWF2g9DmN
tx_landed=true
custom_error_code=1
expected_custom_error_code=8
state_and_token_accounts_unchanged=true
supply_and_balance_values_unchanged=true
processed_event_exists_after=false
```

## D-005 — Diagnose-only inspection identifies mint field mismatch as current leading hypothesis

Decision:

```text
Use diagnose-only tooling before any third execution.
```

Diagnosis evidence:

```text
evidence_dir=/home/sergey/xenchanted-stage20-activation-evidence-c332814/runtime-state-provisioning-minimal-live-smoke-option1-diagnose-transaction-no-execution-20260814T200028Z
transactions_executed=false
account_contract_flags_match=true
account_contract_mismatches=[]
consume_data_layout_ok=true
mint_field_equals_legacy_mint_id=true
mint_field_equals_target_spl_mint=false
```

Working hypothesis:

```text
The consume instruction field at bytes 80..112 must likely be target_spl_mint, not legacy_mint_id, for the current runtime planning path.
Reason: execution_plan.mint is sourced from args.mint_id, and CPI planning compares execution_plan.mint with the real SPL mint account key.
```

## D-006 — Current next step is v3 candidate no-execution, not execution

Decision:

```text
Add a v3 candidate no-execution script.
Do not send another transaction until the candidate transaction bytes are validated offline.
```

v3 candidate requirements:

```text
transactions_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false
mint_field=target_spl_mint
signed_message schema unchanged
signed_message mint component remains target_spl_mint
recipient token account component remains recipient_ata
ed25519 instructions remain prior to consume
consume instruction index remains 3
real account count remains 12
instructions sysvar index remains 11
account contract flags must match
prestate assumptions must be checked
candidate summary must clearly state whether it is ready for a possible third execution
```

Stop conditions:

```text
Any mismatch in account metas, layout, signatures, public constants, or prestate blocks execution.
Any uncertainty about the required mint field blocks execution and should be escalated for review.
```
