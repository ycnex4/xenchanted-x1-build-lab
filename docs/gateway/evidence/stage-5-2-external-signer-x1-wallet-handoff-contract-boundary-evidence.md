# Stage 5.2 Evidence — External Signer / X1 Wallet Handoff Contract Boundary

## Runtime reference

Runtime repository:

    ~/xenchanted-x1-lab/hello-x1

Runtime branch:

    stage-5-2-external-signer-x1-wallet-handoff-contract-boundary

Runtime commit:

    6a1df6e Add Stage 5.2 external signer X1 wallet handoff contract boundary

## Purpose

Stage 5.2 defines the offline contract between the runtime-generated handoff plan and an existing X1 wallet or external signer.

Stage 5.2 does not sign.

Stage 5.2 does not submit a transaction.

Stage 5.2 does not spend SOL.

Stage 5.2 does not load a local signer, access keypairs, access private keys, access seed phrases, access wallet files, output transaction serialization, use live RPC, or run simulation.

The stage exists to make the external signer / X1 wallet boundary explicit before any later unsigned payload export, wallet approval, signed payload intake, simulation, or live submit stage.

## Runtime files added

    tests/helpers/stage5ExternalSignerX1WalletHandoffContractPrototype.ts
    tests/stage5_external_signer_x1_wallet_handoff_contract_boundary.test.ts

## Artifact introduced

    stage5_external_signer_x1_wallet_handoff_contract_result

Artifact metadata:

    schemaVersion: 1
    stage: 5.2
    executionMode: external_signer_x1_wallet_handoff_contract_offline

Raw contract marker:

    stage5_external_signer_x1_wallet_handoff_contract

Raw verification requirements marker:

    stage5_external_signer_verification_requirements

Raw runtime limits marker:

    stage5_runtime_handoff_limits

## Source dependency

Stage 5.2 requires the Stage 5.1 opening artifact:

    stage5_explicit_live_send_readiness_opening_result

Required Stage 5.1 runtime commit:

    422d261

Stage 5.2 also binds the Stage 4.20 closure lineage:

    sourceStage4RuntimeCommit: 69f3c5b

## Handoff contract

Stage 5.2 defines this handoff contract:

    contractKind: stage5_external_signer_x1_wallet_handoff_contract
    handoffLayer: existing_x1_wallet_or_external_signer
    payloadFormat: x1_external_wallet_receipt_bound_message_plan
    runtimeRole: payload_preparation_only
    externalSignerRole: payer_signature_after_user_approval_later
    runtimeCustody: none
    custodyWalletProduct: out_of_scope
    walletPrivateKeyBoundary: must_remain_external_to_runtime
    operatorConfirmationRequiredLater: true
    userApprovalRequiredLater: true
    unsignedPayloadExportRequiredLater: true
    signedPayloadIntakeRequiredLater: true
    liveRpcSimulationRequiredLater: true
    liveSubmitRequiresSeparateStageLater: true
    transactionSubmissionStatus: not_allowed_in_stage5_2
    solSpendStatus: not_allowed_in_stage5_2

## External signer verification requirements

The external signer / X1 wallet layer must later verify:

    network
    program id
    payer public key
    recipient
    instruction name
    amount
    fee
    source Stage 4 closure digest
    source Stage 5 opening digest
    payload digest

The external signer must later display a human-readable summary and reject unexpected program id, amount, or recipient.

## Runtime handoff limits

Stage 5.2 runtime limits are:

    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionSerializationOutput: not_allowed_in_stage5_2
    liveRpc: not_used
    simulation: not_performed
    operatorConfirmationCollection: not_collected_in_stage5_2
    userWalletApproval: not_requested_in_stage5_2

## Digest binding

Stage 5.2 derives an external signer / X1 wallet handoff contract digest from:

    handoffContractKind: stage5_external_signer_x1_wallet_handoff_contract
    sourceStage5OpeningArtifact: stage5_explicit_live_send_readiness_opening_result
    sourceStage5OpeningStage: 5.1
    sourceStage5OpeningDigest
    sourceStage5RuntimeCommit: 422d261
    sourceStage4ClosureDigest
    sourceStage4RuntimeCommit: 69f3c5b
    handoffContractDigest
    verificationRequirementsDigest
    runtimeHandoffLimitsDigest

The handoff contract digest changes if the Stage 5.1 opening changes.

## Policy boundary

Stage 5.2 policy states:

    handoffContractOnly: true
    sourceStage5OpeningRequired: stage5_explicit_live_send_readiness_opening_result
    sourceStage5RuntimeCommitRequired: 422d261
    sourceStage4RuntimeCommitRequired: 69f3c5b
    externalSignerLayer: existing_x1_wallet_or_external_signer
    payloadFormat: x1_external_wallet_receipt_bound_message_plan
    custodyWalletProduct: out_of_scope
    runtimeCustody: none
    localSignerLoading: not_allowed
    keypairAccess: not_allowed
    privateKeyAccess: not_allowed
    seedPhraseAccess: not_allowed
    walletFileAccess: not_allowed
    runtimeSigning: not_performed
    runtimeTransactionSubmission: not_allowed
    runtimeSolSpendAllowed: false
    transactionSerializationOutput: not_allowed_in_stage5_2
    liveRpc: not_used
    simulation: not_performed
    operatorConfirmationCollection: not_collected_in_stage5_2
    userWalletApproval: not_requested_in_stage5_2
    liveSubmitRequiresSeparateStageLater: true

## Invariants

Stage 5.2 preserves these invariants:

    sourceStage5OpeningBound: true
    sourceStage4ClosureBound: true
    handoffContractBound: true
    verificationRequirementsBound: true
    runtimeHandoffLimitsBound: true
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
    noTransactionSerialization: true
    noLiveRpc: true
    noSimulation: true
    liveSendNotAuthorized: true
    liveSubmitRequiresSeparateStageLater: true

## Negative coverage

Stage 5.2 rejects:

- Malformed creation timestamps.
- Invalid Stage 5.1 opening artifacts.
- Wrong Stage 5.1 runtime commit references.
- Handoff contract digest mismatches.
- Forbidden secret-bearing values.
- Implicit live-send operations.

## Checks performed

Runtime checks passed:

    Stage 5.2 strict final marker check: passed
    Stage 5.2 test: 4 passing
    Stage 5.1 + Stage 5.2 smoke: 8 passing
    Stage 3.10 + Stage 4.1 through Stage 5.2 full smoke: 83 passing
    Prettier check: passed
    git diff --check: clean

Build-lab evidence checks:

    npm run typecheck
    npm test
    npm run build
    git diff --check

## Boundary decision

Stage 5.2 closes the external signer / X1 wallet handoff contract boundary.

Stage 5.2 does not authorize live transaction submission.

Stage 5.2 does not authorize SOL spend.

Stage 5.2 does not introduce runtime signing, local custody, local signer loading, keypair access, private-key access, seed phrase access, wallet file access, transaction serialization output, live RPC, or simulation.

The next valid stage is:

    Stage 5.3 — unsigned payload export package boundary

Stage 5.3 should define the offline unsigned payload package that can later be presented to an existing X1 wallet or external signer, while still not signing, not submitting, and not spending SOL.
