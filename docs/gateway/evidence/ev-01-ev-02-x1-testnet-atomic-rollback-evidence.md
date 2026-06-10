# EV-01 / EV-02 X1 Testnet Atomic Rollback Evidence

## Purpose

This document tracks the first real X1 testnet/runtime evidence experiment for the future X1-side gateway.

Covered evidence IDs:

- EV-01 transaction-level atomicity
- EV-02 account write rollback

This experiment follows the local reference model:

- src/prototypes/atomic-rollback-reference.ts
- tests/prototypes/atomic-rollback-reference.test.ts

The local reference model defines expected behavior.

This X1 testnet experiment must check whether the actual X1 runtime behaves the same way.

## Branch

- prototype-x1-testnet-atomic-rollback-evidence

## Scope

The testnet experiment should verify the minimal rollback scenario:

1. create or use a test-only state account
2. read and record initial state
3. perform one or more state writes
4. intentionally fail after the writes
5. read final state
6. prove final state equals initial state

If possible, the experiment should also verify multi-account rollback:

1. write account A
2. write account B
3. intentionally fail
4. prove both A and B remain unchanged

## Out of scope

This experiment must not include:

- production gateway implementation
- production bridge implementation
- XXXL mint authority
- live token minting
- guardian signature verification
- canonicalEventKey production handling
- processed burn registry production handling
- source chain event processing
- frontend bridge UX
- relayer production logic
- admin recovery assumptions

EV-03 token mint/CPI rollback is out of scope for this branch.

## Required safety boundaries

The experiment must be testnet-only.

The experiment must not print:

- private keys
- seed phrases
- RPC API keys
- wallet secrets
- access tokens
- raw environment values

Commands and logs should show only non-sensitive status, file names, public addresses, transaction signatures, and redacted configuration.

## Evidence fields to collect

When the experiment is performed, record:

- X1 network name
- toolchain used
- test account/program identifiers
- command used, with secrets redacted
- initial state
- attempted mutation
- intentional failure reason
- final state
- transaction signature or test output if available
- pass/fail result
- limitations
- remaining uncertainty

## Pass condition

EV-01 / EV-02 can be considered evidence-supported only if X1 testnet shows:

- failed transaction leaves no partial writes
- failed instruction leaves no partial writes
- multi-account write failure rolls back all tested accounts
- result is repeatable
- no manual/admin repair is required

## Fail condition

The experiment fails if:

- any partial state persists after failure
- one account rolls back while another does not
- rollback depends on manual cleanup
- behavior is inconsistent between runs
- the runtime behavior is unclear or unobservable



## X1 testnet probe preparation attempt

A local X1 Anchor-style testnet probe was prepared in the separate local repository:

- ~/xenchanted-x1-lab/hello-x1

Local branch:

- ev-01-ev-02-atomic-rollback-testnet-probe

Local commits:

- be66899 Add atomic rollback testnet probe instructions
- cfb3407 Add atomic rollback testnet probe client

Probe program additions:

- RollbackProbe state account
- initialize_rollback_probe instruction
- write_then_fail instruction
- IntentionalRollbackFailure error

Probe client addition:

- tests/atomic_rollback_probe.ts

Local checks:

- cargo test passed
- anchor build passed

Runtime attempt result:

The X1 testnet runtime evidence was not collected yet.

The updated program could not be deployed to X1 testnet during this attempt because deploy/upgrade failed with RPC/testnet reliability errors:

- Blockhash expired
- Max retries exceeded
- 429 Too Many Requests

After the failed deploy, the client test still reached the old deployed program version and failed with:

- InstructionFallbackNotFound

Interpretation:

This does not prove or disprove EV-01 or EV-02.

It only proves that the rollback probe code and client were prepared locally, while runtime evidence remains blocked by X1 testnet deployment/RPC availability.

Status:

- EV-01 remains open
- EV-02 remains open



## Theo sanity-check on X1 testnet deployment blocker

Theo reviewed the X1 testnet rollback probe blocker and confirmed the interpretation:

- InstructionFallbackNotFound means the deployed on-chain program does not contain the new instruction discriminator.
- The rollback test client was most likely reaching the old deployed program.
- The failed upgrade/deploy is the blocker, not the rollback test logic.
- Repeated failed upgrades should not leave partial program state, because Solana/Anchor-style program upgrade is atomic.
- The practical risk is leftover buffer accounts, not partially upgraded program logic.

Observed deployment blockers:

- Blockhash expired
- Max retries exceeded
- 429 Too Many Requests

Theo's recommended next steps, ranked by practicality:

1. Use a fresh program ID for the rollback probe instead of upgrading the old program.
2. Build release / smaller program if possible.
3. If public X1 RPC remains unstable, try another RPC endpoint.
4. If deploy still fails, use buffer-based deploy as a fallback.
5. Local validator can provide faster feedback, but it would not count as X1 testnet runtime evidence.

Current conclusion:

The local probe remains prepared, but EV-01 and EV-02 remain open until the rollback scenario is successfully executed against X1 testnet runtime.



## X1 testnet rollback probe result

The EV-01 / EV-02 rollback probe was successfully executed against X1 testnet runtime.

Fresh probe program:

- Program ID: 9tCJe4M1MJQtE1gDxNYNE75fNUGpSAKiX56rgUMR8984
- Deploy signature: 63FmJHD7KGV1n6Z8zyWn9qkvhDpagi8tHkWd5MUWYU4Ppdg2JaYyZY6WbH5XhuZ9P7UvXERqhTK8oyyaFkz4ZA1q

Probe execution:

- command: yarn ts-mocha -p ./tsconfig.json -t 1000000 tests/atomic_rollback_probe.ts
- result: 1 passing

Observed test result:

- EV-01 / EV-02 X1 atomic rollback probe
- keeps rollback probe state unchanged after intentional failure
- 1 passing

What the probe verified:

1. initialize_rollback_probe created and initialized a RollbackProbe account.
2. write_then_fail attempted to mutate the same account.
3. write_then_fail intentionally returned an error after mutation.
4. The client re-read the account after the failed transaction.
5. The account state remained equal to the initial state.

Interpretation:

This provides X1 testnet runtime evidence that a failed instruction transaction rolls back account writes in this probe scenario.

Status:

- EV-01 transaction-level atomicity: confirmed by X1 testnet probe
- EV-02 account write rollback: confirmed by X1 testnet probe

Scope:

This is runtime evidence for the rollback behavior needed before gateway design can continue.

It is not a production gateway implementation, not bridge logic, not mint authority logic, and not a relayer/watcher implementation.

## Current status

Status: planned.

No X1 testnet runtime evidence has been collected in this branch yet.

EV-01 and EV-02 remain open.
