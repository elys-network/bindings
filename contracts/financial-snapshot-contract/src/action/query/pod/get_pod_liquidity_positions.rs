use elys_bindings::ElysQuery;

use super::*;
use crate::msg::query_resp::pod::GetLiquidityPositionsResp;

pub fn get_pod_liquidity_positions(
    deps: Deps<ElysQuery>,
) -> Result<GetLiquidityPositionsResp, ContractError> {
    let liquidity_positions: Vec<LiquidityPosition> = LIQUIDITY_POSITIONS.load(deps.storage)?;
    let resp: GetLiquidityPositionsResp;

    if liquidity_positions.len() > 0 {
        resp = GetLiquidityPositionsResp {
            data: liquidity_positions,
        };
    } else {
        resp = GetLiquidityPositionsResp {
            data: LiquidityPosition::new_dummys(),
        };
    }

    Ok(resp)
}
