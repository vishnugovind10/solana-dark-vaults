use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("arithmetic operation overflowed or underflowed")]
    ArithmeticOverflow,
    #[msg("amount must be greater than zero")]
    ZeroAmount,
    #[msg("deposit is too small to mint one share")]
    DepositTooSmall,
    #[msg("withdrawal is too small to return one asset unit")]
    WithdrawalTooSmall,
    #[msg("requested shares exceed the outstanding supply")]
    InsufficientShares,
    #[msg("vault is paused")]
    VaultPaused,
    #[msg("vault is already in the requested pause state")]
    StatusUnchanged,
    #[msg("a computation is already pending")]
    ComputationAlreadyPending,
    #[msg("input commitment must not be all zeroes")]
    InvalidCommitment,
    #[msg("no computation is pending")]
    NoPendingComputation,
    #[msg("callback epoch does not match the pending computation")]
    ComputationEpochMismatch,
    #[msg("allocation weights must sum to exactly 10,000 bps")]
    InvalidWeightSum,
    #[msg("an allocation weight exceeds 10,000 bps")]
    InvalidWeight,
    #[msg("signer is not the configured authority")]
    Unauthorized,
    #[msg("signer is not the configured computation callback authority")]
    UnauthorizedCallback,
}
