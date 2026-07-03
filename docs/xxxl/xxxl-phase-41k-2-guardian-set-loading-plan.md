# XXXL Phase 41K.2 — Real Guardian-Set Account/PDA Loading Plan

Date: 2026-07-03

Status: draft plan

Branch:

`stage-xxxl-x1-testnet-local-runtime-skeleton-phase-41k-2-guardian-set-loading-plan`

Base checkpoint:

`20d65a8 Merge XXXL phase 41K.1 instructions sysvar implementation acceptance`

## Purpose

Phase 41K.2 is the second sub-gate of the accepted 41K live-wiring master plan.

41K.1 accepted the first real runtime-loading surface:

`real Instructions sysvar -> checked current index -> checked prior instruction loading -> N prior Ed25519 precompile enumeration`

41K.2 plans the next real runtime-loading surface:

`real guardian-set account/PDA -> checked account identity -> checked account data decode -> authoritative guardian-set wrapper`

The purpose is to replace the remaining abstract guardian-set source with a real program-controlled on-chain account source.

## Accepted Input Boundary Before 41K.2

Before 41K.2, 41H/41I can validate guardian membership and quorum only if an authoritative guardian-set reference is already supplied by an internal boundary.

41H already rejects:

- caller-supplied guardian sets;
- unauthenticated guardian sets;
- empty guardian sets;
- invalid thresholds;
- duplicate guardian public keys;
- payload guardian-set id mismatch.

41K.2 must preserve this boundary.

41K.2 must not re-open guardian-set data as caller instruction data.

## Runtime Source Rule

The only valid source for guardian-set membership in 41K.2 is a real program-controlled guardian-set account/PDA.

Invalid sources:

- guardian list supplied in instruction data;
- guardian list supplied by frontend;
- guardian list supplied by watcher;
- guardian list supplied by relayer;
- arbitrary account pretending to be guardian-set account;
- account owned by an unexpected program;
- account with malformed data;
- account with unsupported schema version;
- account with mismatching guardian-set id;
- account with invalid threshold;
- account with duplicate guardian public keys.

## Guardian-Set Account Identity Rule

41K.2 implementation must verify account identity before trusting data.

The plan requires:

- the guardian-set account key must match the expected PDA derivation;
- the guardian-set PDA derivation must be deterministic;
- the PDA derivation must include the guardian-set id or a clearly defined active-set seed;
- the PDA bump must be checked if stored or required;
- the account owner must be the expected XXXL program id / runtime authority;
- arbitrary accounts must fail closed before deserialization is trusted;
- account identity checks must happen before constructing an authoritative guardian-set wrapper.

Open design detail for implementation review:

- exact seed format must be fixed before 41K.2 code acceptance.

Expected seed model options:

1. `guardian-set-by-id`:

   `["xxxl", "guardian-set", guardian_set_id]`

2. `active-guardian-set` singleton:

   `["xxxl", "guardian-set", "active"]`

3. hybrid model:

   active pointer account + guardian-set-by-id account.

41K.2 plan prefers option 1 for deterministic simplicity unless review requires an active pointer model.

## Guardian-Set Data Shape

41K.2 implementation must define and validate a minimal guardian-set account schema.

Required decoded fields:

- discriminator / account kind;
- schema version;
- guardian_set_id `[u8; 32]`;
- threshold;
- guardian_count;
- guardian public keys;
- active flag or equivalent status;
- optional bump if used by PDA verification.

Required validation:

- account kind must be guardian-set;
- schema version must be supported;
- guardian_set_id must match expected id / PDA seed;
- threshold must be greater than zero;
- threshold must not exceed guardian_count;
- guardian_count must be greater than zero;
- guardian_count must not exceed the maximum supported guardian count;
- guardian public keys must be exactly 32 bytes each;
- duplicate guardian public keys must be rejected;
- inactive guardian set must be rejected unless a future rotation rule explicitly allows it;
- trailing or malformed bytes must be handled deterministically.

## Authoritative Wrapper Rule

41K.2 must produce only an internal authoritative guardian-set reference.

The accepted output must be equivalent to:

`AuthoritativeGuardianSetSource::ProgramControlledOnChain`

The implementation must not expose a public unrestricted constructor for production account data.

The loaded account may feed later 41H/41I only through the internal authoritative wrapper.

Forbidden:

- constructing `CallerInstructionData` guardian-set source from account-like bytes;
- accepting unauthenticated guardian-set source;
- accepting frontend/watcher/relayer guardian-set source as authority;
- passing decoded guardian data directly into quorum without the authoritative wrapper.

## Payload Binding Rule

41K.2 itself does not decode the gateway payload and does not authorize mint.

However, it must preserve the data necessary for later binding:

- loaded guardian_set_id must be available to 41H;
- loaded guardian_set_id must be checked by 41H against decoded payload guardian_set_id;
- no free decoded payload input is accepted by 41K.2;
- no payload-derived guardian-set id may be trusted unless it comes through the accepted raw-payload decode path.

41K.2 does not replace 41H guardian-set id binding.

It supplies the real on-chain guardian-set source that 41H can bind against payload data.

## Rotation / Active-Set Rule

41K.2 must not silently accept rotated, deprecated, or inactive sets.

Minimum rule for this phase:

- only active guardian-set account may be accepted;
- inactive / deprecated flag must reject;
- future rotation logic must be a separate reviewed gate if not fully defined in 41K.2.

41K.2 does not implement governance, signer rotation, or guardian-set update instructions.

## Panic-Safety Rule

41K.2 implementation must be panic-safe.

Required:

- no unchecked indexing;
- no unchecked slicing;
- no `unwrap`;
- no `expect`;
- no unchecked length assumptions;
- all account-data parsing must use checked bounds;
- malformed account data must return deterministic failure;
- oversized account data must not cause unbounded allocation;
- guardian count must be bounded before allocation / iteration;
- threshold validation must happen before authorization composition.

## Output of 41K.2

41K.2 should output a boundary result containing:

- status;
- rejection case;
- guardian_set_id;
- threshold;
- guardian_count;
- guardian public keys;
- account key;
- owner check result;
- PDA check result;
- schema version check result;
- duplicate check result;
- active status check result;
- source marker: program-controlled on-chain;
- flags showing no later runtime surfaces enabled.

The output is not authorization.

It is only the real guardian-set loading boundary.

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

## Safety Flags Required

Any 41K.2 implementation report must keep these false:

- `processed_registry_runtime_loading_enabled`;
- `replay_write_enabled`;
- `processed_event_marking_enabled`;
- `account_mutation_enabled`;
- `cpi_enabled`;
- `invoke_signed_enabled`;
- `spl_token_mint_to_enabled`;
- `process_instruction_handler_added`;
- `live_route_enabled`.

41K.2 may plan to enable only this new surface:

- `guardian_set_runtime_loading_enabled: true`

Implementation must separately show:

- `guardian_set_account_key_checked: true`;
- `guardian_set_account_owner_checked: true`;
- `guardian_set_pda_checked: true`;
- `guardian_set_schema_checked: true`;
- `guardian_set_threshold_checked: true`;
- `guardian_set_duplicates_rejected: true`;
- `caller_supplied_guardian_set_rejected: true`.

## Deployment Blockers

41K.2 plan does not remove deployment blockers.

The following blockers remain active:

- `PRODUCTION_PROGRAM_ID_UNSET`;
- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`;
- `PRODUCTION_GUARDIAN_SET_UNSET`;
- `PRODUCTION_PROOF_LOG_UNSET`;
- `SPL_CPI_EXECUTION_DISABLED`;
- `LIVE_ROUTE_DISABLED`;
- `EXTERNAL_REVIEW_INCOMPLETE`.

41K.2 directly addresses the design side of:

`PRODUCTION_GUARDIAN_SET_UNSET`

but the blocker remains until a reviewed production guardian-set account / PDA and deployment policy exist.

## Forbidden Design Patterns

41K.2 must reject any design that:

- accepts guardian list from caller instruction data;
- accepts guardian list from frontend/watcher/relayer as authority;
- accepts arbitrary account as guardian-set account;
- skips PDA verification;
- skips owner verification;
- skips schema/discriminator verification;
- skips threshold validation;
- skips duplicate guardian key rejection;
- accepts inactive guardian set silently;
- constructs authoritative wrapper before account identity is checked;
- authorizes quorum directly from raw decoded account bytes without wrapper;
- enables registry writes;
- enables mint/CPI;
- enables process instruction handler;
- enables live route.

## Review Questions

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

## Review Notes Incorporated

The following review notes are incorporated into the 41K.2 plan before implementation.

### Exact PDA Seed Format Must Be Fixed Before Code Acceptance

41K.2 code acceptance must fix the exact guardian-set PDA seed format.

The implementation review must confirm:

- the guardian-set account is program-derived;
- the PDA is derived under the expected XXXL program id / runtime authority;
- the guardian-set account owner is the expected XXXL program id;
- the exact seed bytes are documented and tested;
- the PDA bump is checked if used or stored;
- the stored guardian_set_id matches the guardian_set_id implied by the PDA seed.

For the preferred by-id model, this means:

`stored guardian_set_id == seed guardian_set_id`

This prevents guardian-set id / PDA desynchronization.

A guardian-set account must not be accepted if the PDA address implies one guardian_set_id but the stored account data contains another guardian_set_id.

### Read-Only Account Requirement for Handler Wiring

41K.2 is a loading boundary only.

It does not require signer authority and does not require write authority.

The future 41K.5 handler must pass the guardian-set account as:

- read-only;
- non-writable;
- non-signer.

Any future write/update path for guardian-set accounts must be a separate reviewed governance or rotation gate.

41K.2 must not introduce or rely on a writable guardian-set account path.

### Uninitialized / Zero-Discriminator Rejection

41K.2 code must explicitly reject uninitialized guardian-set accounts.

Required rejection cases include:

- all-zero discriminator;
- default discriminator;
- missing discriminator;
- wrong discriminator;
- account kind not equal to guardian-set;
- zeroed program-owned account with otherwise readable data.

This is a defense-in-depth rule against type confusion and accidental acceptance of uninitialized program-owned accounts.

Schema, threshold, count, and active-status checks remain required, but discriminator rejection must be explicit.

## Current Plan Status

This is a docs-only plan.

No Rust code is changed.

No guardian-set account loading is enabled yet.

No deployment blocker is removed.
