use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount, burn, Burn, close_account, CloseAccount} 
};

use crate::{
    constant::ADMIN_BUFFER, errors::TakeoverError, state::{AdminProfile, Takeover}
};

#[derive(Accounts)]
pub struct CancelTakeover<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        mut,
        close = admin,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,

    #[account(mut)]
    pub new_mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub takeover_new_mint_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CancelTakeover<'info> {
    fn empty_and_burn_takeover_vault(&self) -> Result<()> {
        let vault_amount = self.takeover_new_mint_vault.amount;
        let old_mint = self.takeover.old_mint.key().clone();

        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];

        // Burn the tokens in the vault
        burn(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Burn {
                    mint: self.new_mint.to_account_info(),
                    from: self.takeover_new_mint_vault.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            vault_amount,
        )?;

        // Close the vault account
        close_account(
            CpiContext::new_with_signer(
                self.associated_token_program.to_account_info(),
                CloseAccount {
                    account: self.takeover_new_mint_vault.to_account_info(),
                    destination: self.admin.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
        )?; 

        Ok(())
    }
    
}

pub fn handler(ctx: Context<CancelTakeover>) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(Clock::get()?.unix_timestamp - ctx.accounts.admin_profile.creation_time > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check if the takeover has not started yet
    require!(ctx.accounts.takeover.swap_period.start > Clock::get()?.unix_timestamp, TakeoverError::TakeoverAlreadyStarted);

    // Empty and burn the takeover vault amount
    ctx.accounts.empty_and_burn_takeover_vault()?;

    Ok(())
}