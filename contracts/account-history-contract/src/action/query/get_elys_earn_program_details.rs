use super::*;
use crate::msg::query_resp::earn::GetElysEarnProgramResp;
use cosmwasm_std::{Decimal, Deps};
use elys_bindings::{
    account_history::types::{earn_program::ElysEarnProgram, AprElys, ElysDenom},
    query_resp::QueryAprResponse,
    ElysQuerier, ElysQuery,
};

pub fn get_elys_earn_program_details(
    deps: &Deps<ElysQuery>,
    address: Option<String>,
    asset: String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_apr: QueryAprResponse,
    eden_apr: QueryAprResponse,
    edenb_apr: QueryAprResponse,
) -> Result<GetElysEarnProgramResp, ContractError> {
    let bonding_period = 14;
    let denom = ElysDenom::Elys.as_str();
    if asset != denom.to_string() {
        return Err(ContractError::AssetDenomError {});
    }

    let querier = ElysQuerier::new(&deps.querier);

    let mut data = ElysEarnProgram::default();

    data.bonding_period = bonding_period;
    data.apr = AprElys {
        uusdc: usdc_apr.apr,
        ueden: eden_apr.apr,
        uedenb: edenb_apr.apr,
    };

    if let Some(addr) = address {
        let all_rewards = querier
            .get_estaking_rewards(addr.clone())
            .unwrap_or_default();
        let program_rewards = all_rewards
            .get_elys_validators()
            .to_coin_values(&querier)
            .unwrap_or_default()
            .into_iter()
            .map(|coin| coin.1)
            .collect();

        let mut available = querier.get_balance(addr.clone(), asset.clone())?;
        let staked = querier.get_staked_balance(addr.clone(), asset)?;

        let staked_positions = querier.get_staked_positions(addr.clone())?;
        let unstaked_positions = querier.get_unstaked_positions(addr)?;

        // have value in usd
        let mut available_in_usd = uelys_price_in_uusdc
            .checked_mul(Decimal::from_atomics(available.amount, 0).unwrap_or_default())
            .unwrap_or_default();
        available_in_usd = available_in_usd
            .checked_mul(uusdc_usd_price)
            .unwrap_or_default();
        available.usd_amount = available_in_usd;

        data.available = Some(available);
        data.staked = Some(staked);
        data.rewards = Some(program_rewards);
        data.staked_positions = staked_positions.staked_position;
        data.unstaked_positions = unstaked_positions.unstaked_position;
    }

    Ok(GetElysEarnProgramResp { data })
}
