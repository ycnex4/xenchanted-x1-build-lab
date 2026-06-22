# Stage 5.4 Evidence — External Wallet User-Approval Preflight Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-4-external-wallet-user-approval-preflight-boundary

Runtime commit:

    165deb7 Add Stage 5.4 external wallet user approval preflight boundary

## Purpose

Stage 5.4 defines the offline external wallet user-approval preflight boundary.

This stage formalizes what an existing X1 wallet or external signer must later show to the user before any approval, wallet signature, signed payload intake, simulation, live transaction submission, or SOL spend.

Stage 5.4 does not request approval.

Stage 5.4 does not collect an approval decision.

Stage 5.4 does not request a wallet signature.

Stage 5.4 does not receive a signed payload.

Stage 5.4 does not submit a transaction.

Stage 5.4 does not spend SOL.

Stage 5.4 does not create a transaction object, produce transaction serialization, load a local signer, access keypairs, access private keys, access seed phrases, access wallet files, use live RPC, or run simulation.

## Runtime files added

    tests/helpers/stage5ExternalWalletUserApprovalPreflightPrototype.ts
    tests/stage5_external_wallet_user_approval_preflight_boundary.test.ts

## Artifact introduced

    stage5_external_wallet_user_approval_preflight_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.4
    executionMode: external_wallet_user_approval_preflight_offline

Raw preflight marker:

    stage5_external_wallet_user_approval_preflight

Raw wallet review checklist marker:

    stage5_wallet_review_checklist

Raw user approval policy marker:

    stage5_user_approval_decision_policy

## Source dependency

Stage 5.4 requires the Stage 5.3 unsigned payload export package artifact:

    stage5_unsigned_payload_export_package_result

Required Stage 5.3 runtime commit:

    00a71a1

Stage 5.4 also binds the prior runtime lineage:

    sourceStage5HandoffRuntimeCommit: 6a1df6e
    sourceStage4RuntimeCommit: 69f3c5b

## External wallet user-approval preflight

Stage 5.4 defines this approval preflight object:

    preflightKind: stage5_external_wallet_user_approval_preflight
    walletLayer: existing_x1_wallet_or_external_signer
    packageFormat: x1_external_wallet_unsigned_payload_package
    reviewMode: wallet_review_preflight_only
    approvalRequestStatus: not_requested_in_stage5_4
    approvalDecisionStatus: not_collected_in_stage5_4
    walletSignatureStatus: not_requested_in_stage5_4
    signedPayloadStatus: not_available_in_stage5_4
    transactionSubmissionStatus: not_allowed_in_stage5_4
    solSpendStatus: not_allowed_in_stage5_4
    liveRpcStatus: not_used_in_stage5_4
    simulationStatus: not_performed_in_stage5_4
    liveSubmitRequiresSeparateStageLater: true

## Wallet review checklist

Stage 5.4 defines this wallet-facing review checklist:

    walletMustDisplayNetworkLater: true
    walletMustDisplayProgramIdLater: true
    walletMustDisplayPayerPublicKeyLater: true
    walletMustDisplayRecipientLater: true
    walletMustDisplayInstructionNameLater: true
    walletMustDisplayAmountLater: true
    walletMustDisplayFeeLater: true
    walletMustDisplaySourceStage4ClosureDigestLater: true
    walletMustDisplaySourceStage5OpeningDigestLater: true
    walletMustDisplaySourceStage5HandoffDigestLater: true
    walletMustDisplayUnsignedPackageDigestLater: true
    walletMustDisplayApprovalWarningLater: true
    walletMustRejectUnexpectedNetworkLater: true
    walletMustRejectUnexpectedProgramIdLater: true
    walletMustRejectUnexpectedRecipientLater: true
    walletMustRejectUnexpectedAmountLater: true
    walletReviewStatus: defined_not_displayed_in_stage5_4

## User approval decision policy

Stage 5.4 defines this approval decision policy:

    approvalDecisionSource: external_wallet_user_only
    runtimeCannotApprove: true
    runtimeCannotSignForUser: true
    runtimeCannotSubmitAfterApprovalInStage54: true
    userMayApproveLater: true
    userMayRejectLater: true
    approvalCollectionStatus: not_collected_in_stage5_4
    approvalResultStatus: not_available_in_stage5_4
    signedPayloadIntakeRequiredLater: true
    liveRpcSimulationRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true

## Digest binding

Stage 5.4 derives an external wallet user-approval preflight digest from:

    approvalPreflightKind: stage5_external_wallet_user_approval_preflight
    sourceStage5ExportPackageArtifact: stage5_unsigned_payload_export_package_result
    sourceStage5ExportPackageStage: 5.3
    sourceStage5ExportPackageDigest
    sourceStage5RuntimeCommit: 00a71a1
    sourceStage5HandoffDigest
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    approvalPreflightDigest
    walletReviewChecklistDigest
    userApprovalDecisionPolicyDigest

The approval preflight digest changes if the Stage 5.3 unsigned payload package changes.

## Policy boundary

Stage 5.4 policy states:

    approvalPreflightOnly: true
    sourceStage5ExportPackageRequired: stage5_unsigned_payload_export_package_result
    sourceStage5RuntimeCommitRequired: 00a71a1
    sourceStage4RuntimeCommitRequired: 69f3c5b
    walletLayer: existing_x1_wallet_or_external_signer
    packageFormat: x1_external_wallet_unsigned_payload_package
    reviewMode: wallet_review_preflight_only
    runtimeCustody: none
    custodyWalletProduct: out_of_scope
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    walletSignatureRequest: not_requested_in_stage5_4
    signedPayloadIntake: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionObjectCreation: not_performed
    transactionSerialization: not_performed
    liveRpc: not_used
    simulation: not_performed
    approvalCollectionStatus: not_collected_in_stage5_4
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.4 preserves these invariants:

    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    approvalPreflightBound: true
    walletReviewChecklistBound: true
    userApprovalDecisionPolicyBound: true
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
    approvalNotCollected: true
    liveSendNotAuthorized: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.4 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.3 unsigned payload package artifacts.
- Wrong Stage 5.3 runtime commit references.
- Approval preflight digest mismatches.
- Forbidden secret-bearing values.
- Implicit approval, wallet-signature, transaction-submission, or SOL-spend operations.

## Checks performed

Runtime checks passed:

    Stage 5.4 strict final marker check: passed
    Stage 5.4 test: 4 passing
    Stage 5.3 + Stage 5.4 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 5.4 full smoke: 91 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.4 closes the external wallet user-approval preflight boundary.

Stage 5.4 does not authorize live transaction submission.

Stage 5.4 does not authorize SOL spend.

Stage 5.4 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, live RPC, simulation, approval collection, signed payload intake, or transaction submission.

The next valid stage is:

    Stage 5.5 — external wallet approval decision receipt boundary

Stage 5.5 should define an offline receipt for a user approval or rejection decision from the external wallet layer, while still keeping signing material outside runtime and still not authorizing live submission or SOL spend.
