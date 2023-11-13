use cosmwasm_std::{coin, coins, Addr, Coin, Decimal, Int128, StdError, Uint128};
use cw_multi_test::Executor;
use elys_bindings::{
    query_resp::QuerySwapEstimationResponse,
    types::{MarginPosition, OracleAssetInfo, PageRequest, Price, SwapAmountInRoute},
    AmmMsg, AmmQuery, ElysMsg, ElysQuery, MarginMsg, OracleQuery,
};

use super::multitest::*;

fn check_prices(app: &mut ElysApp, prices: &Vec<Price>) {
    let dummy_req = PageRequest::new(20);

    let prices = prices.to_owned();
    let request = ElysQuery::Oracle(OracleQuery::get_all_prices(dummy_req)).into();
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
fn swap_estimation() {
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

    let swap: QuerySwapEstimationResponse = app
        .wrap()
        .query(&cosmwasm_std::QueryRequest::Custom(ElysQuery::Amm(
            AmmQuery::swap_estimation(routes, coin(5, "btc")),
        )))
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

    let req = ElysQuery::Oracle(OracleQuery::asset_info("uatom".to_string())).into();

    let queried_infos: OracleAssetInfo = app.wrap().query(&req).unwrap();

    assert_eq!(infos[0], queried_infos);
}

#[test]
fn asset_info_not_found() {
    let app = ElysApp::new();

    let req = ElysQuery::Oracle(OracleQuery::asset_info("uatom".to_string())).into();

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

    let msg = ElysMsg::Amm(AmmMsg::swap_exact_amount_in(
        "user",
        &coin(5, "btc"),
        &routes,
        Int128::zero(),
        None,
    ));

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

    let msg = ElysMsg::Amm(AmmMsg::swap_exact_amount_in(
        "user",
        &coin(5, "btc"),
        &routes,
        Int128::new(100002),
        None,
    ));

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

    let open_msg = ElysMsg::Margin(MarginMsg::open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Unspecified,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
        None,
    ));

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

    assert_eq!(last_module_used, "MsgOpen");
}

#[test]
fn margin_close_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);

    let open_msg = ElysMsg::Margin(MarginMsg::open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Unspecified,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
        None,
    ));

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

    assert_eq!(last_module_used, "MsgOpen");

    let close_msg = ElysMsg::Margin(MarginMsg::close_position("user", 0, None));

    app.execute(Addr::unchecked("user"), close_msg.into())
        .unwrap();

    let last_module_used = app
        .init_modules(|router, _, store| router.custom.get_last_module(store))
        .unwrap()
        .unwrap();

    assert_eq!(last_module_used, "MsgClose");
}
