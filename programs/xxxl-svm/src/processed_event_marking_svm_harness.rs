use core::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::{error::XxxlError, processed_event_marking_boundary::mark_processed_event_atomic};

pub const PHASE_41K4_MARKING_SVM_HARNESS_MAGIC: &[u8] = b"PHASE_41K4_MARK_ATOMIC_V1";

pub const PHASE_41K4_MARKING_SVM_HARNESS_INSTRUCTION_LEN: usize =
    PHASE_41K4_MARKING_SVM_HARNESS_MAGIC.len() + 32 + 32 + 32 + 16 + 8;

pub fn is_phase_41k4_marking_svm_harness_instruction(instruction_data: &[u8]) -> bool {
    instruction_data.starts_with(PHASE_41K4_MARKING_SVM_HARNESS_MAGIC)
}

pub fn process_phase_41k4_marking_svm_harness_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != PHASE_41K4_MARKING_SVM_HARNESS_INSTRUCTION_LEN
        || !is_phase_41k4_marking_svm_harness_instruction(instruction_data)
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let mut offset = PHASE_41K4_MARKING_SVM_HARNESS_MAGIC.len();

    let canonical_event_key = read_fixed_32(instruction_data, &mut offset)?;
    let route_id = read_fixed_32(instruction_data, &mut offset)?;
    let recipient = read_fixed_32(instruction_data, &mut offset)?;
    let consumed_amount = read_u128_le(instruction_data, &mut offset)?;
    let consumed_slot = read_u64_le(instruction_data, &mut offset)?;

    if offset != instruction_data.len() {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let account_info_iter = &mut accounts.iter();
    let processed_event_account = next_account_info(account_info_iter)?;
    let rent_payer = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;

    let rent = Rent::get()?;

    mark_processed_event_atomic(
        program_id,
        processed_event_account,
        rent_payer,
        system_program_account,
        &canonical_event_key,
        &route_id,
        &recipient,
        consumed_amount,
        consumed_slot,
        &rent,
    )
    .map(|_| ())
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

fn read_u128_le(data: &[u8], offset: &mut usize) -> Result<u128, ProgramError> {
    let end = offset
        .checked_add(16)
        .ok_or(XxxlError::InvalidInstruction)?;
    let bytes: [u8; 16] = data
        .get(*offset..end)
        .ok_or(XxxlError::InvalidInstruction)?
        .try_into()
        .map_err(|_| ProgramError::from(XxxlError::InvalidInstruction))?;

    *offset = end;
    Ok(u128::from_le_bytes(bytes))
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
