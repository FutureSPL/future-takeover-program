use anchor_lang::prelude::*;

#[account]
pub struct AdminProfile {
    pub address: Pubkey,
    pub username: String,
    pub creation_time: i64,
    pub bump: u8,
}

impl Space for AdminProfile {
    const INIT_SPACE: usize = 8 + 32 + 4 + 8 + 1;
}

#[account]
pub struct Takeover {
    pub old_mint: Pubkey,
    pub new_mint: Pubkey,
    pub swap_period: SwapPeriod,
    pub takeover_wallet: Pubkey,
    pub inflation_amount: InflationAmount,
    pub presale_price: u64,
    pub presale_claimed: u64,
    pub phase: Phase,
    pub bump: u8,
}

impl Space for Takeover {
    const INIT_SPACE: usize = 8 + 32 + 32 + SwapPeriod::INIT_SPACE + 32 + InflationAmount::INIT_SPACE + 8 + 8 + Phase::INIT_SPACE + 1;
}

#[account]
pub struct PresaleReceipt {
    pub takeover: Pubkey,
    pub presale_amount: u64,
    pub bump: u8,
}

impl Space for PresaleReceipt {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1;
}

#[account]
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
    pub rewards_basis_point: u16,
    pub treasury_basis_point: u16,
    pub presale_basis_point: u16,
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
    Cleanup,            // 4
    UnlockingAta,       // 5
}


