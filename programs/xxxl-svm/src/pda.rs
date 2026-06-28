use solana_program::pubkey::Pubkey;

pub const GATEWAY_MINT_AUTHORITY_SEED_0: &[u8] = b"xxxl";
pub const GATEWAY_MINT_AUTHORITY_SEED_1: &[u8] = b"gateway-mint-authority";
pub const GATEWAY_MINT_AUTHORITY_SEED_2: &[u8] = b"v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlPdaDerivationKind {
    GatewayMintAuthority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlPdaDerivationInventoryEntry {
    pub kind: XxxlPdaDerivationKind,
    pub name: &'static str,
    pub seeds: [&'static [u8]; 3],
    pub seed_count: usize,
    pub depends_on_program_id: bool,
    pub description: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlPdaFixtureDerivationReport {
    pub kind: XxxlPdaDerivationKind,
    pub name: &'static str,
    pub program_id: Pubkey,
    pub pda: Pubkey,
    pub bump: u8,
}

pub const GATEWAY_MINT_AUTHORITY_SEEDS: [&[u8]; 3] = [
    GATEWAY_MINT_AUTHORITY_SEED_0,
    GATEWAY_MINT_AUTHORITY_SEED_1,
    GATEWAY_MINT_AUTHORITY_SEED_2,
];

pub const XXXL_PDA_DERIVATION_INVENTORY: [XxxlPdaDerivationInventoryEntry; 1] =
    [XxxlPdaDerivationInventoryEntry {
        kind: XxxlPdaDerivationKind::GatewayMintAuthority,
        name: "gateway_mint_authority",
        seeds: GATEWAY_MINT_AUTHORITY_SEEDS,
        seed_count: 3,
        depends_on_program_id: true,
        description:
            "PDA expected to act as the SPL Token mint authority for gateway-backed XXXL minting.",
    }];

pub fn gateway_mint_authority_seeds() -> [&'static [u8]; 3] {
    GATEWAY_MINT_AUTHORITY_SEEDS
}

pub fn find_gateway_mint_authority(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&gateway_mint_authority_seeds(), program_id)
}

pub fn derive_gateway_mint_authority_fixture_report(
    program_id: &Pubkey,
) -> XxxlPdaFixtureDerivationReport {
    let (pda, bump) = find_gateway_mint_authority(program_id);

    XxxlPdaFixtureDerivationReport {
        kind: XxxlPdaDerivationKind::GatewayMintAuthority,
        name: "gateway_mint_authority",
        program_id: *program_id,
        pda,
        bump,
    }
}

pub fn derive_xxxl_pda_fixture_reports(program_id: &Pubkey) -> [XxxlPdaFixtureDerivationReport; 1] {
    [derive_gateway_mint_authority_fixture_report(program_id)]
}

pub fn xxxl_pda_derivation_inventory() -> &'static [XxxlPdaDerivationInventoryEntry] {
    &XXXL_PDA_DERIVATION_INVENTORY
}

pub fn xxxl_pda_derivation_inventory_entry(
    kind: XxxlPdaDerivationKind,
) -> Option<XxxlPdaDerivationInventoryEntry> {
    XXXL_PDA_DERIVATION_INVENTORY
        .iter()
        .find(|entry| entry.kind == kind)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_contract::{consume_gateway_mint_account_contract, AccountOwnerModel};
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
    fn pda_derivation_inventory_is_explicit() {
        let inventory = xxxl_pda_derivation_inventory();

        assert_eq!(inventory.len(), 1);

        let entry = inventory[0];
        assert_eq!(entry.kind, XxxlPdaDerivationKind::GatewayMintAuthority);
        assert_eq!(entry.name, "gateway_mint_authority");
        assert_eq!(entry.seed_count, 3);
        assert!(entry.depends_on_program_id);
        assert_eq!(entry.seeds, gateway_mint_authority_seeds());
        assert!(entry.description.contains("mint authority"));
    }

    #[test]
    fn pda_derivation_inventory_entry_lookup_is_stable() {
        let entry =
            xxxl_pda_derivation_inventory_entry(XxxlPdaDerivationKind::GatewayMintAuthority)
                .expect("gateway mint authority PDA inventory entry");

        assert_eq!(entry.name, "gateway_mint_authority");
        assert_eq!(entry.seeds[0], GATEWAY_MINT_AUTHORITY_SEED_0);
        assert_eq!(entry.seeds[1], GATEWAY_MINT_AUTHORITY_SEED_1);
        assert_eq!(entry.seeds[2], GATEWAY_MINT_AUTHORITY_SEED_2);
    }

    #[test]
    fn pda_derivation_inventory_matches_account_contract() {
        let inventory_entry =
            xxxl_pda_derivation_inventory_entry(XxxlPdaDerivationKind::GatewayMintAuthority)
                .expect("gateway mint authority PDA inventory entry");

        let contract_entry = consume_gateway_mint_account_contract()
            .iter()
            .find(|entry| entry.name == "mint_authority_pda")
            .expect("mint authority PDA account contract entry");

        assert_eq!(
            inventory_entry.kind,
            XxxlPdaDerivationKind::GatewayMintAuthority
        );
        assert_eq!(
            contract_entry.owner_model,
            AccountOwnerModel::ProgramDerivedAddress
        );
    }

    #[test]
    fn gateway_mint_authority_fixture_report_matches_derivation() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let report = derive_gateway_mint_authority_fixture_report(&program_id);
        let (expected_pda, expected_bump) = find_gateway_mint_authority(&program_id);

        assert_eq!(report.kind, XxxlPdaDerivationKind::GatewayMintAuthority);
        assert_eq!(report.name, "gateway_mint_authority");
        assert_eq!(report.program_id, program_id);
        assert_eq!(report.pda, expected_pda);
        assert_eq!(report.bump, expected_bump);
    }

    #[test]
    fn pda_fixture_reports_match_inventory() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let reports = derive_xxxl_pda_fixture_reports(&program_id);
        let inventory = xxxl_pda_derivation_inventory();

        assert_eq!(reports.len(), inventory.len());
        assert_eq!(reports[0].kind, inventory[0].kind);
        assert_eq!(reports[0].name, inventory[0].name);
        assert_eq!(reports[0].program_id, program_id);
    }

    #[test]
    fn pda_fixture_reports_change_with_program_id() {
        let first_program_id =
            Pubkey::from_str("11111111111111111111111111111111").expect("valid fixture program id");
        let second_program_id = Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111")
            .expect("valid fixture program id");

        let first = derive_xxxl_pda_fixture_reports(&first_program_id);
        let second = derive_xxxl_pda_fixture_reports(&second_program_id);

        assert_eq!(first[0].program_id, first_program_id);
        assert_eq!(second[0].program_id, second_program_id);
        assert_ne!(first[0].pda, second[0].pda);
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
