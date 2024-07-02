# process_orders

## Overview:
`process_order` is responsible for processing both "spot_orders" and "perpetual_orders" returning `BankMsg` and SubMsg `ElysMsg` to be executed for processed orders.
It is an execute method with endpoint name `process_order` and hook which is called on end of each block.

## Detailed Description
### PENDING_SPOT_ORDER
"PENDING_SPOT_ORDER" is a storage map used for mapping all pending spot orders, each identified by a unique ID, to their corresponding unique SpotOrders. 

### SORTED_PENDING_SPOT_ORDER
"SORTED_PENDING_SPOT_ORDER" is a storage map used to associate order IDs belonging to the same combination of OrderType, Base Denom, and Quote Denom. Each unique combination serves as the key, which maps to a vector containing the corresponding spot order IDs.
order IDs are sorted in ascending order of price of spot_order. 

### Spot Order Processing

- `generate_spot_order_submsgs_bankmsgs`: Processes all pending "spot_orders" present in "SORTED_PENDING_SPOT_ORDER" of "SpotOrderType" including "{StopLoss, LimitBuy, and LimitSell}" to generate their `BankMsg` and SubMsg `ElysMsg`.

- `get_asset_price_from_denom_in_to_denom_out` fetches the market price for the specific combination of OrderType, Base Denom, and Quote Denom for pending spot orders of types "(LimitSell and StopLoss)". For "LimitBuy" orders, it is calculated inversely. Subsequently, the closest spot price to the market price in the orders is determined. If the price is unavailable, those combinations of spot orders are cancelled.

- `get_asset_price_from_denom_in_to_denom_out` fetches the price of Denom_in to Denom_out of the assets from oracle.

- Routes are validated from Denom_in to Denom_out. If a route is missing (i.e., there's no path to obtain Tokens A by exchanging Tokens B), then the spot_orders of that type are cancelled.

- `split_order_type`: Orders of the same type are divided into two groups based on their trigger price and the current market price: orders to be processed and orders yet to be processed, orders to include in process list based on the below criteria.
  - SpotOrderType::StopLoss => market_price <= order_price,
  - SpotOrderType::LimitSell => market_price >= order_price,
  - SpotOrderType::LimitBuy => market_price <= order_price. 

    > "Example: if some stop loss orders are as follows [90, 91 ,92 , 93, 94 ,95] and current market_price is 93
    so the order_price or the price of closest index would be 93 and all the orders greater and including 93 are added to process_order_ids and the orders lesser are yet to be processed" 

- `process_spot_order`: This function handles the order_ids that meet the aforementioned criteria. It adds the corresponding discount based on the calculated Membership tier . SubMsg is genrated for the new ReplyInfo with `reply_always` to capture the state of the transaction which would either be "Canceled" or "Executed" to update `reply_to_spot_order` will be called.

### PENDING_PERPETUAL_ORDER 
"PENDING_PERPETUAL_ORDER"  is a storage map used for mapping all pending perpetual orders, each identified by a unique ID, to their corresponding unique PerpetualOrder.

### SORTED_PENDING_PERPETUAL_ORDER 
"SORTED_PENDING_SPOT_ORDER" is a storage map used to associate order IDs belonging to the same combination of Position, OrderType, Base Denom, and Quote Denom. Each unique combination serves as the key, which maps to a vector containing the corresponding spot order IDs.
order IDs are sorted in ascending order of price of spot_order. 


### Perpetual Order Processing

- `generate_perpetual_orders_submsgs`:Processes all pending "spot_orders" present in "SORTED_PENDING_PERPETUAL_ORDER" to genrate `ElysMsg`.

- `get_asset_price_from_denom_in_to_denom_out` fetches the market price for the specific combination of Position, OrderType, Base Denom, and Quote Denom for pending perpetual orders. Subsequently, the closest spot price to the market price in the orders is determined. If the price is unavailable, those combinations of spot orders are cancelled.

- `split_perpetual_order`: Orders of the same type are divided into two groups based on their trigger price and the current market price: orders to be processed and orders yet to be processed orders to include in process list based on the below criteria.
    - PerpetualOrderType::LimitOpen, PerpetualPosition::Long => market_price <= order_price,
    - PerpetualOrderType::LimitOpen, PerpetualPosition::Short => market_price >= order_price,
    - PerpetualOrderType::LimitClose, PerpetualPosition::Long => market_price >= order_price,
    - PerpetualOrderType::LimitClose, PerpetualPosition::Short => market_price <= order_price,
    - PerpetualOrderType::StopLoss, PerpetualPosition::Long => market_price <= order_price,
    - PerpetualOrderType::StopLoss, PerpetualPosition::Short => market_price >= order_price,

- `process_perpetual_order`: This function handles the order_ids that meet the aforementioned criteria and SubMsg is genrated for the new ReplyInfo with `reply_always` to capture the state of the transaction which would either be "Canceled" or "Executed" depending on the SubMsg either `reply_to_close_perpetual_order` or `reply_to_open_perpetual_position` would be called to update state of orders.