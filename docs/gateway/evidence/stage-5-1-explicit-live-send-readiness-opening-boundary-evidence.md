# Stage 5.1 Evidence — Explicit Live-Send Readiness Opening Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-1-explicit-live-send-readiness-opening-boundary

Runtime commit:

    422d261 Add Stage 5.1 explicit live-send readiness opening boundary

## Purpose

Stage 5.1 opens Stage 5 as an explicit live-send readiness planning stage.

Stage 5.1 does not authorize a live transaction.

Stage 5.1 does not submit a transaction, spend SOL, sign inside the runtime model, load a local signer, access keypairs, access private keys, output a serialized transaction, run live RPC, or run simulation.

Stage 5.1 exists to make the transition from Stage 4 to Stage 5 explicit:

- Stage 4 is closed as a no-send and no-SOL readiness chain.
- Stage 5 is opened only for planning.
- Live-send authorization remains false.
- Any actual live-send work must be opened by a later explicit stage.
- External signing remains outside the runtime model.
- The project does not build a custody wallet product.

## Runtime files added

    tests/helpers/stage5ExplicitLiveSendReadinessOpeningPrototype.ts
    tests/stage5_explicit_live_send_readiness_opening_boundary.test.ts

## Artifact introduced

    stage5_explicit_live_send_readiness_opening_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.1
    executionMode: explicit_live_send_readiness_opening_offline

Raw opening marker:

    stage5_explicit_live_send_readiness_opening

Raw opening scope marker:

    stage5_explicit_live_send_readiness_opening_scope

Raw operator gate marker:

    stage5_explicit_operator_confirmation_gate

Raw external signer checklist marker:

    stage5_external_signer_readiness_checklist

## Source dependency

Stage 5.1 requires the Stage 4.20 final no-send closure artifact:

    stage4_final_no_send_closure_result

Required Stage 4.20 runtime commit:

    69f3c5b

Required Stage 4.20 properties:

    stage: 4.20
    executionMode: stage4_final_no_send_closure_offline
    closedStageRange: 4.1-4.19
    stage4Closed: true
    noSendChainComplete: true
    noSolSpendChainComplete: true
    noLocalSignerLoaded: true
    noPrivateKeys: true
    noRuntimeSigning: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noSerializedTransaction: true
    liveSendNotAuthorized: true
    nextStageRequiresExplicitLiveSendOpening: true

## Opening scope

Stage 5.1 defines the live-send readiness opening scope:

    stage5Opened: true
    liveSendReadinessStatus: opened_for_planning_only
    liveSendAuthorizationStatus: not_authorized_by_stage5_1
    liveTransactionSubmissionStatus: not_allowed_in_stage5_1
    solSpendStatus: not_allowed_in_stage5_1
    externalSignerLayer: existing_x1_wallet_or_external_signer
    custodyWalletProduct: out_of_scope
    runtimeCustody: none
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    nextStageMustDefineConcreteLiveSendPreconditions: true

## Operator confirmation gate

Stage 5.1 defines a future operator confirmation gate.

Required future confirmation phrase:

    I_UNDERSTAND_STAGE_5_CAN_LEAD_TO_LIVE_TRANSACTION_LATER

Confirmation status in Stage 5.1:

    not_collected_in_stage5_1

The operator must later confirm:

    network
    program id
    payer
    recipient
    amount
    fee
    external wallet
    no private key paste

Stage 5.1 does not collect this confirmation yet.

## External signer readiness checklist

Stage 5.1 defines the external signer readiness checklist:

    externalWalletRequired: true
    x1WalletOrExternalSignerRequired: true
    walletPrivateKeyMustRemainExternal: true
    runtimeMustNotLoadWalletFile: true
    runtimeMustNotAskForSeed: true
    runtimeMustNotAskForPrivateKey: true
    unsignedPayloadExportRequiredLater: true
    userApprovalRequiredLater: true
    signedPayloadIntakeRequiredLater: true
    liveRpcSimulationRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true
    confirmationReceiptRequiredLater: true
    failureRecoveryPolicyRequiredLater: true

## Digest binding

Stage 5.1 derives a live-send readiness opening digest from:

    openingKind: stage5_explicit_live_send_readiness_opening
    sourceStage4ClosureArtifact: stage4_final_no_send_closure_result
    sourceStage4ClosureStage: 4.20
    sourceStage4ClosureDigest
    sourceStage4EvidenceChainDigest
    sourceStage4RuntimeCommit: 69f3c5b
    sourceStage4ClosedStageRange: 4.1-4.19
    stage5OpeningStage: 5.1
    openingScopeDigest
    operatorGateDigest
    externalSignerReadinessChecklistDigest

The opening digest changes if the Stage 4 closure evidence changes.

## Policy boundary

Stage 5.1 policy states:

    stage5OpeningOnly: true
    liveSendReadinessOpened: true
    liveSendAuthorizedByStage51: false
    implicitLiveSendForbidden: true
    sourceStage4ClosureRequired: stage4_final_no_send_closure_result
    sourceStage4RuntimeCommitRequired: 69f3c5b
    sourceStage4ClosureDigestRequired: true
    sourceStage4EvidenceChainDigestRequired: true
    operatorConfirmationRequiredBeforeLiveSend: true
    externalSignerLayer: existing_x1_wallet_or_external_signer
    custodyWalletProduct: out_of_scope
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    unsignedPayloadExportStageRequiredLater: true
    liveRpcSimulationStageRequiredLater: true
    liveTransactionSubmissionStageRequiredLater: true

## Invariants

Stage 5.1 preserves these invariants:

    stage5Opened: true
    openingOnly: true
    sourceStage4ClosureBound: true
    sourceStage4NoSendClosureBound: true
    sourceStage4RuntimeCommitBound: true
    liveSendNotAuthorized: true
    noImplicitLiveSend: true
    noLocalCustody: true
    noLocalSignerLoaded: true
    noPrivateKeys: true
    noRuntimeSigning: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noSerializedTransaction: true
    noLiveRpc: true
    noSimulation: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    nextStageMustDefineConcreteLiveSendPreconditions: true

## Negative coverage

Stage 5.1 rejects:

- Malformed opening timestamps.
- Invalid Stage 4 closure artifacts.
- Wrong Stage 4 runtime commit references.
- Opening digest mismatches.
- Forbidden secret-bearing values.
- Implicit live-send operations.

## Checks performed

Runtime checks passed:

    Stage 5.1 strict final marker check: passed
    Stage 5.1 test: 4 passing
    Stage 4.20 + Stage 5.1 smoke: 8 passing
    Stage 4.13 timeout-sensitive crypto test with larger timeout: 4 passing
    Stage 3.10 + Stage 4.1 through Stage 5.1 full smoke with larger timeout: 79 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5 is now opened only as an explicit live-send readiness planning phase.

Stage 5.1 does not authorize live transaction submission.

Stage 5.1 does not authorize SOL spend.

Stage 5.1 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, serialized transaction output, live RPC, or simulation.

The next valid stage is:

    Stage 5.2 — external signer / X1 wallet handoff contract boundary

Stage 5.2 should define the exact handoff contract between the runtime-generated payload and the existing X1 wallet or external signer, while preserving the rule that private signing material never enters the runtime.
