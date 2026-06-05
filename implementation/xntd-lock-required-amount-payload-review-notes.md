# XNTD Required Lock Amount Payload Review Notes

## Branch

xntd-lock-required-amount-payload-review

## Purpose

This milestone reviews the future watcher / proof / registrar payload shape for XNTD lock / relock epoch minimum validation.

It does not change runtime code.

## Question

Should LOCK_XNTD / RELOCK_XNTD payloads explicitly carry the required XNTD lock amount, or should the registrar derive it internally from authoritative XC state?

## Decision

Use:

- observedRequiredXntdLock

in watcher / proof / registrar payloads.

The registrar / integration layer must still verify:

observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

and:

amountXntd >= observedRequiredXntdLock

## Reasoning

This provides a balanced model:

1. The proof remains self-describing.
   - It records what requirement the watcher observed.

2. The registrar does not blindly trust the proof payload.
   - It verifies the observed value against authoritative XC state.

3. Audit / debug / logs are clearer.
   - The submitted observed requirement and the authoritative expected requirement can be compared.

4. Build state remains clean.
   - After successful validation, Build state records requiredXntdLock.

## Conceptual mapping

Payload:

amountXntd = actual user lock / relock amount
observedRequiredXntdLock = requirement observed by watcher for lockEpoch
lockEpoch = XC epoch used for requirement

Validation:

amountXntd > 0
observedRequiredXntdLock > 0
amountXntd >= observedRequiredXntdLock
observedRequiredXntdLock == authoritativeEpochMinimum(lockEpoch)

After validation:

lockedXntd = amountXntd
requiredXntdLock = observedRequiredXntdLock

## Alternatives considered

### Registrar derives requirement internally only

Pros:

- less trust in watcher payload
- source of truth stays closer to registrar validation

Cons:

- proof is less self-describing
- audit trail is weaker
- harder to explain why a specific requiredXntdLock was recorded

### Payload carries requiredXntdLock directly

Pros:

- simple naming
- maps directly to Build state

Cons:

- may imply the payload is authoritative
- less clear that the value is only observed until verified

## Scope boundary

This milestone updates design documentation only.

It does not change:

- proof types
- watcher candidate types
- proof conversion
- registrar payload builders
- proof submission
- registrar handlers
- lockXntd()
- relockXntd()
- tests

## Updated document

Updated:

- docs/registrar/xntd-lock-epoch-minimum-validation.md

## Validation

After this milestone:

- npm run typecheck: passed
- npm test: passed
- npm run build: passed
- npm audit --audit-level=moderate: found 0 vulnerabilities
- 29 test files passed
- 179 tests passed
