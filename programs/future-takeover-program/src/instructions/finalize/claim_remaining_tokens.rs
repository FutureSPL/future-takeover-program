use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ transfer_checked as spl_transfer, TransferChecked as SplTransfer, Mint, TokenInterface, TokenAccount }
};

use crate::{state::{ Takeover, AdminProfile, Phase::*}, errors::TakeoverError};

#[derive(Accounts)]
pub struct ClaimRemainingTokens<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Box<Account<'info, AdminProfile>>,

    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub takeover_wallet: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = takeover_wallet,
    )]
    pub new_mint_takeover_wallet_token: Box<InterfaceAccount<'info, TokenAccount>>,
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
    pub new_mint_takeover_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program:  Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimRemainingTokens<'info> {
    pub fn transfer_remaining_amount(&self) -> Result<()> {
        let old_mint_key = self.takeover.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];
        
        spl_transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                SplTransfer {
                    from: self.new_mint_takeover_vault.to_account_info(),
                    mint: self.new_mint.to_account_info(),
                    to: self.new_mint_takeover_wallet_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ), 
            self.new_mint_takeover_vault.amount,
            self.new_mint.decimals
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<ClaimRemainingTokens>) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        ClaimTokens => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Check if the takeover_wallet is the correct one
    require!(ctx.accounts.takeover_wallet.key() == ctx.accounts.takeover.takeover_wallet, TakeoverError::InvalidTakeoverWallet);

    // Transfer the remaining amount to the Takeover Wallet
    ctx.accounts.transfer_remaining_amount()?;

    Ok(())
}