use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer}
};
use anchor_spl::token_interface::Mint;

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
        init_if_needed,
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
    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyPresale<'info> {
    // Initialize the presale receipt
    pub fn initialize_receipt(&mut self, presale_amount: u64, bump: u8) -> Result<()> {
        self.presale_receipt.takeover = self.takeover.key();
        self.presale_receipt.presale_amount = self.presale_receipt.presale_amount
            .checked_add(presale_amount)
            .ok_or(TakeoverError::Overflow)?;
        self.presale_receipt.bump = bump;

        Ok(())
    }

    // Execute the presale purchase and transfer the funds to the takeover vault
    pub fn execute_presale(&self, amount: u64) -> Result<()> {
        let transfer_amount = self.takeover.presale_price
            .checked_mul(amount)
            .ok_or(TakeoverError::Overflow)?;

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.user.to_account_info(),
                    to: self.takeover_vault.to_account_info(),
                }
            ),
            transfer_amount,
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<BuyPresale>, amount: u64) -> Result<()> {
    // Check that the takeover is already started and the swap period is active
    // require_lt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.start, TakeoverError::SwapPeriodNotStarted); - To be added back
    // require_gt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.end, TakeoverError::SwapPeriodEnded); - To be added back

    // Check if the amount is greater than 0
    require_gt!(amount, 0, TakeoverError::InvalidAmount);

    // Convert the amount to the appropriate decimal form
    let decimals_factor = 10u64
        .checked_pow(ctx.accounts.new_mint.decimals as u32)
        .ok_or(TakeoverError::Overflow)?;

    let amount_in_decimals = amount
        .checked_mul(decimals_factor)
        .ok_or(TakeoverError::Overflow)?;

    // Check if there are enough tokens in the presale vault
    require_gte!(
        ctx.accounts.takeover.inflation_amount.presale_amount
            .checked_sub(ctx.accounts.takeover.presale_claimed)
            .ok_or(TakeoverError::Underflow)?,
        amount_in_decimals,
        TakeoverError::NotEnoughTokens
    );
   
   ctx.accounts.initialize_receipt(amount_in_decimals, ctx.bumps.presale_receipt)?;

   ctx.accounts.execute_presale(amount)?;

    // Add the presale_amount to the Takeover State
    ctx.accounts.takeover.presale_claimed = ctx.accounts.takeover.presale_claimed
        .checked_add(amount_in_decimals)
        .ok_or(TakeoverError::Overflow)?;

    Ok(())
}