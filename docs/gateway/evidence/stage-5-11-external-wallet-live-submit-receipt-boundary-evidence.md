# Stage 5.11 Evidence — External Wallet Live Submit Receipt Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-11-external-wallet-live-submit-receipt-boundary

Runtime commit:

    a7a833d Add Stage 5.11 external wallet live submit receipt boundary

Final Stage 5.11 runtime commit:

    a7a833d

## Purpose

Stage 5.11 defines receipt intake for a live submit performed by the external wallet or external signer layer.

This stage records only an external submit receipt summary and digest.

Stage 5.11 does not submit a transaction.

Stage 5.11 does not spend SOL.

Stage 5.11 does not perform runtime RPC.

Stage 5.11 does not perform runtime simulation.

Stage 5.11 does not load a signer.

Stage 5.11 does not access keypairs, private keys, seed phrases, wallet files, raw payload bytes, or raw signatures.

Stage 5.11 does not create a transaction object.

Stage 5.11 does not produce transaction serialization.

Stage 5.11 does not release the quarantine.

Stage 5.11 only records that an external wallet or external signer reported a live submit receipt.

A successful Stage 5.11 receipt still does not perform confirmation observation. Confirmation observation requires a separate later stage.

## Runtime files added

    tests/helpers/stage5ExternalWalletLiveSubmitReceiptPrototype.ts
    tests/stage5_external_wallet_live_submit_receipt_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_live_submit_receipt_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.11
    executionMode: external_wallet_live_submit_receipt_offline

Raw receipt marker:

    stage5_external_wallet_live_submit_receipt

Raw policy marker:

    stage5_external_wallet_live_submit_receipt_policy

Raw gate marker:

    stage5_external_wallet_live_submit_receipt_gate

## Source dependency

Stage 5.11 requires the Stage 5.10 external wallet live submit authorization artifact:

    stage5_external_wallet_live_submit_authorization_result

Required Stage 5.10 final runtime commit:

    1093c5a

Stage 5.11 also binds the prior runtime lineage:

    sourceStage5SimulationReceiptRuntimeCommit: 3775577
    sourceStage4RuntimeCommit: 69f3c5b

## External wallet live submit receipt

Stage 5.11 defines this receipt object:

    receiptKind: stage5_external_wallet_live_submit_receipt
    submitReceiptSource: external_wallet_or_external_signer_only
    submitReceiptMode: external_submit_receipt_digest_summary_only
    externalSubmitStatus: external_wallet_live_submit_recorded
    runtimeSubmissionStatus: not_performed_in_stage5_11
    runtimeSolSpendStatus: not_performed_in_stage5_11
    runtimeSignerStatus: not_loaded_stage5_11
    runtimeRpcCallStatus: not_performed_in_stage5_11
    runtimeSimulationStatus: not_performed_in_stage5_11
    rawPayloadBytesStatus: not_required_not_stored_stage5_11
    rawSignatureStatus: not_required_not_stored_stage5_11
    walletSignatureMaterialStatus: external_to_runtime_stage5_11
    quarantineReleaseStatus: not_released_in_stage5_11
    runtimeSubmitExecutionStatus: not_allowed_in_stage5_11
    liveSubmitReceiptStatus: external_wallet_submit_receipt_recorded
    futureConfirmationObservationRequiresSeparateStageLater: true

The external submit receipt summary binds:

    externalSubmitReceiptDigest
    externalTransactionSignatureDigest
    externalSubmitSlot

The Stage 5.11 receipt binds:

    sourceStage5SubmitAuthorizationDigest
    sourceStage5SimulationReceiptDigest
    sourceStage5LiveRpcSimulationPreflightDigest
    sourceStage5QuarantineValidationDigest
    sourceStage5QuarantineDigest
    sourceStage5ApprovalDecisionReceiptDigest
    sourceStage5ApprovalPreflightDigest
    sourceStage5ExportPackageDigest
    sourceStage5HandoffDigest
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    signedPayloadReferenceDigest
    externallySignedPayloadDigest
    externallySignedPayloadByteLength

## External wallet live submit receipt policy

Stage 5.11 defines this policy:

    policyKind: stage5_external_wallet_live_submit_receipt_policy
    submitReceiptSource: external_wallet_or_external_signer_only
    submitReceiptMode: external_submit_receipt_digest_summary_only
    sourceStage5SubmitAuthorizationRequired: stage5_external_wallet_live_submit_authorization_result
    sourceStage5RuntimeCommitRequired: 1093c5a
    sourceStage4RuntimeCommitRequired: 69f3c5b
    runtimeCannotSubmitInStage511: true
    runtimeCannotSpendSolInStage511: true
    runtimeCannotSignForUser: true
    runtimeCannotCreateWalletSignature: true
    runtimeCannotPerformRpcInStage511: true
    runtimeCannotPerformSimulationInStage511: true
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    quarantineRelease: not_allowed
    externalSubmitReceiptOnly: true
    futureConfirmationObservationRequiresSeparateStageLater: true

## External wallet live submit receipt gate

Stage 5.11 defines this gate:

    gateKind: stage5_external_wallet_live_submit_receipt_gate
    submitReceiptRecorded: true
    sourceSubmitAuthorizationRequired: true
    runtimeMaySubmitLiveTransactionInStage511: false
    runtimeMaySpendSolInStage511: false
    runtimeMayLoadSignerInStage511: false
    runtimeMayPerformRpcInStage511: false
    runtimeMayPerformSimulationInStage511: false
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    quarantineReleaseStatus: not_released_in_stage5_11
    mayProceedToExternalConfirmationObservationLater: true
    futureConfirmationObservationRequiresSeparateStageLater: true

## Digest binding

Stage 5.11 derives an external wallet live submit receipt digest from:

    externalWalletLiveSubmitReceiptKind: stage5_external_wallet_live_submit_receipt
    sourceStage5SubmitAuthorizationArtifact: stage5_external_wallet_live_submit_authorization_result
    sourceStage5SubmitAuthorizationStage: 5.10
    sourceStage5SubmitAuthorizationDigest
    sourceStage5RuntimeCommit: 1093c5a
    sourceStage5SimulationReceiptDigest
    sourceStage5LiveRpcSimulationPreflightDigest
    sourceStage5QuarantineValidationDigest
    sourceStage5QuarantineDigest
    sourceStage5ApprovalDecisionReceiptDigest
    sourceStage5ApprovalPreflightDigest
    sourceStage5ExportPackageDigest
    sourceStage5HandoffDigest
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    signedPayloadReferenceDigest
    externallySignedPayloadDigest
    externallySignedPayloadByteLength
    externalSubmitReceiptDigest
    externalTransactionSignatureDigest
    externalSubmitSlot
    submitReceiptDigest
    submitReceiptPolicyDigest
    submitReceiptGateDigest

The submit receipt digest changes if the Stage 5.10 submit authorization changes.

The submit receipt digest also changes if the external submit receipt digest, external transaction signature digest, submit slot, or payload reference changes.

## Policy boundary

Stage 5.11 policy states:

    externalWalletLiveSubmitReceiptOnly: true
    sourceStage5SubmitAuthorizationRequired: stage5_external_wallet_live_submit_authorization_result
    sourceStage5RuntimeCommitRequired: 1093c5a
    sourceStage4RuntimeCommitRequired: 69f3c5b
    submitReceiptSource: external_wallet_or_external_signer_only
    submitReceiptMode: external_submit_receipt_digest_summary_only
    runtimeRpcCall: not_performed
    runtimeSimulationExecution: not_performed
    runtimeCustody: none
    custodyWalletProduct: out_of_scope
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    walletSignatureCreation: not_performed
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    quarantineRelease: not_allowed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionObjectCreation: not_performed
    transactionSerialization: not_performed
    futureConfirmationObservationRequiresSeparateStageLater: true

## Invariants

Stage 5.11 preserves these invariants:

    sourceStage5SubmitAuthorizationBound: true
    sourceStage5SimulationReceiptBound: true
    sourceStage5LiveRpcSimulationPreflightBound: true
    sourceStage5QuarantineValidationBound: true
    sourceStage5QuarantineBound: true
    sourceStage5ApprovalDecisionReceiptBound: true
    sourceStage5ApprovalPreflightBound: true
    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    externalWalletLiveSubmitReceiptBound: true
    externalWalletLiveSubmitReceiptPolicyBound: true
    externalWalletLiveSubmitReceiptGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    externalSubmitReceiptOnly: true
    noRuntimeRpcCall: true
    noRuntimeCustody: true
    noLocalSignerLoaded: true
    noKeypairAccess: true
    noPrivateKeys: true
    noSeedPhraseAccess: true
    noWalletFileAccess: true
    noRuntimeSigning: true
    noWalletSignatureCreation: true
    noRawPayloadBytesStored: true
    noRawSignatureStored: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noTransactionObjectCreated: true
    noTransactionSerialization: true
    noRuntimeSimulationExecution: true
    quarantineReleaseBlocked: true
    liveSendWasExternalOnly: true
    runtimeLiveSendNotPerformed: true
    futureConfirmationObservationRequiresSeparateStageLater: true

## Negative coverage

Stage 5.11 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.10 external wallet live submit authorization artifacts.
- Wrong Stage 5.10 runtime commit references.
- Invalid external submit receipt digests.
- Invalid external transaction signature digests.
- Invalid external submit slots.
- External wallet live submit receipt digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

Negative validation markers:

    invalid_stage5_external_wallet_submit_authorization
    invalid_external_submit_receipt

## Checks performed

Runtime checks passed:

    Stage 5.11 source check after patch: passed
    Stage 5.11 test after patch: 5 passing
    Stage 5.10 + Stage 5.11 smoke after patch: 11 passing
    Stage 3.10 + Stage 4.1 through Stage 5.11 full smoke: 126 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.11 closes the external wallet live submit receipt boundary.

Stage 5.11 does not perform runtime RPC.

Stage 5.11 does not perform runtime simulation.

Stage 5.11 does not perform live transaction submission.

Stage 5.11 does not authorize runtime SOL spend.

Stage 5.11 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, raw payload byte storage, raw signature storage, signature material storage, quarantine release, runtime simulation execution, or transaction submission.

The next valid stage is:

    Stage 5.12 — external wallet live confirmation observation boundary

Stage 5.12 should define confirmation observation intake for a live submit already performed by the external wallet or external signer layer. It must still not introduce runtime custody, local signer loading, private-key access, wallet-file access, runtime signing, or runtime SOL spend.
