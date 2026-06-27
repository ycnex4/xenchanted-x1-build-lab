use solana_program::program_error::ProgramError;

use crate::error::XxxlError;

pub const CONSUME_GATEWAY_MINT_INSTRUCTION_LEN: usize = 208;
pub const INSTRUCTION_DISCRIMINATOR_LEN: usize = 8;
pub const INSTRUCTION_LAYOUT_VERSION: u16 = 1;

pub enum XxxlInstruction {
    ConsumeGatewayMint(ConsumeGatewayMintArgs),
}

pub struct ConsumeGatewayMintArgs {
    pub raw: [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
}

impl XxxlInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() != CONSUME_GATEWAY_MINT_INSTRUCTION_LEN {
            return Err(XxxlError::InvalidInstruction.into());
        }

        let version = u16::from_le_bytes([input[8], input[9]]);

        if version != INSTRUCTION_LAYOUT_VERSION {
            return Err(XxxlError::InvalidVersion.into());
        }

        let mut raw = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];
        raw.copy_from_slice(input);

        Ok(Self::ConsumeGatewayMint(ConsumeGatewayMintArgs { raw }))
    }
}
