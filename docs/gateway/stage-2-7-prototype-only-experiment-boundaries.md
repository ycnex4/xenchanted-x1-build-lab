# Stage 2.7 Prototype-Only Experiment Boundaries

## Purpose

Stage 2.7 defines the boundaries for future prototype-only experiments related to the X1-side gateway.

This is a planning-only checkpoint.

It does not implement production runtime code, deploy an X1 program, create production mint authority, enable real cross-chain minting, or approve a final gateway architecture.

The purpose is to make sure future evidence collection does not accidentally become hidden gateway implementation.

The main rule remains:

    Do not implement runtime behavior that depends on unconfirmed X1 assumptions.

## Background

Stage 2.6 defined the X1 runtime evidence collection plan.

That plan identified the first evidence targets:

1. transaction atomicity
2. account write rollback
3. CPI/token mint rollback
4. token mint authority model
5. deterministic account derivation
6. processed burn registry persistence

These facts may require prototype-only experiments.

Stage 2.7 defines what prototype-only means.

## Prototype-only definition

A prototype-only experiment is a limited, isolated, non-production test created only to collect evidence about a specific runtime assumption.

A prototype-only experiment may:

- test runtime behavior
- test rollback behavior
- test account persistence
- test deterministic account derivation
- test token mint interface behavior
- test compute budget
- test transaction size
- test event/log observability
- produce documentation evidence
- produce repeatable commands
- produce local or testnet-only results

A prototype-only experiment must not:

- become a production gateway
- expose a live user flow
- create production mint authority
- enable real cross-chain minting
- support mainnet bridge operation
- become frontend-visible as a real protocol feature
- rely on admin recovery
- rely on secret output
- make architecture choices before evidence is documented

## Non-negotiable boundaries

Future prototype branches must follow these boundaries.

| Boundary | Requirement |
| --- | --- |
| No production deployment | Prototype code must not deploy a production X1 gateway. |
| No live mint authority | Prototype code must not control production XXXL mint authority. |
| No real cross-chain minting | Prototype code must not accept real source events and mint live tokens. |
| No frontend production flow | Prototype code must not expose a user-facing production bridge UI. |
| No admin recovery assumption | Prototype design must not depend on manual admin repair. |
| No secrets in logs | Prototype commands must not print private keys, RPC keys, tokens, seed phrases, or sensitive values. |
| Evidence mapping required | Every prototype must map to one or more EV-* evidence IDs. |
| Documentation required | Every result must be documented before it affects architecture choice. |
| No architecture shortcut | A successful prototype does not automatically approve direct mint or claim-based flow. |
| No rule reinterpretation | Prototype results must not reinterpret prior planning rules silently. |

## Allowed prototype categories

Allowed prototype categories include:

- transaction atomicity prototype
- account write rollback prototype
- CPI/token mint rollback prototype
- token mint authority prototype
- deterministic account derivation prototype
- processed burn registry persistence prototype
- compute budget measurement prototype
- transaction size measurement prototype
- event/log/indexing prototype
- token account handling prototype
- pause state behavior prototype
- upgrade/migration boundary prototype
- guardian signature verification limit prototype
- claim storage/rent prototype

Each prototype must be scoped to evidence collection.

## Disallowed prototype categories

Disallowed prototype categories include:

- production gateway program
- production bridge relayer
- production guardian network
- production mint authority
- production frontend bridge flow
- live cross-chain minting
- mainnet user claim flow
- admin recovery tool
- emergency manual mint tool
- hidden upgrade path
- token distribution mechanism
- market/listing integration
- irreversible user-facing migration

If a prototype begins to require any of these, it is no longer prototype-only.

## Required prototype document format

Every prototype-only experiment should have a document with this structure:

- title
- purpose
- branch
- evidence IDs covered
- risks covered
- what is being tested
- what is not being tested
- environment
- commands
- expected result
- observed result
- pass/fail status
- limitations
- conclusion
- next step

Evidence should be written before or alongside prototype code.

Prototype code without evidence documentation should not be merged.

## Evidence ID mapping

Every prototype must explicitly reference one or more Stage 2.6 evidence IDs.

Examples:

| Prototype | Evidence IDs |
| --- | --- |
| Atomic rollback prototype | EV-01, EV-02 |
| Failed token mint rollback prototype | EV-02, EV-03 |
| Mint authority prototype | EV-04 |
| Processed registry prototype | EV-05, EV-06 |
| Transaction payload measurement | EV-07, EV-15 |
| Compute measurement | EV-08, EV-15 |
| Event/log observability prototype | EV-09 |
| Recipient token account prototype | EV-10 |
| Pause behavior prototype | EV-11 |
| Upgrade boundary prototype | EV-12 |
| Source finality/fork documentation | EV-13, EV-14 |
| Claim storage prototype | EV-16 |

A prototype that does not map to an EV-* item should not be started.

## Branch naming

Prototype-only branches should use a clear prefix.

Recommended pattern:

    prototype-x1-evidence-<short-topic>

Examples:

- prototype-x1-evidence-atomic-rollback
- prototype-x1-evidence-token-mint-rollback
- prototype-x1-evidence-processed-registry
- prototype-x1-evidence-compute-budget
- prototype-x1-evidence-transaction-size
- prototype-x1-evidence-event-logs

Planning branches should continue to use Stage numbering.

Prototype branches should not pretend to be production implementation branches.

## Commit rules

Prototype-only commits should make their scope obvious.

Good commit examples:

- Add prototype-only atomic rollback evidence plan
- Add prototype-only token mint rollback experiment
- Document X1 transaction atomicity evidence result
- Record compute budget measurement for gateway verification prototype

Bad commit examples:

- Implement gateway
- Add bridge
- Add minting
- Add production relayer
- Enable X1 bridge
- Add user bridge flow

Commit messages should not imply production readiness.

## File placement

Prototype-only documentation should go under:

    docs/gateway/evidence/

Prototype-only code, if needed later, should be isolated under a clearly named prototype or experiment path.

Suggested future path:

    prototypes/x1-gateway-evidence/

If the repository does not yet have prototype code, Stage 2.7 does not create it.

This checkpoint only defines boundaries.

## Safety rules for commands

Prototype commands must not print secrets.

Do not print:

- private keys
- seed phrases
- mnemonic values
- RPC API keys
- bearer tokens
- signing keys
- guardian private material
- environment file contents

Safe commands should prefer:

- filename-only checks
- redacted output
- public addresses
- test-only generated keys
- local-only mock values
- non-production RPC endpoints
- explicit warnings before any risky command

If a command may expose sensitive values, it must not be used in chat without redaction.

## Merge requirements

A prototype-only branch may be merged only if:

- it does not enable production behavior
- it does not include production secrets
- it does not include live mint authority
- it maps to EV-* evidence IDs
- it documents observed results
- it documents limitations
- tests pass if code is added
- build passes if code is added
- audit remains clean if dependencies are added
- README/checkpoint are updated if the evidence changes project status

A prototype that fails is still useful if it is documented clearly.

Failed evidence can be merged as documentation if it informs architecture decisions.

## Architecture decision rule

A prototype result is not an architecture decision by itself.

The architecture decision must happen only after:

- evidence is collected
- evidence is documented
- risks are updated
- direct mint and claim-based impacts are compared
- unresolved assumptions are listed
- no-admin/first-principles compatibility is reviewed

Direct mint remains preferred only if evidence supports it.

Claim-based remains fallback only if evidence supports it.

Neither candidate should proceed if critical evidence remains missing.

## Current conclusion

Stage 2.7 defines the guardrails for future prototype-only work.

The repository should continue to avoid production gateway implementation until evidence exists.

The next useful step is to begin the first prototype-only evidence branch for transaction atomicity and account write rollback, or to collect official X1 documentation if available.

The preferred first evidence target is:

    EV-01 transaction-level atomicity
    EV-02 account write rollback

These are foundational for both direct mint and claim-based flow.
