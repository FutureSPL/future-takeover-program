use anchor_lang::prelude::*;

use crate::{
    state::{Takeover, AdminProfile},
    errors::TakeoverError,
    constant::{TAKEOVER_BUFFER, ADMIN_BUFFER},
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct UpdateTakeoverArgs {
    pub start: i64,
    pub end: i64,
    pub takeover_wallet: Pubkey,
    pub presale_price: u64,
}

#[derive(Accounts)]
pub struct UpdateTakeover<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,

    pub system_program: Program<'info, System>,
}

impl<'info> UpdateTakeover<'info> {
    pub fn update_takeover(&mut self, start: i64, end: i64, takeover_wallet: Pubkey, presale_price: u64) -> Result<()> {
        self.takeover.swap_period.start = start;
        self.takeover.swap_period.end = end;
        self.takeover.takeover_wallet = takeover_wallet;
        self.takeover.presale_price = presale_price;

        Ok(())
    }
    
}

pub fn handler(ctx: Context<UpdateTakeover>, args: UpdateTakeoverArgs) -> Result<()> {
    // Check that the takeover is not going to start in the next 24h
    require!(ctx.accounts.takeover.swap_period.start > Clock::get()?.unix_timestamp - TAKEOVER_BUFFER, TakeoverError::TakeoverAlreadyStarted);

    // Check if the admin has been initialized more than 16h ago
    require!(ctx.accounts.admin_profile.creation_time - Clock::get()?.unix_timestamp > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check the Swap Period Parameters
    require!(args.start < args.end && args.start > Clock::get()?.unix_timestamp, TakeoverError::InvalidSwapPeriod);

    // Update the Takeover State
    if ctx.accounts.takeover.swap_period.start != args.start || ctx.accounts.takeover.swap_period.end != args.end || ctx.accounts.takeover.takeover_wallet != args.takeover_wallet || ctx.accounts.takeover.presale_price != args.presale_price{
        ctx.accounts.update_takeover(args.start, args.end, args.takeover_wallet, args.presale_price)?;
    }

    Ok(())
}