use crate::{types::Mtp, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Order, SignedDecimal, StdError, StdResult, Storage};

use crate::trade_shield::{states::PENDING_PERPETUAL_ORDER, types::PerpetualOrder};

use super::{OrderPrice, PerpetualOrderType, PerpetualPosition};

#[cw_serde]
pub struct PerpetualPositionPlus {
    pub mtp: Mtp,
    pub unrealized_pnl: SignedDecimal,
    pub liquidation_price: SignedDecimal,
    pub stop_loss_price: Option<OrderPrice>,
}

impl PerpetualPositionPlus {
    pub fn news(
        mtps: Vec<Mtp>,
        storage: &dyn Storage,
        querier: &ElysQuerier<'_>,
    ) -> StdResult<Vec<Self>> {
        let stop_loss_prices = Self::get_stop_loss_prices(&mtps, storage);

        if mtps.len() != stop_loss_prices.len() {
            return Err(StdError::generic_err("parsing stop_loss_prices error"));
        }

        let mut mtps_plus: Vec<PerpetualPositionPlus> = Vec::new();

        for i in 0..mtps.len() {
            let mtp = mtps[i].clone();

            let mtp_plus = Self::new(mtp, storage, querier)?;

            mtps_plus.push(mtp_plus)
        }

        Ok(mtps_plus)
    }

    pub fn new(mtp: Mtp, storage: &dyn Storage, querier: &ElysQuerier<'_>) -> StdResult<Self> {
        let collateral_info = querier.asset_info(mtp.collateral_asset.clone())?;

        let collateral_amount = SignedDecimal::from_atomics(
            mtp.collateral.i128(),
            collateral_info.asset_info.decimal as u32,
        )
        .map_err(|e| StdError::generic_err(e.to_string()))?;

        let custody_amount = SignedDecimal::from_atomics(
            mtp.custody.i128(),
            collateral_info.asset_info.decimal as u32,
        )
        .map_err(|e| StdError::generic_err(e.to_string()))?;

        let _liabilities_amount = SignedDecimal::from_atomics(
            mtp.custody.i128(),
            collateral_info.asset_info.decimal as u32,
        )
        .map_err(|e| StdError::generic_err(e.to_string()))?;

        let unrealized_pnl = Self::calc_unrealized_pnl(
            &mtp,
            // &custody_amount,
            // &collateral_amount,
            // &liabilities_amount,
        )
        .map_err(|e| StdError::generic_err(format!("unrealized_pnl: {:?}", e.to_string())))?;
        let liquidation_price =
            Self::calc_liquidation_price(&mtp, &collateral_amount, &custody_amount).map_err(
                |e| StdError::generic_err(format!("liquidation_price: {:?}", e.to_string())),
            )?;
        let stop_loss_price = Self::get_stop_loss_price(&mtp, storage);

        Ok(Self {
            mtp,
            unrealized_pnl,
            liquidation_price,
            stop_loss_price,
        })
    }

    fn calc_unrealized_pnl(mtp: &Mtp) -> StdResult<SignedDecimal> {
        let custody_amount: SignedDecimal = SignedDecimal::from_atomics(mtp.custody, 0).unwrap();
        let collateral_amount: SignedDecimal =
            SignedDecimal::from_atomics(mtp.collateral, 0).unwrap();
        let liabilities_amount: SignedDecimal =
            SignedDecimal::from_atomics(mtp.liabilities, 0).unwrap();
        let take_profit_price = SignedDecimal::try_from(mtp.take_profit_price.clone())
            .map_err(|e| StdError::generic_err(e.to_string()))?;
        let open_price = mtp.open_price.clone();

        match PerpetualPosition::try_from_i32(mtp.position)? {
            //estimated_pnl = custody_amount - (liability_amount + collateral_amount) / take_profit_price
            PerpetualPosition::Long => custody_amount
                .checked_sub(
                    liabilities_amount
                        .checked_add(collateral_amount.to_owned())?
                        .checked_div(take_profit_price)
                        .map_err(|e| StdError::generic_err(e.to_string()))?,
                )
                .map_err(|e| StdError::generic_err(e.to_string())),

            // if position is short then estimated pnl is custody_amount / open_price - (liability_amount + collateral_amount) / take_profit_price
            PerpetualPosition::Short => {
                liabilities_amount
                    .clone()
                    .checked_add(collateral_amount.checked_div(take_profit_price).map_err(
                        |e| StdError::generic_err(format!("take_profit_price: {}", e.to_string())),
                    )?)?
                    .checked_sub(liabilities_amount.checked_div(open_price.clone()).map_err(
                        |e| {
                            StdError::generic_err(format!(
                                "open_price: {:?}: {}",
                                &open_price,
                                e.to_string()
                            ))
                        },
                    )?)
                    .map_err(|e| StdError::generic_err(e.to_string()))
            }

            PerpetualPosition::Unspecified => Err(StdError::generic_err("Position is Unspecified")),
        }
    }

    fn calc_liquidation_price(
        mtp: &Mtp,
        collateral_amount: &SignedDecimal,
        custody_amount: &SignedDecimal,
    ) -> StdResult<SignedDecimal> {
        let open_price = mtp.open_price.clone();

        match PerpetualPosition::try_from_i32(mtp.position)? {
            // liquidation_price = open_price_value - collateral_amount / custody_amount
            PerpetualPosition::Long => Ok(open_price.checked_sub(
                collateral_amount
                    .checked_div(custody_amount.to_owned())
                    .map_err(|e| StdError::generic_err(e.to_string()))?,
            )?),

            // if position is short then liquidation price is open price + collateral amount / (custody amount / open price)
            PerpetualPosition::Short => open_price
                .clone()
                .checked_add(collateral_amount.to_owned())?
                .checked_div(
                    custody_amount
                        .checked_div(open_price)
                        .map_err(|e| StdError::generic_err(e.to_string()))?,
                )
                .map_err(|e| StdError::generic_err(e.to_string())),

            PerpetualPosition::Unspecified => Err(StdError::generic_err("Position is Unspecified")),
        }
    }

    fn get_stop_loss_price(mtp: &Mtp, storage: &dyn Storage) -> Option<OrderPrice> {
        let perpetual_order = PENDING_PERPETUAL_ORDER
            .prefix_range(storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok().map(|r| r.1))
            .find(|order| {
                order.position_id == Some(mtp.id)
                    && order.order_type == PerpetualOrderType::StopLoss
            });

        match perpetual_order {
            Some(order) => order.trigger_price,
            None => None,
        }
    }

    fn get_stop_loss_prices(mtps: &Vec<Mtp>, storage: &dyn Storage) -> Vec<Option<OrderPrice>> {
        let perpetual_orders: Vec<PerpetualOrder> = PENDING_PERPETUAL_ORDER
            .prefix_range(storage, None, None, Order::Ascending)
            .filter_map(|res| res.ok().map(|r| r.1))
            .collect();

        let mut stop_loss_prices: Vec<Option<OrderPrice>> = Vec::new();

        for Mtp { id, .. } in mtps {
            let price = match perpetual_orders
                .iter()
                .find(|order| order.position_id == Some(*id))
            {
                Some(order) => order.trigger_price.to_owned(),
                None => None,
            };

            stop_loss_prices.push(price)
        }

        stop_loss_prices
    }
}
