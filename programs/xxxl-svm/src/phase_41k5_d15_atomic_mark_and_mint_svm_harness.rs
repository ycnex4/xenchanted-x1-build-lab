#[cfg(not(feature = "dangerously-allow-phase-41k5-spl-mint-to-cpi-test-gate-sbf-build"))]
compile_error!(
    "phase-41k5-spl-mint-to-cpi-test-gate is a non-production unsafe SVM test harness. \
     It bypasses the production SPL mint_to gate and must never be included in deploy artifacts. \
     For Mollusk SBF tests only, explicitly add feature \
     dangerously-allow-phase-41k5-spl-mint-to-cpi-test-gate-sbf-build."
);

use core::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::{
    cpi::{mint_to_cpi_boundary, MintToCpiAccounts, MintToCpiBoundary},
    error::XxxlError,
    processed_event_marking_boundary::mark_processed_event_atomic,
};

pub const PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC: &[u8] =
    b"PHASE_41K5_D15_MARK_MINT_V1";

pub const PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_INSTRUCTION_LEN: usize =
    PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC.len() + 32 + 32 + 32 + 8 + 1;

pub fn is_phase_41k5_d15_atomic_mark_and_mint_svm_harness_instruction(
    instruction_data: &[u8],
) -> bool {
    instruction_data.starts_with(PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC)
}

pub fn process_phase_41k5_d15_atomic_mark_and_mint_svm_harness_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_INSTRUCTION_LEN
        || !is_phase_41k5_d15_atomic_mark_and_mint_svm_harness_instruction(instruction_data)
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let mut offset = PHASE_41K5_D15_ATOMIC_MARK_AND_MINT_SVM_HARNESS_MAGIC.len();

    let canonical_event_key = read_fixed_32(instruction_data, &mut offset)?;
    let route_id = read_fixed_32(instruction_data, &mut offset)?;
    let recipient = read_fixed_32(instruction_data, &mut offset)?;
    let amount = read_u64_le(instruction_data, &mut offset)?;
    let mint_authority_bump = read_u8(instruction_data, &mut offset)?;

    if offset != instruction_data.len() || amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let account_info_iter = &mut accounts.iter();
    let processed_event_account = next_account_info(account_info_iter)?;
    let spl_token_mint_account = next_account_info(account_info_iter)?;
    let recipient_token_account = next_account_info(account_info_iter)?;
    let mint_authority_pda = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let rent_payer = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;

    let rent = Rent::get()?;
    let clock = Clock::get()?;

    mark_processed_event_atomic(
        program_id,
        processed_event_account,
        rent_payer,
        system_program_account,
        &canonical_event_key,
        &route_id,
        &recipient,
        amount as u128,
        clock.slot,
        &rent,
    )?;

    mint_to_cpi_boundary(
        program_id,
        MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program,
                mint: spl_token_mint_account,
                recipient_token_account,
                mint_authority_pda,
            },
            mint_authority_bump,
            amount,
        },
    )
}

fn read_fixed_32(data: &[u8], offset: &mut usize) -> Result<[u8; 32], ProgramError> {
    let end = offset
        .checked_add(32)
        .ok_or(XxxlError::InvalidInstruction)?;
    let value = data
        .get(*offset..end)
        .ok_or(XxxlError::InvalidInstruction)?
        .try_into()
        .map_err(|_| ProgramError::from(XxxlError::InvalidInstruction))?;

    *offset = end;
    Ok(value)
}

fn read_u64_le(data: &[u8], offset: &mut usize) -> Result<u64, ProgramError> {
    let end = offset.checked_add(8).ok_or(XxxlError::InvalidInstruction)?;
    let bytes: [u8; 8] = data
        .get(*offset..end)
        .ok_or(XxxlError::InvalidInstruction)?
        .try_into()
        .map_err(|_| ProgramError::from(XxxlError::InvalidInstruction))?;

    *offset = end;
    Ok(u64::from_le_bytes(bytes))
}

fn read_u8(data: &[u8], offset: &mut usize) -> Result<u8, ProgramError> {
    let value = *data.get(*offset).ok_or(XxxlError::InvalidInstruction)?;
    *offset = offset.checked_add(1).ok_or(XxxlError::InvalidInstruction)?;
    Ok(value)
}
