use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdError;

#[cw_serde]
pub enum SpotOrderType {
    StopLoss,
    LimitSell,
    LimitBuy,
    MarketBuy,
}

impl ToString for SpotOrderType {
    fn to_string(&self) -> String {
        match self {
            SpotOrderType::StopLoss => "StopLoss".to_string(),
            SpotOrderType::LimitSell => "LimitSell".to_string(),
            SpotOrderType::LimitBuy => "LimitBuy".to_string(),
            SpotOrderType::MarketBuy => "MarketBuy".to_string(),
        }
    }
}

impl FromStr for SpotOrderType {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "StopLoss" => Self::StopLoss,
            "LimitSell" => Self::LimitSell,
            "LimitBuy" => Self::LimitBuy,
            "MarketBuy" => Self::MarketBuy,
            _ => return Err(StdError::generic_err("unknow type")),
        })
    }
}
