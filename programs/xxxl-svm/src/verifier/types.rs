#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierBoundaryStatus {
    ReadOnlyScaffoldOnly,
    FutureVerifierObligation,
}

impl VerifierBoundaryStatus {
    pub fn code(self) -> &'static str {
        match self {
            VerifierBoundaryStatus::ReadOnlyScaffoldOnly => "READ_ONLY_SCAFFOLD_ONLY",
            VerifierBoundaryStatus::FutureVerifierObligation => "FUTURE_VERIFIER_OBLIGATION",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeVerifierBoundaryComponent {
    RawPayloadDecoder,
    CanonicalPayloadValidation,
    SourceProofIdentityVerifier,
    GuardianApprovalAndQuorumVerifier,
    RouteBindingVerifier,
    TargetMintLegitimacyVerifier,
    ReplayVerifier,
    ExpirationVerifier,
    AmountControlVerifier,
    DeterministicErrorSurface,
}

impl RuntimeVerifierBoundaryComponent {
    pub fn code(self) -> &'static str {
        match self {
            RuntimeVerifierBoundaryComponent::RawPayloadDecoder => "RAW_PAYLOAD_DECODER",
            RuntimeVerifierBoundaryComponent::CanonicalPayloadValidation => {
                "CANONICAL_PAYLOAD_VALIDATION"
            }
            RuntimeVerifierBoundaryComponent::SourceProofIdentityVerifier => {
                "SOURCE_PROOF_IDENTITY_VERIFIER"
            }
            RuntimeVerifierBoundaryComponent::GuardianApprovalAndQuorumVerifier => {
                "GUARDIAN_APPROVAL_AND_QUORUM_VERIFIER"
            }
            RuntimeVerifierBoundaryComponent::RouteBindingVerifier => "ROUTE_BINDING_VERIFIER",
            RuntimeVerifierBoundaryComponent::TargetMintLegitimacyVerifier => {
                "TARGET_MINT_LEGITIMACY_VERIFIER"
            }
            RuntimeVerifierBoundaryComponent::ReplayVerifier => "REPLAY_VERIFIER",
            RuntimeVerifierBoundaryComponent::ExpirationVerifier => "EXPIRATION_VERIFIER",
            RuntimeVerifierBoundaryComponent::AmountControlVerifier => "AMOUNT_CONTROL_VERIFIER",
            RuntimeVerifierBoundaryComponent::DeterministicErrorSurface => {
                "DETERMINISTIC_ERROR_SURFACE"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FutureRuntimeParityCase {
    WrongFieldOrder,
    WrongByteEncoding,
    WrongCanonicalEventKeyPreimage,
    WrongSourceBurnTxHash,
    WrongSourceBurnEventIndex,
    AmountOverRouteCap,
    InvalidTargetMint,
}

impl FutureRuntimeParityCase {
    pub fn case_id(self) -> &'static str {
        match self {
            FutureRuntimeParityCase::WrongFieldOrder => "wrong-field-order",
            FutureRuntimeParityCase::WrongByteEncoding => "wrong-byte-encoding",
            FutureRuntimeParityCase::WrongCanonicalEventKeyPreimage => {
                "wrong-canonical-event-key-preimage"
            }
            FutureRuntimeParityCase::WrongSourceBurnTxHash => "wrong-source-burn-tx-hash",
            FutureRuntimeParityCase::WrongSourceBurnEventIndex => "wrong-source-burn-event-index",
            FutureRuntimeParityCase::AmountOverRouteCap => "amount-over-route-cap",
            FutureRuntimeParityCase::InvalidTargetMint => "invalid-target-mint",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FutureRuntimeParityCaseReport {
    pub case: FutureRuntimeParityCase,
    pub case_id: &'static str,
    pub status: VerifierBoundaryStatus,
    pub implemented: bool,
    pub required_component: RuntimeVerifierBoundaryComponent,
    pub reason: &'static str,
}
