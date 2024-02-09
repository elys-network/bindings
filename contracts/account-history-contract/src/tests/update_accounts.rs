use std::str::FromStr;

use crate::msg::query_resp::UserValueResponse;
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::{entry_point::*, msg::SudoMsg};
use cosmwasm_std::{coins, Addr, Coin, DecCoin, Decimal, Decimal256, Uint128};
use cw_multi_test::{BankSudo, ContractWrapper, Executor, SudoMsg as AppSudo};
use cw_utils::Expiration;
use elys_bindings::types::{OracleAssetInfo, Price};
use elys_bindings_test::ElysApp;
use trade_shield_contract::entry_point::{
    execute as trade_shield_execute, instantiate as trade_shield_init, query as trade_shield_query,
};
use trade_shield_contract::msg::InstantiateMsg as TradeShieldInstantiateMsg;

#[test]
fn history() {
    let wallets: Vec<(&str, Vec<Coin>)> = vec![("user-a", coins(300, "uelys"))];

    let prices: Vec<Price> = vec![
        Price::new("uelys", Decimal::from_str("1.5").unwrap()),
        Price::new(
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
            Decimal::from_str("1.0").unwrap(),
        ),
    ];

    let infos = vec![
        OracleAssetInfo::new(
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
            "".to_string(),
            "".to_string(),
            2,
        ),
        OracleAssetInfo::new(
            "uelys".to_string(),
            "UELYS".to_string(),
            "".to_string(),
            "".to_string(),
            2,
        ),
        OracleAssetInfo::new(
            "ueden".to_string(),
            "UEDEN".to_string(),
            "".to_string(),
            "".to_string(),
            2,
        ),
    ];

    let mut app = ElysApp::new_with_wallets(wallets.clone());
    app.init_modules(|router, _, store| {
        router.custom.set_prices(store, &prices).unwrap();

        router.custom.set_asset_infos(store, &infos)
    })
    .unwrap();

    let history_code = ContractWrapper::new(execute, instantiate, query).with_sudo(sudo);
    let history_code_id = app.store_code(Box::new(history_code));

    let trade_shield_code =
        ContractWrapper::new(trade_shield_execute, trade_shield_init, trade_shield_query);

    let trade_shield_code_id = app.store_code(Box::new(trade_shield_code));

    let trade_shield_init = TradeShieldInstantiateMsg {
        account_history_address: None,
    };

    let trade_shield_address = app
        .instantiate_contract(
            trade_shield_code_id,
            Addr::unchecked("owner"),
            &trade_shield_init,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    let init_msg = InstantiateMsg {
        limit: Some(2),
        expiration: Some(Expiration::AtHeight(2)),
        trade_shield_address: Some(trade_shield_address.to_string()),
    };

    let addr = app
        .instantiate_contract(
            history_code_id,
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

    assert_eq!(
        res.value.liquid_asset.total_liquid_asset_balance,
        DecCoin::new(
            Decimal256::from_atomics(Uint128::new(45), 1).unwrap(),
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
        )
    );

    app.sudo(AppSudo::Bank(BankSudo::Mint {
        to_address: "user-a".to_string(),
        amount: coins(200, "uelys"),
    }))
    .unwrap();

    app.wasm_sudo(addr.clone(), &update_msg).unwrap();
    app.next_block();

    let res: UserValueResponse = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

    assert_eq!(
        res.value.liquid_asset.total_liquid_asset_balance,
        DecCoin::new(
            Decimal256::from_atomics(Uint128::new(75), 1).unwrap(),
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
        )
    ); // The previous value wasn't removed yet but wasn't read either since it's expired.
}
