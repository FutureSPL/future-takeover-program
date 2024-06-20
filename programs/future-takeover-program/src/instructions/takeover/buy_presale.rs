use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer}
};
use anchor_spl::token::Mint;

use crate::{
    state::{Takeover, PresaleReceipt},
    errors::TakeoverError,
};

#[derive(Accounts)]
pub struct BuyPresale<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        init,
        payer = user,
        space = PresaleReceipt::INIT_SPACE,
        seeds = [b"presale_receipt", takeover.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub presale_receipt: Account<'info, PresaleReceipt>,

    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_vault: SystemAccount<'info>,
    pub new_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
}

impl<'info> BuyPresale<'info> {
    pub fn initialize_presale_receipt(&mut self, presale_amount: u64, bump: u8) -> Result<()> {
        // Initialize the Presale Receipt
        self.presale_receipt.set_inner(
            PresaleReceipt {
                takeover: self.takeover.key(),
                presale_amount,
                bump,
            }
        );

        Ok(())
    }

    pub fn buy_presale(&self, amount: u64) -> Result<()> {
        // Transfer Sol from User to Takeover Vault
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.user.to_account_info(),
                    to: self.takeover_vault.to_account_info(),
                }
            ),
            self.takeover.presale_price.checked_mul(amount).ok_or(TakeoverError::Overflow)?
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<BuyPresale>, amount: u64) -> Result<()> {
    // Check that the takeover is already started and the swap period is active
    require!(ctx.accounts.takeover.swap_period.start < Clock::get()?.unix_timestamp, TakeoverError::SwapPeriodNotStarted);
    require!(ctx.accounts.takeover.swap_period.end > Clock::get()?.unix_timestamp, TakeoverError::SwapPeriodEnded);  

    // Check if the amount is greater than 0
    require!(amount > 0, TakeoverError::InvalidAmount);

    // Check if there are enough tokens in the presale vault
    require!(ctx.accounts.takeover.inflation_amount.presale_amount.checked_sub(ctx.accounts.takeover.presale_claimed).ok_or(TakeoverError::Underflow)? >= amount, TakeoverError::NotEnoughTokens);
   
    // Initialize the presale receipt
    ctx.accounts.initialize_presale_receipt(amount, ctx.bumps.presale_receipt)?;

    // Buy Presale Allocation
    ctx.accounts.buy_presale(amount)?;

    // Add the presale_amount to the Takeover State
    ctx.accounts.takeover.presale_claimed = ctx.accounts.takeover.presale_claimed.checked_add(amount).ok_or(TakeoverError::Overflow)?;

    Ok(())
}