use std::str::FromStr;

use crate::entry_point::{execute, query, sudo};
use crate::tests::get_order_id_from_events::get_attr_from_events;
use anyhow::{bail, Result as AnyResult};
use cosmwasm_std::{
    coin, coins, to_json_binary, Addr, BankMsg, Coin, Decimal, Empty, Int128, SignedDecimal,
    SignedDecimal256, StdError, Uint64,
};
use cw_multi_test::{AppResponse, BasicAppBuilder, ContractWrapper, Executor, Module};
use elys_bindings::msg_resp::PerpetualOpenResponse;
use elys_bindings::query_resp::{
    OracleAssetInfoResponse, PerpetualGetPositionsForAddressResponseRaw,
    PerpetualOpenEstimationRawResponse, QueryGetEntryResponseRaw, QueryGetPriceResponse, RawEntry,
    TierCalculateDiscountResponse,
};
use elys_bindings::trade_shield::msg::query_resp::GetPerpetualOrderResp;
use elys_bindings::trade_shield::msg::{ExecuteMsg, QueryMsg, SudoMsg};
use elys_bindings::trade_shield::types::{
    OrderPrice, PerpetualOrderPlus, PerpetualOrderType, Status,
};
use elys_bindings::types::{Mtp, OracleAssetInfo, PageResponse, PerpetualPosition, Price};
use elys_bindings::{ElysMsg, ElysQuery};
use elys_bindings_test::{
    ElysModule, ACCOUNT, ASSET_INFO, LAST_MODULE_USED, PERPETUAL_OPENED_POSITION, PRICES,
};

use super::InstantiateMockMsg;
use super::{instantiate, reply};

const USDC_DENOM: &str = "ibc/2180E84E20F5679FCC760D8C165B60F42065DEF7F46A72B447CFF1B7DC6C0A65";
const ATOM_DENOM: &str = "ibc/E2D2F6ADCC68AA3384B2F5DFACCA437923D137C14E86FB8A10207CF3BED0C8D4";

struct ElysModuleWrapper(ElysModule);

impl Module for ElysModuleWrapper {
    type QueryT = ElysQuery;
    type ExecT = ElysMsg;
    type SudoT = Empty;

    fn query(
        &self,
        _api: &dyn cosmwasm_std::Api,
        storage: &dyn cosmwasm_std::Storage,
        _querier: &dyn cosmwasm_std::Querier,
        _block: &cosmwasm_std::BlockInfo,
        request: Self::QueryT,
    ) -> AnyResult<cosmwasm_std::Binary> {
        match request {
            ElysQuery::AssetProfileEntry { base_denom } => {
                let entry = match base_denom.as_str() {
                    "uusdc" => RawEntry {
                        base_denom: Some(base_denom.clone()),
                        decimals: Some(6),
                        denom: Some(USDC_DENOM.to_string()),
                        path: None,
                        ibc_channel_id: None,
                        ibc_counterparty_channel_id: None,
                        display_name: Some("USDC".to_string()),
                        display_symbol: None,
                        network: None,
                        address: None,
                        external_symbol: None,
                        transfer_limit: None,
                        permissions: None,
                        unit_denom: None,
                        ibc_counterparty_denom: None,
                        ibc_counterparty_chain_id: None,
                        authority: None,
                        commit_enabled: None,
                        withdraw_enabled: None,
                    },
                    _ => panic!(
                        "request: AssetProfileEntry, denom_not_found: {}",
                        base_denom
                    ),
                };
                Ok(to_json_binary(&QueryGetEntryResponseRaw { entry })?)
            }
            ElysQuery::OraclePrice { asset, .. } => match asset.as_str() {
                "USDC" => Ok(to_json_binary(&QueryGetPriceResponse {
                    price: Price {
                        asset,
                        price: Decimal::one(),
                        source: "".to_string(),
                        provider: "".to_string(),
                        timestamp: 0,
                        block_height: 0,
                    },
                })?),
                _ => Err(StdError::not_found("").into()),
            },
            ElysQuery::OracleAssetInfo { denom } => match denom.as_str() {
                ATOM_DENOM => Ok(to_json_binary(&OracleAssetInfoResponse {
                    asset_info: OracleAssetInfo {
                        denom,
                        display: "ATOM".to_string(),
                        band_ticker: "".to_string(),
                        elys_ticker: "".to_string(),
                        decimal: 6,
                    },
                })?),
                _ => panic!("OracleAssetInfo: notfound: {}", denom),
            },
            ElysQuery::AmmPriceByDenom { token_in, .. } => match token_in.denom.as_str() {
                ATOM_DENOM => Ok(to_json_binary(&Decimal::from_str("12.38").unwrap())?),
                _ => panic!("AmmPriceByDenom: notfound: {}", token_in.denom.as_str()),
            },
            ElysQuery::PerpetualOpenEstimation {
                position,
                leverage,
                trading_asset,
                collateral,
                take_profit_price,
                discount,
            } => Ok(to_json_binary(&PerpetualOpenEstimationRawResponse {
                position,
                leverage: leverage.to_string(),
                trading_asset,
                min_collateral: collateral.clone(),
                position_size: coin(0, ""),
                collateral,
                available_liquidity: coin(0, ""),
                valid_collateral: Some(true),
                swap_fee: Decimal::zero().to_string(),
                discount: discount.to_string(),
                open_price: Decimal::zero().to_string(),
                take_profit_price: take_profit_price.to_string(),
                liquidation_price: Decimal::zero().to_string(),
                estimated_pnl: Int128::zero(),
                estimated_pnl_denom: "".to_string(),
                slippage: Decimal::zero().to_string(),
                weight_balance_ratio: Decimal::zero().to_string(),
                borrow_interest_rate: Decimal::zero().to_string(),
                funding_rate: Decimal::zero().to_string(),
                price_impact: Decimal::zero().to_string(),
                borrow_fee: Coin::new(0, ""),
                funding_fee: Coin::new(0, ""),
            })?),
            //ignoring address here since we only use one user
            ElysQuery::PerpetualGetPositionsForAddress { .. } => {
                let mtps = PERPETUAL_OPENED_POSITION.load(storage)?;
                let mtps = if mtps.is_empty() { None } else { Some(mtps) };

                Ok(to_json_binary(
                    &PerpetualGetPositionsForAddressResponseRaw {
                        mtps,
                        pagination: PageResponse::empty(false),
                    },
                )?)
            }
            ElysQuery::TierCalculateDiscount { .. } => {
                let resp = TierCalculateDiscountResponse {
                    tier: "bronze".to_string(),
                    discount: "0".to_string(),
                    portfolio: "10".to_string(),
                };
                Ok(to_json_binary(&resp)?)
            }
            _ => panic!("not implemented {request:?}"),
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
            ElysMsg::PerpetualOpen {
                creator,
                position,
                collateral,
                trading_asset,
                leverage,
                take_profit_price,
                owner,
            } => {
                let mut mtp_vec = PERPETUAL_OPENED_POSITION.load(storage)?;

                let mtp_id: u64 = match mtp_vec.iter().max_by_key(|mtp| mtp.id) {
                    Some(mtp) => mtp.id + 1,
                    None => 0,
                };
                let collaterals = vec![collateral.clone()];
                let custody = (leverage.clone()
                    * SignedDecimal::from_atomics(
                        Int128::new(collateral.amount.u128() as i128),
                        0,
                    )
                    .unwrap())
                .floor()
                .atomics();
                let mtp: Mtp = Mtp {
                    address: owner,
                    liabilities: Int128::zero(),
                    take_profit_liabilities: Int128::zero(),
                    mtp_health: SignedDecimal::one(),
                    position,
                    id: mtp_id,
                    amm_pool_id: 0,
                    consolidate_leverage: SignedDecimal::zero(),
                    sum_collateral: Int128::zero(),
                    take_profit_price,
                    borrow_interest_paid_collateral: Int128::zero(),
                    borrow_interest_paid_custody: Int128::zero(),
                    borrow_interest_unpaid_collateral: Int128::zero(),
                    collateral_asset: collateral.denom,
                    collateral: Int128::new((collateral.amount.u128()) as i128),
                    custody,
                    custody_asset: "".to_string(),
                    funding_fee_paid_collateral: Int128::zero(),
                    funding_fee_paid_custody: Int128::zero(),
                    funding_fee_received_collateral: Int128::zero(),
                    funding_fee_received_custody: Int128::zero(),
                    leverage: SignedDecimal::try_from(leverage)?,
                    liabilities_asset: "".to_string(),
                    open_price: SignedDecimal::zero(),
                    take_profit_borrow_rate: SignedDecimal::zero(),
                    take_profit_custody: Int128::zero(),
                    trading_asset,
                    stop_loss_price: SignedDecimal::zero(),
                    last_interest_calc_time: None,
                    last_interest_calc_block: None,
                    last_funding_calc_time: None,
                    last_funding_calc_block: None,
                };

                let msg_resp = PerpetualOpenResponse { id: mtp.id };

                let resp = AppResponse {
                    events: vec![],
                    data: Some(to_json_binary(&msg_resp)?),
                };

                mtp_vec.push(mtp);
                PERPETUAL_OPENED_POSITION.save(storage, &mtp_vec).unwrap();

                let burn_msg = BankMsg::Burn {
                    amount: collaterals,
                };
                router
                    .execute(
                        api,
                        storage,
                        block,
                        Addr::unchecked(creator),
                        burn_msg.into(),
                    )
                    .unwrap();

                Ok(resp)
            }
            _ => panic!("not implemented {msg:?}"),
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
fn pending_limit_open_short_order_with_price_not_met() {
    // Create a wallet for the "user" with an initial balance of 110 usdc
    let wallet = vec![("user", vec![coin(110_000000, USDC_DENOM)])];

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

    // Create a mock message to instantiate the contract with the dummy order.
    let instantiate_msg = InstantiateMockMsg {
        spot_orders: vec![],
        perpetual_orders: vec![],
    };

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

    // Create a "limit open" order with a specific rate and balance.
    let resp = app
        .execute_contract(
            Addr::unchecked("user"),
            addr.clone(),
            &ExecuteMsg::CreatePerpetualOrder {
                position: Some(PerpetualPosition::Short),
                leverage: Some(SignedDecimal::from_str("5.0").unwrap()),
                trading_asset: Some(ATOM_DENOM.to_string()),
                take_profit_price: Some(SignedDecimal256::from_str("30.0").unwrap()),
                order_type: PerpetualOrderType::LimitOpen,
                trigger_price: Some(OrderPrice {
                    base_denom: USDC_DENOM.to_string(),
                    quote_denom: ATOM_DENOM.to_string(),
                    rate: Decimal::from_str("15.0").unwrap(),
                }),
                position_id: None,
            },
            &coins(110_000000, USDC_DENOM),
        )
        .unwrap();

    let order_id = get_attr_from_events(&resp.events, "perpetual_order_id").unwrap();

    let GetPerpetualOrderResp {
        order: PerpetualOrderPlus { order, .. },
    } = app
        .wrap()
        .query_wasm_smart(
            addr.clone(),
            &QueryMsg::GetPerpetualOrder {
                id: u64::from_str(&order_id).unwrap(),
            },
        )
        .unwrap();

    assert_eq!(order.status, Status::Pending);

    app.wasm_sudo(addr.clone(), &SudoMsg::ClockEndBlock {})
        .unwrap();

    let GetPerpetualOrderResp {
        order: PerpetualOrderPlus { order, .. },
    } = app
        .wrap()
        .query_wasm_smart(addr.clone(), &QueryMsg::GetPerpetualOrder { id: 0 })
        .unwrap();

    assert_eq!(order.status, Status::Pending);

    assert_eq!(
        app.wrap()
            .query_balance(&addr, USDC_DENOM)
            .unwrap()
            .amount
            .u128(),
        110_000000
    );

    assert_eq!(
        app.wrap()
            .query_balance("user", USDC_DENOM)
            .unwrap()
            .amount
            .u128(),
        0
    );

    let PerpetualGetPositionsForAddressResponseRaw { mtps, .. } = app
        .wrap()
        .query(&ElysQuery::perpetual_get_position_for_address("user".to_string(), None).into())
        .unwrap();

    assert!(mtps.is_none());
}
