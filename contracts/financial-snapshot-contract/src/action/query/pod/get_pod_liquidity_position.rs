use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetLiquidityPositionResp};

pub fn get_pod_liquidity_position(deps: Deps<ElysQuery>, pool_id: u64) -> Result<GetLiquidityPositionResp, ContractError> {
    let liquidity_positions: Vec<LiquidityPosition> = LIQUIDITY_POSITIONS.load(deps.storage)?;
    let have_pool_id: Option<&LiquidityPosition> = liquidity_positions.iter().find(|liquidity_position| liquidity_position.pool_id == pool_id);
    let resp: GetLiquidityPositionResp = GetLiquidityPositionResp {
        liquidity_position: match have_pool_id {
            Some(liquidity_position) => liquidity_position.to_owned(),
            None => LiquidityPosition::new_dummy(),
        },
    };

    Ok(resp)
}
