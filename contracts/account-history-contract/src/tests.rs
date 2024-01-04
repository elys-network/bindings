use std::str::FromStr;

use crate::msg::query_resp::UserValueResponse;
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::{entry_point::*, msg::SudoMsg};
use cosmwasm_std::{coin, coins, Addr, Coin, Decimal};
use cw_multi_test::{BankSudo, ContractWrapper, Executor, SudoMsg as AppSudo};
use cw_utils::Expiration;
use elys_bindings::types::Price;
use elys_bindings_test::ElysApp;

#[test]
fn history() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user-a", coins(300, "uelys"))];

    let prices: Vec<Price> = vec![
        Price::new("uelys", Decimal::from_str("1.5").unwrap()),
        Price::new("uusdc", Decimal::from_str("1.0").unwrap()),
    ];

    let mut app = ElysApp::new_with_wallets(wallets.clone());
    app.init_modules(|router, _, store| router.custom.set_prices(store, &prices))
        .unwrap();

    let code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let code_id = app.store_code(Box::new(code));

    let init_msg = InstantiateMsg {
        limit: 2,
        expiration: Expiration::AtHeight(2),
        value_denom: "usdc".to_string(),
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

    let update_msg = SudoMsg::ClockEndBlock {};

    app.wasm_sudo(addr.clone(), &update_msg).unwrap();

    let query_msg = QueryMsg::UserValue {
        user_address: "user-a".to_string(),
    };

    app.next_block();

    let res: UserValueResponse = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(res.value.account_value, coin(450, "uusdc"));

    app.sudo(AppSudo::Bank(BankSudo::Mint {
        to_address: "user-a".to_string(),
        amount: coins(200, "uelys"),
    }))
    .unwrap();

    app.wasm_sudo(addr.clone(), &update_msg).unwrap();
    app.next_block();

    let res: UserValueResponse = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(res.value.account_value, coin(750, "uusdc")); // The previous value wasn't removed yet but wasn't read either since it's expired.
}
