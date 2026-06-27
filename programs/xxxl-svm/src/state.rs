pub const MINT_STATE_ACCOUNT_LEN: usize = 176;
pub const GATEWAY_CONFIG_ACCOUNT_LEN: usize = 256;
pub const GUARDIAN_SET_ACCOUNT_LEN: usize = 320;
pub const PROCESSED_EVENT_ACCOUNT_LEN: usize = 144;
pub const RECIPIENT_BALANCE_ACCOUNT_LEN: usize = 144;

pub const ACCOUNT_DISCRIMINATOR_LEN: usize = 8;
pub const VERSION_LEN: usize = 2;
pub const RUNTIME_LAYOUT_VERSION: u16 = 1;

pub struct MintStateAccountView<'a> {
    pub data: &'a [u8],
}

pub struct GatewayConfigAccountView<'a> {
    pub data: &'a [u8],
}

pub struct GuardianSetAccountView<'a> {
    pub data: &'a [u8],
}

pub struct ProcessedEventAccountView<'a> {
    pub data: &'a [u8],
}

pub struct RecipientBalanceAccountView<'a> {
    pub data: &'a [u8],
}

impl<'a> MintStateAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> GatewayConfigAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> GuardianSetAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> ProcessedEventAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

impl<'a> RecipientBalanceAccountView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}
