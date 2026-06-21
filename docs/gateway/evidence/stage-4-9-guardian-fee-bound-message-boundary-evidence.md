# Stage 4.9 Guardian Fee-Bound Approval Message Boundary Evidence

This document records Stage 4.9 guardian fee-bound approval message boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-9-guardian-fee-bound-message-boundary

Runtime commit:

    d4a7060 Add Stage 4.9 guardian fee-bound approval message boundary

Base runtime commit:

    a960c16 Add Stage 4.8 gateway fee policy boundary

## Stage position

Stage 4.7 fixed the guardian set model:

    guardian_count = 5
    quorum_threshold = 3
    guardian_set_version = 1

Stage 4.8 established the gateway fee policy model:

    feeMode: fixed_service_fee
    pricingSource: manual_config_only
    feeRecipientSource: configured_public_address
    grossAmount
    feeAmount
    netAmount
    feeQuoteExpiresAtIso
    guardianSetVersion binding

Stage 4.9 now builds the guardian approval message with gateway fee fields bound into the message digest.

This stage is still fully offline.

It does not use live RPC.

It does not load a wallet.

It does not sign anything.

It does not verify cryptographic signatures yet.

It does not submit transactions.

It does not spend SOL.

## Scope

Stage 4.9 defines the fee-bound guardian approval message model.

It proves that guardians will not later approve one event while a different fee is applied.

The gateway fee is included in the message digest through canonical fields.

The message binds:

    routeId
    feeQuoteId
    feeMode
    feeAsset
    feeRecipient
    grossAmount
    feeAmount
    netAmount
    feeQuoteExpiresAtIso
    guardianSetVersion

This stage prepares the model for a later guardian signature verification boundary.

## Runtime changes

New helper:

    tests/helpers/stage4GuardianFeeBoundApprovalMessagePrototype.ts

New test:

    tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts

## Dependency on Stage 4.8

Stage 4.9 consumes Stage 4.8 gateway fee policy evidence.

Required source artifact:

    stage4_gateway_fee_policy_result

Required source stage:

    4.8

Required source execution mode:

    gateway_fee_policy_offline

Required source conditions:

    sourceGuardianQuorumStage must be 4.7
    sourceGuardianQuorumOk must be true
    guardianSetVersion must be 1
    feeMode must be fixed_service_fee
    oracleLookup must be not_performed
    walletLoading must be not_allowed
    signing must be not_performed
    transactionSubmission must be not_allowed
    solSpendAllowed must be false
    result ok must be true

Stage 4.9 rejects failed fee policy evidence.

## Message type

Stage 4.9 defines the message type:

    STAGE4_GUARDIAN_FEE_BOUND_APPROVAL_MESSAGE

New result artifact:

    stage4_guardian_fee_bound_approval_message_result

Execution mode:

    guardian_approval_message_fee_bound_offline

## Canonical field order

Stage 4.9 fixes this canonical field order:

    messageType
    schemaVersion
    stage
    guardianSetVersion
    routeId
    feeQuoteId
    feeMode
    feeAsset
    feeRecipient
    grossAmount
    feeAmount
    netAmount
    feeQuoteExpiresAtIso

This order is part of the message digest model.

Changing fee amount, net amount, fee asset, fee quote id, fee recipient, deadline, route id, or guardian set version changes the digest.

## Digest model

Digest algorithm name:

    sha256_model_hash

This is a model-stage digest, not a final production signing format.

The helper builds a canonical preimage by joining key/value pairs in canonical field order.

Then it computes:

    sha256(preimage)

The resulting digest is used as the fee-bound message digest.

## Fee binding

Stage 4.9 result includes:

    feeBinding.routeId
    feeBinding.feeQuoteId
    feeBinding.feeMode
    feeBinding.feeAsset
    feeBinding.feeRecipient
    feeBinding.grossAmount
    feeBinding.feeAmount
    feeBinding.netAmount
    feeBinding.feeQuoteExpiresAtIso

The fee binding is copied from the Stage 4.8 fee policy result.

## Policy object

The result includes a policy object:

    approvalMessageOnly: true
    feeFieldsRequired: true
    grossAmountBound: true
    feeAmountBound: true
    netAmountBound: true
    feeRecipientBound: true
    feeQuoteDeadlineBound: true
    guardianSetVersionBound: 1
    signing: not_performed
    signatureVerification: not_performed
    walletLoading: not_allowed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.9 invariants:

    offlineOnly: true
    feeBoundIntoMessage: true
    digestDependsOnFeeAmount: true
    digestDependsOnNetAmount: true
    digestDependsOnFeeRecipient: true
    digestDependsOnDeadline: true
    boundToGuardianSetVersion: true
    noWalletLoaded: true
    noSigning: true
    noSignatureVerification: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed approval message operations

New type:

    Stage4GuardianFeeBoundApprovalMessageOperation

Allowed operations:

    validateGatewayFeePolicyResult
    buildFeeBoundApprovalMessage
    recordFeeBindingDigest

Rejected example operation:

    signMessage

## Error model

New class:

    Stage4GuardianFeeBoundApprovalMessageError

New reason type:

    Stage4GuardianFeeBoundApprovalMessageErrorReason

Reasons:

    invalid_built_at_iso
    fee_quote_expired_at_build_time
    invalid_fee_policy_result
    fee_policy_not_ok
    invalid_expected_digest
    forbidden_value
    invalid_approval_message_operation

## New helpers

Canonical preimage builder:

    buildStage4GuardianFeeBoundApprovalMessagePreimagePrototype

Digest helper:

    computeStage4GuardianFeeBoundApprovalMessageDigestPrototype

Operation assertion helper:

    assertStage4GuardianFeeBoundApprovalMessageOperationPrototype

Message runner:

    runStage4GuardianFeeBoundApprovalMessagePrototype

Result checker:

    checkStage4GuardianFeeBoundApprovalMessageResultPrototype

## Secret, wallet, signing, verification, and transaction boundary

Stage 4.9 rejects values containing forbidden markers such as:

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

No signature verification path is introduced.

No transaction submission path is introduced.

No serialized transaction material is introduced.

## Successful fee-bound approval message test

Confirmed behavior:

- builds an offline guardian approval message from a valid Stage 4.8 gateway fee policy result
- message type is STAGE4_GUARDIAN_FEE_BOUND_APPROVAL_MESSAGE
- sourceFeePolicyStage is 4.8
- sourceFeePolicyOk is true
- guardianSetVersion is 1
- canonical field order is fixed
- feeBoundMessageDigestAlgorithm is sha256_model_hash
- feeBoundMessageDigest is produced
- fee binding includes routeId
- fee binding includes feeQuoteId
- fee binding includes feeMode
- fee binding includes feeAsset
- fee binding includes feeRecipient
- fee binding includes grossAmount
- fee binding includes feeAmount
- fee binding includes netAmount
- fee binding includes feeQuoteExpiresAtIso
- checkStage4GuardianFeeBoundApprovalMessageResultPrototype returns true

## Digest mutation test

Confirmed behavior:

- changing fee amount changes the digest
- changing net amount changes the digest
- changing fee asset changes the digest
- changing fee quote id changes the digest
- changing fee quote deadline changes the digest

This proves that fee data is not external to the guardian approval message.

The fee is bound into the message digest.

## Safe result JSON test

Confirmed behavior:

- fee-bound approval message result JSON does not contain wallet path
- fee-bound approval message result JSON does not contain private key markers
- fee-bound approval message result JSON does not contain signing methods
- fee-bound approval message result JSON does not contain transaction submission methods
- fee-bound approval message result JSON does not contain serialized transaction marker

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

Exact marker verification confirmed that the assertions use full markers:

- PRIVATE_KEY
- RPC_API_KEY
- signMessage

No truncated marker assertions remain.

## Rejection test

Confirmed behavior:

- bad builtAtIso is rejected as invalid_built_at_iso
- expired fee quote is rejected as fee_quote_expired_at_build_time
- failed fee policy is rejected as fee_policy_not_ok
- wrong expected digest is rejected as invalid_expected_digest
- feeQuoteId containing privateKey marker is rejected as forbidden_value
- signMessage operation is rejected as invalid_approval_message_operation

## Stage 4.9 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts

Result:

    Stage 4.9 guardian fee-bound approval message boundary
      ✔ builds an offline guardian approval message with gateway fee fields bound into the digest
      ✔ changes the fee-bound digest when fee amount, net amount, fee asset, fee quote id, or deadline changes
      ✔ keeps fee-bound approval message result JSON free of wallet paths, secrets, signing, verification, and transaction submission methods
      ✔ rejects bad build metadata, expired fee quote, failed fee policy, wrong expected digest, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.9 smoke

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
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts

Result:

    29 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.9 smoke

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
      tests/stage4_guardian_fee_bound_approval_message_boundary.test.ts

Result:

    32 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Pasted terminal fragments check:

    clean

Exact safety marker verification:

    ok

Expected forbidden-marker and secret-safety assertion lines exist only as defensive checks.

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

## Boundary classification

Stage 4.9 is:

    guardian fee-bound approval message boundary
    fee-bound message digest boundary
    canonical field order boundary
    fee binding boundary
    guardianSetVersion-bound message boundary
    offline model boundary

Stage 4.9 is not:

    guardian signature boundary
    guardian signature verification boundary
    guardian private key boundary
    wallet access boundary
    transaction preflight boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.9 proves that the gateway fee is bound into the guardian approval message digest.

It prevents a design where guardians approve one event while a different fee recipient, fee amount, net amount, fee quote id, or deadline is applied later.

The next valid stage is Stage 4.10 guardian fee-bound approval verification boundary.
