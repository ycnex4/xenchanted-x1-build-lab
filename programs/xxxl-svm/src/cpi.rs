use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::error::XxxlError;

pub struct MintToCpiAccounts<'a, 'b> {
    pub token_program: &'a AccountInfo<'b>,
    pub mint: &'a AccountInfo<'b>,
    pub recipient_token_account: &'a AccountInfo<'b>,
    pub mint_authority_pda: &'a AccountInfo<'b>,
}

pub struct MintToCpiBoundary<'a, 'b> {
    pub accounts: MintToCpiAccounts<'a, 'b>,
    pub mint_authority_bump: u8,
    pub amount: u64,
}

pub fn mint_to_cpi_boundary(
    _program_id: &Pubkey,
    _boundary: MintToCpiBoundary,
) -> Result<(), ProgramError> {
    // Real spl_token::instruction::mint_to + invoke_signed fixture is intentionally
    // deferred to the SPL Token CPI fixture stage.
    Err(XxxlError::CpiBoundaryNotReady.into())
}
