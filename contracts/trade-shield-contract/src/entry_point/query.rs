use super::*;
use msg::QueryMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ElysQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    use action::query;
    use QueryMsg::*;

    match msg {
        GetSpotOrder { order_id } => Ok(to_json_binary(&query::get_spot_order(deps, order_id)?)?),
        GetAllPrices { limit } => Ok(to_json_binary(&query::get_all_prices(deps, limit)?)?),
        GetAssetPrice { denom } => Ok(to_json_binary(&query::get_asset_price(deps, denom)?)?),
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
    }
}
