# Stage 4.6 Transaction Preflight No-Send Boundary Evidence

This document records Stage 4.6 transaction preflight / no-send boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-6-transaction-preflight-no-send-boundary

Runtime commit:

    5c967c0 Add Stage 4.6 transaction preflight no-send boundary

Base runtime commit:

    93665db Add Stage 4.5 guardian operation policy boundary

## Stage position

Stage 1 is closed as the deterministic model layer.

Stage 2 is closed as the runtime / evidence layer.

Stage 3 is closed as the offline tooling / production surface layer.

Stage 4.0 defined the live runtime charter.

Stage 4.1 established the redacted live config boundary.

Stage 4.2 established the read-only RPC connectivity boundary.

Stage 4.3 established the watcher read-only observation boundary.

Stage 4.4 established the relayer dry-run / no-send boundary.

Stage 4.5 established the guardian operation policy boundary.

Stage 4.6 adds the transaction preflight / no-send boundary.

## Scope

Stage 4.6 introduces a transaction preflight model.

It is preflight-only.

It is no-send only.

It builds an unsigned transaction envelope.

It does not create a serialized transaction.

It does not load a wallet.

It does not sign anything.

It does not simulate a transaction.

It does not submit a transaction.

It does not spend SOL.

It does not perform live-send.

It does not deploy anything.

The boundary proves that the future transaction path can be modeled as an unsigned, non-sendable, non-simulated preflight envelope before wallet access, signing, simulation, or live submission are introduced.

## Runtime changes

New helper:

    tests/helpers/stage4TransactionPreflightNoSendPrototype.ts

New test:

    tests/stage4_transaction_preflight_no_send_boundary.test.ts

## Dependency on Stage 4.5

Stage 4.6 consumes the Stage 4.5 guardian operation policy result.

Required source artifact:

    stage4_guardian_operation_policy_result

Required source stage:

    4.5

Required source execution mode:

    guardian_policy_no_key_material

Required source condition:

    guardian policy result ok must be true

Stage 4.6 rejects failed guardian policy evidence.

Stage 4.6 also requires the Stage 4.5 no-secret / no-wallet / no-signing / no-send protections:

    keyMaterialHandling: public_keys_only
    privateKeyAccess: not_allowed
    walletLoading: not_allowed
    signingAuthorization: policy_only_not_signature
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Allowed transaction preflight operations

New type:

    Stage4TransactionPreflightOperation

Allowed operations:

    validateGuardianPolicy
    buildNoSendPreflightEnvelope
    runNoSendPreflightChecks

Live-send, signing, and serialized transaction operations are rejected.

Example rejected operation:

    sendTransaction

## New transaction preflight step code model

New type:

    Stage4TransactionPreflightStepCode

Step codes:

    guardian_policy_validation
    unsigned_transaction_envelope
    no_send_preflight_checks

## New account meta model

New type:

    Stage4TransactionAccountMeta

Fields:

    pubkey
    isSigner
    isWritable
    role

Allowed account roles:

    program
    payer
    mint
    recipient
    processed_burn_registry
    guardian_quorum

Important invariant:

    isSigner must remain false for all account metas

## New unsigned transaction envelope model

New type:

    Stage4UnsignedTransactionEnvelope

Fields:

    instructionName
    programId
    payerPublicKey
    accountMetas
    signerCount
    requiredSignatureCount
    serializedTransaction
    transactionSubmission
    simulation

Required envelope values:

    instructionName: mint_xxxl_from_gateway_message
    signerCount: 0
    requiredSignatureCount: 0
    serialized transaction status: <not_created:no_wallet_no_signing>
    transactionSubmission: not_allowed
    simulation: not_performed

The envelope is intentionally unsigned and non-sendable.

No serialized transaction is created.

## New preflight planner model

New type:

    Stage4TransactionPreflightPlannerRequest

Fields:

    operation
    programId
    payerPublicKey
    guardianPublicKeyCount
    quorumThreshold
    id

New type:

    Stage4TransactionPreflightPlannerResponse

Fields:

    ok
    hasPreflightResult
    unsignedOnly
    noSend
    preflightKind
    errorMessage

New type:

    Stage4TransactionPreflightPlanner

The planner is injected.

The planner must return:

    unsignedOnly: true
    noSend: true

If a planner returns a signed or sendable preflight, Stage 4.6 rejects it.

## New transaction preflight step model

New type:

    Stage4TransactionPreflightStep

Fields:

    code
    operation
    ok
    hasPreflightResult
    unsignedOnly
    noSend
    preflightKind
    errorMessage

Preflight kinds:

    guardian_policy_validation
    unsigned_transaction_envelope
    no_send_checks

Every step must remain unsignedOnly and noSend.

## New result artifact

New type:

    Stage4TransactionPreflightNoSendResult

Artifact type:

    stage4_transaction_preflight_no_send_result

Schema version:

    1

Stage:

    4.6

Execution mode:

    transaction_preflight_no_send

Fields:

    preflightAtIso
    networkName
    programId
    payerPublicKey
    sourceGuardianPolicyStage
    sourceGuardianPolicyOk
    guardianPublicKeyCount
    quorumThreshold
    unsignedEnvelope
    steps
    policy
    invariants
    ok

## Policy model

The result includes a policy object:

    preflightOnly: true
    walletLoading: not_allowed
    signing: not_performed
    transactionSubmission: not_allowed
    simulation: not_performed
    solSpendAllowed: false

This explicitly prevents transaction preflight from becoming signing, simulation, or live-send.

## Invariants

Stage 4.6 result invariants:

    noWalletLoaded: true
    noSigning: true
    noTransactionsSubmitted: true
    noSolSpend: true
    noLiveSend: true
    noSerializedTransaction: true
    preflightOnly: true

All invariants must remain true.

## New error type

New class:

    Stage4TransactionPreflightNoSendError

New reason type:

    Stage4TransactionPreflightNoSendErrorReason

Reasons:

    invalid_preflight_at_iso
    invalid_guardian_policy_result
    guardian_policy_not_ok
    forbidden_value
    invalid_preflight_operation
    planner_failed
    planner_returned_signed_or_sendable_preflight
    invalid_unsigned_envelope

## New helpers

Operation assertion helper:

    assertStage4TransactionPreflightOperationPrototype

Preflight runner:

    runStage4TransactionPreflightNoSendPrototype

Result checker:

    checkStage4TransactionPreflightNoSendResultPrototype

## Secret, signing, transaction, and serialization boundary

Stage 4.6 rejects values containing forbidden markers such as:

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
- signMessage
- exportPrivateKey
- serializedTransaction

These strings are defensive markers only.

No real secret values are introduced.

No guardian private key material is introduced.

No wallet-loading path is introduced.

No signing path is introduced.

No simulation path is introduced.

No transaction submission path is introduced.

No serialized transaction is introduced.

## False-positive safety note

An earlier draft used the operation name:

    buildUnsignedTransactionEnvelope

That name contained the substring:

    signedTransaction

inside the word:

    UnsignedTransaction

The JSON safety test correctly flagged this as a forbidden substring.

The operation was renamed to:

    buildNoSendPreflightEnvelope

This preserves the safety check without weakening it.

## Successful transaction preflight test

Confirmed behavior:

- builds a no-send unsigned transaction preflight envelope
- consumes Stage 4.5 guardian operation policy evidence
- uses injected preflight planner
- does not load wallet
- does not sign
- does not simulate
- does not submit transactions
- does not spend SOL
- does not perform live-send
- does not create serialized transaction material
- calls only validateGuardianPolicy, buildNoSendPreflightEnvelope, and runNoSendPreflightChecks
- preserves networkName
- preserves programId
- preserves payerPublicKey
- sourceGuardianPolicyStage is 4.5
- sourceGuardianPolicyOk is true
- guardianPublicKeyCount is 2
- quorumThreshold is 2
- unsignedEnvelope signerCount is 0
- unsignedEnvelope requiredSignatureCount is 0
- unsignedEnvelope serializedTransaction is <not_created:no_wallet_no_signing>
- unsignedEnvelope transactionSubmission is not_allowed
- unsignedEnvelope simulation is not_performed
- every account meta has isSigner false
- all preflight steps are unsignedOnly and noSend
- all invariants are true
- checkStage4TransactionPreflightNoSendResultPrototype returns true

## Safe result JSON test

Confirmed behavior:

- transaction preflight result JSON does not contain wallet path
- transaction preflight result JSON does not contain secret-bearing markers
- transaction preflight result JSON does not contain serialized transaction material
- transaction preflight result JSON does not contain send/sign methods

Secret-bearing and forbidden action checks include:

- PRIVATE_KEY
- MNEMONIC
- SECRET_KEY
- RPC_API_KEY
- GUARDIAN_PRIVATE_KEY
- DEPLOYER_PRIVATE_KEY
- sendTransaction
- signTransaction
- signedTransaction
- signMessage
- exportPrivateKey

## Rejection test

Confirmed behavior:

- bad preflightAtIso is rejected as invalid_preflight_at_iso
- failed guardian policy result is rejected as guardian_policy_not_ok
- guardian policy value containing privateKey marker is rejected as forbidden_value
- sendTransaction preflight operation is rejected as invalid_preflight_operation
- planner returning unsignedOnly false is rejected as planner_returned_signed_or_sendable_preflight
- planner throw is rejected as planner_failed

## Stage 4.6 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts

Result:

    Stage 4.6 transaction preflight no-send boundary
      ✔ builds a no-send unsigned transaction preflight envelope without wallet loading, signing, simulation, submission, or SOL spend
      ✔ keeps transaction preflight result JSON free of wallet paths, secret markers, serialized transactions, and send/sign methods
      ✔ rejects malformed metadata, failed guardian policy, forbidden values, live operations, signed/sendable preflight, and planner failures

    3 passing

## Stage 4.1 through Stage 4.6 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts

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

    Stage 4.5 guardian operation policy boundary
      ✔ creates a guardian policy-only result without private keys, wallet loading, signing, sending, or SOL spend
      ✔ keeps guardian policy result JSON free of wallet paths, secret markers, private key material, and send/sign methods
      ✔ rejects malformed metadata, failed dry-run, malformed guardians, bad quorum, forbidden values, and signing/send actions

    Stage 4.6 transaction preflight no-send boundary
      ✔ builds a no-send unsigned transaction preflight envelope without wallet loading, signing, simulation, submission, or SOL spend
      ✔ keeps transaction preflight result JSON free of wallet paths, secret markers, serialized transactions, and send/sign methods
      ✔ rejects malformed metadata, failed guardian policy, forbidden values, live operations, signed/sendable preflight, and planner failures

    18 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.6 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts

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

    Stage 4.5 guardian operation policy boundary
      ✔ creates a guardian policy-only result without private keys, wallet loading, signing, sending, or SOL spend
      ✔ keeps guardian policy result JSON free of wallet paths, secret markers, private key material, and send/sign methods
      ✔ rejects malformed metadata, failed dry-run, malformed guardians, bad quorum, forbidden values, and signing/send actions

    Stage 4.6 transaction preflight no-send boundary
      ✔ builds a no-send unsigned transaction preflight envelope without wallet loading, signing, simulation, submission, or SOL spend
      ✔ keeps transaction preflight result JSON free of wallet paths, secret markers, serialized transactions, and send/sign methods
      ✔ rejects malformed metadata, failed guardian policy, forbidden values, live operations, signed/sendable preflight, and planner failures

    21 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No secret-like material was introduced.

No guardian private key material was introduced.

No wallet-loading path was introduced.

No signing path was introduced.

No simulation path was introduced.

No transaction submission path was introduced.

No serialized transaction material was introduced.

## Boundary classification

Stage 4.6 is:

    transaction preflight no-send boundary
    unsigned transaction envelope boundary
    no-serialized-transaction boundary
    no-wallet-loading
    no-signing
    no-simulation
    no-transaction-submission
    no-SOL-spend
    no-live-send
    preflight-only

Stage 4.6 is not:

    wallet access boundary
    signing boundary
    transaction simulation boundary
    transaction submission boundary
    live-send boundary
    deployment boundary

## Current conclusion

Stage 4.6 establishes the transaction preflight / no-send boundary.

It proves that the future transaction path can be modeled as an unsigned, non-sendable, non-simulated preflight envelope using Stage 4.5 guardian policy evidence and an injected preflight planner, while preserving the no-wallet, no-signing, no-simulation, no-transaction-submission, no-SOL-spend, no-live-send, and no-serialized-transaction invariants.

The next valid stage is Stage 4.7 wallet access boundary.
