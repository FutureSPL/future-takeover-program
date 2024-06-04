use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod constant;

pub mod instructions;
pub use instructions::*;

declare_id!("Az7xrYvsyP7M6vC955gEk5sCp4XkX1dCREh1TrP5b5wB");

#[program]
pub mod future_takeover_program {
    use super::*;

    // Setup Instructions

    // - Admin Setup
    pub fn admin_init(ctx: Context<AdminInit>, username: String) -> Result<()> {
        instructions::admin_init::handler(ctx, username)
    }

    pub fn admin_delete(ctx: Context<AdminDelete>) -> Result<()> {
        instructions::admin_delete::handler(ctx)
    }

    // - Takeover Setup
    pub fn create_takeover(ctx: Context<CreateTakeover>, args: CreateTakeoverArgs) -> Result<()> {
        instructions::create_takeover::handler(ctx, args)
    }

    pub fn update_takeover(ctx: Context<UpdateTakeover>, args: UpdateTakeoverArgs) -> Result<()> {
        instructions::update_takeover::handler(ctx, args)
    }

    pub fn cancel_takeover(ctx: Context<CancelTakeover>) -> Result<()> {
        instructions::cancel_takeover::handler(ctx)
    }

    // Takeover Instructions
    pub fn buy_presale(ctx: Context<BuyPresale>, amount: u64) -> Result<()> {
        instructions::buy_presale::handler(ctx, amount)
    }

    pub fn swap_old_token(ctx: Context<SwapOldToken>) -> Result<()> {
        instructions::swap_old_token::handler(ctx)
    }

    // Finalize Instructions
    pub fn finalize_takeover(ctx: Context<FinalizeTakeover>) -> Result<()> {
        instructions::finalize_takeover::handler(ctx)
    }

    // - Successful Takeover
    pub fn sell_token(ctx: Context<SellToken>, amount: u64) -> Result<()> {
        instructions::sell_token::handler(ctx, amount)
    }

    pub fn finalize_sell(ctx: Context<FinalizeSellToken>) -> Result<()> {
        instructions::finalize_sell::handler(ctx)
    }

    pub fn open_and_deposit_position(ctx: Context<OpenAndDepositPosition>, args: OpenAndDepositPositionArgs) -> Result<()> {
        instructions::open_and_deposit_position::handler(ctx, args)
    }

    pub fn cleanup(ctx: Context<Cleanup>) -> Result<()> {
        instructions::cleanup::handler(ctx)
    }

    pub fn claim_tokens(ctx: Context<ClaimTokens>) -> Result<()> {
        instructions::claim_tokens::handler(ctx)
    }

    // - Failed Takeover    
    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        instructions::claim_refund::handler(ctx)
    }
}