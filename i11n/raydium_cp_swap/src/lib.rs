use anchor_lang::prelude::*;

declare_id!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");

// Accounts
#[derive(Accounts)]
pub struct CreateAmmConfig<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateAmmConfig<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePoolStatus<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectProtocolFee<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_0_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_1_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub recipient_token_0_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub recipient_token_1_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CollectFundFee<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_0_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_1_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub recipient_token_0_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub recipient_token_1_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub creator: AccountInfo<'info>,
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_0_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_1_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub lp_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub creator_token_0: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub creator_token_1: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub creator_lp_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub create_pool_fee: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub observation_state: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_0_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_1_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub owner_lp_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_0_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_1_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub lp_mint: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub owner: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub owner_lp_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_0_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub token_1_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program_2022: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_0_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub vault_1_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub lp_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub memo_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapBaseInput<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub input_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub output_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub input_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub output_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub input_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub output_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub input_token_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub output_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub observation_state: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapBaseOutput<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub amm_config: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub pool_state: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub input_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub output_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub input_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub output_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub input_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub output_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub input_token_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub output_token_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub observation_state: AccountInfo<'info>,
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use anchor_i11n::Discriminator;
    use super::*;

    pub fn create_amm_config<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateAmmConfig<'info>>,
        index: u16,
        trade_fee_rate: u64,
        protocol_fee_rate: u64,
        fund_fee_rate: u64,
        create_pool_fee: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateAmmConfig { index, trade_fee_rate, protocol_fee_rate, fund_fee_rate, create_pool_fee };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateAmmConfig::DISCRIMINATOR);
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

    pub fn update_amm_config<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateAmmConfig<'info>>,
        param: u8,
        value: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateAmmConfig { param, value };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateAmmConfig::DISCRIMINATOR);
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

    pub fn update_pool_status<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdatePoolStatus<'info>>,
        status: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdatePoolStatus { status };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdatePoolStatus::DISCRIMINATOR);
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

    pub fn collect_protocol_fee<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectProtocolFee<'info>>,
        amount_0_requested: u64,
        amount_1_requested: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectProtocolFee { amount_0_requested, amount_1_requested };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectProtocolFee::DISCRIMINATOR);
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

    pub fn collect_fund_fee<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CollectFundFee<'info>>,
        amount_0_requested: u64,
        amount_1_requested: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CollectFundFee { amount_0_requested, amount_1_requested };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CollectFundFee::DISCRIMINATOR);
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

    pub fn initialize<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Initialize<'info>>,
        init_amount_0: u64,
        init_amount_1: u64,
        open_time: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Initialize { init_amount_0, init_amount_1, open_time };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Initialize::DISCRIMINATOR);
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

    pub fn deposit<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Deposit<'info>>,
        lp_token_amount: u64,
        maximum_token_0_amount: u64,
        maximum_token_1_amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Deposit { lp_token_amount, maximum_token_0_amount, maximum_token_1_amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Deposit::DISCRIMINATOR);
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

    pub fn withdraw<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Withdraw<'info>>,
        lp_token_amount: u64,
        minimum_token_0_amount: u64,
        minimum_token_1_amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Withdraw { lp_token_amount, minimum_token_0_amount, minimum_token_1_amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Withdraw::DISCRIMINATOR);
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

    pub fn swap_base_input<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseInput<'info>>,
        amount_in: u64,
        minimum_amount_out: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SwapBaseInput { amount_in, minimum_amount_out };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SwapBaseInput::DISCRIMINATOR);
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

    pub fn swap_base_output<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SwapBaseOutput<'info>>,
        max_amount_in: u64,
        amount_out: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SwapBaseOutput { max_amount_in, amount_out };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SwapBaseOutput::DISCRIMINATOR);
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

    // CreateAmmConfig
    #[derive(TryFromInstruction)]
    pub struct CreateAmmConfigI11n<'info> {
        pub accounts: CreateAmmConfigAccountMetas<'info>,
        pub args: CreateAmmConfig,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateAmmConfig
    #[derive(TryFromInstruction)]
    pub struct UpdateAmmConfigI11n<'info> {
        pub accounts: UpdateAmmConfigAccountMetas<'info>,
        pub args: UpdateAmmConfig,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdatePoolStatus
    #[derive(TryFromInstruction)]
    pub struct UpdatePoolStatusI11n<'info> {
        pub accounts: UpdatePoolStatusAccountMetas<'info>,
        pub args: UpdatePoolStatus,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectProtocolFee
    #[derive(TryFromInstruction)]
    pub struct CollectProtocolFeeI11n<'info> {
        pub accounts: CollectProtocolFeeAccountMetas<'info>,
        pub args: CollectProtocolFee,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CollectFundFee
    #[derive(TryFromInstruction)]
    pub struct CollectFundFeeI11n<'info> {
        pub accounts: CollectFundFeeAccountMetas<'info>,
        pub args: CollectFundFee,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Initialize
    #[derive(TryFromInstruction)]
    pub struct InitializeI11n<'info> {
        pub accounts: InitializeAccountMetas<'info>,
        pub args: Initialize,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Deposit
    #[derive(TryFromInstruction)]
    pub struct DepositI11n<'info> {
        pub accounts: DepositAccountMetas<'info>,
        pub args: Deposit,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Withdraw
    #[derive(TryFromInstruction)]
    pub struct WithdrawI11n<'info> {
        pub accounts: WithdrawAccountMetas<'info>,
        pub args: Withdraw,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SwapBaseInput
    #[derive(TryFromInstruction)]
    pub struct SwapBaseInputI11n<'info> {
        pub accounts: SwapBaseInputAccountMetas<'info>,
        pub args: SwapBaseInput,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SwapBaseOutput
    #[derive(TryFromInstruction)]
    pub struct SwapBaseOutputI11n<'info> {
        pub accounts: SwapBaseOutputAccountMetas<'info>,
        pub args: SwapBaseOutput,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct CreateAmmConfigAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateAmmConfigAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdatePoolStatusAccountMetas<'info> {
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectProtocolFeeAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub token_0_vault: &'info AccountMeta,
        pub token_1_vault: &'info AccountMeta,
        pub vault_0_mint: &'info AccountMeta,
        pub vault_1_mint: &'info AccountMeta,
        pub recipient_token_0_account: &'info AccountMeta,
        pub recipient_token_1_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CollectFundFeeAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub token_0_vault: &'info AccountMeta,
        pub token_1_vault: &'info AccountMeta,
        pub vault_0_mint: &'info AccountMeta,
        pub vault_1_mint: &'info AccountMeta,
        pub recipient_token_0_account: &'info AccountMeta,
        pub recipient_token_1_account: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct InitializeAccountMetas<'info> {
        pub creator: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub token_0_mint: &'info AccountMeta,
        pub token_1_mint: &'info AccountMeta,
        pub lp_mint: &'info AccountMeta,
        pub creator_token_0: &'info AccountMeta,
        pub creator_token_1: &'info AccountMeta,
        pub creator_lp_token: &'info AccountMeta,
        pub token_0_vault: &'info AccountMeta,
        pub token_1_vault: &'info AccountMeta,
        pub create_pool_fee: &'info AccountMeta,
        pub observation_state: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub token_0_program: &'info AccountMeta,
        pub token_1_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct DepositAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub owner_lp_token: &'info AccountMeta,
        pub token_0_account: &'info AccountMeta,
        pub token_1_account: &'info AccountMeta,
        pub token_0_vault: &'info AccountMeta,
        pub token_1_vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
        pub vault_0_mint: &'info AccountMeta,
        pub vault_1_mint: &'info AccountMeta,
        pub lp_mint: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct WithdrawAccountMetas<'info> {
        pub owner: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub owner_lp_token: &'info AccountMeta,
        pub token_0_account: &'info AccountMeta,
        pub token_1_account: &'info AccountMeta,
        pub token_0_vault: &'info AccountMeta,
        pub token_1_vault: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub token_program_2022: &'info AccountMeta,
        pub vault_0_mint: &'info AccountMeta,
        pub vault_1_mint: &'info AccountMeta,
        pub lp_mint: &'info AccountMeta,
        pub memo_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SwapBaseInputAccountMetas<'info> {
        pub payer: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub input_token_account: &'info AccountMeta,
        pub output_token_account: &'info AccountMeta,
        pub input_vault: &'info AccountMeta,
        pub output_vault: &'info AccountMeta,
        pub input_token_program: &'info AccountMeta,
        pub output_token_program: &'info AccountMeta,
        pub input_token_mint: &'info AccountMeta,
        pub output_token_mint: &'info AccountMeta,
        pub observation_state: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SwapBaseOutputAccountMetas<'info> {
        pub payer: &'info AccountMeta,
        pub authority: &'info AccountMeta,
        pub amm_config: &'info AccountMeta,
        pub pool_state: &'info AccountMeta,
        pub input_token_account: &'info AccountMeta,
        pub output_token_account: &'info AccountMeta,
        pub input_vault: &'info AccountMeta,
        pub output_vault: &'info AccountMeta,
        pub input_token_program: &'info AccountMeta,
        pub output_token_program: &'info AccountMeta,
        pub input_token_mint: &'info AccountMeta,
        pub output_token_mint: &'info AccountMeta,
        pub observation_state: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreateAmmConfig {
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAmmConfig {
        pub param: u8,
        pub value: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePoolStatus {
        pub status: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CollectProtocolFee {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CollectFundFee {
        pub amount_0_requested: u64,
        pub amount_1_requested: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Initialize {
        pub init_amount_0: u64,
        pub init_amount_1: u64,
        pub open_time: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Deposit {
        pub lp_token_amount: u64,
        pub maximum_token_0_amount: u64,
        pub maximum_token_1_amount: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Withdraw {
        pub lp_token_amount: u64,
        pub minimum_token_0_amount: u64,
        pub minimum_token_1_amount: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SwapBaseInput {
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SwapBaseOutput {
        pub max_amount_in: u64,
        pub amount_out: u64,
    }        
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

   #[account]
    pub struct AmmConfig {
        pub bump: u8,
        pub disable_create_pool: bool,
        pub index: u16,
        pub trade_fee_rate: u64,
        pub protocol_fee_rate: u64,
        pub fund_fee_rate: u64,
        pub create_pool_fee: u64,
        pub protocol_owner: Pubkey,
        pub fund_owner: Pubkey,
        pub padding: [u64;16]
    }

   #[account]
    pub struct ObservationState {
        pub initialized: bool,
        pub observation_index: u16,
        pub pool_id: Pubkey,
        pub observations: [Observation;100],
        pub padding: [u64;4]
    }

   #[account]
    pub struct PoolState {
        pub amm_config: Pubkey,
        pub pool_creator: Pubkey,
        pub token_0_vault: Pubkey,
        pub token_1_vault: Pubkey,
        pub lp_mint: Pubkey,
        pub token_0_mint: Pubkey,
        pub token_1_mint: Pubkey,
        pub token_0_program: Pubkey,
        pub token_1_program: Pubkey,
        pub observation_key: Pubkey,
        pub auth_bump: u8,
        pub status: u8,
        pub lp_mint_decimals: u8,
        pub mint_0_decimals: u8,
        pub mint_1_decimals: u8,
        pub lp_supply: u64,
        pub protocol_fees_token_0: u64,
        pub protocol_fees_token_1: u64,
        pub fund_fees_token_0: u64,
        pub fund_fees_token_1: u64,
        pub open_time: u64,
        pub padding: [u64;32]
    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Observation {
    pub block_timestamp: u64,
    pub cumulative_token_0_price_x_32: u128,
    pub cumulative_token_1_price_x_32: u128,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum TradeDirection {
    ZeroForOne,
    OneForZero
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum RoundDirection {
    Floor,
    Ceiling
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum PoolStatusBitIndex {
    Deposit,
    Withdraw,
    Swap
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable
}