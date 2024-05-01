use super::*;
use crate::msg::query_resp::earn::GetEdenBoostEarnProgramResp;
use cosmwasm_std::Deps;
use elys_bindings::{
    account_history::types::{
        earn_detail::earn_detail::AprEdenBoost, earn_program::EdenBoostEarnProgram, ElysDenom
    },
    query_resp::{QueryAprResponse, Validator},
    ElysQuerier, ElysQuery,
};

pub fn get_eden_boost_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    asset: String,
    usdc_denom: String,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    let bonding_period = 0;
    let denom = ElysDenom::EdenBoost.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError {});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let resp = GetEdenBoostEarnProgramResp {
        data: match address {
            Some(addr) => {
                let all_rewards = querier.get_estaking_rewards(addr.clone()).unwrap_or_default();
                let program_rewards = all_rewards
                    .get_validator_rewards(Validator::EdenBoost)
                    .to_dec_coin_values(&querier, &usdc_denom.clone())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|coin| coin.1)
                    .collect();

                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr, asset)?;

                EdenBoostEarnProgram {
                    bonding_period,
                    apr: AprEdenBoost {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(available.amount),
                    staked: Some(staked.amount),
                    rewards: Some(program_rewards)
                }
            }
            None => EdenBoostEarnProgram::default(),
        },
    };

    Ok(resp)
}
