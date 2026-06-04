# Audit Vitest Upgrade Notes

## Branch

audit-vitest-upgrade

## Purpose

This milestone addresses the npm audit findings caused by the older Vitest / Vite / esbuild dependency chain.

## Change

Updated dev dependency:

- vitest: ^2.1.0 -> ^4.1.8

Added:

- vitest.config.ts

## Reason

The previous npm audit report showed vulnerabilities through:

- vitest
- vite
- esbuild

The recommended automatic fix required a breaking Vitest upgrade, so the upgrade was handled explicitly on a separate branch instead of running npm audit fix --force blindly.

## Vitest 4 behavior note

After upgrading to Vitest 4, the test runner also discovered compiled tests under dist/tests after npm run build.

To preserve the intended test boundary, vitest.config.ts excludes:

- dist/**

This keeps npm test focused on source tests under tests/.

## Validation result

After the upgrade and config change:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 28 test files passed
- 171 tests passed

## Runtime note

Vitest 4 dependency tooling may require newer Node versions through its dependency graph.

The current local environment uses Node 24 and passes all checks.

## Architectural boundary

This milestone changes dev tooling only.

It does not change protocol logic.

It does not change application state logic.

It does not change CLI behavior.

It does not change snapshot behavior.
