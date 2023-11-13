use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Coin, CosmosMsg, CustomMsg, Decimal, Int128};

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
        meta_data: Option<Binary>,
    },
    MarginClose {
        creator: String,
        id: u64,
        meta_data: Option<Binary>,
    },
    AmmSwapExactAmountIn {
        sender: String,
        routes: Vec<SwapAmountInRoute>,
        token_in: Coin,
        token_out_min_amount: Int128,
        meta_data: Option<Binary>,
    },
}

impl ElysMsg {
    pub fn amm_swap_exact_amount_in(
        sender: &str,
        token_in: &Coin,
        token_route: &Vec<SwapAmountInRoute>,
        token_out_min_amount: Int128,
        meta_data: Option<Binary>,
    ) -> Self {
        Self::AmmSwapExactAmountIn {
            sender: sender.to_owned(),
            routes: token_route.to_owned(),
            token_in: token_in.to_owned(),
            token_out_min_amount,
            meta_data,
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
        meta_data: Option<Binary>,
    ) -> Self {
        Self::MarginOpen {
            creator: creator.into(),
            collateral_asset: collateral_asset.into(),
            collateral_amount,
            borrow_asset: borrow_asset.into(),
            position: position as i32,
            leverage,
            take_profit_price,
            meta_data,
        }
    }

    pub fn margin_close_position(
        creator: impl Into<String>,
        id: u64,
        meta_data: Option<Binary>,
    ) -> Self {
        Self::MarginClose {
            creator: creator.into(),
            id,
            meta_data,
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
