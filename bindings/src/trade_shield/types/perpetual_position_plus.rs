use crate::{query_resp::AmmSwapEstimationByDenomResponse, types::Mtp, ElysQuerier};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Decimal, Order, SignedDecimal, StdError, StdResult, Storage};

use crate::trade_shield::{states::PENDING_PERPETUAL_ORDER, types::PerpetualOrder};

use super::{OrderPrice, PerpetualOrderType};

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

        let collateral_amount = match SignedDecimal::from_atomics(mtp.collateral.i128(), collateral_info.asset_info.decimal as u32) {
            Ok(collateral_amount) => collateral_amount,
            Err(e) => return Err(StdError::generic_err(e.to_string())),
        };

        let current_price = querier.get_asset_price(&mtp.trading_asset)?;

        let unrealized_pnl = Self::calc_unrealized_pnl(&mtp, &collateral_amount, &current_price)?;
        let liquidation_price = Self::calc_liquidation_price(&mtp, &collateral_amount)?;
        let stop_loss_price = Self::get_stop_loss_price(&mtp, storage);

        Ok(Self {
            mtp,
            unrealized_pnl,
            liquidation_price,
            stop_loss_price,
        })
    }

    fn calc_unrealized_pnl(
        mtp: &Mtp,
        collateral_amount: &SignedDecimal,
        current_price: &Decimal,
    ) -> StdResult<SignedDecimal> {
        let current_price = match SignedDecimal::try_from(current_price.to_owned()) {
            Ok(current_price) => current_price,
            Err(e) => return Err(StdError::generic_err(e.to_string())),
        };

        let price_difference = current_price.checked_sub(mtp.open_price)?;

        //unrealized_pnl = collateral_amount * leverage * (current_price - open_price)
        let unrealized_pnl = collateral_amount
            .checked_mul(mtp.leverage)?
            .checked_mul(price_difference)?;

        Ok(unrealized_pnl)
    }

    fn calc_liquidation_price(
        mtp: &Mtp,
        collateral_amount: &SignedDecimal,
    ) -> StdResult<SignedDecimal> {
        //liquidation_price = -collateral_amount / ( collateral_amount * leverage ) + open_price
        let liquidation_price_div = match collateral_amount
            .clone()
            .checked_div(collateral_amount.checked_mul(mtp.leverage)?)
        {
            Ok(liquidation_price_div) => -liquidation_price_div,
            Err(e) => return Err(StdError::generic_err(e.to_string())),
        };

        let liquidation_price = liquidation_price_div.checked_add(mtp.open_price)?;

        Ok(liquidation_price)
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
