use solana_program::pubkey::Pubkey;

pub const GATEWAY_MINT_AUTHORITY_SEED_0: &[u8] = b"xxxl";
pub const GATEWAY_MINT_AUTHORITY_SEED_1: &[u8] = b"gateway-mint-authority";
pub const GATEWAY_MINT_AUTHORITY_SEED_2: &[u8] = b"v1";

pub fn gateway_mint_authority_seeds() -> [&'static [u8]; 3] {
    [
        GATEWAY_MINT_AUTHORITY_SEED_0,
        GATEWAY_MINT_AUTHORITY_SEED_1,
        GATEWAY_MINT_AUTHORITY_SEED_2,
    ]
}

pub fn find_gateway_mint_authority(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&gateway_mint_authority_seeds(), program_id)
}
