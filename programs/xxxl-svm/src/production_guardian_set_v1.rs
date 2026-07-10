//! Production guardian set v1 public record and source binding.
//!
//! This module binds public guardian keys only. It contains no private keys,
//! no keypair JSON, no signing package, no RPC mutation, and no activation.

use crate::verifier::{
    verify_guardian_quorum_structural, GuardianApprovalRef, GuardianPublicKey,
    GuardianQuorumStructuralError, GuardianQuorumStructuralErrorKind,
    GuardianQuorumStructuralResult, GuardianSetRef,
};

pub const PRODUCTION_GUARDIAN_SET_V1_STATUS: &str =
    "SOURCE_CONFIG_BOUND_PUBLIC_KEYS_ONLY_NO_ACTIVATION";
pub const PRODUCTION_GUARDIAN_SET_V1_VERSION: u64 = 1;
pub const PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_COUNT: usize = 5;
pub const PRODUCTION_GUARDIAN_SET_V1_THRESHOLD: u8 = 3;
pub const PRODUCTION_GUARDIAN_SET_V1_QUORUM_MODEL: &str = "3-of-5";
pub const PRODUCTION_GUARDIAN_SET_V1_KEY_TYPE: &str = "Ed25519/Solana public keys";
pub const PRODUCTION_GUARDIAN_SET_V1_ORDERING_RULE: &str =
    "explicit_descriptor_order_guardian_01_to_guardian_05";
pub const PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN: &str =
    "xxxl:x1-testnet:gateway-mint:v1:guardian-set-v1";
pub const PRODUCTION_GUARDIAN_SET_V1_EFFECTIVE_PACKAGE: &str =
    "production-guardian-set-v1-public-record-and-source-change";
pub const PRODUCTION_GUARDIAN_SET_V1_DESCRIPTOR_HASH_SHA256: &str =
    "4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83";
pub const PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID: [u8; 32] =
    [0x40, 0x88, 0xa1, 0xf7, 0x18, 0x70, 0xe6, 0x17, 0xf3, 0x63, 0x5d, 0x1c, 0x29, 0xae, 0xdd, 0x9f, 0xc5, 0x3a, 0x0c, 0x13, 0x6c, 0x6f, 0x69, 0xe0, 0xcb, 0x34, 0x3d, 0x21, 0x7a, 0xb1, 0xcd, 0x83];

pub const PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS_BASE58: [&str; 5] = [
    "7TqrbZeX5t3eVNMEYMpp4MuYRd1RF4Hj7raHRouCoNLf",
    "GN995izQ4ktAd2RCrv7Np8Hes7xK7hBu5trC7UWQNwQp",
    "6BjFvTkng4ViT8Kpywh726YqUfVTVqPQhq6pEYQWy5ih",
    "9xP4DDWP5B9RErNCSSTnLH8ej85drMabWc8y3MfYJwrY",
    "UB3nJ3qN8rNqe5F6MKyxG3kQBadfVwHGzYAXZJaZsjg",
];

pub const PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS: [GuardianPublicKey; 5] = [
    GuardianPublicKey([0x60, 0x06, 0x76, 0x96, 0x7f, 0x93, 0x4a, 0xef, 0x83, 0xcb, 0x57, 0x88, 0x70, 0x8a, 0xc1, 0x28, 0x0f, 0x3c, 0xd7, 0x4d, 0xc8, 0x82, 0x36, 0xa2, 0x7c, 0xf0, 0x04, 0x87, 0x7d, 0x7e, 0xb9, 0x48]),
    GuardianPublicKey([0xe4, 0x49, 0xb4, 0x3d, 0x62, 0x05, 0x7e, 0x52, 0x7c, 0x3a, 0x29, 0x4d, 0x99, 0xd1, 0xad, 0x3d, 0x88, 0xda, 0x45, 0xeb, 0x01, 0xea, 0x30, 0x0a, 0x0c, 0x07, 0xd7, 0x96, 0x0e, 0x93, 0x08, 0x95]),
    GuardianPublicKey([0x4d, 0x0a, 0x05, 0xee, 0x94, 0xd3, 0xc9, 0x60, 0x6e, 0xad, 0x16, 0xc2, 0xbd, 0x76, 0x25, 0x5b, 0x98, 0xeb, 0xdb, 0x2b, 0x45, 0x1d, 0xf6, 0x9e, 0xf0, 0x45, 0x28, 0xaa, 0x58, 0xa4, 0xce, 0x32]),
    GuardianPublicKey([0x85, 0x0d, 0x60, 0xc3, 0x49, 0x20, 0xf2, 0x8f, 0x68, 0x12, 0x43, 0xa0, 0x16, 0x6d, 0x81, 0xfd, 0x13, 0x69, 0x9b, 0x96, 0x58, 0x9d, 0x93, 0xa9, 0x26, 0x71, 0xfa, 0x2b, 0x7a, 0xc3, 0x6c, 0x29]),
    GuardianPublicKey([0x06, 0xf6, 0x0b, 0x08, 0x46, 0x1c, 0x3c, 0xe5, 0x64, 0x90, 0xc9, 0xbb, 0x54, 0x21, 0x0d, 0x83, 0x44, 0x15, 0x0a, 0x80, 0x49, 0x97, 0x4c, 0x94, 0x09, 0xa1, 0x78, 0xc7, 0x17, 0x05, 0x8c, 0x63]),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionGuardianSetV1Report {
    pub status: &'static str,
    pub version: u64,
    pub guardian_count: usize,
    pub threshold: u8,
    pub quorum_model: &'static str,
    pub key_type: &'static str,
    pub signature_domain: &'static str,
    pub descriptor_hash_sha256: &'static str,
    pub public_keys_only: bool,
    pub source_config_bound: bool,
    pub private_key_material_present: bool,
    pub signing_package_constructed: bool,
    pub activation_authorized: bool,
    pub route_enablement_authorized: bool,
    pub spl_cpi_enablement_authorized: bool,
    pub proof_log_instantiation_authorized: bool,
}

pub const PRODUCTION_GUARDIAN_SET_V1_REPORT: ProductionGuardianSetV1Report =
    ProductionGuardianSetV1Report {
        status: PRODUCTION_GUARDIAN_SET_V1_STATUS,
        version: PRODUCTION_GUARDIAN_SET_V1_VERSION,
        guardian_count: PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_COUNT,
        threshold: PRODUCTION_GUARDIAN_SET_V1_THRESHOLD,
        quorum_model: PRODUCTION_GUARDIAN_SET_V1_QUORUM_MODEL,
        key_type: PRODUCTION_GUARDIAN_SET_V1_KEY_TYPE,
        signature_domain: PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN,
        descriptor_hash_sha256: PRODUCTION_GUARDIAN_SET_V1_DESCRIPTOR_HASH_SHA256,
        public_keys_only: true,
        source_config_bound: true,
        private_key_material_present: false,
        signing_package_constructed: false,
        activation_authorized: false,
        route_enablement_authorized: false,
        spl_cpi_enablement_authorized: false,
        proof_log_instantiation_authorized: false,
    };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProductionGuardianSetV1Approval<'a> {
    pub guardian_set_id: &'a [u8; 32],
    pub guardian_public_key: GuardianPublicKey,
    pub signature_domain: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProductionGuardianSetV1ErrorKind {
    WrongSignatureDomain,
    GuardianQuorumStructural(GuardianQuorumStructuralErrorKind),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductionGuardianSetV1Error {
    pub kind: ProductionGuardianSetV1ErrorKind,
    pub approval_index: Option<usize>,
    pub structural_error: Option<GuardianQuorumStructuralError>,
}

pub fn production_guardian_set_v1_report() -> &'static ProductionGuardianSetV1Report {
    &PRODUCTION_GUARDIAN_SET_V1_REPORT
}

pub fn production_guardian_set_v1_ref() -> GuardianSetRef<'static> {
    GuardianSetRef {
        guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
        threshold: PRODUCTION_GUARDIAN_SET_V1_THRESHOLD,
        guardians: &PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS,
    }
}

pub fn verify_production_guardian_set_v1_approvals(
    approvals: &[ProductionGuardianSetV1Approval<'_>],
) -> Result<GuardianQuorumStructuralResult<'static>, ProductionGuardianSetV1Error> {
    for (approval_index, approval) in approvals.iter().enumerate() {
        if approval.signature_domain != PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN {
            return Err(ProductionGuardianSetV1Error {
                kind: ProductionGuardianSetV1ErrorKind::WrongSignatureDomain,
                approval_index: Some(approval_index),
                structural_error: None,
            });
        }

        if approval.guardian_set_id != &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID {
            return Err(ProductionGuardianSetV1Error {
                kind: ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(
                    GuardianQuorumStructuralErrorKind::GuardianSetIdMismatch,
                ),
                approval_index: Some(approval_index),
                structural_error: None,
            });
        }
    }

    let structural_approvals: Vec<GuardianApprovalRef<'static>> = approvals
        .iter()
        .map(|approval| GuardianApprovalRef {
            guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
            guardian_public_key: approval.guardian_public_key,
        })
        .collect();

    verify_guardian_quorum_structural(production_guardian_set_v1_ref(), &structural_approvals)
        .map_err(|structural_error| ProductionGuardianSetV1Error {
            kind: ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(structural_error.kind),
            approval_index: structural_error.approval_index,
            structural_error: Some(structural_error),
        })
}

pub fn production_guardian_set_v1_source_binding_complete() -> bool {
    PRODUCTION_GUARDIAN_SET_V1_REPORT.public_keys_only
        && PRODUCTION_GUARDIAN_SET_V1_REPORT.source_config_bound
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.private_key_material_present
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.signing_package_constructed
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.activation_authorized
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.route_enablement_authorized
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.spl_cpi_enablement_authorized
        && !PRODUCTION_GUARDIAN_SET_V1_REPORT.proof_log_instantiation_authorized
        && PRODUCTION_GUARDIAN_SET_V1_VERSION == 1
        && PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_COUNT == 5
        && PRODUCTION_GUARDIAN_SET_V1_THRESHOLD == 3
        && PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS.len() == PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_COUNT
}

#[cfg(test)]
mod tests {
    use super::*;

    const WRONG_GUARDIAN_SET_ID: [u8; 32] = [0x44; 32];

    fn approval(index: usize) -> ProductionGuardianSetV1Approval<'static> {
        ProductionGuardianSetV1Approval {
            guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
            guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[index],
            signature_domain: PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN,
        }
    }

    #[test]
    fn public_record_constants_match_approved_3_of_5_model() {
        let report = production_guardian_set_v1_report();

        assert_eq!(report.status, "SOURCE_CONFIG_BOUND_PUBLIC_KEYS_ONLY_NO_ACTIVATION");
        assert_eq!(report.version, 1);
        assert_eq!(report.guardian_count, 5);
        assert_eq!(report.threshold, 3);
        assert_eq!(report.quorum_model, "3-of-5");
        assert_eq!(report.key_type, "Ed25519/Solana public keys");
        assert_eq!(PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS_BASE58.len(), 5);
        assert_eq!(PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS.len(), 5);
        assert_eq!(
            report.descriptor_hash_sha256,
            "4088a1f71870e617f3635d1c29aedd9fc53a0c136c6f69e0cb343d217ab1cd83"
        );
        assert!(report.public_keys_only);
        assert!(report.source_config_bound);
        assert!(!report.private_key_material_present);
        assert!(!report.signing_package_constructed);
    }

    #[test]
    fn source_binding_complete_and_execution_surfaces_remain_blocked() {
        let report = production_guardian_set_v1_report();

        assert!(production_guardian_set_v1_source_binding_complete());
        assert!(!report.activation_authorized);
        assert!(!report.route_enablement_authorized);
        assert!(!report.spl_cpi_enablement_authorized);
        assert!(!report.proof_log_instantiation_authorized);
    }

    #[test]
    fn valid_3_of_5_quorum_is_accepted() {
        let approvals = [approval(0), approval(2), approval(4)];
        let result = verify_production_guardian_set_v1_approvals(&approvals)
            .expect("valid production guardian set v1 quorum");

        assert_eq!(result.guardian_set_id, &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID);
        assert_eq!(result.threshold, 3);
        assert_eq!(result.guardian_count, 5);
        assert_eq!(result.unique_known_approval_count, 3);
        assert!(result.quorum_reached);
        assert!(!result.ed25519_signature_verification_performed);
        assert!(!result.cryptographic_signature_proof_accepted);
    }

    #[test]
    fn insufficient_2_of_5_quorum_is_rejected() {
        let approvals = [approval(0), approval(1)];
        let err = verify_production_guardian_set_v1_approvals(&approvals)
            .expect_err("2-of-5 must not satisfy 3-of-5 quorum");

        assert_eq!(
            err.kind,
            ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(
                GuardianQuorumStructuralErrorKind::QuorumNotReached
            )
        );
    }

    #[test]
    fn duplicate_signer_is_rejected() {
        let approvals = [approval(0), approval(0), approval(1)];
        let err = verify_production_guardian_set_v1_approvals(&approvals)
            .expect_err("duplicate signer must be rejected");

        assert_eq!(
            err.kind,
            ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(
                GuardianQuorumStructuralErrorKind::DuplicateGuardianApproval
            )
        );
        assert_eq!(err.approval_index, Some(1));
    }

    #[test]
    fn unknown_signer_is_rejected() {
        let approvals = [
            approval(0),
            approval(1),
            ProductionGuardianSetV1Approval {
                guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
                guardian_public_key: GuardianPublicKey([0x99; 32]),
                signature_domain: PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN,
            },
        ];

        let err = verify_production_guardian_set_v1_approvals(&approvals)
            .expect_err("unknown signer must be rejected");

        assert_eq!(
            err.kind,
            ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(
                GuardianQuorumStructuralErrorKind::UnknownGuardian
            )
        );
    }

    #[test]
    fn wrong_signature_domain_is_rejected() {
        let approvals = [
            approval(0),
            ProductionGuardianSetV1Approval {
                guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
                guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[1],
                signature_domain: "xxxl:wrong-domain",
            },
            approval(2),
        ];

        let err = verify_production_guardian_set_v1_approvals(&approvals)
            .expect_err("wrong signature domain must be rejected");

        assert_eq!(err.kind, ProductionGuardianSetV1ErrorKind::WrongSignatureDomain);
        assert_eq!(err.approval_index, Some(1));
        assert!(err.structural_error.is_none());
    }

    #[test]
    fn wrong_guardian_set_id_is_rejected() {
        let approvals = [
            approval(0),
            ProductionGuardianSetV1Approval {
                guardian_set_id: &WRONG_GUARDIAN_SET_ID,
                guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[1],
                signature_domain: PRODUCTION_GUARDIAN_SET_V1_SIGNATURE_DOMAIN,
            },
            approval(2),
        ];

        let err = verify_production_guardian_set_v1_approvals(&approvals)
            .expect_err("wrong guardian set id must be rejected");

        assert_eq!(
            err.kind,
            ProductionGuardianSetV1ErrorKind::GuardianQuorumStructural(
                GuardianQuorumStructuralErrorKind::GuardianSetIdMismatch
            )
        );
        assert_eq!(err.approval_index, Some(1));
    }

    #[test]
    fn duplicate_guardian_list_is_rejected_by_structural_verifier() {
        let guardians = [
            PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[0],
            PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[0],
            PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[1],
        ];
        let guardian_set = GuardianSetRef {
            guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
            threshold: 2,
            guardians: &guardians,
        };
        let approvals = [
            GuardianApprovalRef {
                guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
                guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[0],
            },
            GuardianApprovalRef {
                guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
                guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[1],
            },
        ];

        let err = verify_guardian_quorum_structural(guardian_set, &approvals)
            .expect_err("duplicate guardian list must be rejected");

        assert_eq!(err.kind, GuardianQuorumStructuralErrorKind::DuplicateGuardianPublicKey);
    }

    #[test]
    fn invalid_threshold_is_rejected_by_structural_verifier() {
        let guardian_set = GuardianSetRef {
            guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
            threshold: 6,
            guardians: &PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS,
        };
        let approvals = [GuardianApprovalRef {
            guardian_set_id: &PRODUCTION_GUARDIAN_SET_V1_GUARDIAN_SET_ID,
            guardian_public_key: PRODUCTION_GUARDIAN_SET_V1_PUBLIC_KEYS[0],
        }];

        let err = verify_guardian_quorum_structural(guardian_set, &approvals)
            .expect_err("threshold greater than guardian count must be rejected");

        assert_eq!(err.kind, GuardianQuorumStructuralErrorKind::ThresholdExceedsGuardianSet);
    }
}
