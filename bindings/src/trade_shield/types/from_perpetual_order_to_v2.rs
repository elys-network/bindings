use super::{PerpetualOrder, PerpetualOrderV2};

impl Into<PerpetualOrderV2> for PerpetualOrder {
    fn into(self) -> PerpetualOrderV2 {
        PerpetualOrderV2 {
            order_id: self.order_id,
            owner: self.owner,
            order_type: self.order_type,
            position: self.position,
            trigger_price: self.trigger_price,
            collateral: self.collateral,
            trading_asset: self.trading_asset,
            leverage: self.leverage,
            take_profit_price: self.take_profit_price,
            position_id: self.position_id,
            status: self.status,
            size: None,
            liquidation: None,
            borrow_fee: None,
            funding_fee: None,
        }
    }
}
