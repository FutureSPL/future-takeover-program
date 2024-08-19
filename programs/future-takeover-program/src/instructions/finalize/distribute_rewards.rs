use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{ transfer_checked as spl_transfer, TransferChecked as SplTransfer, Mint, TokenInterface, TokenAccount }
};

use crate::{
    state::{ Takeover, AdminProfile, Phase::*}, 
    errors::TakeoverError, 
    constant::reward_wallet
};

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
        associated_token::token_program = token_program,
    )]
    pub new_mint_takeover_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
    pub reward_wallet: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = reward_wallet,
        associated_token::token_program = token_program,
    )]
    pub new_mint_reward_wallet_token: Box<InterfaceAccount<'info, TokenAccount>>,
    pub referral_wallet: Option<SystemAccount<'info>>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = referral_wallet,
        associated_token::token_program = token_program,
    )]
    pub new_mint_referral_wallet_token: Option<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
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
                    mint: self.new_mint.to_account_info(),
                    to: self.new_mint_referral_wallet_token.as_ref().unwrap().to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover.inflation_amount.referral_amount,
            self.new_mint.decimals
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
                    mint: self.new_mint.to_account_info(),
                    to: self.new_mint_reward_wallet_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            self.takeover.inflation_amount.rewards_amount,
            self.new_mint.decimals
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<DistributeRewards>) -> Result<()> {
    // Check if it's the right phase
    // match ctx.accounts.takeover.phase {
    //     RewardDistribution => (),
    //     _ => return Err(TakeoverError::InvalidPhase.into()),
    // } - To be added later

    // Set the phase to the next phase
    ctx.accounts.takeover.phase = Cleanup;

    // Check if the reward_wallet is the correct one
    // require_eq!(
    //     ctx.accounts.reward_wallet.key(), 
    //     reward_wallet::id(), 
    //     TakeoverError::InvalidRewardWallet
    // );

    // Check if the referral if it exists and is the correct one
    if ctx.accounts.takeover.referral.is_some() {
        require_eq!(
            ctx.accounts.takeover.referral.unwrap().key(), 
            ctx.accounts.referral_wallet.as_ref().unwrap().key(), 
            TakeoverError::InvalidReferralWallet
        );
        ctx.accounts.transfer_referral_amount()?;
    } else {
        require!(ctx.accounts.takeover.referral.is_none(), TakeoverError::InvalidReferralWallet);
    }

    // Transfer the Rewards amount to the Reward Wallet
    ctx.accounts.transfer_reward_amount()?;

    Ok(())
}