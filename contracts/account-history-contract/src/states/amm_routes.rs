use cw_storage_plus::Item;
use elys_bindings::types::SwapAmountInRoute;

pub const AMM_ROUTES: Item<Vec<SwapAmountInRoute>> = Item::new("amm_routes");
