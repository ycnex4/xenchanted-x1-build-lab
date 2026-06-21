# Stage 4.13 Offline Cryptographic Signature Verification Boundary Evidence

This document records Stage 4.13 offline cryptographic signature verification boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-13-offline-cryptographic-signature-verification-boundary

Runtime commit:

    c256bba Add Stage 4.13 offline cryptographic signature verification boundary

Base runtime commit:

    2c84294 Add Stage 4.12 production signature verification design boundary

## Stage position

Stage 4.9 created the fee-bound guardian approval message digest.

Stage 4.10 verified guardian approvals against the exact fee-bound digest at model level.

Stage 4.11 fixed the XNTD -> XXXL amount conversion policy:

    1 XXXL = 100,000,000 XNTD
    XNTD ERC-20 decimals = 18
    XXXL X1 decimals = 9
    xntdRawPerXxxlRaw = 100000000000000000

Stage 4.12 defined the production signature verification design contract:

    signatureScheme = ed25519
    publicKeyEncoding = base58_x1_guardian_public_key
    signatureEncoding = base64_ed25519_signature
    messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy

Stage 4.13 is the first boundary that performs real offline ed25519 signature verification.

It uses:

    @noble/curves/ed25519

This stage is still offline.

It does not use live RPC.

It does not load a wallet.

It does not access private keys.

It does not export private keys.

It does not introduce a production signing path.

It does not submit transactions.

It does not spend SOL.

## Important test fixture note

Stage 4.13 tests use deterministic test-only fixture seeds to create local test signatures.

Those fixture seeds are not production guardian keys.

They are not wallet keys.

They are not exported from any wallet.

They are only deterministic local test material used to prove that the verifier rejects invalid signatures and accepts valid ed25519 signatures.

The runtime helper being tested performs verification only.

The result policy still records:

    signing = not_performed
    privateKeyAccess = not_allowed
    walletLoading = not_allowed
    liveRpc = not_used
    transactionSubmission = not_allowed
    solSpendAllowed = false

## Scope

Stage 4.13 verifies real ed25519 guardian signatures over a deterministic Stage 4.13 payload.

The payload binds:

    feeBoundMessageDigest
    guardianSetVersion
    guardianPublicKey
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

This means a guardian signature is not accepted merely because it is a valid ed25519 signature.

It must be valid for the exact Stage 4.9 fee-bound digest and Stage 4.11 amount conversion policy.

## Runtime changes

New helper:

    tests/helpers/stage4OfflineCryptographicSignatureVerificationPrototype.ts

New test:

    tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts

## Required source

Stage 4.13 consumes Stage 4.12 production signature verification design result:

    stage4_production_signature_verification_design_result

Required Stage 4.12 conditions:

    stage = 4.12
    executionMode = production_signature_verification_design_offline
    sourceApprovalVerificationStage = 4.10
    sourceApprovalVerificationOk = true
    sourceAmountConversionStage = 4.11
    sourceAmountConversionOk = true
    guardianSetVersion = 1
    guardianCount = 5
    quorumThreshold = 3
    signatureScheme = ed25519
    publicKeyEncoding = base58_x1_guardian_public_key
    signatureEncoding = base64_ed25519_signature
    messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
    feeBoundDigestAlgorithm = sha256_model_hash
    xntdErc20Decimals = 18
    xxxlX1Decimals = 9
    xntdPerXxxl = 100000000
    xntdRawPerXxxlRaw = 100000000000000000
    productionVerificationTarget is bound to the same digest and amount conversion fields
    signing = not_performed
    privateKeyAccess = not_allowed
    walletLoading = not_allowed
    transactionSubmission = not_allowed
    solSpendAllowed = false
    ok = true

## Guardian set

Stage 4.13 requires exactly 5 guardian public keys.

Each guardian public key must be:

    base58_x1_guardian_public_key
    decoded to exactly 32 bytes
    unique
    free of forbidden secret-like markers

The quorum threshold is fixed:

    3-of-5

Accepted quorum sizes:

    3-of-5
    4-of-5
    5-of-5

Rejected:

    2-of-5

## Signature input

New input type:

    Stage4OfflineGuardianSignatureInput

Fields:

    guardianPublicKey
    guardianSetVersion
    approvalKind
    signatureBase64

Required values:

    guardianSetVersion = 1
    approvalKind = fee_bound_message_digest_approval
    signatureBase64 = base64_ed25519_signature

The signature must be 64 bytes after base64 decoding.

The guardian public key must be known in the fixed 5 guardian set.

Duplicate signatures from the same guardian are rejected.

Unknown guardians are rejected.

Invalid ed25519 signatures are rejected.

## Signed payload

New helper:

    buildStage4OfflineCryptographicSignaturePayloadPrototype

Payload structure:

    stage4.13
    feeBoundMessageDigest=<digest>
    guardianSetVersion=<version>
    guardianPublicKey=<guardianPublicKey>
    xntdRawPerXxxlRaw=<ratio>
    burnedXntdRaw=<burnedXntdRaw>
    xxxlMintRaw=<xxxlMintRaw>
    messageBinding=<binding>

Joined by:

    |

This is still a model-stage payload format.

It is deterministic and explicit.

It is bound to the fee digest, guardian identity, guardian set version, and amount conversion policy.

## New result artifact

New result type:

    Stage4OfflineCryptographicSignatureVerificationResult

Artifact type:

    stage4_offline_cryptographic_signature_verification_result

Schema version:

    1

Stage:

    4.13

Execution mode:

    offline_cryptographic_signature_verification

Fields:

    verifiedAtIso
    sourceDesignStage
    sourceDesignOk
    guardianSetVersion
    guardianCount
    quorumThreshold
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
    guardianPublicKeys
    verifiedGuardianPublicKeys
    verifiedSignatureCount
    quorumReached
    verifiedSignatures
    operations
    policy
    invariants
    ok

## Policy object

The result includes a policy object:

    offlineCryptographicVerificationOnly: true
    signatureSchemeRequired: ed25519
    publicKeyEncodingRequired: base58_x1_guardian_public_key
    signatureEncodingRequired: base64_ed25519_signature
    messageBindingRequired: stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
    exactFeeDigestMatchRequired: true
    exactAmountConversionRequired: true
    fixedGuardianCount: 5
    fixedQuorumThreshold: 3
    duplicateSignatureHandling: reject
    unknownGuardianHandling: reject
    invalidSignatureHandling: reject
    guardianSetVersionBound: 1
    signing: not_performed
    privateKeyAccess: not_allowed
    walletLoading: not_allowed
    liveRpc: not_used
    transactionSubmission: not_allowed
    solSpendAllowed: false

## Invariants

Stage 4.13 invariants:

    offlineOnly: true
    realEd25519Verification: true
    noSigning: true
    noPrivateKeys: true
    noWalletLoaded: true
    noLiveRpc: true
    noTransactionsSubmitted: true
    noSolSpend: true
    feeBoundMessageDigestBound: true
    amountConversionPolicyBound: true
    exactFeeDigestMatch: true
    exactAmountConversion: true
    boundToGuardianSetVersion: true
    exactlyFiveGuardians: true
    threeOfFiveQuorum: true
    duplicateSignaturesRejected: true
    unknownGuardiansRejected: true
    invalidSignaturesRejected: true

All invariants must remain true.

## Allowed operations

New type:

    Stage4OfflineCryptographicSignatureVerificationOperation

Allowed operations:

    validateProductionSignatureVerificationDesign
    decodeGuardianPublicKey
    decodeGuardianSignature
    verifyEd25519Signature
    recordOfflineCryptographicVerification

Rejected example operations:

    sendTransaction
    signMessage

## Error model

New class:

    Stage4OfflineCryptographicSignatureVerificationError

New reason type:

    Stage4OfflineCryptographicSignatureVerificationErrorReason

Reasons:

    invalid_verified_at_iso
    invalid_production_signature_verification_design
    production_signature_verification_design_not_ok
    invalid_guardian_set
    duplicate_guardian_public_key
    invalid_guardian_public_key
    invalid_signature_input
    duplicate_signature
    unknown_guardian
    invalid_signature
    quorum_not_reached
    forbidden_value
    invalid_offline_cryptographic_signature_verification_operation

## New helpers

Payload builder:

    buildStage4OfflineCryptographicSignaturePayloadPrototype

Operation assertion helper:

    assertStage4OfflineCryptographicSignatureVerificationOperationPrototype

Verification runner:

    runStage4OfflineCryptographicSignatureVerificationPrototype

Result checker:

    checkStage4OfflineCryptographicSignatureVerificationResultPrototype

## Successful verification test

Confirmed behavior:

- verifies a real offline ed25519 3-of-5 guardian signature quorum
- uses @noble/curves/ed25519 for verification
- accepts valid ed25519 signatures over the deterministic Stage 4.13 payload
- verifiedSignatureCount = 3
- quorumReached = true
- signatureScheme = ed25519
- publicKeyEncoding = base58_x1_guardian_public_key
- signatureEncoding = base64_ed25519_signature
- messageBinding = stage4_9_fee_bound_digest_and_stage4_11_amount_conversion_policy
- verifiedFeeBoundMessageDigest is bound
- xntdRawPerXxxlRaw = 100000000000000000
- burnedXntdRaw = 100000000000000000000000000
- xxxlMintRaw = 1000000000
- checkStage4OfflineCryptographicSignatureVerificationResultPrototype returns true

## Quorum test

Confirmed behavior:

- accepts 3-of-5
- accepts 4-of-5
- accepts 5-of-5
- rejects 2-of-5 as quorum_not_reached

## Safe result JSON test

Confirmed behavior:

- offline crypto result JSON does not contain wallet path
- offline crypto result JSON does not contain private key markers
- offline crypto result JSON does not contain signing methods
- offline crypto result JSON does not contain serialized transaction marker
- offline crypto result JSON does not contain transaction submission methods
- offline crypto result JSON does not contain live_rpc marker

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

- malformed verifiedAtIso is rejected as invalid_verified_at_iso
- failed Stage 4.12 design is rejected as production_signature_verification_design_not_ok
- guardian set with fewer than 5 guardians is rejected as invalid_guardian_set
- duplicate guardian in guardian set is rejected as duplicate_guardian_public_key
- duplicate signature from the same guardian is rejected as duplicate_signature
- unknown guardian signature is rejected as unknown_guardian
- invalid ed25519 signature is rejected as invalid_signature
- forbidden signature marker is rejected as forbidden_value
- sendTransaction operation is rejected as invalid_offline_cryptographic_signature_verification_operation
- signMessage operation is rejected as invalid_offline_cryptographic_signature_verification_operation

## Stage 4.13 test

Command:

    TS_NODE_TRANSPILE_ONLY=1 npx mocha -r ts-node/register \
      tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts

Result:

    Stage 4.13 offline cryptographic signature verification boundary
      ✔ verifies a real offline ed25519 3-of-5 guardian signature quorum
      ✔ accepts 3-of-5, 4-of-5, and 5-of-5 while rejecting 2-of-5
      ✔ keeps offline crypto result JSON free of wallet paths, secrets, signing methods, live RPC, and transaction submission methods
      ✔ rejects malformed metadata, invalid design, bad guardian set, duplicate signatures, unknown guardians, invalid signatures, forbidden values, and invalid operations

    4 passing

## Stage 4.1 through Stage 4.13 smoke

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
      tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts

Result:

    44 passing

## Stage 3.10 plus Stage 4.1 through Stage 4.13 smoke

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
      tests/stage4_offline_cryptographic_signature_verification_boundary.test.ts

Result:

    47 passing

## Formatting and safety checks

Prettier check:

    All matched files use Prettier code style.

Diff check:

    clean

Exact safety marker verification:

    ok

Pasted terminal fragments check:

    clean

No SOL is spent by this stage.

No live RPC is required by this stage.

No wallet is loaded by this stage.

No production private key material was introduced.

No production signing path was introduced.

No transaction submission path was introduced.

## Boundary classification

Stage 4.13 is:

    offline cryptographic signature verification boundary
    real ed25519 verification boundary
    3-of-5 guardian signature quorum boundary
    duplicate signature rejection boundary
    unknown guardian rejection boundary
    invalid signature rejection boundary
    fee-bound digest and amount conversion binding boundary
    offline model boundary

Stage 4.13 is not:

    production guardian key custody boundary
    wallet access boundary
    private key export boundary
    live RPC boundary
    signing boundary
    transaction submission boundary
    live-send boundary

## Current conclusion

Stage 4.13 introduces real offline ed25519 verification.

It verifies guardian signatures over a deterministic payload bound to:

    Stage 4.9 fee-bound message digest
    Stage 4.11 exact XNTD -> XXXL amount conversion policy
    guardianSetVersion
    guardianPublicKey

It accepts 3-of-5, 4-of-5, and 5-of-5 valid guardian signatures.

It rejects 2-of-5, duplicate signatures, unknown guardians, and invalid signatures.

The next valid stage is Stage 4.14 cryptographic verification receipt boundary.
