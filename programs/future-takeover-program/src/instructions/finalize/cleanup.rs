use anchor_lang::{
    prelude::*, 
    system_program::{transfer, Transfer}
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer as spl_transfer, Transfer as SplTransfer, Mint, Token, TokenAccount }
};

use crate::{state::{ Takeover, AdminProfile, Phase::*}, errors::TakeoverError};

#[derive(Accounts)]
pub struct Cleanup<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Box<Account<'info, AdminProfile>>,

    pub new_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub takeover_wallet: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = takeover_wallet,
    )]
    pub new_mint_takeover_wallet_token: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = takeover_wallet
    )]
    pub takeover: Box<Account<'info, Takeover>>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub new_mint_takeover_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Cleanup<'info> {
    pub fn transfer_remaining_amount(&self) -> Result<()> {
        let old_mint_key = self.takeover.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];
        spl_transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                SplTransfer {
                    from: self.new_mint_takeover_vault.to_account_info(),
                    to: self.new_mint_takeover_wallet_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ), self.new_mint_takeover_vault.amount.checked_sub(self.takeover.presale_claimed).ok_or(TakeoverError::Underflow)?.checked_sub(self.takeover.token_swapped).ok_or(TakeoverError::Underflow)?
        )?;

        Ok(())
    }

    pub fn transfer_remaining_sol(&self, bump: u8) -> Result<()> {
        let takeover_key = self.takeover.key();
        let signer_seeds = &[b"takeover_vault", takeover_key.as_ref(), &[bump]];        
        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.takeover_vault.to_account_info(),
                    to: self.takeover_wallet.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover_vault.lamports(),
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<Cleanup>) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        Cleanup => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Change the phase
    ctx.accounts.takeover.phase = ClaimTokens;

    // Check if the takeover_wallet is the correct one
    require!(ctx.accounts.takeover_wallet.key() == ctx.accounts.takeover.takeover_wallet, TakeoverError::InvalidTakeoverWallet);

    // Transfer the remaining amount to the Takeover Wallet
    ctx.accounts.transfer_remaining_amount()?;

    // Transfer the remaining Sol to the Takeover Wallet
    ctx.accounts.transfer_remaining_sol(ctx.bumps.takeover_vault)?;

    Ok(())
}