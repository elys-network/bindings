use super::*;
use crate::{bindings::{query::ElysQuery, querier::ElysQuerier}, msg::query_resp::earn::GetElysEarnProgramResp};
use crate::types::{earn_program::elys_earn::ElysEarnProgram, ElysDenom, BalanceReward, AprElys, StakedPosition, UnstakedPosition};
use cosmwasm_std::{coin, Decimal, Uint128};
use elys_bindings::types::EarnType;

pub fn get_elys_earn_program_details(deps: Deps<ElysQuery>, address: Option<String>, asset: String) -> Result<GetElysEarnProgramResp, ContractError> {
    let denom = ElysDenom::Elys.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
   
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    
    let usdc_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Eden.as_str().to_string())?;
    let edenb_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::EdenBoost.as_str().to_string())?;

    let resp = GetElysEarnProgramResp {
        data: match address {
            Some(addr) => {
                let usdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::ElysProgram as i32)?;
                let eden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let edenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let mut staked_positions = querier.get_staked_positions(addr.clone())?;
                let mut unstaked_positions = querier.get_unstaked_positions(addr.clone())?;

                let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let usdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(1000000), 0).unwrap()).unwrap();
                
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                let usdc_rewards_in_usd = usdc_rewards.usd_amount.checked_mul(usdc_usd_price).unwrap();
                let elys_price_in_usd = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;

                // have value in usd
                let mut eden_rewards_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(eden_rewards.amount, 0).unwrap()).unwrap();
                eden_rewards_in_usd = eden_rewards_in_usd.checked_mul(usdc_usd_price).unwrap();

                // have value in usd
                let mut available_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(available.amount, 0).unwrap()).unwrap();
                available_in_usd = available_in_usd.checked_mul(usdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                let mut staked_in_usd = elys_price_in_usd.checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap()).unwrap();
                staked_in_usd = staked_in_usd.checked_mul(usdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;
              
                let new_staked_position = match staked_positions.staked_position {
                    Some(staked_positions) => {
                        let mut new_staked_positions: Vec<StakedPosition> = Vec::new();
                        for mut s in staked_positions {
                            s.staked.usd_amount = s.staked.usd_amount.checked_mul(elys_price_in_usd).unwrap();
                            s.staked.usd_amount = s.staked.usd_amount.checked_mul(usdc_usd_price).unwrap();
                            new_staked_positions.push(s)
                        }
                    
                        new_staked_positions
                    },
                    None => vec![],
                };

                staked_positions.staked_position = Some(new_staked_position);

                let new_unstaked_position = match unstaked_positions.unstaked_position {
                    Some(unstaked_positions) => {
                        let mut new_unstaked_positions: Vec<UnstakedPosition> = Vec::new();
                        for mut s in unstaked_positions {
                            s.unstaked.usd_amount = s.unstaked.usd_amount.checked_mul(elys_price_in_usd).unwrap();
                            s.unstaked.usd_amount = s.unstaked.usd_amount.checked_mul(usdc_usd_price).unwrap();

                            s.remaining_time = s.remaining_time*1000;
                            new_unstaked_positions.push(s)
                        }
                    
                        new_unstaked_positions
                    },
                    None => vec![],
                };

                unstaked_positions.unstaked_position = Some(new_unstaked_position);

                ElysEarnProgram {
                    bonding_period: 14,
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
                    staked_positions: staked_positions.staked_position,
                    unstaked_positions: unstaked_positions.unstaked_position,
                }
            },
            None => {
                ElysEarnProgram {
                    bonding_period: 90,
                    apr: AprElys {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr,
                        uedenb: edenb_apr.apr,
                    },
                    available: None,
                    staked: None,
                    rewards: None,
                    staked_positions: None,
                    unstaked_positions: None,
                }
            }
        }
    };

    Ok(resp)
}