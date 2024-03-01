use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Coin, CosmosMsg, CustomMsg, Decimal, Int128, SignedDecimal, SignedDecimal256, Uint128,
};

use crate::{
    trade_shield::types::default_take_profit_price,
    types::{EarnType, PerpetualPosition, SwapAmountInRoute},
};

#[cw_serde]
pub enum ElysMsg {
    PerpetualOpen {
        creator: String,
        position: i32,
        collateral: Coin,
        trading_asset: String,
        leverage: SignedDecimal,
        take_profit_price: SignedDecimal256,
    },
    PerpetualClose {
        creator: String,
        id: u64,
        amount: Int128,
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
    CommitmentStake {
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    CommitmentUnstake {
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    IncentiveBeginRedelegate {
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    },
    IncentiveCancelUnbondingDelegation {
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    },
    CommitmentVest {
        creator: String,
        amount: Int128,
        denom: String,
    },
    CommitmentCancelVest {
        creator: String,
        amount: Int128,
        denom: String,
    },
    IncentiveWithdrawRewards {
        delegator_address: String,
        withdraw_type: i32,
    },
    IncentiveWithdrawValidatorCommission {
        delegator_address: String,
        validator_address: String,
    },
    AmmJoinPool {
        sender: String,
        pool_id: u64,
        max_amounts_in: Vec<Coin>,
        share_amount_out: Uint128,
        no_remaining: bool,
    },
    AmmExitPool {
        sender: String,
        pool_id: u64,
        min_amounts_out: Vec<Coin>,
        share_amount_in: Uint128,
        token_out_denom: String,
    },

    LeveragelpOpen {
        creator: String,
        collateral_asset: String,
        collateral_amount: Int128,
        amm_pool_id: u64,
        leverage: SignedDecimal,
        stop_loss_price: SignedDecimal,
    },
    LeveragelpClose {
        creator: String,
        position_id: u64,
        amount: Int128,
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

    pub fn perpetual_open_position(
        creator: impl Into<String>,
        collateral: Coin,
        trading_asset: impl Into<String>,
        position: PerpetualPosition,
        leverage: SignedDecimal,
        take_profit_price: Option<SignedDecimal256>,
    ) -> Self {
        let take_profit_price = match take_profit_price {
            Some(price) => price,
            None => default_take_profit_price(),
        };
        Self::PerpetualOpen {
            creator: creator.into(),
            collateral,
            position: position as i32,
            leverage,
            take_profit_price,
            trading_asset: trading_asset.into(),
        }
    }

    pub fn perpetual_close_position(creator: impl Into<String>, id: u64, amount: i128) -> Self {
        Self::PerpetualClose {
            creator: creator.into(),
            id,
            amount: Int128::new(amount),
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

    pub fn stake_token(
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentStake {
            creator: creator.to_owned(),
            amount: amount.to_owned(),
            asset: asset.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }

    pub fn unstake_token(
        creator: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentUnstake {
            creator: creator.to_owned(),
            amount: amount.to_owned(),
            asset: asset.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }

    pub fn begin_redelegate(
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    ) -> Self {
        Self::IncentiveBeginRedelegate {
            delegator_address: delegator_address.to_owned(),
            validator_src_address: validator_src_address.to_owned(),
            validator_dst_address: validator_dst_address.to_owned(),
            amount: amount.to_owned(),
        }
    }

    pub fn cancel_unbonding(
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    ) -> Self {
        Self::IncentiveCancelUnbondingDelegation {
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
            amount: amount.to_owned(),
            creation_height: creation_height.to_owned(),
        }
    }

    pub fn eden_vesting(creator: String, amount: Int128, denom: String) -> Self {
        Self::CommitmentVest {
            creator: creator.to_owned(),
            amount: amount,
            denom: denom.to_owned(),
        }
    }

    pub fn eden_cancel_vesting(creator: String, amount: Int128, denom: String) -> Self {
        Self::CommitmentCancelVest {
            creator: creator.to_owned(),
            amount: amount,
            denom: denom.to_owned(),
        }
    }

    pub fn withdraw_rewards(delegator_address: String, withdraw_type: EarnType) -> Self {
        Self::IncentiveWithdrawRewards {
            delegator_address: delegator_address.to_owned(),
            withdraw_type: withdraw_type as i32,
        }
    }

    pub fn withdraw_validator_commissions(
        delegator_address: String,
        validator_address: String,
    ) -> Self {
        Self::IncentiveWithdrawValidatorCommission {
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }

    pub fn amm_join_pool(
        sender: String,
        pool_id: u64,
        max_amounts_in: Vec<Coin>,
        share_amount_out: Uint128,
        no_remaining: bool,
    ) -> Self {
        Self::AmmJoinPool {
            sender: sender,
            pool_id: pool_id,
            max_amounts_in: max_amounts_in,
            share_amount_out: share_amount_out,
            no_remaining: no_remaining,
        }
    }

    pub fn amm_exit_pool(
        sender: String,
        pool_id: u64,
        min_amounts_out: Vec<Coin>,
        share_amount_in: Uint128,
        token_out_denom: String,
    ) -> Self {
        Self::AmmExitPool {
            sender: sender,
            pool_id: pool_id,
            min_amounts_out: min_amounts_out,
            share_amount_in: share_amount_in,
            token_out_denom: token_out_denom,
        }
    }

    pub fn leveragelp_open_position(
        creator: String,
        amm_pool_id: u64,
        collateral_asset: String,
        collateral_amount: Int128,
        leverage: SignedDecimal,
        stop_loss_price: SignedDecimal,
    ) -> Self {
        Self::LeveragelpOpen {
            creator: creator,
            collateral_asset: collateral_asset,
            collateral_amount: collateral_amount,
            amm_pool_id: amm_pool_id,
            leverage: leverage,
            stop_loss_price: stop_loss_price,
        }
    }

    pub fn leveragelp_close_position(creator: String, position_id: u64, amount: Int128) -> Self {
        Self::LeveragelpClose {
            creator: creator,
            position_id: position_id,
            amount: amount,
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
