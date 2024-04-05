use super::super::spot_order::SpotOrder;
use crate::trade_shield::{states::PENDING_SPOT_ORDER, types::OrderPrice};
use cosmwasm_std::{Decimal, StdError, StdResult, Storage};

impl SpotOrder {
    pub fn binary_search(
        rate: &Decimal,
        storage: &dyn Storage,
        list: &Vec<u64>,
    ) -> StdResult<usize> {
        let mut low = 0;
        let mut high = list.len();

        while low < high {
            let mid = low + (high - low) / 2;
            let SpotOrder {
                order_price: OrderPrice { rate: mid_rate, .. },
                ..
            } = match PENDING_SPOT_ORDER.may_load(storage, list[mid])? {
                Some(order) => order,
                None => {
                    return Err(StdError::generic_err(
                        "spot=: binary search: price not found",
                    ))
                }
            };

            if mid_rate < *rate {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        Ok(low)
    }
}
