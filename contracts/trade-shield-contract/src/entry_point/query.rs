use super::*;
use cosmwasm_std::{coin, Addr, Decimal, Order, StdError, Timestamp};
use elys_bindings::trade_shield::{
    msg::query_resp::{
        GetSortedOrderListResp, NumberOfPendingOrderResp, OrdersStates, TradeShieldParamsResponse,
    },
    states::{
        LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED, PARAMS_ADMIN,
        PENDING_PERPETUAL_ORDER, PENDING_SPOT_ORDER, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED,
        REWARD_ENABLED, SORTED_PENDING_SPOT_ORDER, SPOT_ORDER, STAKE_ENABLED, SWAP_ENABLED,
    },
    types::{Date, OrderPrice, PerpetualOrder, SpotOrder, Status},
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
        AmmGetPool { pool_id } => {
            let querier = ElysQuerier::new(&deps.querier);
            Ok(to_json_binary(&querier.amm_get_pool(pool_id)?)?)
        }
        AmmGetPools { pagination } => {
            let querier = ElysQuerier::new(&deps.querier);
            Ok(to_json_binary(&querier.amm_get_pools(pagination)?)?)
        }
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
        NumberOfPendingOrder {} => {
            let spot_orders: Vec<SpotOrder> = PENDING_SPOT_ORDER
                .prefix_range(deps.storage, None, None, Order::Ascending)
                .filter_map(|res| res.ok().map(|r| r.1))
                .collect();
            let perpetual_orders: Vec<PerpetualOrder> = PENDING_PERPETUAL_ORDER
                .prefix_range(deps.storage, None, None, Order::Ascending)
                .filter_map(|res| res.ok().map(|r| r.1))
                .collect();

            Ok(to_json_binary(&NumberOfPendingOrderResp {
                spot_orders: spot_orders.len() as u128,
                perpetual_orders: perpetual_orders.len() as u128,
            })?)
        }
        GetParams {} => Ok(to_json_binary(&{
            let params_admin = PARAMS_ADMIN.load(deps.storage)?;

            let market_order_enabled = MARKET_ORDER_ENABLED.load(deps.storage)?;
            let stake_enabled = STAKE_ENABLED.load(deps.storage)?;
            let process_order_enabled = PROCESS_ORDERS_ENABLED.load(deps.storage)?;
            let swap_enabled = SWAP_ENABLED.load(deps.storage)?;
            let perpetual_enabled = PERPETUAL_ENABLED.load(deps.storage)?;
            let reward_enabled = REWARD_ENABLED.load(deps.storage)?;
            let leverage_enabled = LEVERAGE_ENABLED.load(deps.storage)?;
            let limit_process_order = LIMIT_PROCESS_ORDER.load(deps.storage)?;

            TradeShieldParamsResponse {
                params_admin,
                market_order_enabled,
                stake_enabled,
                process_order_enabled,
                swap_enabled,
                perpetual_enabled,
                reward_enabled,
                leverage_enabled,
                limit_process_order,
            }
        })?),
        GetSortedOrderList {
            order_type,
            base_denom,
            quote_denom,
        } => {
            let dummy_order = SpotOrder {
                order_type,
                order_id: 0,
                order_price: OrderPrice {
                    base_denom,
                    quote_denom,
                    rate: Decimal::zero(),
                },
                order_amount: coin(0, ""),
                owner_address: Addr::unchecked(""),
                order_target_denom: "".to_string(),
                status: trade_shield::types::Status::Canceled,
                date: Date {
                    height: 0,
                    time: Timestamp::from_nanos(0),
                },
            };
            let k = dummy_order.gen_key()?;
            let v = match SORTED_PENDING_SPOT_ORDER.may_load(deps.storage, k.as_str())? {
                Some(v) => v,
                None => return Err(StdError::generic_err("no order found").into()),
            };
            let mut orders_states: Vec<OrdersStates> = vec![];
            for i in 0..v.len() {
                let id = v[i];
                let (status, found) = match SPOT_ORDER.may_load(deps.storage, id)? {
                    Some(order) => (order.status, true),
                    None => (Status::Canceled, false),
                };
                let is_in_pending = PENDING_SPOT_ORDER.may_load(deps.storage, id)?.is_some();

                orders_states.push(OrdersStates {
                    id,
                    status,
                    is_in_pending,
                    found,
                });
                if i >= 50 {
                    break;
                }
            }
            Ok(to_json_binary(&GetSortedOrderListResp { orders_states })?)
        }
    }
}
