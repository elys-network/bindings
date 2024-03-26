use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;

#[cw_serde]
pub enum PerpetualOrderType {
    LimitOpen,
    LimitClose,

    MarketOpen,
    MarketClose,

    StopLoss,
}

impl ToString for PerpetualOrderType {
    fn to_string(&self) -> String {
        match self {
            PerpetualOrderType::LimitOpen => "LimitOpen".to_string(),
            PerpetualOrderType::LimitClose => "LimitClose".to_string(),

            PerpetualOrderType::MarketOpen => "MarketOpen".to_string(),
            PerpetualOrderType::MarketClose => "MarketClose".to_string(),

            PerpetualOrderType::StopLoss => "StopLoss".to_string(),
        }
    }
}

impl FromStr for PerpetualOrderType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "LimitOpen" => Self::LimitOpen,
            "LimitClose" => Self::LimitClose,
            "MarketOpen" => Self::MarketOpen,
            "MarketClose" => Self::MarketClose,
            "StopLoss" => Self::StopLoss,
            _ => return Err(StdError::generic_err("unknow type")),
        })
    }
}
