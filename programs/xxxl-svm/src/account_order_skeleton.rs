use crate::state_instruction_skeleton::XxxlGatewayInstructionTag;

pub const XXXL_ACCOUNT_ORDER_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlAccountRoleSkeleton {
    Payer,
    Relayer,
    GatewayConfig,
    GuardianSet,
    MintState,
    ProcessedEvent,
    Mint,
    RecipientTokenAccount,
    GatewayMintAuthorityPda,
    TokenProgram,
    SystemProgram,
    RentSysvar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlAccountOwnerExpectationSkeleton {
    SignerWallet,
    ProgramOwned,
    ProgramDerivedAddress,
    SplTokenMint,
    SplTokenAccount,
    SplTokenProgram,
    SystemProgram,
    Sysvar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlAccountRequirementSkeleton {
    pub index: usize,
    pub role: XxxlAccountRoleSkeleton,
    pub name: &'static str,
    pub is_signer: bool,
    pub is_writable: bool,
    pub owner_expectation: XxxlAccountOwnerExpectationSkeleton,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlAccountOrderSkeletonError {
    InvalidAccountCount,
}

pub const INITIALIZE_GATEWAY_CONFIG_ACCOUNTS: [XxxlAccountRequirementSkeleton; 4] = [
    XxxlAccountRequirementSkeleton {
        index: 0,
        role: XxxlAccountRoleSkeleton::Payer,
        name: "payer",
        is_signer: true,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SignerWallet,
    },
    XxxlAccountRequirementSkeleton {
        index: 1,
        role: XxxlAccountRoleSkeleton::GatewayConfig,
        name: "gateway_config",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 2,
        role: XxxlAccountRoleSkeleton::SystemProgram,
        name: "system_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SystemProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 3,
        role: XxxlAccountRoleSkeleton::RentSysvar,
        name: "rent_sysvar",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::Sysvar,
    },
];

pub const INITIALIZE_GUARDIAN_SET_ACCOUNTS: [XxxlAccountRequirementSkeleton; 4] = [
    XxxlAccountRequirementSkeleton {
        index: 0,
        role: XxxlAccountRoleSkeleton::Payer,
        name: "payer",
        is_signer: true,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SignerWallet,
    },
    XxxlAccountRequirementSkeleton {
        index: 1,
        role: XxxlAccountRoleSkeleton::GuardianSet,
        name: "guardian_set",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 2,
        role: XxxlAccountRoleSkeleton::SystemProgram,
        name: "system_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SystemProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 3,
        role: XxxlAccountRoleSkeleton::RentSysvar,
        name: "rent_sysvar",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::Sysvar,
    },
];

pub const INITIALIZE_MINT_STATE_ACCOUNTS: [XxxlAccountRequirementSkeleton; 7] = [
    XxxlAccountRequirementSkeleton {
        index: 0,
        role: XxxlAccountRoleSkeleton::Payer,
        name: "payer",
        is_signer: true,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SignerWallet,
    },
    XxxlAccountRequirementSkeleton {
        index: 1,
        role: XxxlAccountRoleSkeleton::MintState,
        name: "mint_state",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 2,
        role: XxxlAccountRoleSkeleton::Mint,
        name: "mint",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenMint,
    },
    XxxlAccountRequirementSkeleton {
        index: 3,
        role: XxxlAccountRoleSkeleton::GatewayMintAuthorityPda,
        name: "gateway_mint_authority_pda",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
    },
    XxxlAccountRequirementSkeleton {
        index: 4,
        role: XxxlAccountRoleSkeleton::TokenProgram,
        name: "token_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 5,
        role: XxxlAccountRoleSkeleton::SystemProgram,
        name: "system_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SystemProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 6,
        role: XxxlAccountRoleSkeleton::RentSysvar,
        name: "rent_sysvar",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::Sysvar,
    },
];

pub const CONSUME_GATEWAY_MINT_ACCOUNTS: [XxxlAccountRequirementSkeleton; 11] = [
    XxxlAccountRequirementSkeleton {
        index: 0,
        role: XxxlAccountRoleSkeleton::Relayer,
        name: "payer_or_relayer",
        is_signer: true,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SignerWallet,
    },
    XxxlAccountRequirementSkeleton {
        index: 1,
        role: XxxlAccountRoleSkeleton::GatewayConfig,
        name: "gateway_config",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 2,
        role: XxxlAccountRoleSkeleton::GuardianSet,
        name: "guardian_set",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 3,
        role: XxxlAccountRoleSkeleton::MintState,
        name: "mint_state",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 4,
        role: XxxlAccountRoleSkeleton::ProcessedEvent,
        name: "processed_event",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramOwned,
    },
    XxxlAccountRequirementSkeleton {
        index: 5,
        role: XxxlAccountRoleSkeleton::Mint,
        name: "mint",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenMint,
    },
    XxxlAccountRequirementSkeleton {
        index: 6,
        role: XxxlAccountRoleSkeleton::RecipientTokenAccount,
        name: "recipient_token_account",
        is_signer: false,
        is_writable: true,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenAccount,
    },
    XxxlAccountRequirementSkeleton {
        index: 7,
        role: XxxlAccountRoleSkeleton::GatewayMintAuthorityPda,
        name: "gateway_mint_authority_pda",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::ProgramDerivedAddress,
    },
    XxxlAccountRequirementSkeleton {
        index: 8,
        role: XxxlAccountRoleSkeleton::TokenProgram,
        name: "token_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SplTokenProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 9,
        role: XxxlAccountRoleSkeleton::SystemProgram,
        name: "system_program",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::SystemProgram,
    },
    XxxlAccountRequirementSkeleton {
        index: 10,
        role: XxxlAccountRoleSkeleton::RentSysvar,
        name: "rent_sysvar",
        is_signer: false,
        is_writable: false,
        owner_expectation: XxxlAccountOwnerExpectationSkeleton::Sysvar,
    },
];

pub fn account_requirements_for_tag(
    tag: XxxlGatewayInstructionTag,
) -> &'static [XxxlAccountRequirementSkeleton] {
    match tag {
        XxxlGatewayInstructionTag::InitializeGatewayConfig => &INITIALIZE_GATEWAY_CONFIG_ACCOUNTS,
        XxxlGatewayInstructionTag::InitializeGuardianSet => &INITIALIZE_GUARDIAN_SET_ACCOUNTS,
        XxxlGatewayInstructionTag::InitializeMintState => &INITIALIZE_MINT_STATE_ACCOUNTS,
        XxxlGatewayInstructionTag::ConsumeGatewayMint => &CONSUME_GATEWAY_MINT_ACCOUNTS,
    }
}

pub fn required_account_count_for_tag(tag: XxxlGatewayInstructionTag) -> usize {
    account_requirements_for_tag(tag).len()
}

pub fn validate_account_count_for_tag(
    tag: XxxlGatewayInstructionTag,
    actual_count: usize,
) -> Result<(), XxxlAccountOrderSkeletonError> {
    if actual_count == required_account_count_for_tag(tag) {
        Ok(())
    } else {
        Err(XxxlAccountOrderSkeletonError::InvalidAccountCount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_order_skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_ACCOUNT_ORDER_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn initialize_gateway_config_account_order_is_stable() {
        let accounts =
            account_requirements_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig);

        assert_eq!(accounts.len(), 4);
        assert_eq!(accounts[0].role, XxxlAccountRoleSkeleton::Payer);
        assert_eq!(accounts[1].role, XxxlAccountRoleSkeleton::GatewayConfig);
        assert_eq!(accounts[2].role, XxxlAccountRoleSkeleton::SystemProgram);
        assert_eq!(accounts[3].role, XxxlAccountRoleSkeleton::RentSysvar);
        assert!(accounts[0].is_signer);
        assert!(accounts[1].is_writable);
    }

    #[test]
    fn initialize_guardian_set_account_order_is_stable() {
        let accounts =
            account_requirements_for_tag(XxxlGatewayInstructionTag::InitializeGuardianSet);

        assert_eq!(accounts.len(), 4);
        assert_eq!(accounts[0].role, XxxlAccountRoleSkeleton::Payer);
        assert_eq!(accounts[1].role, XxxlAccountRoleSkeleton::GuardianSet);
        assert_eq!(accounts[2].role, XxxlAccountRoleSkeleton::SystemProgram);
        assert_eq!(accounts[3].role, XxxlAccountRoleSkeleton::RentSysvar);
        assert!(accounts[0].is_signer);
        assert!(accounts[1].is_writable);
    }

    #[test]
    fn initialize_mint_state_account_order_is_stable() {
        let accounts = account_requirements_for_tag(XxxlGatewayInstructionTag::InitializeMintState);

        assert_eq!(accounts.len(), 7);
        assert_eq!(accounts[0].role, XxxlAccountRoleSkeleton::Payer);
        assert_eq!(accounts[1].role, XxxlAccountRoleSkeleton::MintState);
        assert_eq!(accounts[2].role, XxxlAccountRoleSkeleton::Mint);
        assert_eq!(
            accounts[3].role,
            XxxlAccountRoleSkeleton::GatewayMintAuthorityPda
        );
        assert_eq!(accounts[4].role, XxxlAccountRoleSkeleton::TokenProgram);
        assert_eq!(accounts[5].role, XxxlAccountRoleSkeleton::SystemProgram);
        assert_eq!(accounts[6].role, XxxlAccountRoleSkeleton::RentSysvar);
    }

    #[test]
    fn consume_gateway_mint_account_order_is_stable() {
        let accounts = account_requirements_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        assert_eq!(accounts.len(), 11);
        assert_eq!(accounts[0].role, XxxlAccountRoleSkeleton::Relayer);
        assert_eq!(accounts[1].role, XxxlAccountRoleSkeleton::GatewayConfig);
        assert_eq!(accounts[2].role, XxxlAccountRoleSkeleton::GuardianSet);
        assert_eq!(accounts[3].role, XxxlAccountRoleSkeleton::MintState);
        assert_eq!(accounts[4].role, XxxlAccountRoleSkeleton::ProcessedEvent);
        assert_eq!(accounts[5].role, XxxlAccountRoleSkeleton::Mint);
        assert_eq!(
            accounts[6].role,
            XxxlAccountRoleSkeleton::RecipientTokenAccount
        );
        assert_eq!(
            accounts[7].role,
            XxxlAccountRoleSkeleton::GatewayMintAuthorityPda
        );
        assert_eq!(accounts[8].role, XxxlAccountRoleSkeleton::TokenProgram);
        assert_eq!(accounts[9].role, XxxlAccountRoleSkeleton::SystemProgram);
        assert_eq!(accounts[10].role, XxxlAccountRoleSkeleton::RentSysvar);
    }

    #[test]
    fn consume_gateway_mint_writable_accounts_are_explicit() {
        let accounts = account_requirements_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint);

        assert!(accounts[0].is_writable);
        assert!(!accounts[1].is_writable);
        assert!(!accounts[2].is_writable);
        assert!(!accounts[3].is_writable);
        assert!(accounts[4].is_writable);
        assert!(accounts[5].is_writable);
        assert!(accounts[6].is_writable);
        assert!(!accounts[7].is_writable);
        assert!(!accounts[8].is_writable);
        assert!(!accounts[9].is_writable);
        assert!(!accounts[10].is_writable);
    }

    #[test]
    fn required_account_count_is_enforced() {
        assert_eq!(
            validate_account_count_for_tag(XxxlGatewayInstructionTag::InitializeGatewayConfig, 4),
            Ok(())
        );
        assert_eq!(
            validate_account_count_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint, 11),
            Ok(())
        );
        assert_eq!(
            validate_account_count_for_tag(XxxlGatewayInstructionTag::ConsumeGatewayMint, 10),
            Err(XxxlAccountOrderSkeletonError::InvalidAccountCount)
        );
    }

    #[test]
    fn account_indices_match_position() {
        for tag in [
            XxxlGatewayInstructionTag::InitializeGatewayConfig,
            XxxlGatewayInstructionTag::InitializeGuardianSet,
            XxxlGatewayInstructionTag::InitializeMintState,
            XxxlGatewayInstructionTag::ConsumeGatewayMint,
        ] {
            for (expected_index, account) in account_requirements_for_tag(tag).iter().enumerate() {
                assert_eq!(account.index, expected_index);
            }
        }
    }
}
