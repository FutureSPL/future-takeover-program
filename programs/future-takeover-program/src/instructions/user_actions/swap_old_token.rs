use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken, create, Create}, 
    token_interface::{ transfer_checked, TokenAccount, Mint, TokenInterface, TransferChecked, close_account, CloseAccount } 
};

use crate::{
    errors::TakeoverError, state::{SwapReceipt, Takeover}, takeover
};

#[derive(Accounts)]
pub struct SwapOldToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mints.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        init_if_needed,
        payer = user,
        space = SwapReceipt::INIT_SPACE,
        seeds = [b"swap_receipt", takeover.key().as_ref(), user.key().as_ref()],
        bump,
    )]
    pub swap_receipt: Account<'info, SwapReceipt>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> SwapOldToken<'info> {
    fn update_receipt(&mut self, swapped_amount: u64, bump: u8) -> Result<()> {
        self.swap_receipt.takeover = self.takeover.key();
        self.swap_receipt.swapped_amount = self.swap_receipt.swapped_amount
            .checked_add(swapped_amount)
            .ok_or(TakeoverError::Overflow)?;
        self.swap_receipt.bump = bump;
    
        Ok(())
    }

    fn swap_old_mints(&mut self, chunk: &[AccountInfo<'info>], mut swapped_amount: u64) -> Result<u64> {
        let old_mints_accounts = (&chunk[0], &chunk[1], &chunk[2]);
        let (old_mint, user_old_mint_token, takeover_old_mint_token) = old_mints_accounts;

        // Check if the mint is correct, and if it is, calculate the swapped_amount
        let mint_data = old_mint.try_borrow_data()?;
        let mint = Mint::try_deserialize(&mut &mint_data[..])
            .map_err(|_| TakeoverError::InvalidMint)?;

        drop(mint_data);

        let user_token_data = user_old_mint_token.try_borrow_data()?;
        let user_token = TokenAccount::try_deserialize(&mut &user_token_data[..])
            .map_err(|_| TakeoverError::InvalidAssociatedToken)?;
        require_gt!(user_token.amount, 0, TakeoverError::InvalidAmount);

        drop(user_token_data);

        let takeover_token_data = takeover_old_mint_token.try_borrow_data()?;
        let takeover_token = TokenAccount::try_deserialize(&mut &takeover_token_data[..]);

        drop(takeover_token_data);
        
        match takeover_token {
            Ok(_) => (
                // Do nothing
            ),
            Err(_) => {
                // Create the Token Account
                create(
                    CpiContext::new(
                        self.associated_token_program.to_account_info(),
                        Create {
                            associated_token: takeover_old_mint_token.to_account_info(),
                            authority: self.takeover.to_account_info(),
                            payer: self.user.to_account_info(),
                            mint: old_mint.to_account_info(),
                            system_program: self.system_program.to_account_info(),
                            token_program: self.token_program.to_account_info(),
                        },
                    ),
                )?;
            }
        }

        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: user_old_mint_token.to_account_info(),
                    mint: old_mint.to_account_info(),
                    to: takeover_old_mint_token.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            user_token.amount,
            mint.decimals,
        )?;

        close_account(
            CpiContext::new(
                self.token_program.to_account_info(),
                CloseAccount {
                    account: user_old_mint_token.to_account_info(),
                    destination: self.user.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
        )?;

        let weight = match old_mint.key() {
            key if key == self.takeover.old_mints.old_mint => {
                self.swap_receipt.swapped_old_mints.amount = self.swap_receipt.swapped_old_mints.amount
                    .checked_add(user_token.amount)
                    .ok_or(TakeoverError::Overflow)?;
                self.takeover.old_mints.weight_percentage
                    .ok_or(TakeoverError::WeightedPercentageNotFound)?
            }
            key if self.takeover.old_mints.old_mint_2.is_some() && key == self.takeover.old_mints.old_mint_2.unwrap() => {
                self.swap_receipt.swapped_old_mints.amount_2 = Some(self.swap_receipt.swapped_old_mints.amount_2
                    .unwrap_or(0)
                    .checked_add(user_token.amount)
                    .ok_or(TakeoverError::Overflow)?);
                self.takeover.old_mints.weight_percentage_2
                    .ok_or(TakeoverError::WeightedPercentageNotFound)?
            }
            key if self.takeover.old_mints.old_mint_3.is_some() && key == self.takeover.old_mints.old_mint_3.unwrap() => {
                self.swap_receipt.swapped_old_mints.amount_3 = Some(self.swap_receipt.swapped_old_mints.amount_3
                    .unwrap_or(0)
                    .checked_add(user_token.amount)
                    .ok_or(TakeoverError::Overflow)?);
                self.takeover.old_mints.weight_percentage_3
                    .ok_or(TakeoverError::WeightedPercentageNotFound)?
            }
            _ => return Err(TakeoverError::InvalidMint.into()),
        };

        let weighted_amount = user_token.amount
            .checked_div(100)
            .ok_or(TakeoverError::Underflow)?
            .checked_mul(weight as u64)
            .ok_or(TakeoverError::Overflow)?;

        swapped_amount = swapped_amount
            .checked_add(weighted_amount)
            .ok_or(TakeoverError::Overflow)?;

        Ok(swapped_amount)
    }

}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, SwapOldToken<'info>>) -> Result<()> {
    // Check that the takeover is already started and the swap period is active
    // require_lt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.start, TakeoverError::SwapPeriodNotStarted); - To be added back
    // require_gt!(Clock::get()?.unix_timestamp, ctx.accounts.takeover.swap_period.end, TakeoverError::SwapPeriodEnded); - To be added back

    let mut swapped_amount = 0u64; 

    for chunk in ctx.remaining_accounts.chunks(3) {
        require!(chunk.len() == 3, TakeoverError::InvalidRemainingAccountSchema);

        swapped_amount = ctx.accounts.swap_old_mints(chunk, swapped_amount)?;   
    }

    // Update the token swapped amount
    ctx.accounts.takeover.token_swapped = ctx.accounts.takeover.token_swapped
        .checked_add(swapped_amount)
        .ok_or(TakeoverError::Overflow)?;

    // Initialize the swap receipt
    ctx.accounts.update_receipt(swapped_amount, ctx.bumps.swap_receipt)?;

    Ok(())
}