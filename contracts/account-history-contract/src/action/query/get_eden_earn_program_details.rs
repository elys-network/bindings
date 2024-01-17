use super::*;
use crate::msg::query_resp::earn::GetEdenEarnProgramResp;
use crate::types::{earn_program::eden_earn::EdenEarnProgram, ElysDenom, BalanceReward, AprElys};
use cosmwasm_std::{Decimal, DepsMut};
use elys_bindings::{types::EarnType, ElysQuery, ElysQuerier, query_resp::QueryAprResponse};

pub fn get_eden_earn_program_details(deps: &DepsMut<ElysQuery>,
    address: Option<String>,
    asset: String,
    usdc_denom: String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
    edenb_apr: QueryAprResponse,
) -> Result<GetEdenEarnProgramResp, ContractError> {
    let denom = ElysDenom::Eden.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
    
    let resp = GetEdenEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::EdenProgram as i32)?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let uedenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let mut staked_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap()).unwrap();
                staked_in_usd = staked_in_usd.checked_mul(uusdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;

                // have value in usd
                let mut available_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(available.amount, 0).unwrap()).unwrap();
                available_in_usd = available_in_usd.checked_mul(uusdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(ueden_rewards.amount, 0).unwrap()).unwrap();
                ueden_rewards_in_usd = ueden_rewards_in_usd.checked_mul(uusdc_usd_price).unwrap();

                let uusdc_rewards_in_usd = uusdc_rewards.usd_amount.checked_mul(uusdc_usd_price).unwrap();

                EdenEarnProgram {
                    bonding_period: 0,
                    apr: AprElys {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr,
                        uedenb: edenb_apr.apr,
                    },
                    available: Some(available),
                    staked: Some(staked),
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
                        BalanceReward {
                            asset: ElysDenom::EdenBoost.as_str().to_string(),
                            amount: uedenb_rewards.amount,
                            usd_amount: None,
                        },
                    ]),
                    vesting: None,
                    vesting_details: None,
                }
            },
            None => {
                EdenEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr,
                        uedenb: edenb_apr.apr,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                    vesting: None,
                    vesting_details: None,
                }
            }
        }
    };

    Ok(resp)
}