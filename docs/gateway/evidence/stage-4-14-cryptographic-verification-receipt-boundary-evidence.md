# Stage 4.14 Cryptographic Verification Receipt Boundary Evidence

This document records Stage 4.14 cryptographic verification receipt boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-14-cryptographic-verification-receipt-boundary

Runtime commit:

    7624afa Add Stage 4.14 cryptographic verification receipt boundary

Base runtime commit:

    c256bba Add Stage 4.13 offline cryptographic signature verification boundary

## Stage position

Stage 4.9 created the fee-bound guardian approval message digest.

Stage 4.10 verified guardian approvals against the exact fee-bound digest at model level.

Stage 4.11 fixed the XNTD -> XXXL amount conversion policy:

    1 XXXL = 100,000,000 XNTD
    XNTD ERC-20 decimals = 18
    XXXL X1 decimals = 9
    xntdRawPerXxxlRaw = 100000000000000000

Stage 4.12 defined the production signature verification design contract.

Stage 4.13 introduced real offline ed25519 verification.

Stage 4.14 does not introduce new cryptography.

Stage 4.14 creates a deterministic receipt over the already verified Stage 4.13 cryptographic verification result.

## Scope

Stage 4.14 records a cryptographic verification receipt.

The receipt binds:

    sourceResultDigest
    receiptDigest
    guardianSetVersion
    verifiedFeeBoundMessageDigest
    verifiedSignatureCount
    verifiedGuardianPublicKeysDigest
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

The receipt makes the Stage 4.13 verification result safe to pass forward into later preflight, relayer, audit, or submission planning boundaries without re-exposing the full signature payload every time.

This stage is still offline.

It does not use live RPC.

It does not load a wallet.

It does not access private keys.

It does not export private keys.

It does not sign anything.

It does not submit transactions.

It does not spend SOL.

## Runtime changes

New helper:

    tests/helpers/stage4CryptographicVerificationReceiptPrototype.ts

New test:

    tests/stage4_cryptographic_verification_receipt_boundary.test.ts

## Required source

Stage 4.14 consumes Stage 4.13 offline cryptographic signature verification result:

    stage4_offline_cryptographic_signature_verification_result

Required Stage 4.13 conditions:

    stage = 4.13
    executionMode = offline_cryptographic_signature_verification
    sourceDesignStage = 4.12
    sourceDesignOk = true
    guardianSetVersion = 1
    guardianCount = 5
    quorumThreshold = 3
    verifiedSignatureCount >= 3
    quorumReached = true
    signatureScheme = ed25519
    publicKeyEncoding = base58_x1_guardian_public_key
    signatureEncoding = base64_ed25519_signature
    messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
    xntdErc20Decimals = 18
    xxxlX1Decimals = 9
    xntdPerXxxl = 100000000
    xntdRawPerXxxlRaw = 100000000000000000
    offlineCryptographicVerificationOnly = true
    realEd25519Verification = true
    signing = not_performed
    privateKeyAccess = not_allowed
    walletLoading = not_allowed
    liveRpc = not_used
    transactionSubmission = not_allowed
    solSpendAllowed = false
    ok = true

## Digest model

Stage 4.14 derives two digests:

    sourceResultDigest
    receiptDigest

The sourceResultDigest is:

    sha256(stableStringify(Stage 4.13 source result))

The receiptDigest is:

    sha256(stableStringify(receiptPayload))

The helper uses deterministic stable key ordering before hashing.

This makes receipt generation deterministic across repeated runs for the same source result.

## Receipt payload

New helper:

    buildStage4CryptographicVerificationReceiptPayloadPrototype

Payload fields:

    sourceResultDigest
    guardianSetVersion
    verifiedFeeBoundMessageDigest
    verifiedSignatureCount
    verifiedGuardianPublicKeysDigest
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

The receipt payload is smaller than the full Stage 4.13 verification result.

It still preserves the important binding between:

    fee-bound digest
    guardian quorum
    verified guardian keys digest
    XNTD -> XXXL conversion ratio
    burned XNTD raw amount
    XXXL mint raw amount

## New result artifact

New result type:

    Stage4CryptographicVerificationReceiptResult

Artifact type:

    stage4_cryptographic_verification_receipt_result

Schema version:

    1

Stage:

    4.14

Execution mode:

    cryptographic_verification_receipt_offline

Fields:

    receiptedAtIso
    sourceVerificationStage
    sourceVerificationOk
    sourceArtifactType
    sourceResultDigest
    receiptDigest
    guardianSetVersion
    guardianCount
    quorumThreshold
    verifiedSignatureCount
    quorumReached
    signatureScheme
    publicKeyEncoding
    signatureEncoding
    messageBinding
    verifiedFeeBoundMessageDigest
    xntdErc20Decimals
    xxxlX1Decimals
    xntdPerXxxl
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    guardianPublicKeysDigest
    verifiedGuardianPublicKeysDigest
    verifiedGuardianPublicKeys
    receiptPayload
    operations
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    receiptOnly: true
    sourceVerificationRequired: stage4_offline_cryptographic_signature_verification_result
    sourceResultDigestRequired: true
    receiptDigestRequired: true
    exactFeeDigestMatchRequired: true
    exactAmountConversionRequired: true
    fixedGuardianCount: 5
    fixedQuorumThreshold: 3
    minimumVerifiedSignatureCount: 3
    signing: not_performed
    privateKeyAccess: not_allowed
    walletLoading: not_allowed
    liveRpc: not_used
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.14 invariants:

    offlineOnly: true
    receiptOnly: true
    sourceStage413Bound: true
    sourceResultDigestBound: true
    receiptDigestBound: true
    feeBoundMessageDigestBound: true
    amountConversionPolicyBound: true
    exactFeeDigestMatch: true
    exactAmountConversion: true
    boundToGuardianSetVersion: true
    exactlyFiveGuardians: true
    threeOfFiveQuorum: true
    noSigning: true
    noPrivateKeys: true
    noWalletLoaded: true
    noLiveRpc: true
    noTransactionsSubmitted: true
    noSolSpend: true

All invariants must remain true.

## Allowed operations

New type:

    Stage4CryptographicVerificationReceiptOperation

Allowed operations:

    validateOfflineCryptographicVerificationResult
    deriveSourceVerificationDigest
    buildCryptographicVerificationReceipt
    recordReceiptBoundary

Rejected example operations:

    sendTransaction
    signMessage

## Error model

New class:

    Stage4CryptographicVerificationReceiptError

New reason type:

    Stage4CryptographicVerificationReceiptErrorReason

Reasons:

    invalid_receipted_at_iso
    invalid_offline_cryptographic_verification_result
    offline_cryptographic_verification_not_ok
    invalid_digest
    forbidden_value
    invalid_cryptographic_verification_receipt_operation

## New helpers

Source digest helper:

    deriveStage4SourceVerificationDigestPrototype

Receipt payload helper:

    buildStage4CryptographicVerificationReceiptPayloadPrototype

Operation assertion helper:

    assertStage4CryptographicVerificationReceiptOperationPrototype

Receipt runner:

    runStage4CryptographicVerificationReceiptPrototype

Result checker:

    checkStage4CryptographicVerificationReceiptResultPrototype

## Successful receipt test

Confirmed behavior:

- builds a deterministic receipt over a valid Stage 4.13 offline cryptographic verification result
- derives sourceResultDigest as a 64-char hex digest
- derives receiptDigest as a 64-char hex digest
- sourceVerificationStage = 4.13
- sourceVerificationOk = true
- sourceArtifactType = stage4_offline_cryptographic_signature_verification_result
- guardianSetVersion = 1
- guardianCount = 5
- quorumThreshold = 3
- verifiedSignatureCount = 3
- quorumReached = true
- signatureScheme = ed25519
- publicKeyEncoding = base58_x1_guardian_public_key
- signatureEncoding = base64_ed25519_signature
- messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
- xntdRawPerXxxlRaw = 100000000000000000
- burnedXntdRaw = 100000000000000000000000000
- xxxlMintRaw = 1000000000
- receiptPayload.sourceResultDigest equals sourceResultDigest
- checkStage4CryptographicVerificationReceiptResultPrototype returns true

## Digest stability test

Confirmed behavior:

- the same Stage 4.13 source result produces the same sourceResultDigest
- the same Stage 4.13 source result produces the same receiptDigest
- a changed Stage 4.13 source result changes sourceResultDigest
- a changed Stage 4.13 source result changes receiptDigest

The tested source change was verifiedSignatureCount changing from 3 to 4.

## Safe result JSON test

Confirmed behavior:

- receipt JSON does not contain wallet path
- receipt JSON does not contain private key markers
- receipt JSON does not contain signing methods
- receipt JSON does not contain serialized transaction marker
- receipt JSON does not contain transaction submission methods
- receipt JSON does not contain live_rpc marker

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

- malformed receiptedAtIso is rejected as invalid_receipted_at_iso
- failed Stage 4.13 source is rejected as invalid_offline_cryptographic_verification_result
- wrong source stage is rejected as invalid_offline_cryptographic_verification_result
- wrong expectedSourceResultDigest is rejected as invalid_digest
- wrong expectedReceiptDigest is rejected as invalid_digest
- forbidden value in source amount is rejected as forbidden_value
- sendTransaction operation is rejected as invalid_cryptographic_verification_receipt_operation
- signMessage operation is rejected as invalid_cryptographic_verification_receipt_operation

## Stage 4.14 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_cryptographic_verification_receipt_boundary.test.ts

Result:

    Stage 4.14 cryptographic verification receipt boundary
      ✔ builds a deterministic receipt over a valid Stage 4.13 offline cryptographic verification result
      ✔ keeps source and receipt digests stable for the same source while changing when source verification changes
      ✔ keeps receipt JSON free of wallet paths, secrets, signing methods, live RPC, and transaction submission methods
      ✔ rejects malformed metadata, invalid source, digest mismatches, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.14 smoke

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
      tests/stage4_production_signature_verification_design_boundary.test.ts \
      tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts \
      tests/stage4_cryptographic_verification_receipt_boundary.test.ts

Result:

    48 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.14 smoke

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
      tests/stage4_production_signature_verification_design_boundary.test.ts \
      tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts \
      tests/stage4_cryptographic_verification_receipt_boundary.test.ts

Result:

    51 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Strict sourceResultDigest typo check:

    ok

Exact safety marker verification:

    ok

Pasted terminal fragments check:

    clean

No SOL is spent by this stage.

No live RPC is required by this stage.

No wallet is loaded by this stage.

No private key material was introduced.

No signing path was introduced.

No transaction submission path was introduced.

## Boundary classification

Stage 4.14 is:

    cryptographic verification receipt boundary
    Stage 4.13 source result digest boundary
    receipt digest boundary
    deterministic receipt payload boundary
    guardian quorum receipt boundary
    fee-bound digest and amount conversion receipt boundary
    offline model boundary

Stage 4.14 is not:

    new cryptographic verification boundary
    production guardian key custody boundary
    wallet access boundary
    private key export boundary
    live RPC boundary
    signing boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.14 creates a deterministic receipt over the already verified Stage 4.13 offline cryptographic signature verification result.

The receipt binds:

    Stage 4.13 source result digest
    receipt digest
    Stage 4.9 fee-bound message digest
    Stage 4.11 exact XNTD -> XXXL amount conversion policy
    guardianSetVersion
    verified guardian quorum
    verified guardian public keys digest

It does not introduce new cryptographic verification.

It does not introduce signing.

It does not introduce live RPC or transaction submission.

The next valid stage is Stage 4.15 receipt-bound transaction preflight boundary.
