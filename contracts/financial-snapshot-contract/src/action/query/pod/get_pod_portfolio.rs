use super::*;
use crate::{bindings::query::ElysQuery, msg::query_resp::pod::GetPortfolioResp};

pub fn get_pod_portfolio(deps: Deps<ElysQuery>, address: String) -> Result<GetPortfolioResp, ContractError> {
    let ret = PORTFOLIO.may_load(deps.storage, &address);
    let resp = GetPortfolioResp {
        portfolio: match ret {
            Ok(Some(data)) => data.to_owned(),
            Ok(None) => Portfolio::new_dummy(),
            Err(_) => return Err(ContractError::PortfolioError{}),
        },
    };
   
    Ok(resp)
}
