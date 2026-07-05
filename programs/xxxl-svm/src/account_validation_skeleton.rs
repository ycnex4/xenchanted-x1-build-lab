use crate::{
    account_order_skeleton::{
        account_requirements_for_tag, XxxlAccountOwnerExpectationSkeleton,
    },
    state_instruction_skeleton::XxxlGatewayInstructionTag,
};

pub const XXXL_ACCOUNT_VALIDATION_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlAccountMetaSkeleton {
    pub is_signer: bool,
    pub is_writable: bool,
    pub owner_expectation: XxxlAccountOwnerExpectationSkeleton,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlAccountValidationSkeletonError {
    InvalidAccountCount {
        expected: usize,
        actual: usize,
    },
    SignerMismatch {
        index: usize,
        expected: bool,
        actual: bool,
    },
    WritableMismatch {
        index: usize,
        expected: bool,
        actual: bool,
    },
    OwnerExpectationMismatch {
        index: usize,
        expected: XxxlAccountOwnerExpectationSkeleton,
        actual: XxxlAccountOwnerExpectationSkeleton,
    },
}

pub fn validate_account_metas_for_tag(
    tag: XxxlGatewayInstructionTag,
    actual_accounts: &[XxxlAccountMetaSkeleton],
) -> Result<(), XxxlAccountValidationSkeletonError> {
    let expected_accounts = account_requirements_for_tag(tag);

    if actual_accounts.len() != expected_accounts.len() {
        return Err(XxxlAccountValidationSkeletonError::InvalidAccountCount {
            expected: expected_accounts.len(),
            actual: actual_accounts.len(),
        });
    }

    for (expected, actual) in expected_accounts.iter().zip(actual_accounts.iter()) {
        if actual.is_signer != expected.is_signer {
            return Err(XxxlAccountValidationSkeletonError::SignerMismatch {
                index: expected.index,
                expected: expected.is_signer,
                actual: actual.is_signer,
            });
        }

        if actual.is_writable != expected.is_writable {
            return Err(XxxlAccountValidationSkeletonError::WritableMismatch {
                index: expected.index,
                expected: expected.is_writable,
                actual: actual.is_writable,
            });
        }

        if actual.owner_expectation != expected.owner_expectation {
            return Err(XxxlAccountValidationSkeletonError::OwnerExpectationMismatch {
                index: expected.index,
                expected: expected.owner_expectation,
                actual: actual.owner_expectation,
            });
        }
    }

    Ok(())
}

pub fn expected_account_metas_for_tag(
    tag: XxxlGatewayInstructionTag,
) -> Vec<XxxlAccountMetaSkeleton> {
    account_requirements_for_tag(tag)
        .iter()
        .map(|expected| XxxlAccountMetaSkeleton {
            is_signer: expected.is_signer,
            is_writable: expected.is_writable,
            owner_expectation: expected.owner_expectation,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_order_skeleton::XxxlAccountOwnerExpectationSkeleton;

    #[test]
    fn account_validation_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_ACCOUNT_VALIDATION_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn expected_account_metas_match_initialize_gateway_config_requirements() {
        let metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        assert_eq!(metas.len(), 4);
        assert!(metas[0].is_signer);
        assert!(metas[0].is_writable);
        assert!(!metas[1].is_signer);
        assert!(metas[1].is_writable);

        assert_eq!(
            validate_account_metas_for_tag(
                XxxlGatewayInstructionTag::InitializeGatewayConfig,
                &metas
            ),
            Ok(())
        );
    }

    #[test]
    fn expected_account_metas_match_consume_gateway_mint_requirements() {
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        assert_eq!(metas.len(), 11);
        assert!(metas[0].is_signer);
        assert!(metas[0].is_writable);
        assert!(metas[4].is_writable);
        assert!(metas[5].is_writable);
        assert!(metas[6].is_writable);
        assert!(!metas[7].is_writable);

        assert_eq!(
            validate_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint, &metas),
            Ok(())
        );
    }

    #[test]
    fn validation_rejects_invalid_account_count() {
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);
        let shortened = &metas[..10];

        assert_eq!(
            validate_account_metas_for_tag(
                XxxlGatewayInstructionTag::ConsumeGatewayMint,
                shortened
            ),
            Err(XxxlAccountValidationSkeletonError::InvalidAccountCount {
                expected: 11,
                actual: 10
            })
        );
    }

    #[test]
    fn validation_rejects_signer_mismatch() {
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        metas[0].is_signer = false;

        assert_eq!(
            validate_account_metas_for_tag(
                XxxlGatewayInstructionTag::InitializeGatewayConfig,
                &metas
            ),
            Err(XxxlAccountValidationSkeletonError::SignerMismatch {
                index: 0,
                expected: true,
                actual: false
            })
        );
    }

    #[test]
    fn validation_rejects_writable_mismatch() {
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        metas[1].is_writable = false;

        assert_eq!(
            validate_account_metas_for_tag(
                XxxlGatewayInstructionTag::InitializeGatewayConfig,
                &metas
            ),
            Err(XxxlAccountValidationSkeletonError::WritableMismatch {
                index: 1,
                expected: true,
                actual: false
            })
        );
    }

    #[test]
    fn validation_rejects_owner_expectation_mismatch() {
        let mut metas =
            expected_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

        metas[4].owner_expectation = XxxlAccountOwnerExpectationSkeleton::SystemProgram;

        assert_eq!(
            validate_account_metas_for_tag(XxxlGatewayInstructionTag::InitializeMintState, &metas),
            Err(XxxlAccountValidationSkeletonError::OwnerExpectationMismatch {
                index: 4,
                expected: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
                actual: XxxlAccountOwnerExpectationSkeleton::SystemProgram
            })
        );
    }

    #[test]
    fn consume_gateway_mint_owner_expectations_are_explicit() {
        let metas = expected_account_metas_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        assert_eq!(
            metas[0].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::SignerWallet
        );
        assert_eq!(
            metas[1].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::ProgramOwned
        );
        assert_eq!(
            metas[5].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::SplTokenMint
        );
        assert_eq!(
            metas[6].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::SplTokenAccount
        );
        assert_eq!(
            metas[7].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress
        );
        assert_eq!(
            metas[8].owner_expectation,
            XxxlAccountOwnerExpectationSkeleton::SplTokenProgram
        );
    }
}
