#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerifierErrorCategory {
    RawPayloadDecode,
    CanonicalPayloadValidation,
    PayloadHashDomain,
    GuardianSignature,
    GuardianQuorum,
    RouteBinding,
    SourceProofIdentity,
    Replay,
    Expiration,
    AmountCap,
    TargetMintLegitimacy,
}

impl VerifierErrorCategory {
    pub fn code(self) -> &'static str {
        match self {
            VerifierErrorCategory::RawPayloadDecode => "RAW_PAYLOAD_DECODE_ERROR",
            VerifierErrorCategory::CanonicalPayloadValidation => {
                "CANONICAL_PAYLOAD_VALIDATION_ERROR"
            }
            VerifierErrorCategory::PayloadHashDomain => "PAYLOAD_HASH_DOMAIN_ERROR",
            VerifierErrorCategory::GuardianSignature => "GUARDIAN_SIGNATURE_ERROR",
            VerifierErrorCategory::GuardianQuorum => "GUARDIAN_QUORUM_ERROR",
            VerifierErrorCategory::RouteBinding => "ROUTE_BINDING_ERROR",
            VerifierErrorCategory::SourceProofIdentity => "SOURCE_PROOF_IDENTITY_ERROR",
            VerifierErrorCategory::Replay => "REPLAY_ERROR",
            VerifierErrorCategory::Expiration => "EXPIRATION_ERROR",
            VerifierErrorCategory::AmountCap => "AMOUNT_CAP_ERROR",
            VerifierErrorCategory::TargetMintLegitimacy => "TARGET_MINT_LEGITIMACY_ERROR",
        }
    }
}

pub const VERIFIER_ERROR_CATEGORIES: [VerifierErrorCategory; 11] = [
    VerifierErrorCategory::RawPayloadDecode,
    VerifierErrorCategory::CanonicalPayloadValidation,
    VerifierErrorCategory::PayloadHashDomain,
    VerifierErrorCategory::GuardianSignature,
    VerifierErrorCategory::GuardianQuorum,
    VerifierErrorCategory::RouteBinding,
    VerifierErrorCategory::SourceProofIdentity,
    VerifierErrorCategory::Replay,
    VerifierErrorCategory::Expiration,
    VerifierErrorCategory::AmountCap,
    VerifierErrorCategory::TargetMintLegitimacy,
];
