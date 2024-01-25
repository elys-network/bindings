use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal, Decimal256, SignedDecimal, StdError, StdResult, Uint128};
use elys_bindings::trade_shield::types::MarginPositionPlus;
use elys_bindings::types::MarginPosition;
use elys_bindings::ElysQuerier;

#[cw_serde]
pub struct PerpetualAssets {
    pub total_perpetual_pools_balance: DecCoin,
    pub perpetual_asset: Vec<PerpetualAsset>,
}

#[cw_serde]
pub struct PerpetualAsset {
    pub denom: String,
    pub position: MarginPosition,
    pub pnl: DecCoin,
    pub collateral: DecCoin,
    pub leverage: SignedDecimal,
    pub size: DecCoin,
    pub order_price: DecCoin,
    pub liquidation: DecCoin,
    pub health: SignedDecimal,
    pub profit_price: DecCoin,
    pub stop_loss: Option<DecCoin>,
    pub fees: Decimal,
}

impl PerpetualAsset {
    pub fn new(
        mtp: MarginPositionPlus,
        usdc_denom: String,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<Self> {
        let collateral_info = querier.asset_info(mtp.mtp.collateral_asset.clone())?;

        Ok(PerpetualAsset {
            denom: mtp.mtp.collateral_asset.clone(),
            position: MarginPosition::try_from_i32(mtp.mtp.position).unwrap(),
            pnl: DecCoin {
                denom: usdc_denom.to_owned(),
                amount: Decimal256::try_from(mtp.unrealized_pnl)
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
            },
            collateral: DecCoin {
                denom: mtp.mtp.collateral_asset.clone(),
                amount: Decimal256::from(
                    Decimal::from_atomics(
                        Uint128::new(mtp.mtp.collateral.i128() as u128),
                        collateral_info.asset_info.decimal as u32,
                    )
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
                ),
            },
            leverage: mtp.mtp.leverage,
            size: DecCoin {
                denom: mtp.mtp.collateral_asset.clone(),
                amount: Decimal256::from(
                    Decimal::from_atomics(
                        Uint128::new(mtp.mtp.custody.i128() as u128),
                        collateral_info.asset_info.decimal as u32,
                    )
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
                ),
            },
            order_price: DecCoin {
                amount: Decimal256::try_from(mtp.mtp.open_price)
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
                denom: usdc_denom.to_owned(),
            },
            liquidation: DecCoin {
                amount: Decimal256::try_from(mtp.liquidation_price)
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
                denom: usdc_denom.to_owned(),
            },
            health: mtp.mtp.mtp_health,
            profit_price: DecCoin {
                denom: mtp.mtp.collateral_asset.clone(),
                amount: Decimal256::from(
                    Decimal::from_atomics(
                        Uint128::new(mtp.mtp.custody.i128() as u128),
                        collateral_info.asset_info.decimal as u32,
                    )
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
                ),
            },
            stop_loss: match mtp.stop_loss_price {
                Some(stop_loss) => Some(DecCoin {
                    amount: Decimal256::from(stop_loss.rate),
                    denom: usdc_denom.to_owned(),
                }),
                None => None,
            },
            fees: Decimal::from_atomics(
                Uint128::new(mtp.mtp.borrow_interest_paid_collateral.i128() as u128),
                collateral_info.asset_info.decimal as u32,
            )
            .map_err(|e| StdError::generic_err(e.to_string()))?,
        })
    }
}

impl PerpetualAssets {
    pub fn default() -> Self {
        Self {
            total_perpetual_pools_balance: DecCoin {
                denom: "uusd".to_owned(),
                amount: Decimal256::zero(),
            },
            perpetual_asset: vec![],
        }
    }
}
