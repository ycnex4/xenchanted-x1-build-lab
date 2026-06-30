# XXXL X1 Testnet Local Runtime Skeleton Phase 17 Source Chain ID Runtime Boundary

Status: Docs-only runtime boundary checkpoint - all runtime blockers remain active.

Branch:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-17-source-chain-id-runtime-boundary`

## Purpose

Phase 17 records the current `sourceChainId` runtime boundary.

The current SVM instruction layout does not expose `sourceChainId` as a parsed
runtime field.

The current instruction parser stores the full raw instruction bytes, but the
typed `ConsumeGatewayMintArgs` structure does not contain a `sourceChainId`
field.

Therefore, the runtime must not claim independent source-chain ID binding.

## Scope

Phase 17 is docs-only.

Allowed changes:

- checkpoint documentation only

Not modified:

- `programs/xxxl-svm/src/**`
- `programs/xxxl-svm/tests/**`
- `programs/xxxl-svm/Cargo.toml`
- workspace-level Cargo files
- deployment scripts
- upgrade scripts
- CI/CD workflows that deploy, upgrade, submit transactions, or spend SOL
- `.local-keys/**`
- keypair JSON files
- `.env`
- `target/deploy/**`
- `.so` artifacts

## Current Instruction Layout Boundary

Current consume-gateway-mint instruction length:

- `208` bytes

Currently parsed typed fields:

- discriminator
- instruction layout version
- account meta count
- route account index
- guardian set account index
- mint state account index
- processed event account index
- recipient balance account index
- `route_id`
- `guardian_set_id`
- `mint_id`
- `canonical_event_key`
- `recipient`
- `amount`
- `source_chain_weight_bps`

The typed args do not include:

- `sourceChainId`
- `source_chain_id`
- `burnedAmount`
- explicit source-token address
- explicit source-block binding fields

## Bytes 194..208 Boundary

The current parser reads:

- `amount` from bytes `176..192`
- `source_chain_weight_bps` from bytes `192..194`

Bytes `194..208` remain:

- present in `raw`
- not exposed as typed fields
- not parsed as `sourceChainId`
- not zero-validated
- not semantically validated

Therefore, bytes `194..208` must not be silently treated as `sourceChainId`.

## Correct Statement

Correct:

- current runtime does not independently parse `sourceChainId`
- current runtime stores raw instruction bytes
- current runtime parses `source_chain_weight_bps`
- bytes `194..208` remain reserved / unparsed / not zero-validated
- route-level and canonical-event-level context may reflect Stage 1 source
  context, but runtime does not independently bind a parsed `sourceChainId`

Incorrect:

- runtime already parses `sourceChainId`
- bytes `194..208` are already `sourceChainId`
- current runtime independently validates source chain identity
- current runtime source-chain binding is production-ready
- current runtime can safely enable live route without resolving source-chain ID
  semantics

## Stage 1 / Runtime Boundary

Stage 1 message schema includes source-chain context.

The current runtime skeleton does not directly reproduce every Stage 1 field as
a parsed SVM instruction field.

This is acceptable only while live route remains disabled.

Before live-route or SPL-CPI enablement, source-chain ID handling must be
explicitly resolved.

Resolution may require one of the following future choices:

1. add an explicit parsed `sourceChainId` field to the runtime instruction
   layout and validate it against route config
2. define route config as the exclusive runtime source-chain binding and
   document that source-chain ID is bound through `route_id`
3. use another explicit verified binding mechanism

No such resolution is completed in Phase 17.

## Relationship To route_id

`route_id` is parsed by the runtime.

`route_id` may represent route-level source/destination context.

However, current runtime code must not claim that `route_id` is the same thing
as an independently parsed `sourceChainId`.

If future design chooses route-level binding, the invariant must be documented
and tested before live-route activation.

## Relationship To canonicalEventKey

`canonicalEventKey` is parsed by the runtime.

`canonicalEventKey` may reflect Stage 1 event identity.

However, current runtime code must not claim that `canonicalEventKey` is the
same thing as an independently parsed `sourceChainId`.

If future design relies on `canonicalEventKey` to include source-chain identity,
that invariant belongs to Stage 1 / watcher / authorization boundary and must be
documented as such.

## Current Disabled Semantics Preserved

Current `Ok(())` still means:

- validation succeeded
- disabled execution plan was built
- no live gateway success
- no XXXL mint success
- no Processed Event consumption
- no Recipient Balance credit
- no Mint State / supply accounting mutation
- no SPL CPI
- no `invoke_signed`
- no SPL Token `mint_to`

## What Phase 17 Proves

Phase 17 proves at documentation / source-boundary level only:

- current runtime typed args do not include `sourceChainId`
- bytes `194..208` are not parsed as `sourceChainId`
- source-chain ID semantics remain unresolved at runtime level
- current runtime must not claim independent source-chain ID binding
- source-chain ID resolution remains a blocker before live-route enablement

## What Phase 17 Does Not Prove

Phase 17 does not prove:

- live-route readiness
- SPL CPI readiness
- production readiness
- deploy readiness
- external review completion
- final source-chain binding
- route-level source-chain binding correctness
- canonical-event-key source-chain binding correctness
- persistent Stage 1 processed-burn storage
- `messageNonce` runtime replay semantics
- invalid-vector e2e coverage
- removal of the 10 ignored Mollusk tests
- final immutability

## Gate Preservation

Phase 17 preserves these gates:

- bytes `194..208` remain reserved, unparsed, and not zero-validated
- the `u128` amount layout with `u64` SPL range remains a design gap
- dormant CPI helpers remain gated
- direct-call local mutation boundary remains distinct from enabled path
- 10 ignored Mollusk tests remain an evidence gap
- current `Ok(())` remains disabled-plan no-op return
- live atomicity remains unimplemented
- `sourceChainId` runtime handling remains unresolved before live-route or
  SPL-CPI enablement
- source-chain ID must not be resolved silently through bytes `194..208`

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

Phase 17 made no runtime code changes.

Phase 17 made no test code changes.

Phase 17 did not deploy.

Phase 17 did not upgrade.

Phase 17 did not submit transactions.

Phase 17 did not spend SOL.

No blocker was removed.

No production readiness is claimed.

No final immutability is claimed while upgrade authority exists.

## Next Recommended Stage

Recommended next stage:

- `stage-xxxl-x1-testnet-local-runtime-skeleton-phase-18-u128-u64-spl-amount-boundary`

That future stage should clarify the unresolved `u128` runtime amount versus
`u64` SPL Token mint amount boundary before any SPL-CPI enablement.
