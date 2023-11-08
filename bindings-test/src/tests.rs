use cosmwasm_std::{coin, coins, from_binary, Addr, Coin, Decimal, Int128, StdError, Uint128};
use cw_multi_test::Executor;
use elys_bindings::{
    msg_resp::MsgCloseResponse,
    query_resp::QuerySwapEstimationResponse,
    types::{AssetInfo, MarginPosition, PageRequest, Price, SwapAmountInRoute},
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
    let infos: Vec<AssetInfo> = vec![AssetInfo {
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

    let queried_infos: AssetInfo = app.wrap().query(&req).unwrap();

    assert_eq!(infos[0], queried_infos);
}

#[test]
fn asset_info_not_found() {
    let app = ElysApp::new();

    let req = ElysQuery::Oracle(OracleQuery::asset_info("uatom".to_string())).into();

    let err: StdError = app.wrap().query::<AssetInfo>(&req).unwrap_err();

    let error_reference = StdError::GenericErr {
        msg: format!(
            "Querier contract error: {}",
            StdError::not_found("asset denom").to_string()
        ),
    };

    assert_eq!(err, error_reference);
}

#[test]
fn open_position() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user", coins(5, "btc"))];
    let mut app = ElysApp::new_with_wallets(wallets);
    let prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

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
fn margin_close_not_found() {
    let id: u64 = 1;
    let creator = "user".to_string();
    let mut app = ElysApp::default();

    let msg = ElysMsg::Margin(MarginMsg::close_position(creator, id, None));

    let err = app
        .execute(Addr::unchecked("user"), msg.into())
        .unwrap_err();

    assert_eq!(
        StdError::not_found(format!("{id:?}")),
        err.downcast().unwrap()
    );
}

#[test]
fn margin_close_unauthorize() {
    let prices: Vec<Price> = vec![
        Price::new(
            "btc",
            Decimal::from_atomics(Uint128::new(20000), 0).unwrap(),
        ),
        Price::new("usdc", Decimal::from_atomics(Uint128::new(1), 0).unwrap()),
    ];

    let wallets: Vec<(&str, Vec<Coin>)> = vec![("order_owner", coins(5, "btc"))];

    let mut app = ElysApp::new_with_wallets(wallets);
    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

    let open_msg = ElysMsg::Margin(MarginMsg::open_position(
        "order_owner",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Unspecified,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
        None,
    ));

    app.execute(Addr::unchecked("order_owner"), open_msg.into())
        .unwrap();

    let close_msg = ElysMsg::Margin(MarginMsg::close_position(
        "order_owner".to_string(),
        0,
        None,
    ));

    let err = app
        .execute(Addr::unchecked("random"), close_msg.into())
        .unwrap_err();

    assert_eq!(
        StdError::generic_err("Unauthtorized"),
        err.downcast().unwrap()
    );
}

#[test]
fn margin_close_long_win() {
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

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();
}

#[test]
fn margin_close_long_lose() {
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

    app.init_modules(|router, _, storage| router.custom.set_prices(storage, &prices))
        .unwrap();

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

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();
}

#[test]
fn margin_close_short_win() {
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

    let open_msg = ElysMsg::Margin(MarginMsg::open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Short,
        Decimal::from_atomics(Uint128::new(3), 0).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
        None,
    ));

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();

    app.init_modules(|router, _, storage| {
        router.custom.new_price(
            storage,
            &Price::new(
                "btc",
                Decimal::from_atomics(Uint128::new(15000), 0).unwrap(),
            ),
        )
    })
    .unwrap();

    let close_msg = ElysMsg::Margin(MarginMsg::close_position("user", 0, None));

    let resp: MsgCloseResponse = from_binary(
        &app.execute(Addr::unchecked("user"), close_msg.into())
            .unwrap()
            .data
            .unwrap(),
    )
    .unwrap();

    assert_eq!(resp.id, 0);

    panic!(
        "{:?}",
        app.wrap().query_all_balances(Addr::unchecked("user"))
    );
}

#[test]
fn margin_close_short_lose() {
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

    let open_msg = ElysMsg::Margin(MarginMsg::open_position(
        "user",
        "btc",
        Int128::new(5),
        "btc",
        MarginPosition::Short,
        Decimal::from_atomics(Uint128::new(25), 1).unwrap(),
        Decimal::from_atomics(Uint128::new(11), 1).unwrap(),
        None,
    ));

    app.execute(Addr::unchecked("user"), open_msg.into())
        .unwrap();
}
