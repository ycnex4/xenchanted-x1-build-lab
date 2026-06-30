# XXXL X1 Testnet Local Runtime Skeleton Phase 21 Source-chain ID Binding

Status: Runtime source, Rust test, and checkpoint documentation boundary.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-21-source-chain-id-binding`

## Purpose

Phase 21 promotes the instruction bytes `194..201` from raw reserved bytes to
the runtime instruction field:

- `source_chain_id: u64`
- little-endian encoding
- instruction layout version `2`

Phase 21 binds instruction `source_chain_id` to the existing
`GatewayConfig.source_chain_id()` field.

GatewayConfig account layout is unchanged.

Phase 21 does not enable live route execution.

Phase 21 does not enable SPL CPI execution.

Phase 21 does not enable `invoke_signed`.

Phase 21 does not enable SPL Token `mint_to`.

Phase 21 does not claim production readiness.

Phase 21 does not claim final immutability.

## Instruction Layout

The `consume_gateway_mint` instruction length remains:

- `CONSUME_GATEWAY_MINT_INSTRUCTION_LEN = 208`

The instruction layout version is now:

- `INSTRUCTION_LAYOUT_VERSION = 2`

The Phase 21 layout is:

- `0..7`: discriminator
- `8..9`: layout version `2`
- `10`: account meta count
- `11`: route account index
- `12`: guardian set account index
- `13`: mint state account index
- `14`: processed event account index
- `15`: recipient balance account index
- `16..47`: route id
- `48..79`: guardian set id
- `80..111`: mint id
- `112..143`: canonical event key
- `144..175`: recipient
- `176..191`: amount `u128` little-endian
- `192..193`: source chain weight bps `u16` little-endian
- `194..201`: source chain id `u64` little-endian
- `202..207`: reserved zero bytes

Version `1` instructions are rejected with `InvalidVersion`.

Bytes `194..201` are no longer reserved.

Bytes `202..207` remain reserved and must be zero.

Nonzero bytes in `202..207` reject with `InvalidInstructionReserved`.

## Runtime Binding

Phase 21 adds the processor-side binding:

- instruction `args.source_chain_id`
- existing `GatewayConfigAccountView::source_chain_id()`

If these values differ, the runtime returns:

- `InvalidSourceChain`

This check is performed in `prepare_consume_gateway_mint_cpi_boundary`, where
the instruction args and GatewayConfig account view are both available.

GatewayConfig account layout is not changed.

## Errors

Phase 21 adds these error variants:

- `InvalidSourceChain = 9`
- `InvalidInstructionReserved = 10`

Existing error values are preserved.

## Source-chain Weight

Phase 21 does not move `source_chain_weight_bps`.

The existing dual-source design remains:

- instruction `source_chain_weight_bps`
- GatewayConfig `source_chain_weight_bps`

They must still match.

The instruction code records the follow-up:

- `TODO(Phase 22): Evaluate whether source_chain_weight_bps should remain in
  instruction as part of the signed guardian payload or move to
  GatewayConfig-only. Phase 21 preserves the existing dual-source design.`

## Test Evidence

Instruction tests added or updated:

- `consume_gateway_mint_rejects_version_1`
- `consume_gateway_mint_rejects_version_0`
- `consume_gateway_mint_v2_parses_source_chain_id`
- `consume_gateway_mint_v2_parses_max_source_chain_id`
- `consume_gateway_mint_rejects_nonzero_reserved_202_207`
- `consume_gateway_mint_rejects_nonzero_reserved_any_byte`

Instruction reserved-byte tests now prove:

- bytes `194..201` are semantic `source_chain_id` in v2
- bytes `202..207` remain reserved and reject if nonzero
- version `1` rejects before reserved/source-chain semantics

Runtime/handler tests added:

- `consume_gateway_mint_v2_happy_path_matches_gateway_config`
- `handler_rejects_source_chain_id_mismatch`
- `handler_rejects_source_chain_id_zero`
- `handler_rejects_source_chain_id_unexpected`

Mollusk/integration tests updated or added:

- `mollusk_valid_v2_matching_source_chain_id_leaves_mutable_accounts_unchanged`
- `process_instruction_v2_still_disabled_plan`
- `mollusk_source_chain_id_mismatch_rejection_leaves_mutable_accounts_unchanged`
- `mollusk_nonzero_reserved_202_207_rejects_before_scaffold_path`

Phase 20 runtime-observable matrix was updated to treat sourceChainId final
runtime binding as covered by Phase 21.

## Not Implemented

Phase 21 does not implement:

- proof `emitter_chain_id` binding
- source block or finality runtime fields
- `messageNonce` runtime replay semantics
- guardian signature parsing
- guardian quorum validation
- watcher/model canonical encoding field-order vector validation
- decimal string encoding vectors from the Stage 1 model
- live SPL mint execution success path
- rollback after live SPL CPI failure
- production readiness
- final immutability

## Validation

Commands run:

    cd /mnt/c/Users/user/xenchanted-x1-build-lab/programs/xxxl-svm
    cargo fmt --check
    cargo test --test mollusk_consume_gateway_mint
    cargo test --test disabled_cpi_reachability
    cargo test --test instruction_reserved_bytes
    cargo test --lib

Results:

- `cargo fmt --check`: passed
- `cargo test --test disabled_cpi_reachability`: 7 passed, 0 failed
- `cargo test --test instruction_reserved_bytes`: 3 passed, 0 failed
- `cargo test --lib`: 211 passed, 0 failed, 1 ignored
- `cargo test --test mollusk_consume_gateway_mint`: blocked by stale
  `target/deploy/xxxl_svm.so`

Mollusk failure detail:

- the existing `target/deploy/xxxl_svm.so` rejects v2 instructions with
  `InvalidVersion`
- Phase 21 did not rebuild or replace `.so` artifacts because this task
  explicitly forbids creating `.so` artifacts and touching `target/deploy`

## Correct Statement

Correct:

- instruction layout version is now `2`
- version `1` instructions reject with `InvalidVersion`
- bytes `194..201` are `source_chain_id`
- bytes `202..207` remain reserved and must be zero
- instruction `source_chain_id` must match existing
  GatewayConfig `source_chain_id`
- GatewayConfig layout is unchanged
- live route remains disabled
- SPL CPI remains disabled
- enabled `process_instruction` remains a disabled-plan no-op for live
  atomicity

Incorrect:

- Phase 21 changes GatewayConfig layout
- Phase 21 binds proof `emitter_chain_id`
- Phase 21 implements guardian signature or quorum validation
- Phase 21 implements `messageNonce` replay semantics
- Phase 21 enables live route
- Phase 21 enables SPL CPI
- Phase 21 enables `invoke_signed`
- Phase 21 enables SPL Token `mint_to`
- Phase 21 proves production readiness
- Phase 21 proves final immutability

## Safety Blocker Preservation

Current X1 status remains:

- `X1_TESTNET_PROGRAM_DEPLOYED_RUNTIME_LOCKED`

Active blockers remain:

- `PRODUCTION_PROGRAM_ID_UNSET`
- `LIVE_ROUTE_DISABLED`
- `SPL_CPI_EXECUTION_DISABLED`
- `PRODUCTION_GUARDIAN_SET_UNSET`
- `PRODUCTION_PROOF_LOG_UNSET`
- `EXTERNAL_REVIEW_INCOMPLETE`

Phase 21 did not deploy.

Phase 21 did not upgrade.

Phase 21 did not submit transactions.

Phase 21 did not spend SOL.

Phase 21 did not create `.so` artifacts.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.
