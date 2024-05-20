use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{
    errors::TakeoverError, state::{AdminProfile, Takeover, Phase::* }, constant::ADMIN_BUFFER
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
        has_one = new_mint,
        has_one = old_mint,
    )]
    pub takeover: Account<'info, Takeover>,

    #[account(mut)]
    pub old_mint: Account<'info, Mint>,
    #[account(mut)]
    pub new_mint: Account<'info, Mint>,    

    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeTakeover<'info> {
    
}

pub fn handler(ctx: Context<FinalizeTakeover>) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(ctx.accounts.admin_profile.creation_time - Clock::get()?.unix_timestamp > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check that the takeover is already started and the swap period is active
    // require!(ctx.accounts.takeover.swap_period.end < Clock::get()?.unix_timestamp, TakeoverError::SwapPeriodNotEnded);

    // Check if the presale is successful, then migrate to successful or unsuccessful takeover account
    let presale_amount = ctx.accounts.old_mint.supply.checked_mul(ctx.accounts.takeover.inflation_amount.presale_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;
    let success_amount = presale_amount.checked_mul(60).ok_or(TakeoverError::Overflow)?.checked_div(100).ok_or(TakeoverError::Underflow)?;
    
    if success_amount < ctx.accounts.takeover.presale_claimed {
        ctx.accounts.takeover.phase = TokenSelling;
    } else {
        ctx.accounts.takeover.phase = FailedTakeover;
    }

    Ok(())
}