use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("RewardError")]
    RewardError {},
    #[error("AssetDenomError")]
    AssetDenomError {},
    #[error("{balance} is smaller than {amount}")]
    InsufficientBalanceError { balance: u128, amount: u64 },
}
