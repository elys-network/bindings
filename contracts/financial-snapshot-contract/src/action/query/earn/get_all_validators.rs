use cosmwasm_std::{Decimal, StdError};
use elys_bindings::{
    account_history::types::ElysDenom, query_resp::QueryDelegatorValidatorsResponse, ElysQuerier,
    ElysQuery,
};

use super::*;

pub fn get_all_validators(
    deps: Deps<ElysQuery>,
    delegator_address: Option<String>,
) -> Result<QueryDelegatorValidatorsResponse, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let uelys_price_in_uusdc = querier.get_asset_price(ElysDenom::Elys.as_str())?;

    let usdc_denom_entry = querier
        .get_asset_profile(ElysDenom::Usdc.as_str().to_string())
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc denom"))?;
    let usdc_oracle_price = querier
        .get_oracle_price(
            usdc_denom_entry.entry.display_name,
            ElysDenom::AnySource.as_str().to_string(),
            0,
        )
        .map_err(|_| StdError::generic_err("an error occurred while getting usdc price"))?;
    let uusdc_usd_price = usdc_oracle_price.price.price;

    let mut resp = match delegator_address {
        Some(address) => querier.get_all_validators(address)?,
        None => querier.get_all_validators("".to_string())?,
    };

    if let Some(validators) = resp.validators.as_mut() {
        for validator in validators.iter_mut() {
            if let Some(staked) = &mut validator.staked {
                staked.usd_amount = Decimal::from_atomics(staked.amount, 6)
                    .unwrap()
                    .checked_mul(uelys_price_in_uusdc)
                    .unwrap()
                    .checked_mul(uusdc_usd_price)
                    .unwrap();
            }
        }
    }

    Ok(resp)
}
