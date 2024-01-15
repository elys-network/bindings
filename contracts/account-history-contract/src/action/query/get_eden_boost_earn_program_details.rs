use super::*;
use crate::msg::query_resp::earn::GetEdenBoostEarnProgramResp;
use crate::types::{earn_program::eden_boost_earn::EdenBoostEarnProgram, ElysDenom, BalanceReward, AprUsdc};
use cosmwasm_std::{coin, Decimal, Uint128, DepsMut};
use elys_bindings::{types::EarnType,ElysQuery, ElysQuerier};

pub fn get_eden_boost_earn_program_details(deps: &DepsMut<ElysQuery>, address: Option<String>, asset: String) -> Result<GetEdenBoostEarnProgramResp, ContractError> {
    let denom = ElysDenom::EdenBoost.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError{});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let usdc_denom_entry = querier.get_asset_profile(ElysDenom::Usdc.as_str().to_string())?;
    let usdc_denom = usdc_denom_entry.entry.denom;
    let usdc_display_denom = usdc_denom_entry.entry.display_name;
    let usdc_decimal = u64::checked_pow(10, usdc_denom_entry.entry.decimals as u32).unwrap();

    let eden_denom_entry = querier.get_asset_profile(ElysDenom::Eden.as_str().to_string())?;
    let eden_decimal = u64::checked_pow(10, eden_denom_entry.entry.decimals as u32).unwrap();

    let usdc_apr = querier.get_incentive_apr(EarnType::EdenBProgram as i32, ElysDenom::Usdc.as_str().to_string())?;
    let eden_apr = querier.get_incentive_apr(EarnType::EdenBProgram as i32, ElysDenom::Eden.as_str().to_string())?;

    let resp = GetEdenBoostEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), usdc_denom.clone(), EarnType::EdenBProgram as i32)?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(addr.clone(), ElysDenom::Eden.as_str().to_string(), EarnType::EdenBProgram as i32)?;
                
                let available = querier.get_balance(addr.clone(), asset.clone())?;
                let staked = querier.get_staked_balance(addr.clone(), asset.clone())?;
                
                let discount = Decimal::from_atomics(Uint128::new(1000000), 0).unwrap();
                let usdc_oracle_price = querier.get_oracle_price(usdc_display_denom.clone(), ElysDenom::AnySource.as_str().to_string(), 0)?;
                let uusdc_usd_price = usdc_oracle_price.price.price.checked_div(Decimal::from_atomics(Uint128::new(usdc_decimal as u128), 0).unwrap()).unwrap();
                let uelys_price_in_uusdc = querier.get_amm_price_by_denom(coin(Uint128::new(1000000).u128(), ElysDenom::Elys.as_str().to_string()), discount)?;

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc.checked_mul(Decimal::from_atomics(ueden_rewards.amount, 0).unwrap()).unwrap();
                ueden_rewards_in_usd = ueden_rewards_in_usd.checked_mul(uusdc_usd_price).unwrap();
                
                let uusdc_rewards_in_usd = uusdc_rewards.usd_amount.checked_mul(uusdc_usd_price).unwrap();

                EdenBoostEarnProgram {
                    bonding_period: 0,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr.to_owned(),
                        ueden: eden_apr.apr.to_owned(),
                    },
                    available: Some(available.amount.checked_div(Uint128::new(eden_decimal as u128)).unwrap()),
                    staked: Some(staked.amount.checked_div(Uint128::new(eden_decimal as u128)).unwrap()),
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