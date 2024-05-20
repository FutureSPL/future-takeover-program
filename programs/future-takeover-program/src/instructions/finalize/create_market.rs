use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction, 
        sysvar::{
            self,
            instructions::{load_current_index_checked, load_instruction_at_checked},
        }
    },
    system_program
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer, Transfer, sync_native, SyncNative, Mint, Token, TokenAccount }
};

use raydium_cp_swap_sdk::i11n::InitializeI11n;

use crate::{
    state::{ Takeover, AdminProfile, Phase::*},
    errors::TakeoverError,
};

pub struct CreateMarketArgs {
    pub amount_token_0: u64,
    pub amount_token_1: u64,
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin_profile", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub new_mint: Account<'info, Mint>,
    #[account( address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap())]
    pub wsol: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = admin,
    )]
    pub new_mint_admin_token: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = wsol,
        associated_token::authority = admin,
    )]
    pub wsol_admin_token: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
        has_one = new_mint,
    )]
    pub takeover: Account<'info, Takeover>,
    #[account(
        mut,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub new_mint_takeover_vault: Account<'info, TokenAccount>,
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

impl<'info> CreateMarket<'info> {
    fn wrap_sol(&mut self, amount: u64) -> Result<()> {
        // transfer sol to token account
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];

        system_program::transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
            system_program::Transfer {
                    from: self.takeover_sol_vault.to_account_info(),
                    to: self.wsol_admin_token.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        // Sync the native token to reflect the new SOL balance as wSOL
        sync_native(
            CpiContext::new(
                self.token_program.to_account_info(),
                SyncNative {
                    account: self.wsol_admin_token.to_account_info(),
                }
            )
        )?;

        Ok(())  
    }

    fn receieve_market_creation_pair(&mut self, amount: u64) -> Result<()> {
        // Transfer the old tokens from the vault to the user
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[
            b"takeover",
            old_mint.as_ref(),
            &[self.takeover.bump],
        ];

        // Transfer the new tokens from the vault to the user
        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.new_mint_takeover_vault.to_account_info(),
                    to: self.new_mint_admin_token.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        Ok(())
    }

    fn introspect_initialize(&mut self, amount_token_0: u64, amount_token_1: u64, initialize: Instruction) -> Result<()> {

        // Checking that this is the right Swap Instruction
        let instruction = InitializeI11n::try_from(&initialize).unwrap();

        // Checking the Args
        require_eq!(instruction.args.init_amount_0, amount_token_0, TakeoverError::InvalidInitializeAmount);
        require_eq!(instruction.args.init_amount_1, amount_token_1, TakeoverError::InvalidInitializeAmount);

        // Checking the Accounts
        require_eq!(instruction.accounts.token_0_mint.pubkey, self.wsol.key(), TakeoverError::InvalidInitializeMintAccount);
        require_eq!(instruction.accounts.token_1_mint.pubkey, self.new_mint.key(), TakeoverError::InvalidInitializeMintAccount);

        Ok(())
    }

}

pub fn handler(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        MarketCreation => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Change the phase
    ctx.accounts.takeover.phase = Cleanup;

    // Check if the amount requested is valid
    require!(args.amount_token_0 > 0 && args.amount_token_1 > 0, TakeoverError::InvalidAmount);

    // Wrap Sol into wSol
    ctx.accounts.wrap_sol(args.amount_token_0)?;

    // Receive the pair to create the market
    ctx.accounts.receieve_market_creation_pair(args.amount_token_1)?;

    // Setup for Introspection
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // Load & Check the Market Creation Instruction
    let initialize_ix = load_instruction_at_checked(current_index + 1, &ixs).map_err(|_| TakeoverError::MissingInitializeTx)?;
    ctx.accounts.introspect_initialize(args.amount_token_0, args.amount_token_1, initialize_ix)?;

    Ok(())
}