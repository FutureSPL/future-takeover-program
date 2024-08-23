use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ transfer_checked, TransferChecked, Mint, TokenInterface, TokenAccount}
};

use crate::{
    errors::TakeoverError, 
    state::{Takeover, PresaleReceipt, SwapReceipt, Phase::*},
};

#[derive(Accounts)]
pub struct ClaimTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mints.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = new_mint,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        seeds = [b"presale_receipt", takeover.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    /// CHECK: This account gets checked during the instruction since it could be uninitialized
    pub presale_receipt: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"swap_receipt", takeover.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    /// CHECK: This account gets checked during the instruction since it could be uninitialized
    pub swap_receipt: UncheckedAccount<'info>,
    #[account(mut)]
    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
        associated_token::token_program = token_program,
    )]
    pub takeover_new_mint_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = new_mint,
        associated_token::authority = user,
        associated_token::token_program = token_program,
    )]
    pub user_new_mint_token: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimTokens<'info> {
    pub fn claim_tokens(&mut self, amount: u64) -> Result<()> {
        let old_mint_key = self.takeover.old_mints.old_mint.clone();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.takeover_new_mint_vault.to_account_info(),
                    mint: self.new_mint.to_account_info(),
                    to: self.user_new_mint_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
            self.new_mint.decimals,
        )?;
        
        Ok(())
    }
}

pub fn handler(ctx: Context<ClaimTokens>) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        ClaimTokens => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Verify if there is a swap receipt account and if there is, refund the presale
    let info = ctx.accounts.swap_receipt.to_account_info();
    let mut data = info.try_borrow_mut_data()?;

    match  SwapReceipt::try_deserialize(&mut &data[..]) {
        Ok(mut swap_receipt ) => {
            // Check if the takeover in the swap_receipt is the same as the current takeover
            require_eq!(
                swap_receipt.takeover,
                ctx.accounts.takeover.key(), 
                TakeoverError::InvalidTakeoverData
            );

            // Check if the swapped amount is greater than 0
            require_gt!(
                swap_receipt.swapped_amount,
                0, 
                TakeoverError::PresaleAmountZero
            );

            // Refund the presale amount
            let swapped_amount = swap_receipt.swapped_amount;
            ctx.accounts.claim_tokens(swapped_amount)?;

            // Update the amount to 0
            swap_receipt.swapped_amount = 0;

            // Serialize it back and update the account
            let mut writer = &mut data[..];
            swap_receipt.try_serialize(&mut writer)?;

        },
        Err(_) => {
            // Do nothing
        }
    }
    
    // Verify if there is a presale receipt account and if there is, refund the presale
    let info = ctx.accounts.presale_receipt.to_account_info();
    let mut data = info.try_borrow_mut_data()?;

    match  PresaleReceipt::try_deserialize(&mut &data[..]) {
        Ok(mut presale_receipt ) => {
            // Check if the takeover in the swap_receipt is the same as the current takeover
            require_eq!(
                presale_receipt.takeover,
                ctx.accounts.takeover.key(), 
                TakeoverError::InvalidTakeoverData
            );

            // Check if the swapped amount is greater than 0
            require_gt!(
                presale_receipt.presale_amount,
                0, 
                TakeoverError::PresaleAmountZero
            );

            // Refund the presale amount
            let presale_amount = presale_receipt.presale_amount;
            ctx.accounts.claim_tokens(presale_amount)?;

            // Update the amount to 0
            presale_receipt.presale_amount = 0;

            // Serialize it back and update the account
            let mut writer = &mut data[..];
            presale_receipt.try_serialize(&mut writer)?;
        },
        Err(_) => {
            // Do nothing
        }
    }

    Ok(())
}