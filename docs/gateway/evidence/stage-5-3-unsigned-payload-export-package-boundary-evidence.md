# Stage 5.3 Evidence — Unsigned Payload Export Package Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-3-unsigned-payload-export-package-boundary

Runtime commit:

    00a71a1 Add Stage 5.3 unsigned payload export package boundary

## Purpose

Stage 5.3 defines an offline unsigned payload export package for later review by an existing X1 wallet or external signer.

Stage 5.3 does not sign.

Stage 5.3 does not submit a transaction.

Stage 5.3 does not spend SOL.

Stage 5.3 does not create a transaction object, produce transaction serialization, load a local signer, access keypairs, access private keys, access seed phrases, access wallet files, use live RPC, run simulation, or request wallet approval.

The stage exists to define the package format and digest binding before any later wallet review, approval, signed payload intake, simulation, or live submit stage.

## Runtime files added

    tests/helpers/stage5UnsignedPayloadExportPackagePrototype.ts
    tests/stage5_unsigned_payload_export_package_boundary.test.ts

## Artifact introduced

    stage5_unsigned_payload_export_package_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.3
    executionMode: unsigned_payload_export_package_offline

Raw package marker:

    stage5_unsigned_payload_export_package

Raw package format marker:

    x1_external_wallet_unsigned_payload_package

Raw signer summary marker:

    stage5_human_readable_signer_summary

Raw verification checklist marker:

    stage5_unsigned_payload_verification_checklist

## Source dependency

Stage 5.3 requires the Stage 5.2 handoff contract artifact:

    stage5_external_signer_x1_wallet_handoff_contract_result

Required Stage 5.2 runtime commit:

    6a1df6e

Stage 5.3 also binds the prior runtime lineage:

    sourceStage5OpeningRuntimeCommit: 422d261
    sourceStage4RuntimeCommit: 69f3c5b

## Unsigned payload package

Stage 5.3 defines this unsigned package:

    packageKind: stage5_unsigned_payload_export_package
    packageFormat: x1_external_wallet_unsigned_payload_package
    packageVersion: 1
    handoffLayer: existing_x1_wallet_or_external_signer
    payloadPurpose: receipt_bound_gateway_mint_for_external_wallet_review_later
    packageStatus: format_defined_offline_only
    payloadExportStatus: not_exported_to_wallet_in_stage5_3
    transactionObjectStatus: not_created_in_stage5_3
    transactionSerializationStatus: not_produced_in_stage5_3
    signatureStatus: not_signed_in_stage5_3
    liveRpcStatus: not_used_in_stage5_3
    simulationStatus: not_performed_in_stage5_3
    transactionSubmissionStatus: not_allowed_in_stage5_3
    solSpendStatus: not_allowed_in_stage5_3
    userWalletApprovalStatus: not_requested_in_stage5_3

## Human-readable signer summary

Stage 5.3 defines a human-readable signer summary requirement.

The summary must later display:

    network
    program id
    payer public key
    recipient
    instruction name
    amount
    fee
    source Stage 4 closure digest
    source Stage 5 opening digest
    source Stage 5 handoff digest
    package digest

Stage 5.3 only defines the requirement:

    signerSummaryStatus: defined_not_displayed_in_stage5_3

## Payload verification checklist

Stage 5.3 defines this payload verification checklist:

    externalSignerMustVerifyNetworkLater: true
    externalSignerMustVerifyProgramIdLater: true
    externalSignerMustVerifyPayerPublicKeyLater: true
    externalSignerMustVerifyRecipientLater: true
    externalSignerMustVerifyInstructionNameLater: true
    externalSignerMustVerifyAmountLater: true
    externalSignerMustVerifyFeeLater: true
    externalSignerMustVerifyStage4ClosureDigestLater: true
    externalSignerMustVerifyStage5OpeningDigestLater: true
    externalSignerMustVerifyStage5HandoffDigestLater: true
    externalSignerMustVerifyPackageDigestLater: true
    runtimeMustNotMutatePackageAfterDigest: true
    walletApprovalRequiredLater: true
    signedPayloadIntakeRequiredLater: true
    liveRpcSimulationRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true

## Digest binding

Stage 5.3 derives an unsigned payload export package digest from:

    exportPackageKind: stage5_unsigned_payload_export_package
    sourceStage5HandoffArtifact: stage5_external_signer_x1_wallet_handoff_contract_result
    sourceStage5HandoffStage: 5.2
    sourceStage5HandoffDigest
    sourceStage5RuntimeCommit: 6a1df6e
    sourceStage5OpeningDigest
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    unsignedPayloadPackageDigest
    humanReadableSignerSummaryDigest
    payloadVerificationChecklistDigest

The unsigned payload export package digest changes if the Stage 5.2 handoff changes.

## Policy boundary

Stage 5.3 policy states:

    unsignedPayloadExportPackageOnly: true
    sourceStage5HandoffRequired: stage5_external_signer_x1_wallet_handoff_contract_result
    sourceStage5RuntimeCommitRequired: 6a1df6e
    sourceStage4RuntimeCommitRequired: 69f3c5b
    handoffLayer: existing_x1_wallet_or_external_signer
    packageFormat: x1_external_wallet_unsigned_payload_package
    runtimeCustody: none
    custodyWalletProduct: out_of_scope
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionObjectCreation: not_performed
    transactionSerialization: not_performed
    liveRpc: not_used
    simulation: not_performed
    userWalletApproval: not_requested_in_stage5_3
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.3 preserves these invariants:

    sourceStage5HandoffBound: true
    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    unsignedPayloadPackageBound: true
    humanReadableSignerSummaryBound: true
    payloadVerificationChecklistBound: true
    externalSignerIntegrationOnly: true
    custodyWalletOutOfScope: true
    noRuntimeCustody: true
    noLocalSignerLoaded: true
    noKeypairAccess: true
    noPrivateKeys: true
    noSeedPhraseAccess: true
    noWalletFileAccess: true
    noRuntimeSigning: true
    noRuntimeSubmission: true
    noRuntimeSolSpend: true
    noTransactionObjectCreated: true
    noTransactionSerialization: true
    noLiveRpc: true
    noSimulation: true
    liveSendNotAuthorized: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.3 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.2 handoff contract artifacts.
- Wrong Stage 5.2 runtime commit references.
- Unsigned payload package digest mismatches.
- Forbidden secret-bearing values.
- Implicit live-send operations.

## Checks performed

Runtime checks passed:

    Stage 5.3 strict final marker check: passed
    Stage 5.3 test: 4 passing
    Stage 5.2 + Stage 5.3 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 5.3 full smoke: 87 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.3 closes the unsigned payload export package boundary.

Stage 5.3 does not authorize live transaction submission.

Stage 5.3 does not authorize SOL spend.

Stage 5.3 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction object creation, transaction serialization, live RPC, simulation, wallet approval, or transaction submission.

The next valid stage is:

    Stage 5.4 — external wallet user-approval preflight boundary

Stage 5.4 should define the wallet-facing review and user-approval preflight boundary, while still separating review/approval from live submission and preserving the rule that private signing material never enters the runtime.
