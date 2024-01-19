use cosmwasm_schema::write_api;
use financial_snapshot_contract::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg
    }
}
