# Stage 5.6 Evidence — Signed Payload Intake Quarantine Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-6-signed-payload-intake-quarantine-boundary

Runtime commit:

    f34cba3 Add Stage 5.6 signed payload intake quarantine boundary

## Purpose

Stage 5.6 defines a quarantine boundary for an externally signed payload reference on the approved path only.

This stage accepts only a digest reference and byte length for an externally signed payload.

Stage 5.6 does not store raw payload bytes.

Stage 5.6 does not store raw signatures.

Stage 5.6 does not import wallet signature material into runtime.

Stage 5.6 does not create a wallet signature.

Stage 5.6 does not submit a transaction.

Stage 5.6 does not spend SOL.

Stage 5.6 does not create a transaction object, produce transaction serialization, load a local signer, access keypairs, access private keys, access seed phrases, access wallet files, use live RPC, or run simulation.

The rejected decision path is blocked before signed payload quarantine.

## Runtime files added

    tests/helpers/stage5SignedPayloadIntakeQuarantinePrototype.ts
    tests/stage5_signed_payload_intake_quarantine_boundary.test.ts

## Artifact introduced

    stage5_signed_payload_intake_quarantine_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.6
    executionMode: signed_payload_intake_quarantine_offline

Raw quarantine marker:

    stage5_signed_payload_intake_quarantine

Raw external signed payload reference marker:

    stage5_external_wallet_signed_payload_reference

Raw quarantine policy marker:

    stage5_signed_payload_quarantine_policy

Raw quarantine release gate marker:

    stage5_signed_payload_quarantine_release_gate

## Source dependency

Stage 5.6 requires the Stage 5.5 external wallet approval decision receipt artifact:

    stage5_external_wallet_approval_decision_receipt_result

Required Stage 5.5 runtime commit:

    db6c1b6

Stage 5.6 also binds the prior runtime lineage:

    sourceStage5ApprovalPreflightRuntimeCommit: 165deb7
    sourceStage4RuntimeCommit: 69f3c5b

## Approved path only

Stage 5.6 accepts only:

    external_wallet_user_approved

Stage 5.6 rejects:

    external_wallet_user_rejected

Rejected decision error marker:

    rejected_decision_not_allowed

## External signed payload reference

Stage 5.6 defines this signed payload reference:

    referenceKind: stage5_external_wallet_signed_payload_reference
    walletLayer: existing_x1_wallet_or_external_signer
    decision: external_wallet_user_approved
    payloadReferenceMode: digest_reference_only
    payloadBytesStatus: not_stored_in_runtime_stage5_6
    rawSignatureStatus: not_stored_in_runtime_stage5_6
    signatureMaterialStatus: external_to_runtime_stage5_6
    walletSignatureStatus: externally_collected_not_imported_stage5_6
    signedPayloadStatus: reference_quarantined_stage5_6
    transactionSubmissionStatus: not_allowed_in_stage5_6
    solSpendStatus: not_allowed_in_stage5_6
    liveRpcStatus: not_used_in_stage5_6
    simulationStatus: not_performed_in_stage5_6
    quarantineStatus: quarantined_not_released_stage5_6
    releaseToLiveSubmitStatus: not_allowed_in_stage5_6
    liveSubmitRequiresSeparateStageLater: true

The reference includes:

    externallySignedPayloadDigest
    externallySignedPayloadByteLength

The runtime stores only the digest reference and byte length.

## Signed payload quarantine policy

Stage 5.6 defines this policy:

    policyKind: stage5_signed_payload_quarantine_policy
    approvedDecisionRequired: external_wallet_user_approved
    rejectedDecisionBlocked: external_wallet_user_rejected
    payloadReferenceMode: digest_reference_only
    rawPayloadBytesStorage: not_allowed
    rawSignatureStorage: not_allowed
    signatureMaterialStorage: not_allowed
    runtimeCannotSignForUser: true
    runtimeCannotCreateWalletSignature: true
    runtimeCannotSubmitQuarantinedPayloadInStage56: true
    runtimeCannotSpendSolInStage56: true
    quarantineReleaseRequiresSeparateStageLater: true
    liveRpcSimulationRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true

## Quarantine release gate

Stage 5.6 defines this release gate:

    gateKind: stage5_signed_payload_quarantine_release_gate
    approvedDecisionRequired: true
    signedPayloadReferenceDigestRequired: true
    rawPayloadBytesMustRemainExternal: true
    rawSignatureMustRemainExternal: true
    mayProceedToQuarantineValidationLater: true
    maySubmitLiveTransactionInStage56: false
    maySpendSolInStage56: false
    quarantineReleaseRequiresSeparateStageLater: true
    liveRpcSimulationStageRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true
    gateStatus: defined_offline_only

## Digest binding

Stage 5.6 derives a signed payload intake quarantine digest from:

    signedPayloadIntakeQuarantineKind: stage5_signed_payload_intake_quarantine
    sourceStage5ApprovalDecisionReceiptArtifact: stage5_external_wallet_approval_decision_receipt_result
    sourceStage5ApprovalDecisionReceiptStage: 5.5
    sourceStage5ApprovalDecisionReceiptDigest
    sourceStage5RuntimeCommit: db6c1b6
    sourceStage5ApprovalPreflightDigest
    sourceStage5ExportPackageDigest
    sourceStage5HandoffDigest
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    externallySignedPayloadDigest
    externallySignedPayloadByteLength
    signedPayloadReferenceDigest
    quarantinePolicyDigest
    quarantineReleaseGateDigest

The quarantine digest changes if the Stage 5.5 receipt changes.

The quarantine digest also changes if the external signed payload digest or byte length changes.

## Policy boundary

Stage 5.6 policy states:

    signedPayloadIntakeQuarantineOnly: true
    sourceStage5ApprovalDecisionReceiptRequired: stage5_external_wallet_approval_decision_receipt_result
    sourceStage5RuntimeCommitRequired: db6c1b6
    sourceStage4RuntimeCommitRequired: 69f3c5b
    approvedPathOnly: true
    rejectedPathBlocked: true
    walletLayer: existing_x1_wallet_or_external_signer
    payloadReferenceMode: digest_reference_only
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
    signedPayloadReferenceStatus: quarantined_not_released_stage5_6
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionObjectCreation: not_performed
    transactionSerialization: not_performed
    liveRpc: not_used
    simulation: not_performed
    quarantineReleaseRequiresSeparateStageLater: true
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.6 preserves these invariants:

    sourceStage5ApprovalDecisionReceiptBound: true
    sourceStage5ApprovalPreflightBound: true
    sourceStage5ExportPackageBound: true
    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    signedPayloadReferenceBound: true
    quarantinePolicyBound: true
    quarantineReleaseGateBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    approvedPathOnly: true
    rejectedPathBlocked: true
    payloadReferenceOnly: true
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
    noLiveRpc: true
    noSimulation: true
    liveSendNotAuthorized: true
    quarantineNotReleased: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.6 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.5 approval decision receipt artifacts.
- Wrong Stage 5.5 runtime commit references.
- Rejected decision receipts.
- Invalid external signed payload digest references.
- Invalid external signed payload byte lengths.
- Signed payload intake quarantine digest mismatches.
- Forbidden secret-bearing values.
- Implicit transaction-submission or signing operations.

## Checks performed

Runtime checks passed:

    Stage 5.6 strict final marker check: passed
    Stage 5.6 test: 5 passing
    Stage 5.5 + Stage 5.6 smoke: 10 passing
    Stage 3.10 + Stage 4.1 through Stage 5.6 full smoke: 101 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.6 closes the signed payload intake quarantine boundary.

Stage 5.6 does not authorize live transaction submission.

Stage 5.6 does not authorize SOL spend.

Stage 5.6 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, live RPC, simulation, raw payload byte storage, raw signature storage, signature material storage, quarantine release, or transaction submission.

The next valid stage is:

    Stage 5.7 — signed payload quarantine validation boundary

Stage 5.7 should define validation of the quarantined signed payload reference and its source bindings, while still keeping raw signing material outside runtime and still not authorizing live submission or SOL spend.
