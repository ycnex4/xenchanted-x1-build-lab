# Checkpoint: XXXL Live Route Activation and Bootstrap Guardian Policy

Stage: stage-xxxl-live-route-activation-and-bootstrap-guardian-policy

Status: COMPLETED

## Goal

Document the conditions required before live XXXL route activation and define the acceptable bootstrap guardian model.

## Completed

Added:

- `docs/xxxl/xxxl-live-route-activation-checklist.md`
- `docs/gateway/bootstrap-guardian-policy.md`

The live route activation checklist defines gates for:

- runtime account contract
- mint authority model
- replay protection
- atomicity
- SPL CPI execution proof
- guardian approval model
- bootstrap guardian disclosure
- public proof log
- caps and blast-radius limits
- monitoring and incident response
- external review

The bootstrap guardian policy defines:

- correct terminology
- incorrect decentralization claims
- security meaning of 5 keys on one server
- minimum bootstrap requirements
- burn-backed mint rule
- public proof bundle
- caps
- migration path to independent guardians
- reputation rule

## Safety boundary

No live route was activated.

No code path was enabled.

No SPL CPI behavior was changed.

No guardian keys were generated.

No secrets were handled.

No deploy action was performed.

## Decision

Live XXXL route activation remains blocked until the activation checklist is satisfied.

Operator-controlled bootstrap guardians are acceptable only as a temporary, disclosed, capped, auditable launch mode.
