use super::*;
use crate::msg::query_resp::earn::GetUsdcEarnProgramResp;
use cosmwasm_std::{Decimal, Deps};
use elys_bindings::{
    account_history::types::{earn_program::UsdcEarnProgram, AprUsdc, BalanceReward, ElysDenom},
    query_resp::QueryAprResponse,
    types::EarnType,
    ElysQuerier, ElysQuery,
};

pub fn get_usdc_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    asset: String,
    usdc_denom: String,
    usdc_base_denom: String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
) -> Result<GetUsdcEarnProgramResp, ContractError> {
    let denom = ElysDenom::Usdc.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError {});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let resp = GetUsdcEarnProgramResp {
        data: match address {
            Some(addr) => {
                let uusdc_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    usdc_denom.clone(),
                    EarnType::UsdcProgram as i32,
                )?;
                let ueden_rewards = querier.get_sub_bucket_rewards_balance(
                    addr.clone(),
                    ElysDenom::Eden.as_str().to_string(),
                    EarnType::UsdcProgram as i32,
                )?;

                let mut available = querier.get_balance(addr.clone(), usdc_denom.clone())?;
                available.usd_amount = available
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);

                let mut staked =
                    querier.get_staked_balance(addr.clone(), usdc_base_denom.clone())?;

                let mut borrowed = querier.get_borrowed_balance()?;
                borrowed.usd_amount = borrowed
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);

                // have value in usd
                let mut ueden_rewards_in_usd = uelys_price_in_uusdc
                    .checked_mul(
                        Decimal::from_atomics(ueden_rewards.amount, 0)
                            .map_or(Decimal::zero(), |res| res),
                    )
                    .map_or(Decimal::zero(), |res| res);
                ueden_rewards_in_usd = ueden_rewards_in_usd
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);

                let uusdc_rewards_in_usd = uusdc_rewards
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);
                staked.lockups = None;

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
                            amount: uusdc_rewards.amount,
                            usd_amount: Some(uusdc_rewards_in_usd),
                        },
                        BalanceReward {
                            asset: ElysDenom::Eden.as_str().to_string(),
                            amount: ueden_rewards.amount,
                            usd_amount: Some(ueden_rewards_in_usd),
                        },
                    ]),
                    borrowed: Some(borrowed),
                }
            }
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
            },
        },
    };

    Ok(resp)
}
