use solana_program::pubkey::Pubkey;

pub const XXXL_STATE_INSTRUCTION_SKELETON_STATUS: &str = "LOCAL_ONLY_NOT_DEPLOYABLE";

pub const GATEWAY_CONFIG_SEED_0: &[u8] = b"xxxl";
pub const GATEWAY_CONFIG_SEED_1: &[u8] = b"gateway-config";
pub const GATEWAY_CONFIG_SEED_2: &[u8] = b"v1";

pub const GUARDIAN_SET_SEED_0: &[u8] = b"xxxl";
pub const GUARDIAN_SET_SEED_1: &[u8] = b"guardian-set";

pub const MINT_STATE_SEED_0: &[u8] = b"xxxl";
pub const MINT_STATE_SEED_1: &[u8] = b"mint-state";

pub const PROCESSED_EVENT_SEED_0: &[u8] = b"xxxl";
pub const PROCESSED_EVENT_SEED_1: &[u8] = b"processed-event";

pub const GATEWAY_CONFIG_SEEDS: [&[u8]; 3] = [
    GATEWAY_CONFIG_SEED_0,
    GATEWAY_CONFIG_SEED_1,
    GATEWAY_CONFIG_SEED_2,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XxxlGatewayInstructionTag {
    InitializeGatewayConfig = 0,
    InitializeGuardianSet = 1,
    InitializeMintState = 2,
    ConsumeGatewayMint = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlGatewayInstructionTagDecodeError {
    InvalidTag,
}

impl TryFrom<u8> for XxxlGatewayInstructionTag {
    type Error = XxxlGatewayInstructionTagDecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::InitializeGatewayConfig),
            1 => Ok(Self::InitializeGuardianSet),
            2 => Ok(Self::InitializeMintState),
            3 => Ok(Self::ConsumeGatewayMint),
            _ => Err(XxxlGatewayInstructionTagDecodeError::InvalidTag),
        }
    }
}

impl From<XxxlGatewayInstructionTag> for u8 {
    fn from(value: XxxlGatewayInstructionTag) -> Self {
        value as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XxxlStateAccountKind {
    GatewayConfig,
    GuardianSet,
    MintState,
    ProcessedEvent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XxxlStateAccountSkeleton {
    pub kind: XxxlStateAccountKind,
    pub name: &'static str,
    pub discriminator: [u8; 8],
    pub version: u8,
    pub depends_on_program_id: bool,
    pub requires_idempotency_guard: bool,
    pub description: &'static str,
}

pub const GATEWAY_CONFIG_DISCRIMINATOR: [u8; 8] = *b"XCFG0001";
pub const GUARDIAN_SET_DISCRIMINATOR: [u8; 8] = *b"XGDN0001";
pub const MINT_STATE_DISCRIMINATOR: [u8; 8] = *b"XMNT0001";
pub const PROCESSED_EVENT_DISCRIMINATOR: [u8; 8] = *b"XEVT0001";

pub const XXXL_STATE_ACCOUNT_SKELETONS: [XxxlStateAccountSkeleton; 4] = [
    XxxlStateAccountSkeleton {
        kind: XxxlStateAccountKind::GatewayConfig,
        name: "gateway_config",
        discriminator: GATEWAY_CONFIG_DISCRIMINATOR,
        version: 1,
        depends_on_program_id: true,
        requires_idempotency_guard: true,
        description: "Global gateway route configuration for XXXL gateway minting.",
    },
    XxxlStateAccountSkeleton {
        kind: XxxlStateAccountKind::GuardianSet,
        name: "guardian_set",
        discriminator: GUARDIAN_SET_DISCRIMINATOR,
        version: 1,
        depends_on_program_id: true,
        requires_idempotency_guard: true,
        description: "Authoritative testnet guardian set descriptor.",
    },
    XxxlStateAccountSkeleton {
        kind: XxxlStateAccountKind::MintState,
        name: "mint_state",
        discriminator: MINT_STATE_DISCRIMINATOR,
        version: 1,
        depends_on_program_id: true,
        requires_idempotency_guard: true,
        description: "Program-owned metadata for the XXXL SPL mint.",
    },
    XxxlStateAccountSkeleton {
        kind: XxxlStateAccountKind::ProcessedEvent,
        name: "processed_event",
        discriminator: PROCESSED_EVENT_DISCRIMINATOR,
        version: 1,
        depends_on_program_id: true,
        requires_idempotency_guard: true,
        description: "Replay-protection marker for one canonical source burn event.",
    },
];

pub fn xxxl_state_account_skeletons() -> &'static [XxxlStateAccountSkeleton] {
    &XXXL_STATE_ACCOUNT_SKELETONS
}

pub fn gateway_config_pda(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&GATEWAY_CONFIG_SEEDS, program_id)
}

pub fn guardian_set_pda(program_id: &Pubkey, guardian_set_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GUARDIAN_SET_SEED_0,
            GUARDIAN_SET_SEED_1,
            &guardian_set_id.to_le_bytes(),
        ],
        program_id,
    )
}

pub fn mint_state_pda(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_STATE_SEED_0, MINT_STATE_SEED_1, mint.as_ref()], program_id)
}

pub fn processed_event_pda(program_id: &Pubkey, canonical_event_key: &[u8; 32]) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PROCESSED_EVENT_SEED_0,
            PROCESSED_EVENT_SEED_1,
            canonical_event_key,
        ],
        program_id,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";
    const OTHER_PROGRAM_ID: &str = "BPFLoaderUpgradeab1e11111111111111111111111";

    fn fixture_program_id() -> Pubkey {
        Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id")
    }

    fn other_program_id() -> Pubkey {
        Pubkey::from_str(OTHER_PROGRAM_ID).expect("valid other program id")
    }

    #[test]
    fn skeleton_status_is_local_only_not_deployable() {
        assert_eq!(
            XXXL_STATE_INSTRUCTION_SKELETON_STATUS,
            "LOCAL_ONLY_NOT_DEPLOYABLE"
        );
    }

    #[test]
    fn instruction_tags_are_reserved_and_stable() {
        assert_eq!(u8::from(XxxlGatewayInstructionTag::InitializeGatewayConfig), 0);
        assert_eq!(u8::from(XxxlGatewayInstructionTag::InitializeGuardianSet), 1);
        assert_eq!(u8::from(XxxlGatewayInstructionTag::InitializeMintState), 2);
        assert_eq!(u8::from(XxxlGatewayInstructionTag::ConsumeGatewayMint), 3);
    }

    #[test]
    fn instruction_tag_decoding_rejects_unknown_tags() {
        assert_eq!(
            XxxlGatewayInstructionTag::try_from(0),
            Ok(XxxlGatewayInstructionTag::InitializeGatewayConfig)
        );
        assert_eq!(
            XxxlGatewayInstructionTag::try_from(3),
            Ok(XxxlGatewayInstructionTag::ConsumeGatewayMint)
        );
        assert_eq!(
            XxxlGatewayInstructionTag::try_from(4),
            Err(XxxlGatewayInstructionTagDecodeError::InvalidTag)
        );
        assert_eq!(
            XxxlGatewayInstructionTag::try_from(u8::MAX),
            Err(XxxlGatewayInstructionTagDecodeError::InvalidTag)
        );
    }

    #[test]
    fn state_account_skeleton_inventory_is_complete() {
        let skeletons = xxxl_state_account_skeletons();

        assert_eq!(skeletons.len(), 4);
        assert_eq!(skeletons[0].name, "gateway_config");
        assert_eq!(skeletons[1].name, "guardian_set");
        assert_eq!(skeletons[2].name, "mint_state");
        assert_eq!(skeletons[3].name, "processed_event");

        for skeleton in skeletons {
            assert_eq!(skeleton.version, 1);
            assert!(skeleton.depends_on_program_id);
            assert!(skeleton.requires_idempotency_guard);
        }
    }

    #[test]
    fn discriminators_are_unique_and_eight_bytes() {
        let mut discriminators = xxxl_state_account_skeletons()
            .iter()
            .map(|skeleton| skeleton.discriminator)
            .collect::<Vec<_>>();

        discriminators.sort();
        discriminators.dedup();

        assert_eq!(discriminators.len(), xxxl_state_account_skeletons().len());
        for discriminator in discriminators {
            assert_eq!(discriminator.len(), 8);
        }
    }

    #[test]
    fn gateway_config_pda_is_program_id_parametric() {
        let first = gateway_config_pda(&fixture_program_id());
        let second = gateway_config_pda(&other_program_id());

        assert_ne!(first, second);
    }

    #[test]
    fn guardian_set_pda_changes_with_guardian_set_id() {
        let program_id = fixture_program_id();

        let first = guardian_set_pda(&program_id, 1);
        let second = guardian_set_pda(&program_id, 2);

        assert_ne!(first, second);
    }

    #[test]
    fn mint_state_pda_changes_with_mint() {
        let program_id = fixture_program_id();
        let first_mint = Pubkey::new_unique();
        let second_mint = Pubkey::new_unique();

        let first = mint_state_pda(&program_id, &first_mint);
        let second = mint_state_pda(&program_id, &second_mint);

        assert_ne!(first, second);
    }

    #[test]
    fn processed_event_pda_changes_with_canonical_event_key() {
        let program_id = fixture_program_id();
        let first_key = [1_u8; 32];
        let second_key = [2_u8; 32];

        let first = processed_event_pda(&program_id, &first_key);
        let second = processed_event_pda(&program_id, &second_key);

        assert_ne!(first, second);
    }
}
