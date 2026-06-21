# Stage 4.3 Watcher Read-Only Observation Boundary Evidence

This document records Stage 4.3 watcher read-only observation boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-3-watcher-read-only-observation-boundary

Runtime commit:

    c5b77cf Add Stage 4.3 watcher read-only observation boundary

Base runtime commit:

    eb6ef26 Add Stage 4.2 read-only RPC connectivity boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 established the redacted live config boundary.

Stage 4.2 established the read-only RPC connectivity boundary.

Stage 4.3 adds the watcher runtime read-only observation boundary.

## Scope

Stage 4.3 introduces a watcher observation boundary.

It is a single read-only observation cycle.

It is not a continuous watcher loop.

It does not load a wallet.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

It does not start relayer loops.

It does not deploy anything.

The runtime tests use an injected observation source.

The boundary proves that watcher observation can consume Stage 4.2 read-only connectivity evidence and produce a deterministic observation result without wallet loading, signing, transaction submission, or SOL spend.

## Runtime changes

New helper:

    tests/helpers/stage4WatcherReadOnlyObservationPrototype.ts

New test:

    tests/stage4_watcher_read_only_observation_boundary.test.ts

## Dependency on Stage 4.2

Stage 4.3 consumes the Stage 4.2 read-only RPC connectivity result.

Required source artifact:

    stage4_read_only_rpc_connectivity_result

Required source stage:

    4.2

Required source execution mode:

    read_only_rpc_no_wallet

Required source mode:

    read_only

Required source condition:

    source connectivity ok must be true

Stage 4.3 rejects failed connectivity evidence.

## Allowed watcher read-only methods

New type:

    Stage4WatcherReadOnlyMethod

Allowed methods:

    getSlot
    getBlockHeight
    getAccountInfo
    getBalance

Non-read-only methods are rejected.

Example rejected method:

    sendTransaction

## New watcher observation code model

New type:

    Stage4WatcherObservationCode

Observation codes:

    slot_observation
    block_height_observation
    program_account_observation
    payer_balance_observation

## New watcher observation source model

New type:

    Stage4WatcherObservationSourceRequest

Fields:

    method
    params
    id

New type:

    Stage4WatcherObservationSourceResponse

Fields:

    ok
    hasResult
    observedValueKind
    errorMessage

New type:

    Stage4WatcherObservationSource

The observation source is injected.

This keeps the boundary testable without forcing a live watcher loop.

## New watcher observation model

New type:

    Stage4WatcherObservation

Fields:

    code
    method
    ok
    hasResult
    observedValueKind
    errorMessage

Observed value kinds:

    slot
    block_height
    account
    balance

## New result artifact

New type:

    Stage4WatcherReadOnlyObservationResult

Artifact type:

    stage4_watcher_read_only_observation_result

Schema version:

    1

Stage:

    4.3

Execution mode:

    watcher_read_only_no_wallet

Fields:

    observedAtIso
    networkName
    programId
    payerPublicKey
    sourceConnectivityStage
    sourceConnectivityOk
    observations
    checkpoint
    invariants
    ok

## Checkpoint model

The watcher observation result includes a checkpoint:

    observationCount
    successfulObservationCount
    firstMethod
    lastMethod

Expected successful checkpoint:

    observationCount: 4
    successfulObservationCount: 4
    firstMethod: getSlot
    lastMethod: getBalance

## Invariants

Stage 4.3 result invariants:

    noWalletLoaded: true
    noSigning: true
    noTransactions: true
    noSolSpend: true
    readOnlyObservationOnly: true
    noContinuousLoop: true

All invariants must remain true.

## New error type

New class:

    Stage4WatcherReadOnlyObservationError

New reason type:

    Stage4WatcherReadOnlyObservationErrorReason

Reasons:

    invalid_observed_at_iso
    invalid_connectivity_result
    connectivity_not_ok
    forbidden_connectivity_value
    invalid_watcher_method
    observation_source_failed

## New helpers

Method assertion helper:

    assertStage4WatcherReadOnlyMethodPrototype

Observation runner:

    runStage4WatcherReadOnlyObservationPrototype

Result checker:

    checkStage4WatcherReadOnlyObservationResultPrototype

## Secret and transaction boundary

Stage 4.3 rejects values containing forbidden markers such as:

- PRIVATE_KEY
- MNEMONIC
- SEED_PHRASE
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY
- secretKey
- privateKey
- mnemonic
- seed phrase
- seed_phrase
- wallet json
- wallet.json
- bearer
- api_key
- rpc_api_key
- guardianSigners
- sendTransaction

These strings are defensive markers only.

No real secret values are introduced.

## Successful watcher observation test

Confirmed behavior:

- runs one watcher read-only observation cycle
- consumes Stage 4.2 connectivity evidence
- uses injected observation source
- does not load wallet
- does not sign
- does not submit transactions
- does not spend SOL
- does not run a continuous watcher loop
- calls only getSlot, getBlockHeight, getAccountInfo, and getBalance
- preserves networkName
- preserves programId
- preserves payerPublicKey
- sourceConnectivityStage is 4.2
- sourceConnectivityOk is true
- checkpoint has four observations
- all observations are successful
- all invariants are true
- checkStage4WatcherReadOnlyObservationResultPrototype returns true

## Redacted / safe result JSON test

Confirmed behavior:

- watcher observation result JSON does not contain wallet path
- watcher observation result JSON does not contain secret-bearing markers
- watcher observation result JSON does not contain sendTransaction

Secret-bearing field checks include:

- PRIVATE_KEY
- MNEMONIC
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY

## Rejection test

Confirmed behavior:

- bad observedAtIso is rejected as invalid_observed_at_iso
- failed connectivity result is rejected as connectivity_not_ok
- connectivity value containing privateKey marker is rejected as forbidden_connectivity_value
- sendTransaction watcher method is rejected as invalid_watcher_method
- observation source throw is rejected as observation_source_failed

## Stage 4.3 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_watcher_read_only_observation_boundary.test.ts

Result:

    Stage 4.3 watcher read-only observation boundary
      ✔ runs one watcher read-only observation cycle without wallet loading, signing, transactions, or SOL spend
      ✔ keeps watcher observation result JSON free of wallet paths, secret markers, and transaction methods
      ✔ rejects malformed metadata, failed connectivity, forbidden values, non-read-only watcher methods, and source failures

    3 passing

## Stage 4.1 plus Stage 4.2 plus Stage 4.3 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts

Result:

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    Stage 4.3 watcher read-only observation boundary
      ✔ runs one watcher read-only observation cycle without wallet loading, signing, transactions, or SOL spend
      ✔ keeps watcher observation result JSON free of wallet paths, secret markers, and transaction methods
      ✔ rejects malformed metadata, failed connectivity, forbidden values, non-read-only watcher methods, and source failures

    9 passing

## Stage 3.10 plus Stage 4.1 plus Stage 4.2 plus Stage 4.3 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts

Result:

    Stage 3.10 final closure boundary
      ✔ creates a final Stage 3 closure boundary from Stage 3.1 through Stage 3.9 evidence
      ✔ rejects missing, duplicate, unordered, unclosed, or non-offline evidence entries
      ✔ rejects malformed closure metadata, failed invariants, and forbidden values

    Stage 4.1 redacted live config boundary
      ✔ parses a read-only live config shape without RPC calls or wallet loading
      ✔ creates a redacted public view that never prints live config secrets or wallet paths
      ✔ rejects missing, malformed, live-send, and forbidden secret-bearing config values

    Stage 4.2 read-only RPC connectivity boundary
      ✔ runs only read-only RPC checks without wallet loading, signing, transactions, or SOL spend
      ✔ keeps public result JSON redacted and free of wallet paths or secret-bearing markers
      ✔ rejects malformed metadata, live-send configs, forbidden values, non-read-only methods, and transport failures

    Stage 4.3 watcher read-only observation boundary
      ✔ runs one watcher read-only observation cycle without wallet loading, signing, transactions, or SOL spend
      ✔ keeps watcher observation result JSON free of wallet paths, secret markers, and transaction methods
      ✔ rejects malformed metadata, failed connectivity, forbidden values, non-read-only watcher methods, and source failures

    12 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

## Boundary classification

Stage 4.3 is:

    watcher read-only observation boundary
    single observation cycle
    injected source
    no-wallet-loading
    no-signing
    no-transaction
    no-SOL-spend
    no-continuous-loop

Stage 4.3 is not:

    continuous watcher runtime loop
    relayer runtime loop
    transaction construction
    transaction simulation
    dry-run send
    live-send
    deployment

## Current conclusion

Stage 4.3 establishes the watcher read-only observation boundary.

It proves that watcher-style observation can be modeled as a single read-only observation cycle using Stage 4.2 connectivity evidence and an injected source, while preserving the no-wallet, no-signing, no-transaction, no-SOL-spend, and no-continuous-loop invariants.

The next valid stage is Stage 4.4 relayer dry-run / no-send boundary.
