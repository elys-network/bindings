
use cosmwasm_std::{Decimal, Uint128, Coin};
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum FilterType {
    FilterAll = 0,
	FilterPerpetual = 1,
	FilterFixedWeight = 2,
	FilterDynamicWeight = 3,
	FilterLeverage = 4,
}

#[cw_serde]
pub struct PoolAsset {
    pub token: Coin, 
    pub weight: Uint128,
}

#[cw_serde]
pub struct PoolResp {
    pub assets: Vec<PoolAsset>, // eg : [{{"denom":"uatom", "amount":"1000"}, "weight":"10"}, {{"denom":"uusdc", "amount":"100"}, "weight":"1"}, ...]
    pub pool_ratio: String,
    pub rewards_apr: Decimal,
    pub borrow_apr: Decimal,
    pub leverage_lp: Decimal,
    pub perpetual: Decimal,
    pub tvl: Decimal,
    pub rewards: Decimal,
}

#[cw_serde]
pub struct QueryEarnPoolResponse {
    pub pools: Option<Vec<PoolResp>>,
}
