use anchor_lang::{
    prelude::*, 
    system_program::{transfer, Transfer}
};

use anchor_spl::{
    associated_token::{create, get_associated_token_address_with_program_id, AssociatedToken, Create},
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked}
};

use crate::{
    errors::TakeoverError, 
    state::{Takeover, PresaleReceipt, SwapReceipt, Phase::*},
};

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
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
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_vault: SystemAccount<'info>,
    pub new_mint: Box<InterfaceAccount<'info, Mint>>,
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
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimRefund<'info> {
    // Convert the amount to its non-decimal form and refund the presale amount
    pub fn refund_presale(&mut self, amount: u64, bump: u8) -> Result<()> {
        let takeover_key = self.takeover.key();
        let signer_seeds = &[b"takeover_vault", takeover_key.as_ref(), &[bump]];

        let decimals_factor = 10u64
            .checked_pow(self.new_mint.decimals as u32)
            .ok_or(TakeoverError::Overflow)?;
        let amount_in_sol = amount
            .checked_div(decimals_factor)
            .ok_or(TakeoverError::Overflow)?;

        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.takeover_vault.to_account_info(),
                    to: self.user.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount_in_sol
                .checked_mul(self.takeover.presale_price)
                .ok_or(TakeoverError::Overflow)?,
        )?;

        Ok(())
    }

    pub fn refund_swap(&mut self, chunk: &[AccountInfo<'info>], mut swap_receipt: SwapReceipt) -> Result<SwapReceipt> {

        let old_mints_accounts = (&chunk[0], &chunk[1], &chunk[2]);
        let (old_mint, user_old_mint_token, takeover_old_mint_token) = old_mints_accounts;

        // Check if the mint is correct, and if it is, calculate the swapped_amount
        let mint_data = old_mint.try_borrow_data()?;
        let mint = Mint::try_deserialize(&mut &mint_data[..])
            .map_err(|_| TakeoverError::InvalidMint)?;

        drop(mint_data);

        require_eq!(
            takeover_old_mint_token.key(), 
            get_associated_token_address_with_program_id(&self.takeover.key(), old_mint.key, &self.token_program.key()),
            TakeoverError::InvalidAssociatedToken
        );
        let takeover_token_data = takeover_old_mint_token.try_borrow_mut_data()?;
        TokenAccount::try_deserialize(&mut &takeover_token_data[..])
            .map_err(|_| TakeoverError::InvalidAssociatedToken)?;

        drop(takeover_token_data);

        let user_token_data = user_old_mint_token.try_borrow_mut_data()?;
        let user_token = TokenAccount::try_deserialize(&mut &user_token_data[..]);

        drop(user_token_data);
        
        match user_token {
            Ok(_) => (
                // Do nothing
            ),
            Err(_) => {
                // Create the Token Account
                create(
                    CpiContext::new(
                        self.associated_token_program.to_account_info(),
                        Create {
                            associated_token: user_old_mint_token.to_account_info(),
                            authority: self.user.to_account_info(),
                            payer: self.user.to_account_info(),
                            mint: old_mint.to_account_info(),
                            system_program: self.system_program.to_account_info(),
                            token_program: self.token_program.to_account_info(),
                        },
                    ),
                )?;
            }
        }

        let old_mint_key = self.takeover.old_mints.old_mint.key();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];

        let amount = match old_mint.key() {
            key if key == self.takeover.old_mints.old_mint => {
                let amount = swap_receipt.swapped_old_mints.amount;
                swap_receipt.swapped_old_mints.amount = 0;
                amount
            }
            key if self.takeover.old_mints.old_mint_2.is_some() && key == self.takeover.old_mints.old_mint_2.unwrap() => {
                let amount = swap_receipt.swapped_old_mints.amount_2.ok_or(TakeoverError::InvalidMint)?;
                swap_receipt.swapped_old_mints.amount_2 = Some(0);
                amount
            }
            key if self.takeover.old_mints.old_mint_3.is_some() && key == self.takeover.old_mints.old_mint_3.unwrap() => {
                let amount = swap_receipt.swapped_old_mints.amount_3.ok_or(TakeoverError::InvalidMint)?;
                swap_receipt.swapped_old_mints.amount_3 = Some(0);
                amount
            }
            _ => return Err(TakeoverError::InvalidMint.into()),
        };

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: takeover_old_mint_token.to_account_info(),
                    mint: old_mint.to_account_info(),
                    to: user_old_mint_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
            mint.decimals,
        )?;

        Ok(swap_receipt)
    }
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, ClaimRefund<'info>>) -> Result<()> {
    // Check if it's the right phase
    // match ctx.accounts.takeover.phase {
    //     FailedTakeover => (),
    //     _ => return Err(TakeoverError::InvalidPhase.into()),
    // } - To be added later

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

            // Refund the swap amount for each mint
            for chunk in ctx.remaining_accounts.chunks(3) {
                require!(chunk.len() == 3, TakeoverError::InvalidRemainingAccountSchema);
        
                swap_receipt = ctx.accounts.refund_swap(chunk, swap_receipt)?;   
            }

            // Check if we have refunded everything and if we have, update the swapped amount to 0
            require!(
                swap_receipt.swapped_old_mints.amount == 0 &&
                swap_receipt.swapped_old_mints.amount_2.unwrap_or(0) == 0 &&
                swap_receipt.swapped_old_mints.amount_3.unwrap_or(0) == 0,
                TakeoverError::NonZeroAmountsRemain
            );
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
            ctx.accounts.refund_presale(presale_amount, ctx.bumps.takeover_vault)?;

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