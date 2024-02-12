use super::*;
use crate::msg::query_resp::earn::GetElysEarnProgramResp;
use cosmwasm_std::{Decimal, Deps};
use elys_bindings::{
    account_history::types::{earn_program::ElysEarnProgram, AprElys, BalanceReward, ElysDenom},
    query_resp::QueryAprResponse,
    types::{EarnType, StakedPosition, UnstakedPosition},
    ElysQuerier, ElysQuery,
};

pub fn get_elys_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    asset: String,
    usdc_denom: String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
    edenb_apr: QueryAprResponse,
) -> Result<GetElysEarnProgramResp, ContractError> {
    let denom = ElysDenom::Elys.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError {});
    }

    let querier = ElysQuerier::new(&deps.querier);
    let resp = GetElysEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    usdc_denom.clone(),
                    EarnType::ElysProgram as i32,
                )?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    ElysDenom::Eden.as_str().to_string(),
                    EarnType::ElysProgram as i32,
                )?;
                let uedenb_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    ElysDenom::EdenBoost.as_str().to_string(),
                    EarnType::ElysProgram as i32,
                )?;
                let mut available = querier.get_balance(addr.clone(), asset.clone())?;
                let mut staked = querier.get_staked_balance(addr.clone(), asset.clone())?;

                let mut staked_positions = querier.get_staked_positions(addr.clone())?;
                let mut unstaked_positions = querier.get_unstaked_positions(addr.clone())?;

                let uusdc_rewards_in_usd = uusdc_rewards
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .unwrap();

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc
                    .checked_mul(Decimal::from_atomics(ueden_rewards.amount, 0).unwrap())
                    .unwrap();
                ueden_rewards_in_usd = ueden_rewards_in_usd.checked_mul(uusdc_usd_price).unwrap();

                // have value in usd
                let mut available_in_usd = uelys_price_in_uusdc
                    .checked_mul(Decimal::from_atomics(available.amount, 0).unwrap())
                    .unwrap();
                available_in_usd = available_in_usd.checked_mul(uusdc_usd_price).unwrap();
                available.usd_amount = available_in_usd;

                let mut staked_in_usd = uelys_price_in_uusdc
                    .checked_mul(Decimal::from_atomics(staked.amount, 0).unwrap())
                    .unwrap();
                staked_in_usd = staked_in_usd.checked_mul(uusdc_usd_price).unwrap();
                staked.usd_amount = staked_in_usd;

                let new_staked_position = match staked_positions.staked_position {
                    Some(staked_positions) => {
                        let mut new_staked_positions: Vec<StakedPosition> = Vec::new();
                        for mut s in staked_positions {
                            s.staked.usd_amount = s
                                .staked
                                .usd_amount
                                .checked_mul(uelys_price_in_uusdc)
                                .unwrap();
                            s.staked.usd_amount =
                                s.staked.usd_amount.checked_mul(uusdc_usd_price).unwrap();
                            new_staked_positions.push(s)
                        }

                        new_staked_positions
                    }
                    None => vec![],
                };

                staked_positions.staked_position = Some(new_staked_position);

                let new_unstaked_position = match unstaked_positions.unstaked_position {
                    Some(unstaked_positions) => {
                        let mut new_unstaked_positions: Vec<UnstakedPosition> = Vec::new();
                        for mut s in unstaked_positions {
                            s.unstaked.usd_amount = s
                                .unstaked
                                .usd_amount
                                .checked_mul(uelys_price_in_uusdc)
                                .unwrap();
                            s.unstaked.usd_amount =
                                s.unstaked.usd_amount.checked_mul(uusdc_usd_price).unwrap();

                            s.remaining_time = s.remaining_time * 1000;
                            new_unstaked_positions.push(s)
                        }

                        new_unstaked_positions
                    }
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
            }
            None => ElysEarnProgram {
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
            },
        },
    };

    Ok(resp)
}
