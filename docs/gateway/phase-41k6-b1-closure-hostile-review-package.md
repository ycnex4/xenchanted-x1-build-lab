# Phase 41K.6 B1 Closure — Hostile Review Package

Status: review package
Base merge: fce6ddf Merge phase 41K.6 B1C handler integration

## Purpose

This document closes the B1 blocker review.

B1 blocker statement:

The live ConsumeGatewayMint mark + mint path must be unreachable unless guardian quorum authorization is proven from real prior Ed25519 precompile evidence.

Phase 41K.6 B1C now implements that authorization pipeline end-to-end, feature-gated, with default builds closed.

## Current conclusion

B1 is resolved at implementation level.

The gated handler path now requires:

1. authoritative guardian set loading
2. real instructions sysvar evidence
3. prior Ed25519 precompile parsing
4. payload hash binding to the current mint operation
5. guardian membership validation
6. unique guardian quorum counting
7. authorization before mark + mint

Default builds remain closed.

## Architecture trace

### B1A — V3 account contract

Adds the V3 ConsumeGatewayMint account contract with instructions_sysvar.

Key invariants:

- the handler path that uses guardian authorization has access to the real instructions sysvar account
- account order and mutability requirements are explicit
- default legacy account contract remains separate

### B1B — Guardian set loading bridge

Loads the authoritative guardian set from the program-controlled PDA.

Key invariants:

- no caller-supplied guardian set
- account key checked
- account owner checked
- PDA checked
- active status checked
- threshold checked
- guardian count checked
- duplicate guardian keys rejected
- loading remains read-only

Current guardian set layout has active status but no expiry slot. If a future layout adds active_until_slot, B1C.7 must enforce current_slot <= active_until_slot.

### B1C.1 — Authorization result types

Defines isolated authorization result types.

Key invariant:

- authorization result modeling does not mutate state

### B1C.2 — Instructions sysvar evidence bridge

Loads prior Ed25519 evidence from the real instructions sysvar boundary.

Key invariants:

- uses real instructions sysvar account info
- checks instructions sysvar account id
- loads current instruction index through checked runtime API
- loads only prior instructions
- filters Ed25519 precompile instructions
- accepts no frontend/watcher proof as authorization

### B1C.3 — Ed25519 evidence parser

Parses Ed25519 precompile instruction data.

Key invariants:

- parser checks header/offset layout
- extracts signer public key
- extracts signed message
- does not perform authorization alone
- does not mutate state

### B1C.3-connect — Adapter

Connects checked prior instruction loading to B1C.3 parser.

Key invariants:

- consumes runtime-loaded prior instructions
- rejects malformed evidence
- preserves source instruction index
- remains non-authorizing alone

### B1C.4 — Payload hash binding

Computes expected payload hash locally and binds parsed Ed25519 signed messages to it.

Payload context includes:

- processed event account
- mint
- recipient token account
- amount
- guardian set numeric id

Key invariants:

- local hash computation
- no caller-provided hash accepted
- current slot intentionally excluded
- mismatched signed message rejects before authorization
- no mutation

### B1C.5 — Guardian membership validation

Validates that payload-bound signers are members of the authoritative B1B guardian set.

Key invariants:

- consumes B1B-loaded guardian set
- rejects unauthorized signer
- preserves duplicate signers for later quorum deduplication
- no mutation

### B1C.6 — Quorum counting

Counts unique validated guardians and compares them to the B1B threshold.

Key invariants:

- duplicate signer counts once
- threshold comes from authoritative guardian set
- quorum met means sufficient unique guardians signed
- authorization remains disabled in B1C.6 alone

### B1C.7 — Handler authorization boundary

Composes the full B1C pipeline.

Pipeline:

    B1B guardian set
    -> B1C.2 instructions_sysvar evidence
    -> B1C.3/connect parser
    -> B1C.4 payload hash binding
    -> B1C.5 guardian membership
    -> B1C.6 quorum counting
    -> authorization_enabled = true

Key invariants:

- fail-fast before mutation
- no mark
- no mint
- no live route by boundary alone

### B1C.7 final — Mark + mint wiring

Wires B1C.7 authorization before the actual mark + mint boundary.

Key invariants:

- B1C.7 authorization is called before mark + mint
- rejected authorization exits before mutation
- mark + mint only runs after authorization_enabled = true
- CPI gate is checked before mutation
- default/non-B1C7 path remains unchanged
- B1C7-gated handler path uses V3 account contract with instructions_sysvar

## Safety invariants to review

### Authorization-before-mutation

There must be no path where processed_event is marked before full B1C authorization succeeds.

### Mint-before-authorization

There must be no path where SPL mint CPI is attempted before full B1C authorization succeeds.

### Caller-provided evidence

There must be no path where frontend, watcher, or caller-supplied proof bytes are accepted as authorization evidence.

Only real prior Ed25519 precompile instructions loaded from instructions sysvar may authorize.

### Payload binding

A valid guardian signature for another operation must not authorize this operation.

Binding must include operation-specific values and reject payload mismatch.

### Guardian authority

Guardian membership and threshold must come from the authoritative B1B guardian set.

### Duplicate signer

The same guardian signing multiple Ed25519 precompile instructions must count once.

### Replay protection

Replay protection remains check-before-mark.

Mark + mint remains one execution path after authorization.

### Feature gate

Default builds must remain closed.

B1C7 integration must require explicit dangerous test allow feature when feature-enabled.

Earlier compile_error guards must not be removed or weakened.

### CPI gate

Even after authorization, CPI gate must be checked before mutation.

If CPI gate is closed, there must be no processed_event mark.

## Validation already run

Latest accepted validation:

    default lib passed
    B1C7 gated lib passed
    closed-gate Mollusk passed
    zero regression

## Hostile review questions

1. Can any mutable account change happen before B1C.7 authorization succeeds?
2. Can processed_event be marked if authorization fails?
3. Can SPL mint CPI run if authorization fails?
4. Can a caller provide fake evidence bytes instead of real prior Ed25519 precompile instructions?
5. Can a valid signature for another operation authorize this operation?
6. Can an unauthorized signer pass membership validation?
7. Can one guardian sign multiple times and satisfy a threshold greater than 1?
8. Can a non-authoritative guardian set define threshold/signers?
9. Can default builds accidentally open the B1C7 path?
10. Can CPI gate failure happen after processed_event mark?
11. Does replay protection still reject already-consumed events?
12. Are B1C slices still isolated enough to audit individually?

## Known non-goals

B1 closure does not include:

- production guardian keys
- off-chain signer infrastructure
- guardian operations runbook
- production deployment
- production feature enablement
- guardian rotation governance
- future guardian expiry field

These are later operational/deployment topics.

## Review request

Theo / Claude:

Please hostile-review the B1 closure state.

Review target:

    main @ fce6ddf
    Phase 41K.6 B1C complete
    B1C.7 mark+mint wiring merged

Question:

Does this fully resolve the B1 blocker, or is there any remaining authorization bypass / mutation-before-authorization path?


## Hostile review response — Theo vectors

Status: response to Theo B1 closure hostile review.

### Vector 1 — Guardian set account provenance

Conclusion: clean.

The guardian set account is not accepted as an unconstrained caller-provided account.

The loader derives the expected guardian set PDA from:

    seed 0: "xxxl"
    seed 1: "guardian-set"
    seed 2: guardian_set_id
    program id: current program_id

The loader then checks:

- guardian_set_account.owner == expected_program_id
- guardian_set_account.key == expected_pda
- account is readonly
- account is non-signer
- account discriminator is correct
- schema version is correct
- account data guardian_set_id == expected guardian_set_id
- active status is active
- threshold and guardian count are sane
- duplicate guardian public keys are rejected

Therefore an attacker cannot pass an arbitrary account with threshold = 1 and their own key unless that account is also the valid program-derived PDA for the expected guardian_set_id and owned by the program.

The specific fake-account attack described by Theo should fail at the owner/key/PDA checks before B1C.5 membership validation or B1C.6 quorum counting.

### Vector 2 — Threshold sanity

Conclusion: clean.

B1B rejects:

- threshold = 0
- guardian_count = 0
- guardian_count > MAX_SUPPORTED_GUARDIAN_COUNT
- threshold > guardian_count
- duplicate guardian public keys

This means zero-signature authorization and impossible threshold configurations are rejected at guardian set loading.

### Vector 3 — Guardian set freshness / rotation

Conclusion: partially addressed by current layout; operational follow-up remains.

Current guardian set layout has active status but no active_until_slot / expiry field.

Current enforced freshness mechanism:

- active guardian sets are accepted
- inactive or deprecated guardian sets are rejected

Therefore rotation safety depends on old guardian sets being marked inactive/deprecated in the current layout.

Future hardening recommendation:

- add active_until_slot or monotonic guardian set generation/version policy
- enforce current_slot <= active_until_slot if that field is added

This is not an authorization bypass in the current implemented model if rotation correctly deprecates old sets, but it is a production operations requirement.

### Vector 4 — Other mint paths

Conclusion: clean for default/non-harness runtime path.

The SVM entrypoint routes to processor::process_instruction.

The runtime instruction enum currently contains only:

    ConsumeGatewayMint

The normal processor match routes only:

    XxxlInstruction::ConsumeGatewayMint(args) -> process_consume_gateway_mint(...)

There is no admin mint instruction, no config instruction, no emergency mint instruction, and no second normal runtime instruction variant.

There are test/harness paths behind explicit feature gates:

- phase-41k4-svm-test-harness
- phase-41k5-spl-mint-to-cpi-test-gate

These are non-default test scaffolds and are not default runtime paths.

SPL mint_to is centralized in the CPI boundary. The guarded CPI boundary checks spl_mint_to_cpi_execution_enabled before invoking mint_to. If the CPI gate is closed, it returns CpiBoundaryNotReady.

B1C.7 final wiring additionally checks the CPI gate before calling atomic_mark_and_mint_boundary, so CPI gate failure cannot happen after processed_event mark in the B1C7 path.

Important distinction:

- B1C7 authorization gate alone does not silently open SPL mint CPI.
- Actual mint execution still requires the explicit CPI execution gate.
- Default builds remain closed.

### Vector 5 — Processed event registry isolation

Conclusion: clean for default/non-harness runtime path, with test harness caveat.

The processed_event write boundary:

- derives the expected processed_event PDA from canonical_event_key
- rejects wrong PDA
- uses the 41K.3 processed registry loader as a gate
- requires the account to be unprocessed before marking
- writes final consumed image
- re-decodes after write

The B1C7 path calls mark only after authorization succeeds and after the CPI gate check.

There is a phase-41k4 test harness for marking, but it is feature-gated and not part of default runtime behavior.

Therefore the default/non-harness runtime does not expose a standalone registry mark instruction.

### Response to Theo's two blocker questions

Question 1: How is the guardian set account address constrained/verified in the handler?

Answer:

The handler path passes the guardian set AccountInfo into B1B loading with program_id and args.guardian_set_id. B1B derives expected PDA from the fixed guardian set seed format and rejects if the provided account key does not match the expected PDA or if owner does not match program_id. It also checks that the account data guardian_set_id matches the expected guardian_set_id.

So the guardian set account is protocol-constrained by PDA + owner + embedded guardian_set_id.

Question 2: Whether any other instruction or path can invoke mint_to or mark the registry?

Answer:

For default/non-harness runtime path, no.

The normal instruction enum has only ConsumeGatewayMint, and the normal processor match routes only to process_consume_gateway_mint.

SPL mint_to is centralized in the CPI boundary and gated by spl_mint_to_cpi_execution_enabled.

Processed event marking is centralized in mark_processed_event_atomic and is called by the B1C7-authorized mark+mint path after authorization and CPI gate checks.

There are test harness paths behind explicit feature gates. These are not default runtime paths and remain outside production closure.

### Updated closure claim

B1 closure should be accepted for the default/non-harness runtime path.

Required production follow-ups remain:

- guardian operations / rotation runbook
- production guardian set initialization procedure
- explicit policy for deprecating old guardian sets
- optional future active_until_slot hardening
- production decision on feature gates and deployment


## Claude hostile review BLOCK — authorization bypass fix

Status: BLOCK acknowledged.

Claude identified a real authorization bypass in the previous closure branch:

- non-B1C7 process_consume_gateway_mint called atomic_mark_and_mint_boundary directly
- atomic_mark_and_mint_boundary marked processed_event before the CPI gate
- SPL mint CPI execution was controlled by D2 feature gates independent of B1C7
- the old D2 production-path e2e test proved mark + mint without guardian evidence

Fix strategy:

1. D2 production-path gate now depends on B1C7 handler integration gate.
2. D2 dangerous allow now depends on B1C7 dangerous allow.
3. spl_mint_to_cpi_execution_enabled opens only when D2 + D2 dangerous + B1C7 + B1C7 dangerous are all enabled.
4. non-B1C7 process_consume_gateway_mint now rejects with CpiBoundaryNotReady before Rent, Clock, mark, or mint.
5. the old D2 success test is rewritten into a rejection-before-mutation regression test.

Security result:

- no feature combination can open SPL mint CPI without B1C7 being enabled
- default/non-B1C7 ConsumeGatewayMint cannot call atomic_mark_and_mint_boundary
- D2-only mark+mint bypass is removed
- old exploit test is inverted and must now prove no mutation
