use anchor_lang::{prelude::*, solana_program::program_memory::sol_memcpy};
use anchor_spl::token::Mint;

use crate::{
    errors::TakeoverError, state::{AdminProfile, Takeover, SuccessfulTakeover, FailedTakeover, Level::*, Phase::* }, constant::ADMIN_BUFFER
};

#[derive(Accounts)]
pub struct FinalizeTakeover<'info> {
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
        has_one = new_mint,
        has_one = old_mint,
    )]
    pub takeover: Account<'info, Takeover>,

    #[account(mut)]
    pub old_mint: Account<'info, Mint>,
    #[account(mut)]
    pub new_mint: Account<'info, Mint>,    

    pub system_program: Program<'info, System>,
}

impl<'info> FinalizeTakeover<'info> {
    pub fn migrate_to_successful(&mut self) -> Result<()> {
        // Get the original Data from the account
        let info = self.takeover.to_account_info(); 
        let mut data = info.try_borrow_mut_data()?;
        
        // Transform to SuccessfulTakeover
        let successful_takeover = SuccessfulTakeover {
            old_mint: self.old_mint.key(),
            new_mint: self.new_mint.key(),
            phase: TokenSelling,
            bump: self.takeover.bump,
        };

        // Serialize
        let mut writer: Vec<u8> = vec![];
        successful_takeover.try_serialize(&mut writer)?;

        let padding_len = Takeover::INIT_SPACE - SuccessfulTakeover::INIT_SPACE; 
        writer.extend_from_slice(&vec![0; padding_len]);

        // Copy back to original Account
        sol_memcpy(&mut data, &writer, writer.len());
        
        Ok(())
    }

    pub fn migrate_to_failed(&mut self) -> Result<()> {
        // Get the original Data from the account
        let info = self.takeover.to_account_info(); 
        let mut data = info.try_borrow_mut_data()?;
        
        // Transform to FailedTakeover
        let failed_takeover = FailedTakeover {
            old_mint: self.takeover.old_mint,
            new_mint: self.takeover.new_mint,
            presale_price: self.takeover.presale_price,
            bump: self.takeover.bump,
        };

        // Serialize
        let mut writer: Vec<u8> = vec![];
        failed_takeover.try_serialize(&mut writer)?;

        let padding_len = Takeover::INIT_SPACE - FailedTakeover::INIT_SPACE; 
        writer.extend_from_slice(&vec![0; padding_len]);

        // Copy back to original Account
        sol_memcpy(&mut data, &writer, writer.len());

        Ok(())
    }
    
}

pub fn handler(ctx: Context<FinalizeTakeover>) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(ctx.accounts.admin_profile.creation_time - Clock::get()?.unix_timestamp > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check that the takeover is already started and the swap period is active
    // require!(ctx.accounts.takeover.swap_period.end < Clock::get()?.unix_timestamp, TakeoverError::SwapPeriodNotEnded);

    // Check if the presale is successful, then migrate to successful or unsuccessful takeover account
    let presale_amount = ctx.accounts.old_mint.supply.checked_mul(ctx.accounts.takeover.inflation_amount.presale_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;
    let success_amount = presale_amount.checked_mul(60).ok_or(TakeoverError::Overflow)?.checked_div(100).ok_or(TakeoverError::Underflow)?;
    
    match ctx.accounts.takeover.inflation_amount.level {
        Low => {
            if success_amount < ctx.accounts.takeover.presale_claimed {
                ctx.accounts.migrate_to_successful()?;
            } else {
                ctx.accounts.migrate_to_failed()?;
            }
        }
        Medium => {
            if success_amount < ctx.accounts.takeover.presale_claimed {
                ctx.accounts.migrate_to_successful()?;
            } else {
                ctx.accounts.migrate_to_failed()?;
            }
        }
        High => {
            if success_amount < ctx.accounts.takeover.presale_claimed {
                ctx.accounts.migrate_to_successful()?;
            } else {
                ctx.accounts.migrate_to_failed()?;
            }
        }
    }

    Ok(())
}