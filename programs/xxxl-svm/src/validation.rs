use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
};

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

pub fn assert_recipient_ata_boundary(
    _recipient_token_account: &AccountInfo,
    _recipient_owner: &Pubkey,
    _mint: &Pubkey,
) -> Result<(), ProgramError> {
    // Real ATA validation fixture is intentionally deferred to the next stage.
    Err(XxxlError::InvalidRecipientAta.into())
}
