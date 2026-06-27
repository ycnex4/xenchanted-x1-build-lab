use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::{
    instruction::XxxlInstruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = XxxlInstruction::unpack(instruction_data)?;

    match instruction {
        XxxlInstruction::ConsumeGatewayMint(args) => {
            process_consume_gateway_mint(program_id, accounts, args.raw.as_ref())
        }
    }
}

fn process_consume_gateway_mint(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("XXXL consume_gateway_mint scaffold reached");
    Ok(())
}
