use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, CosmosMsg, CustomMsg, Decimal, Int128};

use crate::types::{EarnType, MarginPosition, SwapAmountInRoute};

#[cw_serde]
pub enum ElysMsg {
    MarginOpen {
        creator: String,
        position: i32,
        collateral: Coin,
        trading_asset: String,
        leverage: Decimal,
        take_profit_price: Decimal,
    },
    MarginClose {
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
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    CommitmentUnstake {
        creator: String,
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    },
    IncentiveBeginRedelegate {
        creator: String,
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    },
    IncentiveCancelUnbondingDelegation {
        creator: String,
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    },
    CommitmentVest {
        creator: String,
        address: String,
        amount: Int128,
        denom: String,
    },
    CommitmentCancelVest {
        creator: String,
        address: String,
        amount: Int128,
        denom: String,
    },
    IncentiveWithdrawRewards {
        creator: String,
        delegator_address: String,
        withdraw_type: i32,
    },
    IncentiveWithdrawValidatorCommission {
        creator: String,
        delegator_address: String,
        validator_address: String,
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
        collateral: Coin,
        trading_asset: impl Into<String>,
        position: MarginPosition,
        leverage: Decimal,
        take_profit_price: Decimal,
    ) -> Self {
        Self::MarginOpen {
            creator: creator.into(),
            collateral,
            position: position as i32,
            leverage,
            take_profit_price,
            trading_asset: trading_asset.into(),
        }
    }

    pub fn margin_close_position(creator: impl Into<String>, id: u64, amount: i128) -> Self {
        Self::MarginClose {
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
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentStake {
            creator: creator.to_owned(),
            address: address.to_owned(),
            amount: amount.to_owned(),
            asset: asset.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }

    pub fn unstake_token(
        creator: String,
        address: String,
        amount: Int128,
        asset: String,
        validator_address: Option<String>,
    ) -> Self {
        Self::CommitmentUnstake {
            creator: creator.to_owned(),
            address: address.to_owned(),
            amount: amount.to_owned(),
            asset: asset.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }

    pub fn begin_redelegate(
        creator: String,
        delegator_address: String,
        validator_src_address: String,
        validator_dst_address: String,
        amount: Coin,
    ) -> Self {
        Self::IncentiveBeginRedelegate {
            creator: creator.to_owned(),
            delegator_address: delegator_address.to_owned(),
            validator_src_address: validator_src_address.to_owned(),
            validator_dst_address: validator_dst_address.to_owned(),
            amount: amount.to_owned(),
        }
    }

    pub fn cancel_unbonding(
        creator: String,
        delegator_address: String,
        validator_address: String,
        amount: Coin,
        creation_height: i64,
    ) -> Self {
        Self::IncentiveCancelUnbondingDelegation {
            creator: creator.to_owned(),
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
            amount: amount.to_owned(),
            creation_height: creation_height.to_owned(),
        }
    }

    pub fn eden_vesting(creator: String, address: String, amount: Int128, denom: String) -> Self {
        Self::CommitmentVest {
            creator: creator.to_owned(),
            address: address.to_owned(),
            amount: amount,
            denom: denom.to_owned(),
        }
    }

    pub fn eden_cancel_vesting(
        creator: String,
        address: String,
        amount: Int128,
        denom: String,
    ) -> Self {
        Self::CommitmentCancelVest {
            creator: creator.to_owned(),
            address: address.to_owned(),
            amount: amount,
            denom: denom.to_owned(),
        }
    }

    pub fn withdraw_rewards(
        creator: String,
        delegator_address: String,
        witdhraw_type: EarnType,
    ) -> Self {
        Self::IncentiveWithdrawRewards {
            creator: creator.to_owned(),
            delegator_address: delegator_address.to_owned(),
            withdraw_type: witdhraw_type as i32,
        }
    }

    pub fn withdraw_validator_commissions(
        creator: String,
        delegator_address: String,
        validator_address: String,
    ) -> Self {
        Self::IncentiveWithdrawValidatorCommission {
            creator: creator.to_owned(),
            delegator_address: delegator_address.to_owned(),
            validator_address: validator_address.to_owned(),
        }
    }
}

impl From<ElysMsg> for CosmosMsg<ElysMsg> {
    fn from(msg: ElysMsg) -> CosmosMsg<ElysMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for ElysMsg {}
