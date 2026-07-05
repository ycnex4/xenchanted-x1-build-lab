# Phase 41K.6 B2 Valid Quorum Live-Gated Success

Status: B2.4 Mollusk live-gated success implemented; B2.5 regression gates pending.

Base checkpoint:

    main @ c5e3c38
    Phase 41K.6 B1 closed
    Post-B1 payload binding hardening closed
    Next proof target: valid quorum live-gated success path

## Purpose

B2 proves the positive gated ConsumeGatewayMint execution path after B1 closure.

The target path is:

    prior Ed25519 precompile evidence
    -> payload hash v2 match
    -> guardian membership validation
    -> unique guardian quorum
    -> B1C7 authorization
    -> CPI gate open under explicit test features
    -> processed_event consumed
    -> SPL mint executed

## What B2 proves

B2 must prove that a valid operation can pass the full B1C7 authorization boundary and reach mark + mint only after authorization.

The success fixture must bind the guardian-signed payload hash to:

    processed_event
    route_id
    mint
    recipient token account
    amount
    guardian_set_id [u8; 32]

The success path must prove:

    authorization.status == Authorized
    authorization.authorization_enabled == true
    authorization.fail_fast_before_mutation == true
    authorization.evidence_from_prior_ed25519_instructions == true
    authorization.payload_hash_bound == true
    authorization.guardian_membership_validated == true
    authorization.quorum_met == true

And at the runtime/SBF boundary:

    processed_event becomes consumed
    SPL mint supply increases by amount
    recipient token account balance increases by amount

## What B2 does not prove

B2 is not production activation.

B2 does not claim that the live route is enabled on main/testnet deployment artifacts.

B2 does not replace B3 negative tests for invalid quorum, threshold failure, replay, wrong guardian set, or payload mismatch.

B2 does not decide B4 production activation policy.

B2 does not introduce production guardian keys, production watcher/relayer, production finality policy, or live deployment configuration.

## Feature gates

B2 success tests are allowed only under explicit gated feature combinations.

Expected B1C7 authorization features:

    phase-41k6-b1c7-handler-integration-test-gate
    dangerously-allow-phase-41k6-b1c7-handler-integration-test-gate-sbf-build

Expected D2 / mark+mint production-path test features, where needed:

    phase-41k5-d2-production-path-test-gate
    dangerously-allow-phase-41k5-d2-production-path-test-gate-sbf-build

These gates remain test-only and must not silently become production defaults.

## Implementation plan

### B2.1 Payload v2 fixture

Create a deterministic test fixture for:

    processed_event
    route_id
    mint
    recipient token account
    amount
    guardian_set_id [u8; 32]

Compute expected_payload_hash locally using the existing payload hash binding function.

### B2.2 Valid prior Ed25519 instruction fixture

Build synthetic prior Ed25519 precompile instructions where threshold guardians sign exactly expected_payload_hash.

The fixture must pass the current runtime adapter/parser path and must not rely on caller-supplied signature claims.

### B2.3 Lib-level positive authorization test

Add or extend lib-level tests proving the full B1C7 authorization result becomes Authorized with all required authorization flags true and mutation flags still false.

## B2.1-B2.3 lib-level evidence

Implemented before the Mollusk/SBF live-gated success test:

    - explicit B2 payload v2 fixture
    - payload hash regression proving all B2 success-path fields are bound
    - valid prior Ed25519 instruction fixture for threshold guardians
    - positive B1C7 authorization test proving all success flags before mutation
    - mutation flags remain false at authorization boundary

### B2.4 Mollusk live-gated success test

Add a Mollusk/SBF success test for the full live-gated path.

The test must execute process_instruction with:

    12-account B1 v3 account contract
    instructions sysvar account
    valid prior Ed25519 instructions
    valid guardian set PDA
    valid processed_event PDA
    valid SPL mint
    valid recipient token account
    valid mint authority PDA

Expected result:

    Check::success()
    processed_event consumed
    SPL mint supply += amount
    recipient token account amount += amount

## B2.4 Mollusk live-gated success evidence

Implemented as:

    programs/xxxl-svm/tests/phase_41k6_b2_valid_quorum_live_gated_success.rs

The B2.4 test builds SBF with both B1C7 and D2 dangerous test gates, injects valid prior Ed25519 precompile evidence through the instructions sysvar, loads a PDA-bound active guardian set, and executes ConsumeGatewayMint through process_instruction.

Expected success checks:

    Check::success()
    processed_event owner becomes program id
    processed_event data becomes final consumed image
    SPL mint supply increases by amount
    recipient token account amount increases by amount
    recipient_balance account remains unchanged

### B2.5 Regression gates

Keep the old safety gates intact:

    default/non-B1C7 ConsumeGatewayMint rejects before mutation
    B1C7 feature without dangerous allow compile-fails for SBF
    D2 feature without dangerous allow compile-fails for SBF
    D2 cannot open mark+mint without B1C7
    invalid or missing prior Ed25519 evidence rejects before mutation

### B2.6 Review package

Document final B2 evidence and ask for targeted hostile review before merge.

## Expected review question

Does B2 prove a valid guardian quorum can execute the gated ConsumeGatewayMint success path without reopening any B1 authorization bypass, replay bypass, or mutation-before-authorization path?

### Harness note: instructions sysvar source

B2 uses Mollusk transaction-instruction execution rather than a manually supplied `instructions_sysvar` account. The positive path requires the current `ConsumeGatewayMint` instruction to observe two strictly prior Ed25519 precompile instructions from the transaction instructions sysvar. A single-instruction Mollusk call makes the current instruction index `0`, which produces an empty prior range and correctly rejects authorization before mutation.

### Harness note: Ed25519 execution cache

Mollusk transaction-instruction execution requires every prior instruction program id to exist in the program cache. B2 therefore loads a no-op SBF ELF under the Ed25519 precompile program id for this harness only. The current handler does not trust the no-op program result as production authorization; it reads the prior instruction bytes from the instructions sysvar and routes them through the B1C evidence, payload-binding, membership, and quorum pipeline before any mutation.
