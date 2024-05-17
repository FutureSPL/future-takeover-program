use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount, transfer, Transfer} 
};
use crate::{
    state::{Takeover, AdminProfile},
    errors::TakeoverError,
};

#[derive(Accounts)]
pub struct ClaimPresale<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(mut)]
    pub new_mint: Account<'info, Mint>,
    #[account(mut)]
    pub old_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = new_mint,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub takeover_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = admin,
    )]
    pub admin_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimPresale<'info> {
    fn claim_presale(&mut self) -> Result<()> {
        // Calculate the presale amount: rewards% of the old mint supply
        let presale_amount = self.old_mint.supply.checked_mul(self.takeover.inflation_amount.presale_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;

        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];


        // Claim the presale tokens in the vault
        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.takeover_vault.to_account_info(),
                    to: self.admin_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            presale_amount,
        )?;

        // Update the Presale Amount
        self.takeover.presale_claimed = presale_amount;

        Ok(())
    }
    
}

pub fn handler(ctx: Context<ClaimPresale>) -> Result<()> {
    // Check that the takeover is not going to start in the next 24h (?)
    require!(ctx.accounts.takeover.swap_period.start > Clock::get()?.unix_timestamp - 24 * 60 * 60, TakeoverError::TakeoverAlreadyStarted);

    // Check if the admin has been initialized more than 16h ago
    require!(Clock::get()?.unix_timestamp - ctx.accounts.admin_profile.creation_time > 16 * 60 * 60, TakeoverError::UnauthorizedAdmin);

    // Claim the Presale tokens in the vault
    ctx.accounts.claim_presale()?;

    // Check if the admin then will send this token to Atlas (?)

    Ok(())
}