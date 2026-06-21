# Stage 4.15 Receipt-Bound Transaction Preflight Boundary Evidence

This document records Stage 4.15 receipt-bound transaction preflight boundary evidence for the X1 direct mint gateway live runtime / operations layer.

## Runtime repository

Runtime repo:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-4-15-receipt-bound-transaction-preflight-boundary

Runtime commit:

    723a21d Add Stage 4.15 receipt-bound transaction preflight boundary

Base runtime commit:

    7624afa Add Stage 4.14 cryptographic verification receipt boundary

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

Stage 4.14 created a deterministic cryptographic verification receipt over the already verified Stage 4.13 result.

Stage 4.15 consumes the Stage 4.14 receipt and creates a receipt-bound transaction preflight result.

## Scope

Stage 4.15 adds a receipt-bound transaction preflight boundary.

It does not submit a transaction.

It does not sign a transaction.

It does not simulate a transaction.

It does not load a wallet.

It does not access private keys.

It does not export private keys.

It does not use live RPC.

It does not spend SOL.

Stage 4.15 turns a Stage 4.14 cryptographic verification receipt into a no-send transaction preflight envelope that is explicitly bound to:

    sourceReceiptDigest
    sourceResultDigest
    instructionDataDigest
    verifiedFeeBoundMessageDigest
    guardianSetVersion
    verifiedSignatureCount
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

## Required source

Stage 4.15 consumes:

    stage4_cryptographic_verification_receipt_result

Required Stage 4.14 source conditions:

    stage = 4.14
    executionMode = cryptographic_verification_receipt_offline
    sourceVerificationStage = 4.13
    sourceVerificationOk = true
    sourceArtifactType = stage4_offline_cryptographic_signature_verification_result
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
    receiptOnly = true
    sourceResultDigestRequired = true
    receiptDigestRequired = true
    signing = not_performed
    privateKeyAccess = not_allowed
    walletLoading = not_allowed
    liveRpc = not_used
    transactionSubmission = not_allowed
    solSpendAllowed = false
    ok = true

## Runtime changes

New helper:

    tests/helpers/stage4ReceiptBoundTransactionPreflightPrototype.ts

New test:

    tests/stage4_receipt_bound_transaction_preflight_boundary.test.ts

## New result artifact

New result type:

    Stage4ReceiptBoundTransactionPreflightResult

Artifact type:

    stage4_receipt_bound_transaction_preflight_result

Schema version:

    1

Stage:

    4.15

Execution mode:

    receipt_bound_transaction_preflight_no_send

## Receipt-bound instruction data

Stage 4.15 derives receipt-bound instruction data from the Stage 4.14 receipt.

Instruction data fields:

    receiptDigest
    sourceResultDigest
    verifiedFeeBoundMessageDigest
    guardianSetVersion
    verifiedSignatureCount
    xntdRawPerXxxlRaw
    burnedXntdRaw
    xxxlMintRaw
    messageBinding

Instruction data digest:

    instructionDataDigest = sha256(stableStringify(instructionData))

This digest is deterministic for the same receipt-bound input.

It changes when the underlying receipt verification changes.

## Unsigned preflight envelope

New unsigned envelope type:

    Stage4ReceiptBoundUnsignedTransactionEnvelope

Instruction name:

    mint_xxxl_from_receipt_bound_gateway_message

The unsigned preflight envelope binds:

    instructionData
    instructionDataDigest
    sourceReceiptDigest
    sourceResultDigest
    programId
    payerPublicKey
    accountMetas

The envelope explicitly remains no-send:

    signerCount = 0
    requiredSignatureCount = 0
    transactionSerializationStatus = not_created_no_wallet_no_signing
    transactionSubmission = not_allowed
    simulation = not_performed

Important safety improvement versus earlier preflight boundaries:

    Stage 4.15 public result does not expose a JSON key named serializedTransaction.

Instead it records:

    transactionSerializationStatus = not_created_no_wallet_no_signing

This avoids carrying a transaction-like field name forward while still preserving the no-wallet and no-signing invariant.

## Account meta model

Stage 4.15 account roles include:

    program
    payer
    mint
    recipient
    processed_burn_registry
    guardian_quorum
    cryptographic_verification_receipt

All account metas are non-signers in this boundary.

## Planner model

New planner request type:

    Stage4ReceiptBoundTransactionPreflightPlannerRequest

Planner operations:

    validateCryptographicVerificationReceipt
    deriveReceiptBoundInstructionDigest
    buildReceiptBoundNoSendPreflightEnvelope
    runReceiptBoundNoSendPreflightChecks

Planner response must be:

    unsignedOnly = true
    noSend = true
    receiptBound = true

A planner response that is signed, sendable, or not receipt-bound is rejected.

## New policy object

Stage 4.15 policy object:

    receiptBoundPreflightOnly = true
    sourceReceiptRequired = stage4_cryptographic_verification_receipt_result
    sourceReceiptDigestRequired = true
    sourceResultDigestRequired = true
    instructionDataDigestRequired = true
    exactFeeDigestMatchRequired = true
    exactAmountConversionRequired = true
    walletLoading = not_allowed
    signing = not_performed
    transactionSubmission = not_allowed
    simulation = not_performed
    liveRpc = not_used
    solSpendAllowed = false

## New invariants

Stage 4.15 invariants:

    offlineOnly = true
    receiptBoundPreflightOnly = true
    sourceReceiptStage414Bound = true
    sourceReceiptDigestBound = true
    sourceResultDigestBound = true
    instructionDataDigestBound = true
    feeBoundMessageDigestBound = true
    amountConversionPolicyBound = true
    exactFeeDigestMatch = true
    exactAmountConversion = true
    boundToGuardianSetVersion = true
    exactlyFiveGuardians = true
    threeOfFiveQuorum = true
    noWalletLoaded = true
    noSigning = true
    noTransactionsSubmitted = true
    noSolSpend = true
    noLiveSend = true
    noLiveRpc = true
    noSerializedTransaction = true
    preflightOnly = true

## New helper functions

New helper functions:

    assertStage4ReceiptBoundTransactionPreflightOperationPrototype
    buildStage4ReceiptBoundInstructionDataPrototype
    deriveStage4ReceiptBoundInstructionDataDigestPrototype
    runStage4ReceiptBoundTransactionPreflightPrototype
    checkStage4ReceiptBoundTransactionPreflightResultPrototype

New error class:

    Stage4ReceiptBoundTransactionPreflightError

## Confirmed successful behavior

Confirmed behavior:

    builds a receipt-bound no-send transaction preflight envelope
    binds sourceReceiptDigest from Stage 4.14
    binds sourceResultDigest from Stage 4.14
    derives instructionDataDigest
    binds verifiedFeeBoundMessageDigest
    binds Stage 4.11 exact amount conversion policy
    binds burnedXntdRaw
    binds xxxlMintRaw
    keeps guardianSetVersion = 1
    keeps guardianCount = 5
    keeps quorumThreshold = 3
    keeps verifiedSignatureCount >= 3
    keeps quorumReached = true
    keeps signerCount = 0
    keeps requiredSignatureCount = 0
    keeps transactionSerializationStatus = not_created_no_wallet_no_signing
    keeps transactionSubmission = not_allowed
    keeps simulation = not_performed
    keeps every account meta non-signer
    checkStage4ReceiptBoundTransactionPreflightResultPrototype returns true

## Confirmed digest stability behavior

Confirmed digest behavior:

    same Stage 4.14 receipt produces same instructionDataDigest
    changed Stage 4.14 receipt changes instructionDataDigest

## Confirmed safe output behavior

Confirmed safe output behavior:

    receipt-bound preflight JSON does not contain wallet path
    receipt-bound preflight JSON does not contain private key markers
    receipt-bound preflight JSON does not contain signing methods
    receipt-bound preflight JSON does not contain live RPC marker
    receipt-bound preflight JSON does not contain serialized transaction key
    receipt-bound preflight JSON does not contain transaction submission methods

## Confirmed rejection behavior

Confirmed rejection behavior:

    malformed preflightAtIso rejected as invalid_preflight_at_iso
    malformed networkName rejected as invalid_network_name
    malformed public key rejected as invalid_public_key
    invalid Stage 4.14 receipt rejected as invalid_cryptographic_verification_receipt
    wrong source receipt stage rejected as invalid_cryptographic_verification_receipt
    wrong expectedReceiptDigest rejected as invalid_digest
    wrong expectedSourceResultDigest rejected as invalid_digest
    wrong expectedInstructionDataDigest rejected as invalid_digest
    forbidden value in receipt amount rejected as forbidden_value
    planner response without receiptBound rejected as planner_returned_unbound_signed_or_sendable_preflight
    failed planner rejected as planner_failed
    sendTransaction operation rejected as invalid_receipt_bound_preflight_operation
    signMessage operation rejected as invalid_receipt_bound_preflight_operation

## Checks passed

Checks passed:

    Stage 4.15 receipt-bound transaction preflight boundary: 4 passing
    Stage 4.1 through Stage 4.15 smoke: 52 passing
    Stage 3.10 plus Stage 4.1 through Stage 4.15 smoke: 55 passing
    Prettier check passed
    git diff --check clean
    exact bad marker check passed
    final amend committed

## Safety conclusion

Stage 4.15 is still a no-send boundary.

It moves the pipeline forward from a verified cryptographic receipt into a transaction-preflight-shaped artifact, but it does not cross into signing, transaction serialization, live RPC, transaction submission, or SOL spend.

The next valid stage is Stage 4.16 receipt-bound transaction assembly design boundary.
