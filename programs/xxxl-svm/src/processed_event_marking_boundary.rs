use solana_program::{
    account_info::AccountInfo, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program,
};

use crate::{
    error::XxxlError,
    state::{
        ACCOUNT_DISCRIMINATOR_LEN, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
        PROCESSED_EVENT_ACCOUNT_LEN, RUNTIME_LAYOUT_VERSION,
    },
    verifier::{
        find_phase_41k_3_processed_event_pda, load_phase_41k_3_processed_registry_account_info,
        Phase41K3ProcessedRegistryAccountLoadingStatus, PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET,
        PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET, PROCESSED_EVENT_CONSUMED_OFFSET,
        PROCESSED_EVENT_CONSUMED_SLOT_OFFSET, PROCESSED_EVENT_PDA_SEED_0,
        PROCESSED_EVENT_PDA_SEED_1, PROCESSED_EVENT_RECIPIENT_OFFSET,
        PROCESSED_EVENT_ROUTE_ID_OFFSET,
    },
};

pub const PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_PHASE: &str = "41K.4";
pub const PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_VERSION: &str = "0.1.0";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Phase41K4ProcessedEventMarkingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_surface: &'static str,
    pub marking_only: bool,
    pub account_info_used: bool,
    pub requires_system_owned_empty_data_entry: bool,
    pub accepts_lamport_dusted_empty_pda: bool,
    pub uses_41k3_loader_as_gate: bool,
    pub derives_pda_from_authorized_canonical_event_key: bool,
    pub uses_canonical_find_program_address_bump: bool,
    pub ignores_caller_supplied_bump: bool,
    pub writes_final_consumed_image: bool,
    pub rejects_initialized_unconsumed_repair: bool,
    pub redecodes_with_41k3_loader_after_write: bool,
    pub system_program_cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
    pub production_route_enabled: bool,
}

pub const PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_REPORT:
    Phase41K4ProcessedEventMarkingBoundaryReport = Phase41K4ProcessedEventMarkingBoundaryReport {
    phase: PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_PHASE,
    version: PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_VERSION,
    runtime_surface: "processed_event_atomic_marking_boundary",
    marking_only: true,
    account_info_used: true,
    requires_system_owned_empty_data_entry: true,
    accepts_lamport_dusted_empty_pda: true,
    uses_41k3_loader_as_gate: true,
    derives_pda_from_authorized_canonical_event_key: true,
    uses_canonical_find_program_address_bump: true,
    ignores_caller_supplied_bump: true,
    writes_final_consumed_image: true,
    rejects_initialized_unconsumed_repair: true,
    redecodes_with_41k3_loader_after_write: true,
    system_program_cpi_enabled: true,
    invoke_signed_enabled: true,
    spl_token_mint_to_enabled: false,
    process_instruction_handler_added: false,
    live_route_enabled: false,
    production_route_enabled: false,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProcessedEventMarkingWitness {
    pub expected_processed_event_pda: Pubkey,
    pub pda_bump: u8,
    pub canonical_event_key: [u8; 32],
    pub route_id: [u8; 32],
    pub recipient: [u8; 32],
    pub consumed_amount: u128,
    pub consumed_slot: u64,
    pub rent_exempt_minimum_lamports: u64,
    pub rent_top_up_lamports: u64,
    pub final_redecode_passed: bool,
    pub system_program_cpi_used: bool,
    pub invoke_signed_used: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

pub fn phase_41k_4_processed_event_marking_boundary_report(
) -> &'static Phase41K4ProcessedEventMarkingBoundaryReport {
    &PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_REPORT
}

pub fn processed_event_signer_seeds<'a>(
    canonical_event_key: &'a [u8; 32],
    bump: &'a u8,
) -> [&'a [u8]; 4] {
    [
        PROCESSED_EVENT_PDA_SEED_0,
        PROCESSED_EVENT_PDA_SEED_1,
        canonical_event_key.as_ref(),
        core::slice::from_ref(bump),
    ]
}

pub fn build_final_consumed_processed_event_account_image(
    canonical_event_key: &[u8; 32],
    route_id: &[u8; 32],
    recipient: &[u8; 32],
    consumed_amount: u128,
    consumed_slot: u64,
) -> Result<[u8; PROCESSED_EVENT_ACCOUNT_LEN], ProgramError> {
    if consumed_amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let mut data = [0u8; PROCESSED_EVENT_ACCOUNT_LEN];

    data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR);
    data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
    data[PROCESSED_EVENT_CONSUMED_OFFSET] = 1;
    data[PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET
        ..PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET + 32]
        .copy_from_slice(canonical_event_key);
    data[PROCESSED_EVENT_ROUTE_ID_OFFSET..PROCESSED_EVENT_ROUTE_ID_OFFSET + 32]
        .copy_from_slice(route_id);
    data[PROCESSED_EVENT_RECIPIENT_OFFSET..PROCESSED_EVENT_RECIPIENT_OFFSET + 32]
        .copy_from_slice(recipient);
    data[PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET..PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET + 16]
        .copy_from_slice(&consumed_amount.to_le_bytes());
    data[PROCESSED_EVENT_CONSUMED_SLOT_OFFSET..PROCESSED_EVENT_CONSUMED_SLOT_OFFSET + 8]
        .copy_from_slice(&consumed_slot.to_le_bytes());

    Ok(data)
}

pub fn mark_processed_event_atomic<'a>(
    program_id: &Pubkey,
    processed_event_account: &AccountInfo<'a>,
    rent_payer: &AccountInfo<'a>,
    system_program_account: &AccountInfo<'a>,
    canonical_event_key: &[u8; 32],
    route_id: &[u8; 32],
    recipient: &[u8; 32],
    consumed_amount: u128,
    consumed_slot: u64,
    rent: &Rent,
) -> Result<ProcessedEventMarkingWitness, ProgramError> {
    if consumed_amount == 0 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if system_program_account.key != &system_program::id() {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    if !processed_event_account.is_writable
        || processed_event_account.is_signer
        || processed_event_account.executable
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if !rent_payer.is_signer || !rent_payer.is_writable {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let (expected_pda, pda_bump) =
        find_phase_41k_3_processed_event_pda(program_id, canonical_event_key);

    if processed_event_account.key != &expected_pda {
        return Err(XxxlError::InvalidPda.into());
    }

    let initial_load = load_phase_41k_3_processed_registry_account_info(
        Some(processed_event_account),
        program_id,
        canonical_event_key,
        route_id,
        recipient,
    );

    if initial_load.status
        != Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountUnprocessed
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let rent_exempt_minimum_lamports = rent.minimum_balance(PROCESSED_EVENT_ACCOUNT_LEN);
    let rent_top_up_lamports = top_up_processed_event_rent_if_needed(
        processed_event_account,
        rent_payer,
        rent_exempt_minimum_lamports,
    )?;

    allocate_processed_event_pda(processed_event_account, canonical_event_key, pda_bump)?;

    assign_processed_event_pda_to_program(
        processed_event_account,
        program_id,
        canonical_event_key,
        pda_bump,
    )?;

    let final_image = build_final_consumed_processed_event_account_image(
        canonical_event_key,
        route_id,
        recipient,
        consumed_amount,
        consumed_slot,
    )?;

    {
        let mut data = processed_event_account.try_borrow_mut_data()?;

        if data.len() != PROCESSED_EVENT_ACCOUNT_LEN {
            return Err(XxxlError::InvalidInstruction.into());
        }

        data.copy_from_slice(&final_image);
    }

    let final_load = load_phase_41k_3_processed_registry_account_info(
        Some(processed_event_account),
        program_id,
        canonical_event_key,
        route_id,
        recipient,
    );

    if final_load.status
        != Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountProcessed
        || final_load.consumed_amount != Some(consumed_amount)
        || final_load.consumed_slot != Some(consumed_slot)
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(ProcessedEventMarkingWitness {
        expected_processed_event_pda: expected_pda,
        pda_bump,
        canonical_event_key: *canonical_event_key,
        route_id: *route_id,
        recipient: *recipient,
        consumed_amount,
        consumed_slot,
        rent_exempt_minimum_lamports,
        rent_top_up_lamports,
        final_redecode_passed: true,
        system_program_cpi_used: true,
        invoke_signed_used: true,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    })
}

fn top_up_processed_event_rent_if_needed<'a>(
    processed_event_account: &AccountInfo<'a>,
    rent_payer: &AccountInfo<'a>,
    rent_exempt_minimum_lamports: u64,
) -> Result<u64, ProgramError> {
    let current_lamports = **processed_event_account.lamports.borrow();

    if current_lamports >= rent_exempt_minimum_lamports {
        return Ok(0);
    }

    let shortfall = rent_exempt_minimum_lamports
        .checked_sub(current_lamports)
        .ok_or(XxxlError::InvalidRentExemption)?;

    if **rent_payer.lamports.borrow() < shortfall {
        return Err(XxxlError::InvalidRentExemption.into());
    }

    let instruction =
        system_instruction::transfer(rent_payer.key, processed_event_account.key, shortfall);

    invoke_signed(
        &instruction,
        &[rent_payer.clone(), processed_event_account.clone()],
        &[],
    )?;

    Ok(shortfall)
}

fn allocate_processed_event_pda<'a>(
    processed_event_account: &AccountInfo<'a>,
    canonical_event_key: &[u8; 32],
    pda_bump: u8,
) -> Result<(), ProgramError> {
    let instruction = system_instruction::allocate(
        processed_event_account.key,
        PROCESSED_EVENT_ACCOUNT_LEN as u64,
    );
    let signer_seeds = processed_event_signer_seeds(canonical_event_key, &pda_bump);

    invoke_signed(
        &instruction,
        &[processed_event_account.clone()],
        &[&signer_seeds],
    )
}

fn assign_processed_event_pda_to_program<'a>(
    processed_event_account: &AccountInfo<'a>,
    program_id: &Pubkey,
    canonical_event_key: &[u8; 32],
    pda_bump: u8,
) -> Result<(), ProgramError> {
    let instruction = system_instruction::assign(processed_event_account.key, program_id);
    let signer_seeds = processed_event_signer_seeds(canonical_event_key, &pda_bump);

    invoke_signed(
        &instruction,
        &[processed_event_account.clone()],
        &[&signer_seeds],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{account_info::AccountInfo, clock::Epoch};

    const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
    const OTHER_CANONICAL_EVENT_KEY: [u8; 32] = [0x45; 32];
    const ROUTE_ID: [u8; 32] = [0x11; 32];
    const RECIPIENT: [u8; 32] = [0x55; 32];
    const CONSUMED_AMOUNT: u128 = 1_000;
    const CONSUMED_SLOT: u64 = 77;

    #[test]
    fn boundary_report_enables_only_marking_not_live_route_or_spl_mint() {
        let report = phase_41k_4_processed_event_marking_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41K_4_PROCESSED_EVENT_MARKING_BOUNDARY_PHASE
        );
        assert!(report.marking_only);
        assert!(report.account_info_used);
        assert!(report.requires_system_owned_empty_data_entry);
        assert!(report.accepts_lamport_dusted_empty_pda);
        assert!(report.uses_41k3_loader_as_gate);
        assert!(report.derives_pda_from_authorized_canonical_event_key);
        assert!(report.uses_canonical_find_program_address_bump);
        assert!(report.ignores_caller_supplied_bump);
        assert!(report.writes_final_consumed_image);
        assert!(report.rejects_initialized_unconsumed_repair);
        assert!(report.redecodes_with_41k3_loader_after_write);
        assert!(report.system_program_cpi_enabled);
        assert!(report.invoke_signed_enabled);
        assert!(!report.spl_token_mint_to_enabled);
        assert!(!report.process_instruction_handler_added);
        assert!(!report.live_route_enabled);
        assert!(!report.production_route_enabled);
    }

    #[test]
    fn signer_seeds_match_41k3_processed_event_pda_seed_format() {
        let bump = 201;
        let seeds = processed_event_signer_seeds(&CANONICAL_EVENT_KEY, &bump);

        assert_eq!(seeds[0], b"xxxl");
        assert_eq!(seeds[1], b"processed-event");
        assert_eq!(seeds[2], CANONICAL_EVENT_KEY.as_ref());
        assert_eq!(seeds[3], &[bump]);
    }

    #[test]
    fn final_consumed_image_redecodes_as_processed_with_41k3_loader_decoder() {
        let image = build_final_consumed_processed_event_account_image(
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
            CONSUMED_AMOUNT,
            CONSUMED_SLOT,
        )
        .expect("final image");

        let result = crate::verifier::decode_phase_41k_3_processed_event_account_data(
            &image,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountProcessed
        );
        assert_eq!(result.canonical_event_key, Some(CANONICAL_EVENT_KEY));
        assert_eq!(result.route_id, Some(ROUTE_ID));
        assert_eq!(result.recipient, Some(RECIPIENT));
        assert_eq!(result.consumed_amount, Some(CONSUMED_AMOUNT));
        assert_eq!(result.consumed_slot, Some(CONSUMED_SLOT));
    }

    #[test]
    fn final_consumed_image_uses_xxxl_mint_amount_as_consumed_amount() {
        let xxxl_mint_amount = 12_345u128;
        let image = build_final_consumed_processed_event_account_image(
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
            xxxl_mint_amount,
            CONSUMED_SLOT,
        )
        .expect("final image");

        let mut amount_bytes = [0u8; 16];
        amount_bytes.copy_from_slice(
            &image[PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET
                ..PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET + 16],
        );

        assert_eq!(u128::from_le_bytes(amount_bytes), xxxl_mint_amount);
    }

    #[test]
    fn final_consumed_image_rejects_zero_amount() {
        let result = build_final_consumed_processed_event_account_image(
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
            0,
            CONSUMED_SLOT,
        );

        assert!(result.is_err());
    }

    #[test]
    fn wrong_pda_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_empty(false, true, 0);
        fixture.processed_event_key = Pubkey::new_unique();

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data.len(), 0);
    }

    #[test]
    fn signer_processed_event_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_empty(true, true, 0);

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data.len(), 0);
    }

    #[test]
    fn readonly_processed_event_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_empty(false, false, 0);

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data.len(), 0);
    }

    #[test]
    fn executable_processed_event_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_empty(false, true, 0);
        fixture.processed_event_executable = true;

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data.len(), 0);
    }

    #[test]
    fn already_consumed_processed_event_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::program_owned_processed(1);

        assert_marking_rejects(&mut fixture);
        assert_eq!(
            fixture.processed_event_data[PROCESSED_EVENT_CONSUMED_OFFSET],
            1
        );
    }

    #[test]
    fn initialized_unconsumed_processed_event_rejects_and_is_not_repaired() {
        let mut fixture = MarkingFixture::program_owned_processed(0);

        assert_marking_rejects(&mut fixture);
        assert_eq!(
            fixture.processed_event_data[PROCESSED_EVENT_CONSUMED_OFFSET],
            0
        );
    }

    #[test]
    fn system_owned_nonzero_data_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_nonzero_data();

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data, vec![0u8; 1]);
    }

    #[test]
    fn wrong_system_program_account_rejects_before_cpi_or_mutation() {
        let mut fixture = MarkingFixture::system_owned_empty(false, true, 0);
        fixture.system_program_key = Pubkey::new_unique();

        assert_marking_rejects(&mut fixture);
        assert_eq!(fixture.processed_event_data.len(), 0);
    }

    #[test]
    fn rent_payer_must_be_signer_and_writable() {
        let mut fixture = MarkingFixture::system_owned_empty(false, true, 0);
        fixture.rent_payer_is_signer = false;

        assert_marking_rejects(&mut fixture);

        let mut fixture = MarkingFixture::system_owned_empty(false, true, 0);
        fixture.rent_payer_is_writable = false;

        assert_marking_rejects(&mut fixture);
    }

    struct MarkingFixture {
        program_id: Pubkey,
        processed_event_key: Pubkey,
        processed_event_owner: Pubkey,
        processed_event_lamports: u64,
        processed_event_data: Vec<u8>,
        processed_event_is_signer: bool,
        processed_event_is_writable: bool,
        processed_event_executable: bool,
        rent_payer_key: Pubkey,
        rent_payer_owner: Pubkey,
        rent_payer_lamports: u64,
        rent_payer_data: Vec<u8>,
        rent_payer_is_signer: bool,
        rent_payer_is_writable: bool,
        system_program_key: Pubkey,
        system_program_owner: Pubkey,
        system_program_lamports: u64,
        system_program_data: Vec<u8>,
    }

    impl MarkingFixture {
        fn system_owned_empty(is_signer: bool, is_writable: bool, lamports: u64) -> Self {
            let program_id = Pubkey::new_unique();
            let (processed_event_key, _) =
                find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);

            Self {
                program_id,
                processed_event_key,
                processed_event_owner: system_program::id(),
                processed_event_lamports: lamports,
                processed_event_data: Vec::new(),
                processed_event_is_signer: is_signer,
                processed_event_is_writable: is_writable,
                processed_event_executable: false,
                rent_payer_key: Pubkey::new_unique(),
                rent_payer_owner: system_program::id(),
                rent_payer_lamports: 10_000_000,
                rent_payer_data: Vec::new(),
                rent_payer_is_signer: true,
                rent_payer_is_writable: true,
                system_program_key: system_program::id(),
                system_program_owner: Pubkey::new_unique(),
                system_program_lamports: 0,
                system_program_data: Vec::new(),
            }
        }

        fn system_owned_nonzero_data() -> Self {
            let mut fixture = Self::system_owned_empty(false, true, 1);
            fixture.processed_event_data = vec![0u8; 1];
            fixture
        }

        fn program_owned_processed(consumed_flag: u8) -> Self {
            let mut fixture = Self::system_owned_empty(false, true, 1);
            fixture.processed_event_owner = fixture.program_id;
            fixture.processed_event_data = build_final_consumed_processed_event_account_image(
                &CANONICAL_EVENT_KEY,
                &ROUTE_ID,
                &RECIPIENT,
                CONSUMED_AMOUNT,
                CONSUMED_SLOT,
            )
            .expect("valid image")
            .to_vec();
            fixture.processed_event_data[PROCESSED_EVENT_CONSUMED_OFFSET] = consumed_flag;
            fixture
        }

        fn with_accounts<T>(&mut self, f: impl FnOnce(&[AccountInfo<'_>; 3]) -> T) -> T {
            let processed_event = AccountInfo::new(
                &self.processed_event_key,
                self.processed_event_is_signer,
                self.processed_event_is_writable,
                &mut self.processed_event_lamports,
                self.processed_event_data.as_mut_slice(),
                &self.processed_event_owner,
                self.processed_event_executable,
                Epoch::default(),
            );

            let rent_payer = AccountInfo::new(
                &self.rent_payer_key,
                self.rent_payer_is_signer,
                self.rent_payer_is_writable,
                &mut self.rent_payer_lamports,
                self.rent_payer_data.as_mut_slice(),
                &self.rent_payer_owner,
                false,
                Epoch::default(),
            );

            let system_program_account = AccountInfo::new(
                &self.system_program_key,
                false,
                false,
                &mut self.system_program_lamports,
                self.system_program_data.as_mut_slice(),
                &self.system_program_owner,
                true,
                Epoch::default(),
            );

            f(&[processed_event, rent_payer, system_program_account])
        }
    }

    fn assert_marking_rejects(fixture: &mut MarkingFixture) {
        let program_id = fixture.program_id;

        let result = fixture.with_accounts(|accounts| {
            mark_processed_event_atomic(
                &program_id,
                &accounts[0],
                &accounts[1],
                &accounts[2],
                &CANONICAL_EVENT_KEY,
                &ROUTE_ID,
                &RECIPIENT,
                CONSUMED_AMOUNT,
                CONSUMED_SLOT,
                &Rent::default(),
            )
        });

        assert!(result.is_err());
    }

    #[test]
    fn different_canonical_event_key_derives_different_processed_event_pda() {
        let program_id = Pubkey::new_unique();

        let (first, first_bump) =
            find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let (second, second_bump) =
            find_phase_41k_3_processed_event_pda(&program_id, &OTHER_CANONICAL_EVENT_KEY);

        assert_ne!(first, second);
        assert_ne!((first, first_bump), (second, second_bump));
    }
}
