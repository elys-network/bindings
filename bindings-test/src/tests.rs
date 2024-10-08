use cosmwasm_std::{
    coin, coins, Addr, Coin, Decimal, Int128, Int64, SignedDecimal, SignedDecimal256, StdError,
    Uint128,
};
use cw_multi_test::Executor;
use elys_bindings::{
    query_resp::{
        AmmSwapEstimationResponse, AuthAddressesResponse, OracleAssetInfoResponse,
        PerpetualMtpResponse, PerpetualQueryPositionsResponse,
    },
    types::{Mtp, OracleAssetInfo, PageRequest, PerpetualPosition, Price, SwapAmountInRoute},
    ElysMsg, ElysQuery,
};

use super::multitest::*;

fn check_prices(app: &mut ElysApp, prices: &Vec<Price>) {
    let dummy_req = PageRequest::new(20);

    let prices = prices.to_owned();
    let request = ElysQuery::oracle_get_all_prices(dummy_req).into();
    let actual_prices: Vec<Price> = app.wrap().query(&request).unwrap();
    assert_eq!(prices, actual_prices);
}

#[test]
fn query_price() {
    let mut prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    check_prices(&mut app, &prices);

    let new_price = Price::new("eth", Decimal::from_atomics(Uint128::new(1700), 0).unwrap());
    app.init_modules(|router, _, storage| router.custom.new_price(storage, &new_price))
        .unwrap();
    prices.push(new_price);

    check_prices(&mut app, &prices);

    let new_price = Price::new("eth", Decimal::from_atomics(Uint128::new(1700), 0).unwrap());
    app.init_modules(|router, _, storage| router.custom.new_price(storage, &new_price))
        .unwrap();
    prices[2].price = new_price.price;
    check_prices(&mut app, &prices);
}

#[test]
fn amm_swap_estimation() {
    let prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    let routes = vec![SwapAmountInRoute {
        pool_id: 1,
        token_out_denom: "usdc".to_string(),
    }];

    let swap: AmmSwapEstimationResponse = app
        .wrap()
        .query(&cosmwasm_std::QueryRequest::Custom(
            ElysQuery::amm_swap_estimation(routes, coin(5, "btc"), Decimal::zero()),
        ))
        .unwrap();

    assert_eq!(
        swap.spot_price,
        Decimal::from_atomics(Uint128::new(20000), 0).unwrap()
    );

    assert_eq!(swap.token_out, coin(100000, "usdc"));
}

#[test]
fn asset_info() {
    let infos: Vec<OracleAssetInfo> = vec![OracleAssetInfo {
        denom: "uatom".to_string(),
        display: "ATOM".to_string(),
        band_ticker: "ATOM".to_string(),
        elys_ticker: "ATOM".to_string(),
        decimal: 6,
    }];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_asset_infos(storage, &infos))
        .unwrap();

    let req = ElysQuery::oracle_asset_info("uatom".to_string()).into();

    let queried_infos: OracleAssetInfoResponse = app.wrap().query(&req).unwrap();

    assert_eq!(infos[0], queried_infos.asset_info);
}

#[test]
fn asset_info_not_found() {
    let app = ElysApp::new();

    let req = ElysQuery::oracle_asset_info("uatom".to_string()).into();

    let err: StdError = app.wrap().query::<OracleAssetInfo>(&req).unwrap_err();

    let error_reference = StdError::GenericErr {
        msg: format!(
            "Querier contract error: {}",
            StdError::not_found("asset denom").to_string()
        ),
    };

    assert_eq!(err, error_reference);
}
#[test]
fn query_positions() {
    let mtps: Vec<Mtp> = vec![Mtp {
        address: "user".to_string(),
        position: 1,
        id: 0,
        amm_pool_id: 0,
        mtp_health: SignedDecimal::zero(),
        consolidate_leverage: SignedDecimal::zero(),
        sum_collateral: Int128::zero(),
        take_profit_price: SignedDecimal256::zero(),
        take_profit_liabilities: Int128::zero(),
        liabilities: Int128::zero(),
        borrow_interest_paid_collateral: Int128::zero(),
        borrow_interest_paid_custody: Int128::zero(),
        borrow_interest_unpaid_collateral: Int128::zero(),
        collateral_asset: "".to_string(),
        collateral: Int128::zero(),
        custody: Int128::zero(),
        custody_asset: "".to_string(),
        funding_fee_paid_collateral: Int128::zero(),
        funding_fee_paid_custody: Int128::zero(),
        funding_fee_received_collateral: Int128::zero(),
        funding_fee_received_custody: Int128::zero(),
        leverage: SignedDecimal::zero(),
        liabilities_asset: "".to_string(),
        open_price: SignedDecimal::zero(),
        take_profit_borrow_rate: SignedDecimal::zero(),
        take_profit_custody: Int128::zero(),
        trading_asset: "".to_string(),
        stop_loss_price: SignedDecimal::zero(),
        last_interest_calc_time: None,
        last_interest_calc_block: None,
        last_funding_calc_time: None,
        last_funding_calc_block: None,
    }];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::positions(PageRequest::new(5)).into();

    let mtps_found: PerpetualQueryPositionsResponse = app.wrap().query(&req).unwrap();

    let u_mtps = mtps_found
        .mtps
        .unwrap()
        .iter()
        .map(|v| v.get_mtp())
        .collect::<Vec<_>>();

    assert_eq!(u_mtps, mtps);
}

#[test]
fn query_single_mtp() {
    let mtps: Vec<Mtp> = vec![Mtp {
        address: "user".to_string(),
        take_profit_liabilities: Int128::zero(),
        liabilities: Int128::zero(),
        mtp_health: SignedDecimal::zero(),
        position: 1,
        id: 0,
        amm_pool_id: 0,
        consolidate_leverage: SignedDecimal::zero(),
        sum_collateral: Int128::zero(),
        take_profit_price: SignedDecimal256::zero(),
        borrow_interest_paid_collateral: Int128::zero(),
        borrow_interest_paid_custody: Int128::zero(),
        borrow_interest_unpaid_collateral: Int128::zero(),
        collateral_asset: "".to_string(),
        collateral: Int128::zero(),
        custody: Int128::zero(),
        custody_asset: "".to_string(),
        funding_fee_paid_collateral: Int128::zero(),
        funding_fee_paid_custody: Int128::zero(),
        funding_fee_received_collateral: Int128::zero(),
        funding_fee_received_custody: Int128::zero(),
        leverage: SignedDecimal::zero(),
        liabilities_asset: "".to_string(),
        open_price: SignedDecimal::zero(),
        take_profit_borrow_rate: SignedDecimal::zero(),
        take_profit_custody: Int128::zero(),
        trading_asset: "".to_string(),
        stop_loss_price: SignedDecimal::zero(),
        last_interest_calc_time: None,
        last_interest_calc_block: None,
        last_funding_calc_time: None,
        last_funding_calc_block: None,
    }];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::mtp("user", 0).into();

    let mtp_found: PerpetualMtpResponse = app.wrap().query(&req).unwrap();

    assert_eq!(mtps[0], mtp_found.mtp.unwrap().mtp);
}

#[test]
fn query_mtp_not_found() {
    let mtps: Vec<Mtp> = vec![];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::mtp("user", 0).into();

    let err = app.wrap().query::<PerpetualMtpResponse>(&req).unwrap_err();

    let not_found_err = StdError::not_found("perpetual trading position");

    let err_ref = StdError::generic_err(format!(
        "Querier contract error: {}",
        not_found_err.to_string()
    ));

    assert_eq!(err, err_ref);
}

#[test]
fn swap() {
    let prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);
    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();
    let routes = vec![SwapAmountInRoute {
        pool_id: 1,
        token_out_denom: "usdc".to_string(),
    }];

    let msg = ElysMsg::amm_swap_exact_amount_in(
        "user",
        &coin(5, "btc"),
        &routes,
        Int128::zero(),
        Decimal::zero(),
        "user",
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    app.execute(Addr::unchecked("user"), msg.into()).unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        5 * 20000
    );
}

#[test]
fn swap_error() {
    let prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);
    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();
    let routes = vec![SwapAmountInRoute {
        pool_id: 1,
        token_out_denom: "usdc".to_string(),
    }];

    let msg = ElysMsg::amm_swap_exact_amount_in(
        "user",
        &coin(5, "btc"),
        &routes,
        Int128::new(100002),
        Decimal::zero(),
        "user",
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let err = app
        .execute(Addr::unchecked("user"), msg.into())
        .unwrap_err();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );
    assert_eq!(
        app.wrap()
            .query_balance("user", "usdc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    assert_eq!(
        StdError::generic_err("not enough token"),
        err.downcast().unwrap()
    );
}

#[test]
fn open_perpetual_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("contract_addr", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let open_msg = ElysMsg::perpetual_open_position(
        "contract_addr",
        coin(5, "btc"),
        "uusdc",
        PerpetualPosition::Short,
        SignedDecimal::from_atomics(Int64::new(25), 1).unwrap(),
        Some(SignedDecimal256::from_atomics(Uint128::new(11), 1).unwrap()),
        "user",
    );

    assert_eq!(
        app.wrap()
            .query_balance("contract_addr", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("contract_addr", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "PerpetualOpen");
}

#[test]
fn perpetual_perpetual_close_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("contract_addr", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let open_msg = ElysMsg::perpetual_open_position(
        "contract_addr",
        coin(5, "btc"),
        "uusdc",
        PerpetualPosition::Short,
        SignedDecimal::from_atomics(Int64::new(5), 0).unwrap(),
        Some(SignedDecimal256::from_atomics(Uint128::new(11), 1).unwrap()),
        "user",
    );

    assert_eq!(
        app.wrap()
            .query_balance("contract_addr", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("contract_addr", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "PerpetualOpen");

    let close_msg = ElysMsg::perpetual_close_position("contract_addr", 0, 25, "user");

    app.execute(Addr::unchecked("user"), close_msg.into())
        .unwrap();

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "PerpetualClose");
}
#[test]
fn auth_account() {
    let wallets: Vec<(&str, Vec<Coin>)> =
        vec![("user", coins(5, "btc")), ("user2", coins(1, "usdc"))];
    let app = ElysApp::new_with_wallets(wallets.clone());
    let req = ElysQuery::AuthAddresses { pagination: None }.into();
    let resp: AuthAddressesResponse = app.wrap().query(&req).unwrap();

    assert_eq!(resp.addresses[0], wallets[0].0);
    assert_eq!(resp.addresses[1], wallets[1].0);
}
