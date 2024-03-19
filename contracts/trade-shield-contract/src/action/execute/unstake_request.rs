use super::*;
use cosmwasm_std::{Int128, StdError};

pub fn unstake_request(
    info: MessageInfo,
    deps: DepsMut<ElysQuery>,
    // the amount to be staked in base denomination.
    amount: u64,
    // The asset to be staked
    asset: String,
    // The validator Address is required only if the staked asset is
    // uelys.
    validator_address: Option<String>,
) -> Result<Response<ElysMsg>, ContractError> {
    if STAKE_ENABLED.load(deps.storage)? == false {
        return Err(StdError::generic_err("stake endpoint is disable").into());
    }

    let querier = ElysQuerier::new(&deps.querier);
    let denom_entry = querier.get_asset_profile(asset.clone())?;
    let real_denom = denom_entry.entry.denom;
    let uelys_denom = "uelys".to_string();

    if amount == 0 {
        return Err(StdError::generic_err("amount is zero").into());
    }

    if validator_address.is_none() && asset == uelys_denom {
        return Err(StdError::generic_err(
            "The validator Address is required only if the staked asset is uelys",
        )
        .into());
    };

    let msg = ElysMsg::unstake_token(
        info.sender.into_string(),
        Int128::from(amount),
        real_denom,
        validator_address,
    );

    let resp = Response::new().add_message(msg);
    Ok(resp)
}
