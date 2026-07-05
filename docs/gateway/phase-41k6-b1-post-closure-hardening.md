# Phase 41K.6 B1 Post-Closure Hardening

Status: implemented on branch stage-41k6-b1-post-closure-hardening.

Base checkpoint:

    main @ c8a8e36
    Phase 41K.6 B1 closed
    Claude B1 blocker resolved
    D2 mint bypass closed

## Purpose

This branch closes the non-blocking notes from the final B1 hostile review before moving to B2.

These changes are defense-in-depth cleanup, not blocker fixes.

## Payload hash v2

The B1C authorization payload hash moved from:

    consume_gateway_mint_authorization_v1

to:

    consume_gateway_mint_authorization_v2

The signed payload now binds:

    processed_event
    route_id
    mint
    recipient
    amount
    guardian_set_id [u8; 32]

This explicitly closes two notes:

    - route_id is now part of the guardian-signed payload
    - guardian_set_id is now bound as the full [u8; 32], not a u32 projection

## current_slot / deadline policy

current_slot remains intentionally excluded from the B1C authorization payload hash.

Policy:

    A guardian authorization signature is valid for the exact operation payload until the processed_event is consumed.

Replay protection is provided by:

    canonical_event_key
    processed_event PDA
    single-consumption processed_event registry

Future production schemas may add:

    deadline slot
    finality slot
    active_until_slot
    authorization expiry policy

These are not B1 closure blockers.

## Adapter regression

The B1C connect adapter now has explicit regression coverage proving that:

    - valid-looking non-Ed25519 instructions are discarded
    - non-Ed25519 instructions cannot re-enter the parsed evidence pipeline
    - only real Ed25519 precompile prior instructions can produce parsed B1C evidence

## Result

Post-B1 cleanup prepares the codebase for B2:

    B2: valid quorum live-gated success test
