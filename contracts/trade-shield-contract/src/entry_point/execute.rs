use crate::helper::get_mut_discount;

use super::*;
use crate::action::sudo::process_orders;
use cosmwasm_std::{CosmosMsg, Int128, StdError, WasmMsg};
use elys_bindings::account_history::msg::ExecuteMsg as AccountHistoryMsg;
use elys_bindings::trade_shield::states::{
    ACCOUNT_HISTORY_ADDRESS, LEVERAGE_ENABLED, LIMIT_PROCESS_ORDER, MARKET_ORDER_ENABLED,
    PARAMS_ADMIN, PERPETUAL_ENABLED, PROCESS_ORDERS_ENABLED, REWARD_ENABLED, STAKE_ENABLED,
    SWAP_ENABLED,
};
use msg::ExecuteMsg;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ElysQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ElysMsg>, ContractError> {
    use action::execute::*;
    use ExecuteMsg::*;

    let account_history_address = ACCOUNT_HISTORY_ADDRESS.load(deps.storage)?;
    let user_address = info.sender.to_string();

    let resp = match msg {
        CreateSpotOrder {
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
        } => create_spot_order(
            env,
            deps,
            info,
            order_type,
            order_source_denom,
            order_target_denom,
            order_price,
        ),
        CancelSpotOrder { order_id } => cancel_spot_order(info, deps, order_id),
        CancelSpotOrders {
            order_ids,
            order_type,
        } => cancel_spot_orders(info, deps, order_ids, order_type),

        CreatePerpetualOrder {
            position,
            leverage,
            trading_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        } => create_perpetual_order(
            env,
            info,
            deps,
            position,
            leverage,
            trading_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        ),
        CancelPerpetualOrder { order_id } => cancel_perpetual_order(info, deps, order_id),
        CancelPerpetualOrders {
            order_ids,
            order_type,
        } => cancel_perpetual_orders(info, deps, order_ids, order_type),
        ClosePerpetualPosition { id, amount } => {
            close_perpetual_position(info, deps, env, id, amount)
        }

        StakeRequest {
            amount,
            asset,
            validator_address,
        } => stake_request(info, deps, amount, asset, validator_address),
        UnstakeRequest {
            amount,
            asset,
            validator_address,
        } => unstake_request(info, deps, amount, asset, validator_address),
        ElysRedelegateRequest {
            validator_src_address,
            validator_dst_address,
            amount,
        } => elys_redelegation_request(
            info,
            deps,
            validator_src_address,
            validator_dst_address,
            amount,
        ),
        ElysCancelUnstakeRequest {
            validator_address,
            amount,
            creation_height,
        } => elys_cancel_unstake_request(info, deps, validator_address, amount, creation_height),
        EdenVestRequest { amount } => eden_vest_request(info, deps, amount),
        EdenCancelVestRequest { amount } => eden_cancel_vest_request(info, deps, amount),
        EdenClaimVestingRequest {} => eden_claim_vesting_request(info),
        ClaimRewardsRequest {} => claim_rewards_request(info, deps),
        AmmJoinPoolRequest {
            pool_id,
            max_amounts_in,
            share_amount_out,
            no_remaining,
        } => join_amm_pool_request(
            info,
            deps,
            pool_id,
            max_amounts_in,
            share_amount_out,
            no_remaining,
        ),
        AmmExitPoolRequest {
            pool_id,
            min_amounts_out,
            share_amount_in,
            token_out_denom,
        } => exit_amm_pool_request(
            info,
            deps,
            pool_id,
            min_amounts_out,
            share_amount_in,
            token_out_denom,
        ),
        AmmSwapExactAmountIn { routes } => {
            cw_utils::one_coin(&info)?;

            let msg = ElysMsg::AmmSwapExactAmountIn {
                sender: env.contract.address.into_string(),
                routes,
                token_in: info.funds[0].clone(),
                token_out_min_amount: Int128::zero(),
                discount: get_mut_discount(deps.storage, deps.querier, info.sender.to_string())?,
                recipient: "".to_string(),
            };

            Ok(Response::new().add_message(msg))
        }

        LeveragelpOpen {
            amm_pool_id,
            collateral_asset,
            collateral_amount,
            leverage,
            stop_loss_price,
        } => open_leveragelp_position_request(
            info,
            deps,
            amm_pool_id,
            collateral_asset,
            collateral_amount,
            leverage,
            stop_loss_price,
        ),

        LeveragelpClose {
            position_id,
            amount,
        } => close_leveragelp_position_request(info, deps, position_id, amount),

        LeveragelpUpdateStopLoss { position, price } => {
            update_stop_loss_leveragelp_request(info, deps, position, price)
        }

        SetParams {
            market_order_enabled,
            stake_enabled,
            process_order_enabled,
            swap_enabled,
            perpetual_enabled,
            reward_enabled,
            leverage_enabled,
            limit_process_order,
        } => {
            let admin = PARAMS_ADMIN.load(deps.storage)?;

            if admin.as_str() != info.sender.as_str() {
                return Err(StdError::generic_err("Unauthorize: wrong sender").into());
            }
            if let Some(market_order_enabled) = market_order_enabled {
                MARKET_ORDER_ENABLED.save(deps.storage, &market_order_enabled)?;
            }
            if let Some(stake_enabled) = stake_enabled {
                STAKE_ENABLED.save(deps.storage, &stake_enabled)?;
            }
            if let Some(swap_enabled) = swap_enabled {
                SWAP_ENABLED.save(deps.storage, &swap_enabled)?;
            }
            if let Some(process_order_enabled) = process_order_enabled {
                PROCESS_ORDERS_ENABLED.save(deps.storage, &process_order_enabled)?;
            }
            if let Some(perpetual_enabled) = perpetual_enabled {
                PERPETUAL_ENABLED.save(deps.storage, &perpetual_enabled)?;
            }
            if let Some(reward_enabled) = reward_enabled {
                REWARD_ENABLED.save(deps.storage, &reward_enabled)?;
            }
            if let Some(leverage_enabled) = leverage_enabled {
                LEVERAGE_ENABLED.save(deps.storage, &leverage_enabled)?;
            }
            if let Some(limit_process_order) = limit_process_order {
                match limit_process_order {
                    0 => LIMIT_PROCESS_ORDER.save(deps.storage, &None)?,
                    x => LIMIT_PROCESS_ORDER.save(deps.storage, &Some(x))?,
                };
            }
            Ok(Response::new())
        }
        EstakingWithdrawElysStakingRewards {} => estaking_withdraw_elys_staking_rewards(info, deps),

        MasterchefClaimRewards { pool_ids } => masterchef_claim_rewards(info, pool_ids),

        EstakingWithdrawReward { validator_address } => {
            estaking_withdraw_reward(info, deps, validator_address)
        }
        ProcessOrders {} => {
            if info.sender != PARAMS_ADMIN.load(deps.storage)? {
                return Err(StdError::generic_err("Unauthorized").into());
            }

            let resp = process_orders(deps, env)?;
            Ok(resp)
        }
    }?;

    let resp = if let Some(account_history_address) = account_history_address {
        resp.add_message(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: account_history_address,
            msg: to_json_binary(&AccountHistoryMsg::AddUserAddressToQueue { user_address })?,
            funds: vec![],
        }))
    } else {
        resp
    };
    Ok(resp)
}
