use super::super::spot_order::SpotOrder;
use crate::trade_shield::{states::SPOT_ORDER, types::OrderPrice};
use cosmwasm_std::{Decimal, StdResult, Storage};

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
            } = SPOT_ORDER.load(storage, list[mid])?;

            if mid_rate == *rate {
                return Ok(mid);
            }
            if mid_rate < *rate {
                low = mid
            } else {
                high = mid
            }
        }
        Ok(low)
    }
}
