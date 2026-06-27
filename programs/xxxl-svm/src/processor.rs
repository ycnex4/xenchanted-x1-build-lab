use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
};

use crate::{
    cpi::{
        assert_gateway_mint_authority_pda, MintToCpiAccounts, MintToCpiBoundary,
    },
    error::XxxlError,
    instruction::{
        ConsumeGatewayMintArgs, XxxlInstruction,
        CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT,
    },
    state::{
        GatewayConfigAccountView, GuardianSetAccountView, MintStateAccountView,
        ProcessedEventAccountView, RecipientBalanceAccountView,
    },
    validation::{
        assert_account_owner, assert_initialized_mint_account,
        assert_recipient_ata_boundary, assert_rent_exempt,
    },
};

pub const CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS: usize = 9;

pub const ACCOUNT_INDEX_MINT_STATE: usize = 0;
pub const ACCOUNT_INDEX_GATEWAY_CONFIG: usize = 1;
pub const ACCOUNT_INDEX_GUARDIAN_SET: usize = 2;
pub const ACCOUNT_INDEX_PROCESSED_EVENT: usize = 3;
pub const ACCOUNT_INDEX_RECIPIENT_BALANCE: usize = 4;
pub const ACCOUNT_INDEX_SPL_TOKEN_MINT: usize = 5;
pub const ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT: usize = 6;
pub const ACCOUNT_INDEX_MINT_AUTHORITY_PDA: usize = 7;
pub const ACCOUNT_INDEX_TOKEN_PROGRAM: usize = 8;

pub struct PreparedConsumeGatewayMintCpi<'a, 'b> {
    pub boundary: MintToCpiBoundary<'a, 'b>,
    pub mint_decimals: u8,
    pub source_chain_weight_bps: u16,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = XxxlInstruction::unpack(instruction_data)?;

    match instruction {
        XxxlInstruction::ConsumeGatewayMint(args) => {
            process_consume_gateway_mint(program_id, accounts, &args)
        }
    }
}

fn process_consume_gateway_mint(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _args: &ConsumeGatewayMintArgs,
) -> ProgramResult {
    msg!("XXXL consume_gateway_mint scaffold reached; live route execution is not activated");
    Ok(())
}

pub fn prepare_consume_gateway_mint_cpi_boundary<'a, 'b>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
    args: &ConsumeGatewayMintArgs,
    rent: &Rent,
) -> Result<PreparedConsumeGatewayMintCpi<'a, 'b>, ProgramError> {
    if accounts.len() != CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS
        || args.account_meta_count != CONSUME_GATEWAY_MINT_ACCOUNT_META_COUNT
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    let mint_state_account = account_at(accounts, args.mint_state_account_index as usize)?;
    let gateway_config_account = account_at(accounts, args.route_account_index as usize)?;
    let guardian_set_account = account_at(accounts, args.guardian_set_account_index as usize)?;
    let processed_event_account =
        account_at(accounts, args.processed_event_account_index as usize)?;
    let recipient_balance_account =
        account_at(accounts, args.recipient_balance_account_index as usize)?;
    let spl_token_mint_account = account_at(accounts, ACCOUNT_INDEX_SPL_TOKEN_MINT)?;
    let recipient_token_account = account_at(accounts, ACCOUNT_INDEX_RECIPIENT_TOKEN_ACCOUNT)?;
    let mint_authority_pda = account_at(accounts, ACCOUNT_INDEX_MINT_AUTHORITY_PDA)?;
    let token_program = account_at(accounts, ACCOUNT_INDEX_TOKEN_PROGRAM)?;

    if token_program.key != &spl_token::id() {
        return Err(XxxlError::InvalidAccountOwner.into());
    }

    for program_owned_account in [
        mint_state_account,
        gateway_config_account,
        guardian_set_account,
        processed_event_account,
        recipient_balance_account,
    ] {
        assert_account_owner(program_owned_account, program_id)?;
        assert_rent_exempt(program_owned_account, rent)?;
    }

    assert_rent_exempt(spl_token_mint_account, rent)?;
    assert_rent_exempt(recipient_token_account, rent)?;

    let mint_state_data = mint_state_account.try_borrow_data()?;
    let gateway_config_data = gateway_config_account.try_borrow_data()?;
    let guardian_set_data = guardian_set_account.try_borrow_data()?;
    let processed_event_data = processed_event_account.try_borrow_data()?;
    let recipient_balance_data = recipient_balance_account.try_borrow_data()?;

    let mint_state = MintStateAccountView::new(&mint_state_data)?;
    let gateway_config = GatewayConfigAccountView::new(&gateway_config_data)?;
    let guardian_set = GuardianSetAccountView::new(&guardian_set_data)?;
    let processed_event = ProcessedEventAccountView::new(&processed_event_data)?;
    let recipient_balance = RecipientBalanceAccountView::new(&recipient_balance_data)?;

    if mint_state.mint_pubkey() != args.mint_id
        || mint_state.gateway_mint_authority_pda() != mint_authority_pda.key.to_bytes()
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    assert_gateway_mint_authority_pda(
        program_id,
        mint_authority_pda.key,
        mint_state.gateway_mint_authority_bump(),
    )?;

    if gateway_config.route_id() != args.route_id
        || gateway_config.guardian_set_id() != args.guardian_set_id
        || gateway_config.target_mint() != args.mint_id
        || gateway_config.source_chain_weight_bps() != args.source_chain_weight_bps
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if guardian_set.guardian_set_id() != args.guardian_set_id {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if processed_event.consumed()
        || processed_event.canonical_event_key() != args.canonical_event_key
        || processed_event.route_id() != args.route_id
        || processed_event.recipient() != args.recipient
    {
        return Err(XxxlError::InvalidInstruction.into());
    }

    if recipient_balance.owner() != args.recipient
        || recipient_balance.mint() != args.mint_id
    {
        return Err(XxxlError::InvalidRecipientAta.into());
    }

    let mint_pubkey = Pubkey::new_from_array(args.mint_id);
    let recipient_owner = Pubkey::new_from_array(args.recipient);

    let mint_decimals =
        assert_initialized_mint_account(spl_token_mint_account, mint_authority_pda.key)?;

    assert_recipient_ata_boundary(
        recipient_token_account,
        &recipient_owner,
        &mint_pubkey,
    )?;

    if args.amount == 0 || args.amount > u64::MAX as u128 {
        return Err(XxxlError::InvalidInstruction.into());
    }

    Ok(PreparedConsumeGatewayMintCpi {
        boundary: MintToCpiBoundary {
            accounts: MintToCpiAccounts {
                token_program,
                mint: spl_token_mint_account,
                recipient_token_account,
                mint_authority_pda,
            },
            mint_authority_bump: mint_state.gateway_mint_authority_bump(),
            amount: args.amount as u64,
        },
        mint_decimals,
        source_chain_weight_bps: args.source_chain_weight_bps,
    })
}

fn account_at<'a, 'b>(
    accounts: &'a [AccountInfo<'b>],
    index: usize,
) -> Result<&'a AccountInfo<'b>, ProgramError> {
    accounts
        .get(index)
        .ok_or_else(|| XxxlError::InvalidInstruction.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        instruction::{
            CONSUME_GATEWAY_MINT_DISCRIMINATOR,
            CONSUME_GATEWAY_MINT_INSTRUCTION_LEN,
            INSTRUCTION_LAYOUT_VERSION,
        },
        pda::find_gateway_mint_authority,
        state::{
            GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR, GATEWAY_CONFIG_ACCOUNT_LEN,
            GUARDIAN_SET_ACCOUNT_DISCRIMINATOR, GUARDIAN_SET_ACCOUNT_LEN,
            MINT_STATE_ACCOUNT_DISCRIMINATOR, MINT_STATE_ACCOUNT_LEN,
            PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR, PROCESSED_EVENT_ACCOUNT_LEN,
            RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR, RECIPIENT_BALANCE_ACCOUNT_LEN,
            RUNTIME_LAYOUT_VERSION,
        },
    };
    use solana_program::{
        account_info::AccountInfo,
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };
    use spl_token::state::{
        Account as SplTokenAccount, AccountState, Mint as SplTokenMint,
    };
    use std::str::FromStr;

    const FIXTURE_PROGRAM_ID: &str = "11111111111111111111111111111111";

    #[test]
    fn handler_integration_prepares_cpi_boundary_after_decode_and_validation() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();

        let accounts = fixture.accounts();

        let prepared = prepare_consume_gateway_mint_cpi_boundary(
            &program_id,
            &accounts,
            &args,
            &rent,
        )
        .expect("prepared CPI boundary");

        assert_eq!(prepared.boundary.amount, 1_000);
        assert_eq!(prepared.boundary.mint_authority_bump, fixture_bump());
        assert_eq!(prepared.mint_decimals, 18);
        assert_eq!(prepared.source_chain_weight_bps, 10_000);
        assert_eq!(prepared.boundary.accounts.token_program.key, &spl_token::id());
    }

    #[test]
    fn handler_integration_rejects_wrong_account_count() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();

        let accounts = fixture.accounts();
        let result = prepare_consume_gateway_mint_cpi_boundary(
            &program_id,
            &accounts[..CONSUME_GATEWAY_MINT_REQUIRED_ACCOUNTS - 1],
            &args,
            &rent,
        );

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_gateway_route_mismatch() {
        let mut fixture = HandlerFixture::new();
        fixture.data.gateway_config[16] ^= 0xff;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_consumed_processed_event() {
        let mut fixture = HandlerFixture::new();
        fixture.data.processed_event[10] = 1;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_mint_authority_pda() {
        let mut fixture = HandlerFixture::new();
        fixture.keys.mint_authority_pda = Pubkey::new_unique();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn handler_integration_rejects_wrong_spl_mint_owner() {
        let mut fixture = HandlerFixture::new();
        fixture.owners.spl_token = Pubkey::new_unique();

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidAccountOwner);
    }

    #[test]
    fn handler_integration_rejects_wrong_recipient_token_mint() {
        let mut fixture = HandlerFixture::new();
        fixture.data.recipient_token_account =
            packed_token_account(Pubkey::new_unique(), fixture.keys.recipient_owner, AccountState::Initialized);

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidRecipientAta);
    }

    #[test]
    fn handler_integration_rejects_zero_amount() {
        let mut fixture = HandlerFixture::new();
        fixture.args.amount = 0;

        let program_id = fixture.program_id;
        let args = fixture.args;
        let rent = Rent::default();
        let accounts = fixture.accounts();

        let result =
            prepare_consume_gateway_mint_cpi_boundary(&program_id, &accounts, &args, &rent);

        assert_custom_error(result, XxxlError::InvalidInstruction);
    }

    #[test]
    fn process_instruction_remains_scaffold_only_not_live_route_activation() {
        let mut fixture = HandlerFixture::new();
        let program_id = fixture.program_id;
        let instruction_data = fixture.instruction_data;
        let accounts = fixture.accounts();

        process_instruction(&program_id, &accounts, &instruction_data)
            .expect("scaffold handler remains non-live");
    }

    struct HandlerFixture {
        program_id: Pubkey,
        owners: FixtureOwners,
        keys: FixtureKeys,
        lamports: FixtureLamports,
        data: FixtureData,
        args: ConsumeGatewayMintArgs,
        instruction_data: [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
    }

    struct FixtureOwners {
        program: Pubkey,
        spl_token: Pubkey,
        token_program_owner: Pubkey,
    }

    struct FixtureKeys {
        mint_state: Pubkey,
        gateway_config: Pubkey,
        guardian_set: Pubkey,
        processed_event: Pubkey,
        recipient_balance: Pubkey,
        spl_mint: Pubkey,
        recipient_token_account: Pubkey,
        mint_authority_pda: Pubkey,
        token_program: Pubkey,
        recipient_owner: Pubkey,
    }

    struct FixtureLamports {
        mint_state: u64,
        gateway_config: u64,
        guardian_set: u64,
        processed_event: u64,
        recipient_balance: u64,
        spl_mint: u64,
        recipient_token_account: u64,
        mint_authority_pda: u64,
        token_program: u64,
    }

    struct FixtureData {
        mint_state: Vec<u8>,
        gateway_config: Vec<u8>,
        guardian_set: Vec<u8>,
        processed_event: Vec<u8>,
        recipient_balance: Vec<u8>,
        spl_mint: Vec<u8>,
        recipient_token_account: Vec<u8>,
        mint_authority_pda: Vec<u8>,
        token_program: Vec<u8>,
    }

    impl HandlerFixture {
        fn new() -> Self {
            let program_id =
                Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
            let (mint_authority_pda, bump) = find_gateway_mint_authority(&program_id);

            let spl_mint = Pubkey::new_unique();
            let recipient_owner = Pubkey::new_unique();
            let route_id = [0x11; 32];
            let guardian_set_id = [0x22; 32];
            let canonical_event_key = [0x44; 32];

            let owners = FixtureOwners {
                program: program_id,
                spl_token: spl_token::id(),
                token_program_owner: Pubkey::new_unique(),
            };

            let keys = FixtureKeys {
                mint_state: Pubkey::new_unique(),
                gateway_config: Pubkey::new_unique(),
                guardian_set: Pubkey::new_unique(),
                processed_event: Pubkey::new_unique(),
                recipient_balance: Pubkey::new_unique(),
                spl_mint,
                recipient_token_account: Pubkey::new_unique(),
                mint_authority_pda,
                token_program: spl_token::id(),
                recipient_owner,
            };

            let data = FixtureData {
                mint_state: mint_state_data(spl_mint, mint_authority_pda, bump),
                gateway_config: gateway_config_data(
                    route_id,
                    guardian_set_id,
                    spl_mint,
                    10_000,
                ),
                guardian_set: guardian_set_data(guardian_set_id),
                processed_event: processed_event_data(
                    false,
                    canonical_event_key,
                    route_id,
                    recipient_owner,
                ),
                recipient_balance: recipient_balance_data(recipient_owner, spl_mint),
                spl_mint: packed_mint(mint_authority_pda, true),
                recipient_token_account: packed_token_account(
                    spl_mint,
                    recipient_owner,
                    AccountState::Initialized,
                ),
                mint_authority_pda: Vec::new(),
                token_program: Vec::new(),
            };

            let rent = Rent::default();

            let lamports = FixtureLamports {
                mint_state: rent.minimum_balance(data.mint_state.len()),
                gateway_config: rent.minimum_balance(data.gateway_config.len()),
                guardian_set: rent.minimum_balance(data.guardian_set.len()),
                processed_event: rent.minimum_balance(data.processed_event.len()),
                recipient_balance: rent.minimum_balance(data.recipient_balance.len()),
                spl_mint: rent.minimum_balance(data.spl_mint.len()),
                recipient_token_account: rent.minimum_balance(data.recipient_token_account.len()),
                mint_authority_pda: 0,
                token_program: 0,
            };

            let args = ConsumeGatewayMintArgs {
                raw: [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN],
                account_meta_count: 9,
                route_account_index: 1,
                guardian_set_account_index: 2,
                mint_state_account_index: 0,
                processed_event_account_index: 3,
                recipient_balance_account_index: 4,
                route_id,
                guardian_set_id,
                mint_id: spl_mint.to_bytes(),
                canonical_event_key,
                recipient: recipient_owner.to_bytes(),
                amount: 1_000,
                source_chain_weight_bps: 10_000,
            };

            let instruction_data = instruction_data_from_args(&args);

            Self {
                program_id,
                owners,
                keys,
                lamports,
                data,
                args,
                instruction_data,
            }
        }

        fn accounts(&mut self) -> Vec<AccountInfo<'_>> {
            vec![
                AccountInfo::new(
                    &self.keys.mint_state,
                    false,
                    true,
                    &mut self.lamports.mint_state,
                    &mut self.data.mint_state,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.gateway_config,
                    false,
                    true,
                    &mut self.lamports.gateway_config,
                    &mut self.data.gateway_config,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.guardian_set,
                    false,
                    false,
                    &mut self.lamports.guardian_set,
                    &mut self.data.guardian_set,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.processed_event,
                    false,
                    true,
                    &mut self.lamports.processed_event,
                    &mut self.data.processed_event,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.recipient_balance,
                    false,
                    true,
                    &mut self.lamports.recipient_balance,
                    &mut self.data.recipient_balance,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.spl_mint,
                    false,
                    true,
                    &mut self.lamports.spl_mint,
                    &mut self.data.spl_mint,
                    &self.owners.spl_token,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.recipient_token_account,
                    false,
                    true,
                    &mut self.lamports.recipient_token_account,
                    &mut self.data.recipient_token_account,
                    &self.owners.spl_token,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.mint_authority_pda,
                    false,
                    false,
                    &mut self.lamports.mint_authority_pda,
                    &mut self.data.mint_authority_pda,
                    &self.owners.program,
                    false,
                    0,
                ),
                AccountInfo::new(
                    &self.keys.token_program,
                    false,
                    false,
                    &mut self.lamports.token_program,
                    &mut self.data.token_program,
                    &self.owners.token_program_owner,
                    true,
                    0,
                ),
            ]
        }
    }

    fn mint_state_data(mint: Pubkey, pda: Pubkey, bump: u8) -> Vec<u8> {
        let mut data = account_data(MINT_STATE_ACCOUNT_LEN, MINT_STATE_ACCOUNT_DISCRIMINATOR);
        data[10] = 18;
        data[13] = bump;
        data[16..48].copy_from_slice(&mint.to_bytes());
        data[64..96].copy_from_slice(&pda.to_bytes());
        data
    }

    fn gateway_config_data(
        route_id: [u8; 32],
        guardian_set_id: [u8; 32],
        target_mint: Pubkey,
        weight_bps: u16,
    ) -> Vec<u8> {
        let mut data =
            account_data(GATEWAY_CONFIG_ACCOUNT_LEN, GATEWAY_CONFIG_ACCOUNT_DISCRIMINATOR);
        data[12..14].copy_from_slice(&weight_bps.to_le_bytes());
        data[16..48].copy_from_slice(&route_id);
        data[88..120].copy_from_slice(&target_mint.to_bytes());
        data[120..152].copy_from_slice(&guardian_set_id);
        data
    }

    fn guardian_set_data(guardian_set_id: [u8; 32]) -> Vec<u8> {
        let mut data = account_data(GUARDIAN_SET_ACCOUNT_LEN, GUARDIAN_SET_ACCOUNT_DISCRIMINATOR);
        data[272..304].copy_from_slice(&guardian_set_id);
        data
    }

    fn processed_event_data(
        consumed: bool,
        canonical_event_key: [u8; 32],
        route_id: [u8; 32],
        recipient: Pubkey,
    ) -> Vec<u8> {
        let mut data =
            account_data(PROCESSED_EVENT_ACCOUNT_LEN, PROCESSED_EVENT_ACCOUNT_DISCRIMINATOR);
        data[10] = if consumed { 1 } else { 0 };
        data[16..48].copy_from_slice(&canonical_event_key);
        data[48..80].copy_from_slice(&route_id);
        data[80..112].copy_from_slice(&recipient.to_bytes());
        data
    }

    fn recipient_balance_data(owner: Pubkey, mint: Pubkey) -> Vec<u8> {
        let mut data =
            account_data(RECIPIENT_BALANCE_ACCOUNT_LEN, RECIPIENT_BALANCE_ACCOUNT_DISCRIMINATOR);
        data[16..48].copy_from_slice(&owner.to_bytes());
        data[48..80].copy_from_slice(&mint.to_bytes());
        data
    }

    fn account_data(len: usize, discriminator: [u8; 8]) -> Vec<u8> {
        let mut data = vec![0u8; len];
        data[0..8].copy_from_slice(&discriminator);
        data[8..10].copy_from_slice(&RUNTIME_LAYOUT_VERSION.to_le_bytes());
        data
    }

    fn packed_mint(mint_authority: Pubkey, initialized: bool) -> Vec<u8> {
        let mut data = vec![0u8; SplTokenMint::LEN];
        let mint = SplTokenMint {
            mint_authority: COption::Some(mint_authority),
            supply: 0,
            decimals: 18,
            is_initialized: initialized,
            freeze_authority: COption::None,
        };

        SplTokenMint::pack(mint, &mut data).expect("pack mint");
        data
    }

    fn packed_token_account(
        mint: Pubkey,
        owner: Pubkey,
        state: AccountState,
    ) -> Vec<u8> {
        let mut data = vec![0u8; SplTokenAccount::LEN];
        let account = SplTokenAccount {
            mint,
            owner,
            amount: 0,
            delegate: COption::None,
            state,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        SplTokenAccount::pack(account, &mut data).expect("pack token account");
        data
    }

    fn instruction_data_from_args(
        args: &ConsumeGatewayMintArgs,
    ) -> [u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN] {
        let mut bytes = [0u8; CONSUME_GATEWAY_MINT_INSTRUCTION_LEN];

        bytes[0..8].copy_from_slice(&CONSUME_GATEWAY_MINT_DISCRIMINATOR);
        bytes[8..10].copy_from_slice(&INSTRUCTION_LAYOUT_VERSION.to_le_bytes());
        bytes[10] = args.account_meta_count;
        bytes[11] = args.route_account_index;
        bytes[12] = args.guardian_set_account_index;
        bytes[13] = args.mint_state_account_index;
        bytes[14] = args.processed_event_account_index;
        bytes[15] = args.recipient_balance_account_index;
        bytes[16..48].copy_from_slice(&args.route_id);
        bytes[48..80].copy_from_slice(&args.guardian_set_id);
        bytes[80..112].copy_from_slice(&args.mint_id);
        bytes[112..144].copy_from_slice(&args.canonical_event_key);
        bytes[144..176].copy_from_slice(&args.recipient);
        bytes[176..192].copy_from_slice(&args.amount.to_le_bytes());
        bytes[192..194].copy_from_slice(&args.source_chain_weight_bps.to_le_bytes());

        bytes
    }

    fn fixture_bump() -> u8 {
        let program_id =
            Pubkey::from_str(FIXTURE_PROGRAM_ID).expect("valid fixture program id");
        let (_pda, bump) = find_gateway_mint_authority(&program_id);
        bump
    }

    fn assert_custom_error<T>(
        result: Result<T, ProgramError>,
        error: XxxlError,
    ) {
        assert!(matches!(result, Err(ProgramError::Custom(code)) if code == error as u32));
    }
}
