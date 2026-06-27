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

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";

    #[test]
    fn gateway_mint_authority_seeds_are_exact() {
        let seeds = gateway_mint_authority_seeds();

        assert_eq!(seeds[0], b"xxxl");
        assert_eq!(seeds[1], b"gateway-mint-authority");
        assert_eq!(seeds[2], b"v1");
    }

    #[test]
    fn gateway_mint_authority_uses_real_find_program_address() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, bump) = find_gateway_mint_authority(&program_id);
        let (expected_pda, expected_bump) =
            Pubkey::find_program_address(&gateway_mint_authority_seeds(), &program_id);

        assert_eq!(pda, expected_pda);
        assert_eq!(bump, expected_bump);
    }

    #[test]
    fn gateway_mint_authority_derivation_is_deterministic() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let left = find_gateway_mint_authority(&program_id);
        let right = find_gateway_mint_authority(&program_id);

        assert_eq!(left, right);
    }

    #[test]
    fn gateway_mint_authority_changes_with_program_id() {
        let first_program_id =
            Pubkey::from_str("11111111111111111111111111111111").expect("valid fixture program id");
        let second_program_id = Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111")
            .expect("valid fixture program id");

        let first = find_gateway_mint_authority(&first_program_id);
        let second = find_gateway_mint_authority(&second_program_id);

        assert_ne!(first.0, second.0);
    }

    #[test]
    fn gateway_mint_authority_fixture_is_not_placeholder_program_id() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (pda, _bump) = find_gateway_mint_authority(&program_id);

        assert_ne!(pda, program_id);
    }
}
