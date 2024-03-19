use super::*;
use cosmwasm_std::{Int128, StdError};

pub fn stake_request(
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
    let address = info.sender.into_string();
    let uelys_denom = "uelys".to_string();

    if amount == 0 {
        return Err(StdError::generic_err("amount is zero").into());
    }

    let denom_entry = querier.get_asset_profile(asset.clone())?;
    let real_denom = denom_entry.entry.denom;

    let balance = querier.get_balance(address.to_owned(), real_denom.clone())?;
    let token_amount: u128 = balance.amount.into();
    if token_amount < amount as u128 {
        return Err(ContractError::InsufficientBalanceError {
            balance: balance.amount.into(),
            amount,
        });
    }
    if validator_address.is_none() && asset == uelys_denom {
        return Err(StdError::generic_err(
            "The validator Address is required only if the staked asset is uelys",
        )
        .into());
    };

    let msg = ElysMsg::stake_token(
        address,
        Int128::from(amount),
        real_denom.clone(),
        validator_address,
    );
    let resp = Response::new().add_message(msg);
    Ok(resp)
}
