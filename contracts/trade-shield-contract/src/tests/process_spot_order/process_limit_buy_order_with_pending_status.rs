use std::str::FromStr;

use crate::entry_point::{execute, query, sudo};
use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{
    coin, to_json_binary, Addr, BankMsg, BlockInfo, Decimal, Empty, SignedDecimal, Timestamp,
    Uint128,
};
use cw_multi_test::BankSudo;
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::query_resp::AmmSwapEstimationByDenomResponse;
use elys_bindings::trade_shield::msg::query_resp::GetSpotOrderResp;
use elys_bindings::trade_shield::msg::{QueryMsg, SudoMsg};
use elys_bindings::trade_shield::types::{OrderPrice, SpotOrder, SpotOrderType, Status};
use elys_bindings::types::{SwapAmountInRoute, SwapAmountOutRoute};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};

use super::InstantiateMockMsg;
use super::{instantiate, reply};

struct ElysModuleWrapper(ElysModule);

impl Module for ElysModuleWrapper {
    type QueryT = ElysQuery;
    type ExecT = ElysMsg;
    type SudoT = Empty;

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            ElysQuery::AmmSwapEstimationByDenom {
                amount,
                denom_in,
                denom_out,
                ..
            } => {
                let spot_price = match (denom_in.as_str(), denom_out.as_str()) {
                    (
                        "uelys",
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ) => Decimal::from_str("3.5").unwrap(),
                    _ => panic!(
                        "price not found for the pair of {} and {}",
                        denom_in, denom_out
                    ),
                };
                let amount_value = amount.amount * spot_price.clone();

                let (in_route, out_route) = if amount.denom == denom_in {
                    (
                        Some(vec![SwapAmountInRoute {
                            pool_id: 1,
                            token_out_denom: denom_out.clone(),
                        }]),
                        None,
                    )
                } else {
                    (
                        None,
                        Some(vec![SwapAmountOutRoute {
                            pool_id: 1,
                            token_in_denom: denom_in.clone(),
                        }]),
                    )
                };

                let resp = AmmSwapEstimationByDenomResponse {
                    in_route: in_route,
                    out_route: out_route,
                    spot_price,
                    amount: coin(amount_value.u128(), &denom_in),
                    swap_fee: SignedDecimal::zero(),
                    discount: SignedDecimal::zero(),
                    available_liquidity: coin(95841644452, &denom_in),
                    weight_balance_ratio: SignedDecimal::zero(),
                    price_impact: SignedDecimal::zero(),
                };

                Ok(to_json_binary(&resp)?)
            }
            _ => panic!("not implemented"),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        _sender: Addr,
        msg: Self::ExecT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        match msg {
            ElysMsg::AmmSwapExactAmountIn {
                sender,
                routes,
                token_in,
                recipient,
                ..
            } => {
                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&token_in)?),
                };

                let burn_msg = BankMsg::Burn {
                    amount: vec![token_in.clone()],
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(sender.clone()),
                        burn_msg.into(),
                    )
                    .unwrap();

                let price = match (token_in.denom.as_str(), routes[0].token_out_denom.as_str()) {
                    (
                        "uelys",
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ) => Decimal::from_atomics(Uint128::new(2), 0).unwrap(),
                    _ => panic!("price"),
                };

                let mint_amount: u128 = price
                    .checked_mul(Decimal::from_atomics(token_in.amount, 0).unwrap())
                    .unwrap()
                    .to_uint_floor()
                    .u128();

                let mint_msg = BankSudo::Mint {
                    to_address: recipient,
                    amount: vec![coin(mint_amount, &routes[0].token_out_denom)],
                };
                router.sudo(api, storage, block, mint_msg.into()).unwrap();

                Ok(resp)
            }
            _ => panic!("not implemented"),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("sudo is not implemented for ElysModule")
    }
}

#[test]
fn process_limit_buy_order_with_pending_status() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![
        ("user", vec![coin(90_000000, "uelys")]),
        ("owner", vec![coin(10_000000, "uelys")]),
    ];

    let mut addresses: Vec<String> = vec![];
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModuleWrapper(ElysModule {}))
        .build(|roouter, _, storage| {
            for (wallet_owner, wallet_contenent) in wallet {
                roouter
                    .bank
                    .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                    .unwrap();
                addresses.push(wallet_owner.to_owned())
            }
            ACCOUNT.save(storage, &addresses).unwrap();
            PERPETUAL_OPENED_POSITION.save(storage, &vec![]).unwrap();
            ASSET_INFO.save(storage, &vec![]).unwrap();
            PRICES.save(storage, &vec![]).unwrap();
            LAST_MODULE_USED.save(storage, &None).unwrap();
        });

    // trade shield deployment
    let code = ContractWrapper::new(execute, instantiate, query)
        .with_sudo(sudo)
        .with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    // Create a "limit buy" order with a specific rate and balance.
    let order = SpotOrder::new(
        0,
        SpotOrderType::LimitBuy,
        Some(OrderPrice {
            base_denom: "uelys".to_string(),
            quote_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
                .to_string(),
            rate: Decimal::from_atomics(Uint128::new(2), 0).unwrap(),
        }),
        coin(10_000000, "uelys"),
        Addr::unchecked("user"),
        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
        &BlockInfo {
            height: 50,
            time: Timestamp::from_seconds(600),
            chain_id: "elys-app".to_string(),
        },
    );

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![order],
        perpetual_orders: vec![],
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[coin(10_000000, "uelys")],
            "Contract",
            None,
        )
        .unwrap();

    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    let o: GetSpotOrderResp = app
        .wrap()
        .query_wasm_smart(addr, &QueryMsg::GetSpotOrder { order_id: 0 })
        .unwrap();

    assert_eq!(o.order.status, Status::Pending);
}
