use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer}
};

use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, sync_native, SyncNative, close_account, CloseAccount},
};

use whirlpool_cpi::{self, state::*};

use crate::{
    state::{ Takeover, AdminProfile, Phase::*},
    errors::TakeoverError,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct OpenAndDepositPositionArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(Accounts)]
pub struct OpenAndDepositPosition<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // Funder for open_position
    #[account(
        seeds = [b"admin_profile", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Box<Account<'info, AdminProfile>>,

    // Orca accounts
    #[account(mut)]
    /// CHECK: init by whirlpool
    pub position: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: init by whirlpool
    pub position_mint: Signer<'info>,
    #[account(mut)]
    /// CHECK: init by whirlpool
    pub position_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub whirlpool: Box<Account<'info, Whirlpool>>,
    #[account(mut, has_one = whirlpool)]
    pub tick_array_lower: AccountLoader<'info, TickArray>,
    #[account(mut, has_one = whirlpool)]
    pub tick_array_upper: AccountLoader<'info, TickArray>,
    #[account(mut)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account( address = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap())]
    pub wsol: Box<Account<'info, Mint>>,
    #[account(
        mut,
        seeds = [b"takeover", takeover.old_mint.key().as_ref()],
        bump = takeover.bump,
    )]
    pub takeover: Box<Account<'info, Takeover>>, // owner for open_position; position_authority for increase_liquidity
    #[account(
        mut,
        associated_token::mint = takeover.new_mint,
        associated_token::authority = takeover,
    )]
    pub new_mint_takeover_vault: Box<Account<'info, TokenAccount>>, // token_owner_account_b
    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = wsol,
        associated_token::authority = takeover,
    )]
    pub wsol_takeover_vault: Box<Account<'info, TokenAccount>>, // token_owner_account_a
    #[account(
        mut,
        seeds = [b"takeover_vault", takeover.key().as_ref()],
        bump,
    )]
    pub takeover_sol_vault: SystemAccount<'info>,

    pub whirlpool_program: Program<'info, whirlpool_cpi::program::Whirlpool>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> OpenAndDepositPosition<'info> {
    fn wrap_sol(&mut self, amount: u64) -> Result<()> {
        // transfer sol to token account
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds = &[b"takeover", old_mint.as_ref(), &[self.takeover.bump]];

        transfer(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.takeover_sol_vault.to_account_info(),
                    to: self.wsol_takeover_vault.to_account_info(),
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
                    account: self.wsol_takeover_vault.to_account_info(),
                }
            )
        )?;

        Ok(())  
    }

    pub fn open_position(&self, tick_lower_index: i32, tick_upper_index: i32) -> Result<()> {
        let cpi_program = self.whirlpool_program.to_account_info();
      
        let cpi_accounts = whirlpool_cpi::cpi::accounts::OpenPosition {
          funder: self.admin.to_account_info(),
          owner: self.takeover.to_account_info(),
          position: self.position.to_account_info(),
          position_mint: self.position_mint.to_account_info(),
          position_token_account: self.position_token_account.to_account_info(),
          whirlpool: self.whirlpool.to_account_info(),
          token_program: self.token_program.to_account_info(),
          system_program: self.system_program.to_account_info(),
          rent: self.rent.to_account_info(),
          associated_token_program: self.associated_token_program.to_account_info(),
        };
      
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
      
        // execute CPI
        whirlpool_cpi::cpi::open_position(cpi_ctx, whirlpool_cpi::state::OpenPositionBumps { position_bump: 0 }, tick_lower_index, tick_upper_index)?;
      
        Ok(())
    }

    pub fn increase_liquidity(&self, liquidity_amount: u128, token_max_a: u64, token_max_b: u64) -> Result<()> {
        let cpi_program = self.whirlpool_program.to_account_info();

        let cpi_accounts = whirlpool_cpi::cpi::accounts::ModifyLiquidity {
            whirlpool: self.whirlpool.to_account_info(),
            token_program: self.token_program.to_account_info(),
            position_authority: self.takeover.to_account_info(), 
            position: self.position.to_account_info(),
            position_token_account: self.position_token_account.to_account_info(),
            token_owner_account_a: self.wsol_takeover_vault.to_account_info(),
            token_owner_account_b: self.new_mint_takeover_vault.to_account_info(),
            token_vault_a: self.token_vault_a.to_account_info(),
            token_vault_b: self.token_vault_b.to_account_info(),
            tick_array_lower: self.tick_array_lower.to_account_info(),
            tick_array_upper: self.tick_array_upper.to_account_info(),
        };

        let old_mint = self.takeover.old_mint.key().clone();
        let seeds: &[&[u8]; 3] = &[b"takeover", old_mint.as_ref(), &[self.takeover.bump]];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds); 

        // execute CPI
        whirlpool_cpi::cpi::increase_liquidity(cpi_ctx, liquidity_amount, token_max_a, token_max_b)?;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        self.wsol_takeover_vault.reload()?;
        // Close wSOL account and send to the Solana vault
        let old_mint = self.takeover.old_mint.key().clone();
        let signer_seeds: &[&[u8]; 3] = &[b"takeover", old_mint.as_ref(), &[self.takeover.bump]];
        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(), 
                CloseAccount {
                    account: self.wsol_takeover_vault.to_account_info(),
                    destination: self.takeover_sol_vault.to_account_info(),
                    authority: self.takeover.to_account_info(),
                },
                &[signer_seeds]
            )
        )?;

        Ok(())
    }

}

pub fn handler(ctx: Context<OpenAndDepositPosition>, args: OpenAndDepositPositionArgs) -> Result<()> {
    // Check if it's the right phase
    match ctx.accounts.takeover.phase {
        MarketCreation => (),
        _ => return Err(TakeoverError::InvalidPhase.into()),
    }

    // Change the phase
    ctx.accounts.takeover.phase = Cleanup;

    // Wrap SOL to wSOL
    ctx.accounts.wrap_sol(args.token_max_a)?;

    // Open the position
    ctx.accounts.open_position(args.tick_lower_index, args.tick_upper_index)?;

    // Increase the position Liquidity
    ctx.accounts.increase_liquidity(args.liquidity_amount, args.token_max_a, args.token_max_b)?;

    // Clenup the wSol account
    ctx.accounts.cleanup()?;

    Ok(())
}