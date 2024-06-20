use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer as spl_transfer, Transfer as SplTransfer, Mint, Token, TokenAccount }
};

use crate::{state::{ Takeover, AdminProfile, Phase::*}, errors::TakeoverError, constant::reward_wallet};

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Box<Account<'info, AdminProfile>>,

    pub new_mint: Box<Account<'info, Mint>>,
    pub reward_wallet: SystemAccount<'info>,
    pub referral_wallet: Option<SystemAccount<'info>>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = reward_wallet,
    )]
    pub new_mint_reward_wallet_token: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = referral_wallet,
    )]
    pub new_mint_referral_wallet_token: Option<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Box<Account<'info, Takeover>>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub new_mint_takeover_vault: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> DistributeRewards<'info> {
    pub fn transfer_referral_amount(&self) -> Result<()> {
        let old_mint_key = self.takeover.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];
        spl_transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                SplTransfer {
                    from: self.new_mint_takeover_vault.to_account_info(),
                    to: self.new_mint_referral_wallet_token.as_ref().unwrap().to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover.inflation_amount.referral_amount,
        )?;

        Ok(())
    }

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
}

pub fn handler(ctx: Context<DistributeRewards>) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        RewardDistribution => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Set the phase to the next phase
    ctx.accounts.takeover.phase = Cleanup;

    // Check if the reward_wallet is the correct one
    require!(ctx.accounts.reward_wallet.key() == reward_wallet::id(), TakeoverError::InvalidRewardWallet);

    // Check if the referral if it exists and is the correct one
    if ctx.accounts.takeover.referral.is_some() {
        require!(ctx.accounts.takeover.referral.unwrap().key() == ctx.accounts.referral_wallet.as_ref().unwrap().key(), TakeoverError::InvalidReferralWallet);
        ctx.accounts.transfer_referral_amount()?;
    }

    // Transfer the Rewards amount to the Reward Wallet
    ctx.accounts.transfer_reward_amount()?;

    Ok(())
}