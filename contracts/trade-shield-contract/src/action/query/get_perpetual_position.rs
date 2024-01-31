use cosmwasm_std::StdError;

use super::*;

pub fn get_perpetual_position(
    deps: Deps<ElysQuery>,
    address: String,
    id: u64,
) -> Result<GetPerpetualPositionResp, ContractError> {
    let querier = ElysQuerier::new(&deps.querier);

    let mtp = match querier.mtp(address, id)?.mtp {
        Some(mtp) => mtp,
        None => return Err(StdError::not_found("perpetual trading position").into()),
    };

    let mtp_plus = PerpetualPositionPlus::new(mtp, deps.storage, &querier)?;

    Ok(GetPerpetualPositionResp { mtp: mtp_plus })
}
