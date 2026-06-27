use solana_program::program_error::ProgramError;

use crate::error::XxxlError;

pub const MINT_STATE_ACCOUNT_LEN: usize = 176;
pub const GATEWAY_CONFIG_ACCOUNT_LEN: usize = 256;
pub const GUARDIAN_SET_ACCOUNT_LEN: usize = 320;
pub const PROCESSED_EVENT_ACCOUNT_LEN: usize = 144;
pub const RECIPIENT_BALANCE_ACCOUNT_LEN: usize = 144;

pub const ACCOUNT_DISCRIMINATOR_LEN: usize = 8;
pub const VERSION_LEN: usize = 2;
pub const RUNTIME_LAYOUT_VERSION: u16 = 1;

pub const MINT_STATE_ACCOUNT_DISCRIMINATOR: [u8; ACCOUNT_DISCRIMINATOR_LEN] =
    [0x18, 0xf0, 0xf4, 0x99, 0x66, 0x90, 0x66, 0x60];
pub const GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR: [u8; ACCOUNT_DISCRIMINATOR_LEN] =
    [0xa6, 0x12, 0x0c, 0x7e, 0xd7, 0x69, 0x02, 0xae];
pub const GUARDIAN_SET_ACCOUNT_DISCRIMINATOR: [u8; ACCOUNT_DISCRIMINATOR_LEN] =
    [0xa6, 0xf6, 0xef, 0x1a, 0xae, 0xc6, 0x13, 0xae];
pub const PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR: [u8; ACCOUNT_DISCRIMINATOR_LEN] =
    [0x8f, 0x54, 0x5b, 0x81, 0x40, 0xa2, 0xd5, 0xb5];
pub const RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR: [u8; ACCOUNT_DISCRIMINATOR_LEN] =
    [0xb5, 0x63, 0x86, 0x24, 0x50, 0x14, 0xf5, 0xf4];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MintStateAccountView<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatewayConfigAccountView<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GuardianSetAccountView<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProcessedEventAccountView<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecipientBalanceAccountView<'a> {
    pub data: &'a [u8],
}

impl<'a> MintStateAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ProgramError> {
        assert_account_layout(
            data,
            MINT_STATE_ACCOUNT_LEN,
            &MINT_STATE_ACCOUNT_DISCRIMINATOR,
        )?;

        Ok(Self { data })
    }

    pub fn decimals(&self) -> u8 {
        self.data[10]
    }

    pub fn gateway_mint_authority_bump(&self) -> u8 {
        self.data[13]
    }

    pub fn total_supply(&self) -> u128 {
        read_u128_le(self.data, 48)
    }

    pub fn mint_pubkey(&self) -> [u8; 32] {
        read_fixed_32(self.data, 16)
    }

    pub fn gateway_mint_authority_pda(&self) -> [u8; 32] {
        read_fixed_32(self.data, 64)
    }
}

impl<'a> GatewayConfigAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ProgramError> {
        assert_account_layout(
            data,
            GATEWAY_CONFIG_ACCOUNT_LEN,
            &GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR,
        )?;

        Ok(Self { data })
    }

    pub fn source_chain_weight_bps(&self) -> u16 {
        read_u16_le(self.data, 12)
    }

    pub fn route_id(&self) -> [u8; 32] {
        read_fixed_32(self.data, 16)
    }

    pub fn source_chain_id(&self) -> u64 {
        read_u64_le(self.data, 48)
    }

    pub fn target_mint(&self) -> [u8; 32] {
        read_fixed_32(self.data, 88)
    }

    pub fn guardian_set_id(&self) -> [u8; 32] {
        read_fixed_32(self.data, 120)
    }
}

impl<'a> GuardianSetAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ProgramError> {
        assert_account_layout(
            data,
            GUARDIAN_SET_ACCOUNT_LEN,
            &GUARDIAN_SET_ACCOUNT_DISCRIMINATOR,
        )?;

        Ok(Self { data })
    }

    pub fn quorum_threshold(&self) -> u16 {
        read_u16_le(self.data, 12)
    }

    pub fn guardian_count(&self) -> u8 {
        self.data[14]
    }

    pub fn guardian_set_id(&self) -> [u8; 32] {
        read_fixed_32(self.data, 272)
    }
}

impl<'a> ProcessedEventAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ProgramError> {
        assert_account_layout(
            data,
            PROCESSED_EVENT_ACCOUNT_LEN,
            &PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        )?;

        Ok(Self { data })
    }

    pub fn consumed(&self) -> bool {
        self.data[10] != 0
    }

    pub fn canonical_event_key(&self) -> [u8; 32] {
        read_fixed_32(self.data, 16)
    }

    pub fn route_id(&self) -> [u8; 32] {
        read_fixed_32(self.data, 48)
    }

    pub fn recipient(&self) -> [u8; 32] {
        read_fixed_32(self.data, 80)
    }

    pub fn consumed_amount(&self) -> u128 {
        read_u128_le(self.data, 112)
    }
}

impl<'a> RecipientBalanceAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Result<Self, ProgramError> {
        assert_account_layout(
            data,
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            &RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        )?;

        Ok(Self { data })
    }

    pub fn owner(&self) -> [u8; 32] {
        read_fixed_32(self.data, 16)
    }

    pub fn mint(&self) -> [u8; 32] {
        read_fixed_32(self.data, 48)
    }

    pub fn balance(&self) -> u128 {
        read_u128_le(self.data, 80)
    }
}

pub fn mark_processed_event_consumed(
    data: &mut [u8],
    expected_canonical_event_key: [u8; 32],
    expected_route_id: [u8; 32],
    expected_recipient: [u8; 32],
    consumed_amount: u128,
    consumed_slot: u64,
) -> Result<(), ProgramError> {
    {
        let view = ProcessedEventAccountView::new(data)?;

        if view.consumed()
            || view.canonical_event_key() != expected_canonical_event_key
            || view.route_id() != expected_route_id
            || view.recipient() != expected_recipient
            || consumed_amount == 0
        {
            return Err(XxxlError::InvalidInstruction.into());
        }
    }

    data[10] = 1;
    data[112..128].copy_from_slice(&consumed_amount.to_le_bytes());
    data[128..136].copy_from_slice(&consumed_slot.to_le_bytes());

    Ok(())
}

pub fn credit_recipient_balance(
    data: &mut [u8],
    expected_owner: [u8; 32],
    expected_mint: [u8; 32],
    amount: u128,
    canonical_event_key: [u8; 32],
) -> Result<u128, ProgramError> {
    let current_balance = {
        let view = RecipientBalanceAccountView::new(data)?;

        if view.owner() != expected_owner || view.mint() != expected_mint || amount == 0 {
            return Err(XxxlError::InvalidRecipientAta.into());
        }

        view.balance()
    };

    let next_balance = current_balance
        .checked_add(amount)
        .ok_or(XxxlError::InvalidInstruction)?;

    data[80..96].copy_from_slice(&next_balance.to_le_bytes());
    data[96..128].copy_from_slice(&canonical_event_key);

    Ok(next_balance)
}

fn assert_account_layout(
    data: &[u8],
    expected_len: usize,
    expected_discriminator: &[u8; ACCOUNT_DISCRIMINATOR_LEN],
) -> Result<(), ProgramError> {
    if data.len() != expected_len {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if &data[0..ACCOUNT_DISCRIMINATOR_LEN] != expected_discriminator {
        return Err(XxxlError::InvalidDiscriminator.into());
    }

    if read_u16_le(data, 8) != RUNTIME_LAYOUT_VERSION {
        return Err(XxxlError::InvalidVersion.into());
    }

    Ok(())
}

fn read_fixed_32(input: &[u8], offset: usize) -> [u8; 32] {
    let mut output = [0u8; 32];
    output.copy_from_slice(&input[offset..offset + 32]);
    output
}

fn read_u16_le(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([input[offset], input[offset + 1]])
}

fn read_u64_le(input: &[u8], offset: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&input[offset..offset + 8]);
    u64::from_le_bytes(bytes)
}

fn read_u128_le(input: &[u8], offset: usize) -> u128 {
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&input[offset..offset + 16]);
    u128::from_le_bytes(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::program_error::ProgramError;

    #[test]
    fn account_layout_lengths_match_production_layout() {
        assert_eq!(MINT_STATE_ACCOUNT_LEN, 176);
        assert_eq!(GATEWAY_CONFIG_ACCOUNT_LEN, 256);
        assert_eq!(GUARDIAN_SET_ACCOUNT_LEN, 320);
        assert_eq!(PROCESSED_EVENT_ACCOUNT_LEN, 144);
        assert_eq!(RECIPIENT_BALANCE_ACCOUNT_LEN, 144);
    }

    #[test]
    fn mint_state_account_view_parses_known_fields() {
        let mut data = valid_account(MINT_STATE_ACCOUNT_LEN, MINT_STATE_ACCOUNT_DISCRIMINATOR);
        data[10] = 18;
        data[13] = 201;
        data[48..64].copy_from_slice(&500u128.to_le_bytes());

        let view = MintStateAccountView::new(&data).expect("valid mint state");

        assert_eq!(view.decimals(), 18);
        assert_eq!(view.gateway_mint_authority_bump(), 201);
        assert_eq!(view.total_supply(), 500);
    }

    #[test]
    fn gateway_config_account_view_parses_known_fields() {
        let mut data = valid_account(
            GATEWAY_CONFIG_ACCOUNT_LEN,
            GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR,
        );
        data[12..14].copy_from_slice(&10_000u16.to_le_bytes());
        data[16..48].copy_from_slice(&[0x11; 32]);
        data[48..56].copy_from_slice(&1u64.to_le_bytes());

        let view = GatewayConfigAccountView::new(&data).expect("valid gateway config");

        assert_eq!(view.source_chain_weight_bps(), 10_000);
        assert_eq!(view.route_id(), [0x11; 32]);
        assert_eq!(view.source_chain_id(), 1);
    }

    #[test]
    fn guardian_set_account_view_parses_known_fields() {
        let mut data = valid_account(GUARDIAN_SET_ACCOUNT_LEN, GUARDIAN_SET_ACCOUNT_DISCRIMINATOR);
        data[12..14].copy_from_slice(&2u16.to_le_bytes());
        data[14] = 3;

        let view = GuardianSetAccountView::new(&data).expect("valid guardian set");

        assert_eq!(view.quorum_threshold(), 2);
        assert_eq!(view.guardian_count(), 3);
    }

    #[test]
    fn processed_event_account_view_parses_known_fields() {
        let mut data = valid_account(
            PROCESSED_EVENT_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        );
        data[10] = 1;
        data[16..48].copy_from_slice(&[0x44; 32]);
        data[112..128].copy_from_slice(&1_000u128.to_le_bytes());

        let view = ProcessedEventAccountView::new(&data).expect("valid processed event");

        assert!(view.consumed());
        assert_eq!(view.canonical_event_key(), [0x44; 32]);
        assert_eq!(view.consumed_amount(), 1_000);
    }

    #[test]
    fn recipient_balance_account_view_parses_known_fields() {
        let mut data = valid_account(
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        );
        data[16..48].copy_from_slice(&[0x55; 32]);
        data[48..80].copy_from_slice(&[0x33; 32]);
        data[80..96].copy_from_slice(&200u128.to_le_bytes());

        let view = RecipientBalanceAccountView::new(&data).expect("valid recipient balance");

        assert_eq!(view.owner(), [0x55; 32]);
        assert_eq!(view.mint(), [0x33; 32]);
        assert_eq!(view.balance(), 200);
    }

    #[test]
    fn account_view_rejects_wrong_discriminator() {
        let mut data = valid_account(MINT_STATE_ACCOUNT_LEN, MINT_STATE_ACCOUNT_DISCRIMINATOR);
        data[0] ^= 0xff;

        assert_custom_error(
            MintStateAccountView::new(&data),
            XxxlError::InvalidDiscriminator,
        );
    }

    #[test]
    fn account_view_rejects_wrong_version() {
        let mut data = valid_account(MINT_STATE_ACCOUNT_LEN, MINT_STATE_ACCOUNT_DISCRIMINATOR);
        data[8..10].copy_from_slice(&2u16.to_le_bytes());

        assert_custom_error(MintStateAccountView::new(&data), XxxlError::InvalidVersion);
    }

    #[test]
    fn account_view_rejects_truncated_data() {
        let data = valid_account(MINT_STATE_ACCOUNT_LEN - 1, MINT_STATE_ACCOUNT_DISCRIMINATOR);

        assert_custom_error(
            MintStateAccountView::new(&data),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn processed_event_mutation_marks_event_consumed_and_writes_amount_and_slot() {
        let canonical_event_key = [0x44; 32];
        let route_id = [0x11; 32];
        let recipient = [0x55; 32];
        let mut data = valid_account(
            PROCESSED_EVENT_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&canonical_event_key);
        data[48..80].copy_from_slice(&route_id);
        data[80..112].copy_from_slice(&recipient);

        mark_processed_event_consumed(
            &mut data,
            canonical_event_key,
            route_id,
            recipient,
            1_000,
            77,
        )
        .expect("processed event mutation");

        let view = ProcessedEventAccountView::new(&data).expect("valid processed event");

        assert!(view.consumed());
        assert_eq!(view.consumed_amount(), 1_000);
        assert_eq!(read_u64_le(&data, 128), 77);
    }

    #[test]
    fn processed_event_mutation_rejects_replay() {
        let canonical_event_key = [0x44; 32];
        let route_id = [0x11; 32];
        let recipient = [0x55; 32];
        let mut data = valid_account(
            PROCESSED_EVENT_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        );

        data[10] = 1;
        data[16..48].copy_from_slice(&canonical_event_key);
        data[48..80].copy_from_slice(&route_id);
        data[80..112].copy_from_slice(&recipient);

        assert_custom_error(
            mark_processed_event_consumed(
                &mut data,
                canonical_event_key,
                route_id,
                recipient,
                1_000,
                77,
            ),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn processed_event_mutation_rejects_wrong_canonical_event_key() {
        let canonical_event_key = [0x44; 32];
        let route_id = [0x11; 32];
        let recipient = [0x55; 32];
        let mut data = valid_account(
            PROCESSED_EVENT_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&canonical_event_key);
        data[48..80].copy_from_slice(&route_id);
        data[80..112].copy_from_slice(&recipient);

        assert_custom_error(
            mark_processed_event_consumed(&mut data, [0x99; 32], route_id, recipient, 1_000, 77),
            XxxlError::InvalidInstruction,
        );
    }

    #[test]
    fn recipient_balance_mutation_credits_balance_and_writes_last_event_key() {
        let owner = [0x55; 32];
        let mint = [0x33; 32];
        let canonical_event_key = [0x44; 32];
        let mut data = valid_account(
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&owner);
        data[48..80].copy_from_slice(&mint);
        data[80..96].copy_from_slice(&200u128.to_le_bytes());

        let next_balance =
            credit_recipient_balance(&mut data, owner, mint, 1_000, canonical_event_key)
                .expect("credit recipient balance");

        let view = RecipientBalanceAccountView::new(&data).expect("valid recipient balance");

        assert_eq!(next_balance, 1_200);
        assert_eq!(view.balance(), 1_200);
        assert_eq!(read_fixed_32(&data, 96), canonical_event_key);
    }

    #[test]
    fn recipient_balance_mutation_rejects_wrong_owner() {
        let owner = [0x55; 32];
        let mint = [0x33; 32];
        let canonical_event_key = [0x44; 32];
        let mut data = valid_account(
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&owner);
        data[48..80].copy_from_slice(&mint);

        assert_custom_error(
            credit_recipient_balance(&mut data, [0x99; 32], mint, 1_000, canonical_event_key),
            XxxlError::InvalidRecipientAta,
        );
    }

    #[test]
    fn recipient_balance_mutation_rejects_overflow() {
        let owner = [0x55; 32];
        let mint = [0x33; 32];
        let canonical_event_key = [0x44; 32];
        let mut data = valid_account(
            RECIPIENT_BALANCE_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR,
        );

        data[16..48].copy_from_slice(&owner);
        data[48..80].copy_from_slice(&mint);
        data[80..96].copy_from_slice(&u128::MAX.to_le_bytes());

        assert_custom_error(
            credit_recipient_balance(&mut data, owner, mint, 1, canonical_event_key),
            XxxlError::InvalidInstruction,
        );
    }

    fn valid_account(len: usize, discriminator: [u8; ACCOUNT_DISCRIMINATOR_LEN]) -> Vec<u8> {
        let mut data = vec![0u8; len];

        if len >= ACCOUNT_DISCRIMINATOR_LEN {
            data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&discriminator);
        }

        if len >= 10 {
            data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
        }

        data
    }

    fn assert_custom_error<T>(result: Result<T, ProgramError>, error: XxxlError) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }
}
