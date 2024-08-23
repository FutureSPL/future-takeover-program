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
    #[msg("You used Invalid Inflation Amounts.")]
    InvalidInflationAmounts,
    #[msg("You used Invalid Token Extension Args.")]
    InvalidTokenExtensionArgs,

    #[msg("You used an Invalid Takeover Account.")]
    InvalidTakeoverAddress,
    #[msg("You used an Invalid Mint")]
    InvalidMint,
    #[msg("You used an Invalid Associated Token Account.")]
    InvalidAssociatedToken,
    #[msg("You used an Invalid Token Program.")]
    InvalidTokenProgram,
    #[msg("You used an Invalid Remaining Account Schema.")]
    InvalidRemainingAccountSchema,

    #[msg("The Total doesn't add up to 100")]
    InvalidTotalPercentage,

    #[msg("The takeover has already started.")]
    TakeoverAlreadyStarted,

    #[msg("The "decimal" Input is not there.")]
    DecimalsNotFound,
    #[msg("The "old_tokens" Input is missing.")]
    OldTokensNotFound,
    #[msg("The "old_mint" Input is missing.")]
    OldMintNotFound,
    #[msg("The "weight_percentage" Input is missing.")]
    WeightedPercentageNotFound,

    #[msg("You passed too many mints (Max is 3)")]
    TooManyMint,
    #[msg("This Extension is not currently available.")]
    ExtensionNotAvaialble,
    #[msg("There are still non-zero amounts in the swap_receipt.")]
    NonZeroAmountsRemain,

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
    SwapIxNotFound,
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
    FinalizeSellIxNotFound,
    #[msg("The Finalize Sell Instruction has invalid Admin.")]
    InvalidFinalizeSellAdmin,
    #[msg("The Finalize Sell Instruction has invalid Takeover.")]
    InvalidFinalizeSellTakeover,

    #[msg("The Initialize Instruction is missing.")]
    InitializeTxNotFound,
    #[msg("The Initialize Instruction has invalid Amount.")]
    InvalidInitializeAmount,
    #[msg("The Initialize Instruction has invalid Mint Account.")]
    InvalidInitializeMintAccount,

    #[msg("The Increase Liquidity Instruction is missing.")]
    IncreaseLiquidityIxNotFound,
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