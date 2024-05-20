use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::token::{TokenAccount, Mint, thaw_account, ThawAccount, Token};

use crate::{
    errors::TakeoverError, state::{ Takeover, PresaleReceipt, Phase::*}
};

#[derive(Accounts)]
pub struct UnfreezeToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub new_mint: Account<'info, Mint>,
    #[account(
        mut,
        token::mint = new_mint,
        token::authority = user,
    )]
    pub user_token: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = new_mint,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        seeds = [b"presale_receipt", user.key().as_ref()],
        bump,
    )]
    /// CHECK: This account gets checked during the instruction since it could be uninitialized
    pub presale_receipt: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> UnfreezeToken<'info> {
    pub fn unfreeze_token(&mut self) -> Result<()> {
        // Thaw the account
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];

        thaw_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                ThawAccount {
                    account: self.user_token.to_account_info(),
                    mint: self.new_mint.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            )
        )?;

        Ok(())
    }

    pub fn close_presale_receipt(&mut self) -> Result<()> {
        let dest_starting_lamports = self.user.to_account_info().lamports();
        **self.user.to_account_info().lamports.borrow_mut() = dest_starting_lamports.checked_add(self.presale_receipt.to_account_info().lamports()).unwrap();
        **self.presale_receipt.to_account_info().lamports.borrow_mut() = 0;
    
        self.presale_receipt.to_account_info().assign(&system_program::ID);
        self.presale_receipt.to_account_info().realloc(0, false).map_err::<anchor_lang::prelude::ProgramError, _>(Into::into)?;

        Ok(())
    }
    
}

pub fn handler(ctx: Context<UnfreezeToken>) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        UnlockingAta => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Check if there are tokens in the TokenAccount
    require!(ctx.accounts.user_token.amount > 0, TakeoverError::TokenEmpty);

    // Unfreeze the token
    ctx.accounts.unfreeze_token()?;

    // Verify if there is a presale receipt account and if there is, close that account
    let info = ctx.accounts.presale_receipt.to_account_info();
    let data = info.try_borrow_mut_data()?;

    match  PresaleReceipt::try_deserialize(&mut &data[..]) {
        Ok(presale_receipt ) => {
            // Check if the presale receipt is valid
            require!(presale_receipt.takeover == ctx.accounts.takeover.key(), TakeoverError::InvalidTakeoverData);
            // // Check if the presale amount is greater than 0
            // require!(presale_receipt.presale_amount > 0, TakeoverError::PresaleAmountZero);

            // Close the presale receipt account to refund the rent
            ctx.accounts.close_presale_receipt()?;

            // // Change the presale amount to 0 so it can't be refunded again, and serialize it back
            // presale_receipt.presale_amount = 0;
            // let mut writer = &mut data[..];
            // presale_receipt.try_serialize(&mut writer)?;

        },
        Err(_) => {
            // Do nothing
        }
    }

    Ok(())
}