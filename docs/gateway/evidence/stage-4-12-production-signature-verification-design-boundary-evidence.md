# Stage 4.12 Production Signature Verification Design Boundary Evidence

This document records Stage 4.12 production signature verification design boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-12-production-signature-verification-design-boundary

Runtime commit:

    2c84294 Add Stage 4.12 production signature verification design boundary

Base runtime commit:

    0be7dd4 Add Stage 4.11 XNTD XXXL amount conversion boundary

## Stage position

Stage 4.9 created the fee-bound guardian approval message digest.

Stage 4.10 verified guardian approvals against the exact fee-bound digest.

Stage 4.11 fixed the XNTD -> XXXL amount conversion policy:

    1 XXXL = 100,000,000 XNTD
    XNTD ERC-20 decimals = 18
    XXXL X1 decimals = 9
    xntdRawPerXxxlRaw = 100000000000000000

Stage 4.12 now defines the production signature verification design contract.

This stage is design-only.

It does not verify real cryptographic signatures yet.

It does not load a wallet.

It does not access private keys.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

## Scope

Stage 4.12 defines what future production signature verification must be bound to.

The future verifier must bind guardian signatures to both:

    Stage 4.9 fee-bound message digest
    Stage 4.11 amount conversion policy

The signature verification target includes:

    feeBoundMessageDigest
    messageType
    requiredApprovalKind
    requiredGuardianSetVersion
    requiredGuardianCount
    requiredQuorumThreshold
    amountConversionStage
    xntdErc20Decimals
    xxxlX1Decimals
    xntdPerXxxl
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw

This prevents production signature verification from being designed only around a fee-bound digest while accidentally ignoring the amount conversion policy.

## Runtime changes

New helper:

    tests/helpers/stage4ProductionSignatureVerificationDesignPrototype.ts

New test:

    tests/stage4_production_signature_verification_design_boundary.test.ts

## Required sources

Stage 4.12 consumes Stage 4.10 approval verification result:

    stage4_guardian_fee_bound_approval_verification_result

Required Stage 4.10 conditions:

    stage = 4.10
    executionMode = guardian_fee_bound_approval_verification_offline
    sourceApprovalMessageStage = 4.9
    sourceApprovalMessageOk = true
    guardianSetVersion = 1
    guardianCount = 5
    quorumThreshold = 3
    approvalCount >= 3
    quorumReached = true
    messageType = STAGE4_GUARDIAN_FEE_BOUND_APPROVAL_MESSAGE
    verifiedMessageDigest is valid model digest
    signing = not_performed
    cryptographicSignatureVerification = not_performed
    walletLoading = not_allowed
    transactionSubmission = not_allowed
    solSpendAllowed = false
    ok = true

Stage 4.12 also consumes Stage 4.11 amount conversion policy result:

    stage4_xntd_xxxl_amount_conversion_policy_result

Required Stage 4.11 conditions:

    stage = 4.11
    executionMode = xntd_xxxl_amount_conversion_policy_offline
    xntdErc20Decimals = 18
    xxxlX1Decimals = 9
    xntdPerXxxl = 100000000
    xntdRawPerXxxlRaw = 100000000000000000
    xxxlMintRaw = expectedXxxlMintRaw
    conversionRemainderRaw = 0
    signing = not_performed
    walletLoading = not_allowed
    transactionSubmission = not_allowed
    solSpendAllowed = false
    ok = true

Stage 4.12 requires route consistency between Stage 4.10 fee binding and Stage 4.11 amount conversion.

## Signature design contract

Stage 4.12 fixes the design contract:

    signatureScheme = ed25519
    publicKeyEncoding = base58_x1_guardian_public_key
    signatureEncoding = base64_ed25519_signature
    messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
    feeBoundDigestAlgorithm = sha256_model_hash

This does not perform real signature verification yet.

It only defines the expected production verification design.

## New result artifact

New result type:

    Stage4ProductionSignatureVerificationDesignResult

Artifact type:

    stage4_production_signature_verification_design_result

Schema version:

    1

Stage:

    4.12

Execution mode:

    production_signature_verification_design_offline

Fields:

    designedAtIso
    sourceApprovalVerificationStage
    sourceApprovalVerificationOk
    sourceAmountConversionStage
    sourceAmountConversionOk
    guardianSetVersion
    guardianCount
    quorumThreshold
    verifiedFeeBoundMessageDigest
    xntdErc20Decimals
    xxxlX1Decimals
    xntdPerXxxl
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    signatureScheme
    publicKeyEncoding
    signatureEncoding
    messageBinding
    feeBoundDigestAlgorithm
    productionVerificationTarget
    operations
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    designOnly: true
    productionSignatureSchemeRequired: ed25519
    publicKeyEncodingRequired: base58_x1_guardian_public_key
    signatureEncodingRequired: base64_ed25519_signature
    messageBindingRequired: stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
    exactFeeDigestMatchRequired: true
    exactAmountConversionRequired: true
    fixedGuardianCount: 5
    fixedQuorumThreshold: 3
    duplicateApprovalHandling: reject
    unknownGuardianHandling: reject
    guardianSetVersionBound: 1
    signing: not_performed
    cryptographicSignatureVerification: not_performed
    privateKeyAccess: not_allowed
    walletLoading: not_allowed
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.12 invariants:

    offlineOnly: true
    designOnly: true
    ed25519Required: true
    publicKeyEncodingDefined: true
    signatureEncodingDefined: true
    feeBoundMessageDigestBound: true
    amountConversionPolicyBound: true
    exactFeeDigestMatch: true
    exactAmountConversion: true
    boundToGuardianSetVersion: true
    exactlyFiveGuardians: true
    threeOfFiveQuorum: true
    noPrivateKeys: true
    noSigning: true
    noCryptographicSignatureVerification: true
    noWalletLoaded: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed design operations

New type:

    Stage4ProductionSignatureVerificationDesignOperation

Allowed operations:

    validateFeeBoundApprovalVerificationResult
    validateAmountConversionPolicyResult
    defineProductionSignatureVerificationContract
    recordSignatureVerificationReadiness

Rejected example operations:

    sendTransaction
    signMessage

## Error model

New class:

    Stage4ProductionSignatureVerificationDesignError

New reason type:

    Stage4ProductionSignatureVerificationDesignErrorReason

Reasons:

    invalid_designed_at_iso
    invalid_fee_bound_approval_verification_result
    fee_bound_approval_verification_not_ok
    invalid_amount_conversion_policy_result
    amount_conversion_policy_not_ok
    route_mismatch
    invalid_signature_scheme
    invalid_public_key_encoding
    invalid_signature_encoding
    invalid_message_binding
    invalid_expected_fee_bound_digest
    forbidden_value
    invalid_signature_verification_design_operation

## New helpers

Operation assertion helper:

    assertStage4ProductionSignatureVerificationDesignOperationPrototype

Design runner:

    runStage4ProductionSignatureVerificationDesignPrototype

Result checker:

    checkStage4ProductionSignatureVerificationDesignResultPrototype

## Successful design test

Confirmed behavior:

- Stage 4.12 result binds to Stage 4.10 approval verification result
- Stage 4.12 result binds to Stage 4.11 amount conversion policy result
- signatureScheme is ed25519
- publicKeyEncoding is base58_x1_guardian_public_key
- signatureEncoding is base64_ed25519_signature
- messageBinding is stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
- feeBoundDigestAlgorithm is sha256_model_hash
- guardianSetVersion is 1
- guardianCount is 5
- quorumThreshold is 3
- xntdErc20Decimals is 18
- xxxlX1Decimals is 9
- xntdPerXxxl is 100000000
- xntdRawPerXxxlRaw is 100000000000000000
- burnedXntdRaw is 100000000000000000000000000
- xxxlMintRaw is 1000000000
- checkStage4ProductionSignatureVerificationDesignResultPrototype returns true

## Safe result JSON test

Confirmed behavior:

- signature verification design result JSON does not contain wallet path
- signature verification design result JSON does not contain private key markers
- signature verification design result JSON does not contain signing methods
- signature verification design result JSON does not contain serialized transaction marker
- signature verification design result JSON does not contain transaction submission methods

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

These are defensive markers only.

No real secret values are introduced.

## Rejection test

Confirmed behavior:

- malformed designedAtIso is rejected as invalid_designed_at_iso
- failed Stage 4.10 approval verification is rejected as fee_bound_approval_verification_not_ok
- failed Stage 4.11 amount conversion is rejected as amount_conversion_policy_not_ok
- route mismatch is rejected as route_mismatch
- unsupported signature scheme is rejected as invalid_signature_scheme
- unsupported public key encoding is rejected as invalid_public_key_encoding
- unsupported signature encoding is rejected as invalid_signature_encoding
- unsupported message binding is rejected as invalid_message_binding
- wrong expected fee-bound digest is rejected as invalid_expected_fee_bound_digest
- forbidden expected digest value is rejected as forbidden_value
- sendTransaction operation is rejected as invalid_signature_verification_design_operation
- signMessage operation is rejected as invalid_signature_verification_design_operation

## Stage 4.12 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_production_signature_verification_design_boundary.test.ts

Result:

    Stage 4.12 production signature verification design boundary
      ✔ defines the offline production signature verification contract bound to fee digest and amount conversion policy
      ✔ keeps signature verification design JSON free of wallet paths, secrets, signing, real verification, and transaction submission methods
      ✔ rejects malformed metadata, failed sources, route mismatch, unsupported signature contract values, wrong digest, forbidden values, and invalid operations

    3 passing

## Stage 4.1 through Stage 4.12 smoke

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
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts \
      tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts \
      tests/stage4_production_signature_verification_design_boundary.test.ts

Result:

    40 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.12 smoke

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
      tests/stage4_guardian_fee_bound_approval_verification_boundary.test.ts \
      tests/stage4_xntd_xxxl_amount_conversion_boundary.test.ts \
      tests/stage4_production_signature_verification_design_boundary.test.ts

Result:

    43 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Exact safety marker verification:

    ok

Suspicious typo check:

    clean

Pasted terminal fragments check:

    clean

No SOL is spent by this stage.

No live RPC is required by this stage.

No secret-like material was introduced.

No private key material was introduced.

No real cryptographic signature verification was introduced.

## Boundary classification

Stage 4.12 is:

    production signature verification design boundary
    ed25519 signature scheme design boundary
    base58 guardian public key encoding design boundary
    base64 signature encoding design boundary
    fee-bound digest and amount conversion binding design boundary
    offline model boundary

Stage 4.12 is not:

    real cryptographic signature verification boundary
    guardian private key boundary
    wallet access boundary
    signing boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.12 defines the future production signature verification contract.

It binds the future signature verification target to:

    Stage 4.9 fee-bound message digest
    Stage 4.10 guardian approval verification
    Stage 4.11 exact XNTD -> XXXL amount conversion policy

It does not yet verify real ed25519 signatures.

The next valid stage is Stage 4.13 offline cryptographic signature verification boundary.
