use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    StdError(#[from] StdError),
    #[error("PortfolioError")]
    PortfolioError {},
    #[error("RewardError")]
    RewardError {},
    #[error("TotalBalanceError")]
    TotalBalanceError {},
    #[error("AssetDenomError")]
    AssetDenomError {},
    #[error("{balance} is smaller than {amount}")]
    InsufficientBalanceError {balance: u128, amount: u64},
}
