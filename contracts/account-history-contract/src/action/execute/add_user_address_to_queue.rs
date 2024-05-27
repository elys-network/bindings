use crate::states::USER_ADDRESS_QUEUE;
use cosmwasm_std::{DepsMut, StdResult};
use elys_bindings::ElysQuery;

pub fn add_user_address_to_queue(deps: DepsMut<ElysQuery>, user_address: String) -> StdResult<()> {
    USER_ADDRESS_QUEUE.push_front(deps.storage, &user_address)?;

    Ok(())
}
