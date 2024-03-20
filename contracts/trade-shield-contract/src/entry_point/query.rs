use super::*;
use elys_bindings::trade_shield::{
    msg::query_resp::TradeShieldParamsResponse,
    states::{
        LEVERAGE_ENABLE, MARKET_ORDER_ENABLED, PARAMS_ADMIN, PERPETUAL_ENABLED,
        PROCESS_ORDERS_ENABLED, REWARD_ENABLE, STAKE_ENABLED, SWAP_ENABLED,
    },
};
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetSpotOrder { order_id } => Ok(to_json_binary(&query::get_spot_order(deps, order_id)?)?),
        GetAllPrices { limit } => Ok(to_json_binary(&query::get_all_prices(deps, limit)?)?),
        AssetInfo { denom } => Ok(to_json_binary(&query::asset_info(deps, denom)?)?),
        GetPerpetualPosition { address, id } => Ok(to_json_binary(
            &query::get_perpetual_position(deps, address, id)?,
        )?),
        GetPerpetualPositions { pagination } => Ok(to_json_binary(
            &query::get_perpetual_positions(deps, pagination)?,
        )?),
        GetSpotOrders {
            pagination,
            order_owner,
            order_type,
            order_status,
        } => Ok(to_json_binary(&query::get_spot_orders(
            deps,
            pagination,
            order_owner,
            order_type,
            order_status,
        )?)?),
        GetPerpetualOrders {
            pagination,
            order_owner,
            order_type,
            order_status,
        } => Ok(to_json_binary(&query::get_perpetual_orders(
            deps,
            pagination,
            order_owner,
            order_type,
            order_status,
        )?)?),
        SwapEstimationByDenom {
            amount,
            denom_in,
            denom_out,
            user_address,
        } => Ok(to_json_binary(&query::swap_estimation_by_denom(
            deps,
            amount,
            denom_in,
            denom_out,
            user_address,
        )?)?),
        GetPerpetualOrder { id } => Ok(to_json_binary(&query::get_perpetual_order(deps, id)?)?),
        PerpetualOpenEstimation {
            position,
            leverage,
            trading_asset,
            collateral,
            take_profit_price,
            user_address,
        } => Ok(to_json_binary(&query::perpetual_open_estimation(
            deps,
            position,
            leverage,
            trading_asset,
            collateral,
            take_profit_price,
            user_address,
        )?)?),
        PerpetualGetPositionsForAddress {
            address,
            pagination,
        } => Ok(to_json_binary(&query::perpetual_get_position_for_address(
            deps, address, pagination,
        )?)?),
        GetParams {} => Ok(to_json_binary(&{
            let params_admin = PARAMS_ADMIN.load(deps.storage)?;

            let market_order_enabled = MARKET_ORDER_ENABLED.load(deps.storage)?;
            let stake_enabled = STAKE_ENABLED.load(deps.storage)?;
            let process_order_enabled = PROCESS_ORDERS_ENABLED.load(deps.storage)?;
            let swap_enabled = SWAP_ENABLED.load(deps.storage)?;
            let perpetual_enabled = PERPETUAL_ENABLED.load(deps.storage)?;
            let reward_enabled = REWARD_ENABLE.load(deps.storage)?;
            let leverage_enabled = LEVERAGE_ENABLE.load(deps.storage)?;

            TradeShieldParamsResponse {
                params_admin,
                market_order_enabled,
                stake_enabled,
                process_order_enabled,
                swap_enabled,
                perpetual_enabled,
                reward_enabled,
                leverage_enabled,
            }
        })?),
    }
}
