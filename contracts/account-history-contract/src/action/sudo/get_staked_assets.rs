use cosmwasm_std::{DecCoin, Decimal, Decimal256, DepsMut};
use elys_bindings::{query_resp::QueryAprResponse, ElysQuery};

use crate::{
    action::query::{
        get_eden_boost_earn_program_details, get_eden_earn_program_details,
        get_elys_earn_program_details, get_usdc_earn_program_details,
    },
    msg::query_resp::StakedAssetsResponse,
    types::{
        earn_program::{EdenBoostEarnProgram, EdenEarnProgram, ElysEarnProgram, UsdcEarnProgram},
        ElysDenom, StakedAssets,
    },
};

pub fn get_staked_assets(
    deps: &DepsMut<ElysQuery>,
    address: &String,
    uusdc_usd_price: Decimal,
    uelys_price_in_uusdc: Decimal,
    usdc_denom: String,
    usdc_base_denom: String,
    eden_decimal: u64,
    usdc_apr_usdc: QueryAprResponse,
    eden_apr_usdc: QueryAprResponse,
    usdc_apr_edenb: QueryAprResponse,
    eden_apr_edenb: QueryAprResponse,
    usdc_apr_eden: QueryAprResponse,
    eden_apr_eden: QueryAprResponse,
    edenb_apr_eden: QueryAprResponse,
    usdc_apr_elys: QueryAprResponse,
    eden_apr_elys: QueryAprResponse,
    edenb_apr_elys: QueryAprResponse,
) -> StakedAssetsResponse {
    // create staked_assets variable that is a StakedAssets struct
    let mut staked_assets = StakedAssets::default();
    let mut total_balance = Decimal::zero();

    let usdc_details = get_usdc_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Usdc.as_str().to_string(),
        usdc_denom.to_owned(),
        usdc_base_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_usdc,
        eden_apr_usdc,
    )
    .unwrap();
    // usdc program
    let staked_asset_usdc = usdc_details.data.clone();
    total_balance = total_balance
        .checked_add(match staked_asset_usdc.clone() {
            UsdcEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.usdc_earn_program = staked_asset_usdc;

    // elys program
    let elys_details = get_elys_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Elys.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_elys,
        eden_apr_elys,
        edenb_apr_elys,
    )
    .unwrap();
    let staked_asset_elys = elys_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_elys.clone() {
            ElysEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.elys_earn_program = staked_asset_elys;

    // eden program
    let eden_details = get_eden_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::Eden.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        usdc_apr_eden,
        eden_apr_eden,
        edenb_apr_eden,
    )
    .unwrap();
    let staked_asset_eden = eden_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_eden.clone() {
            EdenEarnProgram {
                staked: Some(r), ..
            } => r.usd_amount,
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.eden_earn_program = staked_asset_eden;

    let edenb_details = get_eden_boost_earn_program_details(
        deps,
        Some(address.to_owned()),
        ElysDenom::EdenBoost.as_str().to_string(),
        usdc_denom.to_owned(),
        uusdc_usd_price,
        uelys_price_in_uusdc,
        eden_decimal,
        usdc_apr_edenb,
        eden_apr_edenb,
    )
    .unwrap();
    let staked_asset_edenb = edenb_details.data;
    total_balance = total_balance
        .checked_add(match staked_asset_edenb.clone() {
            EdenBoostEarnProgram {
                rewards: Some(r), ..
            } => r.iter().fold(Decimal::zero(), |acc, item| {
                acc.checked_add(item.usd_amount.unwrap()).unwrap()
            }),
            _ => Decimal::zero(),
        })
        .unwrap();
    staked_assets.eden_boost_earn_program = staked_asset_edenb;

    StakedAssetsResponse {
        staked_assets,
        total_staked_balance: DecCoin::new(Decimal256::from(total_balance), usdc_denom),
    }
}
