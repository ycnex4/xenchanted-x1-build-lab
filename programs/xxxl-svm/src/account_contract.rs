use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::error::XxxlError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountWriteAccess {
    Readonly,
    Writable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountSignerRequirement {
    NotSigner,
    Signer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccountOwnerModel {
    ProgramOwned,
    SplTokenOwned,
    ProgramDerivedAddress,
    SplTokenProgram,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConsumeGatewayMintAccountContractEntry {
    pub index: usize,
    pub name: &'static str,
    pub write_access: AccountWriteAccess,
    pub signer_requirement: AccountSignerRequirement,
    pub owner_model: AccountOwnerModel,
}

pub const CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT: [ConsumeGatewayMintAccountContractEntry; 9] = [
    ConsumeGatewayMintAccountContractEntry {
        index: 0,
        name: "mint_state",
        write_access: AccountWriteAccess::Readonly,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 1,
        name: "gateway_config",
        write_access: AccountWriteAccess::Readonly,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 2,
        name: "guardian_set",
        write_access: AccountWriteAccess::Readonly,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramOwned,
    },
    // LEGACY / PRE-41K.4:
    // This instruction-level scaffold assumes processed_event is already
    // program-owned. It is not the Phase 41K.4 atomic marking account
    // manifest, where entry state is a system-owned empty-data PDA.
    ConsumeGatewayMintAccountContractEntry {
        index: 3,
        name: "processed_event",
        write_access: AccountWriteAccess::Writable,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 4,
        name: "recipient_balance",
        write_access: AccountWriteAccess::Writable,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 5,
        name: "spl_token_mint",
        write_access: AccountWriteAccess::Writable,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::SplTokenOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 6,
        name: "recipient_token_account",
        write_access: AccountWriteAccess::Writable,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::SplTokenOwned,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 7,
        name: "mint_authority_pda",
        write_access: AccountWriteAccess::Readonly,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::ProgramDerivedAddress,
    },
    ConsumeGatewayMintAccountContractEntry {
        index: 8,
        name: "token_program",
        write_access: AccountWriteAccess::Readonly,
        signer_requirement: AccountSignerRequirement::NotSigner,
        owner_model: AccountOwnerModel::SplTokenProgram,
    },
];

pub fn consume_gateway_mint_account_contract() -> &'static [ConsumeGatewayMintAccountContractEntry]
{
    &CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT
}

pub fn consume_gateway_mint_account_contract_entry(
    index: usize,
) -> Option<ConsumeGatewayMintAccountContractEntry> {
    CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT
        .iter()
        .find(|entry| entry.index == index)
        .copied()
}

pub fn assert_consume_gateway_mint_account_contract(
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    if accounts.len() != CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT.len() {
        return Err(XxxlError::InvalidInstruction.into());
    }

    for entry in CONSUME_GATEWAY_MINT_ACCOUNT_CONTRACT {
        let account = accounts
            .get(entry.index)
            .ok_or_else(|| ProgramError::from(XxxlError::InvalidInstruction))?;

        let expected_writable = matches!(entry.write_access, AccountWriteAccess::Writable);
        let expected_signer = matches!(entry.signer_requirement, AccountSignerRequirement::Signer);

        if account.is_writable != expected_writable || account.is_signer != expected_signer {
            return Err(XxxlError::InvalidInstruction.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instruction::{
            CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT,
            CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX,
            CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX,
            CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX,
            CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX,
            CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX,
        },
        processor::{
            ACCOUNT_INDEX_GATEWAY_CONFIG, ACCOUNT_INDEX_GUARDIAN_SET,
            ACCOUNT_INDEX_MINT_AUTHORITY_PDA, ACCOUNT_INDEX_MINT_STATE,
            ACCOUNT_INDEX_PROCESSED_EVENT, ACCOUNT_INDEX_RECIPIENT_BALANCE,
            ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT, ACCOUNT_INDEX_SPL_TOKEN_MINT,
            ACCOUNT_INDEX_TOKEN_PROGRAM, CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS,
        },
    };

    #[test]
    fn consume_gateway_mint_account_contract_has_expected_length_and_indices() {
        let contract = consume_gateway_mint_account_contract();

        assert_eq!(
            contract.len(),
            CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT as usize
        );
        assert_eq!(contract.len(), CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS);

        for (expected_index, entry) in contract.iter().enumerate() {
            assert_eq!(entry.index, expected_index);
        }
    }

    #[test]
    fn consume_gateway_mint_account_contract_matches_processor_indices() {
        assert_entry(ACCOUNT_INDEX_MINT_STATE, "mint_state");
        assert_entry(ACCOUNT_INDEX_GATEWAY_CONFIG, "gateway_config");
        assert_entry(ACCOUNT_INDEX_GUARDIAN_SET, "guardian_set");
        assert_entry(ACCOUNT_INDEX_PROCESSED_EVENT, "processed_event");
        assert_entry(ACCOUNT_INDEX_RECIPIENT_BALANCE, "recipient_balance");
        assert_entry(ACCOUNT_INDEX_SPL_TOKEN_MINT, "spl_token_mint");
        assert_entry(
            ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT,
            "recipient_token_account",
        );
        assert_entry(ACCOUNT_INDEX_MINT_AUTHORITY_PDA, "mint_authority_pda");
        assert_entry(ACCOUNT_INDEX_TOKEN_PROGRAM, "token_program");
    }

    #[test]
    fn consume_gateway_mint_account_contract_matches_instruction_indices() {
        assert_eq!(
            ACCOUNT_INDEX_MINT_STATE as u8,
            CONSUME_GATEWAY_MINT_MINT_STATE_ACCOUNT_INDEX
        );
        assert_eq!(
            ACCOUNT_INDEX_GATEWAY_CONFIG as u8,
            CONSUME_GATEWAY_MINT_ROUTE_ACCOUNT_INDEX
        );
        assert_eq!(
            ACCOUNT_INDEX_GUARDIAN_SET as u8,
            CONSUME_GATEWAY_MINT_GUARDIAN_SET_ACCOUNT_INDEX
        );
        assert_eq!(
            ACCOUNT_INDEX_PROCESSED_EVENT as u8,
            CONSUME_GATEWAY_MINT_PROCESSED_EVENT_ACCOUNT_INDEX
        );
        assert_eq!(
            ACCOUNT_INDEX_RECIPIENT_BALANCE as u8,
            CONSUME_GATEWAY_MINT_RECIPIENT_BALANCE_ACCOUNT_INDEX
        );
    }

    #[test]
    fn consume_gateway_mint_account_contract_marks_only_mutable_accounts_writable() {
        assert_readonly(ACCOUNT_INDEX_MINT_STATE);
        assert_readonly(ACCOUNT_INDEX_GATEWAY_CONFIG);
        assert_readonly(ACCOUNT_INDEX_GUARDIAN_SET);
        assert_writable(ACCOUNT_INDEX_PROCESSED_EVENT);
        assert_writable(ACCOUNT_INDEX_RECIPIENT_BALANCE);
        assert_writable(ACCOUNT_INDEX_SPL_TOKEN_MINT);
        assert_writable(ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT);
        assert_readonly(ACCOUNT_INDEX_MINT_AUTHORITY_PDA);
        assert_readonly(ACCOUNT_INDEX_TOKEN_PROGRAM);
    }

    #[test]
    fn consume_gateway_mint_account_contract_requires_no_external_signers() {
        for entry in consume_gateway_mint_account_contract() {
            assert_eq!(
                entry.signer_requirement,
                AccountSignerRequirement::NotSigner
            );
        }
    }

    #[test]
    fn consume_gateway_mint_account_contract_documents_owner_models() {
        assert_owner_model(ACCOUNT_INDEX_MINT_STATE, AccountOwnerModel::ProgramOwned);
        assert_owner_model(
            ACCOUNT_INDEX_GATEWAY_CONFIG,
            AccountOwnerModel::ProgramOwned,
        );
        assert_owner_model(ACCOUNT_INDEX_GUARDIAN_SET, AccountOwnerModel::ProgramOwned);
        assert_owner_model(
            ACCOUNT_INDEX_PROCESSED_EVENT,
            AccountOwnerModel::ProgramOwned,
        );
        assert_owner_model(
            ACCOUNT_INDEX_RECIPIENT_BALANCE,
            AccountOwnerModel::ProgramOwned,
        );
        assert_owner_model(
            ACCOUNT_INDEX_SPL_TOKEN_MINT,
            AccountOwnerModel::SplTokenOwned,
        );
        assert_owner_model(
            ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT,
            AccountOwnerModel::SplTokenOwned,
        );
        assert_owner_model(
            ACCOUNT_INDEX_MINT_AUTHORITY_PDA,
            AccountOwnerModel::ProgramDerivedAddress,
        );
        assert_owner_model(
            ACCOUNT_INDEX_TOKEN_PROGRAM,
            AccountOwnerModel::SplTokenProgram,
        );
    }

    #[test]
    fn consume_gateway_mint_account_contract_lookup_rejects_out_of_range_index() {
        assert!(consume_gateway_mint_account_contract_entry(
            CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS
        )
        .is_none());
    }

    fn assert_entry(index: usize, name: &str) {
        let entry = consume_gateway_mint_account_contract_entry(index)
            .expect("account contract entry should exist");
        assert_eq!(entry.name, name);
    }

    fn assert_readonly(index: usize) {
        let entry = consume_gateway_mint_account_contract_entry(index)
            .expect("account contract entry should exist");
        assert_eq!(entry.write_access, AccountWriteAccess::Readonly);
    }

    fn assert_writable(index: usize) {
        let entry = consume_gateway_mint_account_contract_entry(index)
            .expect("account contract entry should exist");
        assert_eq!(entry.write_access, AccountWriteAccess::Writable);
    }

    fn assert_owner_model(index: usize, owner_model: AccountOwnerModel) {
        let entry = consume_gateway_mint_account_contract_entry(index)
            .expect("account contract entry should exist");
        assert_eq!(entry.owner_model, owner_model);
    }
}
