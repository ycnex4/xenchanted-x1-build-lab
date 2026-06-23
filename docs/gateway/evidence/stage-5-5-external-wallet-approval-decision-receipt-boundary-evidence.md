# Stage 5.5 Evidence — External Wallet Approval Decision Receipt Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-5-external-wallet-approval-decision-receipt-boundary

Runtime commit:

    db6c1b6 Add Stage 5.5 external wallet approval decision receipt boundary

## Purpose

Stage 5.5 defines an offline receipt for a user approval or rejection decision from the external wallet layer.

This stage records only the decision boundary.

Stage 5.5 does not collect a wallet signature.

Stage 5.5 does not receive a signed payload.

Stage 5.5 does not submit a transaction.

Stage 5.5 does not spend SOL.

Stage 5.5 does not create a transaction object, produce transaction serialization, load a local signer, access keypairs, access private keys, access seed phrases, access wallet files, use live RPC, or run simulation.

The approval decision source is the external wallet user only. Runtime cannot approve, reject, sign, or submit for the user.

## Runtime files added

    tests/helpers/stage5ExternalWalletApprovalDecisionReceiptPrototype.ts
    tests/stage5_external_wallet_approval_decision_receipt_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_approval_decision_receipt_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.5
    executionMode: external_wallet_approval_decision_receipt_offline

Raw receipt marker:

    stage5_external_wallet_approval_decision_receipt

Raw policy marker:

    stage5_approval_decision_receipt_policy

Raw next-stage gate marker:

    stage5_approval_decision_next_stage_gate

## Source dependency

Stage 5.5 requires the Stage 5.4 external wallet user-approval preflight artifact:

    stage5_external_wallet_user_approval_preflight_result

Required Stage 5.4 runtime commit:

    165deb7

Stage 5.5 also binds the prior runtime lineage:

    sourceStage5ExportPackageRuntimeCommit: 00a71a1
    sourceStage4RuntimeCommit: 69f3c5b

## Decision values

Stage 5.5 supports exactly two external wallet user decisions:

    external_wallet_user_approved
    external_wallet_user_rejected

Invalid decision values are rejected.

## Approval decision receipt

Stage 5.5 defines this decision receipt object:

    receiptKind: stage5_external_wallet_approval_decision_receipt
    walletLayer: existing_x1_wallet_or_external_signer
    decisionSource: external_wallet_user_only
    decisionReceiptStatus: decision_recorded_offline_only
    walletSignatureStatus: not_collected_in_stage5_5
    signedPayloadStatus: not_available_in_stage5_5
    transactionSubmissionStatus: not_allowed_in_stage5_5
    solSpendStatus: not_allowed_in_stage5_5
    liveRpcStatus: not_used_in_stage5_5
    simulationStatus: not_performed_in_stage5_5
    liveSubmitRequiresSeparateStageLater: true

For the approved path:

    decision: external_wallet_user_approved
    approvedPathRequiresLaterSignedPayload: true
    rejectedPathStopsLiveSend: false

For the rejected path:

    decision: external_wallet_user_rejected
    approvedPathRequiresLaterSignedPayload: false
    rejectedPathStopsLiveSend: true

## Approval decision receipt policy

Stage 5.5 defines this policy:

    policyKind: stage5_approval_decision_receipt_policy
    approvalDecisionSource: external_wallet_user_only
    approvedDecisionValue: external_wallet_user_approved
    rejectedDecisionValue: external_wallet_user_rejected
    runtimeCannotApprove: true
    runtimeCannotRejectForUser: true
    runtimeCannotSignForUser: true
    runtimeCannotSubmitAfterApprovalInStage55: true
    walletSignatureNotCollectedInStage55: true
    signedPayloadIntakeRequiredLaterForApprovedPath: true
    rejectedPathMustNotContinueToLiveSubmit: true
    liveRpcSimulationRequiredLaterForApprovedPath: true
    liveSubmitRequiresSeparateStageLater: true

## Approval decision next-stage gate

Stage 5.5 defines this next-stage gate:

    gateKind: stage5_approval_decision_next_stage_gate
    nextStageGateStatus: defined_offline_only
    liveSubmitRequiresSeparateStageLater: true

For approved decisions:

    mayProceedToSignedPayloadIntakeLater: true
    mustStopLiveSendPath: false
    signedPayloadIntakeStageRequiredLater: true
    liveRpcSimulationStageRequiredLater: true

For rejected decisions:

    mayProceedToSignedPayloadIntakeLater: false
    mustStopLiveSendPath: true
    signedPayloadIntakeStageRequiredLater: false
    liveRpcSimulationStageRequiredLater: false

## Digest binding

Stage 5.5 derives an external wallet approval decision receipt digest from:

    approvalDecisionReceiptKind: stage5_external_wallet_approval_decision_receipt
    sourceStage5ApprovalPreflightArtifact: stage5_external_wallet_user_approval_preflight_result
    sourceStage5ApprovalPreflightStage: 5.4
    sourceStage5ApprovalPreflightDigest
    sourceStage5RuntimeCommit: 165deb7
    sourceStage5ExportPackageDigest
    sourceStage5HandoffDigest
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    decision
    approvalDecisionReceiptDigest
    approvalDecisionReceiptPolicyDigest
    approvalDecisionNextStageGateDigest

The decision receipt digest changes if the Stage 5.4 approval preflight changes.

The decision receipt digest also changes if the decision changes from approval to rejection.

## Policy boundary

Stage 5.5 policy states:

    approvalDecisionReceiptOnly: true
    sourceStage5ApprovalPreflightRequired: stage5_external_wallet_user_approval_preflight_result
    sourceStage5RuntimeCommitRequired: 165deb7
    sourceStage4RuntimeCommitRequired: 69f3c5b
    walletLayer: existing_x1_wallet_or_external_signer
    decisionSource: external_wallet_user_only
    runtimeCustody: none
    custodyWalletProduct: out_of_scope
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    walletSignatureCollection: not_collected_in_stage5_5
    signedPayloadIntake: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionObjectCreation: not_performed
    transactionSerialization: not_performed
    liveRpc: not_used
    simulation: not_performed
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.5 preserves these invariants:

    sourceStage5ApprovalPreflightBound: true
    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    approvalDecisionReceiptBound: true
    approvalDecisionReceiptPolicyBound: true
    approvalDecisionNextStageGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    noRuntimeCustody: true
    noLocalSignerLoaded: true
    noKeypairAccess: true
    noPrivateKeys: true
    noSeedPhraseAccess: true
    noWalletFileAccess: true
    noRuntimeSigning: true
    noWalletSignature: true
    noSignedPayload: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noTransactionObjectCreated: true
    noTransactionSerialization: true
    noLiveRpc: true
    noSimulation: true
    liveSendNotAuthorized: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.5 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.4 approval preflight artifacts.
- Wrong Stage 5.4 runtime commit references.
- Invalid decision values.
- Approval decision receipt digest mismatches.
- Forbidden secret-bearing values.
- Implicit wallet-signature, transaction-submission, or SOL-spend operations.

## Checks performed

Runtime checks passed:

    Stage 5.5 final marker check: passed
    Stage 5.5 test: 5 passing
    Stage 5.4 + Stage 5.5 smoke: 9 passing
    Stage 3.10 + Stage 4.1 through Stage 5.5 full smoke: 96 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.5 closes the external wallet approval decision receipt boundary.

Stage 5.5 does not authorize live transaction submission.

Stage 5.5 does not authorize SOL spend.

Stage 5.5 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, live RPC, simulation, wallet signature collection, signed payload intake, or transaction submission.

The next valid stage is:

    Stage 5.6 — signed payload intake quarantine boundary

Stage 5.6 should define a quarantine boundary for a later externally signed payload on the approved path only. It must still preserve that signing material never enters runtime and must still not authorize live submission or SOL spend.
