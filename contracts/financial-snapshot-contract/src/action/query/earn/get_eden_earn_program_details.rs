use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetEdenEarnProgramResp};
use crate::types::{earn_program::eden_earn::EdenEarnProgram, ElysDenom, BalanceReward, AprElys, VestingDetail};
use cosmwasm_std::{coin, Decimal, Uint128};
use elys_bindings::types::EarnType;

pub fn get_eden_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenEarnProgramResp, ContractError> {
    let denom = ElysDenom::Eden.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
    
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    
    let usdc_apr = querier.get_incentive_apr(EarnType::EdenProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::EdenProgram as i32, ElysDenom::Eden.as_str().to_string())?;
    let edenb_apr = querier.get_incentive_apr(EarnType::EdenProgram as i32, ElysDenom::EdenBoost.as_str().to_string())?;

    let resp = GetEdenEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::EdenProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let edenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::EdenProgram as i32)?;
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                let mut vesting_info = querier.get_vesting_info(addr.clone())?;
                
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                
                let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                
                // have value in usd
                let elys_price_in_usd = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;
  
                let mut staked_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap()).unwrap();
                staked_in_usd = staked_in_usd.checked_mul(usdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;

                // have value in usd
                let mut available_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(available.amount, 0).unwrap()).unwrap();
                available_in_usd = available_in_usd.checked_mul(usdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                // have value in usd
                let mut eden_rewards_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_mul(usdc_usd_price).unwrap();
                
                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_mul(usdc_usd_price).unwrap();

                let total_vesting_in_usd = vesting_info.vesting.usd_amount.checked_mul(usdc_usd_price).unwrap();
                vesting_info.vesting.usd_amount = total_vesting_in_usd;
                
                let new_vesting_details = match vesting_info.vesting_details {
                    Some(vesting_detials) => {
                        let mut new_vesting_details: Vec<VestingDetail> = Vec::new();
                        for mut v in vesting_detials {
                            v.remaining_time = v.remaining_time*1000;
                            v.balance_vested.usd_amount = v.balance_vested.usd_amount.checked_mul(usdc_usd_price).unwrap();
                            v.remaining_vest.usd_amount = v.remaining_vest.usd_amount.checked_mul(elys_price_in_usd).unwrap();
                            v.remaining_vest.usd_amount = v.remaining_vest.usd_amount.checked_mul(usdc_usd_price).unwrap();
                            
                            v.total_vest.usd_amount = v.total_vest.usd_amount.checked_mul(elys_price_in_usd).unwrap();
                            v.total_vest.usd_amount = v.total_vest.usd_amount.checked_mul(usdc_usd_price).unwrap();

                            new_vesting_details.push(v)
                        }
                    
                        new_vesting_details
                    },
                    None => vec![],
                };
                vesting_info.vesting_details = Some(new_vesting_details);
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
                            amount: usdc_rewards.amount,
                            usd_amount: Some(usdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: eden_rewards.amount,
                            usd_amount: Some(eden_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::EdenBoost.as_str().to_string(),
                            amount: edenb_rewards.amount,
                            usd_amount: None,
                        },
                    ]),
                    vesting: Some(vesting_info.vesting),
                    vesting_details: vesting_info.vesting_details,
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