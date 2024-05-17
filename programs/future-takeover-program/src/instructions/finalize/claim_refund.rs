use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, close_account, thaw_account, transfer, Burn, CloseAccount, ThawAccount, Transfer, Mint, Token, TokenAccount}
};

use crate::{
    errors::TakeoverError, 
    state::{FailedTakeover, PresaleReceipt}
};

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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
    pub takeover: Account<'info, FailedTakeover>,
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub presale_vault: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub old_mint_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"presale_receipt", user.key().as_ref()],
        bump,
    )]
    /// CHECK: This account gets checked during the instruction since it could be uninitialized
    pub presale_receipt: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"swap_receipt", user.key().as_ref()],
        bump,
    )]
    /// CHECK: This account gets checked during the instruction since it could be uninitialized
    pub swap_receipt: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = old_mint,
        associated_token::authority = user,
    )]
    pub user_old_mint_token: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimRefund<'info> {
    pub fn refund_presale(&mut self, amount: u64, bump: u8) -> Result<()> {
        // Refund the presale amount in Sol
        let takeover_key = self.takeover.key().clone();
        let signer_seeds = &[
            b"takeover_vault",
            takeover_key.as_ref(),
            &[bump],
        ];

        system_program::transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(), 
                system_program::Transfer {
                    from: self.presale_vault.to_account_info(),
                    to: self.user.to_account_info(),
                },
            &[signer_seeds],
            ),
            amount.checked_mul(self.takeover.presale_price).ok_or(TakeoverError::Overflow)?,
        )?;

        // Close the presale receipt account
        let dest_starting_lamports = self.user.to_account_info().lamports();
        **self.user.to_account_info().lamports.borrow_mut() = dest_starting_lamports.checked_add(self.presale_receipt.to_account_info().lamports()).unwrap();
        **self.presale_receipt.to_account_info().lamports.borrow_mut() = 0;
    
        self.presale_receipt.to_account_info().assign(&system_program::ID);
        self.presale_receipt.to_account_info().realloc(0, false).map_err::<anchor_lang::prelude::ProgramError, _>(Into::into)?;

        Ok(())
    }

    pub fn refund_swap(&mut self, amount:u64) -> Result<()> {
        // Transfer the old_mint tokens to the user
        let old_mint_key = self.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint_key.as_ref(),
            &[self.takeover.bump],
        ];

        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.old_mint_vault.to_account_info(),
                    to: self.user_old_mint_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<ClaimRefund>) -> Result<()> {
    // Verify if there is a swap receipt account and if there is, refund the presale
    let info = ctx.accounts.presale_receipt.to_account_info();
    let data = info.try_borrow_mut_data()?;

    match  PresaleReceipt::try_deserialize(&mut &data[..]) {
        Ok(presale_receipt ) => {
            // Check if the presale receipt is valid
            require!(presale_receipt.takeover == ctx.accounts.takeover.key(), TakeoverError::InvalidTakeoverData);
            // Check if the presale amount is greater than 0
            require!(presale_receipt.presale_amount > 0, TakeoverError::PresaleAmountZero);

            // Refund the presale amount
            let presale_amount = presale_receipt.presale_amount;
            ctx.accounts.refund_presale(presale_amount, ctx.bumps.presale_vault)?;
        },
        Err(_) => {
        }
    }
    

    // Verify if there is a presale receipt account and if there is, refund the presale
    let info = ctx.accounts.presale_receipt.to_account_info();
    let data = info.try_borrow_mut_data()?;

    match  PresaleReceipt::try_deserialize(&mut &data[..]) {
        Ok(presale_receipt ) => {
            // Check if the presale receipt is valid
            require!(presale_receipt.takeover == ctx.accounts.takeover.key(), TakeoverError::InvalidTakeoverData);
            // Check if the presale amount is greater than 0
            require!(presale_receipt.presale_amount > 0, TakeoverError::PresaleAmountZero);

            // Refund the presale amount
            let presale_amount = presale_receipt.presale_amount;
            ctx.accounts.refund_presale(presale_amount, ctx.bumps.presale_vault)?;
        },
        Err(_) => {
        }
    }

    Ok(())
}