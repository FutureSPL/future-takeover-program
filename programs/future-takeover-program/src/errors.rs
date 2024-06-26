use anchor_lang::prelude::*;

#[error_code]
pub enum TakeoverError {
    // Setup Errors
    #[msg("You are not Authorized to perform this action.")]
    Unauthorized,

    #[msg("You need to wait 16h after the initialization of the admin to perform this action.")]
    UnauthorizedAdmin,
    #[msg("End time must be greater than start time && start time must be greater than current time.")]
    InvalidSwapPeriod,
    #[msg("You have to choosen a FDMC value that is not available")]
    InvalidFdmcValue,
    #[msg("The takeover has already started.")]
    TakeoverAlreadyStarted,
    #[msg("You used Invalid Inflation Amounts.")]
    InvalidInflationAmounts,

    // Takeover Errors
    #[msg("You need to wait for the swap period to start")]
    SwapPeriodNotStarted,
    #[msg("The swap period has ended.")]
    SwapPeriodEnded,
    #[msg("The amount of token you're trying to buy needs to be greater than 0.")]
    InvalidAmount,
    #[msg("You're trying to buy more token than available in the presale vault.")]
    NotEnoughTokens,

    // Finalize Errors
    #[msg("You need to wait for the swap period to end.")]
    SwapPeriodNotEnded,
    #[msg("This is the wrong phase to perform this action.")]
    InvalidPhase,
    #[msg("There are no Token to Refund in your wallet.")]
    TokenEmpty,
    #[msg("This Presale Receipt is not for this Takeover.")]
    InvalidTakeoverData,
    #[msg("You already claimed your Presale Refund.")]
    PresaleAmountZero,
    #[msg("There are not enough funds in the Vault to fulfill this request.")]
    InsufficientFunds,

    #[msg("The Swap Instruction is missing.")]
    MissingSwapIx,
    #[msg("The Swap Instruction has invalid Slippage.")]
    InvalidSwapSlippage,
    #[msg("The Swap Instruction has invalid Amount.")]
    InvalidSwapAmount,
    #[msg("The Swap Instruction has invalid Source Mint.")]
    InvalidSwapSourceMint,
    #[msg("The Swap Instruction has invalid Source Token Account.")]
    InvalidSwapSourceTokenAccount,
    #[msg("The Swap Instruction has invalid Destination Mint.")]
    InvalidSwapDestinationMint,
    #[msg("The Swap Instruction has invalid Destination Token Account.")]
    InvalidSwapDestinationTokenAccount,

    #[msg("The Finalize Sell Instruction is missing.")]
    MissingFinalizeSellIx,
    #[msg("The Finalize Sell Instruction has invalid Admin.")]
    InvalidFinalizeSellAdmin,
    #[msg("The Finalize Sell Instruction has invalid Takeover.")]
    InvalidFinalizeSellTakeover,

    #[msg("The Initialize Instruction is missing.")]
    MissingInitializeTx,
    #[msg("The Initialize Instruction has invalid Amount.")]
    InvalidInitializeAmount,
    #[msg("The Initialize Instruction has invalid Mint Account.")]
    InvalidInitializeMintAccount,

    #[msg("The Increase Liquidity Instruction is missing.")]
    MissingIncreaseLiquidityIx,
    #[msg("The Token A used to create the market is not the same as what you received.")]
    WrongAmountTokenA,
    #[msg("The Token B used to create the market is not the same as what you received.")]
    WrongAmountTokenB,
    #[msg("The Token A ATA used to create the market is not the same as what is expected.")]
    WrongAtaTokenA,
    #[msg("The Token B ATA used to create the market is not the same as what is expected.")]
    WrongAtaTokenB,

    #[msg("The Takeover Wallet used is not the correct one.")]
    InvalidTakeoverWallet,
    #[msg("The Reward Wallet used is not the correct one.")]
    InvalidRewardWallet,
    #[msg("The Referral Wallet used is not the correct one.")]
    InvalidReferralWallet,

    // General Errors
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
}