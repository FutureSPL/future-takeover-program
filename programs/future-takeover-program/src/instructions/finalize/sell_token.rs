use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::{
        sysvar::{
            self,
            instructions::{load_current_index_checked, load_instruction_at_checked},
        },
        instruction::Instruction,
    }
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer, Transfer, Mint, Token, TokenAccount }
};

use jupiter_sdk::i11n::RouteI11n;
use future_takeover_program_sdk::i11n::FinalizeSellI11n;

use crate::{
    state::{ Takeover, AdminProfile, Phase::*},
    errors::TakeoverError,
};

#[derive(Accounts)]
pub struct SellToken<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub old_mint: Box<Account<'info, Mint>>,
    #[account( address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap())]
    pub wsol: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = old_mint,
        associated_token::authority = admin,
    )]
    pub old_mint_admin_token: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = wsol,
        associated_token::authority = admin,
    )]
    pub wsol_admin_token: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"takeover", old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = old_mint,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        associated_token::mint = old_mint,
        associated_token::authority = takeover,
    )]
    pub old_mint_takeover_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_sol_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: InstructionsSysvar account
    #[account(address = sysvar::instructions::ID)]
    instructions: UncheckedAccount<'info>,
}

impl<'info> SellToken<'info> {
    fn receieve_old_token(&mut self, amount: u64) -> Result<()> {
        // Transfer the old tokens from the vault to the user
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];

        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.old_mint_takeover_vault.to_account_info(),
                    to: self.old_mint_admin_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        Ok(())
    }

    fn introspect_swap(&mut self, amount: u64, swap_ix: Instruction) -> Result<()> {

        // Checking that this is the right Swap Instruction
        let instruction = RouteI11n::try_from(&swap_ix).unwrap();

        // Checking the Args like: in_amount = amount
        require_eq!(instruction.args.in_amount, amount, TakeoverError::InvalidSwapAmount);

        // Checking the Accounts like: source Token and TokenAccount + destination Token and TokenAccount
        require_eq!(instruction.accounts.user_source_token_account.pubkey, self.old_mint_admin_token.key(), TakeoverError::InvalidSwapSourceTokenAccount);
        require_eq!(instruction.accounts.destination_mint.pubkey, self.wsol.key(), TakeoverError::InvalidSwapDestinationMint);
        require_eq!(instruction.accounts.user_destination_token_account.pubkey, self.wsol_admin_token.key(), TakeoverError::InvalidSwapDestinationTokenAccount);

        Ok(())
    }

    fn introspect_finalize_sell(&mut self, finalize_sell_ix: Instruction) -> Result<()> {
        // Checking that this is the right Finalize_sell Instruction
        let instruction = FinalizeSellI11n::try_from(&finalize_sell_ix).unwrap();

        // Checking the Accounts
        require_eq!(instruction.accounts.admin.pubkey, self.admin.key(), TakeoverError::InvalidFinalizeSellAdmin);
        require_eq!(instruction.accounts.takeover.pubkey, self.takeover.key(), TakeoverError::InvalidFinalizeSellTakeover);

        Ok(())
    }
}

pub fn handler(ctx: Context<SellToken>, amount: u64) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        TokenSelling => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Check if the amount requested is valid
    require!(amount > 0, TakeoverError::InvalidAmount);

    // Check if the Vault has enough tokens, if it's all the amount, then move to the next phase
    if ctx.accounts.old_mint_takeover_vault.amount == amount {
        ctx.accounts.takeover.phase = MarketCreation;
    } else if ctx.accounts.old_mint_takeover_vault.amount < amount {
        return Err(TakeoverError::InsufficientFunds.into());
    }

    // Receive the old tokens
    ctx.accounts.receieve_old_token(amount)?;

    // Setup for Introspection
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // Load & Check the Swap Instruction
    let swap_ix = load_instruction_at_checked(current_index + 1, &ixs).map_err(|_| TakeoverError::MissingSwapIx)?;
    ctx.accounts.introspect_swap(amount, swap_ix)?;

    // Load & Check the Finalize_sell Instruction
    let finalize_sell_ix = load_instruction_at_checked(current_index + 2, &ixs).map_err(|_| TakeoverError::MissingFinalizeSellIx)?;
    ctx.accounts.introspect_finalize_sell(finalize_sell_ix)?;
    
    Ok(())
}