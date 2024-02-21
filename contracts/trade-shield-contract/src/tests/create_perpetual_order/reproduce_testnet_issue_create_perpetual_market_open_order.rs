use anyhow::{bail, Result as AnyResult};

use cosmwasm_std::{to_json_binary, Addr, BankMsg, Empty, Int128, Int64, SignedDecimal, StdError};
use cw_multi_test::{AppResponse, BasicAppBuilder, Module};
use elys_bindings::{
    msg_resp::PerpetualOpenResponse,
    query_resp::{Entry, PerpetualOpenEstimationRawResponse, QueryGetEntryResponse},
    ElysMsg, ElysQuery,
};

use super::*;

struct ElysModule {}

impl Module for ElysModule {
    type ExecT = ElysMsg;
    type QueryT = ElysQuery;
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
            ElysQuery::AssetProfileEntry { base_denom } => {
                let resp = match base_denom.as_str() {
                    "uusdc" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "uusdc".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
                            display_name: "USDC".to_string(),
                            display_symbol: "uUSDC".to_string(),
                            external_symbol: "uUSDC".to_string(),
                            ibc_channel_id: "channel-12".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "channel-19".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "transfer/channel-12".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "uusdc".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    "ueden" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "ueden".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "ueden".to_string(),
                            display_name: "EDEN".to_string(),
                            display_symbol: "".to_string(),
                            external_symbol: "".to_string(),
                            ibc_channel_id: "".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    "uelys" => QueryGetEntryResponse {
                        entry: Entry {
                            address: "".to_string(),
                            authority: "elys10d07y265gmmuvt4z0w9aw880jnsr700j6z2zm3".to_string(),
                            base_denom: "uelys".to_string(),
                            commit_enabled: true,
                            decimals: 6,
                            denom: "uelys".to_string(),
                            display_name: "ELYS".to_string(),
                            display_symbol: "".to_string(),
                            external_symbol: "".to_string(),
                            ibc_channel_id: "".to_string(),
                            ibc_counterparty_chain_id: "".to_string(),
                            ibc_counterparty_channel_id: "".to_string(),
                            ibc_counterparty_denom: "".to_string(),
                            network: "".to_string(),
                            path: "".to_string(),
                            permissions: vec![],
                            transfer_limit: "".to_string(),
                            unit_denom: "".to_string(),
                            withdraw_enabled: true,
                        },
                    },
                    _ => return Err(StdError::not_found(base_denom).into()),
                };
                Ok(to_json_binary(&resp)?)
            }
            ElysQuery::PerpetualOpenEstimation {
                position,
                leverage,
                trading_asset,
                collateral,
                take_profit_price,
                discount,
            } => {
                let resp = PerpetualOpenEstimationRawResponse {
                    position,
                    leverage: leverage.clone().to_string(),
                    trading_asset: trading_asset.clone(),
                    collateral: collateral.clone(),
                    min_collateral: coin(
                        8333333,
                        "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
                    ),
                    valid_collateral: Some(true),
                    position_size: collateral.clone(),
                    swap_fee: Decimal::zero().to_string(),
                    discount: discount.clone().to_string(),
                    open_price: Decimal::from_atomics(Uint128::new(9_440_848_026_817_446_325), 18)
                        .unwrap().to_string(),
                    take_profit_price: take_profit_price.clone().to_string(),
                    liquidation_price: Decimal::from_atomics(
                        Uint128::new(9_240_848_026_817_446_325),
                        18,
                    )
                    .unwrap().to_string(),
                    estimated_pnl: Int128::from_str(
                        // "4_999_999_999_999_999_999_999_999_999_999_999_999_527_957_598_6",
                        "4999999999999999999999999999999999999",
                    )
                    .unwrap(),
                    estimated_pnl_denom: "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65".to_string(),
                    available_liquidity: coin(
                        7705931608,
                        "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4",
                    ),
                    weight_balance_ratio: Decimal::zero().to_string(),
                    borrow_interest_rate: Decimal::from_atomics(Uint128::new(323_793_793_684), 18)
                        .unwrap().to_string(),
                    funding_rate: Decimal::from_atomics(Uint128::new(1_000_000_000_000_000), 18)
                        .unwrap().to_string(),
                    price_impact: Decimal::from_atomics(Uint128::new(6_495_303_442_450), 18)
                        .unwrap().to_string(),
                };

                return Ok(to_json_binary(&resp)?);
            }
            _ => bail!("query is not implemented for ElysMsg"),
        }
    }

    fn execute<ExecC, QueryC>(
        &self,
        api: &dyn cosmwasm_std::Api,
        storage: &mut dyn cosmwasm_std::Storage,
        router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        block: &cosmwasm_std::BlockInfo,
        sender: cosmwasm_std::Addr,
        msg: Self::ExecT,
    ) -> AnyResult<cw_multi_test::AppResponse>
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
            ElysMsg::PerpetualOpen { collateral, .. } => {
                let msg_resp = PerpetualOpenResponse { id: 1 };

                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&msg_resp)?),
                };

                let burn_msg = BankMsg::Burn {
                    amount: vec![collateral],
                };
                router
                    .execute(api, storage, block, sender, burn_msg.into())
                    .unwrap();

                Ok(resp)
            }
            _ => bail!("execute is not implemented for ElysMsg"),
        }
    }

    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn cosmwasm_std::Api,
        _storage: &mut dyn cosmwasm_std::Storage,
        _router: &dyn cw_multi_test::CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &cosmwasm_std::BlockInfo,
        _msg: Self::SudoT,
    ) -> AnyResult<cw_multi_test::AppResponse>
    where
        ExecC: std::fmt::Debug
            + Clone
            + PartialEq
            + schemars::JsonSchema
            + serde::de::DeserializeOwned
            + 'static,
        QueryC: cosmwasm_std::CustomQuery + serde::de::DeserializeOwned + 'static,
    {
        bail!("sudo is not implemented for ElysMsg")
    }
}

#[test]
fn reproduce_testnet_issue_create_perpetual_market_open_order() {
    // Create a wallet for the "user" with an initial balance of 100 usdc
    let wallet = vec![(
        "user",
        coins(
            200__000_000,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ),
    )];

    // Initialize the ElysApp instance with the specified wallet.
    let mut addresses: Vec<String> = vec![];
    let mut app = BasicAppBuilder::<ElysMsg, ElysQuery>::new_custom()
        .with_custom(ElysModule {})
        .build(|roouter, _, storage| {
            for (wallet_owner, wallet_contenent) in wallet {
                roouter
                    .bank
                    .init_balance(storage, &Addr::unchecked(wallet_owner), wallet_contenent)
                    .unwrap();
                addresses.push(wallet_owner.to_owned())
            }
        });

    // Create a mock message to instantiate the contract with no initial orders.
    let instantiate_msg = InstantiateMockMsg {
        account_history_address: None,
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

    // Create a contract wrapper and store its code.
    let code = ContractWrapper::new(execute, instantiate, query).with_reply(reply);
    let code_id = app.store_code(Box::new(code));

    // Instantiate the contract with "owner" as the deployer.
    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("owner"),
            &instantiate_msg,
            &[],
            "Contract",
            None,
        )
        .unwrap();

    // User "user" creates a "MakerBuy" perpetual order for BTC
    app.execute_contract(
        Addr::unchecked("user"),
        addr.clone(),
        &ExecuteMsg::CreatePerpetualOrder {
            position: Some(PerpetualPosition::Long),
            leverage: Some(SignedDecimal::from_atomics(Int64::new(5), 0).unwrap()), // 5x leverage
            trading_asset: Some(
                "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4".to_string(),
            ), // atom
            take_profit_price: None,
            order_type: PerpetualOrderType::MarketOpen,
            trigger_price: None,
            position_id: None,
        },
        &coins(
            100__000_000,
            "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65",
        ), // collateral.
    )
    .unwrap();

    // Verify that the "user" has 100 usdc left after creating the order.
    assert_eq!(
        app.wrap()
            .query_balance(
                "user",
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        100__000_000
    );

    // Verify that the contract address send the usdc to the Perpetual Module.
    assert_eq!(
        app.wrap()
            .query_balance(
                &addr,
                "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65"
            )
            .unwrap()
            .amount
            .u128(),
        0
    );
}
