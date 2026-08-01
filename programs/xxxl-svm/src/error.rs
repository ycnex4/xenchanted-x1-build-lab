use solana_program::program_error::ProgramError;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlError {
    InvalidInstruction = 1,
    InvalidAccountOwner = 2,
    InvalidRentExemption = 3,
    InvalidRecipientAta = 4,
    InvalidPda = 5,
    InvalidDiscriminator = 6,
    InvalidVersion = 7,
    CpiBoundaryNotReady = 8,
    InvalidSourceChain = 9,
    InvalidInstructionReserved = 10,
    InvalidAccountData = 11,
    InvalidAuthority = 12,
    AccountAlreadyInitialized = 13,
    InsufficientRent = 14,
}

impl From<XxxlError> for ProgramError {
    fn from(error: XxxlError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
