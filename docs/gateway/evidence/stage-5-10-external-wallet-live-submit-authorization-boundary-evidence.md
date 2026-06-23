# Stage 5.10 Evidence — External Wallet Live Submit Authorization Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-10-external-wallet-live-submit-authorization-boundary

Runtime commits:

    e53ed49 Add Stage 5.10 external wallet live submit authorization boundary
    1093c5a Fix Stage 5.10 submit authorization test markers

Final Stage 5.10 runtime commit:

    1093c5a

## Purpose

Stage 5.10 defines an external wallet live submit authorization boundary after a successful external wallet simulation receipt.

This stage records only an external submit authorization summary and digest.

Stage 5.10 does not submit a transaction.

Stage 5.10 does not spend SOL.

Stage 5.10 does not perform runtime RPC.

Stage 5.10 does not perform runtime simulation.

Stage 5.10 does not load a signer.

Stage 5.10 does not access keypairs, private keys, seed phrases, wallet files, raw payload bytes, or raw signatures.

Stage 5.10 does not create a transaction object.

Stage 5.10 does not produce transaction serialization.

Stage 5.10 does not release the quarantine.

Stage 5.10 requires the Stage 5.9 external simulation receipt to have succeeded.

If the Stage 5.9 simulation receipt failed, Stage 5.10 blocks submit authorization.

A successful Stage 5.10 authorization still does not execute live submit. Live submit execution requires a separate later stage.

## Runtime files added

    tests/helpers/stage5ExternalWalletLiveSubmitAuthorizationPrototype.ts
    tests/stage5_external_wallet_live_submit_authorization_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_live_submit_authorization_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.10
    executionMode: external_wallet_live_submit_authorization_offline

Raw authorization marker:

    stage5_external_wallet_live_submit_authorization

Raw policy marker:

    stage5_external_wallet_live_submit_authorization_policy

Raw gate marker:

    stage5_external_wallet_live_submit_authorization_gate

## Source dependency

Stage 5.10 requires the Stage 5.9 external wallet live RPC simulation receipt artifact:

    stage5_external_wallet_live_rpc_simulation_receipt_result

Required Stage 5.9 final runtime commit:

    3775577

Stage 5.10 also binds the prior runtime lineage:

    sourceStage5LiveRpcSimulationPreflightRuntimeCommit: 012ea0b
    sourceStage4RuntimeCommit: 69f3c5b

## External wallet live submit authorization

Stage 5.10 defines this authorization object:

    authorizationKind: stage5_external_wallet_live_submit_authorization
    simulationOutcome: external_wallet_simulation_succeeded
    authorizationSource: external_wallet_or_external_signer_only
    authorizationMode: external_submit_authorization_digest_summary_only
    runtimeSubmissionStatus: not_performed_in_stage5_10
    runtimeSolSpendStatus: not_performed_in_stage5_10
    runtimeSignerStatus: not_loaded_stage5_10
    runtimeRpcCallStatus: not_performed_in_stage5_10
    runtimeSimulationStatus: not_performed_in_stage5_10
    rawPayloadBytesStatus: not_required_not_stored_stage5_10
    rawSignatureStatus: not_required_not_stored_stage5_10
    walletSignatureMaterialStatus: external_to_runtime_stage5_10
    quarantineReleaseStatus: not_released_in_stage5_10
    liveSubmitAuthorizationStatus: external_wallet_submit_authorized_for_later_stage
    liveSubmitExecutionStatus: not_allowed_in_stage5_10
    liveSubmitRequiresSeparateStageLater: true

The external authorization summary binds:

    externalSubmitAuthorizationDigest
    externalSubmitAuthorizationDeadlineSlot

The Stage 5.10 authorization binds:

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

## External wallet live submit authorization policy

Stage 5.10 defines this policy:

    policyKind: stage5_external_wallet_live_submit_authorization_policy
    authorizationSource: external_wallet_or_external_signer_only
    authorizationMode: external_submit_authorization_digest_summary_only
    successfulExternalSimulationRequired: true
    failedSimulationMustBlockAuthorization: true
    runtimeCannotSubmitInStage510: true
    runtimeCannotSpendSolInStage510: true
    runtimeCannotSignForUser: true
    runtimeCannotCreateWalletSignature: true
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    quarantineRelease: not_allowed
    futureSubmitExecutionRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## External wallet live submit authorization gate

Stage 5.10 defines this gate:

    gateKind: stage5_external_wallet_live_submit_authorization_gate
    submitAuthorizationRecorded: true
    sourceSimulationOutcomeRequired: external_wallet_simulation_succeeded
    failedSimulationBlocksAuthorization: true
    runtimeMaySubmitLiveTransactionInStage510: false
    runtimeMaySpendSolInStage510: false
    runtimeMayLoadSignerInStage510: false
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    quarantineReleaseStatus: not_released_in_stage5_10
    mayProceedToExternalSubmitReceiptLater: true
    liveSubmitRequiresSeparateStageLater: true

## Digest binding

Stage 5.10 derives an external wallet live submit authorization digest from:

    externalWalletLiveSubmitAuthorizationKind: stage5_external_wallet_live_submit_authorization
    sourceStage5SimulationReceiptArtifact: stage5_external_wallet_live_rpc_simulation_receipt_result
    sourceStage5SimulationReceiptStage: 5.9
    sourceStage5SimulationReceiptDigest
    sourceStage5RuntimeCommit: 3775577
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
    simulationOutcome: external_wallet_simulation_succeeded
    externalSubmitAuthorizationDigest
    externalSubmitAuthorizationDeadlineSlot
    submitAuthorizationDigest
    submitAuthorizationPolicyDigest
    submitAuthorizationGateDigest

The submit authorization digest changes if the Stage 5.9 simulation receipt changes.

The submit authorization digest also changes if the external submit authorization digest, deadline slot, payload reference, or prior evidence lineage changes.

## Policy boundary

Stage 5.10 policy states:

    externalWalletLiveSubmitAuthorizationOnly: true
    sourceStage5SimulationReceiptRequired: stage5_external_wallet_live_rpc_simulation_receipt_result
    sourceStage5RuntimeCommitRequired: 3775577
    sourceStage4RuntimeCommitRequired: 69f3c5b
    authorizationSource: external_wallet_or_external_signer_only
    authorizationMode: external_submit_authorization_digest_summary_only
    successfulExternalSimulationRequired: true
    failedSimulationMustBlockAuthorization: true
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
    futureSubmitExecutionRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.10 preserves these invariants:

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
    externalWalletLiveSubmitAuthorizationBound: true
    externalWalletLiveSubmitAuthorizationPolicyBound: true
    externalWalletLiveSubmitAuthorizationGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    externalSubmitAuthorizationOnly: true
    successfulSimulationRequired: true
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
    liveSendNotAuthorized: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.10 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.9 external wallet simulation receipt artifacts.
- Failed external wallet simulation receipts.
- Wrong Stage 5.9 runtime commit references.
- Invalid external submit authorization digests.
- Invalid external submit authorization deadline slots.
- External wallet live submit authorization digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

Negative validation markers:

    invalid_stage5_external_wallet_simulation_receipt
    failed_simulation_blocks_submit_authorization
    invalid_external_submit_authorization

## Checks performed

Runtime checks passed:

    Stage 5.10 source check after marker fix: passed
    Stage 5.10 test after marker fix: 6 passing
    Stage 5.9 + Stage 5.10 smoke after marker fix: 12 passing
    Stage 3.10 + Stage 4.1 through Stage 5.10 full smoke after marker fix: 121 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.10 closes the external wallet live submit authorization boundary.

Stage 5.10 does not perform runtime RPC.

Stage 5.10 does not perform runtime simulation.

Stage 5.10 does not perform live transaction submission.

Stage 5.10 does not authorize runtime SOL spend.

Stage 5.10 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, raw payload byte storage, raw signature storage, signature material storage, quarantine release, runtime simulation execution, or transaction submission.

The next valid stage is:

    Stage 5.11 — external wallet live submit receipt boundary

Stage 5.11 should define receipt intake for a live submit performed by the external wallet or external signer layer. It must still not introduce runtime custody, local signer loading, private-key access, wallet-file access, runtime signing, or runtime SOL spend.
