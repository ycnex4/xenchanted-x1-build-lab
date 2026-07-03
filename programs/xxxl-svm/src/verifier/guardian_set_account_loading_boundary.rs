use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

use crate::state::{
    ACCOUNT_DISCRIMINATOR_LEN, GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
    RUNTIME_LAYOUT_VERSION,
};

use super::guardian_quorum::GuardianPublicKey;

pub const PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_PHASE: &str = "41K.2";
pub const PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_VERSION: &str = "0.1.0";

pub const GUARDIAN_SET_ACTIVE_STATUS_OFFSET: usize = 10;
pub const GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET: usize = 12;
pub const GUARDIAN_SET_GUARDIAN_COUNT_OFFSET: usize = 14;
pub const GUARDIAN_SET_GUARDIAN_KEYS_OFFSET: usize = 16;
pub const GUARDIAN_SET_GUARDIAN_SET_ID_OFFSET: usize = 272;

pub const GUARDIAN_PUBLIC_KEY_LEN: usize = 32;
pub const MAX_SUPPORTED_GUARDIAN_COUNT: usize = 8;

pub const GUARDIAN_SET_PDA_SEED_0: &[u8] = b"xxxl";
pub const GUARDIAN_SET_PDA_SEED_1: &[u8] = b"guardian-set";

pub const GUARDIAN_SET_STATUS_ACTIVE: u8 = 1;
pub const GUARDIAN_SET_STATUS_INACTIVE: u8 = 2;
pub const GUARDIAN_SET_STATUS_DEPRECATED: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41K2GuardianSetAccountLoadingStatus {
    GuardianSetAccountDataDecoded,
    GuardianSetAccountDataRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase41K2GuardianSetAccountRejectionCase {
    MissingGuardianSetAccount,
    AccountDataBorrowFailed,
    GuardianSetAccountIsSigner,
    GuardianSetAccountIsWritable,
    GuardianSetAccountOwnerMismatch,
    GuardianSetAccountPdaMismatch,
    MissingDiscriminator,
    InvalidAccountDataLength,
    ZeroDiscriminator,
    WrongDiscriminator,
    UnsupportedSchemaVersion,
    InactiveOrDeprecatedGuardianSet,
    InvalidThresholdZero,
    GuardianCountZero,
    GuardianCountExceedsMaxSupported,
    ThresholdExceedsGuardianCount,
    GuardianSetIdMismatch,
    DuplicateGuardianPublicKey,
    MalformedGuardianPublicKeyBytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Phase41K2GuardianSetAccountLoadingResult {
    pub status: Phase41K2GuardianSetAccountLoadingStatus,
    pub rejection_case: Option<Phase41K2GuardianSetAccountRejectionCase>,
    pub account_data_len: usize,
    pub guardian_set_id: Option<[u8; 32]>,
    pub threshold: Option<u8>,
    pub guardian_count: usize,
    pub guardians: Vec<GuardianPublicKey>,
    pub account_key: Option<Pubkey>,
    pub expected_account_key: Option<Pubkey>,
    pub account_owner: Option<Pubkey>,
    pub expected_program_id: Option<Pubkey>,
    pub pda_bump: Option<u8>,
    pub active: bool,
    pub discriminator_checked: bool,
    pub zero_discriminator_rejected: bool,
    pub wrong_discriminator_rejected: bool,
    pub schema_version_checked: bool,
    pub threshold_checked: bool,
    pub guardian_count_checked: bool,
    pub duplicate_guardian_public_key_rejected: bool,
    pub guardian_set_id_checked: bool,
    pub active_status_checked: bool,
    pub guardian_set_account_readonly: bool,
    pub guardian_set_account_non_signer: bool,
    pub source_marker_program_controlled_on_chain: bool,
    pub caller_supplied_guardian_set_rejected: bool,
    pub account_info_used: bool,
    pub account_key_checked: bool,
    pub account_owner_checked: bool,
    pub pda_checked: bool,
    pub authoritative_wrapper_constructed: bool,
    pub guardian_set_runtime_loading_enabled: bool,
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
pub struct Phase41K2GuardianSetAccountLoadingBoundaryReport {
    pub phase: &'static str,
    pub version: &'static str,
    pub runtime_surface: &'static str,
    pub pure_account_data_decoder_enabled: bool,
    pub uses_account_info: bool,
    pub checks_account_key: bool,
    pub checks_account_owner: bool,
    pub checks_pda: bool,
    pub checks_guardian_set_account_readonly: bool,
    pub checks_guardian_set_account_non_signer: bool,
    pub fixed_guardian_set_pda_seed_format: bool,
    pub checks_discriminator: bool,
    pub rejects_zero_discriminator: bool,
    pub rejects_wrong_discriminator: bool,
    pub checks_schema_version: bool,
    pub checks_guardian_set_id: bool,
    pub checks_active_status: bool,
    pub checks_threshold: bool,
    pub checks_guardian_count: bool,
    pub rejects_empty_guardian_set: bool,
    pub rejects_guardian_count_above_max: bool,
    pub rejects_threshold_zero: bool,
    pub rejects_threshold_above_guardian_count: bool,
    pub rejects_duplicate_guardian_public_key: bool,
    pub constructs_authoritative_wrapper: bool,
    pub accepts_caller_supplied_guardian_list: bool,
    pub accepts_frontend_or_watcher_guardian_list: bool,
    pub guardian_set_runtime_loading_enabled: bool,
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

pub const PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_REPORT:
    Phase41K2GuardianSetAccountLoadingBoundaryReport =
    Phase41K2GuardianSetAccountLoadingBoundaryReport {
        phase: PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_PHASE,
        version: PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_VERSION,
        runtime_surface: "guardian_set_accountinfo_pda_loading",
        pure_account_data_decoder_enabled: true,
        uses_account_info: true,
        checks_account_key: true,
        checks_account_owner: true,
        checks_pda: true,
        checks_guardian_set_account_readonly: true,
        checks_guardian_set_account_non_signer: true,
        fixed_guardian_set_pda_seed_format: true,
        checks_discriminator: true,
        rejects_zero_discriminator: true,
        rejects_wrong_discriminator: true,
        checks_schema_version: true,
        checks_guardian_set_id: true,
        checks_active_status: true,
        checks_threshold: true,
        checks_guardian_count: true,
        rejects_empty_guardian_set: true,
        rejects_guardian_count_above_max: true,
        rejects_threshold_zero: true,
        rejects_threshold_above_guardian_count: true,
        rejects_duplicate_guardian_public_key: true,
        constructs_authoritative_wrapper: false,
        accepts_caller_supplied_guardian_list: false,
        accepts_frontend_or_watcher_guardian_list: false,
        guardian_set_runtime_loading_enabled: true,
        processed_registry_runtime_loading_enabled: false,
        replay_write_enabled: false,
        processed_event_marking_enabled: false,
        account_mutation_enabled: false,
        cpi_enabled: false,
        invoke_signed_enabled: false,
        spl_token_mint_to_enabled: false,
        process_instruction_handler_added: false,
        live_route_enabled: false,
    };

pub fn phase_41k_2_guardian_set_account_loading_boundary_report(
) -> Phase41K2GuardianSetAccountLoadingBoundaryReport {
    PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_REPORT
}

pub fn find_phase_41k_2_guardian_set_pda(
    program_id: &Pubkey,
    guardian_set_id: &[u8; 32],
) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            GUARDIAN_SET_PDA_SEED_0,
            GUARDIAN_SET_PDA_SEED_1,
            guardian_set_id,
        ],
        program_id,
    )
}

pub fn load_phase_41k_2_guardian_set_account_info(
    guardian_set_account: Option<&AccountInfo<'_>>,
    expected_program_id: &Pubkey,
    expected_guardian_set_id: &[u8; 32],
) -> Phase41K2GuardianSetAccountLoadingResult {
    let (expected_pda, pda_bump) =
        find_phase_41k_2_guardian_set_pda(expected_program_id, expected_guardian_set_id);

    let Some(guardian_set_account) = guardian_set_account else {
        return with_missing_account_metadata(
            rejected(
                0,
                Phase41K2GuardianSetAccountRejectionCase::MissingGuardianSetAccount,
                None,
                None,
                0,
                Vec::new(),
                false,
            ),
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    };

    if guardian_set_account.is_signer {
        return with_account_metadata(
            rejected(
                guardian_set_account.data_len(),
                Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountIsSigner,
                None,
                None,
                0,
                Vec::new(),
                false,
            ),
            guardian_set_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    }

    if guardian_set_account.is_writable {
        return with_account_metadata(
            rejected(
                guardian_set_account.data_len(),
                Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountIsWritable,
                None,
                None,
                0,
                Vec::new(),
                false,
            ),
            guardian_set_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    }

    if guardian_set_account.owner != expected_program_id {
        return with_account_metadata(
            rejected(
                guardian_set_account.data_len(),
                Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountOwnerMismatch,
                None,
                None,
                0,
                Vec::new(),
                false,
            ),
            guardian_set_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    }

    if *guardian_set_account.key != expected_pda {
        return with_account_metadata(
            rejected(
                guardian_set_account.data_len(),
                Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountPdaMismatch,
                None,
                None,
                0,
                Vec::new(),
                false,
            ),
            guardian_set_account,
            expected_pda,
            *expected_program_id,
            pda_bump,
        );
    }

    let data = match guardian_set_account.try_borrow_data() {
        Ok(data) => data,
        Err(_) => {
            return with_account_metadata(
                rejected(
                    guardian_set_account.data_len(),
                    Phase41K2GuardianSetAccountRejectionCase::AccountDataBorrowFailed,
                    None,
                    None,
                    0,
                    Vec::new(),
                    false,
                ),
                guardian_set_account,
                expected_pda,
                *expected_program_id,
                pda_bump,
            );
        }
    };

    let result =
        decode_phase_41k_2_guardian_set_account_data(data.as_ref(), expected_guardian_set_id);

    with_account_metadata(
        result,
        guardian_set_account,
        expected_pda,
        *expected_program_id,
        pda_bump,
    )
}

pub fn decode_phase_41k_2_guardian_set_account_data(
    data: &[u8],
    expected_guardian_set_id: &[u8; 32],
) -> Phase41K2GuardianSetAccountLoadingResult {
    if data.len() < ACCOUNT_DISCRIMINATOR_LEN {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::MissingDiscriminator,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    if data.len() != GUARDIAN_SET_ACCOUNT_LEN {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::InvalidAccountDataLength,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    let Some(discriminator) = read_fixed::<ACCOUNT_DISCRIMINATOR_LEN>(data, 0) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::MissingDiscriminator,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    };

    if discriminator == [0u8; ACCOUNT_DISCRIMINATOR_LEN] {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::ZeroDiscriminator,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    if discriminator != GUARDIAN_SET_ACCOUNT_DISCRIMINATOR {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::WrongDiscriminator,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    let Some(version) = read_u16_le(data, 8) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::UnsupportedSchemaVersion,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    };

    if version != RUNTIME_LAYOUT_VERSION {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::UnsupportedSchemaVersion,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    let Some(active_status) = read_u8(data, GUARDIAN_SET_ACTIVE_STATUS_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    };

    if active_status != GUARDIAN_SET_STATUS_ACTIVE {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet,
            None,
            None,
            0,
            Vec::new(),
            false,
        );
    }

    let Some(threshold_raw) = read_u16_le(data, GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::InvalidThresholdZero,
            None,
            None,
            0,
            Vec::new(),
            true,
        );
    };

    let Some(guardian_count_raw) = read_u8(data, GUARDIAN_SET_GUARDIAN_COUNT_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::GuardianCountZero,
            None,
            None,
            0,
            Vec::new(),
            true,
        );
    };

    let guardian_count = usize::from(guardian_count_raw);

    if threshold_raw == 0 {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::InvalidThresholdZero,
            None,
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    }

    if guardian_count == 0 {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::GuardianCountZero,
            None,
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    }

    if guardian_count > MAX_SUPPORTED_GUARDIAN_COUNT {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::GuardianCountExceedsMaxSupported,
            None,
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    }

    if usize::from(threshold_raw) > guardian_count {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::ThresholdExceedsGuardianCount,
            None,
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    }

    let Some(guardian_set_id) = read_fixed::<32>(data, GUARDIAN_SET_GUARDIAN_SET_ID_OFFSET) else {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::GuardianSetIdMismatch,
            None,
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    };

    if &guardian_set_id != expected_guardian_set_id {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::GuardianSetIdMismatch,
            Some(guardian_set_id),
            None,
            guardian_count,
            Vec::new(),
            true,
        );
    }

    let mut guardians = Vec::with_capacity(guardian_count);

    for index in 0..guardian_count {
        let Some(key_offset) =
            GUARDIAN_SET_GUARDIAN_KEYS_OFFSET.checked_add(index * GUARDIAN_PUBLIC_KEY_LEN)
        else {
            return rejected(
                data.len(),
                Phase41K2GuardianSetAccountRejectionCase::MalformedGuardianPublicKeyBytes,
                Some(guardian_set_id),
                None,
                guardian_count,
                guardians,
                true,
            );
        };

        let Some(public_key) = read_fixed::<GUARDIAN_PUBLIC_KEY_LEN>(data, key_offset) else {
            return rejected(
                data.len(),
                Phase41K2GuardianSetAccountRejectionCase::MalformedGuardianPublicKeyBytes,
                Some(guardian_set_id),
                None,
                guardian_count,
                guardians,
                true,
            );
        };

        guardians.push(GuardianPublicKey(public_key));
    }

    if duplicate_guardian_public_key_index(&guardians).is_some() {
        return rejected(
            data.len(),
            Phase41K2GuardianSetAccountRejectionCase::DuplicateGuardianPublicKey,
            Some(guardian_set_id),
            None,
            guardian_count,
            guardians,
            true,
        );
    }

    decoded(
        data.len(),
        guardian_set_id,
        threshold_raw as u8,
        guardian_count,
        guardians,
    )
}

fn decoded(
    account_data_len: usize,
    guardian_set_id: [u8; 32],
    threshold: u8,
    guardian_count: usize,
    guardians: Vec<GuardianPublicKey>,
) -> Phase41K2GuardianSetAccountLoadingResult {
    Phase41K2GuardianSetAccountLoadingResult {
        status: Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded,
        rejection_case: None,
        account_data_len,
        guardian_set_id: Some(guardian_set_id),
        threshold: Some(threshold),
        guardian_count,
        guardians,
        account_key: None,
        expected_account_key: None,
        account_owner: None,
        expected_program_id: None,
        pda_bump: None,
        active: true,
        discriminator_checked: true,
        zero_discriminator_rejected: true,
        wrong_discriminator_rejected: true,
        schema_version_checked: true,
        threshold_checked: true,
        guardian_count_checked: true,
        duplicate_guardian_public_key_rejected: true,
        guardian_set_id_checked: true,
        active_status_checked: true,
        guardian_set_account_readonly: false,
        guardian_set_account_non_signer: false,
        source_marker_program_controlled_on_chain: false,
        caller_supplied_guardian_set_rejected: true,
        account_info_used: false,
        account_key_checked: false,
        account_owner_checked: false,
        pda_checked: false,
        authoritative_wrapper_constructed: false,
        guardian_set_runtime_loading_enabled: false,
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
    rejection_case: Phase41K2GuardianSetAccountRejectionCase,
    guardian_set_id: Option<[u8; 32]>,
    threshold: Option<u8>,
    guardian_count: usize,
    guardians: Vec<GuardianPublicKey>,
    active: bool,
) -> Phase41K2GuardianSetAccountLoadingResult {
    Phase41K2GuardianSetAccountLoadingResult {
        status: Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected,
        rejection_case: Some(rejection_case),
        account_data_len,
        guardian_set_id,
        threshold,
        guardian_count,
        guardians,
        account_key: None,
        expected_account_key: None,
        account_owner: None,
        expected_program_id: None,
        pda_bump: None,
        active,
        discriminator_checked: true,
        zero_discriminator_rejected: true,
        wrong_discriminator_rejected: true,
        schema_version_checked: true,
        threshold_checked: true,
        guardian_count_checked: true,
        duplicate_guardian_public_key_rejected: true,
        guardian_set_id_checked: true,
        active_status_checked: true,
        guardian_set_account_readonly: false,
        guardian_set_account_non_signer: false,
        source_marker_program_controlled_on_chain: false,
        caller_supplied_guardian_set_rejected: true,
        account_info_used: false,
        account_key_checked: false,
        account_owner_checked: false,
        pda_checked: false,
        authoritative_wrapper_constructed: false,
        guardian_set_runtime_loading_enabled: false,
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

fn with_missing_account_metadata(
    mut result: Phase41K2GuardianSetAccountLoadingResult,
    expected_account_key: Pubkey,
    expected_program_id: Pubkey,
    pda_bump: u8,
) -> Phase41K2GuardianSetAccountLoadingResult {
    result.account_info_used = true;
    result.expected_account_key = Some(expected_account_key);
    result.expected_program_id = Some(expected_program_id);
    result.pda_bump = Some(pda_bump);
    result.guardian_set_runtime_loading_enabled = true;
    result.caller_supplied_guardian_set_rejected = true;
    result
}

fn with_account_metadata(
    mut result: Phase41K2GuardianSetAccountLoadingResult,
    account: &AccountInfo<'_>,
    expected_account_key: Pubkey,
    expected_program_id: Pubkey,
    pda_bump: u8,
) -> Phase41K2GuardianSetAccountLoadingResult {
    result.account_info_used = true;
    result.account_key = Some(*account.key);
    result.expected_account_key = Some(expected_account_key);
    result.account_owner = Some(*account.owner);
    result.expected_program_id = Some(expected_program_id);
    result.pda_bump = Some(pda_bump);
    result.account_key_checked = true;
    result.account_owner_checked = true;
    result.pda_checked = true;
    result.guardian_set_account_readonly = !account.is_writable;
    result.guardian_set_account_non_signer = !account.is_signer;
    result.guardian_set_runtime_loading_enabled = true;
    result.caller_supplied_guardian_set_rejected = true;
    result.source_marker_program_controlled_on_chain =
        result.status == Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded;
    result
}

fn duplicate_guardian_public_key_index(guardians: &[GuardianPublicKey]) -> Option<usize> {
    for (index, guardian) in guardians.iter().enumerate() {
        if let Some(previous_guardians) = guardians.get(..index) {
            if previous_guardians
                .iter()
                .any(|previous_guardian| previous_guardian == guardian)
            {
                return Some(index);
            }
        }
    }

    None
}

fn read_u8(input: &[u8], offset: usize) -> Option<u8> {
    input.get(offset).copied()
}

fn read_u16_le(input: &[u8], offset: usize) -> Option<u16> {
    let bytes = read_fixed::<2>(input, offset)?;
    Some(u16::from_le_bytes(bytes))
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

    const GUARDIAN_SET_ID: [u8; 32] = [0x22; 32];
    const OTHER_GUARDIAN_SET_ID: [u8; 32] = [0x23; 32];

    #[test]
    fn boundary_report_is_decoder_only_and_keeps_downstream_surfaces_disabled() {
        let report = phase_41k_2_guardian_set_account_loading_boundary_report();

        assert_eq!(
            report.phase,
            PHASE_41K_2_GUARDIAN_SET_ACCOUNT_LOADING_BOUNDARY_PHASE
        );
        assert_eq!(
            report.runtime_surface,
            "guardian_set_accountinfo_pda_loading"
        );
        assert!(report.pure_account_data_decoder_enabled);
        assert!(report.checks_discriminator);
        assert!(report.rejects_zero_discriminator);
        assert!(report.checks_schema_version);
        assert!(report.checks_guardian_set_id);
        assert!(report.checks_active_status);
        assert!(report.checks_threshold);
        assert!(report.checks_guardian_count);
        assert!(report.rejects_duplicate_guardian_public_key);
        assert!(report.uses_account_info);
        assert!(report.checks_account_key);
        assert!(report.checks_account_owner);
        assert!(report.checks_pda);
        assert!(report.checks_guardian_set_account_readonly);
        assert!(report.checks_guardian_set_account_non_signer);
        assert!(report.fixed_guardian_set_pda_seed_format);
        assert!(!report.constructs_authoritative_wrapper);
        assert!(!report.accepts_caller_supplied_guardian_list);
        assert!(!report.accepts_frontend_or_watcher_guardian_list);
        assert!(report.guardian_set_runtime_loading_enabled);
        assert!(!report.processed_registry_runtime_loading_enabled);
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
    fn decoder_accepts_valid_guardian_set_account_data() {
        let data = valid_guardian_set_account_data();

        let result = decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID);

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.account_data_len, GUARDIAN_SET_ACCOUNT_LEN);
        assert_eq!(result.guardian_set_id, Some(GUARDIAN_SET_ID));
        assert_eq!(result.threshold, Some(2));
        assert_eq!(result.guardian_count, 3);
        assert_eq!(result.guardians.len(), 3);
        assert_eq!(result.guardians[0], GuardianPublicKey([0x31; 32]));
        assert_eq!(result.guardians[1], GuardianPublicKey([0x32; 32]));
        assert_eq!(result.guardians[2], GuardianPublicKey([0x33; 32]));
        assert!(result.active);
        assert!(!result.account_info_used);
        assert!(!result.account_key_checked);
        assert!(!result.account_owner_checked);
        assert!(!result.pda_checked);
        assert!(!result.authoritative_wrapper_constructed);
        assert!(!result.guardian_set_runtime_loading_enabled);
        assert!(!result.processed_registry_runtime_loading_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn loader_accepts_program_controlled_readonly_non_signer_guardian_set_pda() {
        let program_id = Pubkey::new_unique();
        let (guardian_set_pda, expected_bump) =
            find_phase_41k_2_guardian_set_pda(&program_id, &GUARDIAN_SET_ID);
        let mut data = valid_guardian_set_account_data();
        let mut lamports = 1;

        let account = AccountInfo::new(
            &guardian_set_pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataDecoded
        );
        assert_eq!(result.rejection_case, None);
        assert_eq!(result.account_key, Some(guardian_set_pda));
        assert_eq!(result.expected_account_key, Some(guardian_set_pda));
        assert_eq!(result.account_owner, Some(program_id));
        assert_eq!(result.expected_program_id, Some(program_id));
        assert_eq!(result.pda_bump, Some(expected_bump));
        assert_eq!(result.guardian_set_id, Some(GUARDIAN_SET_ID));
        assert_eq!(result.threshold, Some(2));
        assert_eq!(result.guardian_count, 3);
        assert!(result.account_info_used);
        assert!(result.account_key_checked);
        assert!(result.account_owner_checked);
        assert!(result.pda_checked);
        assert!(result.guardian_set_account_readonly);
        assert!(result.guardian_set_account_non_signer);
        assert!(result.source_marker_program_controlled_on_chain);
        assert!(result.caller_supplied_guardian_set_rejected);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.processed_registry_runtime_loading_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn loader_rejects_wrong_owner_before_data_trust() {
        let program_id = Pubkey::new_unique();
        let wrong_owner = Pubkey::new_unique();
        let (guardian_set_pda, _) =
            find_phase_41k_2_guardian_set_pda(&program_id, &GUARDIAN_SET_ID);
        let mut data = valid_guardian_set_account_data();
        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&[0u8; ACCOUNT_DISCRIMINATOR_LEN]);
        let mut lamports = 1;

        let account = AccountInfo::new(
            &guardian_set_pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &wrong_owner,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountOwnerMismatch)
        );
        assert_eq!(result.account_owner, Some(wrong_owner));
        assert_eq!(result.expected_program_id, Some(program_id));
        assert!(result.account_info_used);
        assert!(result.account_owner_checked);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.source_marker_program_controlled_on_chain);
    }

    #[test]
    fn loader_rejects_wrong_pda_before_data_trust() {
        let program_id = Pubkey::new_unique();
        let wrong_pda = Pubkey::new_unique();
        let mut data = valid_guardian_set_account_data();
        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&[0u8; ACCOUNT_DISCRIMINATOR_LEN]);
        let mut lamports = 1;

        let account = AccountInfo::new(
            &wrong_pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountPdaMismatch)
        );
        assert_eq!(result.account_key, Some(wrong_pda));
        assert_eq!(result.expected_program_id, Some(program_id));
        assert!(result.account_info_used);
        assert!(result.pda_checked);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.source_marker_program_controlled_on_chain);
    }

    #[test]
    fn loader_rejects_missing_guardian_set_account() {
        let program_id = Pubkey::new_unique();

        let result =
            load_phase_41k_2_guardian_set_account_info(None, &program_id, &GUARDIAN_SET_ID);

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::MissingGuardianSetAccount)
        );
        assert!(result.account_info_used);
        assert_eq!(result.expected_program_id, Some(program_id));
        assert!(result.expected_account_key.is_some());
        assert!(result.pda_bump.is_some());
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.processed_registry_runtime_loading_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn loader_rejects_signer_guardian_set_account() {
        let program_id = Pubkey::new_unique();
        let (guardian_set_pda, _) =
            find_phase_41k_2_guardian_set_pda(&program_id, &GUARDIAN_SET_ID);
        let mut data = valid_guardian_set_account_data();
        let mut lamports = 1;

        let account = AccountInfo::new(
            &guardian_set_pda,
            true,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountIsSigner)
        );
        assert!(result.account_info_used);
        assert_eq!(result.account_key, Some(guardian_set_pda));
        assert_eq!(result.account_owner, Some(program_id));
        assert!(!result.guardian_set_account_non_signer);
        assert!(result.guardian_set_account_readonly);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.source_marker_program_controlled_on_chain);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn loader_rejects_writable_guardian_set_account() {
        let program_id = Pubkey::new_unique();
        let (guardian_set_pda, _) =
            find_phase_41k_2_guardian_set_pda(&program_id, &GUARDIAN_SET_ID);
        let mut data = valid_guardian_set_account_data();
        let mut lamports = 1;

        let account = AccountInfo::new(
            &guardian_set_pda,
            false,
            true,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::GuardianSetAccountIsWritable)
        );
        assert!(result.account_info_used);
        assert_eq!(result.account_key, Some(guardian_set_pda));
        assert_eq!(result.account_owner, Some(program_id));
        assert!(result.guardian_set_account_non_signer);
        assert!(!result.guardian_set_account_readonly);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.source_marker_program_controlled_on_chain);
        assert!(!result.cpi_enabled);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn loader_rejects_checked_account_data_without_enabling_downstream_surfaces() {
        let program_id = Pubkey::new_unique();
        let (guardian_set_pda, _) =
            find_phase_41k_2_guardian_set_pda(&program_id, &GUARDIAN_SET_ID);
        let mut data = valid_guardian_set_account_data();
        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&[0u8; ACCOUNT_DISCRIMINATOR_LEN]);
        let mut lamports = 1;

        let account = AccountInfo::new(
            &guardian_set_pda,
            false,
            false,
            &mut lamports,
            &mut data,
            &program_id,
            false,
            0,
        );

        let result = load_phase_41k_2_guardian_set_account_info(
            Some(&account),
            &program_id,
            &GUARDIAN_SET_ID,
        );

        assert_eq!(
            result.rejection_case,
            Some(Phase41K2GuardianSetAccountRejectionCase::ZeroDiscriminator)
        );
        assert!(result.account_info_used);
        assert!(result.account_key_checked);
        assert!(result.account_owner_checked);
        assert!(result.pda_checked);
        assert!(result.guardian_set_runtime_loading_enabled);
        assert!(!result.source_marker_program_controlled_on_chain);
        assert!(!result.processed_registry_runtime_loading_enabled);
        assert!(!result.replay_write_enabled);
        assert!(!result.processed_event_marking_enabled);
        assert!(!result.account_mutation_enabled);
        assert!(!result.cpi_enabled);
        assert!(!result.invoke_signed_enabled);
        assert!(!result.spl_token_mint_to_enabled);
        assert!(!result.process_instruction_handler_added);
        assert!(!result.live_route_enabled);
    }

    #[test]
    fn decoder_rejects_missing_discriminator() {
        let data = vec![0u8; ACCOUNT_DISCRIMINATOR_LEN - 1];

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::MissingDiscriminator,
        );
    }

    #[test]
    fn decoder_rejects_invalid_account_data_length() {
        let data = vec![0u8; GUARDIAN_SET_ACCOUNT_LEN - 1];

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::InvalidAccountDataLength,
        );
    }

    #[test]
    fn decoder_rejects_zero_discriminator() {
        let mut data = valid_guardian_set_account_data();
        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&[0u8; ACCOUNT_DISCRIMINATOR_LEN]);

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::ZeroDiscriminator,
        );
    }

    #[test]
    fn decoder_rejects_wrong_discriminator() {
        let mut data = valid_guardian_set_account_data();
        data[0] ^= 0xff;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::WrongDiscriminator,
        );
    }

    #[test]
    fn decoder_rejects_unsupported_schema_version() {
        let mut data = valid_guardian_set_account_data();
        data[8..10].copy_from_slice(&2u16.to_le_bytes());

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::UnsupportedSchemaVersion,
        );
    }

    #[test]
    fn decoder_rejects_inactive_guardian_set() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_ACTIVE_STATUS_OFFSET] = GUARDIAN_SET_STATUS_INACTIVE;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet,
        );
    }

    #[test]
    fn decoder_rejects_deprecated_guardian_set() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_ACTIVE_STATUS_OFFSET] = GUARDIAN_SET_STATUS_DEPRECATED;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::InactiveOrDeprecatedGuardianSet,
        );
    }

    #[test]
    fn decoder_rejects_threshold_zero() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET..GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET + 2]
            .copy_from_slice(&0u16.to_le_bytes());

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::InvalidThresholdZero,
        );
    }

    #[test]
    fn decoder_rejects_guardian_count_zero() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_GUARDIAN_COUNT_OFFSET] = 0;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::GuardianCountZero,
        );
    }

    #[test]
    fn decoder_rejects_guardian_count_above_max_supported() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_GUARDIAN_COUNT_OFFSET] = (MAX_SUPPORTED_GUARDIAN_COUNT + 1) as u8;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::GuardianCountExceedsMaxSupported,
        );
    }

    #[test]
    fn decoder_rejects_threshold_above_guardian_count() {
        let mut data = valid_guardian_set_account_data();
        data[GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET..GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET + 2]
            .copy_from_slice(&4u16.to_le_bytes());
        data[GUARDIAN_SET_GUARDIAN_COUNT_OFFSET] = 3;

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::ThresholdExceedsGuardianCount,
        );
    }

    #[test]
    fn decoder_rejects_guardian_set_id_mismatch() {
        let data = valid_guardian_set_account_data();

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &OTHER_GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::GuardianSetIdMismatch,
        );
    }

    #[test]
    fn decoder_rejects_duplicate_guardian_public_key() {
        let mut data = valid_guardian_set_account_data();

        let first_key_start = GUARDIAN_SET_GUARDIAN_KEYS_OFFSET;
        let second_key_start = GUARDIAN_SET_GUARDIAN_KEYS_OFFSET + GUARDIAN_PUBLIC_KEY_LEN;
        let first_key = data[first_key_start..first_key_start + GUARDIAN_PUBLIC_KEY_LEN].to_vec();
        data[second_key_start..second_key_start + GUARDIAN_PUBLIC_KEY_LEN]
            .copy_from_slice(&first_key);

        assert_rejection(
            decode_phase_41k_2_guardian_set_account_data(&data, &GUARDIAN_SET_ID),
            Phase41K2GuardianSetAccountRejectionCase::DuplicateGuardianPublicKey,
        );
    }

    fn valid_guardian_set_account_data() -> Vec<u8> {
        let mut data = vec![0u8; GUARDIAN_SET_ACCOUNT_LEN];

        data[0..ACCOUNT_DISCRIMINATOR_LEN].copy_from_slice(&GUARDIAN_SET_ACCOUNT_DISCRIMINATOR);
        data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
        data[GUARDIAN_SET_ACTIVE_STATUS_OFFSET] = GUARDIAN_SET_STATUS_ACTIVE;
        data[GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET..GUARDIAN_SET_QUORUM_THRESHOLD_OFFSET + 2]
            .copy_from_slice(&2u16.to_le_bytes());
        data[GUARDIAN_SET_GUARDIAN_COUNT_OFFSET] = 3;

        write_guardian_key(&mut data, 0, [0x31; 32]);
        write_guardian_key(&mut data, 1, [0x32; 32]);
        write_guardian_key(&mut data, 2, [0x33; 32]);

        data[GUARDIAN_SET_GUARDIAN_SET_ID_OFFSET..GUARDIAN_SET_GUARDIAN_SET_ID_OFFSET + 32]
            .copy_from_slice(&GUARDIAN_SET_ID);

        data
    }

    fn write_guardian_key(data: &mut [u8], index: usize, key: [u8; 32]) {
        let start = GUARDIAN_SET_GUARDIAN_KEYS_OFFSET + index * GUARDIAN_PUBLIC_KEY_LEN;
        let end = start + GUARDIAN_PUBLIC_KEY_LEN;
        data[start..end].copy_from_slice(&key);
    }

    fn assert_rejection(
        result: Phase41K2GuardianSetAccountLoadingResult,
        expected_rejection_case: Phase41K2GuardianSetAccountRejectionCase,
    ) {
        assert_eq!(
            result.status,
            Phase41K2GuardianSetAccountLoadingStatus::GuardianSetAccountDataRejected
        );
        assert_eq!(result.rejection_case, Some(expected_rejection_case));
        assert!(!result.guardian_set_runtime_loading_enabled);
        assert!(!result.processed_registry_runtime_loading_enabled);
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
