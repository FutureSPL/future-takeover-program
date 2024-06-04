use anchor_lang::{
    prelude::*,
    solana_program::program::invoke,
    system_program::{create_account, CreateAccount}
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::Mint,
    token_2022::Token2022,
    token_interface::{TokenAccount, mint_to, MintTo, set_authority, SetAuthority},
};

pub use spl_token_2022::{
    extension::ExtensionType,
    instruction::{initialize_mint_close_authority, initialize_permanent_delegate, initialize_mint2},
    extension::{
        transfer_hook::instruction::initialize as intialize_transfer_hook,
        metadata_pointer::instruction::initialize as initialize_metadata_pointer,
    }
};

pub use spl_token_metadata_interface::{
    state::{TokenMetadata, Field},
    instruction::{initialize as initialize_metadata_account, update_field as update_metadata_account},
};


use crate::{
    state::{Takeover, AdminProfile, SwapPeriod, InflationAmount, Level, Phase::*},
    errors::TakeoverError,
    constant::*,
};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateT22TakeoverArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub start: i64,
    pub end: i64,
    pub takeover_wallet: Pubkey,
    pub presale_price: u64,
    pub fdmc: u8,
    pub presale_inflation: u16,
    pub treasury_inflation: u16,
    pub rewards_inflation: u16,
}

#[derive(Accounts)]
pub struct CreateT22Takeover<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump = admin_profile.bump,
    )]
    pub admin_profile: Box<Account<'info, AdminProfile>>,

    #[account(
        init,
        payer = admin,
        seeds = [b"takeover", old_mint.key().as_ref()],
        bump,
        space = Takeover::INIT_SPACE,
    )]
    pub takeover: Box<Account<'info, Takeover>>,
    
    pub old_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    /// CHECK: This is fine, will be checked by t22 instruction
    pub new_mint: UncheckedAccount<'info>,
    #[account(
        init,
        payer = admin,
        associated_token::mint = new_mint,
        associated_token::authority = takeover,
    )]
    pub takeover_new_mint_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_2022_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateT22Takeover<'info> {
    fn initialize_takeover(&mut self,  inflation_amount: InflationAmount, swap_period: SwapPeriod, takeover_wallet: Pubkey, presale_price: u64, bump: u8) -> Result<()> {
        // Populate the takeover account
        self.takeover.set_inner(
            Takeover {
                old_mint: self.old_mint.key(),
                new_mint: self.new_mint.key(),
                swap_period,
                takeover_wallet,
                inflation_amount: inflation_amount.clone(),
                presale_price,
                presale_claimed: 0,
                phase: Ongoing,
                bump,
            }
        );

        // Mint the new supply + the inflation amount to the takeover vault
        let amount = self.old_mint.supply
            .checked_add(inflation_amount.presale_amount.clone()).ok_or(TakeoverError::Overflow)?
            .checked_add(inflation_amount.treasury_amount.clone()).ok_or(TakeoverError::Overflow)?
            .checked_add(inflation_amount.rewards_amount.clone()).ok_or(TakeoverError::Overflow)?;

        mint_to(
            CpiContext::new(
                self.token_2022_program.to_account_info(),
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
        // Step 1: Initialize Account
        let size = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(&[ExtensionType::MetadataPointer]).unwrap();

        let metadata = TokenMetadata {
            update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(None).unwrap(),
            mint: self.new_mint.key(),
            name,
            symbol,
            uri,
            additional_metadata: vec![]
        };

        let extension_extra_space = metadata.tlv_size_of().unwrap();
        let rent = &Rent::from_account_info(&self.rent.to_account_info())?;
        let lamports = rent.minimum_balance(size + extension_extra_space);

        create_account(
            CpiContext::new(
                    self.system_program.to_account_info(),
                    CreateAccount {
                        from: self.admin.to_account_info(),
                        to: self.new_mint.to_account_info(),
                    },
                ),
                lamports,
                (size).try_into().unwrap(),
                &spl_token_2022::id(),
        )?;

        invoke(
            &initialize_metadata_pointer(
                &self.token_2022_program.key(),
                &self.new_mint.key(),
                None,
                Some(self.new_mint.key()),
            )?,
            &vec![
                self.new_mint.to_account_info(),
            ],
        )?;

        invoke(
            &initialize_mint2(
                &self.token_2022_program.key(),
                &self.new_mint.key(),
                &self.admin.key(),
                None,
                0,
            )?,
            &vec![
                self.new_mint.to_account_info(),
            ],
        )?;

        // To Update
        invoke(
            &initialize_metadata_account(
                &self.token_2022_program.key(),
                &self.new_mint.key(),
                &Pubkey::default(),
                &self.new_mint.key(),
                &self.admin.key(),
                metadata.name,
                metadata.symbol,
                metadata.uri,
            ),
            &vec![
                self.new_mint.to_account_info(),
                self.admin.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn remove_mint_authority(&self) -> Result<()> {
        set_authority(
            CpiContext::new(
                self.token_2022_program.to_account_info(),
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

pub fn handler(ctx: Context<CreateT22Takeover>, args: CreateT22TakeoverArgs) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(Clock::get()?.unix_timestamp - ctx.accounts.admin_profile.creation_time > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check and Save the Swap Period Parameters
    // require!(args.start < args.end && args.start > Clock::get()?.unix_timestamp, TakeoverError::InvalidSwapPeriod);
    let swap_period = SwapPeriod {
        start: args.start,
        end: args.end,
    };

    // Set parameters for rewards, presale and treasury based on the FDMC
    let inflation_amount: InflationAmount;
    
    match args.fdmc {
        0 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && args.rewards_inflation >= 0 &&
                args.presale_inflation < LOW_PRESALE_BASIS_POINT && args.treasury_inflation < LOW_TREASURY_BASIS_POINT && args.rewards_inflation < LOW_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::Low,
                presale_amount: ctx.accounts.old_mint.supply.checked_mul(args.presale_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                treasury_amount: ctx.accounts.old_mint.supply.checked_mul(args.treasury_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                rewards_amount: ctx.accounts.old_mint.supply.checked_mul(args.rewards_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
            };
        }
        1 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && args.rewards_inflation >= 0 &&
                args.presale_inflation < MEDIUM_PRESALE_BASIS_POINT && args.treasury_inflation < MEDIUM_TREASURY_BASIS_POINT && args.rewards_inflation < MEDIUM_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::Medium,
                presale_amount: ctx.accounts.old_mint.supply.checked_mul(args.presale_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                treasury_amount: ctx.accounts.old_mint.supply.checked_mul(args.treasury_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                rewards_amount: ctx.accounts.old_mint.supply.checked_mul(args.rewards_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
            };
        }
        2 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && args.rewards_inflation >= 0 &&
                args.presale_inflation < HIGH_PRESALE_BASIS_POINT && args.treasury_inflation < HIGH_TREASURY_BASIS_POINT && args.rewards_inflation < HIGH_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::High,
                presale_amount: ctx.accounts.old_mint.supply.checked_mul(args.presale_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                treasury_amount: ctx.accounts.old_mint.supply.checked_mul(args.treasury_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
                rewards_amount: ctx.accounts.old_mint.supply.checked_mul(args.rewards_inflation as u64).ok_or(TakeoverError::Overflow)?.checked_div(10000).ok_or(TakeoverError::Underflow)?,
            };
        }
        _ => return Err(TakeoverError::InvalidFdmcValue.into()),
    }

    // Generate the bumps
    let bumps = ctx.bumps;

    // Initialize the takeover and mint rewards + old_mint supply to the takeover vault
    ctx.accounts.initialize_takeover(inflation_amount, swap_period, args.takeover_wallet, args.presale_price, bumps.takeover)?;

    // Initialize the new mint using Metaplex
    ctx.accounts.intialize_new_mint(args.name, args.symbol, args.uri)?;

    // Remove the mint authority so nobody can mint more tokens
    ctx.accounts.remove_mint_authority()?;

    Ok(())
}