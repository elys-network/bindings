use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum FilterType {
    FilterAll = 0,
    FilterPerpetual = 1,
    FilterFixedWeight = 2,
    FilterDynamicWeight = 3,
    FilterLeverage = 4,
}
