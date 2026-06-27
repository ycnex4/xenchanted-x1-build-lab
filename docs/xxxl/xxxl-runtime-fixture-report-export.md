# XXXL Runtime Fixture Report Export

## Purpose

This document records the deterministic report/export layer for XXXL runtime dry-run fixtures.

The report export is a review-oriented summary over the model-layer dry-run fixtures.

It is designed to be used before deployment dry-runs and before live runtime implementation.

## What the export contains

The export summarizes:

- fixture count
- unique execution vector count
- fixture ids
- vector ids
- CPI committed vectors
- CPI skipped vectors
- supply audit OK vectors
- expected rejection vectors
- route-aware success vectors
- per-fixture report details
- canonical JSON
- markdown summary

## Why this exists

Execution vectors prove deterministic runtime behavior.

Dry-run fixtures prove the vectors can be re-executed as a package.

The report export turns that dry-run package into a stable review surface.

This makes it easier to inspect readiness without reading every test.

## Route-aware coverage

The report explicitly surfaces the two successful route-aware vectors:

- Ethereum primary full-weight gateway mint
- Avalanche low-weight route-aware gateway mint

This confirms that the runtime path is not hardcoded as Ethereum-only.

Avalanche remains only a policy-modeled route candidate.

This stage does not activate Avalanche.

## CPI and rejection coverage

Successful vectors are reported as CPI committed.

Rejected vectors are reported as CPI skipped.

This keeps the atomicity boundary visible in the exported report.

Expected rejection vectors are not failures when they match expectation.

## Canonical outputs

The report export includes:

- canonical JSON
- markdown report

Both outputs are deterministic and validated by tests.

## Non-goals

This stage does not implement live X1/SVM runtime code.

It does not submit transactions.

It does not connect to RPC.

It does not derive real PDAs.

It does not activate any route.

It does not change supply policy.
