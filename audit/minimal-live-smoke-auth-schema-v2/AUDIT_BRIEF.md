# Minimal Live Smoke Guardian Authorization Schema Audit

You do not have local repo access. Use only these GitHub links:

- SOURCE_LINKS.md in this audit folder
- the exact source-file links listed there

Do not inspect the whole repository.

## Boundaries

READ-ONLY AUDIT ONLY.

Do not execute Solana transactions.
Do not create accounts.
Do not run ConsumeGatewayMint.
Do not deploy.
Do not upgrade.
Do not push.
Do not print private keys, seed phrases, or secret key bytes.

## Main goal

Verify or reject whether the minimal live smoke transaction can be built safely without guessing the guardian/ed25519 authorization schema.

## Previous daemon claim to verify against linked source

- signed_message is a 32-byte SHA-256 payload-v2 hash
- domain = "consume_gateway_mint_authorization_v2"
- preimage allegedly includes:
  domain || processed_event(32) || route_id(32) || mint(32) || recipient_token_account(32) || amount(u64 LE) || guardian_set_id(32)
- guardians sign exactly that 32-byte hash via prior ed25519 precompile instructions
- prior ed25519 instructions must appear before ConsumeGatewayMint
- ed25519 instruction data must use self-contained current-instruction sentinel u16::MAX
- live ConsumeGatewayMint uses 12 real accounts, including instructions_sysvar at account index 11, while payload account_meta_count remains 11
- actual mark+mint may require D2/B1C7 dangerous-gated build; default/prod may reject before mutation

## Known smoke constants

Program ID:
D7AQmZNtFFFoJbducz93atteeSZhw3jq6RmsqBvaf1my

Source commit:
6b3a2c6ffa1c7da3b61c0e080fc551ece49d716f

amount_atomic:
1

recipient:
DTfvjtRL63u3XYHXQfgRQCdhEanUK1qqawvfEAM9hxAc

recipient_ata:
9ncbUzU9z98xf3DgQuj2NatneQm4FqRBvQcdH9aUXM3k

target_spl_mint:
g7JQFuKj42NEtyDyYfhW9Wj38DMy7H7yh8mTYNfjwaM

route_id:
d3ddc75b33c427328cdcdd783cc68e447836f8f7456a0d3c810927f1de314e9c

guardian_set_id:
4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83

mint_id:
479f84fd9f7f0c239516a8492cb58b6f8b389d2595f3a48d7e24708f07a5a458

source_chain_id_u64:
1

dummy_burn_hash:
e0c871d52145b2fb50b989259f43a622774c3898361c73dc7f9396b5be90f102

guardian_signature_indices:
[0,1,2]

guardian_quorum:
3

## Known ConsumeGatewayMint payload layout to verify

instruction len = 208

0..8 discriminator
8..10 version
10 account_meta_count = 11
11 route_account_index = 1
12 guardian_set_account_index = 2
13 mint_state_account_index = 0
14 processed_event_account_index = 3
15 recipient_balance_account_index = 4
16..48 route_id
48..80 guardian_set_id
80..112 mint_id
112..144 canonical_event_key
144..176 recipient
176..192 amount u128
194..202 source_chain_id u64
202..208 reserved zeros

## Known likely real account order to verify

0 mint_state
1 gateway_config
2 guardian_set
3 processed_event
4 recipient_balance
5 spl_token_mint
6 recipient_token_account
7 mint_authority_pda
8 token_program
9 rent_payer
10 system_program
11 instructions_sysvar

## Required audit questions

1. Confirm exact active runtime path:
   processor.rs -> process_consume_gateway_mint
   -> b1c7_handler_authorization_boundary
   -> b1c_connect_ed25519_evidence_adapter
   -> b1c_ed25519_evidence_parser
   -> payload binding
   -> b1c_quorum_counting.

2. Confirm exact real account count and account order.

3. Confirm exact payload account_meta_count.

4. Confirm exact function computing expected payload hash.

5. Confirm exact signed message source:
   - canonical_event_key?
   - expected_payload_hash?
   - dummy_burn_hash?
   - another payload hash?

6. Confirm exact signed message bytes for the provided smoke constants if source supports deriving it.

7. Confirm exact ed25519 instruction data requirements:
   - number of prior ed25519 instructions
   - one signature per instruction or multi-signature
   - signature offset
   - public key offset
   - message offset
   - message length
   - sentinel fields
   - position rule relative to ConsumeGatewayMint

8. Confirm whether actual mark+mint requires D2/B1C7 dangerous-gated build, and whether the currently described live smoke would mutate state on the deployed artifact or only validate structure before rejecting.

9. Confirm whether linked files reference guardian private key material. Do not print secrets.

## Required output format

=== audit daemon compact summary for chat ===
AUDIT_PACKAGE=minimal_live_smoke_guardian_authorization_schema_review_v2
READ_ONLY_AUDIT=true

active_runtime_path_confirmed=
consume_payload_layout_confirmed=
real_account_count_required=
instructions_sysvar_required=
payload_account_meta_count=

guardian_authorization_schema_confirmed=
expected_payload_hash_function=
signed_message_source=
signed_message_preimage_schema=
signed_message_len=
signed_message_hex=
canonical_event_key_source=
canonical_event_key_hex=

ed25519_instruction_count=
ed25519_one_signature_per_instruction=
ed25519_signature_offset=
ed25519_public_key_offset=
ed25519_message_offset=
ed25519_message_len=
ed25519_current_instruction_sentinel_required=
prior_ed25519_position_rule=

actual_mark_and_mint_requires_dangerous_gated_build=
current_deployed_artifact_expected_to_mutate_state=
mutation_blocker_if_any=

guardian_key_material_available_without_disclosure=
guardian_key_material_location_type=

ready_for_smoke_execution_builder=
blocker=
recommended_next_step=

transactions_executed=false
consume_gateway_mint_transaction_executed=false
processed_event_mutation_executed=false
deploy_executed=false
upgrade_executed=false
push_executed=false
