use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct AdminProfile {
    pub address: Pubkey,
    #[max_len(30)]
    pub username: String,
    pub creation_time: i64,
    pub bump: u8,
}

#[account]
pub struct Takeover {
    pub old_mints: OldMints,
    pub new_mint: Pubkey,
    pub swap_period: SwapPeriod,
    pub takeover_wallet: Pubkey,
    pub referral: Option<Pubkey>,
    pub inflation_amount: InflationAmount,
    pub presale_price: u64,
    pub presale_claimed: u64,
    pub token_swapped: u64,
    pub phase: Phase,
    pub bump: u8,
}

impl Space for Takeover {
    const INIT_SPACE: usize = 8 + OldMints::INIT_SPACE + 32 + SwapPeriod::INIT_SPACE + 32 + (1 + 32) + InflationAmount::INIT_SPACE + 8 + 8 + 8 + Phase::INIT_SPACE + 1;
}

#[account]
// #[derive(InitSpace)]
pub struct PresaleReceipt {
    pub takeover: Pubkey,
    pub presale_amount: u64,
    pub bump: u8,
}

impl Space for PresaleReceipt {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1;
}

#[account]
// #[derive(InitSpace)]
pub struct SwapReceipt {
    pub takeover: Pubkey,
    pub swapped_amount: u64,
    pub bump: u8,
}

impl Space for SwapReceipt {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct SwapPeriod {
    pub start: i64,
    pub end: i64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct InflationAmount {
    pub level: Level,
    pub rewards_amount: u64,
    pub treasury_amount: u64,
    pub presale_amount: u64,
    pub referral_amount: u64
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct OldMints {
    pub old_mint: Pubkey,
    pub weight_percentage: Option<u8>,
    pub old_mint_2: Option<Pubkey>,
    pub weight_percentage_2: Option<u8>,
    pub old_mint_3: Option<Pubkey>,
    pub weight_percentage_3: Option<u8>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum Level {
    Low,        // 0
    Medium,     // 1
    High,       // 2
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum Phase {
    Ongoing,            // 0
    FailedTakeover,     // 1
    TokenSelling,       // 2
    MarketCreation,     // 3
    RewardDistribution, // 4
    Cleanup,            // 5
    ClaimTokens,        // 6
}


