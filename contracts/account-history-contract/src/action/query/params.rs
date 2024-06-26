use crate::{
    msg::query_resp::ParamsResp,
    states::{
        DELETE_EPOCH, DELETE_OLD_DATA_ENABLED, EXPIRATION, METADATA, PROCESSED_ACCOUNT_PER_BLOCK,
        TRADE_SHIELD_ADDRESS, UPDATE_ACCOUNT_ENABLED,
    },
};
use cosmwasm_std::{Deps, StdResult};
use elys_bindings::ElysQuery;

pub fn params(deps: Deps<ElysQuery>) -> StdResult<ParamsResp> {
    let expiration = EXPIRATION.load(deps.storage)?;
    let processed_account_per_block = PROCESSED_ACCOUNT_PER_BLOCK.load(deps.storage)?;
    let trade_shield_address = TRADE_SHIELD_ADDRESS.load(deps.storage)?;
    let update_account_enabled = UPDATE_ACCOUNT_ENABLED.load(deps.storage)?;
    let metadata = METADATA.load(deps.storage)?;
    let delete_old_data_enabled = DELETE_OLD_DATA_ENABLED.load(deps.storage)?;
    let delete_epoch = DELETE_EPOCH.load(deps.storage)?;

    Ok(ParamsResp {
        expiration,
        processed_account_per_block,
        update_account_enabled,
        trade_shield_address,
        metadata,
        delete_old_data_enabled,
        delete_epoch,
    })
}
