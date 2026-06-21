# Stage 4.10 Guardian Fee-Bound Approval Verification Boundary Evidence

This document records Stage 4.10 guardian fee-bound approval verification boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-10-guardian-fee-bound-approval-verification-boundary

Runtime commit:

    fdbc3b8 Add Stage 4.10 guardian fee-bound approval verification boundary

Base runtime commit:

    d4a7060 Add Stage 4.9 guardian fee-bound approval message boundary

## Stage position

Stage 4.7 fixed the guardian set model:

    guardian_count = 5
    quorum_threshold = 3
    guardian_set_version = 1

Stage 4.8 established the gateway fee policy model.

Stage 4.9 built the guardian approval message with gateway fee fields bound into the message digest.

Stage 4.10 now verifies guardian approvals against the exact fee-bound message digest.

This stage is still fully offline.

It does not use live RPC.

It does not load a wallet.

It does not access guardian private keys.

It does not sign anything.

It does not perform cryptographic signature verification yet.

It does not submit transactions.

It does not spend SOL.

## Scope

Stage 4.10 defines the guardian fee-bound approval verification model.

It proves that guardian approvals are accepted only when they reference the exact fee-bound message digest produced by Stage 4.9.

This prevents a design where:

- guardians approve one fee-bound message
- but a different fee amount is later applied
- or a different net amount is later applied
- or a different fee recipient is later applied
- or a different fee quote id is later applied
- or a different fee deadline is later applied
- or approvals are counted from unknown guardians
- or duplicate approvals from the same guardian are counted twice

Stage 4.10 remains a model-stage verification boundary, not a production cryptographic signature boundary.

## Runtime changes

New helper:

    tests/helpers/stage4GuardianFeeBoundApprovalVerificationPrototype.ts

New test:

    tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts

## Dependency on Stage 4.9

Stage 4.10 consumes Stage 4.9 fee-bound approval message evidence.

Required source artifact:

    stage4_guardian_fee_bound_approval_message_result

Required source stage:

    4.9

Required source execution mode:

    guardian_approval_message_fee_bound_offline

Required source conditions:

    sourceFeePolicyStage must be 4.8
    sourceFeePolicyOk must be true
    guardianSetVersion must be 1
    messageType must be STAGE4_GUARDIAN_FEE_BOUND_APPROVAL_MESSAGE
    feeBoundMessageDigestAlgorithm must be sha256_model_hash
    feeBoundMessageDigest must be a valid model digest
    signing must be not_performed
    signatureVerification must be not_performed
    walletLoading must be not_allowed
    transactionSubmission must be not_allowed
    solSpendAllowed must be false
    result ok must be true

Stage 4.10 rejects failed Stage 4.9 approval message evidence.

## Approval model

New approval type:

    Stage4GuardianFeeBoundApproval

Fields:

    guardianPublicKey
    guardianSetVersion
    messageDigest
    approvalId
    approvalKind

Required approvalKind:

    fee_bound_message_digest_approval

Each approval must reference exactly:

    Stage4GuardianFeeBoundApprovalMessageResult.feeBoundMessageDigest

Any approval referencing a different digest is rejected.

## Guardian set and quorum

Stage 4.10 keeps the fixed guardian set policy:

    guardianCount = 5
    quorumThreshold = 3
    guardianSetVersion = 1

Confirmed behavior:

    2-of-5 is rejected
    3-of-5 is accepted
    4-of-5 is accepted
    5-of-5 is accepted

Duplicate guardian approvals are rejected.

Unknown guardian approvals are rejected.

Guardian public keys are public identity only.

No private key material is used.

## New result artifact

New result type:

    Stage4GuardianFeeBoundApprovalVerificationResult

Artifact type:

    stage4_guardian_fee_bound_approval_verification_result

Schema version:

    1

Stage:

    4.10

Execution mode:

    guardian_fee_bound_approval_verification_offline

Fields:

    verifiedAtIso
    sourceApprovalMessageStage
    sourceApprovalMessageOk
    guardianSetVersion
    guardianCount
    quorumThreshold
    approvalCount
    quorumReached
    verifiedMessageDigest
    messageType
    guardianPublicKeys
    verifiedGuardianPublicKeys
    verifiedApprovals
    feeBinding
    steps
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    approvalVerificationOnly: true
    feeBoundMessageRequired: true
    digestBindingRequired: true
    exactDigestMatchRequired: true
    fixedGuardianCount: 5
    fixedQuorumThreshold: 3
    duplicateApprovalHandling: reject
    unknownGuardianHandling: reject
    guardianSetVersionBound: 1
    signing: not_performed
    cryptographicSignatureVerification: not_performed
    walletLoading: not_allowed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.10 invariants:

    offlineOnly: true
    feeBoundMessageRequired: true
    approvalsBoundToFeeDigest: true
    exactDigestMatch: true
    exactlyFiveGuardians: true
    threeOfFiveQuorum: true
    noDuplicateApprovalCounting: true
    noUnknownGuardianAccepted: true
    noPrivateKeys: true
    noSigning: true
    noCryptographicSignatureVerification: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed approval verification operations

New type:

    Stage4GuardianFeeBoundApprovalVerificationOperation

Allowed operations:

    validateFeeBoundApprovalMessage
    verifyDigestBoundApprovals
    recordVerifiedApprovalQuorum

Rejected example operations:

    sendTransaction
    signMessage

## Error model

New class:

    Stage4GuardianFeeBoundApprovalVerificationError

New reason type:

    Stage4GuardianFeeBoundApprovalVerificationErrorReason

Reasons:

    invalid_verified_at_iso
    invalid_fee_bound_approval_message_result
    fee_bound_approval_message_not_ok
    invalid_guardian_set
    invalid_quorum_threshold
    invalid_guardian_set_version
    invalid_approval
    approval_digest_mismatch
    duplicate_guardian_approval
    unknown_guardian_approval
    insufficient_quorum
    forbidden_value
    invalid_approval_verification_operation

## New helpers

Operation assertion helper:

    assertStage4GuardianFeeBoundApprovalVerificationOperationPrototype

Verification runner:

    runStage4GuardianFeeBoundApprovalVerificationPrototype

Result checker:

    checkStage4GuardianFeeBoundApprovalVerificationResultPrototype

## Secret, wallet, signing, verification, and transaction boundary

Stage 4.10 rejects values containing forbidden markers such as:

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

No wallet-loading path is introduced.

No guardian private key material is introduced.

No signing path is introduced.

No production cryptographic signature verification path is introduced.

No transaction submission path is introduced.

No serialized transaction material is introduced.

## Successful verification test

Confirmed behavior:

- verifies 3-of-5 guardian approvals
- approvals are bound to the exact fee-bound message digest
- sourceApprovalMessageStage is 4.9
- sourceApprovalMessageOk is true
- guardianSetVersion is 1
- guardianCount is 5
- quorumThreshold is 3
- approvalCount is 3
- quorumReached is true
- verifiedMessageDigest equals the Stage 4.9 feeBoundMessageDigest
- messageType is STAGE4_GUARDIAN_FEE_BOUND_APPROVAL_MESSAGE
- verifiedGuardianPublicKeys match the accepted guardian approvals
- verifiedApprovals are preserved as public approval records
- feeBinding is preserved from the Stage 4.9 approval message
- checkStage4GuardianFeeBoundApprovalVerificationResultPrototype returns true

## Quorum test

Confirmed behavior:

- 2-of-5 digest-bound approvals are rejected as insufficient_quorum
- 3-of-5 digest-bound approvals are accepted
- 4-of-5 digest-bound approvals are accepted
- 5-of-5 digest-bound approvals are accepted

This keeps the Stage 4.7 guardian policy intact through the Stage 4.10 approval verification boundary.

## Safe result JSON test

Confirmed behavior:

- fee-bound approval verification result JSON does not contain wallet path
- fee-bound approval verification result JSON does not contain private key markers
- fee-bound approval verification result JSON does not contain signing methods
- fee-bound approval verification result JSON does not contain serialized transaction marker
- fee-bound approval verification result JSON does not contain transaction submission methods

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
- serializedTransaction

Exact safety marker verification confirmed that the assertions use full markers:

- PRIVATE_KEY
- RPC_API_KEY
- signMessage

## Rejection test

Confirmed behavior:

- malformed verifiedAtIso is rejected as invalid_verified_at_iso
- failed Stage 4.9 approval message is rejected as fee_bound_approval_message_not_ok
- wrong message digest is rejected as approval_digest_mismatch
- duplicate guardian approval is rejected as duplicate_guardian_approval
- unknown guardian approval is rejected as unknown_guardian_approval
- approvalId containing privateKey marker is rejected as forbidden_value
- sendTransaction operation is rejected as invalid_approval_verification_operation
- signMessage operation is rejected as invalid_approval_verification_operation

## Stage 4.10 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts

Result:

    Stage 4.10 guardian fee-bound approval verification boundary
      ✔ verifies 3-of-5 guardian approvals that are bound to the exact fee-bound message digest
      ✔ rejects 2-of-5 while accepting 3-of-5, 4-of-5, and 5-of-5 digest-bound approvals
      ✔ keeps fee-bound approval verification result JSON free of wallet paths, secrets, signing, cryptographic signature verification, and transaction submission methods
      ✔ rejects malformed metadata, failed message, wrong digest, duplicate approval, unknown guardian, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.10 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts \
      tests/stage4_gateway_fee_policy_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts

Result:

    33 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.10 smoke

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage3_final_closure_boundary.test.ts \
      tests/stage4_redacted_live_config_boundary.test.ts \
      tests/stage4_read_only_rpc_connectivity_boundary.test.ts \
      tests/stage4_watcher_read_only_observation_boundary.test.ts \
      tests/stage4_relayer_dry_run_no_send_boundary.test.ts \
      tests/stage4_guardian_operation_policy_boundary.test.ts \
      tests/stage4_transaction_preflight_no_send_boundary.test.ts \
      tests/stage4_fixed_guardian_set_quorum_boundary.test.ts \
      tests/stage4_gateway_fee_policy_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts \
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts

Result:

    36 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Exact safety marker verification:

    ok

Pasted terminal fragments check:

    clean

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

## Boundary classification

Stage 4.10 is:

    guardian fee-bound approval verification boundary
    exact digest-bound approval boundary
    fixed 5 guardian / 3-of-5 quorum preservation boundary
    duplicate guardian rejection boundary
    unknown guardian rejection boundary
    offline model boundary

Stage 4.10 is not:

    production cryptographic signature verification boundary
    guardian private key boundary
    wallet access boundary
    transaction preflight boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.10 proves that guardian approvals are accepted only when they reference the exact Stage 4.9 fee-bound message digest.

It prevents approvals from being reused across different fee amounts, fee recipients, net amounts, fee quote ids, deadlines, or message digests.

The next valid stage is Stage 4.11 production signature verification design boundary, or a deliberate Stage 4 checkpoint if we want to pause before cryptographic signature verification.
