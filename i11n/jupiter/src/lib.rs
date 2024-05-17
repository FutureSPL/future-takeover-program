use anchor_lang::prelude::*;

declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

// Accounts
#[derive(Accounts)]
pub struct Route<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RouteWithTokenLedger<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SharedAccountsRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SharedAccountsRouteWithTokenLedger<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ExactOutRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_source_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub user_destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_token_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SharedAccountsExactOutRoute<'info> {
    /// CHECK: Skip check
    pub token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub user_transfer_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_source_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_destination_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub source_mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub destination_mint: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub platform_fee_account: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub token_2022_program: Option<AccountInfo<'info>>,
    /// CHECK: Skip check
    pub event_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetTokenLedger<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    /// CHECK: Skip check
    pub token_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateOpenOrders<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateProgramOpenOrders<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub open_orders: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub dex_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub rent: AccountInfo<'info>,
    /// CHECK: Skip check
    pub market: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    /// CHECK: Skip check
    pub wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub wallet: AccountInfo<'info>,
    /// CHECK: Skip check
    pub program_authority: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub program_token_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Skip check
    pub destination_token_account: AccountInfo<'info>,
    /// CHECK: Skip check
    pub mint: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub associated_token_program: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CreateTokenLedger<'info> {
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub token_ledger: AccountInfo<'info>,
    #[account(mut, signer)]
    /// CHECK: Skip check
    pub payer: AccountInfo<'info>,
    /// CHECK: Skip check
    pub system_program: AccountInfo<'info>,
}

// CPI
pub mod cpi {
    #![allow(unused)]
    use anchor_i11n::Discriminator;
    use super::*;

    pub fn route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Route<'info>>,
        route_plan: Vec<RoutePlanStep>,
        in_amount: u64,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Route { route_plan, in_amount, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Route::DISCRIMINATOR);
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

    pub fn route_with_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, RouteWithTokenLedger<'info>>,
        route_plan: Vec<RoutePlanStep>,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::RouteWithTokenLedger { route_plan, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::RouteWithTokenLedger::DISCRIMINATOR);
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

    pub fn shared_accounts_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsRoute<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        in_amount: u64,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsRoute { id, route_plan, in_amount, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsRoute::DISCRIMINATOR);
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

    pub fn shared_accounts_route_with_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsRouteWithTokenLedger<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        quoted_out_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsRouteWithTokenLedger { id, route_plan, quoted_out_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsRouteWithTokenLedger::DISCRIMINATOR);
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

    pub fn exact_out_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ExactOutRoute<'info>>,
        route_plan: Vec<RoutePlanStep>,
        out_amount: u64,
        quoted_in_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ExactOutRoute { route_plan, out_amount, quoted_in_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ExactOutRoute::DISCRIMINATOR);
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

    pub fn shared_accounts_exact_out_route<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SharedAccountsExactOutRoute<'info>>,
        id: u8,
        route_plan: Vec<RoutePlanStep>,
        out_amount: u64,
        quoted_in_amount: u64,
        slippage_bps: u16,
        platform_fee_bps: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SharedAccountsExactOutRoute { id, route_plan, out_amount, quoted_in_amount, slippage_bps, platform_fee_bps };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SharedAccountsExactOutRoute::DISCRIMINATOR);
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

    pub fn set_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, SetTokenLedger<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::SetTokenLedger {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::SetTokenLedger::DISCRIMINATOR);
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

    pub fn create_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateOpenOrders<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateOpenOrders {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateOpenOrders::DISCRIMINATOR);
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

    pub fn create_program_open_orders<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateProgramOpenOrders<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateProgramOpenOrders { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateProgramOpenOrders::DISCRIMINATOR);
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

    pub fn claim<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, Claim<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::Claim { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::Claim::DISCRIMINATOR);
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

    pub fn claim_token<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, ClaimToken<'info>>,
        id: u8
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::ClaimToken { id };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::ClaimToken::DISCRIMINATOR);
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

    pub fn create_token_ledger<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateTokenLedger<'info>>
    ) -> anchor_lang::Result<()> {
        let ix = {
            let ix = instructions::CreateTokenLedger {  };
            let mut data = Vec::with_capacity(256);
            data.extend_from_slice(&instructions::CreateTokenLedger::DISCRIMINATOR);
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

    // Route
    #[derive(TryFromInstruction)]
    pub struct RouteI11n<'info> {
        pub accounts: RouteAccountMetas<'info>,
        pub args: Route,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // RouteWithTokenLedger
    #[derive(TryFromInstruction)]
    pub struct RouteWithTokenLedgerI11n<'info> {
        pub accounts: RouteWithTokenLedgerAccountMetas<'info>,
        pub args: RouteWithTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsRoute
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsRouteI11n<'info> {
        pub accounts: SharedAccountsRouteAccountMetas<'info>,
        pub args: SharedAccountsRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsRouteWithTokenLedger
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsRouteWithTokenLedgerI11n<'info> {
        pub accounts: SharedAccountsRouteWithTokenLedgerAccountMetas<'info>,
        pub args: SharedAccountsRouteWithTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ExactOutRoute
    #[derive(TryFromInstruction)]
    pub struct ExactOutRouteI11n<'info> {
        pub accounts: ExactOutRouteAccountMetas<'info>,
        pub args: ExactOutRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SharedAccountsExactOutRoute
    #[derive(TryFromInstruction)]
    pub struct SharedAccountsExactOutRouteI11n<'info> {
        pub accounts: SharedAccountsExactOutRouteAccountMetas<'info>,
        pub args: SharedAccountsExactOutRoute,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // SetTokenLedger
    #[derive(TryFromInstruction)]
    pub struct SetTokenLedgerI11n<'info> {
        pub accounts: SetTokenLedgerAccountMetas<'info>,
        pub args: SetTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateOpenOrders
    #[derive(TryFromInstruction)]
    pub struct CreateOpenOrdersI11n<'info> {
        pub accounts: CreateOpenOrdersAccountMetas<'info>,
        pub args: CreateOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateProgramOpenOrders
    #[derive(TryFromInstruction)]
    pub struct CreateProgramOpenOrdersI11n<'info> {
        pub accounts: CreateProgramOpenOrdersAccountMetas<'info>,
        pub args: CreateProgramOpenOrders,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // Claim
    #[derive(TryFromInstruction)]
    pub struct ClaimI11n<'info> {
        pub accounts: ClaimAccountMetas<'info>,
        pub args: Claim,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // ClaimToken
    #[derive(TryFromInstruction)]
    pub struct ClaimTokenI11n<'info> {
        pub accounts: ClaimTokenAccountMetas<'info>,
        pub args: ClaimToken,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    // CreateTokenLedger
    #[derive(TryFromInstruction)]
    pub struct CreateTokenLedgerI11n<'info> {
        pub accounts: CreateTokenLedgerAccountMetas<'info>,
        pub args: CreateTokenLedger,
        pub remaining_accounts: Vec<&'info AccountMeta>,
    }

    //Accounts
    #[derive(TryFromAccountMetas)]
    pub struct RouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct RouteWithTokenLedgerAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsRouteWithTokenLedgerAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
        pub token_ledger: &'info AccountMeta,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ExactOutRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub user_source_token_account: &'info AccountMeta,
        pub user_destination_token_account: &'info AccountMeta,
        pub destination_token_account: Option<&'info AccountMeta>,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SharedAccountsExactOutRouteAccountMetas<'info> {
        pub token_program: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub user_transfer_authority: &'info AccountMeta,
        pub source_token_account: &'info AccountMeta,
        pub program_source_token_account: &'info AccountMeta,
        pub program_destination_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub source_mint: &'info AccountMeta,
        pub destination_mint: &'info AccountMeta,
        pub platform_fee_account: Option<&'info AccountMeta>,
        pub token_2022_program: Option<&'info AccountMeta>,
        pub event_authority: &'info AccountMeta,
        pub program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct SetTokenLedgerAccountMetas<'info> {
        pub token_ledger: &'info AccountMeta,
        pub token_account: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreateOpenOrdersAccountMetas<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreateProgramOpenOrdersAccountMetas<'info> {
        pub open_orders: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub dex_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
        pub rent: &'info AccountMeta,
        pub market: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClaimAccountMetas<'info> {
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct ClaimTokenAccountMetas<'info> {
        pub payer: &'info AccountMeta,
        pub wallet: &'info AccountMeta,
        pub program_authority: &'info AccountMeta,
        pub program_token_account: &'info AccountMeta,
        pub destination_token_account: &'info AccountMeta,
        pub mint: &'info AccountMeta,
        pub associated_token_token_program: &'info AccountMeta,
        pub associated_token_program: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }

    #[derive(TryFromAccountMetas)]
    pub struct CreateTokenLedgerAccountMetas<'info> {
        pub token_ledger: &'info AccountMeta,
        pub payer: &'info AccountMeta,
        pub system_program: &'info AccountMeta,
    }
}

// Instructions
pub mod instructions {
    use anchor_lang::prelude::*;
    use anchor_i11n::prelude::*;
    use super::*;

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Route {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct RouteWithTokenLedger {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SharedAccountsRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SharedAccountsRouteWithTokenLedger {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ExactOutRoute {
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SharedAccountsExactOutRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub out_amount: u64,
        pub quoted_in_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct SetTokenLedger {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreateOpenOrders {

    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreateProgramOpenOrders {
        pub id: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct Claim {
        pub id: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct ClaimToken {
        pub id: u8,
    }

    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct CreateTokenLedger {

    }        
}

// Accounts
pub mod accounts {
    #![allow(unused)]
    use super::*;

   #[account]
    pub struct TokenLedger {
        pub token_account: Pubkey,
        pub amount: u64
    }  
}
        
// Defined types
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Copy, PartialEq, Eq)]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema,
    Lifinity,
    Mercurial,
    Cykura,
    Serum,
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin,
    AldrinV2,
    Whirlpool,
    Invariant,
    Meteora,
    GooseFx,
    DeltaFi,
    Balansol,
    MarcoPolo,
    Dradex,
    LifinityV2,
    RaydiumClmm,
    Openbook,
    Phoenix,
    Symmetry,
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake,
    GooseFxv2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
    OpenBookV2,
    RaydiumClmmV2,
    StakeDexPrefundWithdrawStakeAndDepositStake,
    Clone
}