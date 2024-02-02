use cosmwasm_std::{DecCoin, Decimal, Decimal256, Deps, StdResult};
use elys_bindings::ElysQuery;

use crate::{
    action::VALUE_DENOM,
    msg::query_resp::{GetLiquidAssetsResp, LiquidAsset},
    states::HISTORY,
    types::CoinValue,
};

pub fn get_liquid_assets(
    deps: Deps<ElysQuery>,
    user_address: String,
) -> StdResult<GetLiquidAssetsResp> {
    let value_denom = VALUE_DENOM.load(deps.storage)?;
    let snapshots = match HISTORY.may_load(deps.storage, &user_address)? {
        Some(snapshots) => snapshots,
        None => {
            return Ok(GetLiquidAssetsResp {
                liquid_assets: vec![],
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), value_denom),
            });
        }
    };
    let snapshot = match snapshots.last().cloned() {
        Some(snapshot) => snapshot,
        None => {
            return Ok(GetLiquidAssetsResp {
                liquid_assets: vec![],
                total_liquid_asset_balance: DecCoin::new(Decimal256::zero(), value_denom),
            });
        }
    };

    let mut liquid_assets: Vec<LiquidAsset> = vec![];

    for total in snapshot.liquid_asset.total_value_per_asset {
        let (available_amount, available_value) =
            get_info(&snapshot.liquid_asset.available_asset_balance, &total.denom);
        let (in_order_amount, in_order_value) =
            get_info(&snapshot.liquid_asset.in_orders_asset_balance, &total.denom);

        liquid_assets.push(LiquidAsset {
            denom: total.denom,
            price: total.price,
            available_amount,
            available_value,
            in_order_amount,
            in_order_value,
            total_amount: total.amount_token,
            total_value: total.amount_usdc,
        });
    }

    Ok(GetLiquidAssetsResp {
        liquid_assets,
        total_liquid_asset_balance: snapshot.liquid_asset.total_liquid_asset_balance,
    })
}

fn get_info(list_info: &Vec<CoinValue>, denom: &String) -> (Decimal, Decimal) {
    match list_info.iter().find(|info| &info.denom == denom).cloned() {
        Some(data) => (data.amount_token, data.amount_usdc),
        None => (Decimal::zero(), Decimal::zero()),
    }
}
