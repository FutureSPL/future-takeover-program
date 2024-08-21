use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked } 
};

use crate::{
    state::{Takeover, SwapReceipt},
    errors::TakeoverError,
};

#[derive(Accounts)]
pub struct SwapOldToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"takeover", old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        init_if_needed,
        payer = user,
        space = SwapReceipt::INIT_SPACE,
        seeds = [b"swap_receipt", takeover.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub swap_receipt: Account<'info, SwapReceipt>,
    pub old_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = old_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_old_mint_token: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = old_mint,
        associated_token::authority = takeover,
        associated_token::token_program = token_program
    )]
    pub takeover_old_mint_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SwapOldToken<'info> {
    fn initialize_receipt(&mut self, bump: u8) -> Result<()> {
        self.swap_receipt.takeover = self.takeover.key();
        self.swap_receipt.swapped_amount = self.swap_receipt.swapped_amount
            .checked_add(self.user_old_mint_token.amount)
            .ok_or(TakeoverError::Overflow)?;
        self.swap_receipt.bump = bump;
    
        Ok(())
    }

    fn seize_old_tokens(&mut self) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.user_old_mint_token.to_account_info(),
                    mint: self.old_mint.to_account_info(),
                    to: self.takeover_old_mint_vault.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            self.user_old_mint_token.amount,
            self.old_mint.decimals,
        )?;

        Ok(())
    }

}

pub fn handler(ctx: Context<SwapOldToken>) -> Result<()> {
    // Check that the takeover is already started and the swap period is active
    // require_lt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.start, TakeoverError::SwapPeriodNotStarted); - To be added back
    // require_gt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.end, TakeoverError::SwapPeriodEnded); - To be added back

    // Check if the amount is greater than 0
    require_gt!(
        ctx.accounts.user_old_mint_token.amount,
        0,
        TakeoverError::InvalidAmount
    );

    // Update the token swapped amount
    ctx.accounts.takeover.token_swapped = ctx.accounts.takeover.token_swapped
        .checked_add(ctx.accounts.user_old_mint_token.amount)
        .ok_or(TakeoverError::Overflow)?;

    // Initialize the swap receipt
    ctx.accounts.initialize_receipt(ctx.bumps.swap_receipt)?;

    // Deposit the old token
    ctx.accounts.seize_old_tokens()?;

    Ok(())
}