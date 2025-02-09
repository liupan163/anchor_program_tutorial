use anchor_lang::prelude::*;

pub type XxxResult<T = ()> = std::result::Result<T, ErrorCode>;

#[error_code]
#[derive(PartialEq, Eq)]
pub enum ErrorCode {
    #[msg("Only owner can call this function!")]
    NotOwner,

    #[msg("Too many hops")]
    TooManyHops,

    #[msg("Min return not reached")]
    MinReturnNotReached,

    #[msg("amount_in must be greater than 0")]
    AmountInMustBeGreaterThanZero,

    #[msg("min_return must be greater than 0")]
    MinReturnMustBeGreaterThanZero,

    #[msg("invalid expect amount out")]
    InvalidExpectAmountOut,

    #[msg("amounts and routes must have the same length")]
    AmountsAndRoutesMustHaveTheSameLength,

    #[msg("total_amounts must be equal to amount_in")]
    TotalAmountsMustBeEqualToAmountIn,

    #[msg("dexes and weights must have the same length")]
    DexesAndWeightsMustHaveTheSameLength,

    #[msg("weights must sum to 100")]
    WeightsMustSumTo100,

    #[msg("Invalid source token account")]
    InvalidSourceTokenAccount,

    #[msg("Invalid destination token account")]
    InvalidDestinationTokenAccount,

    #[msg("Invalid commission rate")]
    InvalidCommissionRate,

    #[msg("Invalid commission token account")]
    InvalidCommissionTokenAccount,

    #[msg("Invalid accounts length")]
    InvalidAccountsLength,

    #[msg("Invalid hop accounts")]
    InvalidHopAccounts,

    #[msg("Invalid hop from account")]
    InvalidHopFromAccount,

    #[msg("Swap authority is not signer")]
    SwapAuthorityIsNotSigner,

    #[msg("Invalid authority pda")]
    InvalidAuthorityPda,

    #[msg("Invalid program id")]
    InvalidProgramId,

    #[msg("Invalid pool")]
    InvalidPool,

    #[msg("Invalid token mint")]
    InvalidTokenMint,

    #[msg("Calculation error")]
    CalculationError,

    #[msg("Authority pda creation failed")]
    AuthorityPDACreationFailed,

    #[msg("Transfer sol failed")]
    TransferSolFailed,

    #[msg("Transfer token failed")]
    TransferTokenFailed,

    #[msg("Invalid sanctum lst state list data")]
    InvalidSanctumLstStateListData,

    #[msg("Invalid sanctum lst state list index")]
    InvalidSanctumLstStateListIndex,

    #[msg("Invalid sanctum swap accounts")]
    InvalidSanctumSwapAccounts,

    #[msg("Bn Conversion Error")]
    BnConversionError,

    #[msg("Casting Failure Error")]
    CastingFailure,

    #[msg("Math Error")]
    MathError,
}
