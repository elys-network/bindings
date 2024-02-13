use super::*;
use crate::msg::query_resp::earn::GetEdenBoostEarnProgramResp;
use cosmwasm_std::{Decimal, Deps, Uint128};
use elys_bindings::{
    account_history::types::{
        earn_program::EdenBoostEarnProgram, AprUsdc, BalanceReward, ElysDenom,
    },
    query_resp::QueryAprResponse,
    types::EarnType,
    ElysQuerier, ElysQuery,
};

pub fn get_eden_boost_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    asset: String,
    usdc_denom: String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    eden_decimal: u64,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    let denom = ElysDenom::EdenBoost.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError {});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let resp = GetEdenBoostEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    usdc_denom.clone(),
                    EarnType::EdenBProgram as i32,
                )?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    ElysDenom::Eden.as_str().to_string(),
                    EarnType::EdenBProgram as i32,
                )?;

                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc
                    .checked_mul(Decimal::from_atomics(ueden_rewards.amount, 0).unwrap())
                    .unwrap();
                ueden_rewards_in_usd = ueden_rewards_in_usd.checked_mul(uusdc_usd_price).unwrap();

                let uusdc_rewards_in_usd = uusdc_rewards
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .unwrap();

                EdenBoostEarnProgram {
                    bonding_period: 0,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(
                        available
                            .amount
                            .checked_div(Uint128::new(eden_decimal as u128))
                            .unwrap(),
                    ),
                    staked: Some(
                        staked
                            .amount
                            .checked_div(Uint128::new(eden_decimal as u128))
                            .unwrap(),
                    ),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: ElysDenom::Usdc.as_str().to_string(),
                            amount: uusdc_rewards.amount,
                            usd_amount: Some(uusdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: ueden_rewards.amount,
                            usd_amount: Some(ueden_rewards_in_usd),
                        },
                    ]),
                }
            }
            None => EdenBoostEarnProgram {
                bonding_period: 90,
                apr: AprUsdc {
                    uusdc: usdc_apr.apr.to_owned(),
                    ueden: eden_apr.apr.to_owned(),
                },
                available: None,
                staked: None,
                rewards: None,
            },
        },
    };

    Ok(resp)
}
