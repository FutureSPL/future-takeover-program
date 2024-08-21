use anchor_lang::prelude::*;

use anchor_spl::token_interface::{ Mint, TokenInterface, TokenAccount, burn, Burn, close_account, CloseAccount };

use crate::{
    constant::ADMIN_BUFFER, 
    errors::TakeoverError, 
    state::{AdminProfile, Takeover}
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
        seeds = [b"takeover", takeover.old_mints.old_mint.as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(mut)]
    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
        associated_token::token_program = token_program
    )]
    pub takeover_new_mint_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CancelTakeover<'info> {

    // Empty and burn the takeover vault amount
    fn burn_and_close_vault(&self) -> Result<()> {
        let old_mint = self.takeover.old_mints.old_mint.key();
        let bump = self.takeover.bump;

        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[bump],
        ];

        let vault_amount = self.takeover_new_mint_vault.amount;

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

        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
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
    require_gt!(
        Clock::get()?.unix_timestamp
            .checked_sub(ctx.accounts.admin_profile.creation_time)
            .ok_or(TakeoverError::Underflow)?, 
        ADMIN_BUFFER, 
        TakeoverError::UnauthorizedAdmin
    );

    // Check if the takeover has not started yet
    // require_gt!(
    //     ctx.accounts.takeover.swap_period.start, 
    //     Clock::get()?.unix_timestamp, 
    //     TakeoverError::TakeoverAlreadyStarted
    // );

    ctx.accounts.burn_and_close_vault()?;

    Ok(())
}