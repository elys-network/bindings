use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier, query_resp::Lockup}, msg::query_resp::earn::GetUsdcEarnProgramResp};
use crate::types::{earn_program::usdc_earn::UsdcEarnProgram, ElysDenom};
use crate::types::{BalanceReward, AprUsdc};
use cosmwasm_std::{coin, Decimal, Uint128};
use elys_bindings::types::EarnType;

pub fn get_usdc_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetUsdcEarnProgramResp, ContractError> {
    let denom = ElysDenom::Usdc.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }
    
    let querier = ElysQuerier::new(&deps.querier);
       
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    
    let usdc_apr = querier.get_incentive_apr(EarnType::UsdcProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::UsdcProgram as i32, ElysDenom::Eden.as_str().to_string())?;

    let resp = GetUsdcEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::UsdcProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::UsdcProgram as i32)?;
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                
                let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                let elys_price_in_usd = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;

                let mut available = querier.get_balance(addr.clone(), usdc_denom.clone())?;
                available.usd_amount = available.usd_amount.checked_mul(usdc_usd_price).unwrap();

                let mut staked = querier.get_staked_balance(addr.clone(), usdc_denom.clone())?;
                staked.usd_amount = staked.usd_amount.checked_mul(usdc_usd_price).unwrap();
                
                let mut borrowed = querier.get_borrowed_balance(addr.clone())?;
                borrowed.usd_amount = borrowed.usd_amount.checked_mul(usdc_usd_price).unwrap();

                // have value in usd
                let mut eden_rewards_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_mul(usdc_usd_price).unwrap();
                
                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_mul(usdc_usd_price).unwrap();

                let new_lockups = match staked.lockups {
                    Some(lockups) => {
                        let mut new_lockups: Vec<Lockup> = Vec::new();
                        for mut lockup in lockups {
                            lockup.unlock_timestamp = lockup.unlock_timestamp*1000;
                            new_lockups.push(lockup)
                        }
                    
                        new_lockups
                    },
                    None => vec![],
                };

                staked.lockups = Some(new_lockups);

                UsdcEarnProgram {
                    bonding_period: 0,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(available),
                    staked: Some(staked),
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
                    borrowed: Some(borrowed),
                }
            },
            None => UsdcEarnProgram {
                bonding_period: 90,
                apr: AprUsdc {
                    uusdc: usdc_apr.apr.to_owned(),
                    ueden: eden_apr.apr.to_owned(),
                },
                available: None,
                staked: None,
                rewards: None,
                borrowed: None,
            }
        }
    };

    Ok(resp)
}