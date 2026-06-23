# Stage 5.9 Evidence — External Wallet Live RPC Simulation Receipt Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-9-external-wallet-live-rpc-simulation-receipt-boundary

Runtime commits:

    d079737 Add Stage 5.9 external wallet live RPC simulation receipt boundary
    3775577 Fix Stage 5.9 simulation receipt test markers

Final Stage 5.9 runtime commit:

    3775577

## Purpose

Stage 5.9 defines receipt intake for a live RPC simulation performed by the external wallet or external signer layer.

This stage records only an external simulation receipt summary and digest.

Stage 5.9 does not perform runtime RPC.

Stage 5.9 does not perform runtime simulation.

Stage 5.9 does not submit a transaction.

Stage 5.9 does not spend SOL.

Stage 5.9 does not load a signer.

Stage 5.9 does not access keypairs, private keys, seed phrases, wallet files, raw payload bytes, or raw signatures.

Stage 5.9 does not create a transaction object.

Stage 5.9 does not produce transaction serialization.

Stage 5.9 does not release the quarantine.

Stage 5.9 records whether external wallet simulation succeeded or failed.

If external simulation failed, the future submit path is blocked.

If external simulation succeeded, future submit is still not authorized in Stage 5.9 and requires a separate later stage.

## Runtime files added

    tests/helpers/stage5ExternalWalletLiveRpcSimulationReceiptPrototype.ts
    tests/stage5_external_wallet_live_rpc_simulation_receipt_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_live_rpc_simulation_receipt_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.9
    executionMode: external_wallet_live_rpc_simulation_receipt_offline

Raw receipt marker:

    stage5_external_wallet_live_rpc_simulation_receipt

Raw policy marker:

    stage5_external_wallet_live_rpc_simulation_receipt_policy

Raw next gate marker:

    stage5_external_wallet_live_rpc_simulation_receipt_next_gate

## Source dependency

Stage 5.9 requires the Stage 5.8 live RPC simulation preflight artifact:

    stage5_live_rpc_simulation_preflight_result

Required Stage 5.8 runtime commit:

    012ea0b

Stage 5.9 also binds the prior runtime lineage:

    sourceStage5QuarantineValidationRuntimeCommit: d32b11a
    sourceStage4RuntimeCommit: 69f3c5b

## External wallet simulation receipt

Stage 5.9 defines this receipt object:

    receiptKind: stage5_external_wallet_live_rpc_simulation_receipt
    walletLayer: existing_x1_wallet_or_external_signer
    simulationReceiptSource: external_wallet_or_external_signer_only
    simulationReceiptMode: external_receipt_digest_summary_only
    runtimeRpcCallStatus: not_performed_in_stage5_9
    runtimeSimulationStatus: not_performed_in_stage5_9
    rawPayloadBytesStatus: not_required_not_stored_stage5_9
    rawSignatureStatus: not_required_not_stored_stage5_9
    walletSignatureMaterialStatus: external_to_runtime_stage5_9
    transactionSubmissionStatus: not_allowed_in_stage5_9
    solSpendStatus: not_allowed_in_stage5_9
    quarantineReleaseStatus: not_released_in_stage5_9
    liveSubmitStatus: not_allowed_in_stage5_9
    successfulSimulationRequiredBeforeFutureSubmit: true
    liveSubmitRequiresSeparateStageLater: true

Supported simulation outcomes:

    external_wallet_simulation_succeeded
    external_wallet_simulation_failed

The external receipt summary binds:

    externalSimulationReceiptDigest
    externalSimulationComputeUnitsConsumed
    externalSimulationSlot
    externalSimulationBlockhashDigest

The Stage 5.9 receipt binds:

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

## External simulation receipt policy

Stage 5.9 defines this policy:

    policyKind: stage5_external_wallet_live_rpc_simulation_receipt_policy
    simulationReceiptSource: external_wallet_or_external_signer_only
    simulationReceiptMode: external_receipt_digest_summary_only
    runtimeCannotPerformRpcSimulationInStage59: true
    runtimeCannotSubmitAfterSimulationReceiptInStage59: true
    runtimeCannotSpendSolInStage59: true
    runtimeCannotSignForUser: true
    runtimeCannotCreateWalletSignature: true
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    failedSimulationMustBlockFutureSubmitPath: true
    successfulSimulationStillRequiresSeparateSubmitStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## External simulation receipt next gate

Stage 5.9 defines this next gate:

    gateKind: stage5_external_wallet_live_rpc_simulation_receipt_next_gate
    simulationReceiptRecorded: true
    runtimeMaySubmitLiveTransactionInStage59: false
    runtimeMaySpendSolInStage59: false
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    quarantineReleaseStatus: not_released_in_stage5_9
    liveSubmitRequiresSeparateStageLater: true

For a successful external simulation:

    successfulOutcomeMayProceedToSubmitAuthorizationLater: true
    failedOutcomeMustStopSubmitPath: false

For a failed external simulation:

    successfulOutcomeMayProceedToSubmitAuthorizationLater: false
    failedOutcomeMustStopSubmitPath: true

## Digest binding

Stage 5.9 derives an external wallet live RPC simulation receipt digest from:

    externalWalletLiveRpcSimulationReceiptKind: stage5_external_wallet_live_rpc_simulation_receipt
    sourceStage5LiveRpcSimulationPreflightArtifact: stage5_live_rpc_simulation_preflight_result
    sourceStage5LiveRpcSimulationPreflightStage: 5.8
    sourceStage5LiveRpcSimulationPreflightDigest
    sourceStage5RuntimeCommit: 012ea0b
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
    simulationOutcome
    externalSimulationReceiptDigest
    externalSimulationComputeUnitsConsumed
    externalSimulationSlot
    externalSimulationBlockhashDigest
    simulationReceiptDigest
    simulationReceiptPolicyDigest
    simulationReceiptNextGateDigest

The receipt digest changes if the Stage 5.8 simulation preflight changes.

The receipt digest also changes if the simulation outcome, external receipt digest, compute units, slot, blockhash digest, or payload reference changes.

## Policy boundary

Stage 5.9 policy states:

    externalWalletLiveRpcSimulationReceiptOnly: true
    sourceStage5LiveRpcSimulationPreflightRequired: stage5_live_rpc_simulation_preflight_result
    sourceStage5RuntimeCommitRequired: 012ea0b
    sourceStage4RuntimeCommitRequired: 69f3c5b
    simulationReceiptSource: external_wallet_or_external_signer_only
    simulationReceiptMode: external_receipt_digest_summary_only
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
    failedSimulationMustBlockFutureSubmitPath: true
    successfulSimulationStillRequiresSeparateSubmitStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.9 preserves these invariants:

    sourceStage5LiveRpcSimulationPreflightBound: true
    sourceStage5QuarantineValidationBound: true
    sourceStage5QuarantineBound: true
    sourceStage5ApprovalDecisionReceiptBound: true
    sourceStage5ApprovalPreflightBound: true
    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    externalWalletSimulationReceiptBound: true
    externalSimulationReceiptPolicyBound: true
    externalSimulationReceiptNextGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    externalReceiptOnly: true
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

Stage 5.9 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.8 live RPC simulation preflight artifacts.
- Wrong Stage 5.8 runtime commit references.
- Invalid simulation outcomes.
- Invalid external simulation receipt digest summaries.
- Invalid external simulation compute units.
- Invalid external simulation slot.
- Invalid external simulation blockhash digest.
- External wallet live RPC simulation receipt digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

Negative validation markers:

    invalid_stage5_live_rpc_simulation_preflight
    invalid_external_simulation_receipt

## Checks performed

Runtime checks passed:

    Stage 5.9 source check after marker fix: passed
    Stage 5.9 test after marker fix: 6 passing
    Stage 5.8 + Stage 5.9 smoke after marker fix: 10 passing
    Stage 3.10 + Stage 4.1 through Stage 5.9 full smoke after marker fix: 115 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.9 closes the external wallet live RPC simulation receipt boundary.

Stage 5.9 does not perform runtime RPC.

Stage 5.9 does not perform runtime simulation.

Stage 5.9 does not authorize live transaction submission.

Stage 5.9 does not authorize SOL spend.

Stage 5.9 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, raw payload byte storage, raw signature storage, signature material storage, quarantine release, runtime simulation execution, or transaction submission.

The next valid stage is:

    Stage 5.10 — external wallet live submit authorization boundary

Stage 5.10 should define a separate submit-authorization boundary after a successful external simulation receipt. It must still not perform runtime transaction submission or SOL spend.
