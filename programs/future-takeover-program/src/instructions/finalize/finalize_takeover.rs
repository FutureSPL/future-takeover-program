use anchor_lang::prelude::*;

use crate::{
    errors::TakeoverError, 
    state::{AdminProfile, Takeover, Phase::* }, 
    constant::{ADMIN_BUFFER, SUCCESS_PERCENTAGE}
};

#[derive(Accounts)]
pub struct FinalizeTakeover<'info> {
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

pub fn handler(ctx: Context<FinalizeTakeover>) -> Result<()> {
    // Ensure the admin has been initialized for more than 16 hours
    require_gt!(
        Clock::get()?.unix_timestamp - ctx.accounts.admin_profile.creation_time,
        ADMIN_BUFFER,
        TakeoverError::UnauthorizedAdmin
    );

    // Ensure the swap period has ended
    // require_gt!(
    //     Clock::get()?.unix_timestamp,
    //     ctx.accounts.takeover.swap_period.end,
    //     TakeoverError::SwapPeriodNotEnded
    // ); - To be added later

    // Calculate the required amount for a successful presale
    let success_threshold = ctx.accounts.takeover.inflation_amount.presale_amount
        .checked_mul(SUCCESS_PERCENTAGE)
        .ok_or(TakeoverError::Overflow)?
        .checked_div(100)
        .ok_or(TakeoverError::Underflow)?;

    // Determine if the takeover is successful or failed
    if ctx.accounts.takeover.presale_claimed >= success_threshold {
        ctx.accounts.takeover.phase = TokenSelling;
    } else {
        ctx.accounts.takeover.phase = FailedTakeover;
    }

    Ok(())
}