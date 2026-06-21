# Stage 4.7 Fixed Guardian Set Quorum Boundary Evidence

This document records Stage 4.7 fixed guardian set quorum boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-7-fixed-guardian-set-quorum-boundary

Runtime commit:

    f63397f Add Stage 4.7 fixed guardian set quorum boundary

Base runtime commit:

    5c967c0 Add Stage 4.6 transaction preflight no-send boundary

## Stage position

Stage 4.1 established the redacted live config boundary.

Stage 4.2 established the read-only RPC connectivity boundary.

Stage 4.3 established the watcher read-only observation boundary.

Stage 4.4 established the relayer dry-run / no-send boundary.

Stage 4.5 established guardian operation policy.

Stage 4.6 established transaction preflight / no-send.

Stage 4.7 now fixes the guardian set model:

    guardian_count = 5
    quorum_threshold = 3
    guardian_set_version = 1

## Scope

Stage 4.7 is a fixed guardian quorum policy boundary.

It is fully offline.

It does not use live RPC.

It does not load a wallet.

It does not access guardian private keys.

It does not sign anything.

It does not verify cryptographic signatures yet.

It does not submit transactions.

It does not spend SOL.

It fixes the guardian set shape before gateway fee policy and before guardian approval messages with fee-bound data.

## Runtime changes

New helper:

    tests/helpers/stage4FixedGuardianSetQuorumPrototype.ts

New test:

    tests/stage4_fixed_guardian_set_quorum_boundary.test.ts

## Dependency on Stage 4.5

Stage 4.7 consumes Stage 4.5 guardian operation policy evidence.

Required source artifact:

    stage4_guardian_operation_policy_result

Required source stage:

    4.5

Required source execution mode:

    guardian_policy_no_key_material

Required source condition:

    guardian policy result ok must be true

Stage 4.7 rejects failed guardian policy evidence.

## Fixed guardian set

Stage 4.7 fixes:

    guardianCount: 5
    quorumThreshold: 3
    guardianSetVersion: 1

This means:

- 2-of-5 is not enough
- 3-of-5 is enough
- 4-of-5 is enough
- 5-of-5 is enough

The model uses guardian public keys only.

Guardian private keys are never introduced.

## Allowed guardian quorum operations

New type:

    Stage4FixedGuardianQuorumOperation

Allowed operations:

    validateGuardianSet
    evaluateApprovalQuorum
    recordGuardianSetVersion

Rejected example operation:

    signTransaction

## Approval model

New type:

    Stage4FixedGuardianApproval

Fields:

    guardianPublicKey
    guardianSetVersion
    approvalId

Important: this is not a cryptographic signature yet.

This is the fixed quorum identity model.

Signature verification remains out of scope for this stage.

## Duplicate and unknown guardian policy

Duplicate guardian approval behavior:

    duplicate_approval is rejected

Unknown guardian approval behavior:

    unknown_guardian is rejected

A guardian public key can only count once.

A public key outside the fixed guardian set cannot count.

## New result artifact

New type:

    Stage4FixedGuardianSetQuorumResult

Artifact type:

    stage4_fixed_guardian_set_quorum_result

Schema version:

    1

Stage:

    4.7

Execution mode:

    fixed_guardian_set_quorum_policy

Fields:

    evaluatedAtIso
    sourceGuardianPolicyStage
    sourceGuardianPolicyOk
    guardianSetVersion
    guardianCount
    quorumThreshold
    guardianPublicKeys
    approvalCount
    uniqueKnownApprovalCount
    quorumReached
    steps
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    fixedGuardianCount: 5
    fixedQuorumThreshold: 3
    approvalIdentityMode: guardian_public_key
    duplicateApprovalHandling: count_once
    unknownGuardianHandling: reject
    guardianSetVersionRequired: 1
    privateKeyAccess: not_allowed
    signing: not_performed
    transactionSubmission: not_allowed

## Invariants

Stage 4.7 invariants:

    exactlyFiveGuardians: true
    threeOfFiveQuorum: true
    publicKeysOnly: true
    noDuplicateApprovalCounting: true
    noUnknownGuardianAccepted: true
    noPrivateKeys: true
    noSigning: true
    noTransactionsSubmitted: true

All invariants must remain true.

## Error model

New class:

    Stage4FixedGuardianSetQuorumError

New reason type:

    Stage4FixedGuardianSetQuorumErrorReason

Reasons:

    invalid_evaluated_at_iso
    invalid_guardian_policy_result
    guardian_policy_not_ok
    invalid_guardian_set
    invalid_quorum_threshold
    invalid_guardian_set_version
    invalid_approval
    duplicate_approval
    unknown_guardian
    forbidden_value
    invalid_quorum_operation

## New helpers

Operation assertion helper:

    assertStage4FixedGuardianQuorumOperationPrototype

Quorum runner:

    runStage4FixedGuardianSetQuorumPrototype

Result checker:

    checkStage4FixedGuardianSetQuorumResultPrototype

## Secret and transaction boundary

Stage 4.7 rejects values containing forbidden markers such as:

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

No transaction submission path is introduced.

No serialized transaction material is introduced.

## Successful fixed quorum test

Confirmed behavior:

- accepts exactly 5 guardians
- requires 3-of-5 quorum
- uses public keys only
- fixes guardianSetVersion to 1
- confirms guardianCount is 5
- confirms quorumThreshold is 3
- confirms 3 unique known approvals reach quorum
- confirms the result is ok when quorum is reached
- preserves policy-only behavior
- does not perform signature verification
- does not access private keys
- does not sign
- does not submit transactions
- checkStage4FixedGuardianSetQuorumResultPrototype returns true

## Quorum threshold test

Confirmed behavior:

- 2-of-5 does not reach quorum
- 3-of-5 reaches quorum
- 4-of-5 reaches quorum
- 5-of-5 reaches quorum

## Safe result JSON test

Confirmed behavior:

- fixed guardian quorum result JSON does not contain wallet path
- fixed guardian quorum result JSON does not contain private key markers
- fixed guardian quorum result JSON does not contain signing methods
- fixed guardian quorum result JSON does not contain transaction submission methods

Forbidden method and marker checks include:

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

- malformed evaluatedAtIso is rejected as invalid_evaluated_at_iso
- failed guardian policy is rejected as guardian_policy_not_ok
- guardian set with 4 keys is rejected as invalid_guardian_set
- quorum threshold 2 is rejected as invalid_quorum_threshold
- guardian set version 2 is rejected as invalid_guardian_set_version
- duplicate approval is rejected as duplicate_approval
- unknown guardian is rejected as unknown_guardian
- approval value containing privateKey marker is rejected as forbidden_value
- signTransaction operation is rejected as invalid_quorum_operation

## Stage 4.7 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts

Result:

    Stage 4.7 fixed guardian set quorum boundary
      ✔ accepts a fixed 5 guardian set with 3-of-5 quorum using public keys only
      ✔ rejects 2-of-5 while accepting 3-of-5, 4-of-5, and 5-of-5
      ✔ keeps fixed guardian quorum result JSON free of private keys, wallet paths, signing, and transaction submission methods
      ✔ rejects malformed guardian set, wrong threshold, wrong version, duplicate approval, unknown guardian, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.7 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts

Result:

    22 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.7 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts

Result:

    25 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

## Boundary classification

Stage 4.7 is:

    fixed guardian set quorum boundary
    5 guardian boundary
    3-of-5 quorum boundary
    guardian set version boundary
    duplicate approval rejection boundary
    unknown guardian rejection boundary
    public-keys-only boundary
    offline model boundary

Stage 4.7 is not:

    cryptographic signature verification boundary
    guardian key loading boundary
    guardian signing boundary
    wallet access boundary
    transaction preflight boundary
    transaction simulation boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.7 establishes a fixed 5 guardian / 3-of-5 quorum model.

It proves that a fixed guardian public key set can be modeled offline with quorum threshold, duplicate rejection, unknown guardian rejection, guardian set versioning, and no-secret/no-signing/no-transaction invariants.

The next valid stage is Stage 4.8 gateway fee policy boundary.
