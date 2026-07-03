# Phase 41K.4 — Atomic Processed-Event Marking Review Request

## Review target

Please review:

docs/gateway/phase-41k4-atomic-processed-event-marking-plan.md

## Scope

This is a design review only.

No runtime code is introduced in this phase.

The plan defines how the XXXL SVM runtime should atomically create, initialize, and consume a processed-event PDA so that replay protection can become real after Phase 41K.3.

## Main question

Is the proposed 41K.4 boundary sufficient before implementation?

Specifically, does it correctly define:

- allowed processed-event PDA states;
- the only allowed transition: SystemOwnedEmpty -> InitializedConsumed;
- rejection of missing, malformed, wrong-owner, already-processed, and initialized-unconsumed states;
- lamport-dusted PDA handling;
- rent top-up requirements;
- avoidance of naive create_account assumptions;
- no durable consumed == false state;
- final byte-image write with consumed == true;
- payload binding to the same internally decoded quorum-authorized payload used by 41J;
- loader expected_canonical_event_key identical to the 41J payload-derived canonical_event_key;
- marked amount binding to an authorized payload amount;
- no SPL mint before successful processed-event marking;
- required tests before implementation.

## Non-goals

Please do not ask for SPL minting, live route, production routing, frontend gateway, relayer implementation, or full instruction handler in this phase.

Those are later phases.

## Known carry-forward constraint

41K.3 must not be wired into live 41J eligibility until 41K.4 is accepted.

A read-only scaffold may exist only if explicitly marked non-live / no-write / no-route / no-SPL-mint.

## Expected answer

Verdict:

ACCEPT / ACCEPT WITH NOTES / REQUIRES FIXES

Required fixes:

Non-blocking notes:

Is the plan sufficient before 41K.4 implementation:

Is the plan sufficient before 41K.3 to 41J live wiring:

Please be hostile.
