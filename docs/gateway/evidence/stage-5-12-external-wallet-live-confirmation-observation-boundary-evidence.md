# Stage 5.12 Evidence — External Wallet Live Confirmation Observation Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-12-external-wallet-live-confirmation-observation-boundary

Runtime commit:

    443168d Add Stage 5.12 external wallet live confirmation observation boundary

Final Stage 5.12 runtime commit:

    443168d

## Purpose

Stage 5.12 defines confirmation observation intake for a live submit already performed by the external wallet or external signer layer.

This stage records only an external confirmation observation summary and digest.

Stage 5.12 does not query the chain.

Stage 5.12 does not perform runtime RPC.

Stage 5.12 does not perform runtime confirmation observation.

Stage 5.12 does not perform runtime simulation.

Stage 5.12 does not submit a transaction.

Stage 5.12 does not spend SOL.

Stage 5.12 does not load a signer.

Stage 5.12 does not access keypairs, private keys, seed phrases, wallet files, raw payload bytes, or raw signatures.

Stage 5.12 does not create a transaction object.

Stage 5.12 does not produce transaction serialization.

Stage 5.12 does not release the quarantine.

Stage 5.12 only records that an external wallet or external signer reported confirmation observation for a previously submitted transaction.

A successful Stage 5.12 observation still does not close the live-send flow. Completion closure requires a separate later stage.

## Runtime files added

    tests/helpers/stage5ExternalWalletLiveConfirmationObservationPrototype.ts
    tests/stage5_external_wallet_live_confirmation_observation_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_live_confirmation_observation_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.12
    executionMode: external_wallet_live_confirmation_observation_offline

Raw observation marker:

    stage5_external_wallet_live_confirmation_observation

Raw policy marker:

    stage5_external_wallet_live_confirmation_observation_policy

Raw gate marker:

    stage5_external_wallet_live_confirmation_observation_gate

## Source dependency

Stage 5.12 requires the Stage 5.11 external wallet live submit receipt artifact:

    stage5_external_wallet_live_submit_receipt_result

Required Stage 5.11 final runtime commit:

    a7a833d

Stage 5.12 also binds the prior runtime lineage:

    sourceStage5SubmitAuthorizationRuntimeCommit: 1093c5a
    sourceStage4RuntimeCommit: 69f3c5b

## External wallet live confirmation observation

Stage 5.12 defines this observation object:

    observationKind: stage5_external_wallet_live_confirmation_observation
    confirmationObservationSource: external_wallet_or_external_signer_only
    confirmationObservationMode: external_confirmation_observation_digest_summary_only
    externalConfirmationStatus: external_wallet_live_confirmation_observed
    runtimeConfirmationObservationStatus: not_performed_in_stage5_12
    runtimeRpcCallStatus: not_performed_in_stage5_12
    runtimeSimulationStatus: not_performed_in_stage5_12
    runtimeSubmissionStatus: not_performed_in_stage5_12
    runtimeSolSpendStatus: not_performed_in_stage5_12
    runtimeSignerStatus: not_loaded_stage5_12
    rawPayloadBytesStatus: not_required_not_stored_stage5_12
    rawSignatureStatus: not_required_not_stored_stage5_12
    walletSignatureMaterialStatus: external_to_runtime_stage5_12
    quarantineReleaseStatus: not_released_in_stage5_12
    runtimeConfirmationExecutionStatus: not_allowed_in_stage5_12
    liveConfirmationObservationStatus: external_wallet_confirmation_observation_recorded
    futureCompletionClosureRequiresSeparateStageLater: true

The external confirmation observation summary binds:

    externalConfirmationObservationDigest
    externalConfirmationSlot

The Stage 5.12 observation binds:

    sourceStage5SubmitReceiptDigest
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
    externalSubmitReceiptDigest
    externalTransactionSignatureDigest
    externalSubmitSlot

## External wallet live confirmation observation policy

Stage 5.12 defines this policy:

    policyKind: stage5_external_wallet_live_confirmation_observation_policy
    confirmationObservationSource: external_wallet_or_external_signer_only
    confirmationObservationMode: external_confirmation_observation_digest_summary_only
    sourceStage5SubmitReceiptRequired: stage5_external_wallet_live_submit_receipt_result
    sourceStage5RuntimeCommitRequired: a7a833d
    sourceStage4RuntimeCommitRequired: 69f3c5b
    runtimeCannotPerformRpcInStage512: true
    runtimeCannotObserveConfirmationInStage512: true
    runtimeCannotSubmitInStage512: true
    runtimeCannotSpendSolInStage512: true
    runtimeCannotSignForUser: true
    runtimeCannotCreateWalletSignature: true
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    quarantineRelease: not_allowed
    externalConfirmationObservationOnly: true
    futureCompletionClosureRequiresSeparateStageLater: true

## External wallet live confirmation observation gate

Stage 5.12 defines this gate:

    gateKind: stage5_external_wallet_live_confirmation_observation_gate
    confirmationObservationRecorded: true
    sourceSubmitReceiptRequired: true
    runtimeMayPerformRpcInStage512: false
    runtimeMayObserveConfirmationInStage512: false
    runtimeMaySubmitLiveTransactionInStage512: false
    runtimeMaySpendSolInStage512: false
    runtimeMayLoadSignerInStage512: false
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    quarantineReleaseStatus: not_released_in_stage5_12
    mayProceedToLiveCompletionClosureLater: true
    futureCompletionClosureRequiresSeparateStageLater: true

## Digest binding

Stage 5.12 derives an external wallet live confirmation observation digest from:

    externalWalletLiveConfirmationObservationKind: stage5_external_wallet_live_confirmation_observation
    sourceStage5SubmitReceiptArtifact: stage5_external_wallet_live_submit_receipt_result
    sourceStage5SubmitReceiptStage: 5.11
    sourceStage5SubmitReceiptDigest
    sourceStage5RuntimeCommit: a7a833d
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
    sourceStage4RuntimeCommit: 69f3c5b
    signedPayloadReferenceDigest
    externallySignedPayloadDigest
    externallySignedPayloadByteLength
    externalSubmitReceiptDigest
    externalTransactionSignatureDigest
    externalSubmitSlot
    externalConfirmationObservationDigest
    externalConfirmationSlot
    externalConfirmationStatus: external_wallet_live_confirmation_observed
    confirmationObservationDigest
    confirmationObservationPolicyDigest
    confirmationObservationGateDigest

The confirmation observation digest changes if the Stage 5.11 submit receipt changes.

The confirmation observation digest also changes if the external confirmation observation digest, confirmation slot, or payload reference changes.

## Policy boundary

Stage 5.12 policy states:

    externalWalletLiveConfirmationObservationOnly: true
    sourceStage5SubmitReceiptRequired: stage5_external_wallet_live_submit_receipt_result
    sourceStage5RuntimeCommitRequired: a7a833d
    sourceStage4RuntimeCommitRequired: 69f3c5b
    confirmationObservationSource: external_wallet_or_external_signer_only
    confirmationObservationMode: external_confirmation_observation_digest_summary_only
    runtimeRpcCall: not_performed
    runtimeConfirmationObservation: not_performed
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
    futureCompletionClosureRequiresSeparateStageLater: true

## Invariants

Stage 5.12 preserves these invariants:

    sourceStage5SubmitReceiptBound: true
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
    externalWalletLiveConfirmationObservationBound: true
    externalWalletLiveConfirmationObservationPolicyBound: true
    externalWalletLiveConfirmationObservationGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    externalConfirmationObservationOnly: true
    noRuntimeRpcCall: true
    noRuntimeConfirmationObservation: true
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
    liveConfirmationWasExternalOnly: true
    runtimeConfirmationNotPerformed: true
    futureCompletionClosureRequiresSeparateStageLater: true

## Negative coverage

Stage 5.12 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.11 external wallet live submit receipt artifacts.
- Wrong Stage 5.11 runtime commit references.
- Invalid external confirmation observation digests.
- Invalid external confirmation slots.
- External wallet live confirmation observation digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

Negative validation markers:

    invalid_stage5_external_wallet_submit_receipt
    invalid_external_confirmation_observation

## Checks performed

Runtime checks passed:

    Stage 5.12 source check before full smoke: passed
    Stage 5.12 test: 5 passing
    Stage 5.11 + Stage 5.12 smoke: 10 passing
    Stage 3.10 + Stage 4.1 through Stage 5.12 full smoke: 131 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.12 closes the external wallet live confirmation observation boundary.

Stage 5.12 does not perform runtime RPC.

Stage 5.12 does not perform runtime confirmation observation.

Stage 5.12 does not perform runtime simulation.

Stage 5.12 does not perform live transaction submission.

Stage 5.12 does not authorize runtime SOL spend.

Stage 5.12 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, raw payload byte storage, raw signature storage, signature material storage, quarantine release, runtime simulation execution, runtime confirmation observation, or transaction submission.

The next valid stage is:

    Stage 5.13 — external wallet live completion closure boundary

Stage 5.13 should close the externally executed live-send path after external submit receipt and external confirmation observation have both been recorded. It must still not introduce runtime custody, local signer loading, private-key access, wallet-file access, runtime signing, runtime RPC, or runtime SOL spend.
