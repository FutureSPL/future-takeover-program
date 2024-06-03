use anchor_lang::{
    prelude::*, 
    system_program::{transfer, Transfer}
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer as spl_transfer, Transfer as SplTransfer, close_account, CloseAccount, Mint, Token, TokenAccount }
};

use crate::{
    state::{ Takeover, AdminProfile, Phase::*},
    errors::TakeoverError,
    constant::reward_wallet,
};

#[derive(Accounts)]
pub struct Cleanup<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_profile", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub new_mint: Account<'info, Mint>,
    #[account(mut)]
    pub takeover_wallet: SystemAccount<'info>,
    #[account(mut, address = reward_wallet::ID)]
    pub reward_wallet: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = reward_wallet,
    )]
    pub new_mint_reward_wallet_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = takeover_wallet,
    )]
    pub new_mint_takeover_wallet_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = takeover_wallet
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        associated_token::mint = takeover.new_mint,
        associated_token::authority = takeover,
    )]
    pub new_mint_takeover_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Cleanup<'info> {
    pub fn transfer_reward_amount(&self) -> Result<()> {
        let old_mint_key = self.takeover.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];
        spl_transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                SplTransfer {
                    from: self.new_mint_takeover_vault.to_account_info(),
                    to: self.new_mint_reward_wallet_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover.inflation_amount.rewards_amount,
        )?;

        Ok(())
    }

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
            ),self.new_mint_takeover_vault.amount,
        )?;

        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                CloseAccount {
                    account: self.new_mint_takeover_vault.to_account_info(),
                    destination: self.takeover_wallet.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            )
        )?;

        Ok(())
    }

    pub fn transfer_remaining_sol(&self) -> Result<()> {
        let old_mint_key = self.takeover.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];        
        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.takeover_sol_vault.to_account_info(),
                    to: self.takeover_wallet.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover_sol_vault.lamports(),
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

    // Transfer the Rewards amount to the Reward Wallet
    ctx.accounts.transfer_reward_amount()?;

    // Transfer the remaining amount to the Takeover Wallet
    ctx.accounts.transfer_remaining_amount()?;

    // Transfer the remaining Sol to the Takeover Wallet
    ctx.accounts.transfer_remaining_sol()?;

    Ok(())
}