# Stage 4.4 Relayer Dry-Run No-Send Boundary Evidence

This document records Stage 4.4 relayer dry-run / no-send boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-4-relayer-dry-run-no-send-boundary

Runtime commit:

    5b3be68 Add Stage 4.4 relayer dry-run no-send boundary

Base runtime commit:

    c5b77cf Add Stage 4.3 watcher read-only observation boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 established the redacted live config boundary.

Stage 4.2 established the read-only RPC connectivity boundary.

Stage 4.3 established the watcher read-only observation boundary.

Stage 4.4 adds the relayer dry-run / no-send boundary.

## Scope

Stage 4.4 introduces a relayer planning boundary.

It is dry-run only.

It is no-send only.

It builds an unsigned relayer plan.

It does not load a wallet.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

It does not perform live-send.

It does not deploy anything.

The runtime tests use an injected planner.

The boundary proves that relayer-style planning can consume Stage 4.3 watcher observation evidence and produce a deterministic unsigned no-send plan without wallet loading, signing, transaction submission, live-send, or SOL spend.

## Runtime changes

New helper:

    tests/helpers/stage4RelayerDryRunNoSendPrototype.ts

New test:

    tests/stage4_relayer_dry_run_no_send_boundary.test.ts

## Dependency on Stage 4.3

Stage 4.4 consumes the Stage 4.3 watcher read-only observation result.

Required source artifact:

    stage4_watcher_read_only_observation_result

Required source stage:

    4.3

Required source execution mode:

    watcher_read_only_no_wallet

Required source condition:

    source observation ok must be true

Stage 4.4 rejects failed observation evidence.

## Allowed relayer dry-run operations

New type:

    Stage4RelayerDryRunOperation

Allowed operations:

    validateMintIntent
    deriveInstructionPlan
    estimateUnsignedMessage

Non-dry-run operations are rejected.

Example rejected operation:

    sendTransaction

## New relayer dry-run step code model

New type:

    Stage4RelayerDryRunStepCode

Step codes:

    mint_intent_validation
    instruction_plan_derivation
    unsigned_message_estimation

## New mint intent model

New type:

    Stage4RelayerMintIntent

Fields:

    routeId
    canonicalEventKey
    x1RecipientHash
    burnedAmount
    xxxlMintAmount
    mintToken

Validation behavior:

- routeId must be a non-empty safe string
- canonicalEventKey must be a non-empty safe string
- x1RecipientHash must be a non-empty safe string
- burnedAmount must be a decimal string
- xxxlMintAmount must be a decimal string
- xxxlMintAmount must be greater than zero
- mintToken must be a non-empty safe string
- forbidden secret-bearing markers are rejected

## New planner model

New type:

    Stage4RelayerDryRunPlannerRequest

Fields:

    operation
    programId
    payerPublicKey
    mintIntent
    id

New type:

    Stage4RelayerDryRunPlannerResponse

Fields:

    ok
    hasPlan
    unsignedOnly
    planKind
    errorMessage

New type:

    Stage4RelayerDryRunPlanner

The planner is injected.

This keeps the boundary testable without forcing wallet access, signing, or transaction submission.

## New relayer dry-run step model

New type:

    Stage4RelayerDryRunStep

Fields:

    code
    operation
    ok
    hasPlan
    unsignedOnly
    planKind
    errorMessage

Plan kinds:

    validation
    instruction_plan
    unsigned_message

Every step must remain unsignedOnly.

If a planner returns a signed or sendable plan, Stage 4.4 rejects it.

## New result artifact

New type:

    Stage4RelayerDryRunNoSendResult

Artifact type:

    stage4_relayer_dry_run_no_send_result

Schema version:

    1

Stage:

    4.4

Execution mode:

    relayer_dry_run_no_send

Fields:

    plannedAtIso
    networkName
    programId
    payerPublicKey
    sourceObservationStage
    sourceObservationOk
    mintIntent
    steps
    unsignedPlan
    invariants
    ok

## Unsigned plan model

The result includes an unsigned plan:

    instructionName: mint_xxxl_from_gateway_message
    signerCount: 0
    transactionSubmission: not_allowed
    walletRequired: false
    signatureRequired: false
    solSpendAllowed: false

This explicitly prevents live-send behavior from entering Stage 4.4.

## Invariants

Stage 4.4 result invariants:

    noWalletLoaded: true
    noSigning: true
    noTransactionsSubmitted: true
    noSolSpend: true
    noLiveSend: true
    dryRunOnly: true

All invariants must remain true.

## New error type

New class:

    Stage4RelayerDryRunNoSendError

New reason type:

    Stage4RelayerDryRunNoSendErrorReason

Reasons:

    invalid_planned_at_iso
    invalid_observation_result
    observation_not_ok
    invalid_mint_intent
    forbidden_value
    invalid_relayer_operation
    planner_failed
    planner_returned_signed_or_sendable_plan

## New helpers

Operation assertion helper:

    assertStage4RelayerDryRunOperationPrototype

Dry-run runner:

    runStage4RelayerDryRunNoSendPrototype

Result checker:

    checkStage4RelayerDryRunNoSendResultPrototype

## Secret, signing, and transaction boundary

Stage 4.4 rejects values containing forbidden markers such as:

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
- signTransaction
- signedTransaction

These strings are defensive markers only.

No real secret values are introduced.

No signed transaction is introduced.

No sendable transaction is introduced.

## Successful relayer dry-run test

Confirmed behavior:

- builds an unsigned relayer dry-run plan
- consumes Stage 4.3 watcher observation evidence
- uses injected planner
- does not load wallet
- does not sign
- does not submit transactions
- does not spend SOL
- does not perform live-send
- calls only validateMintIntent, deriveInstructionPlan, and estimateUnsignedMessage
- preserves networkName
- preserves programId
- preserves payerPublicKey
- sourceObservationStage is 4.3
- sourceObservationOk is true
- unsignedPlan has signerCount 0
- unsignedPlan has transactionSubmission not_allowed
- unsignedPlan has walletRequired false
- unsignedPlan has signatureRequired false
- unsignedPlan has solSpendAllowed false
- all dry-run steps are unsignedOnly
- all invariants are true
- checkStage4RelayerDryRunNoSendResultPrototype returns true

## Safe result JSON test

Confirmed behavior:

- relayer dry-run result JSON does not contain wallet path
- relayer dry-run result JSON does not contain secret-bearing markers
- relayer dry-run result JSON does not contain sendTransaction
- relayer dry-run result JSON does not contain signTransaction
- relayer dry-run result JSON does not contain signedTransaction

Secret-bearing field checks include:

- PRIVATE_KEY
- MNEMONIC
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY

## Rejection test

Confirmed behavior:

- bad plannedAtIso is rejected as invalid_planned_at_iso
- failed observation result is rejected as observation_not_ok
- zero xxxlMintAmount is rejected as invalid_mint_intent
- mint intent value containing privateKey marker is rejected as forbidden_value
- sendTransaction relayer operation is rejected as invalid_relayer_operation
- planner returning unsignedOnly false is rejected as planner_returned_signed_or_sendable_plan
- planner throw is rejected as planner_failed

## Stage 4.4 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts

Result:

    Stage 4.4 relayer dry-run no-send boundary
      ✔ builds an unsigned relayer dry-run plan without wallet loading, signing, sending, or SOL spend
      ✔ keeps relayer dry-run result JSON free of wallet paths, secret markers, and send/sign transaction methods
      ✔ rejects malformed metadata, failed observation, bad mint intent, forbidden values, live operations, signed plans, and planner failures

    3 passing

## Stage 4.1 plus Stage 4.2 plus Stage 4.3 plus Stage 4.4 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts

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

    Stage 4.4 relayer dry-run no-send boundary
      ✔ builds an unsigned relayer dry-run plan without wallet loading, signing, sending, or SOL spend
      ✔ keeps relayer dry-run result JSON free of wallet paths, secret markers, and send/sign transaction methods
      ✔ rejects malformed metadata, failed observation, bad mint intent, forbidden values, live operations, signed plans, and planner failures

    12 passing

## Stage 3.10 plus Stage 4.1 plus Stage 4.2 plus Stage 4.3 plus Stage 4.4 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts

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

    Stage 4.4 relayer dry-run no-send boundary
      ✔ builds an unsigned relayer dry-run plan without wallet loading, signing, sending, or SOL spend
      ✔ keeps relayer dry-run result JSON free of wallet paths, secret markers, and send/sign transaction methods
      ✔ rejects malformed metadata, failed observation, bad mint intent, forbidden values, live operations, signed plans, and planner failures

    15 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

No signed transaction material was introduced.

No sendable transaction material was introduced.

## Boundary classification

Stage 4.4 is:

    relayer dry-run no-send boundary
    unsigned plan boundary
    injected planner
    no-wallet-loading
    no-signing
    no-transaction-submission
    no-SOL-spend
    no-live-send

Stage 4.4 is not:

    guardian signing boundary
    wallet access boundary
    transaction preflight boundary
    transaction simulation boundary
    live-send boundary
    deployment boundary

## Current conclusion

Stage 4.4 establishes the relayer dry-run / no-send boundary.

It proves that relayer-style planning can be modeled as an unsigned, non-sendable dry-run plan using Stage 4.3 watcher observation evidence and an injected planner, while preserving the no-wallet, no-signing, no-transaction-submission, no-SOL-spend, and no-live-send invariants.

The next valid stage is Stage 4.5 guardian operation policy boundary.
