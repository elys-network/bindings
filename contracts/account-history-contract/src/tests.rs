use std::str::FromStr;

use crate::msg::{InstantiateMsg, QueryMsg};
use crate::types::AccountValue;
use crate::{entry_point::*, msg::ExecuteMsg};
use cosmwasm_std::{coin, coins, Addr, Coin, Decimal};
use cw_multi_test::{ContractWrapper, Executor};
use cw_utils::Expiration;
use elys_bindings::types::{Price, SwapAmountInRoute};
use elys_bindings_test::ElysApp;

#[test]
fn history() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![
        ("user-a", coins(300, "uelys")),
        ("user-b", coins(4, "uelys")),
        ("user-c", coins(45, "uelys")),
    ];

    let prices: Vec<Price> = vec![
        Price::new("uelys", Decimal::from_str("1.5").unwrap()),
        Price::new("uusdc", Decimal::from_str("1.0").unwrap()),
    ];

    let mut app = ElysApp::new_with_wallets(wallets.clone());
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices))
        .unwrap();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let init_msg = InstantiateMsg {
        limit: 1,
        expiration: Expiration::AtHeight(2),
        amm_routes: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uusdc".to_string(),
        }],
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &init_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let update_msg = ExecuteMsg::UpdateAccounts {};

    app.execute_contract(Addr::unchecked("xclock"), addr.clone(), &update_msg, &[])
        .unwrap();

    let query_msg = QueryMsg::UserHistory {
        user_address: "user-a".to_string(),
    };

    let res: Vec<AccountValue> = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(res[0].elys_amount.u128(), 300);
    assert_eq!(res[0].elys_value, coin(450, "uusdc"));
}
