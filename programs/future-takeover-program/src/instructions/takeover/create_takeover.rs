use anchor_lang::{
    prelude::*, 
    solana_program::{program::invoke, program_memory::sol_memcpy}, 
    system_program::{ create_account, CreateAccount },
};

use anchor_spl::{
    associated_token::{create, get_associated_token_address_with_program_id, AssociatedToken, Create},
    metadata::{ mpl_token_metadata::{ instructions::CreateV1CpiBuilder, types::TokenStandard }, Metadata}, 
    token::ID as TOKEN_ID, 
    token_2022::spl_token_2022::{state::Mint as SplToken22Mint, ID as TOKEN_2022_ID}, 
    token_interface::{
        mint_to, set_authority, spl_pod, spl_token_2022::{
            extension::{ interest_bearing_mint::instruction::initialize as initialize_interest_bearing_mint, metadata_pointer::instruction::initialize as initialize_metadata_pointer, transfer_fee::instruction::initialize_transfer_fee_config, transfer_hook::instruction::initialize as initialize_transfer_hook, ExtensionType },
            instruction::{initialize_mint_close_authority, initialize_permanent_delegate, AuthorityType}
        }, spl_token_metadata_interface::{instruction::initialize as initialize_metadata_interface, state::TokenMetadata}, Mint, MintTo, SetAuthority, TokenAccount, TokenInterface  
    }
};

use crate::{
    ID as MIGRATION_PROGRAM_ID, constant::*, errors::TakeoverError, state::{AdminProfile, InflationAmount, Level, OldMints, Phase::*, SwapPeriod, Takeover}
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
    pub old_mints: Option<OldMints>, 
    pub decimals: Option<u8>,
    pub referral: Option<Pubkey>,
    pub referral_split: Option<u16>,
    pub token_extension: Option<TokenExtensionArgs>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TokenExtensionArgs {
    pub transfer_fee: Option<TransferFeeArgs>,
    pub interest_bearing: Option<InterestBearingArgs>,
    pub permanent_delegate: Option<PermanentDelegateArgs>,
    pub close_mint: Option<CloseMintArgs>,
    pub transfer_hook: Option<TransferHookArgs>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct TransferFeeArgs {
    pub fee_authority: Pubkey,
    pub transfer_fee_basis_points: u16,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct InterestBearingArgs {
    pub rate_authority: Pubkey,
    pub rate: i16,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct PermanentDelegateArgs {
    pub delegate_authority: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CloseMintArgs {
    pub close_mint_authority: Pubkey,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
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
    #[account(mut)]
    /// CHECK: This will be checked in the instruction
    pub takeover: UncheckedAccount<'info>,
    #[account(mut)]
    pub new_mint: Signer<'info>,
    #[account(mut)]
    /// CHECK: This will be checked by the Metaplex CreateV1Cpi instruction
    pub metadata: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: This will be checked by the finalize_mint function
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
    // Calculate all the parameter needed for the Takeover
    fn calculate_parameters(&mut self, remaining_accounts: &[AccountInfo<'info>], args: &CreateTakeoverArgs) -> Result<(u64, u8, OldMints)> {
    
        let mut supply: u64 = 0;
        let decimals: u8; 
        let old_mints: OldMints;
        let mut token_program: Option<&Pubkey> = None;
    
        if remaining_accounts.len() == 1 {
            let mint_account = remaining_accounts[0].clone();
            let data = mint_account.try_borrow_mut_data()?;
    
            let mint = Mint::try_deserialize(&mut &data[..])
                .map_err(|_| TakeoverError::InvalidMint)?;
    
            supply = supply
                .checked_add(mint.supply)
                .ok_or(TakeoverError::Overflow)?;
    
            decimals = mint.decimals;
    
            old_mints = OldMints {
                old_mint: mint_account.key(),
                weight_percentage: None,
                old_mint_2: None,
                weight_percentage_2: None,
                old_mint_3: None,
                weight_percentage_3: None,
            };
    
        } else {
            old_mints = args.old_mints.as_ref()
                .ok_or(TakeoverError::OldTokensNotFound)?
                .clone();
            
            let mut total_percentage: u8 = 0;
    
            match remaining_accounts.len() {
                1 => require!(old_mints.old_mint_2.is_none() && old_mints.old_mint_3.is_none(), TakeoverError::InvalidMint),
                2 => require!(old_mints.old_mint_3.is_none(), TakeoverError::InvalidMint),
                3 => (),
                _ => return Err(TakeoverError::TooManyMint.into()),
            }
    
            for (i, account) in remaining_accounts.iter().enumerate() {
    
                let (mint, weight) = match i {
                    0 => (old_mints.old_mint, old_mints.weight_percentage.ok_or(TakeoverError::WeightedPercentageNotFound)?),
                    1 => (old_mints.old_mint_2.ok_or(TakeoverError::OldMintNotFound)?, old_mints.weight_percentage_2.ok_or(TakeoverError::WeightedPercentageNotFound)?),
                    2 => (old_mints.old_mint_3.ok_or(TakeoverError::OldMintNotFound)?, old_mints.weight_percentage_3.ok_or(TakeoverError::WeightedPercentageNotFound)?),
                    _ => return Err(TakeoverError::TooManyMint.into()),
                };

                // Ensure the account key matches the old_mint
                require_eq!(account.key(), mint, TakeoverError::InvalidMint);

                // Check that all the Mint have the same token program
                match token_program {
                    None => token_program = Some(account.owner),
                    Some(ref token_program) => require_eq!(token_program, &account.owner, TakeoverError::InvalidTokenProgram),
                }
                
                // Deserialize the mint data
                let data = account.try_borrow_mut_data()?;
                let mint = Mint::try_deserialize(&mut &data[..])
                    .map_err(|_| TakeoverError::InvalidMint)?;
    
                // Update the supply using the weight percentage
                let weighted_supply = mint.supply
                    .checked_div(100)
                    .ok_or(TakeoverError::Underflow)?
                    .checked_mul(weight as u64)
                    .ok_or(TakeoverError::Overflow)?;
    
                supply = supply
                    .checked_add(weighted_supply)
                    .ok_or(TakeoverError::Overflow)?;
            
                // Accumulate the total percentage from old_mints
                total_percentage = total_percentage
                    .checked_add(weight)
                    .ok_or(TakeoverError::Overflow)?;
            }
            
            // Ensure that total_percentage is within expected bounds
            require_eq!(total_percentage, 100, TakeoverError::InvalidTotalPercentage);
    
            decimals = args.decimals.ok_or(TakeoverError::DecimalsNotFound)?;
        }
    
        // Return the final tuple with supply, decimals, and old_mints
        Ok((supply, decimals, old_mints))
    }
    
    // Initialize a new Takeover Account
    fn initialize_takeover(&mut self,  old_mints: OldMints, inflation_amount: InflationAmount, swap_period: SwapPeriod, takeover_wallet: Pubkey, presale_price: u64, referral: Option<Pubkey>) -> Result<()> {

        let (takeover, bump) = Pubkey::find_program_address(&[b"takeover", old_mints.old_mint.as_ref()], &MIGRATION_PROGRAM_ID);
        require_eq!(takeover, self.takeover.key(), TakeoverError::InvalidTakeoverAddress);

        let signer_seeds = &[b"takeover", old_mints.old_mint.as_ref(), &[bump]];
        let lamports = self.rent.minimum_balance(Takeover::INIT_SPACE);

        create_account(
            CpiContext::new_with_signer(
                self.system_program.to_account_info(), 
                CreateAccount {
                    from: self.admin.to_account_info(),
                    to: self.takeover.to_account_info()
                },
                &[signer_seeds],
            ),
            lamports,
            Takeover::INIT_SPACE as u64,
            &MIGRATION_PROGRAM_ID,
        )?;

        let takeover_data =  Takeover {
            old_mints,
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
        };

        let info = self.takeover.to_account_info(); 
        let mut data = info.try_borrow_mut_data()?;

        let mut writer = vec![];
        takeover_data.try_serialize(&mut writer)?;
        
        sol_memcpy(&mut data, &writer, writer.len());

        Ok(())
    }

    // Initialize a new Mint Account & Metaplex Metadata Account
    fn initialize_native_mint(&self, name: String, symbol: String, uri: String, decimals: u8) -> Result<()> {
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
            decimals,
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
            .token_standard(TokenStandard::Fungible)
            .invoke()?;

        msg!("Metaplex Metadata Account Created");
        
        Ok(())
    }

    // Initialize a new Mint Account & T22 Metadata Account + Extensions (if any)
    fn initialize_extension_mint(&self, name: String, symbol: String, uri: String, token_extension: TokenExtensionArgs,  decimals: u8 ) -> Result<()> {
        let mut extension_types: Vec<ExtensionType> = vec![ExtensionType::MetadataPointer];
        
        if token_extension.close_mint.is_some() {
            extension_types.push(ExtensionType::MintCloseAuthority);
        }

        if token_extension.interest_bearing.is_some() {
            extension_types.push(ExtensionType::InterestBearingConfig);
        }

        if token_extension.permanent_delegate.is_some() {
            return Err(TakeoverError::ExtensionNotAvaialble.into());
            // extension_types.push(ExtensionType::PermanentDelegate);
        }

        if token_extension.transfer_fee.is_some() {
            extension_types.push(ExtensionType::TransferFeeConfig);
        }

        // Unavailable for the moment
        if token_extension.transfer_hook.is_some() {
            return Err(TakeoverError::ExtensionNotAvaialble.into());
            // extension_types.push(ExtensionType::TransferHook);
        }

        let size = ExtensionType::try_calculate_account_len::<SplToken22Mint>(&extension_types).unwrap();

        let metadata = TokenMetadata {
            update_authority: spl_pod::optional_keys::OptionalNonZeroPubkey::try_from(None).unwrap(),
            mint: self.new_mint.key(),
            name,
            symbol,
            uri,
            additional_metadata: vec![],
        };
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
            size as u64,
            &self.token_program.key(),
        )?;

        invoke(
            &initialize_metadata_pointer(
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
                &initialize_interest_bearing_mint(
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

        // Maximum Fee set to 0 for the moment -> Owner will need to change this value after
        if let Some(transfer_fee) = token_extension.transfer_fee {
            invoke(
                &initialize_transfer_fee_config(
                    &self.token_program.key(),
                    &self.new_mint.key(),
                    Some(&transfer_fee.fee_authority),
                    Some(&transfer_fee.fee_authority),
                    transfer_fee.transfer_fee_basis_points,
                    0,
                )?,
                &vec![self.new_mint.to_account_info()],
            )?;
        }

        if let Some(transfer_hook) = token_extension.transfer_hook {
            invoke(
                &initialize_transfer_hook(
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
            decimals,
            &self.admin.key(),
            None,
        )?;

        invoke(
            &initialize_metadata_interface(
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

    // Create the ATA, Mint to the Takeover Vault and Set the Mint Authority to None
    fn finalize_mint(&self, inflation_amount: InflationAmount, supply: u64) -> Result<()> {
        require_eq!(
            self.takeover_new_mint_vault.key(),
            get_associated_token_address_with_program_id(self.takeover.key, self.new_mint.key, self.token_program.key),
            TakeoverError::InvalidAssociatedToken
        );

        create(
            CpiContext::new(
                self.associated_token_program.to_account_info(),
                Create {
                    associated_token: self.takeover_new_mint_vault.to_account_info(),
                    authority: self.takeover.to_account_info(),
                    payer: self.admin.to_account_info(),
                    mint: self.new_mint.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                    token_program: self.token_program.to_account_info(),
                },
            ),
        )?;
        
        let amount = supply
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

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, CreateTakeover<'info>>, args: CreateTakeoverArgs) -> Result<()> {
    // Check if the admin has been initialized more than 16h ago
    require_gte!(
        Clock::get()?.unix_timestamp
            .checked_sub(ctx.accounts.admin_profile.creation_time)
            .ok_or(TakeoverError::Underflow)?, 
            ADMIN_BUFFER,
        TakeoverError::UnauthorizedAdmin
    );

    // Check and Save the Swap Period Parameters
    require!(
        args.start < args.end && args.start > Clock::get()?.unix_timestamp, 
        TakeoverError::InvalidSwapPeriod
    );

    // ACTIONS
    let (supply, decimals,old_mints) = ctx.accounts.calculate_parameters(ctx.remaining_accounts, &args)?;

    // Calculate the Presale Amount
    let presale_amount = supply
        .checked_div(10000)
        .ok_or(TakeoverError::Underflow)?
        .checked_mul(args.presale_inflation as u64)
        .ok_or(TakeoverError::Overflow)?;

    // Calculate the Treasury Amount
    let treasury_amount = supply
        .checked_div(10000)
        .ok_or(TakeoverError::Underflow)?
        .checked_mul(args.treasury_inflation as u64)
        .ok_or(TakeoverError::Overflow)?;

    // Calculate the Rewards Amount and Referral Amount (if any)
    let (rewards_amount, referral_amount) = if let Some(referral_split) = args.referral_split {
        let tmp = supply
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?
            .checked_mul(args.rewards_inflation as u64)
            .ok_or(TakeoverError::Overflow)?;
        let referral_amount = tmp
            .checked_mul(referral_split as u64)
            .ok_or(TakeoverError::Overflow)?
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?;
        let rewards_amount = tmp
            .checked_sub(referral_amount)
            .ok_or(TakeoverError::Underflow)?;
        (rewards_amount, referral_amount)
    } else {
        let rewards_amount = supply
            .checked_div(10000)
            .ok_or(TakeoverError::Underflow)?
            .checked_mul(args.rewards_inflation as u64)
            .ok_or(TakeoverError::Overflow)?;
        (rewards_amount, 0)
    };

    // Calculate the Inflation Amounts
    require!(
        args.presale_inflation > 0 && 
        args.treasury_inflation > 0 && 
        (args.referral_split.is_none() || args.referral_split.unwrap() <= MAX_REFERRAL_BASIS_POINT),
        TakeoverError::InvalidInflationAmounts
    );

    let inflation_amount = match args.fdmc {
        0 => {
            require!(
                args.presale_inflation <= LOW_PRESALE_BASIS_POINT && 
                args.treasury_inflation <= LOW_TREASURY_BASIS_POINT && 
                args.rewards_inflation <= LOW_REWARDS_BASIS_POINT, 
                TakeoverError::InvalidInflationAmounts
            );

            InflationAmount {
                level: Level::Low,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            }
        },
        1 => {
            require!(
                args.presale_inflation <= MEDIUM_PRESALE_BASIS_POINT 
                && args.treasury_inflation <= MEDIUM_TREASURY_BASIS_POINT 
                && args.rewards_inflation <= MEDIUM_REWARDS_BASIS_POINT, 
                TakeoverError::InvalidInflationAmounts
            );

            InflationAmount {
                level: Level::Medium,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            }
        },
        2 => {
            require!(
                args.presale_inflation <= HIGH_PRESALE_BASIS_POINT && 
                args.treasury_inflation <= HIGH_TREASURY_BASIS_POINT && 
                args.rewards_inflation <= HIGH_REWARDS_BASIS_POINT, 
                TakeoverError::InvalidInflationAmounts
            );

            InflationAmount {
                level: Level::High,
                presale_amount,
                treasury_amount, 
                rewards_amount,
                referral_amount
            }
        },
        _ => return Err(TakeoverError::InvalidFdmcValue.into()),
    };

    let swap_period = SwapPeriod {
        start: args.start,
        end: args.end,
    };

    // ACTIONS
    ctx.accounts.initialize_takeover(old_mints, inflation_amount.clone(), swap_period, args.takeover_wallet, args.presale_price, args.referral)?;

    match ctx.accounts.token_program.key() {
        TOKEN_2022_ID => {
            require!(args.token_extension.is_some(), TakeoverError::InvalidTokenExtensionArgs);
            ctx.accounts.initialize_extension_mint(args.name, args.symbol, args.uri, args.token_extension.unwrap(), decimals)?;
        },
        TOKEN_ID => {
            require!(args.token_extension.is_none(), TakeoverError::InvalidTokenExtensionArgs);
            ctx.accounts.initialize_native_mint(args.name, args.symbol, args.uri, decimals)?;
        },
        _ => return Err(TakeoverError::InvalidTokenProgram.into()),
    }

    ctx.accounts.finalize_mint(inflation_amount, supply)?;

    Ok(())
}