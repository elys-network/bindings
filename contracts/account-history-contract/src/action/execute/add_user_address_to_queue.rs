use crate::states::USER_ADDRESS_QUEUE;
use cosmwasm_std::{StdResult, Storage};

pub fn add_user_address_to_queue(storage: &mut dyn Storage, user_address: String) -> StdResult<()> {
    let empty = ();

    if USER_ADDRESS_QUEUE.has(storage, user_address.as_str()) == false {
        USER_ADDRESS_QUEUE.save(storage, user_address.as_str(), &empty)?;
    }

    Ok(())
}
