use super::*;
use cosmwasm_std::{Decimal, Int128, SubMsg};
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

    match msg {
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
            owner_address,
            order_type,
        } => cancel_spot_orders(info, deps, order_ids, owner_address, order_type),

        CreatePerpetualOrder {
            position,
            leverage,
            trading_asset,
            take_profit_price,
            order_type,
            trigger_price,
            position_id,
        } => create_perpetual_order(
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
            owner_address,
            order_type,
        } => cancel_perpetual_orders(info, deps, order_ids, owner_address, order_type),
        ClosePerpetualPosition { id, amount } => close_perpetual_position(info, id, amount),

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
        ClaimRewardsRequest { withdraw_type } => claim_rewards_request(info, deps, withdraw_type),
        ClaimValidatorCommissionRequest { validator_address } => {
            claim_validator_commission_request(info, deps, validator_address)
        }
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
                discount: Decimal::zero(),
                recipient: info.sender.into_string(),
            };

            let sub_msg = SubMsg::reply_always(msg, 1);

            Ok(Response::new().add_submessage(sub_msg))
        }
    }
}
