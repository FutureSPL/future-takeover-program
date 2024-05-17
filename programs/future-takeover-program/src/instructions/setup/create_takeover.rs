use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::{
        mpl_token_metadata::{
            instructions::{CreateV1Cpi, CreateV1CpiAccounts, CreateV1InstructionArgs},
            types::TokenStandard::Fungible,
        }, Metadata
    }, token::{ Mint, Token, TokenAccount, mint_to, MintTo, set_authority, SetAuthority} 
};

use crate::{
    state::{Takeover, AdminProfile, SwapPeriod, InflationAmount, Level},
    errors::TakeoverError,
    constant::*,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateTakeoverArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub start: i64,
    pub end: i64,
    pub takeover_wallet: Pubkey,
    pub presale_price: u64,
    pub fdmc: u8,
}

#[derive(Accounts)]
pub struct CreateTakeover<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Account<'info, AdminProfile>,

    #[account(
        init,
        payer = admin,
        seeds = [b"takeover", old_mint.key().as_ref()],
        bump,
        space = Takeover::INIT_SPACE,
    )]
    pub takeover: Account<'info, Takeover>,
    
    pub old_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = admin,
        mint::decimals = old_mint.decimals,
        mint::authority = admin,
    )]
    pub new_mint: Account<'info, Mint>,
    #[account(mut)]
    /// CHECK: This will be checked by the Metaplex CreateV1Cpi instruction
    pub metadata: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub takeover_new_mint_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    /// CHECK: This will be checked by the Metaplex CreateV1Cpi instruction
    pub sysvar_instruction_program: AccountInfo<'info>,
    pub metaplex_token_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateTakeover<'info> {
    fn initialize_takeover(&mut self,  inflation_amount: InflationAmount, swap_period: SwapPeriod, takeover_wallet: Pubkey, presale_price: u64, bump: u8) -> Result<()> {
        // Calculate the presale amount: rewards% of the old mint supply
        let presale_amount = self.old_mint.supply.checked_mul(inflation_amount.presale_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;
        // Calculate the treasury amount: rewards% of the old mint supply
        let treasury_amount = self.old_mint.supply.checked_mul(inflation_amount.treasury_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;
        // Calculate the rewards amount: rewards% of the old mint supply
        let rewards_amount = self.old_mint.supply.checked_mul(inflation_amount.rewards_basis_point as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?;

        // Populate the takeover account
        self.takeover.set_inner(
            Takeover {
                old_mint: self.old_mint.key(),
                new_mint: self.new_mint.key(),
                swap_period,
                takeover_wallet,
                inflation_amount,
                presale_price,
                presale_claimed: 0,
                bump,
            }
        );

        // Mint the new supply + the inflation amount to the takeover vault
        let amount = self.old_mint.supply
            .checked_add(presale_amount).ok_or(TakeoverError::Overflow)?
            .checked_add(treasury_amount).ok_or(TakeoverError::Overflow)?
            .checked_add(rewards_amount).ok_or(TakeoverError::Overflow)?;

        mint_to(
            CpiContext::new(
                self.token_program.to_account_info(),
                MintTo {
                    mint: self.new_mint.to_account_info(),
                    to: self.takeover_new_mint_vault.to_account_info(),
                    authority: self.admin.to_account_info(),
                }
            ),
            amount
        )?;

        Ok(())
    }

    fn intialize_new_mint(&self, name: String, symbol: String, uri: String) -> Result<()> {
        let metaplex_program = &self.metaplex_token_program.to_account_info();
        let metadata = &self.metadata.to_account_info();
        let master_edition = None;
        let mint = (&self.new_mint.to_account_info(), false);
        let authority = &self.admin.to_account_info();
        let payer = &self.admin.to_account_info();
        let update_authority = (&self.admin.to_account_info(), false);
        let system_program = &self.system_program.to_account_info();
        let sysvar_instructions = &self.sysvar_instruction_program.to_account_info();
        let spl_token_program = &self.token_program.to_account_info();

        let accounts = CreateV1CpiAccounts {
            metadata,
            mint,
            payer,
            authority,
            update_authority,
            system_program,
            sysvar_instructions,
            spl_token_program: Some(spl_token_program),
            master_edition,
        };

        let args = CreateV1InstructionArgs {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            primary_sale_happened: false,
            is_mutable: false,
            token_standard: Fungible,
            collection: None,
            uses: None,
            collection_details: None,
            rule_set: None,
            decimals: None,
            print_supply: None,
        };

        CreateV1Cpi::new(
            metaplex_program,
            accounts,
            args,
        ).invoke()?;

        Ok(())
    }

    pub fn remove_mint_authority(&self) -> Result<()> {
        set_authority(
            CpiContext::new(
                self.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: self.new_mint.to_account_info(),
                    current_authority: self.admin.to_account_info(),
                }
            ),
            anchor_spl::token::spl_token::instruction::AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }
    
}

pub fn handler(ctx: Context<CreateTakeover>, args: CreateTakeoverArgs) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(ctx.accounts.admin_profile.creation_time - Clock::get()?.unix_timestamp > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check and Save the Swap Period Parameters
    require!(args.start < args.end && args.start > Clock::get()?.unix_timestamp, TakeoverError::InvalidSwapPeriod);
    let swap_period = SwapPeriod {
        start: args.start,
        end: args.end,
    };

    // Set parameters for rewards, presale and treasury based on the FDMC
    let inflation_amount: InflationAmount;

    match args.fdmc {
        0 => {
            inflation_amount = InflationAmount {
                level: Level::Low,
                rewards_basis_point: LOW_REWARDS_BASIS_POINT,
                treasury_basis_point: LOW_TREASURY_BASIS_POINT,
                presale_basis_point: LOW_PRESALE_BASIS_POINT,
            };
        }
        1 => {
            inflation_amount = InflationAmount {
                level: Level::Medium,
                rewards_basis_point: MEDIUM_REWARDS_BASIS_POINT,
                treasury_basis_point: MEDIUM_TREASURY_BASIS_POINT,
                presale_basis_point: MEDIUM_PRESALE_BASIS_POINT,
            };
        }
        2 => {
            inflation_amount = InflationAmount {
                level: Level::High,
                rewards_basis_point: HIGH_REWARDS_BASIS_POINT,
                treasury_basis_point: HIGH_TREASURY_BASIS_POINT,
                presale_basis_point: HIGH_PRESALE_BASIS_POINT,
            };
        }
        _ => return Err(TakeoverError::InvalidFdmcValue.into()),
    }

    // Generate the bumps
    let bumps = ctx.bumps;

    // Initialize the takeover and mint rewards + old_mint supply to the takeover vault
    ctx.accounts.initialize_takeover(inflation_amount, swap_period, args.takeover_wallet, args.presale_price, bumps.takeover)?;

    msg!("HERE");
    // Initialize the new mint using Metaplex
    ctx.accounts.intialize_new_mint(args.name, args.symbol, args.uri)?;

    msg!("HERE");
    // Remove the mint authority so nobody can mint more tokens
    ctx.accounts.remove_mint_authority()?;

    Ok(())
}