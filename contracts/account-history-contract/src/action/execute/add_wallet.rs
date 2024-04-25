use crate::states::ACCOUNT_LIST;
use cosmwasm_std::{StdResult, Storage};

pub fn add_wallet(storage: &mut dyn Storage, wallet: String) -> StdResult<()> {
    let empty = ();

    if ACCOUNT_LIST.has(storage, wallet.as_str()) == false {
        ACCOUNT_LIST.save(storage, wallet.as_str(), &empty)?;
    }

    Ok(())
}
