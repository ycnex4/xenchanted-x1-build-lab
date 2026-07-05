# Phase 41K.6 B6.5 — Boundary review request

## Purpose

This is a review request for the B6.1 through B6.5 boundary sequence before any possible later testnet submit rehearsal.

This is not a request to approve submit.

This is not a request to approve signing.

This is not a request to approve SOL spend.

This is not a request to approve key handling.

This is not a request to approve deploy.

This is not a request to approve B1C7 guard removal.

This is not a request to approve production or production-like activation.

## Current main checkpoint

Current main checkpoint:

c6b6943 Merge phase 41K.6 B6.5 no-go status snapshot

## Sequence under review

B6 sequence currently merged:

- B6.1: X1 testnet E2E opening boundary.
- B6.2: testnet readiness inventory.
- B6.3: no-send dry-run package rehearsal.
- B6.4: external signer / operator approval boundary.
- B6.5: pre-submit decision packet.
- B6.5: go / no-go status snapshot.

Current B6.5 decision:

NO-GO.

## Review question

Please review whether the current boundary is strong enough before any later explicit testnet submit rehearsal can be considered.

The specific question is:

Are B6.1 through B6.5 sufficient to keep testnet signing, transaction submission, SOL spend, private-key handling, deploy, B1C7 gate changes, and production activation closed until a later explicit go decision?

## Current safety claims

The current documents claim:

- B6.1 only opens the E2E rehearsal track as simulation/dry-run.
- B6.2 only defines readiness inventory.
- B6.3 only rehearses the B5 relayer package shape with no-send constraints.
- B6.4 only defines approval classes and keeps live-action approvals closed.
- B6.5 only defines the pre-submit decision packet.
- The B6.5 status snapshot records current decision as NO-GO.

## Current forbidden actions

The current boundary forbids:

- signing,
- transaction submission,
- SOL spend,
- private-key access,
- seed phrase handling,
- keypair file loading,
- requestAirdrop,
- deploy,
- mintTo,
- processed_event mutation,
- B1C7 compile_error guard removal,
- B1C7 feature gate weakening,
- production activation,
- production-like activation.

## Current allowed actions

The current boundary allows only:

- documentation,
- read-only inventory planning,
- no-send package preparation,
- focused offline package rehearsal,
- redacted value packet preparation,
- external review preparation.

## B6.3 implementation note

B6.3 added a focused TypeScript test:

tests/phase41k6_b6_no_send_dry_run_package_rehearsal.test.ts

The test verifies:

- the B5 relayer submission package can be assembled from readiness-style values,
- the known-answer payload hash is preserved,
- the handler instruction boundary is preserved,
- quorum package structure remains valid,
- no-send boundary flags are preserved,
- handler-bound drift after evidence preparation is rejected,
- operational ids remain outside payload hash binding.

Focused test result when merged:

- 1 file passed.
- 3 tests passed.

## Specific review questions

1. Is the B6.1 through B6.5 boundary sufficiently explicit that B6.5 remains NO-GO?

2. Are there any hidden paths by which signing, submit, SOL spend, key handling, deploy, or gate removal could be interpreted as already approved?

3. Is the B1C7 compile_error / feature gate policy stated strongly enough?

4. Is the separation between no-send dry-run package rehearsal and actual testnet submit clear enough?

5. Is the B6.5 pre-submit decision packet missing any mandatory field before a later GO decision?

6. Is the NO-GO snapshot sufficient as a checkpoint before collecting exact testnet values?

7. Should a redacted testnet value packet be added before any later go/no-go decision?

8. Should another full test gate be required before any later submit boundary, even if intervening changes are docs-only?

9. Are there any terms that could imply production or production-like activation before explicit operator/project approval?

10. Is there any reason B6 should stop here until external review is recorded?

## Expected reviewer output

Please answer with one of:

- APPROVE B6.5 NO-GO BOUNDARY
- APPROVE WITH NOTES
- NEEDS CHANGE
- BLOCKER

If notes are provided, please separate:

- mandatory before any later B6.5 GO decision,
- optional before B6.5 GO,
- can defer after testnet rehearsal.

## Current intended next step after review

If approved, the next safe engineering step is not submit.

The next safe step is a redacted testnet value packet or exact-value inventory draft.

B6.5 remains NO-GO until a later explicit written decision changes it.
