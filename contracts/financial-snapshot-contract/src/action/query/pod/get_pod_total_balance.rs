use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetTotalBalanceResp };

pub fn get_pod_total_balance(deps: Deps<ElysQuery>, address: String) -> Result<GetTotalBalanceResp, ContractError> {
    let ret = TOTAL_BALANCE.may_load(deps.storage, &address);
    let resp = GetTotalBalanceResp {
        data: match ret {
            Ok(Some(data)) => data.to_owned(),
            Ok(None) => TotalBalance::new_dummy(),
            Err(_) => return Err(ContractError::TotalBalanceError{}),
        },
    };

    Ok(resp)
}
