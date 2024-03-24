use std::str::FromStr;

use crate::trade_shield::types::{SpotOrder, SpotOrderType};
use cosmwasm_std::{StdError, StdResult};

impl SpotOrder {
    pub fn gen_key(&self) -> StdResult<String> {
        if self.order_type == SpotOrderType::MarketBuy {
            return Err(StdError::generic_err("gen a key on a market order"));
        }

        Ok(self.order_type.to_string()
            + "\n"
            + &self.order_price.base_denom
            + "\n"
            + &self.order_price.quote_denom)
    }
    pub fn from_key(key: &str) -> StdResult<(SpotOrderType, String, String)> {
        let vec: Vec<&str> = key.split('\n').collect();
        if vec.len() != 3 {
            return Err(StdError::generic_err("Wrong Key"));
        }

        let order_type = SpotOrderType::from_str(vec[0])?;
        if order_type == SpotOrderType::MarketBuy {
            return Err(StdError::generic_err("Market Order"));
        }

        Ok((order_type, vec[1].to_string(), vec[2].to_string()))
    }
}
