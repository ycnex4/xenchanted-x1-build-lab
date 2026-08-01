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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlPdaFixtureVerificationError {
    WrongReportCount,
    WrongKind { index: usize },
    WrongName { index: usize },
    WrongProgramId { index: usize },
    WrongPda { index: usize },
    WrongBump { index: usize },
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

pub const GATEWAY_CONFIG_SEED_1: &[u8] = b"gateway-config";
pub const GUARDIAN_SET_SEED_1: &[u8] = b"guardian-set";
pub const MINT_STATE_SEED_1: &[u8] = b"mint-state";
pub const RECIPIENT_BALANCE_SEED_1: &[u8] = b"recipient-balance";

pub fn find_gateway_config(program_id: &Pubkey, route_id: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GATEWAY_MINT_AUTHORITY_SEED_0,
            GATEWAY_CONFIG_SEED_1,
            route_id,
        ],
        program_id,
    )
}

pub fn find_guardian_set(program_id: &Pubkey, guardian_set_id: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GATEWAY_MINT_AUTHORITY_SEED_0,
            GUARDIAN_SET_SEED_1,
            guardian_set_id,
        ],
        program_id,
    )
}

pub fn find_mint_state(program_id: &Pubkey, mint_id: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[GATEWAY_MINT_AUTHORITY_SEED_0, MINT_STATE_SEED_1, mint_id],
        program_id,
    )
}

pub fn find_recipient_balance(
    program_id: &Pubkey,
    recipient: &[u8; 32],
    mint: &[u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GATEWAY_MINT_AUTHORITY_SEED_0,
            RECIPIENT_BALANCE_SEED_1,
            recipient,
            mint,
        ],
        program_id,
    )
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

pub fn verify_xxxl_pda_fixture_reports(
    program_id: &Pubkey,
    reports: &[XxxlPdaFixtureDerivationReport],
) -> Result<(), XxxlPdaFixtureVerificationError> {
    let expected_reports = derive_xxxl_pda_fixture_reports(program_id);

    if reports.len() != expected_reports.len() {
        return Err(XxxlPdaFixtureVerificationError::WrongReportCount);
    }

    for (index, (actual, expected)) in reports.iter().zip(expected_reports.iter()).enumerate() {
        if actual.kind != expected.kind {
            return Err(XxxlPdaFixtureVerificationError::WrongKind { index });
        }

        if actual.name != expected.name {
            return Err(XxxlPdaFixtureVerificationError::WrongName { index });
        }

        if actual.program_id != expected.program_id {
            return Err(XxxlPdaFixtureVerificationError::WrongProgramId { index });
        }

        if actual.pda != expected.pda {
            return Err(XxxlPdaFixtureVerificationError::WrongPda { index });
        }

        if actual.bump != expected.bump {
            return Err(XxxlPdaFixtureVerificationError::WrongBump { index });
        }
    }

    Ok(())
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
    fn pda_fixture_verification_accepts_derived_reports() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let reports = derive_xxxl_pda_fixture_reports(&program_id);

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Ok(())
        );
    }

    #[test]
    fn pda_fixture_verification_rejects_wrong_report_count() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let reports: [XxxlPdaFixtureDerivationReport; 0] = [];

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Err(XxxlPdaFixtureVerificationError::WrongReportCount)
        );
    }

    #[test]
    fn pda_fixture_verification_rejects_wrong_name() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let mut reports = derive_xxxl_pda_fixture_reports(&program_id);
        reports[0].name = "wrong_gateway_mint_authority";

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Err(XxxlPdaFixtureVerificationError::WrongName { index: 0 })
        );
    }

    #[test]
    fn pda_fixture_verification_rejects_wrong_program_id() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let other_program_id = Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111")
            .expect("valid fixture program id");
        let mut reports = derive_xxxl_pda_fixture_reports(&program_id);
        reports[0].program_id = other_program_id;

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Err(XxxlPdaFixtureVerificationError::WrongProgramId { index: 0 })
        );
    }

    #[test]
    fn pda_fixture_verification_rejects_wrong_pda() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let other_pda = Pubkey::from_str("BPFLoaderUpgradeab1e11111111111111111111111")
            .expect("valid fixture pda");
        let mut reports = derive_xxxl_pda_fixture_reports(&program_id);
        reports[0].pda = other_pda;

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Err(XxxlPdaFixtureVerificationError::WrongPda { index: 0 })
        );
    }

    #[test]
    fn pda_fixture_verification_rejects_wrong_bump() {
        let program_id = Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let mut reports = derive_xxxl_pda_fixture_reports(&program_id);
        reports[0].bump = reports[0].bump.wrapping_add(1);

        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &reports),
            Err(XxxlPdaFixtureVerificationError::WrongBump { index: 0 })
        );
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

#[cfg(test)]
mod x1_testnet_program_id_candidate_dry_run_tests {
    use super::*;
    use solana_program::pubkey::Pubkey;
    use std::str::FromStr;

    #[test]
    #[ignore = "requires XXXL_TESTNET_PROGRAM_ID_CANDIDATE; off-chain only; no RPC, no deploy, no SOL spend"]
    fn x1_testnet_program_id_candidate_pda_dry_run() {
        let candidate = std::env::var("XXXL_TESTNET_PROGRAM_ID_CANDIDATE")
            .expect("set XXXL_TESTNET_PROGRAM_ID_CANDIDATE to the public Program ID candidate");

        assert!(!candidate.trim().is_empty());
        assert_ne!(candidate, crate::XXXL_PROGRAM_ID_PLACEHOLDER);
        assert_ne!(candidate, "11111111111111111111111111111111");
        assert_ne!(candidate, "BPFLoaderUpgradeab1e11111111111111111111111");
        assert_ne!(candidate, crate::XXXL_TOKEN_PROGRAM_ID);

        let program_id =
            Pubkey::from_str(&candidate).expect("candidate must be a valid SVM/Solana pubkey");

        let reports = derive_xxxl_pda_fixture_reports(&program_id);
        let report = reports[0];

        assert_eq!(reports.len(), 1);
        assert_eq!(report.kind, XxxlPdaDerivationKind::GatewayMintAuthority);
        assert_eq!(report.name, "gateway_mint_authority");
        assert_eq!(report.program_id, program_id);

        let (expected_pda, expected_bump) = find_gateway_mint_authority(&program_id);

        assert_eq!(report.pda, expected_pda);
        assert_eq!(report.bump, expected_bump);

        verify_xxxl_pda_fixture_reports(&program_id, &reports)
            .expect("candidate-derived PDA fixture must verify");

        let mut wrong_program_id_reports = reports;
        wrong_program_id_reports[0].program_id = Pubkey::default();
        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &wrong_program_id_reports),
            Err(XxxlPdaFixtureVerificationError::WrongProgramId { index: 0 })
        );

        let mut wrong_pda_reports = reports;
        wrong_pda_reports[0].pda = Pubkey::default();
        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &wrong_pda_reports),
            Err(XxxlPdaFixtureVerificationError::WrongPda { index: 0 })
        );

        let mut wrong_bump_reports = reports;
        wrong_bump_reports[0].bump = report.bump.wrapping_add(1);
        assert_eq!(
            verify_xxxl_pda_fixture_reports(&program_id, &wrong_bump_reports),
            Err(XxxlPdaFixtureVerificationError::WrongBump { index: 0 })
        );

        println!("XXXL_TESTNET_PROGRAM_ID_CANDIDATE={}", program_id);
        println!("GATEWAY_MINT_AUTHORITY_PDA={}", report.pda);
        println!("GATEWAY_MINT_AUTHORITY_BUMP={}", report.bump);
        println!("OFFCHAIN_ONLY=true");
        println!("RPC_USED=false");
        println!("DEPLOYED=false");
        println!("SOL_SPENT=false");
    }
}

#[cfg(test)]
mod state_provisioning_pda_tests {
    use super::*;
    use solana_program::pubkey::Pubkey;

    #[test]
    fn state_provisioning_pda_families_are_deterministic_and_distinct() {
        let program_id = Pubkey::new_unique();
        let route_id = [1u8; 32];
        let guardian_set_id = [2u8; 32];
        let mint_id = [3u8; 32];
        let recipient = [4u8; 32];
        let mint = [5u8; 32];

        let (gateway_config, gateway_config_bump) = find_gateway_config(&program_id, &route_id);
        let (guardian_set, guardian_set_bump) = find_guardian_set(&program_id, &guardian_set_id);
        let (mint_state, mint_state_bump) = find_mint_state(&program_id, &mint_id);
        let (recipient_balance, recipient_balance_bump) =
            find_recipient_balance(&program_id, &recipient, &mint);

        assert_eq!(
            Pubkey::find_program_address(&[b"xxxl", b"gateway-config", &route_id], &program_id),
            (gateway_config, gateway_config_bump)
        );
        assert_eq!(
            Pubkey::find_program_address(
                &[b"xxxl", b"guardian-set", &guardian_set_id],
                &program_id
            ),
            (guardian_set, guardian_set_bump)
        );
        assert_eq!(
            Pubkey::find_program_address(&[b"xxxl", b"mint-state", &mint_id], &program_id),
            (mint_state, mint_state_bump)
        );
        assert_eq!(
            Pubkey::find_program_address(
                &[b"xxxl", b"recipient-balance", &recipient, &mint],
                &program_id
            ),
            (recipient_balance, recipient_balance_bump)
        );

        assert_ne!(gateway_config, guardian_set);
        assert_ne!(gateway_config, mint_state);
        assert_ne!(gateway_config, recipient_balance);
    }
}
