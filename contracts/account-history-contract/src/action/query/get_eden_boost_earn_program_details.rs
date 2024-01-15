use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetEdenBoostEarnProgramResp};
use crate::types::{earn_program::eden_boost_earn::EdenBoostEarnProgram, ElysDenom, BalanceReward, AprUsdc};
use cosmwasm_std::{coin, Decimal, Uint128};
use elys_bindings::types::EarnType;

pub fn get_eden_boost_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    let denom = ElysDenom::EdenBoost.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;

    let usdc_apr = querier.get_incentive_apr(EarnType::EdenBProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::EdenBProgram as i32, ElysDenom::Eden.as_str().to_string())?;

    let resp = GetEdenBoostEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::EdenBProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EdenBProgram as i32)?;
                
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                let elys_price_in_usd = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;

                // have value in usd
                let mut eden_rewards_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_mul(usdc_usd_price).unwrap();
                
                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_mul(usdc_usd_price).unwrap();

                EdenBoostEarnProgram {
                    bonding_period: 0,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(available.amount),
                    staked: Some(staked.amount),
                    rewards: Some(vec![
                        BalanceReward {
                            asset: ElysDenom::Usdc.as_str().to_string(),
                            amount: usdc_rewards.amount,
                            usd_amount: Some(usdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: eden_rewards.amount,
                            usd_amount: Some(eden_rewards_in_usd),
                        },
                    ]),
                }
            },
            None => {
                EdenBoostEarnProgram {
                    bonding_period: 90,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                }
            }
        }
    };

    Ok(resp)
}