use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::sysvar::{
        self,
        instructions::{load_current_index_checked, load_instruction_at_checked},
    },
    system_program::{transfer, Transfer},
};

use anchor_spl::token::{ Mint, Token, TokenAccount, close_account, CloseAccount };

use jupiter_sdk::i11n::SharedAccountsRouteI11n;
use future_takeover_program_sdk::i11n::SellTokenI11n;

use crate::state::{Takeover, AdminProfile};

#[derive(Accounts)]
pub struct FinalizeSellToken<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_profile", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account( address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap())]
    pub wsol: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = wsol,
        associated_token::authority = admin,
    )]
    pub wsol_admin_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: InstructionsSysvar account
    #[account(address = sysvar::instructions::ID)]
    instructions: UncheckedAccount<'info>,
}

impl<'info> FinalizeSellToken<'info> {
    fn cleanup(&mut self, amount: u64) -> Result<()> {
        // Close wSOL account and send to the admin
        close_account(
            CpiContext::new(
                self.token_program.to_account_info(), 
                CloseAccount {
                    account: self.wsol_admin_token.to_account_info(),
                    destination: self.admin.to_account_info(),
                    authority: self.admin.to_account_info(),
                }
            )
        )?;

        // Send the SOL to the vault
        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.admin.to_account_info(),
                    to: self.takeover_sol_vault.to_account_info(),
                }
            ),
            amount
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<FinalizeSellToken>) -> Result<()> {
    // Setup for Introspection
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // Load and Check the SellToken Instruction
    let sell_token_ix = load_instruction_at_checked( current_index - 2, &ixs)?;
    let _sell_token_instruction_check = SellTokenI11n::try_from(&sell_token_ix).unwrap();

    // Load the Swap Instruction and grap the Sol Amount
    let swap_ix = load_instruction_at_checked( current_index - 1, &ixs)?;
    let swap_instruction = SharedAccountsRouteI11n::try_from(&swap_ix).unwrap();
    let swapped_amount = swap_instruction.args.quoted_out_amount;

    // Close the wSol Account and send it to the vault
    ctx.accounts.cleanup(swapped_amount)?;

    Ok(())
}