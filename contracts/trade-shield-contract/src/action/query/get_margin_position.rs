use cosmwasm_std::StdError;

use super::*;

pub fn get_margin_position(
    deps: Deps<ElysQuery>,
    address: String,
    id: u64,
) -> Result<GetMarginPositionResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let mtp = match querier.mtp(address, id)?.mtp {
        Some(mtp) => mtp,
        None => return Err(StdError::not_found("margin trading position").into()),
    };

    let mtp_plus = MarginPositionPlus::new(mtp, deps.storage, &querier)?;

    Ok(GetMarginPositionResp { mtp: mtp_plus })
}
