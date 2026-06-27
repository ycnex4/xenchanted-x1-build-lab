use solana_program::{
    account_info::AccountInfo, instruction::Instruction, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::{
    error::XxxlError,
    pda::{
        find_gateway_mint_authority, GATEWAY_MINT_AUTHORITY_SEED_0, GATEWAY_MINT_AUTHORITY_SEED_1,
        GATEWAY_MINT_AUTHORITY_SEED_2,
    },
};

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

pub fn build_mint_to_instruction(
    token_program_id: &Pubkey,
    mint: &Pubkey,
    recipient_token_account: &Pubkey,
    mint_authority_pda: &Pubkey,
    amount: u64,
) -> Result<Instruction, ProgramError> {
    if amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    spl_token::instruction::mint_to(
        token_program_id,
        mint,
        recipient_token_account,
        mint_authority_pda,
        &[],
        amount,
    )
}

pub fn gateway_mint_authority_signer_seeds<'a>(bump: &'a u8) -> [&'a [u8]; 4] {
    [
        GATEWAY_MINT_AUTHORITY_SEED_0,
        GATEWAY_MINT_AUTHORITY_SEED_1,
        GATEWAY_MINT_AUTHORITY_SEED_2,
        core::slice::from_ref(bump),
    ]
}

pub fn assert_gateway_mint_authority_pda(
    program_id: &Pubkey,
    mint_authority_pda: &Pubkey,
    mint_authority_bump: u8,
) -> Result<(), ProgramError> {
    let (expected_pda, expected_bump) = find_gateway_mint_authority(program_id);

    if mint_authority_pda != &expected_pda || mint_authority_bump != expected_bump {
        return Err(XxxlError::InvalidPda.into());
    }

    Ok(())
}

pub fn mint_to_cpi_boundary(
    program_id: &Pubkey,
    boundary: MintToCpiBoundary<'_, '_>,
) -> Result<(), ProgramError> {
    assert_gateway_mint_authority_pda(
        program_id,
        boundary.accounts.mint_authority_pda.key,
        boundary.mint_authority_bump,
    )?;

    let instruction = build_mint_to_instruction(
        boundary.accounts.token_program.key,
        boundary.accounts.mint.key,
        boundary.accounts.recipient_token_account.key,
        boundary.accounts.mint_authority_pda.key,
        boundary.amount,
    )?;

    let signer_seeds = gateway_mint_authority_signer_seeds(&boundary.mint_authority_bump);

    invoke_signed(
        &instruction,
        &[
            boundary.accounts.token_program.clone(),
            boundary.accounts.mint.clone(),
            boundary.accounts.recipient_token_account.clone(),
            boundary.accounts.mint_authority_pda.clone(),
        ],
        &[&signer_seeds],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";

    #[test]
    fn mint_to_instruction_uses_spl_token_program_and_expected_accounts() {
        let mint = Pubkey::new_unique();
        let recipient_token_account = Pubkey::new_unique();
        let mint_authority_pda = Pubkey::new_unique();

        let instruction = build_mint_to_instruction(
            &spl_token::id(),
            &mint,
            &recipient_token_account,
            &mint_authority_pda,
            1_000,
        )
        .expect("valid mint_to instruction");

        assert_eq!(instruction.program_id, spl_token::id());
        assert_eq!(instruction.accounts.len(), 3);
        assert_eq!(instruction.accounts[0].pubkey, mint);
        assert_eq!(instruction.accounts[1].pubkey, recipient_token_account);
        assert_eq!(instruction.accounts[2].pubkey, mint_authority_pda);
        assert!(instruction.accounts[0].is_writable);
        assert!(instruction.accounts[1].is_writable);
        assert!(instruction.accounts[2].is_signer);
    }

    #[test]
    fn mint_to_instruction_rejects_zero_amount() {
        let result = build_mint_to_instruction(
            &spl_token::id(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            &Pubkey::new_unique(),
            0,
        );

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidInstruction as u32)
        );
    }

    #[test]
    fn signer_seeds_are_exact_and_include_bump() {
        let bump = 201;
        let seeds = gateway_mint_authority_signer_seeds(&bump);

        assert_eq!(seeds[0], b"xxxl");
        assert_eq!(seeds[1], b"gateway-mint-authority");
        assert_eq!(seeds[2], b"v1");
        assert_eq!(seeds[3], &[201]);
    }

    #[test]
    fn gateway_mint_authority_pda_accepts_real_derived_fixture() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);

        assert_gateway_mint_authority_pda(&program_id, &pda, bump)
            .expect("valid gateway mint authority PDA");
    }

    #[test]
    fn gateway_mint_authority_pda_rejects_wrong_pda() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        let wrong_pda = Pubkey::new_unique();

        let result = assert_gateway_mint_authority_pda(&program_id, &wrong_pda, bump);

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }

    #[test]
    fn gateway_mint_authority_pda_rejects_wrong_bump() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);

        let result = assert_gateway_mint_authority_pda(&program_id, &pda, bump.wrapping_add(1));

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }

    #[test]
    fn mint_to_boundary_rejects_wrong_pda_before_invoke_signed() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);

        let token_program_key = spl_token::id();
        let mint_key = Pubkey::new_unique();
        let recipient_key = Pubkey::new_unique();
        let wrong_pda_key = Pubkey::new_unique();

        let mut token_program_lamports = 0;
        let mut mint_lamports = 0;
        let mut recipient_lamports = 0;
        let mut pda_lamports = 0;

        let mut token_program_data = [];
        let mut mint_data = [];
        let mut recipient_data = [];
        let mut pda_data = [];

        let token_program_owner = Pubkey::new_unique();
        let token_program = AccountInfo::new(
            &token_program_key,
            false,
            false,
            &mut token_program_lamports,
            &mut token_program_data,
            &token_program_owner,
            true,
            0,
        );
        let mint = AccountInfo::new(
            &mint_key,
            false,
            true,
            &mut mint_lamports,
            &mut mint_data,
            &token_program_key,
            false,
            0,
        );
        let recipient_token_account = AccountInfo::new(
            &recipient_key,
            false,
            true,
            &mut recipient_lamports,
            &mut recipient_data,
            &token_program_key,
            false,
            0,
        );
        let mint_authority_pda = AccountInfo::new(
            &wrong_pda_key,
            false,
            false,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
            false,
            0,
        );

        let boundary = MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program: &token_program,
                mint: &mint,
                recipient_token_account: &recipient_token_account,
                mint_authority_pda: &mint_authority_pda,
            },
            mint_authority_bump: bump,
            amount: 1,
        };

        let result = mint_to_cpi_boundary(&program_id, boundary);

        assert!(
            matches!(result, Err(ProgramError::Custom(code)) if code == XxxlError::InvalidPda as u32)
        );
    }
}
