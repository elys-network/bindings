use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Empty, StdError};
use cw_storage_plus::Map;
use cw_utils::Expiration;

use crate::types::CoinValue;

use super::*;

const OLD_HISTORY: Map<&str, Vec<OldAccountSnapshot>> = Map::new("history");

#[cw_serde]
struct OldAccountSnapshot {
    pub date: Expiration,
    pub total_liquid_asset_balance: DecCoin,
    pub total_available_balance: DecCoin,
    pub total_in_orders_balance: DecCoin,
    pub available_asset_balance: Vec<CoinValue>,
    pub in_orders_asset_balance: Vec<CoinValue>,
    pub total_value_per_asset: Vec<CoinValue>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut<ElysQuery>, _env: Env, _msg: Empty) -> StdResult<Response<ElysMsg>> {
    let keys: Vec<String> = OLD_HISTORY
        .keys(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|key_result| key_result.map(|key| key))
        .collect::<Result<Vec<String>, StdError>>()?;

    for key in keys {
        OLD_HISTORY.remove(deps.storage, key.as_str());
    }
    Ok(Response::new())
}
