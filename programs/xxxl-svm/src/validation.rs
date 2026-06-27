use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
};
use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};

use crate::error::XxxlError;

pub fn assert_account_owner(
    account: &AccountInfo,
    expected_owner: &Pubkey,
) -> Result<(), ProgramError> {
    if account.owner != expected_owner {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    Ok(())
}

pub fn assert_rent_exempt(account: &AccountInfo, rent: &Rent) -> Result<(), ProgramError> {
    if !rent.is_exempt(account.lamports(), account.data_len()) {
        return Err(XxxlError::InvalidRentExemption.into());
    }

    Ok(())
}

pub fn assert_spl_token_owned_account(account: &AccountInfo) -> Result<(), ProgramError> {
    assert_account_owner(account, &spl_token::id())
}

pub fn assert_initialized_mint_account(
    mint_account: &AccountInfo,
    expected_mint_authority: &Pubkey,
) -> Result<u8, ProgramError> {
    assert_spl_token_owned_account(mint_account)?;

    let data = mint_account.try_borrow_data()?;
    let mint = SplTokenMint::unpack(&data).map_err(|_| XxxlError::InvalidInstruction)?;

    if !mint.is_initialized {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if mint.mint_authority != COption::Some(*expected_mint_authority) {
        return Err(XxxlError::InvalidPda.into());
    }

    Ok(mint.decimals)
}

pub fn assert_recipient_ata_boundary(
    recipient_token_account: &AccountInfo,
    recipient_owner: &Pubkey,
    mint: &Pubkey,
) -> Result<(), ProgramError> {
    assert_spl_token_owned_account(recipient_token_account)?;

    let data = recipient_token_account.try_borrow_data()?;
    let token_account =
        SplTokenAccount::unpack(&data).map_err(|_| XxxlError::InvalidRecipientAta)?;

    if token_account.state != AccountState::Initialized {
        return Err(XxxlError::InvalidRecipientAta.into());
    }

    if token_account.owner != *recipient_owner {
        return Err(XxxlError::InvalidRecipientAta.into());
    }

    if token_account.mint != *mint {
        return Err(XxxlError::InvalidRecipientAta.into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey};
    use spl_token::state::{Account as SplTokenAccount, AccountState, Mint as SplTokenMint};

    #[test]
    fn account_owner_check_accepts_expected_owner() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = [];
        let account = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert_account_owner(&account, &owner).expect("owner matches");
    }

    #[test]
    fn account_owner_check_rejects_wrong_owner() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let wrong_owner = Pubkey::new_unique();
        let mut lamports = 0;
        let mut data = [];
        let account = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        let result = assert_account_owner(&account, &wrong_owner);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidAccountOwner as u32));
    }

    #[test]
    fn rent_exemption_check_accepts_minimum_balance() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let rent = Rent::default();
        let mut data = vec![0u8; SplTokenMint::LEN];
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        assert_rent_exempt(&account, &rent).expect("rent exempt");
    }

    #[test]
    fn rent_exemption_check_rejects_low_balance() {
        let key = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let rent = Rent::default();
        let mut data = vec![0u8; SplTokenMint::LEN];
        let mut lamports = rent.minimum_balance(data.len()).saturating_sub(1);
        let account = AccountInfo::new(
            &key,
            false,
            false,
            &mut lamports,
            &mut data,
            &owner,
            false,
            0,
        );

        let result = assert_rent_exempt(&account, &rent);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidRentExemption as u32));
    }

    #[test]
    fn initialized_mint_check_accepts_spl_token_mint_with_expected_authority() {
        let mint_key = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_mint(mint_authority, true);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let decimals =
            assert_initialized_mint_account(&account, &mint_authority).expect("valid mint");

        assert_eq!(decimals, 18);
    }

    #[test]
    fn initialized_mint_check_rejects_wrong_owner() {
        let mint_key = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let wrong_owner = Pubkey::new_unique();
        let rent = Rent::default();
        let mut data = packed_mint(mint_authority, true);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &wrong_owner,
            false,
            0,
        );

        let result = assert_initialized_mint_account(&account, &mint_authority);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidAccountOwner as u32));
    }

    #[test]
    fn initialized_mint_check_rejects_uninitialized_mint() {
        let mint_key = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_mint(mint_authority, false);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let result = assert_initialized_mint_account(&account, &mint_authority);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidInstruction as u32));
    }

    #[test]
    fn initialized_mint_check_rejects_wrong_mint_authority() {
        let mint_key = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let wrong_authority = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_mint(mint_authority, true);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let result = assert_initialized_mint_account(&account, &wrong_authority);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32));
    }

    #[test]
    fn recipient_ata_boundary_accepts_initialized_matching_token_account() {
        let token_account_key = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_token_account(mint, recipient_owner, AccountState::Initialized);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &token_account_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        assert_recipient_ata_boundary(&account, &recipient_owner, &mint)
            .expect("valid recipient token account");
    }

    #[test]
    fn recipient_ata_boundary_rejects_wrong_owner() {
        let token_account_key = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let wrong_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_token_account(mint, recipient_owner, AccountState::Initialized);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &token_account_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let result = assert_recipient_ata_boundary(&account, &wrong_owner, &mint);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidRecipientAta as u32));
    }

    #[test]
    fn recipient_ata_boundary_rejects_wrong_mint() {
        let token_account_key = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let wrong_mint = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_token_account(mint, recipient_owner, AccountState::Initialized);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &token_account_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let result = assert_recipient_ata_boundary(&account, &recipient_owner, &wrong_mint);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidRecipientAta as u32));
    }

    #[test]
    fn recipient_ata_boundary_rejects_uninitialized_token_account() {
        let token_account_key = Pubkey::new_unique();
        let recipient_owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let token_program = spl_token::id();
        let rent = Rent::default();
        let mut data = packed_token_account(mint, recipient_owner, AccountState::Uninitialized);
        let mut lamports = rent.minimum_balance(data.len());
        let account = AccountInfo::new(
            &token_account_key,
            false,
            true,
            &mut lamports,
            &mut data,
            &token_program,
            false,
            0,
        );

        let result = assert_recipient_ata_boundary(&account, &recipient_owner, &mint);

        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidRecipientAta as u32));
    }

    fn packed_mint(mint_authority: Pubkey, initialized: bool) -> Vec<u8> {
        let mut data = vec![0u8; SplTokenMint::LEN];
        let mint = SplTokenMint {
            mint_authority: COption::Some(mint_authority),
            supply: 0,
            decimals: 18,
            is_initialized: initialized,
            freeze_authority: COption::None,
        };

        SplTokenMint::pack(mint, &mut data).expect("pack mint");
        data
    }

    fn packed_token_account(
        mint: Pubkey,
        owner: Pubkey,
        state: AccountState,
    ) -> Vec<u8> {
        let mut data = vec![0u8; SplTokenAccount::LEN];
        let account = SplTokenAccount {
            mint,
            owner,
            amount: 0,
            delegate: COption::None,
            state,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        SplTokenAccount::pack(account, &mut data).expect("pack token account");
        data
    }
}
