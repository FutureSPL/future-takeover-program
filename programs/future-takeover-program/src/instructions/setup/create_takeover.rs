use anchor_lang::{prelude::*, solana_program::program::invoke, system_program::{create_account, CreateAccount}};

use anchor_spl::{
    associated_token::{create, get_associated_token_address_with_program_id, AssociatedToken, Create}, metadata::{mpl_token_metadata::instructions::CreateV1CpiBuilder, Metadata}, token::ID as TOKEN_ID, token_2022::spl_token_2022::{extension::{self, ExtensionType}, instruction::{initialize_mint_close_authority, initialize_permanent_delegate, AuthorityType}, state::Mint as SplToken22Mint, ID as TOKEN_2022_ID}, token_interface::{
        mint_to, set_authority, spl_pod, spl_token_metadata_interface::{instruction::initialize, state::TokenMetadata}, Mint, MintTo, SetAuthority, TokenInterface 
    }
};

use crate::{
    state::{Takeover, AdminProfile, SwapPeriod, InflationAmount, Level, Phase::*},
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
    pub presale_inflation: u16,
    pub treasury_inflation: u16,
    pub rewards_inflation: u16,
    pub referral: Option<Pubkey>,
    pub referral_split: Option<u16>,
    pub token_extension: Option<TokenExtensionArgs>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct TokenExtensionArgs {
    pub transfer_fee: Option<TransferFeeArgs>,
    pub interest_bearing: Option<InterestBearingArgs>,
    pub permanent_delegate: Option<PermanentDelegateArgs>,
    pub close_mint: Option<CloseMintArgs>,
    pub transfer_hook: Option<TransferHookArgs>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct TransferFeeArgs {
    pub fee_authority: Pubkey,
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct InterestBearingArgs {
    pub rate_authority: Pubkey,
    pub rate: i16,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct PermanentDelegateArgs {
    pub delegate_authority: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct CloseMintArgs {
    pub close_mint_authority: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct TransferHookArgs {
    pub hook_authority: Pubkey,
    pub hook_program_id: Pubkey,
}

#[derive(Accounts)]
pub struct CreateTakeover<'info> {
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
    
    pub old_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub new_mint: Signer<'info>,
    #[account(mut)]
    /// CHECK: This will be checked by the Metaplex CreateV1Cpi instruction
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This will be checked by the create_mint instruction
    pub takeover_new_mint_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    /// CHECK: This will be checked by the Metaplex CreateV1Cpi instruction
    pub sysvar_instruction_program: AccountInfo<'info>,
    pub metaplex_token_program: Program<'info, Metadata>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreateTakeover<'info> {
    fn initialize_takeover(&mut self,  inflation_amount: InflationAmount, swap_period: SwapPeriod, takeover_wallet: Pubkey, presale_price: u64, referral: Option<Pubkey>, bump: u8) -> Result<()> {
        // Populate the takeover account
        self.takeover.set_inner(
            Takeover {
                old_mint: self.old_mint.key(),
                new_mint: self.new_mint.key(),
                swap_period,
                takeover_wallet,
                referral,
                inflation_amount: inflation_amount.clone(),
                presale_price,
                presale_claimed: 0,
                token_swapped: 0,
                phase: Ongoing,
                bump,
            }
        );

        Ok(())
    }

    fn intialize_new_native_mint(&self, name: String, symbol: String, uri: String) -> Result<()> {

        let rent = &Rent::from_account_info(&self.rent.to_account_info())?;
        let space = anchor_spl::token::Mint::LEN;
        let lamports = rent.minimum_balance(space);

        create_account(
        CpiContext::new(
                self.token_program.to_account_info(),
                CreateAccount {
                    from: self.admin.to_account_info(),
                    to: self.new_mint.to_account_info(),
                }
            ),
            lamports,
            space as u64,
            &self.token_program.key(),
        )?;

        anchor_spl::token::initialize_mint2(
            CpiContext::new(
                self.token_program.to_account_info(),
                anchor_spl::token::InitializeMint2 {
                    mint: self.new_mint.to_account_info(),
                },                
            ),
            self.old_mint.decimals,
            &self.admin.key(),
            None,
        )?;

        CreateV1CpiBuilder::new(&self.metaplex_token_program.to_account_info())
            .metadata(&self.metadata.to_account_info())
            .mint(&self.new_mint.to_account_info(), true)
            .authority(&self.admin.to_account_info())
            .payer(&self.admin.to_account_info())
            .update_authority(&self.admin.to_account_info(), true)
            .system_program(&self.system_program.to_account_info())
            .sysvar_instructions(&self.sysvar_instruction_program.to_account_info())
            .spl_token_program(Some(&self.token_program.to_account_info()))
            .name(name)
            .symbol(symbol)
            .uri(uri)
            .seller_fee_basis_points(0)
            .invoke()?;



        Ok(())
    }

    fn initialize_new_extension_mint(&self, name: String, symbol: String, uri: String, token_extension: TokenExtensionArgs) -> Result<()> {

        let mut extension_type: Vec<ExtensionType> = vec![];

        extension_type.push(ExtensionType::MetadataPointer);

        let metadata = TokenMetadata {
            update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(None).unwrap(),
            mint: self.new_mint.key(),
            name,
            symbol,
            uri,
            additional_metadata: vec![],
        };
        
        if token_extension.close_mint.is_some() {
            extension_type.push(ExtensionType::MintCloseAuthority);
        }

        if token_extension.interest_bearing.is_some() {
            extension_type.push(ExtensionType::InterestBearingConfig);
        }

        if token_extension.permanent_delegate.is_some() {
            extension_type.push(ExtensionType::PermanentDelegate);
        }

        if token_extension.transfer_fee.is_some() {
            extension_type.push(ExtensionType::TransferFeeConfig);
        }

        if token_extension.transfer_hook.is_some() {
            extension_type.push(ExtensionType::TransferHook);
        }

        let size = ExtensionType::try_calculate_account_len::<SplToken22Mint>(&extension_type).unwrap();
        let extension_extra_space = metadata.tlv_size_of().unwrap();
        let lamports = self.rent.minimum_balance(size + extension_extra_space);

        create_account(
        CpiContext::new(
                self.token_program.to_account_info(),
                CreateAccount {
                    from: self.admin.to_account_info(),
                    to: self.new_mint.to_account_info(),
                }
            ),
            lamports,
            (size).try_into().unwrap(),
            &self.token_program.key(),
        )?;

        invoke(
            &extension::metadata_pointer::instruction::initialize(
                &self.token_program.key(),
                &self.new_mint.key(),
                None,
                Some(self.new_mint.key()),
            )?,
            &vec![
                self.new_mint.to_account_info(),
            ],
        )?;

        if let Some(close_mint) = token_extension.close_mint {
            invoke(
                &initialize_mint_close_authority(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    Some(&close_mint.close_mint_authority),
                )?,
                &vec![
                    self.new_mint.to_account_info(),
                ],
            )?;
        }

        if let Some(interest_bearing) = token_extension.interest_bearing {
            invoke(
                &extension::interest_bearing_mint::instruction::initialize(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    Some(interest_bearing.rate_authority),
                    interest_bearing.rate,
                )?,
                &vec![
                    self.new_mint.to_account_info()
                ],
            )?;
        }

        if let Some(permanent_delegate) = token_extension.permanent_delegate {
            invoke(
                &initialize_permanent_delegate(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    &permanent_delegate.delegate_authority,
                )?,
                &vec![
                    self.new_mint.to_account_info(),
                ],
            )?;
        }

        if let Some(transfer_fee) = token_extension.transfer_fee {
            invoke(
                &extension::transfer_fee::instruction::initialize_transfer_fee_config(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    Some(&transfer_fee.fee_authority),
                    Some(&transfer_fee.fee_authority),
                    transfer_fee.transfer_fee_basis_points,
                    transfer_fee.maximum_fee,
                )?,
                &vec![self.new_mint.to_account_info()],
            )?;
        }

        if let Some(transfer_hook) = token_extension.transfer_hook {
            invoke(
                &extension::transfer_hook::instruction::initialize(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    Some(transfer_hook.hook_authority),
                    Some(transfer_hook.hook_program_id),
                )?,
                &vec![self.new_mint.to_account_info()],
            )?;
        }

        anchor_spl::token_2022::initialize_mint2(
            CpiContext::new(
                self.token_program.to_account_info(),
                anchor_spl::token_2022::InitializeMint2 {
                    mint: self.new_mint.to_account_info(),
                },                
            ),
            self.old_mint.decimals,
            &self.admin.key(),
            None,
        )?;

        invoke(
            &initialize(
                &self.token_program.key(),
                &self.new_mint.key(),
                &self.new_mint.key(),
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

    fn operate_on_new_mint(&self, inflation_amount: InflationAmount) -> Result<()> {

        // Check if the takeover_new_mint_vault is the associated token account of the takeover wallet
        require_eq!(
            self.takeover_new_mint_vault.key(), 
            get_associated_token_address_with_program_id(
                &self.takeover.key(), 
                &self.new_mint.key(),
                &self.token_program.key()
            ), 
            TakeoverError::InvalidAssociatedToken
        );

        // Create the Takeover ATA
        create(
            CpiContext::new(
                self.token_program.to_account_info(),
                Create {
                    payer: self.admin.to_account_info(),
                    associated_token: self.takeover_new_mint_vault.to_account_info(),
                    authority: self.takeover.to_account_info(),
                    mint: self.new_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                }
            )
        )?;

        // Mint the new supply + the inflation amount to the takeover vault
        let amount = self.old_mint.supply
            .checked_add(inflation_amount.presale_amount.clone())
            .ok_or(TakeoverError::Overflow)?
            .checked_add(inflation_amount.treasury_amount.clone())
            .ok_or(TakeoverError::Overflow)?
            .checked_add(inflation_amount.rewards_amount.clone())
            .ok_or(TakeoverError::Overflow)?;

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

        // Remove the mint authority so nobody can mint more tokens
        set_authority(
            CpiContext::new(
                self.token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: self.new_mint.to_account_info(),
                    current_authority: self.admin.to_account_info(),
                }
            ),
            AuthorityType::MintTokens,
            None,
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateTakeover>, args: CreateTakeoverArgs) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require!(Clock::get()?.unix_timestamp - ctx.accounts.admin_profile.creation_time > ADMIN_BUFFER, TakeoverError::UnauthorizedAdmin);

    // Check and Save the Swap Period Parameters
    require!(args.start < args.end && args.start > Clock::get()?.unix_timestamp, TakeoverError::InvalidSwapPeriod);
    let swap_period = SwapPeriod {
        start: args.start,
        end: args.end,
    };

    // Set parameters for rewards, presale and treasury based on the FDMC
    let inflation_amount: InflationAmount;

    let presale_amount: u64 = (
        ctx.accounts.old_mint.supply
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?
    ).checked_mul(args.presale_inflation as u64)
        .ok_or(TakeoverError::Overflow)?;

    let treasury_amount: u64 = (
        ctx.accounts.old_mint.supply
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?
    ).checked_mul(args.treasury_inflation as u64)
        .ok_or(TakeoverError::Overflow)?;

    let rewards_amount: u64;
    let referral_amount: u64;

    if let Some(referral_split) = args.referral_split {
        let tmp = (
            ctx.accounts.old_mint.supply
                .checked_div(10000)
                .ok_or(TakeoverError::Underflow)?
        ).checked_mul(args.rewards_inflation as u64)
            .ok_or(TakeoverError::Overflow)?;

        referral_amount = tmp
            .checked_mul(referral_split as u64)
            .ok_or(TakeoverError::Overflow)?
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?;

        rewards_amount = tmp
            .checked_sub(referral_amount)
            .ok_or(TakeoverError::Underflow)?;
    } else {
        referral_amount = 0;

        rewards_amount = (
            ctx.accounts.old_mint.supply
                .checked_div(10000)
                .ok_or(TakeoverError::Underflow)?
        ).checked_mul(args.rewards_inflation as u64)
            .ok_or(TakeoverError::Overflow)?;
    }

    match args.fdmc {
        0 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && (args.referral_split.is_none() || args.referral_split.unwrap() <= MAX_REFERRAL_BASIS_POINT) &&
                args.presale_inflation <= LOW_PRESALE_BASIS_POINT && args.treasury_inflation <= LOW_TREASURY_BASIS_POINT && args.rewards_inflation <= LOW_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::Low,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            };
        }
        1 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && (args.referral_split.is_none() || args.referral_split.unwrap() <= MAX_REFERRAL_BASIS_POINT) &&
                args.presale_inflation <= MEDIUM_PRESALE_BASIS_POINT && args.treasury_inflation <= MEDIUM_TREASURY_BASIS_POINT && args.rewards_inflation <= MEDIUM_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::Medium,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            };
        }
        2 => {
            require!(
                args.presale_inflation > 0 && args.treasury_inflation > 0 && (args.referral_split.is_none() || args.referral_split.unwrap() <= MAX_REFERRAL_BASIS_POINT) &&
                args.presale_inflation <= HIGH_PRESALE_BASIS_POINT && args.treasury_inflation <= HIGH_TREASURY_BASIS_POINT && args.rewards_inflation <= HIGH_REWARDS_BASIS_POINT,
                TakeoverError::InvalidInflationAmounts
            );
            inflation_amount = InflationAmount {
                level: Level::High,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            };
        }
        _ => return Err(TakeoverError::InvalidFdmcValue.into()),
    }

    // Generate the bumps
    let bumps = ctx.bumps;

    // Initialize the takeover and mint rewards + old_mint supply to the takeover vault
    ctx.accounts.initialize_takeover(inflation_amount.clone(), swap_period, args.takeover_wallet, args.presale_price, args.referral, bumps.takeover)?;

    // Initialize the new mint using Metaplex
    match ctx.accounts.token_program.key() {
        TOKEN_2022_ID => {
            require!(args.token_extension.is_some(), TakeoverError::InvalidTokenExtensionArgs);
            ctx.accounts.initialize_new_extension_mint(args.name, args.symbol, args.uri, args.token_extension.unwrap())?;
        },
        TOKEN_ID => {
            require!(args.token_extension.is_none(), TakeoverError::InvalidTokenExtensionArgs);
            ctx.accounts.intialize_new_native_mint(args.name, args.symbol, args.uri)?;
        },
        _ => return Err(TakeoverError::InvalidTokenProgram.into()),
    }

    // Remove the mint authority so nobody can mint more tokens
    ctx.accounts.operate_on_new_mint(inflation_amount)?;

    Ok(())
}