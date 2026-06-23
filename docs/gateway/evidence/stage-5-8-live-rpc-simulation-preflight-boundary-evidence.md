# Stage 5.8 Evidence — Live RPC Simulation Preflight Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-8-live-rpc-simulation-preflight-boundary

Runtime commit:

    012ea0b Add Stage 5.8 live RPC simulation preflight boundary

## Purpose

Stage 5.8 defines a live RPC simulation preflight boundary for a validated signed-payload quarantine reference.

This stage prepares the boundary for a future simulation receipt.

Stage 5.8 does not make an RPC call.

Stage 5.8 does not run simulation.

Stage 5.8 does not submit a transaction.

Stage 5.8 does not spend SOL.

Stage 5.8 does not load a signer.

Stage 5.8 does not access keypairs, private keys, seed phrases, wallet files, raw payload bytes, or raw signatures.

Stage 5.8 does not create a transaction object.

Stage 5.8 does not produce transaction serialization.

Stage 5.8 does not release the quarantine.

## Runtime files added

    tests/helpers/stage5LiveRpcSimulationPreflightPrototype.ts
    tests/stage5_live_rpc_simulation_preflight_boundary.test.ts

## Artifact introduced

    stage5_live_rpc_simulation_preflight_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.8
    executionMode: live_rpc_simulation_preflight_offline

Raw preflight marker:

    stage5_live_rpc_simulation_preflight

Raw checklist marker:

    stage5_live_rpc_simulation_preflight_checklist

Raw submission block marker:

    stage5_simulation_submission_block

## Source dependency

Stage 5.8 requires the Stage 5.7 signed payload quarantine validation artifact:

    stage5_signed_payload_quarantine_validation_result

Required Stage 5.7 runtime commit:

    d32b11a

Stage 5.8 also binds the prior runtime lineage:

    sourceStage5QuarantineRuntimeCommit: f34cba3
    sourceStage4RuntimeCommit: 69f3c5b

## Live RPC simulation preflight

Stage 5.8 defines this preflight object:

    preflightKind: stage5_live_rpc_simulation_preflight
    rpcLayer: x1_live_rpc_simulation_layer
    rpcUrlStatus: redacted_not_stored_stage5_8
    simulationScope: preflight_only_no_rpc_call_stage5_8
    simulationMethodIntent: simulate_transaction_later_only
    simulationExecutionStatus: not_performed_in_stage5_8
    rawPayloadBytesStatus: not_required_not_stored_stage5_8
    rawSignatureStatus: not_required_not_stored_stage5_8
    walletSignatureMaterialStatus: external_to_runtime_stage5_8
    runtimeSignerStatus: not_loaded_stage5_8
    transactionSubmissionStatus: not_allowed_in_stage5_8
    solSpendStatus: not_allowed_in_stage5_8
    liveRpcSendStatus: not_allowed_in_stage5_8
    liveRpcReadStatus: not_called_in_stage5_8
    quarantineReleaseStatus: not_released_in_stage5_8
    simulationRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

The preflight binds:

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

## Simulation preflight checklist

Stage 5.8 defines this checklist:

    checklistKind: stage5_live_rpc_simulation_preflight_checklist
    sourceQuarantineValidationDigestMustMatch: true
    sourceQuarantineDigestMustMatch: true
    signedPayloadReferenceDigestMustMatch: true
    externallySignedPayloadDigestMustBeDigest: true
    externallySignedPayloadByteLengthMustBePositiveSafeInteger: true
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    rpcUrlMustRemainRedacted: true
    simulationMustNotRunInStage58: true
    submitMustRemainBlocked: true
    solSpendMustRemainBlocked: true
    localSignerMustRemainBlocked: true
    liveSubmitRequiresSeparateStageLater: true

## Simulation submission block

Stage 5.8 defines this submission block:

    blockKind: stage5_simulation_submission_block
    simulationPreflightDefined: true
    simulationExecutionStatus: not_performed_in_stage5_8
    transactionSubmissionStatus: not_allowed_in_stage5_8
    solSpendStatus: not_allowed_in_stage5_8
    localSignerLoadingStatus: not_allowed_in_stage5_8
    walletFileAccessStatus: not_allowed_in_stage5_8
    rawPayloadBytesStatus: external_only_stage5_8
    rawSignatureStatus: external_only_stage5_8
    mayProceedToSimulationExecutionLater: true
    maySubmitLiveTransactionInStage58: false
    maySpendSolInStage58: false
    simulationRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Digest binding

Stage 5.8 derives a live RPC simulation preflight digest from:

    liveRpcSimulationPreflightKind: stage5_live_rpc_simulation_preflight
    sourceStage5QuarantineValidationArtifact: stage5_signed_payload_quarantine_validation_result
    sourceStage5QuarantineValidationStage: 5.7
    sourceStage5QuarantineValidationDigest
    sourceStage5RuntimeCommit: d32b11a
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
    simulationPreflightDigest
    simulationChecklistDigest
    simulationSubmissionBlockDigest

The simulation preflight digest changes if the Stage 5.7 quarantine validation changes.

The simulation preflight digest also changes if the external signed payload digest or byte length changes.

## Policy boundary

Stage 5.8 policy states:

    liveRpcSimulationPreflightOnly: true
    sourceStage5QuarantineValidationRequired: stage5_signed_payload_quarantine_validation_result
    sourceStage5RuntimeCommitRequired: d32b11a
    sourceStage4RuntimeCommitRequired: 69f3c5b
    rpcLayer: x1_live_rpc_simulation_layer
    rpcUrlStorage: redacted_not_stored_stage5_8
    simulationScope: preflight_only_no_rpc_call_stage5_8
    simulationExecution: not_performed
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
    liveRpcCall: not_performed
    simulationRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.8 preserves these invariants:

    sourceStage5QuarantineValidationBound: true
    sourceStage5QuarantineBound: true
    sourceStage5ApprovalDecisionReceiptBound: true
    sourceStage5ApprovalPreflightBound: true
    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    simulationPreflightBound: true
    simulationPreflightChecklistBound: true
    simulationSubmissionBlockBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    noRpcCall: true
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
    noSimulationExecution: true
    quarantineReleaseBlocked: true
    liveSendNotAuthorized: true
    simulationRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.8 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.7 quarantine validation artifacts.
- Wrong Stage 5.7 runtime commit references.
- Live RPC simulation preflight digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

Negative validation marker:

    invalid_stage5_quarantine_validation

## Checks performed

Runtime checks passed:

    Stage 5.8 strict final marker check: passed
    Stage 5.8 test: 4 passing
    Stage 5.7 + Stage 5.8 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 5.8 full smoke: 109 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.8 closes the live RPC simulation preflight boundary.

Stage 5.8 does not make an RPC call.

Stage 5.8 does not run simulation.

Stage 5.8 does not authorize live transaction submission.

Stage 5.8 does not authorize SOL spend.

Stage 5.8 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, raw payload byte storage, raw signature storage, signature material storage, quarantine release, simulation execution, or transaction submission.

The next valid stage is:

    Stage 5.9 — external wallet live RPC simulation receipt boundary

Stage 5.9 should define receipt intake for a simulation performed by the external wallet or external signer layer. It must still keep raw signing material outside runtime and still must not authorize live submission or SOL spend.
