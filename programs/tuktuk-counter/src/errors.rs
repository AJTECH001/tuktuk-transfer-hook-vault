use anchor_lang::prelude::*;

#[error_code]
pub enum CounterError {
    #[msg("Not whitelisted")]
    NotWhitelisted,
    #[msg("Invalid transfer hook program")]
    InvalidTransferHookProgram,
    #[msg("Vault reward distribution too early")]
    RewardTooEarly,
}
