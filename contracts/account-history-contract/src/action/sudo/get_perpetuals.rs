use cosmwasm_std::{DecCoin, Decimal256, Deps, StdError, StdResult};
use elys_bindings::trade_shield::msg::{query_resp::GetPerpetualPositionsForAddressResp, QueryMsg};
use elys_bindings::{ElysQuerier, ElysQuery};

use crate::types::{PerpetualAsset, PerpetualAssets};

pub fn get_perpetuals(
    deps: Deps<ElysQuery>,
    trade_shield_address: Option<String>,
    usdc_denom: &String,
    address: String,
) -> StdResult<PerpetualAssets> {
    let trade_shield_address = match trade_shield_address {
        Some(trade_shield_address) => trade_shield_address,
        None => return Ok(PerpetualAssets::default()),
    };

    let GetPerpetualPositionsForAddressResp { mtps, .. } = deps
        .querier
        .query_wasm_smart(
            trade_shield_address,
            &QueryMsg::PerpetualGetPositionsForAddress {
                address,
                pagination: None,
            },
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting perpetuals"))?;
    let mut perpetual_vec: Vec<PerpetualAsset> = vec![];
    let querier = ElysQuerier::new(&deps.querier);

    for mtp in mtps {
        match PerpetualAsset::new(mtp, usdc_denom.to_owned(), &querier) {
            Ok(perpetual_asset) => perpetual_vec.push(perpetual_asset),
            Err(_) => continue,
        }
    }

    let total_perpetual_asset_balance_amount = perpetual_vec
        .iter()
        .map(|perpetual| perpetual.size.amount)
        .fold(Decimal256::zero(), |acc, item| acc + item);
    let total_perpetual_asset_balance =
        DecCoin::new(total_perpetual_asset_balance_amount, usdc_denom.to_owned());

    Ok(PerpetualAssets {
        total_perpetual_asset_balance,
        perpetual_asset: perpetual_vec,
    })
}
