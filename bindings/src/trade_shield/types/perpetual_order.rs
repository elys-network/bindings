use std::str::FromStr;

use crate::{
    trade_shield::states::{PENDING_PERPETUAL_ORDER, PERPETUAL_ORDER_MAX_ID},
    types::PerpetualPosition,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Coin, Deps, DepsMut, OverflowError, OverflowOperation, SignedDecimal, SignedDecimal256,
    StdError, StdResult, Storage,
};

use super::{OrderPrice, PerpetualOrderType, Status};

#[cw_serde]
pub struct PerpetualOrder {
    pub order_id: u64,
    pub owner: String,
    pub order_type: PerpetualOrderType,
    pub position: PerpetualPosition,
    pub trigger_price: Option<OrderPrice>,
    pub collateral: Coin,
    pub trading_asset: String,
    pub leverage: SignedDecimal,
    pub take_profit_price: Option<SignedDecimal256>,
    pub position_id: Option<u64>,
    pub status: Status,
}

impl PerpetualOrder {
    pub fn new_open(
        owner: impl Into<String>,
        position: &PerpetualPosition,
        order_type: &PerpetualOrderType,
        collateral: &Coin,
        trading_asset: impl Into<String>,
        leverage: &SignedDecimal,
        take_profit_price: &Option<SignedDecimal256>,
        trigger_price: &Option<OrderPrice>,
        storage: &mut dyn Storage,
    ) -> StdResult<Self> {
        let status = if order_type == &PerpetualOrderType::MarketOpen {
            Status::Executed
        } else {
            Status::Pending
        };

        let order_id = get_new_id(storage)?;

        let order = Self {
            order_id,
            owner: owner.into(),
            position: position.to_owned(),
            collateral: collateral.to_owned(),
            trading_asset: trading_asset.into(),
            leverage: leverage.to_owned(),
            take_profit_price: take_profit_price.to_owned(),
            order_type: order_type.to_owned(),
            trigger_price: trigger_price.to_owned(),
            status,
            position_id: None,
        };

        return Ok(order);
    }
    pub fn new_close(
        owner: impl Into<String>,
        position: i32,
        order_type: &PerpetualOrderType,
        collateral: &Coin,
        trading_asset: impl Into<String>,
        leverage: &SignedDecimal,
        position_id: u64,
        trigger_price: &Option<OrderPrice>,
        take_profit_price: &Option<SignedDecimal256>,
        storage: &mut dyn Storage,
    ) -> StdResult<Self> {
        let order_id: u64 = get_new_id(storage)?;

        let status = if order_type == &PerpetualOrderType::MarketClose {
            Status::Executed
        } else {
            Status::Pending
        };

        let position = PerpetualPosition::try_from_i32(position)?;

        let order: PerpetualOrder = Self {
            order_id,
            status,
            order_type: order_type.to_owned(),
            position,
            owner: owner.into(),
            trigger_price: trigger_price.to_owned(),
            collateral: collateral.to_owned(),
            trading_asset: trading_asset.into(),
            position_id: Some(position_id),
            leverage: leverage.to_owned(),
            take_profit_price: take_profit_price.to_owned(),
        };

        Ok(order)
    }

    pub fn binary_search(
        trigger_price: &Option<OrderPrice>,
        storage: &dyn Storage,
        list: &Vec<u64>,
    ) -> StdResult<usize> {
        let mut low = 0;
        let mut high = list.len();
        let rate = match trigger_price {
            Some(price) => &price.rate,
            None => {
                return Err(StdError::generic_err(
                    "perpetual: binary search: price not found",
                ))
            }
        };

        while low < high {
            let mid = low + (high - low) / 2;
            let PerpetualOrder { trigger_price, .. } =
                match PENDING_PERPETUAL_ORDER.may_load(storage, list[mid])? {
                    Some(order) => order,
                    None => {
                        return Err(StdError::generic_err(
                            "perpetual: binary search: order not found",
                        ))
                    }
                };
            if trigger_price.is_none() {
                return Err(StdError::generic_err(
                    "perpetual: binary search: price not found",
                ));
            }

            if trigger_price.unwrap().rate < *rate {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        Ok(low)
    }

    pub fn gen_key(&self) -> StdResult<String> {
        if self.order_type == PerpetualOrderType::MarketClose
            || self.order_type == PerpetualOrderType::MarketOpen
        {
            return Err(StdError::generic_err("gen a key on a market order"));
        }
        if let Some(price) = &self.trigger_price {
            Ok(self.position.to_string()
                + "\n"
                + &self.order_type.to_string()
                + "\n"
                + &price.base_denom
                + "\n"
                + &price.quote_denom)
        } else {
            Err(StdError::not_found("trigger price not found"))
        }
    }

    pub fn from_key(
        key: &str,
    ) -> StdResult<(PerpetualPosition, PerpetualOrderType, String, String)> {
        let vec: Vec<&str> = key.split('\n').collect();
        if vec.len() != 4 {
            return Err(StdError::generic_err("Wrong Key"));
        }

        let order_position = PerpetualPosition::from_str(vec[0])?;
        let order_type = PerpetualOrderType::from_str(vec[1])?;
        if order_type == PerpetualOrderType::MarketClose
            || order_type == PerpetualOrderType::MarketOpen
        {
            return Err(StdError::generic_err("Market Order"));
        }

        Ok((
            order_position,
            order_type,
            vec[2].to_string(),
            vec[3].to_string(),
        ))
    }
}

fn get_new_id(storage: &mut dyn Storage) -> StdResult<u64> {
    let max_id = match PERPETUAL_ORDER_MAX_ID.may_load(storage)? {
        Some(id) => id + 1,
        None => 0,
    };
    let id = max_id
        .checked_add(1)
        .ok_or(StdError::overflow(OverflowError::new(
            OverflowOperation::Add,
            "perpetual order id",
            "1",
        )))?;
    PERPETUAL_ORDER_MAX_ID.save(storage, &id)?;

    Ok(id)
}
