use anchor_lang::prelude::*;


#[error_code]
pub enum ErrorCode {
    #[msg("Staking has not matured!")]
    UnstakeFreezeDurationInvalid,

    #[msg("Max staking amount is reached!")]
    MaxStakeExceeded
}