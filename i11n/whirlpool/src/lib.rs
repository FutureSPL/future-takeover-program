use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

// Accounts
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub config: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_mint_a: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_mint_b: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    /// CHECK: Skip check
    pub fee_tier: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeTickArray<'info> {
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeFeeTier<'info> {
    /// CHECK: Skip check
    pub config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub fee_tier: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeReward<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Skip check
    pub reward_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRewardEmissions<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub reward_vault: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OpenPositionWithMetadata<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_metadata_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metadata_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metadata_update_auth: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct IncreaseLiquidity<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_upper: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DecreaseLiquidity<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_lower: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_upper: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateFeesAndRewards<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub tick_array_lower: AccountInfo<'info>,
    /// CHECK: Skip check
    pub tick_array_upper: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectReward<'info> {
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub reward_owner_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub reward_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectProtocolFees<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub collect_protocol_fees_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_destination_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_destination_b: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Swap<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_0: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_1: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_2: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetDefaultFeeRate<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub fee_tier: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetDefaultProtocolFeeRate<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFeeRate<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetProtocolFeeRate<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetFeeAuthority<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub fee_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_fee_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetCollectProtocolFeesAuthority<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub collect_protocol_fees_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_collect_protocol_fees_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRewardAuthority<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_reward_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRewardAuthorityBySuperAuthority<'info> {
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_emissions_super_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_reward_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetRewardEmissionsSuperAuthority<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpools_config: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub reward_emissions_super_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_reward_emissions_super_authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TwoHopSwap<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool_one: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub whirlpool_two: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_one_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_one_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_one_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_one_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_two_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_two_a: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_owner_account_two_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_vault_two_b: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_one_0: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_one_1: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_one_2: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_two_0: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_two_1: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub tick_array_two_2: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_one: AccountInfo<'info>,
    /// CHECK: Skip check
    pub oracle_two: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializePositionBundle<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_bundle_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_bundle_owner: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializePositionBundleWithMetadata<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_bundle_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle_metadata: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_bundle_owner: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metadata_update_auth: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metadata_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct DeletePositionBundle<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_bundle_owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub receiver: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct OpenBundledPosition<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub bundled_position: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_bundle_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_bundle_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub whirlpool: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub funder: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseBundledPosition<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub bundled_position: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub position_bundle: AccountInfo<'info>,
    /// CHECK: Skip check
    pub position_bundle_token_account: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub position_bundle_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub receiver: AccountInfo<'info>,
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use anchor_i11n::Discriminator;
    use super::*;

    pub fn initialize_config<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeConfig<'info>>,
        fee_authority: Pubkey,
        collect_protocol_fees_authority: Pubkey,
        reward_emissions_super_authority: Pubkey,
        default_protocol_fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeConfig { fee_authority, collect_protocol_fees_authority, reward_emissions_super_authority, default_protocol_fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeConfig::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_pool<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializePool<'info>>,
        bumps: WhirlpoolBumps,
        tick_spacing: u16,
        initial_sqrt_price: u128
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializePool { bumps, tick_spacing, initial_sqrt_price };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializePool::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_tick_array<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeTickArray<'info>>,
        start_tick_index: i32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeTickArray { start_tick_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeTickArray::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_fee_tier<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeFeeTier<'info>>,
        tick_spacing: u16,
        default_fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeFeeTier { tick_spacing, default_fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeFeeTier::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_reward<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializeReward<'info>>,
        reward_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializeReward { reward_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializeReward::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_reward_emissions<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetRewardEmissions<'info>>,
        reward_index: u8,
        emissions_per_second_x_64: u128
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetRewardEmissions { reward_index, emissions_per_second_x_64 };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetRewardEmissions::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn open_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, OpenPosition<'info>>,
        bumps: OpenPositionBump,
        tick_lower_index: i32,
        tick_upper_index: i32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::OpenPosition { bumps, tick_lower_index, tick_upper_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::OpenPosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn open_position_with_metadata<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, OpenPositionWithMetadata<'info>>,
        bumps: OpenPositionWithMetadataBump,
        tick_lower_index: i32,
        tick_upper_index: i32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::OpenPositionWithMetadata { bumps, tick_lower_index, tick_upper_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::OpenPositionWithMetadata::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn increase_liquidity<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, IncreaseLiquidity<'info>>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::IncreaseLiquidity { liquidity_amount, token_max_a, token_max_b };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::IncreaseLiquidity::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn decrease_liquidity<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DecreaseLiquidity<'info>>,
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DecreaseLiquidity { liquidity_amount, token_min_a, token_min_b };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DecreaseLiquidity::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn update_fees_and_rewards<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateFeesAndRewards<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateFeesAndRewards {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateFeesAndRewards::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn collect_fees<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectFees<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectFees {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectFees::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn collect_reward<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectReward<'info>>,
        reward_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectReward { reward_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectReward::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn collect_protocol_fees<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectProtocolFees<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectProtocolFees {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectProtocolFees::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn swap<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Swap<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Swap { amount, other_amount_threshold, sqrt_price_limit, amount_specified_is_input, a_to_b };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Swap::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn close_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ClosePosition<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ClosePosition {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ClosePosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_default_fee_rate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetDefaultFeeRate<'info>>,
        default_fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetDefaultFeeRate { default_fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetDefaultFeeRate::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_default_protocol_fee_rate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetDefaultProtocolFeeRate<'info>>,
        default_protocol_fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetDefaultProtocolFeeRate { default_protocol_fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetDefaultProtocolFeeRate::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_fee_rate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetFeeRate<'info>>,
        fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetFeeRate { fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetFeeRate::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_protocol_fee_rate<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetProtocolFeeRate<'info>>,
        protocol_fee_rate: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetProtocolFeeRate { protocol_fee_rate };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetProtocolFeeRate::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_fee_authority<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetFeeAuthority<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetFeeAuthority {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetFeeAuthority::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_collect_protocol_fees_authority<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetCollectProtocolFeesAuthority<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetCollectProtocolFeesAuthority {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetCollectProtocolFeesAuthority::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_reward_authority<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetRewardAuthority<'info>>,
        reward_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetRewardAuthority { reward_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetRewardAuthority::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_reward_authority_by_super_authority<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetRewardAuthorityBySuperAuthority<'info>>,
        reward_index: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetRewardAuthorityBySuperAuthority { reward_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetRewardAuthorityBySuperAuthority::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn set_reward_emissions_super_authority<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetRewardEmissionsSuperAuthority<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetRewardEmissionsSuperAuthority {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetRewardEmissionsSuperAuthority::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn two_hop_swap<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, TwoHopSwap<'info>>,
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::TwoHopSwap { amount, other_amount_threshold, amount_specified_is_input, a_to_b_one, a_to_b_two, sqrt_price_limit_one, sqrt_price_limit_two };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::TwoHopSwap::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_position_bundle<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializePositionBundle<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializePositionBundle {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializePositionBundle::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn initialize_position_bundle_with_metadata<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, InitializePositionBundleWithMetadata<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::InitializePositionBundleWithMetadata {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::InitializePositionBundleWithMetadata::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn delete_position_bundle<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, DeletePositionBundle<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::DeletePositionBundle {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::DeletePositionBundle::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn open_bundled_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, OpenBundledPosition<'info>>,
        bundle_index: u16,
        tick_lower_index: i32,
        tick_upper_index: i32
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::OpenBundledPosition { bundle_index, tick_lower_index, tick_upper_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::OpenBundledPosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }

    pub fn close_bundled_position<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CloseBundledPosition<'info>>,
        bundle_index: u16
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CloseBundledPosition { bundle_index };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CloseBundledPosition::DISCRIMINATOR);
            AnchorSerialize::serialize(&ix, &mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotSerialize)?;
            let accounts = ctx.to_account_metas(None);
            anchor_lang::solana_program::instruction::Instruction {
                program_id: ctx.program.key(),
                accounts,
                data,
            }
        };
        let mut acc_infos = ctx.to_account_infos();
        anchor_lang::solana_program::program::invoke_signed(&ix, &acc_infos, ctx.signer_seeds)
            .map_or_else(|e| Err(Into::into(e)), |_| Ok(()))
    }  
}

// I11n
pub mod i11n {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::{instructions::*, ID};

    // InitializeConfig
    #[derive(TryFromInstruction)]
    pub struct InitializeConfigI11n<'info> {
        pub accounts: InitializeConfigAccountMetas<'info>,
        pub args: InitializeConfig,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializePool
    #[derive(TryFromInstruction)]
    pub struct InitializePoolI11n<'info> {
        pub accounts: InitializePoolAccountMetas<'info>,
        pub args: InitializePool,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeTickArray
    #[derive(TryFromInstruction)]
    pub struct InitializeTickArrayI11n<'info> {
        pub accounts: InitializeTickArrayAccountMetas<'info>,
        pub args: InitializeTickArray,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeFeeTier
    #[derive(TryFromInstruction)]
    pub struct InitializeFeeTierI11n<'info> {
        pub accounts: InitializeFeeTierAccountMetas<'info>,
        pub args: InitializeFeeTier,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializeReward
    #[derive(TryFromInstruction)]
    pub struct InitializeRewardI11n<'info> {
        pub accounts: InitializeRewardAccountMetas<'info>,
        pub args: InitializeReward,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetRewardEmissions
    #[derive(TryFromInstruction)]
    pub struct SetRewardEmissionsI11n<'info> {
        pub accounts: SetRewardEmissionsAccountMetas<'info>,
        pub args: SetRewardEmissions,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // OpenPosition
    #[derive(TryFromInstruction)]
    pub struct OpenPositionI11n<'info> {
        pub accounts: OpenPositionAccountMetas<'info>,
        pub args: OpenPosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // OpenPositionWithMetadata
    #[derive(TryFromInstruction)]
    pub struct OpenPositionWithMetadataI11n<'info> {
        pub accounts: OpenPositionWithMetadataAccountMetas<'info>,
        pub args: OpenPositionWithMetadata,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // IncreaseLiquidity
    #[derive(TryFromInstruction)]
    pub struct IncreaseLiquidityI11n<'info> {
        pub accounts: IncreaseLiquidityAccountMetas<'info>,
        pub args: IncreaseLiquidity,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DecreaseLiquidity
    #[derive(TryFromInstruction)]
    pub struct DecreaseLiquidityI11n<'info> {
        pub accounts: DecreaseLiquidityAccountMetas<'info>,
        pub args: DecreaseLiquidity,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateFeesAndRewards
    #[derive(TryFromInstruction)]
    pub struct UpdateFeesAndRewardsI11n<'info> {
        pub accounts: UpdateFeesAndRewardsAccountMetas<'info>,
        pub args: UpdateFeesAndRewards,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectFees
    #[derive(TryFromInstruction)]
    pub struct CollectFeesI11n<'info> {
        pub accounts: CollectFeesAccountMetas<'info>,
        pub args: CollectFees,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectReward
    #[derive(TryFromInstruction)]
    pub struct CollectRewardI11n<'info> {
        pub accounts: CollectRewardAccountMetas<'info>,
        pub args: CollectReward,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectProtocolFees
    #[derive(TryFromInstruction)]
    pub struct CollectProtocolFeesI11n<'info> {
        pub accounts: CollectProtocolFeesAccountMetas<'info>,
        pub args: CollectProtocolFees,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Swap
    #[derive(TryFromInstruction)]
    pub struct SwapI11n<'info> {
        pub accounts: SwapAccountMetas<'info>,
        pub args: Swap,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ClosePosition
    #[derive(TryFromInstruction)]
    pub struct ClosePositionI11n<'info> {
        pub accounts: ClosePositionAccountMetas<'info>,
        pub args: ClosePosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetDefaultFeeRate
    #[derive(TryFromInstruction)]
    pub struct SetDefaultFeeRateI11n<'info> {
        pub accounts: SetDefaultFeeRateAccountMetas<'info>,
        pub args: SetDefaultFeeRate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetDefaultProtocolFeeRate
    #[derive(TryFromInstruction)]
    pub struct SetDefaultProtocolFeeRateI11n<'info> {
        pub accounts: SetDefaultProtocolFeeRateAccountMetas<'info>,
        pub args: SetDefaultProtocolFeeRate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetFeeRate
    #[derive(TryFromInstruction)]
    pub struct SetFeeRateI11n<'info> {
        pub accounts: SetFeeRateAccountMetas<'info>,
        pub args: SetFeeRate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetProtocolFeeRate
    #[derive(TryFromInstruction)]
    pub struct SetProtocolFeeRateI11n<'info> {
        pub accounts: SetProtocolFeeRateAccountMetas<'info>,
        pub args: SetProtocolFeeRate,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetFeeAuthority
    #[derive(TryFromInstruction)]
    pub struct SetFeeAuthorityI11n<'info> {
        pub accounts: SetFeeAuthorityAccountMetas<'info>,
        pub args: SetFeeAuthority,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetCollectProtocolFeesAuthority
    #[derive(TryFromInstruction)]
    pub struct SetCollectProtocolFeesAuthorityI11n<'info> {
        pub accounts: SetCollectProtocolFeesAuthorityAccountMetas<'info>,
        pub args: SetCollectProtocolFeesAuthority,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetRewardAuthority
    #[derive(TryFromInstruction)]
    pub struct SetRewardAuthorityI11n<'info> {
        pub accounts: SetRewardAuthorityAccountMetas<'info>,
        pub args: SetRewardAuthority,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetRewardAuthorityBySuperAuthority
    #[derive(TryFromInstruction)]
    pub struct SetRewardAuthorityBySuperAuthorityI11n<'info> {
        pub accounts: SetRewardAuthorityBySuperAuthorityAccountMetas<'info>,
        pub args: SetRewardAuthorityBySuperAuthority,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetRewardEmissionsSuperAuthority
    #[derive(TryFromInstruction)]
    pub struct SetRewardEmissionsSuperAuthorityI11n<'info> {
        pub accounts: SetRewardEmissionsSuperAuthorityAccountMetas<'info>,
        pub args: SetRewardEmissionsSuperAuthority,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // TwoHopSwap
    #[derive(TryFromInstruction)]
    pub struct TwoHopSwapI11n<'info> {
        pub accounts: TwoHopSwapAccountMetas<'info>,
        pub args: TwoHopSwap,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializePositionBundle
    #[derive(TryFromInstruction)]
    pub struct InitializePositionBundleI11n<'info> {
        pub accounts: InitializePositionBundleAccountMetas<'info>,
        pub args: InitializePositionBundle,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // InitializePositionBundleWithMetadata
    #[derive(TryFromInstruction)]
    pub struct InitializePositionBundleWithMetadataI11n<'info> {
        pub accounts: InitializePositionBundleWithMetadataAccountMetas<'info>,
        pub args: InitializePositionBundleWithMetadata,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // DeletePositionBundle
    #[derive(TryFromInstruction)]
    pub struct DeletePositionBundleI11n<'info> {
        pub accounts: DeletePositionBundleAccountMetas<'info>,
        pub args: DeletePositionBundle,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // OpenBundledPosition
    #[derive(TryFromInstruction)]
    pub struct OpenBundledPositionI11n<'info> {
        pub accounts: OpenBundledPositionAccountMetas<'info>,
        pub args: OpenBundledPosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CloseBundledPosition
    #[derive(TryFromInstruction)]
    pub struct CloseBundledPositionI11n<'info> {
        pub accounts: CloseBundledPositionAccountMetas<'info>,
        pub args: CloseBundledPosition,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct InitializeConfigAccountMetas<'info> {
        pub config: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializePoolAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub token_mint_a: &'info AccountMeta,
        pub token_mint_b: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub fee_tier: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeTickArrayAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub tick_array: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeFeeTierAccountMetas<'info> {
        pub config: &'info AccountMeta,
        pub fee_tier: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeRewardAccountMetas<'info> {
        pub reward_authority: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub reward_mint: &'info AccountMeta,
        pub reward_vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetRewardEmissionsAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub reward_authority: &'info AccountMeta,
        pub reward_vault: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct OpenPositionAccountMetas<'info> {
        pub funder: &'info AccountMeta,
        pub owner: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_mint: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct OpenPositionWithMetadataAccountMetas<'info> {
        pub funder: &'info AccountMeta,
        pub owner: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_mint: &'info AccountMeta,
        pub position_metadata_account: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub metadata_program: &'info AccountMeta,
        pub metadata_update_auth: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct IncreaseLiquidityAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub position_authority: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub token_owner_account_a: &'info AccountMeta,
        pub token_owner_account_b: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub tick_array_lower: &'info AccountMeta,
        pub tick_array_upper: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DecreaseLiquidityAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub position_authority: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub token_owner_account_a: &'info AccountMeta,
        pub token_owner_account_b: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub tick_array_lower: &'info AccountMeta,
        pub tick_array_upper: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateFeesAndRewardsAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub tick_array_lower: &'info AccountMeta,
        pub tick_array_upper: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectFeesAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub position_authority: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub token_owner_account_a: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_owner_account_b: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectRewardAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub position_authority: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub reward_owner_account: &'info AccountMeta,
        pub reward_vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectProtocolFeesAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub collect_protocol_fees_authority: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub token_destination_a: &'info AccountMeta,
        pub token_destination_b: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SwapAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub token_authority: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub token_owner_account_a: &'info AccountMeta,
        pub token_vault_a: &'info AccountMeta,
        pub token_owner_account_b: &'info AccountMeta,
        pub token_vault_b: &'info AccountMeta,
        pub tick_array_0: &'info AccountMeta,
        pub tick_array_1: &'info AccountMeta,
        pub tick_array_2: &'info AccountMeta,
        pub oracle: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClosePositionAccountMetas<'info> {
        pub position_authority: &'info AccountMeta,
        pub receiver: &'info AccountMeta,
        pub position: &'info AccountMeta,
        pub position_mint: &'info AccountMeta,
        pub position_token_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetDefaultFeeRateAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub fee_tier: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetDefaultProtocolFeeRateAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetFeeRateAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetProtocolFeeRateAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetFeeAuthorityAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub fee_authority: &'info AccountMeta,
        pub new_fee_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetCollectProtocolFeesAuthorityAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub collect_protocol_fees_authority: &'info AccountMeta,
        pub new_collect_protocol_fees_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetRewardAuthorityAccountMetas<'info> {
        pub whirlpool: &'info AccountMeta,
        pub reward_authority: &'info AccountMeta,
        pub new_reward_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetRewardAuthorityBySuperAuthorityAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub reward_emissions_super_authority: &'info AccountMeta,
        pub new_reward_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetRewardEmissionsSuperAuthorityAccountMetas<'info> {
        pub whirlpools_config: &'info AccountMeta,
        pub reward_emissions_super_authority: &'info AccountMeta,
        pub new_reward_emissions_super_authority: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct TwoHopSwapAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub token_authority: &'info AccountMeta,
        pub whirlpool_one: &'info AccountMeta,
        pub whirlpool_two: &'info AccountMeta,
        pub token_owner_account_one_a: &'info AccountMeta,
        pub token_vault_one_a: &'info AccountMeta,
        pub token_owner_account_one_b: &'info AccountMeta,
        pub token_vault_one_b: &'info AccountMeta,
        pub token_owner_account_two_a: &'info AccountMeta,
        pub token_vault_two_a: &'info AccountMeta,
        pub token_owner_account_two_b: &'info AccountMeta,
        pub token_vault_two_b: &'info AccountMeta,
        pub tick_array_one_0: &'info AccountMeta,
        pub tick_array_one_1: &'info AccountMeta,
        pub tick_array_one_2: &'info AccountMeta,
        pub tick_array_two_0: &'info AccountMeta,
        pub tick_array_two_1: &'info AccountMeta,
        pub tick_array_two_2: &'info AccountMeta,
        pub oracle_one: &'info AccountMeta,
        pub oracle_two: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializePositionBundleAccountMetas<'info> {
        pub position_bundle: &'info AccountMeta,
        pub position_bundle_mint: &'info AccountMeta,
        pub position_bundle_token_account: &'info AccountMeta,
        pub position_bundle_owner: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializePositionBundleWithMetadataAccountMetas<'info> {
        pub position_bundle: &'info AccountMeta,
        pub position_bundle_mint: &'info AccountMeta,
        pub position_bundle_metadata: &'info AccountMeta,
        pub position_bundle_token_account: &'info AccountMeta,
        pub position_bundle_owner: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub metadata_update_auth: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub metadata_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DeletePositionBundleAccountMetas<'info> {
        pub position_bundle: &'info AccountMeta,
        pub position_bundle_mint: &'info AccountMeta,
        pub position_bundle_token_account: &'info AccountMeta,
        pub position_bundle_owner: &'info AccountMeta,
        pub receiver: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct OpenBundledPositionAccountMetas<'info> {
        pub bundled_position: &'info AccountMeta,
        pub position_bundle: &'info AccountMeta,
        pub position_bundle_token_account: &'info AccountMeta,
        pub position_bundle_authority: &'info AccountMeta,
        pub whirlpool: &'info AccountMeta,
        pub funder: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CloseBundledPositionAccountMetas<'info> {
        pub bundled_position: &'info AccountMeta,
        pub position_bundle: &'info AccountMeta,
        pub position_bundle_token_account: &'info AccountMeta,
        pub position_bundle_authority: &'info AccountMeta,
        pub receiver: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeConfig {
        pub fee_authority: Pubkey,
        pub collect_protocol_fees_authority: Pubkey,
        pub reward_emissions_super_authority: Pubkey,
        pub default_protocol_fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePool {
        pub bumps: WhirlpoolBumps,
        pub tick_spacing: u16,
        pub initial_sqrt_price: u128,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeTickArray {
        pub start_tick_index: i32,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeFeeTier {
        pub tick_spacing: u16,
        pub default_fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeReward {
        pub reward_index: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetRewardEmissions {
        pub reward_index: u8,
        pub emissions_per_second_x_64: u128,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct OpenPosition {
        pub bumps: OpenPositionBump,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct OpenPositionWithMetadata {
        pub bumps: OpenPositionWithMetadataBump,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct IncreaseLiquidity {
        pub liquidity_amount: u128,
        pub token_max_a: u64,
        pub token_max_b: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct DecreaseLiquidity {
        pub liquidity_amount: u128,
        pub token_min_a: u64,
        pub token_min_b: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateFeesAndRewards {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CollectFees {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CollectReward {
        pub reward_index: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CollectProtocolFees {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Swap {
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub sqrt_price_limit: u128,
        pub amount_specified_is_input: bool,
        pub a_to_b: bool,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ClosePosition {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetDefaultFeeRate {
        pub default_fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetDefaultProtocolFeeRate {
        pub default_protocol_fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetFeeRate {
        pub fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetProtocolFeeRate {
        pub protocol_fee_rate: u16,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetFeeAuthority {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetCollectProtocolFeesAuthority {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetRewardAuthority {
        pub reward_index: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetRewardAuthorityBySuperAuthority {
        pub reward_index: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetRewardEmissionsSuperAuthority {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct TwoHopSwap {
        pub amount: u64,
        pub other_amount_threshold: u64,
        pub amount_specified_is_input: bool,
        pub a_to_b_one: bool,
        pub a_to_b_two: bool,
        pub sqrt_price_limit_one: u128,
        pub sqrt_price_limit_two: u128,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePositionBundle {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePositionBundleWithMetadata {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct DeletePositionBundle {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct OpenBundledPosition {
        pub bundle_index: u16,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CloseBundledPosition {
        pub bundle_index: u16,
    }        
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

   #[account]
    pub struct WhirlpoolsConfig {
        pub fee_authority: Pubkey,
        pub collect_protocol_fees_authority: Pubkey,
        pub reward_emissions_super_authority: Pubkey,
        pub default_protocol_fee_rate: u16
    }

   #[account]
    pub struct FeeTier {
        pub whirlpools_config: Pubkey,
        pub tick_spacing: u16,
        pub default_fee_rate: u16
    }

   #[account]
    pub struct PositionBundle {
        pub position_bundle_mint: Pubkey,
        pub position_bitmap: [u8;32]
    }

   #[account]
    pub struct Position {
        pub whirlpool: Pubkey,
        pub position_mint: Pubkey,
        pub liquidity: u128,
        pub tick_lower_index: i32,
        pub tick_upper_index: i32,
        pub fee_growth_checkpoint_a: u128,
        pub fee_owed_a: u64,
        pub fee_growth_checkpoint_b: u128,
        pub fee_owed_b: u64,
        pub reward_infos: [PositionRewardInfo;3]
    }

   #[account]
    pub struct TickArray {
        pub start_tick_index: i32,
        pub ticks: [Tick;88],
        pub whirlpool: Pubkey
    }

   #[account]
    pub struct Whirlpool {
        pub whirlpools_config: Pubkey,
        pub whirlpool_bump: [u8;1],
        pub tick_spacing: u16,
        pub tick_spacing_seed: [u8;2],
        pub fee_rate: u16,
        pub protocol_fee_rate: u16,
        pub liquidity: u128,
        pub sqrt_price: u128,
        pub tick_current_index: i32,
        pub protocol_fee_owed_a: u64,
        pub protocol_fee_owed_b: u64,
        pub token_mint_a: Pubkey,
        pub token_vault_a: Pubkey,
        pub fee_growth_global_a: u128,
        pub token_mint_b: Pubkey,
        pub token_vault_b: Pubkey,
        pub fee_growth_global_b: u128,
        pub reward_last_updated_timestamp: u64,
        pub reward_infos: [WhirlpoolRewardInfo;3]
    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OpenPositionBump {
    pub position_bump: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct OpenPositionWithMetadataBump {
    pub position_bump: u8,
    pub metadata_bump: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PositionRewardInfo {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Tick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128;3],
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhirlpoolRewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub authority: Pubkey,
    pub emissions_per_second_x_64: u128,
    pub growth_global_x_64: u128,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum CurrIndex {
    Below,
    Inside,
    Above
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum TickLabel {
    Upper,
    Lower
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right
}