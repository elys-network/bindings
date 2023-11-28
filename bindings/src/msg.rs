use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Decimal, Int128};

use crate::types::{MarginPosition, SwapAmountInRoute};

#[cw_serde]
pub enum ElysMsg {
    MarginOpen {
        creator: String,
        collateral_asset: String,
        collateral_amount: Int128,
        borrow_asset: String,
        position: i32,
        leverage: Decimal,
        take_profit_price: Decimal,
    },
    MarginClose {
        creator: String,
        id: u64,
    },

    MarginBrokerOpen {
        creator: String,
        collateral_asset: String,
        collateral_amount: Int128,
        borrow_asset: String,
        position: i32,
        leverage: Decimal,
        take_profit_price: Decimal,
        owner: String,
    },

    MarginBrokerClose {
        creator: String,
        id: u64,
        owner: String,
    },
    AmmSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
        discount: Decimal,
        recipient: String,
    },
    AmmSwapByDenom {
        sender: String,
        amount: Coin,
        min_amount: Coin,
        max_amount: Coin,
        in_denom: String,
        out_denom: String,
        discount: Decimal,
        recipient: String,
    },
}

impl ElysMsg {
    pub fn amm_swap_exact_amount_in(
        sender: impl Into<String>,
        token_in: &Coin,
        token_route: &Vec<SwapAmountInRoute>,
        token_out_min_amount: Int128,
        discount: Decimal,
        recipient: impl Into<String>,
    ) -> Self {
        Self::AmmSwapExactAmountIn {
            sender: sender.into(),
            routes: token_route.to_owned(),
            token_in: token_in.to_owned(),
            token_out_min_amount,
            discount,
            recipient: recipient.into(),
        }
    }

    pub fn margin_open_position(
        creator: impl Into<String>,
        collateral_asset: impl Into<String>,
        collateral_amount: Int128,
        borrow_asset: impl Into<String>,
        position: MarginPosition,
        leverage: Decimal,
        take_profit_price: Decimal,
    ) -> Self {
        Self::MarginOpen {
            creator: creator.into(),
            collateral_asset: collateral_asset.into(),
            collateral_amount,
            borrow_asset: borrow_asset.into(),
            position: position as i32,
            leverage,
            take_profit_price,
        }
    }

    pub fn margin_close_position(creator: impl Into<String>, id: u64) -> Self {
        Self::MarginClose {
            creator: creator.into(),
            id,
        }
    }
    pub fn margin_broker_open_position(
        creator: impl Into<String>,
        collateral_asset: impl Into<String>,
        collateral_amount: Int128,
        borrow_asset: impl Into<String>,
        position: i32,
        leverage: Decimal,
        take_profit_price: Decimal,
        owner: impl Into<String>,
    ) -> Self {
        Self::MarginBrokerOpen {
            creator: creator.into(),
            collateral_asset: collateral_asset.into(),
            collateral_amount,
            borrow_asset: borrow_asset.into(),
            position,
            leverage,
            take_profit_price,
            owner: owner.into(),
        }
    }

    pub fn margin_broker_close_position(
        creator: impl Into<String>,
        id: u64,
        owner: impl Into<String>,
    ) -> Self {
        Self::MarginBrokerClose {
            creator: creator.into(),
            id,
            owner: owner.into(),
        }
    }
    pub fn swap_by_denom(
        sender: impl Into<String>,
        amount: Coin,
        min_amount: Coin,
        max_amount: Coin,
        in_denom: impl Into<String>,
        out_denom: impl Into<String>,
        discount: Decimal,
        recipient: impl Into<String>,
    ) -> Self {
        Self::AmmSwapByDenom {
            sender: sender.into(),
            amount,
            min_amount,
            max_amount,
            in_denom: in_denom.into(),
            out_denom: out_denom.into(),
            recipient: recipient.into(),
            discount,
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
