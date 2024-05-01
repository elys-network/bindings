use super::*;
use crate::msg::query_resp::earn::GetUsdcEarnProgramResp;
use cosmwasm_std::{Decimal, Deps};
use elys_bindings::{
    account_history::types::{earn_program::UsdcEarnProgram, AprUsdc, ElysDenom},
    ElysQuerier, ElysQuery,
};

pub fn get_usdc_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    usdc_denom: String,
    usdc_base_denom: String,
    uusdc_usd_price: Decimal
) -> Result<GetUsdcEarnProgramResp, ContractError> {
    let pool_id = 32767u64;
    let bonding_period = 0;

    let querier = ElysQuerier::new(&deps.querier);

    let eden_apr = querier.get_masterchef_stable_stake_apr(ElysDenom::Eden.as_str().to_string()).unwrap_or_default();
    let usdc_apr = querier.get_masterchef_stable_stake_apr(ElysDenom::Usdc.as_str().to_string()).unwrap_or_default();

    let resp = GetUsdcEarnProgramResp {
        data: match address {
            Some(addr) => {
                let rewards = querier.get_masterchef_pending_rewards(addr.clone()).unwrap_or_default();
                let coin_values_rewards = rewards.to_coin_values(&querier, &usdc_denom.clone()).unwrap_or_default();
                let pool_rewards = coin_values_rewards.0[&pool_id].clone();

                let mut available = querier.get_balance(addr.clone(), usdc_denom.clone())?;
                available.usd_amount = available
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);

                let mut staked =
                    querier.get_staked_balance(addr, usdc_base_denom)?;

                let mut borrowed = querier.get_borrowed_balance()?;
                borrowed.usd_amount = borrowed
                    .usd_amount
                    .checked_mul(uusdc_usd_price)
                    .map_or(Decimal::zero(), |res| res);

                staked.lockups = None;

                UsdcEarnProgram {
                    bonding_period,
                    apr: AprUsdc {
                        uusdc: usdc_apr.apr,
                        ueden: eden_apr.apr
                    },
                    available: Some(available),
                    staked: Some(staked),
                    rewards: Some(pool_rewards),
                    borrowed: Some(borrowed),
                }
            }
            None => UsdcEarnProgram::default(),
        },
    };

    Ok(resp)
}
