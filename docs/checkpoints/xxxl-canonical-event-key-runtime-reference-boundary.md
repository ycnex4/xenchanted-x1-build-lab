# XXXL Canonical Event Key Runtime Reference Boundary Checkpoint

Status: COMPLETED.

This checkpoint records the runtime-facing canonical event key reference added after external review feedback.

## Summary

The XXXL SVM runtime documentation now has a direct reference for the existing Stage 1 `canonicalEventKey` policy.

This closes the review concern that the atomic mutation planning notes referenced canonical burn event identity without restating the exact derivation in a short runtime-facing document.

## Canonical event key policy

The runtime reference records:

- `canonicalEventKey = keccak256(canonicalEventKeyPreimage)`
- `canonicalEventKeyPreimage = sourceChainId || sourceToken || sourceBurnTxHash || sourceBurnEventIndex`
- the field order is fixed
- the current Stage 1 vector preimage length is 128 bytes
- the processed registry key is `canonicalEventKey`
- one `canonicalEventKey` can produce at most one successful XXXL mint
- future runtime must recompute and compare the key before mutation

## Updated files

Documentation:

- `docs/xxxl/xxxl-canonical-event-key-runtime-reference-boundary.md`
- `docs/checkpoints/xxxl-canonical-event-key-runtime-reference-boundary.md`
- `docs/xxxl/xxxl-atomic-mutation-planning-notes-boundary.md`
- `docs/xxxl/xxxl-runtime-safety-review-package-boundary.md`
- `docs/checkpoints/current-design-checkpoint.md`
- `README.md`

## Safety result

The runtime remains:

- scaffold-only
- locked
- unreleasable
- not deployable

The release decision remains:

- release allowed: `false`
- release blocked: `true`
- primary blocker code: `RUNTIME_SAFETY_LOCK_ACTIVE`

## Decision

Accepted as a documentation-only runtime reference boundary.

No runtime behavior was enabled.
