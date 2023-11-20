use std::str::FromStr;

use cosmwasm_std::{coin, coins, Addr, Coin, Decimal, Int128, StdError, Uint128};
use cw_multi_test::Executor;
use elys_bindings::{
    query_resp::{
        AmmSwapEstimationResponse, AuthAccountsResponse, MarginMtpResponse,
        MarginQueryPositionsResponse,
    },
    types::{MarginPosition, Mtp, OracleAssetInfo, PageRequest, Price, SwapAmountInRoute},
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
            ElysQuery::amm_swap_estimation(routes, coin(5, "btc")),
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

    let queried_infos: OracleAssetInfo = app.wrap().query(&req).unwrap();

    assert_eq!(infos[0], queried_infos);
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
        collaterals: vec![],
        liabilities: Int128::zero(),
        interest_paid_collaterals: vec![],
        interest_paid_custodies: vec![],
        interest_unpaid_collaterals: vec![],
        custodies: vec![],
        take_profit_liabilities: Int128::zero(),
        take_profit_custodies: vec![],
        leverages: vec![],
        mtp_health: Decimal::zero(),
        position: 1,
        id: 0,
        amm_pool_id: 0,
        consolidate_leverage: Decimal::zero(),
        sum_collateral: Int128::zero(),
        take_profit_price: Decimal::zero(),
    }];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::positions(PageRequest::new(5)).into();

    let mtps_found: MarginQueryPositionsResponse = app.wrap().query(&req).unwrap();

    assert_eq!(mtps_found.mtps.unwrap(), mtps);
}

#[test]
fn query_single_mtp() {
    let mtps: Vec<Mtp> = vec![Mtp {
        address: "user".to_string(),
        collaterals: vec![],
        liabilities: Int128::zero(),
        interest_paid_collaterals: vec![],
        interest_paid_custodies: vec![],
        interest_unpaid_collaterals: vec![],
        custodies: vec![],
        take_profit_liabilities: Int128::zero(),
        take_profit_custodies: vec![],
        leverages: vec![],
        mtp_health: Decimal::zero(),
        position: 1,
        id: 0,
        amm_pool_id: 0,
        consolidate_leverage: Decimal::zero(),
        sum_collateral: Int128::zero(),
        take_profit_price: Decimal::zero(),
    }];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::mtp("user", 0).into();

    let mtp_found: MarginMtpResponse = app.wrap().query(&req).unwrap();

    assert_eq!(mtps[0], mtp_found.mtp.unwrap());
}

#[test]
fn query_mtp_not_found() {
    let mtps: Vec<Mtp> = vec![];
    let mut app = ElysApp::new();

    app.init_modules(|router, _, storage| router.custom.set_mtp(storage, &mtps))
        .unwrap();

    let req = ElysQuery::mtp("user", 0).into();

    let err = app.wrap().query::<MarginMtpResponse>(&req).unwrap_err();

    let not_found_err = StdError::not_found("margin trading position");

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

    let msg = ElysMsg::amm_swap_exact_amount_in("user", &coin(5, "btc"), &routes, Int128::zero());

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

    let msg =
        ElysMsg::amm_swap_exact_amount_in("user", &coin(5, "btc"), &routes, Int128::new(100002));

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
fn open_margin_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let open_msg = ElysMsg::margin_open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Unspecified,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MarginOpen");
}

#[test]
fn margin_margin_close_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let open_msg = ElysMsg::margin_open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Unspecified,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        5
    );

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();

    assert_eq!(
        app.wrap()
            .query_balance("user", "btc")
            .unwrap()
            .amount
            .u128(),
        0
    );

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MarginOpen");

    let close_msg = ElysMsg::margin_close_position("user", 0);

    app.execute(Addr::unchecked("user"), close_msg.into())
        .unwrap();

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MarginClose");
}
#[test]
fn auth_account() {
    let wallets: Vec<(&str, Vec<Coin>)> =
        vec![("user", coins(5, "btc")), ("user2", coins(1, "usdc"))];
    let app = ElysApp::new_with_wallets(wallets.clone());
    let req = ElysQuery::AuthAccounts {
        pagination: PageRequest::new(200),
    }
    .into();
    let resp: AuthAccountsResponse = app.wrap().query(&req).unwrap();

    assert_eq!(resp.accounts[0].address, wallets[0].0);
    assert_eq!(resp.accounts[1].address, wallets[1].0);
}
#[test]
fn margin_broker_open() {
    let mut app = ElysApp::new();

    let req = ElysMsg::margin_broker_open_position(
        "user",
        "btc",
        Int128::new(2),
        "usdc",
        MarginPosition::Short as i32,
        Decimal::from_str("5.0").unwrap(),
        Decimal::from_str("2.2").unwrap(),
        "owner",
    )
    .into();

    app.execute(Addr::unchecked("user"), req).unwrap();

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MarginBrokerOpen");
}

#[test]
fn margin_broker_close() {
    let mut app = ElysApp::new();

    let req = ElysMsg::margin_broker_close_position("user", 0, "owner").into();

    app.execute(Addr::unchecked("user"), req).unwrap();

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MarginBrokerClose");
}
