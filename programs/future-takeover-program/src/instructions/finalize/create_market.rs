use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::{
        sysvar::{
            self,
            instructions::{load_current_index_checked, load_instruction_at_checked},
        },
        instruction::Instruction,
    },
    system_program::{ transfer, Transfer }
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{ transfer as spl_transfer, Transfer as SplTransfer, Mint, Token, TokenAccount }
};

use whirlpool_sdk::i11n::IncreaseLiquidityI11n;

use crate::{
    state::{ Takeover, AdminProfile, Phase::*},
    errors::TakeoverError,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateMarketArgs {
    pub wsol_input: u64,
    pub new_mint_input: u64,
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    pub new_mint: Box<Account<'info, Mint>>,
    #[account( address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap())]
    pub wsol: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = admin,
    )]
    pub new_mint_admin_token: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = wsol,
        associated_token::authority = admin,
    )]
    pub wsol_admin_token: Box<Account<'info, TokenAccount>>,
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
    pub new_mint_takeover_vault: Box<Account<'info, TokenAccount>>,
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
    fn receieve_new_token(&mut self, amount: u64) -> Result<()> {
        // Transfer the old tokens from the vault to the user
        let old_mint_key = self.takeover.old_mint.key().clone();
        let signer_seeds = &[b"takeover", old_mint_key.as_ref(), &[self.takeover.bump]];    

        spl_transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                SplTransfer {
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

    pub fn receieve_sol(&self, amount: u64, bump: u8) -> Result<()> {
        let takeover_key = self.takeover.key();
        let signer_seeds = &[b"takeover_vault", takeover_key.as_ref(), &[bump]];        
        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.takeover_sol_vault.to_account_info(),
                    to: self.admin.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        Ok(())
    }

    fn introspect_market(&mut self, wsol_amount: u64, new_token_amount: u64, increase_liquidity_ix: Instruction) -> Result<()> {

        // Checking that this is the right Swap Instruction
        let instruction = IncreaseLiquidityI11n::try_from(&increase_liquidity_ix).unwrap();

        // Checking the Args like the amount
        require_eq!(instruction.args.token_max_a, wsol_amount, TakeoverError::WrongAmountTokenA);
        require_eq!(instruction.args.token_max_b, new_token_amount, TakeoverError::WrongAmountTokenB);

        // Checking the Accounts like the ATAs
        require_eq!(instruction.accounts.token_owner_account_a.pubkey, self.wsol_admin_token.key(), TakeoverError::WrongAtaTokenA);
        require_eq!(instruction.accounts.token_owner_account_b.pubkey, self.new_mint_admin_token.key(), TakeoverError::WrongAtaTokenB);

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateMarket>, args: CreateMarketArgs) -> Result<()> {
    // Check if it's the right phase
    // match ctx.accounts.takeover.phase {
    //     MarketCreation => (),
    //     _ => return Err(TakeoverError::InvalidPhase.into()),
    // }

    // Check if the amount requested is valid
    require!(args.new_mint_input > 0 && args.wsol_input > 0, TakeoverError::InvalidAmount);

    // Receive the new tokens
    ctx.accounts.receieve_new_token(args.new_mint_input)?;

    // Receive the WSOL
    ctx.accounts.receieve_sol(args.wsol_input, ctx.bumps.takeover_sol_vault)?;

    // Setup for Introspection
    // let ixs = ctx.accounts.instructions.to_account_info();
    // let current_index = load_current_index_checked(&ixs)? as usize;

    // Load & Check the Increase Liquidity Instruction
    // let increase_liquidity_ix = load_instruction_at_checked(current_index + 1, &ixs).map_err(|_| TakeoverError::MissingIncreaseLiquidityIx)?;
    // ctx.accounts.introspect_market(args.wsol_input, args.new_mint_input, increase_liquidity_ix)?;
    
    Ok(())
}