use super::*;
use crate::msg::query_resp::earn::GetElysEarnProgramResp;
use crate::types::{earn_program::elys_earn::ElysEarnProgram, ElysDenom, BalanceReward, AprElys};
use cosmwasm_std::{coin, Decimal, Uint128, DepsMut};
use elys_bindings::{types::{EarnType, StakedPosition, UnstakedPosition}, ElysQuery, ElysQuerier};

pub fn get_elys_earn_program_details(deps: &DepsMut<ElysQuery>, address: Option<String>, asset: String) -> Result<GetElysEarnProgramResp, ContractError> {
    let denom = ElysDenom::Elys.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);
   
    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

    let usdc_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::Eden.as_str().to_string())?;
    let edenb_apr = querier.get_incentive_apr(EarnType::ElysProgram as i32, ElysDenom::EdenBoost.as_str().to_string())?;

    let resp = GetElysEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::ElysProgram as i32)?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let uedenb_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::EdenBoost.as_str().to_string(), EarnType::ElysProgram as i32)?;
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let mut staked_positions = querier.get_staked_positions(addr.clone())?;
                let mut unstaked_positions = querier.get_unstaked_positions(addr.clone())?;

                let uusdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let uusdc_usd_price = uusdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(usdc_decimal as u128), 0).unwrap()).unwrap();
                
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                let uusdc_rewards_in_usd = uusdc_rewards.usd_amount.checked_mul(uusdc_usd_price).unwrap();
                let uelys_price_in_uusdc = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(ueden_rewards.amount, 0).unwrap()).unwrap();
                ueden_rewards_in_usd = ueden_rewards_in_usd.checked_mul(uusdc_usd_price).unwrap();

                // have value in usd
                let mut available_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(available.amount, 0).unwrap()).unwrap();
                available_in_usd = available_in_usd.checked_mul(uusdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                let mut staked_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap()).unwrap();
                staked_in_usd = staked_in_usd.checked_mul(uusdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;
              
                let new_staked_position = match staked_positions.staked_position {
                    Some(staked_positions) => {
                        let mut new_staked_positions: Vec<StakedPosition> = Vec::new();
                        for mut s in staked_positions {
                            s.staked.usd_amount = s.staked.usd_amount.checked_mul(uelys_price_in_uusdc).unwrap();
                            s.staked.usd_amount = s.staked.usd_amount.checked_mul(uusdc_usd_price).unwrap();
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
                            s.unstaked.usd_amount = s.unstaked.usd_amount.checked_mul(uelys_price_in_uusdc).unwrap();
                            s.unstaked.usd_amount = s.unstaked.usd_amount.checked_mul(uusdc_usd_price).unwrap();

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