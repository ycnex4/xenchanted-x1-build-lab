use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::state::{
    ACCOUNT_DISCRIMINATOR_LEN, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR,
    PROCESSED_EVENT_ACCOUNT_LEN, RUNTIME_LAYOUT_VERSION,
};

use super::replay_protection_boundary::AuthoritativeProcessedRegistryViewRef;

pub const PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_PHASE: &str = "41K.3";
pub const PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_VERSION: &str = "0.1.0";

pub const PROCESSED_EVENT_CONSUMED_OFFSET: usize = 10;
pub const PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET: usize = 16;
pub const PROCESSED_EVENT_ROUTE_ID_OFFSET: usize = 48;
pub const PROCESSED_EVENT_RECIPIENT_OFFSET: usize = 80;
pub const PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET: usize = 112;
pub const PROCESSED_EVENT_CONSUMED_SLOT_OFFSET: usize = 128;

pub const PROCESSED_EVENT_PDA_SEED_0: &[u8] = b"xxxl";
pub const PROCESSED_EVENT_PDA_SEED_1: &[u8] = b"processed-event";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41K3ProcessedRegistryAccountLoadingStatus {
    ProcessedEventAccountUnprocessed,
    ProcessedEventAccountProcessed,
    ProcessedEventAccountRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41K3ProcessedRegistryAccountRejectionCase {
    MissingProcessedEventAccount,
    AccountDataBorrowFailed,
    ProcessedEventAccountIsSigner,
    ProcessedEventAccountIsExecutable,
    ProcessedEventAccountPdaMismatch,
    ProcessedEventAccountOwnerMismatch,
    SystemOwnedAccountWithNonzeroData,
    MissingDiscriminator,
    InvalidAccountDataLength,
    ZeroDiscriminator,
    WrongDiscriminator,
    UnsupportedSchemaVersion,
    CanonicalEventKeyMismatch,
    RouteIdMismatch,
    RecipientMismatch,
    InitializedButUnconsumedProcessedEvent,
    MalformedProcessedEventAccountData,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41K3ProcessedRegistryLoadWitness {
    processed_canonical_event_key: Option<[u8; 32]>,
    _private: (),
}

impl Phase41K3ProcessedRegistryLoadWitness {
    fn unprocessed() -> Self {
        Self {
            processed_canonical_event_key: None,
            _private: (),
        }
    }

    fn processed(canonical_event_key: [u8; 32]) -> Self {
        Self {
            processed_canonical_event_key: Some(canonical_event_key),
            _private: (),
        }
    }

    pub fn to_authoritative_processed_registry_view(
        &self,
    ) -> AuthoritativeProcessedRegistryViewRef<'_> {
        AuthoritativeProcessedRegistryViewRef::from_phase_41k_3_processed_registry_load_witness(self)
    }

    pub(crate) fn processed_canonical_event_key_ref(&self) -> Option<&[u8; 32]> {
        self.processed_canonical_event_key.as_ref()
    }

    pub fn processed_canonical_event_key(&self) -> Option<[u8; 32]> {
        self.processed_canonical_event_key
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41K3ProcessedRegistryAccountLoadingResult {
    pub status: Phase41K3ProcessedRegistryAccountLoadingStatus,
    pub rejection_case: Option<Phase41K3ProcessedRegistryAccountRejectionCase>,
    pub account_data_len: usize,
    pub canonical_event_key: Option<[u8; 32]>,
    pub route_id: Option<[u8; 32]>,
    pub recipient: Option<[u8; 32]>,
    pub consumed_amount: Option<u128>,
    pub consumed_slot: Option<u64>,
    pub account_key: Option<Pubkey>,
    pub expected_account_key: Option<Pubkey>,
    pub account_owner: Option<Pubkey>,
    pub expected_program_id: Option<Pubkey>,
    pub pda_bump: Option<u8>,
    pub account_info_used: bool,
    pub account_key_checked: bool,
    pub account_owner_checked: bool,
    pub pda_checked: bool,
    pub discriminator_checked: bool,
    pub zero_discriminator_rejected: bool,
    pub wrong_discriminator_rejected: bool,
    pub schema_version_checked: bool,
    pub canonical_event_key_checked: bool,
    pub route_id_checked: bool,
    pub recipient_checked: bool,
    pub consumed_checked: bool,
    pub processed_event_account_writable: bool,
    pub processed_event_account_non_signer: bool,
    pub processed_event_account_non_executable: bool,
    pub system_owned_empty_data_unprocessed: bool,
    pub lamports_ignored_for_uninitialized_classification: bool,
    pub total_fail_closed_classification: bool,
    pub source_marker_authoritative_runtime_account: bool,
    pub authoritative_witness_constructed: bool,
    pub authoritative_view_witness: Option<Phase41K3ProcessedRegistryLoadWitness>,
    pub processed_registry_runtime_loading_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Phase41K3ProcessedRegistryAccountLoadingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_surface: &'static str,
    pub pure_account_data_decoder_enabled: bool,
    pub uses_account_info: bool,
    pub checks_account_key: bool,
    pub checks_account_owner: bool,
    pub checks_pda: bool,
    pub fixed_processed_event_pda_seed_format: bool,
    pub uses_canonical_find_program_address_bump: bool,
    pub ignores_caller_supplied_bump: bool,
    pub allows_writable_processed_event_account: bool,
    pub rejects_signer: bool,
    pub rejects_executable: bool,
    pub accepts_system_owned_empty_data_as_unprocessed: bool,
    pub ignores_lamports_for_uninitialized_classification: bool,
    pub rejects_system_owned_nonzero_data: bool,
    pub checks_discriminator: bool,
    pub rejects_zero_discriminator: bool,
    pub rejects_wrong_discriminator: bool,
    pub checks_schema_version: bool,
    pub checks_canonical_event_key: bool,
    pub checks_route_id: bool,
    pub checks_recipient: bool,
    pub rejects_initialized_unconsumed: bool,
    pub total_fail_closed_classification: bool,
    pub constructs_authoritative_witness: bool,
    pub adapts_to_41j_list_based_view: bool,
    pub processed_registry_runtime_loading_enabled: bool,
    pub replay_write_enabled: bool,
    pub processed_event_marking_enabled: bool,
    pub account_mutation_enabled: bool,
    pub cpi_enabled: bool,
    pub invoke_signed_enabled: bool,
    pub spl_token_mint_to_enabled: bool,
    pub process_instruction_handler_added: bool,
    pub live_route_enabled: bool,
}

pub const PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_REPORT:
    Phase41K3ProcessedRegistryAccountLoadingBoundaryReport =
    Phase41K3ProcessedRegistryAccountLoadingBoundaryReport {
        phase: PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_PHASE,
        version: PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_VERSION,
        runtime_surface: "processed_registry_accountinfo_pda_loading",
        pure_account_data_decoder_enabled: true,
        uses_account_info: true,
        checks_account_key: true,
        checks_account_owner: true,
        checks_pda: true,
        fixed_processed_event_pda_seed_format: true,
        uses_canonical_find_program_address_bump: true,
        ignores_caller_supplied_bump: true,
        allows_writable_processed_event_account: true,
        rejects_signer: true,
        rejects_executable: true,
        accepts_system_owned_empty_data_as_unprocessed: true,
        ignores_lamports_for_uninitialized_classification: true,
        rejects_system_owned_nonzero_data: true,
        checks_discriminator: true,
        rejects_zero_discriminator: true,
        rejects_wrong_discriminator: true,
        checks_schema_version: true,
        checks_canonical_event_key: true,
        checks_route_id: true,
        checks_recipient: true,
        rejects_initialized_unconsumed: true,
        total_fail_closed_classification: true,
        constructs_authoritative_witness: true,
        adapts_to_41j_list_based_view: true,
        processed_registry_runtime_loading_enabled: true,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    };

pub fn phase_41k_3_processed_registry_account_loading_boundary_report(
) -> Phase41K3ProcessedRegistryAccountLoadingBoundaryReport {
    PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_REPORT
}

pub fn find_phase_41k_3_processed_event_pda(
    program_id: &Pubkey,
    canonical_event_key: &[u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            PROCESSED_EVENT_PDA_SEED_0,
            PROCESSED_EVENT_PDA_SEED_1,
            canonical_event_key,
        ],
        program_id,
    )
}

pub fn load_phase_41k_3_processed_registry_account_info(
    processed_event_account: Option<&AccountInfo<'_>>,
    expected_program_id: &Pubkey,
    expected_canonical_event_key: &[u8; 32],
    expected_route_id: &[u8; 32],
    expected_recipient: &[u8; 32],
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    let (expected_pda, pda_bump) =
        find_phase_41k_3_processed_event_pda(expected_program_id, expected_canonical_event_key);

    let Some(processed_event_account) = processed_event_account else {
        return with_missing_account_metadata(
            rejected(
                0,
                Phase41K3ProcessedRegistryAccountRejectionCase::MissingProcessedEventAccount,
                None,
                None,
                None,
                None,
                None,
            ),
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    };

    if *processed_event_account.key != expected_pda {
        return with_account_metadata(
            rejected(
                processed_event_account.data_len(),
                Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountPdaMismatch,
                None,
                None,
                None,
                None,
                None,
            ),
            processed_event_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
            true,
            false,
            true,
        );
    }

    if processed_event_account.is_signer {
        return with_account_metadata(
            rejected(
                processed_event_account.data_len(),
                Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountIsSigner,
                None,
                None,
                None,
                None,
                None,
            ),
            processed_event_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
            true,
            false,
            true,
        );
    }

    if processed_event_account.executable {
        return with_account_metadata(
            rejected(
                processed_event_account.data_len(),
                Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountIsExecutable,
                None,
                None,
                None,
                None,
                None,
            ),
            processed_event_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
            true,
            false,
            true,
        );
    }

    let system_program_id = Pubkey::from([0u8; 32]);

    if *processed_event_account.owner == system_program_id {
        if processed_event_account.data_len() == 0 {
            return with_account_metadata(
                unprocessed(processed_event_account.data_len()),
                processed_event_account,
                expected_pda,
                *expected_program_id,
                pda_bump,
                true,
                true,
                true,
            );
        }

        return with_account_metadata(
            rejected(
                processed_event_account.data_len(),
                Phase41K3ProcessedRegistryAccountRejectionCase::SystemOwnedAccountWithNonzeroData,
                None,
                None,
                None,
                None,
                None,
            ),
            processed_event_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
            true,
            true,
            true,
        );
    }

    if *processed_event_account.owner != *expected_program_id {
        return with_account_metadata(
            rejected(
                processed_event_account.data_len(),
                Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountOwnerMismatch,
                None,
                None,
                None,
                None,
                None,
            ),
            processed_event_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
            true,
            true,
            true,
        );
    }

    let data = match processed_event_account.try_borrow_data() {
        Ok(data) => data,
        Err(_) => {
            return with_account_metadata(
                rejected(
                    processed_event_account.data_len(),
                    Phase41K3ProcessedRegistryAccountRejectionCase::AccountDataBorrowFailed,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                processed_event_account,
                expected_pda,
                *expected_program_id,
                pda_bump,
                true,
                true,
                true,
            );
        }
    };

    let result = decode_phase_41k_3_processed_event_account_data(
        data.as_ref(),
        expected_canonical_event_key,
        expected_route_id,
        expected_recipient,
    );

    with_account_metadata(
        result,
        processed_event_account,
        expected_pda,
        *expected_program_id,
        pda_bump,
        true,
        true,
        true,
    )
}

pub fn decode_phase_41k_3_processed_event_account_data(
    data: &[u8],
    expected_canonical_event_key: &[u8; 32],
    expected_route_id: &[u8; 32],
    expected_recipient: &[u8; 32],
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    if data.len() < ACCOUNT_DISCRIMINATOR_LEN {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MissingDiscriminator,
            None,
            None,
            None,
            None,
            None,
        );
    }

    if data.len() != PROCESSED_EVENT_ACCOUNT_LEN {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::InvalidAccountDataLength,
            None,
            None,
            None,
            None,
            None,
        );
    }

    let Some(discriminator) = read_fixed::<ACCOUNT_DISCRIMINATOR_LEN>(data, 0) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MissingDiscriminator,
            None,
            None,
            None,
            None,
            None,
        );
    };

    if discriminator == [0u8; ACCOUNT_DISCRIMINATOR_LEN] {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::ZeroDiscriminator,
            None,
            None,
            None,
            None,
            None,
        );
    }

    if discriminator != PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::WrongDiscriminator,
            None,
            None,
            None,
            None,
            None,
        );
    }

    let Some(version) = read_u16_le(data, 8) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::UnsupportedSchemaVersion,
            None,
            None,
            None,
            None,
            None,
        );
    };

    if version != RUNTIME_LAYOUT_VERSION {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::UnsupportedSchemaVersion,
            None,
            None,
            None,
            None,
            None,
        );
    }

    let Some(consumed_flag) = read_u8(data, PROCESSED_EVENT_CONSUMED_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            None,
            None,
            None,
            None,
            None,
        );
    };

    let Some(canonical_event_key) =
        read_fixed::<32>(data, PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET)
    else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            None,
            None,
            None,
            None,
            None,
        );
    };

    let Some(route_id) = read_fixed::<32>(data, PROCESSED_EVENT_ROUTE_ID_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            Some(canonical_event_key),
            None,
            None,
            None,
            None,
        );
    };

    let Some(recipient) = read_fixed::<32>(data, PROCESSED_EVENT_RECIPIENT_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            Some(canonical_event_key),
            Some(route_id),
            None,
            None,
            None,
        );
    };

    let Some(consumed_amount) = read_u128_le(data, PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            None,
            None,
        );
    };

    let Some(consumed_slot) = read_u64_le(data, PROCESSED_EVENT_CONSUMED_SLOT_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            None,
        );
    };

    if &canonical_event_key != expected_canonical_event_key {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::CanonicalEventKeyMismatch,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            Some(consumed_slot),
        );
    }

    if &route_id != expected_route_id {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::RouteIdMismatch,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            Some(consumed_slot),
        );
    }

    if &recipient != expected_recipient {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::RecipientMismatch,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            Some(consumed_slot),
        );
    }

    if consumed_flag == 0 {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::InitializedButUnconsumedProcessedEvent,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            Some(consumed_slot),
        );
    }

    if consumed_flag != 1 {
        return rejected(
            data.len(),
            Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            Some(canonical_event_key),
            Some(route_id),
            Some(recipient),
            Some(consumed_amount),
            Some(consumed_slot),
        );
    }

    processed(
        data.len(),
        canonical_event_key,
        route_id,
        recipient,
        consumed_amount,
        consumed_slot,
    )
}

fn unprocessed(account_data_len: usize) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    let witness = Phase41K3ProcessedRegistryLoadWitness::unprocessed();

    Phase41K3ProcessedRegistryAccountLoadingResult {
        status: Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountUnprocessed,
        rejection_case: None,
        account_data_len,
        canonical_event_key: None,
        route_id: None,
        recipient: None,
        consumed_amount: None,
        consumed_slot: None,
        account_key: None,
        expected_account_key: None,
        account_owner: None,
        expected_program_id: None,
        pda_bump: None,
        account_info_used: false,
        account_key_checked: false,
        account_owner_checked: false,
        pda_checked: false,
        discriminator_checked: false,
        zero_discriminator_rejected: false,
        wrong_discriminator_rejected: false,
        schema_version_checked: false,
        canonical_event_key_checked: false,
        route_id_checked: false,
        recipient_checked: false,
        consumed_checked: false,
        processed_event_account_writable: false,
        processed_event_account_non_signer: false,
        processed_event_account_non_executable: false,
        system_owned_empty_data_unprocessed: true,
        lamports_ignored_for_uninitialized_classification: true,
        total_fail_closed_classification: true,
        source_marker_authoritative_runtime_account: true,
        authoritative_witness_constructed: true,
        authoritative_view_witness: Some(witness),
        processed_registry_runtime_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    }
}

fn processed(
    account_data_len: usize,
    canonical_event_key: [u8; 32],
    route_id: [u8; 32],
    recipient: [u8; 32],
    consumed_amount: u128,
    consumed_slot: u64,
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    let witness = Phase41K3ProcessedRegistryLoadWitness::processed(canonical_event_key);

    Phase41K3ProcessedRegistryAccountLoadingResult {
        status: Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountProcessed,
        rejection_case: None,
        account_data_len,
        canonical_event_key: Some(canonical_event_key),
        route_id: Some(route_id),
        recipient: Some(recipient),
        consumed_amount: Some(consumed_amount),
        consumed_slot: Some(consumed_slot),
        account_key: None,
        expected_account_key: None,
        account_owner: None,
        expected_program_id: None,
        pda_bump: None,
        account_info_used: false,
        account_key_checked: false,
        account_owner_checked: false,
        pda_checked: false,
        discriminator_checked: true,
        zero_discriminator_rejected: true,
        wrong_discriminator_rejected: true,
        schema_version_checked: true,
        canonical_event_key_checked: true,
        route_id_checked: true,
        recipient_checked: true,
        consumed_checked: true,
        processed_event_account_writable: false,
        processed_event_account_non_signer: false,
        processed_event_account_non_executable: false,
        system_owned_empty_data_unprocessed: false,
        lamports_ignored_for_uninitialized_classification: true,
        total_fail_closed_classification: true,
        source_marker_authoritative_runtime_account: true,
        authoritative_witness_constructed: true,
        authoritative_view_witness: Some(witness),
        processed_registry_runtime_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    }
}

fn rejected(
    account_data_len: usize,
    rejection_case: Phase41K3ProcessedRegistryAccountRejectionCase,
    canonical_event_key: Option<[u8; 32]>,
    route_id: Option<[u8; 32]>,
    recipient: Option<[u8; 32]>,
    consumed_amount: Option<u128>,
    consumed_slot: Option<u64>,
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    let (
        discriminator_checked,
        zero_discriminator_rejected,
        wrong_discriminator_rejected,
        schema_version_checked,
        canonical_event_key_checked,
        route_id_checked,
        recipient_checked,
        consumed_checked,
    ) = rejected_decode_check_flags(
        rejection_case,
        canonical_event_key,
        route_id,
        recipient,
        consumed_amount,
        consumed_slot,
    );

    Phase41K3ProcessedRegistryAccountLoadingResult {
        status: Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountRejected,
        rejection_case: Some(rejection_case),
        account_data_len,
        canonical_event_key,
        route_id,
        recipient,
        consumed_amount,
        consumed_slot,
        account_key: None,
        expected_account_key: None,
        account_owner: None,
        expected_program_id: None,
        pda_bump: None,
        account_info_used: false,
        account_key_checked: false,
        account_owner_checked: false,
        pda_checked: false,
        discriminator_checked,
        zero_discriminator_rejected,
        wrong_discriminator_rejected,
        schema_version_checked,
        canonical_event_key_checked,
        route_id_checked,
        recipient_checked,
        consumed_checked,
        processed_event_account_writable: false,
        processed_event_account_non_signer: false,
        processed_event_account_non_executable: false,
        system_owned_empty_data_unprocessed: false,
        lamports_ignored_for_uninitialized_classification: true,
        total_fail_closed_classification: true,
        source_marker_authoritative_runtime_account: false,
        authoritative_witness_constructed: false,
        authoritative_view_witness: None,
        processed_registry_runtime_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    }
}

fn rejected_decode_check_flags(
    rejection_case: Phase41K3ProcessedRegistryAccountRejectionCase,
    canonical_event_key: Option<[u8; 32]>,
    route_id: Option<[u8; 32]>,
    recipient: Option<[u8; 32]>,
    consumed_amount: Option<u128>,
    consumed_slot: Option<u64>,
) -> (bool, bool, bool, bool, bool, bool, bool, bool) {
    use Phase41K3ProcessedRegistryAccountRejectionCase::*;

    match rejection_case {
        MissingProcessedEventAccount
        | AccountDataBorrowFailed
        | ProcessedEventAccountIsSigner
        | ProcessedEventAccountIsExecutable
        | ProcessedEventAccountPdaMismatch
        | ProcessedEventAccountOwnerMismatch
        | SystemOwnedAccountWithNonzeroData
        | MissingDiscriminator
        | InvalidAccountDataLength => (false, false, false, false, false, false, false, false),

        ZeroDiscriminator => (true, true, false, false, false, false, false, false),
        WrongDiscriminator => (true, true, true, false, false, false, false, false),
        UnsupportedSchemaVersion => (true, true, true, true, false, false, false, false),
        CanonicalEventKeyMismatch => (true, true, true, true, true, false, false, false),
        RouteIdMismatch => (true, true, true, true, true, true, false, false),
        RecipientMismatch => (true, true, true, true, true, true, true, false),
        InitializedButUnconsumedProcessedEvent => (true, true, true, true, true, true, true, true),

        MalformedProcessedEventAccountData => (
            true,
            true,
            true,
            true,
            canonical_event_key.is_some(),
            route_id.is_some(),
            recipient.is_some(),
            consumed_amount.is_some() && consumed_slot.is_some(),
        ),
    }
}

fn with_missing_account_metadata(
    mut result: Phase41K3ProcessedRegistryAccountLoadingResult,
    expected_account_key: Pubkey,
    expected_program_id: Pubkey,
    pda_bump: u8,
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    result.account_info_used = true;
    result.expected_account_key = Some(expected_account_key);
    result.expected_program_id = Some(expected_program_id);
    result.pda_bump = Some(pda_bump);
    result.processed_registry_runtime_loading_enabled = true;
    result
}

fn with_account_metadata(
    mut result: Phase41K3ProcessedRegistryAccountLoadingResult,
    account: &AccountInfo<'_>,
    expected_account_key: Pubkey,
    expected_program_id: Pubkey,
    pda_bump: u8,
    account_key_checked: bool,
    account_owner_checked: bool,
    pda_checked: bool,
) -> Phase41K3ProcessedRegistryAccountLoadingResult {
    result.account_info_used = true;
    result.account_key = Some(*account.key);
    result.expected_account_key = Some(expected_account_key);
    result.account_owner = Some(*account.owner);
    result.expected_program_id = Some(expected_program_id);
    result.pda_bump = Some(pda_bump);
    result.account_key_checked = account_key_checked;
    result.account_owner_checked = account_owner_checked;
    result.pda_checked = pda_checked;
    result.processed_event_account_writable = account.is_writable;
    result.processed_event_account_non_signer = !account.is_signer;
    result.processed_event_account_non_executable = !account.executable;
    result.processed_registry_runtime_loading_enabled = true;
    result
}

fn read_u8(input: &[u8], offset: usize) -> Option<u8> {
    input.get(offset).copied()
}

fn read_u16_le(input: &[u8], offset: usize) -> Option<u16> {
    let bytes = read_fixed::<2>(input, offset)?;
    Some(u16::from_le_bytes(bytes))
}

fn read_u64_le(input: &[u8], offset: usize) -> Option<u64> {
    let bytes = read_fixed::<8>(input, offset)?;
    Some(u64::from_le_bytes(bytes))
}

fn read_u128_le(input: &[u8], offset: usize) -> Option<u128> {
    let bytes = read_fixed::<16>(input, offset)?;
    Some(u128::from_le_bytes(bytes))
}

fn read_fixed<const N: usize>(input: &[u8], offset: usize) -> Option<[u8; N]> {
    let end = offset.checked_add(N)?;
    let slice = input.get(offset..end)?;

    let mut output = [0u8; N];
    output.copy_from_slice(slice);
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::{account_info::AccountInfo, clock::Epoch};

    const CANONICAL_EVENT_KEY: [u8; 32] = [0x44; 32];
    const OTHER_CANONICAL_EVENT_KEY: [u8; 32] = [0x45; 32];
    const ROUTE_ID: [u8; 32] = [0x11; 32];
    const OTHER_ROUTE_ID: [u8; 32] = [0x12; 32];
    const RECIPIENT: [u8; 32] = [0x55; 32];
    const OTHER_RECIPIENT: [u8; 32] = [0x56; 32];

    #[test]
    fn boundary_report_enables_only_processed_registry_runtime_loading() {
        let report = phase_41k_3_processed_registry_account_loading_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41K_3_PROCESSED_REGISTRY_ACCOUNT_LOADING_BOUNDARY_PHASE
        );
        assert_eq!(
            report.runtime_surface,
            "processed_registry_accountinfo_pda_loading"
        );
        assert!(report.pure_account_data_decoder_enabled);
        assert!(report.uses_account_info);
        assert!(report.checks_account_key);
        assert!(report.checks_account_owner);
        assert!(report.checks_pda);
        assert!(report.fixed_processed_event_pda_seed_format);
        assert!(report.uses_canonical_find_program_address_bump);
        assert!(report.ignores_caller_supplied_bump);
        assert!(report.allows_writable_processed_event_account);
        assert!(report.rejects_signer);
        assert!(report.rejects_executable);
        assert!(report.accepts_system_owned_empty_data_as_unprocessed);
        assert!(report.ignores_lamports_for_uninitialized_classification);
        assert!(report.rejects_system_owned_nonzero_data);
        assert!(report.checks_discriminator);
        assert!(report.rejects_zero_discriminator);
        assert!(report.rejects_wrong_discriminator);
        assert!(report.checks_schema_version);
        assert!(report.checks_canonical_event_key);
        assert!(report.checks_route_id);
        assert!(report.checks_recipient);
        assert!(report.rejects_initialized_unconsumed);
        assert!(report.total_fail_closed_classification);
        assert!(report.constructs_authoritative_witness);
        assert!(report.adapts_to_41j_list_based_view);
        assert!(report.processed_registry_runtime_loading_enabled);
        assert!(!report.replay_write_enabled);
        assert!(!report.processed_event_marking_enabled);
        assert!(!report.account_mutation_enabled);
        assert!(!report.cpi_enabled);
        assert!(!report.invoke_signed_enabled);
        assert!(!report.spl_token_mint_to_enabled);
        assert!(!report.process_instruction_handler_added);
        assert!(!report.live_route_enabled);
    }

    #[test]
    fn processed_event_pda_uses_exact_canonical_seed_format() {
        let program_id = Pubkey::new_unique();

        let (pda, bump) =
            find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);

        let (expected_pda, expected_bump) = Pubkey::find_program_address(
            &[
                b"xxxl",
                b"processed-event",
                CANONICAL_EVENT_KEY.as_ref(),
            ],
            &program_id,
        );

        assert_eq!(pda, expected_pda);
        assert_eq!(bump, expected_bump);
    }

    #[test]
    fn missing_account_info_rejects() {
        let program_id = Pubkey::new_unique();

        let result = load_phase_41k_3_processed_registry_account_info(
            None,
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::MissingProcessedEventAccount,
        );
        assert!(result.account_info_used);
        assert!(result.expected_account_key.is_some());
        assert_eq!(result.expected_program_id, Some(program_id));
        assert!(result.pda_bump.is_some());
        assert!(!result.authoritative_witness_constructed);
    }

    #[test]
    fn wrong_pda_rejects_before_data_trust() {
        let program_id = Pubkey::new_unique();
        let wrong_key = Pubkey::new_unique();
        let mut lamports = 1;
        let mut data = valid_processed_event_account_data(1);
        let account = account_info(
            &wrong_key,
            false,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountPdaMismatch,
        );
        assert!(result.account_key_checked);
        assert!(result.pda_checked);
        assert!(!result.account_owner_checked);
    }

    #[test]
    fn signer_rejects_in_all_states() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let mut lamports = 1;
        let mut data = valid_processed_event_account_data(1);
        let account = account_info(
            &pda,
            true,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountIsSigner,
        );
        assert!(!result.processed_event_account_non_signer);
    }

    #[test]
    fn executable_rejects_in_all_states() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let mut lamports = 1;
        let mut data = valid_processed_event_account_data(1);
        let account = account_info(
            &pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            true,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountIsExecutable,
        );
        assert!(!result.processed_event_account_non_executable);
    }

    #[test]
    fn system_owned_empty_data_zero_lamports_is_unprocessed() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let system_owner = Pubkey::from([0u8; 32]);
        let mut lamports = 0;
        let mut data = Vec::new();
        let account = account_info(
            &pda,
            false,
            false,
            &mut lamports,
            data.as_mut_slice(),
            &system_owner,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountUnprocessed
        );
        assert_eq!(result.rejection_case, None);
        assert!(result.system_owned_empty_data_unprocessed);
        assert!(result.lamports_ignored_for_uninitialized_classification);
        assert!(result.authoritative_witness_constructed);

        let witness = result
            .authoritative_view_witness
            .as_ref()
            .expect("unprocessed witness");
        let view = witness.to_authoritative_processed_registry_view();

        assert!(view.processed_canonical_event_keys().is_empty());
    }

    #[test]
    fn system_owned_empty_data_nonzero_lamports_is_still_unprocessed() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let system_owner = Pubkey::from([0u8; 32]);
        let mut lamports = 500;
        let mut data = Vec::new();
        let account = account_info(
            &pda,
            false,
            true,
            &mut lamports,
            data.as_mut_slice(),
            &system_owner,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountUnprocessed
        );
        assert!(result.processed_event_account_writable);
        assert!(result.authoritative_witness_constructed);
    }

    #[test]
    fn system_owned_nonzero_data_rejects() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let system_owner = Pubkey::from([0u8; 32]);
        let mut lamports = 1;
        let mut data = vec![0u8; 1];
        let account = account_info(
            &pda,
            false,
            false,
            &mut lamports,
            data.as_mut_slice(),
            &system_owner,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::SystemOwnedAccountWithNonzeroData,
        );
    }

    #[test]
    fn wrong_owner_rejects() {
        let program_id = Pubkey::new_unique();
        let wrong_owner = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let mut lamports = 1;
        let mut data = valid_processed_event_account_data(1);
        let account = account_info(
            &pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &wrong_owner,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::ProcessedEventAccountOwnerMismatch,
        );
    }

    #[test]
    fn zero_discriminator_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&[0u8; ACCOUNT_DISCRIMINATOR_LEN]);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::ZeroDiscriminator,
        );
    }

    #[test]
    fn wrong_discriminator_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[0] ^= 0xff;

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::WrongDiscriminator,
        );
    }

    #[test]
    fn unsupported_version_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[8..10].copy_from_slice(&2u16.to_le_bytes());

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::UnsupportedSchemaVersion,
        );
    }

    #[test]
    fn initialized_unconsumed_rejects() {
        let data = valid_processed_event_account_data(0);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::InitializedButUnconsumedProcessedEvent,
        );
        assert!(result.authoritative_view_witness.is_none());
    }

    #[test]
    fn invalid_consumed_flags_reject_fail_closed() {
        for consumed_flag in [2u8, 0xff] {
            let data = valid_processed_event_account_data(consumed_flag);

            let result = decode_phase_41k_3_processed_event_account_data(
                &data,
                &CANONICAL_EVENT_KEY,
                &ROUTE_ID,
                &RECIPIENT,
            );

            assert_rejected(
                &result,
                Phase41K3ProcessedRegistryAccountRejectionCase::MalformedProcessedEventAccountData,
            );
            assert!(result.authoritative_view_witness.is_none());
        }
    }

    #[test]
    fn initialized_consumed_classifies_processed_and_adapts_to_41j_one_item_list() {
        let data = valid_processed_event_account_data(1);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountProcessed
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.canonical_event_key, Some(CANONICAL_EVENT_KEY));
        assert_eq!(result.route_id, Some(ROUTE_ID));
        assert_eq!(result.recipient, Some(RECIPIENT));
        assert_eq!(result.consumed_amount, Some(1_000));
        assert_eq!(result.consumed_slot, Some(77));
        assert!(result.authoritative_witness_constructed);

        let witness = result
            .authoritative_view_witness
            .as_ref()
            .expect("processed witness");
        let view = witness.to_authoritative_processed_registry_view();

        assert_eq!(view.processed_canonical_event_keys(), &[CANONICAL_EVENT_KEY]);
    }

    #[test]
    fn canonical_event_key_mismatch_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET
            ..PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET + 32]
            .copy_from_slice(&OTHER_CANONICAL_EVENT_KEY);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::CanonicalEventKeyMismatch,
        );
    }

    #[test]
    fn route_id_mismatch_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[PROCESSED_EVENT_ROUTE_ID_OFFSET..PROCESSED_EVENT_ROUTE_ID_OFFSET + 32]
            .copy_from_slice(&OTHER_ROUTE_ID);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::RouteIdMismatch,
        );
    }

    #[test]
    fn recipient_mismatch_rejects() {
        let mut data = valid_processed_event_account_data(1);
        data[PROCESSED_EVENT_RECIPIENT_OFFSET..PROCESSED_EVENT_RECIPIENT_OFFSET + 32]
            .copy_from_slice(&OTHER_RECIPIENT);

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::RecipientMismatch,
        );
    }

    #[test]
    fn writable_processed_event_account_is_allowed_but_not_mutated() {
        let program_id = Pubkey::new_unique();
        let (pda, _) = find_phase_41k_3_processed_event_pda(&program_id, &CANONICAL_EVENT_KEY);
        let mut lamports = 1;
        let mut data = valid_processed_event_account_data(1);
        let before = data.clone();

        let account = account_info(
            &pda,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
        );

        let result = load_phase_41k_3_processed_registry_account_info(
            Some(&account),
            &program_id,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountProcessed
        );
        assert!(result.processed_event_account_writable);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert_eq!(data, before);
    }

    #[test]
    fn truncated_data_rejects_without_panic() {
        let data = vec![0u8; PROCESSED_EVENT_ACCOUNT_LEN - 1];

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::InvalidAccountDataLength,
        );
    }

    #[test]
    fn malformed_short_discriminator_rejects_without_panic() {
        let data = vec![0u8; ACCOUNT_DISCRIMINATOR_LEN - 1];

        let result = decode_phase_41k_3_processed_event_account_data(
            &data,
            &CANONICAL_EVENT_KEY,
            &ROUTE_ID,
            &RECIPIENT,
        );

        assert_rejected(
            &result,
            Phase41K3ProcessedRegistryAccountRejectionCase::MissingDiscriminator,
        );
    }

    fn valid_processed_event_account_data(consumed_flag: u8) -> Vec<u8> {
        let mut data = vec![0u8; PROCESSED_EVENT_ACCOUNT_LEN];
        data[0..ACCOUNT_DISCRIMINATOR_LEN]
            .copy_from_slice(&PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR);
        data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
        data[PROCESSED_EVENT_CONSUMED_OFFSET] = consumed_flag;
        data[PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET
            ..PROCESSED_EVENT_CANONICAL_EVENT_KEY_OFFSET + 32]
            .copy_from_slice(&CANONICAL_EVENT_KEY);
        data[PROCESSED_EVENT_ROUTE_ID_OFFSET..PROCESSED_EVENT_ROUTE_ID_OFFSET + 32]
            .copy_from_slice(&ROUTE_ID);
        data[PROCESSED_EVENT_RECIPIENT_OFFSET..PROCESSED_EVENT_RECIPIENT_OFFSET + 32]
            .copy_from_slice(&RECIPIENT);
        data[PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET..PROCESSED_EVENT_CONSUMED_AMOUNT_OFFSET + 16]
            .copy_from_slice(&1_000u128.to_le_bytes());
        data[PROCESSED_EVENT_CONSUMED_SLOT_OFFSET..PROCESSED_EVENT_CONSUMED_SLOT_OFFSET + 8]
            .copy_from_slice(&77u64.to_le_bytes());
        data
    }

    fn account_info<'a>(
        key: &'a Pubkey,
        is_signer: bool,
        is_writable: bool,
        lamports: &'a mut u64,
        data: &'a mut [u8],
        owner: &'a Pubkey,
        executable: bool,
    ) -> AccountInfo<'a> {
        AccountInfo::new(
            key,
            is_signer,
            is_writable,
            lamports,
            data,
            owner,
            executable,
            Epoch::default(),
        )
    }

    fn assert_rejected(
        result: &Phase41K3ProcessedRegistryAccountLoadingResult,
        rejection_case: Phase41K3ProcessedRegistryAccountRejectionCase,
    ) {
        assert_eq!(
            result.status,
            Phase41K3ProcessedRegistryAccountLoadingStatus::ProcessedEventAccountRejected
        );
        assert_eq!(result.rejection_case, Some(rejection_case));
        assert!(!result.authoritative_witness_constructed);
        assert!(result.authoritative_view_witness.is_none());
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }
}
