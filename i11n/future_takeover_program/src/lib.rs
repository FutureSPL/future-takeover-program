use anchor_lang::prelude::*;

declare_id!("DWF1qAyiiKbSsuRSBBnJmQF6ePkCbtWsks8YwXVrPCcN");

// Accounts
#[derive(Accounts)]
pub struct AdminInit<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct AdminDelete<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub old_admin: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTakeover<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metadata: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub sysvar_instruction_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub metaplex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateTakeover<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CancelTakeover<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BuyPresale<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub presale_receipt: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SwapOldToken<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint_takeover_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FinalizeTakeover<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SellToken<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub wsol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint_admin_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub wsol_admin_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint_takeover_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_sol_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub instructions: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct FinalizeSell<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub admin: AccountInfo<'info>,
    /// CHECK: Skip check
    pub admin_profile: AccountInfo<'info>,
    /// CHECK: Skip check
    pub wsol: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub wsol_admin_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover_sol_vault: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub instructions: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub new_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub takeover: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub presale_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub old_mint_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub presale_receipt: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_new_mint_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub user_old_mint_token: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use anchor_i11n::Discriminator;
    use super::*;

    pub fn admin_init<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AdminInit<'info>>,
        username: String
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AdminInit { username };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AdminInit::DISCRIMINATOR);
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

    pub fn admin_delete<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, AdminDelete<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::AdminDelete {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::AdminDelete::DISCRIMINATOR);
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

    pub fn create_takeover<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateTakeover<'info>>,
        create_takeover_args: CreateTakeoverArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateTakeover { create_takeover_args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateTakeover::DISCRIMINATOR);
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

    pub fn update_takeover<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, UpdateTakeover<'info>>,
        update_takeover_args: UpdateTakeoverArgs
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::UpdateTakeover { update_takeover_args };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::UpdateTakeover::DISCRIMINATOR);
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

    pub fn cancel_takeover<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CancelTakeover<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CancelTakeover {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CancelTakeover::DISCRIMINATOR);
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

    pub fn buy_presale<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, BuyPresale<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::BuyPresale { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::BuyPresale::DISCRIMINATOR);
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

    pub fn swap_old_token<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SwapOldToken<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SwapOldToken {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SwapOldToken::DISCRIMINATOR);
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

    pub fn finalize_takeover<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, FinalizeTakeover<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::FinalizeTakeover {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::FinalizeTakeover::DISCRIMINATOR);
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

    pub fn sell_token<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SellToken<'info>>,
        amount: u64
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SellToken { amount };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SellToken::DISCRIMINATOR);
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

    pub fn finalize_sell<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, FinalizeSell<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::FinalizeSell {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::FinalizeSell::DISCRIMINATOR);
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

    pub fn claim_refund<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ClaimRefund<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ClaimRefund {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ClaimRefund::DISCRIMINATOR);
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

    // AdminInit
    #[derive(TryFromInstruction)]
    pub struct AdminInitI11n<'info> {
        pub accounts: AdminInitAccountMetas<'info>,
        pub args: AdminInit,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // AdminDelete
    #[derive(TryFromInstruction)]
    pub struct AdminDeleteI11n<'info> {
        pub accounts: AdminDeleteAccountMetas<'info>,
        pub args: AdminDelete,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateTakeover
    #[derive(TryFromInstruction)]
    pub struct CreateTakeoverI11n<'info> {
        pub accounts: CreateTakeoverAccountMetas<'info>,
        pub args: CreateTakeover,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // UpdateTakeover
    #[derive(TryFromInstruction)]
    pub struct UpdateTakeoverI11n<'info> {
        pub accounts: UpdateTakeoverAccountMetas<'info>,
        pub args: UpdateTakeover,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CancelTakeover
    #[derive(TryFromInstruction)]
    pub struct CancelTakeoverI11n<'info> {
        pub accounts: CancelTakeoverAccountMetas<'info>,
        pub args: CancelTakeover,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // BuyPresale
    #[derive(TryFromInstruction)]
    pub struct BuyPresaleI11n<'info> {
        pub accounts: BuyPresaleAccountMetas<'info>,
        pub args: BuyPresale,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SwapOldToken
    #[derive(TryFromInstruction)]
    pub struct SwapOldTokenI11n<'info> {
        pub accounts: SwapOldTokenAccountMetas<'info>,
        pub args: SwapOldToken,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // FinalizeTakeover
    #[derive(TryFromInstruction)]
    pub struct FinalizeTakeoverI11n<'info> {
        pub accounts: FinalizeTakeoverAccountMetas<'info>,
        pub args: FinalizeTakeover,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SellToken
    #[derive(TryFromInstruction)]
    pub struct SellTokenI11n<'info> {
        pub accounts: SellTokenAccountMetas<'info>,
        pub args: SellToken,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // FinalizeSell
    #[derive(TryFromInstruction)]
    pub struct FinalizeSellI11n<'info> {
        pub accounts: FinalizeSellAccountMetas<'info>,
        pub args: FinalizeSell,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ClaimRefund
    #[derive(TryFromInstruction)]
    pub struct ClaimRefundI11n<'info> {
        pub accounts: ClaimRefundAccountMetas<'info>,
        pub args: ClaimRefund,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct AdminInitAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub new_admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct AdminDeleteAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub old_admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreateTakeoverAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub metadata: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub new_mint_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub sysvar_instruction_program: &'info AccountMeta,
        pub metaplex_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct UpdateTakeoverAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub takeover_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CancelTakeoverAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub takeover_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct BuyPresaleAccountMetas<'info> {
        pub user: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub takeover_vault: &'info AccountMeta,
        pub presale_receipt: &'info AccountMeta,
        pub user_token: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SwapOldTokenAccountMetas<'info> {
        pub user: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub old_mint_token: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub new_mint_token: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub takeover_vault: &'info AccountMeta,
        pub old_mint_takeover_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct FinalizeTakeoverAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SellTokenAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub wsol: &'info AccountMeta,
        pub old_mint_admin_token: &'info AccountMeta,
        pub wsol_admin_token: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub old_mint_takeover_vault: &'info AccountMeta,
        pub takeover_sol_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub instructions: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct FinalizeSellAccountMetas<'info> {
        pub admin: &'info AccountMeta,
        pub admin_profile: &'info AccountMeta,
        pub wsol: &'info AccountMeta,
        pub wsol_admin_token: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub takeover_sol_vault: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
        pub instructions: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClaimRefundAccountMetas<'info> {
        pub user: &'info AccountMeta,
        pub new_mint: &'info AccountMeta,
        pub old_mint: &'info AccountMeta,
        pub takeover: &'info AccountMeta,
        pub presale_vault: &'info AccountMeta,
        pub old_mint_vault: &'info AccountMeta,
        pub presale_receipt: &'info AccountMeta,
        pub user_new_mint_token: &'info AccountMeta,
        pub user_old_mint_token: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub token_program: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct AdminInit {
        pub username: String,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct AdminDelete {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreateTakeover {
        pub create_takeover_args: CreateTakeoverArgs,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateTakeover {
        pub update_takeover_args: UpdateTakeoverArgs,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelTakeover {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct BuyPresale {
        pub amount: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SwapOldToken {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct FinalizeTakeover {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SellToken {
        pub amount: u64,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct FinalizeSell {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ClaimRefund {

    }        
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

   #[account]
    pub struct AdminProfile {
        pub address: Pubkey,
        pub username: String,
        pub creation_time: i64,
        pub bump: u8
    }

   #[account]
    pub struct Takeover {
        pub old_mint: Pubkey,
        pub new_mint: Pubkey,
        pub swap_period: SwapPeriod,
        pub takeover_wallet: Pubkey,
        pub inflation_amount: InflationAmount,
        pub presale_price: u64,
        pub presale_claimed: u64,
        pub bump: u8
    }

   #[account]
    pub struct PresaleReceipt {
        pub takeover: Pubkey,
        pub presale_amount: u64,
        pub bump: u8
    }

   #[account]
    pub struct FailedTakeover {
        pub old_mint: Pubkey,
        pub new_mint: Pubkey,
        pub presale_price: u64,
        pub bump: u8
    }

   #[account]
    pub struct SuccessfulTakeover {
        pub old_mint: Pubkey,
        pub new_mint: Pubkey,
        pub phase: Phase,
        pub bump: u8
    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateTakeoverArgs {
    pub start: i64,
    pub end: i64,
    pub takeover_wallet: Pubkey,
    pub presale_price: u64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SwapPeriod {
    pub start: i64,
    pub end: i64,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InflationAmount {
    pub level: Level,
    pub rewards_basis_point: u16,
    pub treasury_basis_point: u16,
    pub presale_basis_point: u16,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Level {
    Low,
    Medium,
    High
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Phase {
    TokenSelling,
    MarketCreation,
    Cleanup,
    UnlockingAta
}