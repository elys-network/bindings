use crate::trade_shield::types::PerpetualPositionPlus;
use crate::types::PerpetualPosition;
use crate::ElysQuerier;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DecCoin, Decimal, Decimal256, SignedDecimal, StdError, StdResult, Uint128};

#[cw_serde]
pub struct PerpetualAssets {
    pub total_perpetual_asset_balance: DecCoin,
    pub perpetual_asset: Vec<PerpetualAsset>,
}

#[cw_serde]
pub struct PerpetualAsset {
    pub id: u64,
    pub denom: String,
    pub position: PerpetualPosition,
    pub pnl: SignedDecimal,
    pub collateral: DecCoin,
    pub leverage: SignedDecimal,
    pub size: DecCoin,
    pub order_price: SignedDecimal,
    pub liquidation: SignedDecimal,
    pub health: SignedDecimal,
    pub profit_price: DecCoin,
    pub stop_loss: Option<DecCoin>,
    pub fees: Decimal,
}

impl PerpetualAsset {
    pub fn new(
        mtp: PerpetualPositionPlus,
        usdc_denom: String,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<Self> {
        let collateral_info = querier.asset_info(mtp.mtp.collateral_asset.clone())?;
        let trading_asset_info = querier.asset_info(mtp.mtp.trading_asset.clone())?;

        Ok(PerpetualAsset {
            id: mtp.mtp.id,
            denom: mtp.mtp.collateral_asset.clone(),
            position: PerpetualPosition::try_from_i32(mtp.mtp.position)?,
            pnl: mtp.unrealized_pnl,
            collateral: DecCoin {
                denom: mtp.mtp.collateral_asset.clone(),
                amount: Decimal256::from(
                    Decimal::from_atomics(
                        Uint128::new(mtp.mtp.collateral.i128() as u128),
                        collateral_info.asset_info.decimal as u32,
                    )
                    .map_err(|e| {
                        StdError::generic_err(format!(
                            "failed to convert collateral to Decimal256: {}",
                            e
                        ))
                    })?,
                ),
            },
            leverage: mtp.mtp.leverage,
            size: DecCoin {
                denom: mtp.mtp.trading_asset.clone(),
                amount: Decimal256::from(
                    Decimal::from_atomics(
                        Uint128::new(mtp.mtp.custody.i128() as u128),
                        trading_asset_info.asset_info.decimal as u32,
                    )
                    .map_err(|e| {
                        StdError::generic_err(format!(
                            "failed to convert custody to Decimal256: {}",
                            e
                        ))
                    })?,
                ),
            },
            order_price: mtp.mtp.open_price,
            liquidation: mtp.liquidation_price,
            health: mtp.mtp.mtp_health,
            profit_price: DecCoin {
                denom: mtp.mtp.collateral_asset.clone(),
                amount: Decimal256::try_from(mtp.mtp.take_profit_price).map_err(|e| {
                    StdError::generic_err(format!(
                        "failed to convert take_profit_price to Decimal256: {}",
                        e
                    ))
                })?,
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
            .map_err(|e| {
                StdError::generic_err(format!(
                    "failed to convert borrow_interest_paid_collateral to Decimal256: {}",
                    e
                ))
            })?,
        })
    }
}

impl PerpetualAssets {
    pub fn default() -> Self {
        Self {
            total_perpetual_asset_balance: DecCoin {
                denom: "uusd".to_owned(),
                amount: Decimal256::zero(),
            },
            perpetual_asset: vec![],
        }
    }
}
